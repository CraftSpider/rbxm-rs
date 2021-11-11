//! The deserialization implementation for an RBXM

use crate::model::*;
use crate::serde::encoding::{
    decode_cumulative, Chomp, ChompInterleaved, ChompInterleavedTransform, ChompTransform,
};
use crate::serde::internal::{make_instance, RawProperty};
use crate::serde::io::Read;
use crate::serde::{Error, Result};
use crate::tree::Tree;

use alloc::collections::BTreeMap;
use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Default)]
struct RawInfo {
    meta: BTreeMap<String, String>,
    shared_strs: Vec<Vec<u8>>,
    class_ids: BTreeMap<i32, Vec<i32>>,
    instances: BTreeMap<i32, String>, // BTreeMap<i32, (String, Rc<RefCell<Instance>>)>,
    raw_props: BTreeMap<i32, BTreeMap<String, RawProperty>>,
    parent_info: BTreeMap<i32, i32>,
    child_info: BTreeMap<i32, Vec<i32>>,
}

/// Necessary state for deserializing a value
pub struct Deserializer<R> {
    reader: R,
    raw_info: RawInfo,
}

impl<R: Read> Deserializer<R> {
    /// Create a new deserializer from a reader and if necessary any other state
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer {
            reader,
            raw_info: RawInfo::default(),
        }
    }

    /// Deserialize a model from the input stream
    pub fn deserialize(mut self) -> Result<RbxModel> {
        let magic = <[u8; 16]>::chomp(&mut self.reader)?;

        if magic
            != [
                0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x21, 0x89, 0xFF, 0x0D, 0x0A, 0x1A, 0x0A,
                0x00, 0x00,
            ]
        {
            return Err(Error::BadMagic);
        }

        let num_classes = i32::chomp(&mut self.reader)?;
        let num_instances = i32::chomp(&mut self.reader)?;

        let unknown = (i32::chomp(&mut self.reader)?, i32::chomp(&mut self.reader)?);

        debug_assert_eq!(unknown, (0, 0));

        while self.chomp_block()? {}

        let magic_end = <[u8; 9]>::chomp(&mut self.reader)?;

        if magic_end != [0x3C, 0x2F, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x3E] {
            return Err(Error::BadMagic);
        }

        debug_assert_eq!(self.raw_info.class_ids.len(), num_classes as usize);
        debug_assert_eq!(self.raw_info.instances.len(), num_instances as usize);

        self.make_model()
    }

    fn make_model(self) -> Result<RbxModel> {
        let RawInfo {
            meta,
            shared_strs,
            class_ids: _,
            instances,
            mut raw_props,
            parent_info,
            child_info,
        } = self.raw_info;

        let mut id_key = BTreeMap::new();

        let tree = Tree::new();

        for &id in instances.keys() {
            id_key.insert(
                id,
                tree.add_root(Instance::Other(String::new(), BTreeMap::new())),
            );
        }

        // Do reference resolution, and populate information
        instances
            .iter()
            .try_for_each::<_, Result<()>>(|(&id, class_name)| {
                let raw_props = raw_props.remove(&id).ok_or(Error::UnknownInstance(id))?;

                let props = raw_props
                    .into_iter()
                    .map(|(name, value)| {
                        let prop = match value {
                            RawProperty::RawString(blob) => String::from_utf8(blob)
                                .map(Property::TextString)
                                .unwrap_or_else(|err| Property::BinaryString(err.into_bytes())),
                            RawProperty::RawSharedString(shared_id) => {
                                let blob = &shared_strs[shared_id as usize];
                                String::from_utf8(blob.clone())
                                    .map(Property::SharedTextString)
                                    .unwrap_or_else(|err| {
                                        Property::SharedBinaryString(err.into_bytes())
                                    })
                            }
                            RawProperty::InstanceRef(ref_id) => {
                                let key =
                                    id_key.get(&ref_id).ok_or(Error::UnknownInstance(ref_id))?;

                                Property::InstanceRef(*key)
                            }
                            prop => prop.into_real(),
                        };

                        Ok((name, prop))
                    })
                    .collect::<Result<_>>()?;

                *tree.try_get_mut(id_key[&id]).unwrap() = make_instance(class_name, props)?;

                Ok(())
            })?;

        for (child, parent) in parent_info.into_iter().filter(|&(_, parent)| parent != -1) {
            let parent_key = *id_key.get(&parent).ok_or(Error::UnknownInstance(parent))?;
            let child_key = *id_key.get(&child).ok_or(Error::UnknownInstance(child))?;
            let mut parent = tree.try_get_mut(parent_key).unwrap();
            let child = tree.try_get(child_key).unwrap();

            parent.add_child(&child);
        }

        for (parent, children) in child_info.into_iter().filter(|&(parent, _)| parent != -1) {
            let parent_key = *id_key.get(&parent).ok_or(Error::UnknownInstance(parent))?;
            let parent = tree.try_get(parent_key).unwrap();

            let expected_children = children
                .into_iter()
                .map(|i| id_key.get(&i).copied().ok_or(Error::UnknownInstance(i)))
                .collect::<Result<BTreeSet<_>>>()?;

            let real_children = parent
                .children()
                .into_iter()
                .map(|r| r.map(|r| r.key()))
                .collect::<core::result::Result<BTreeSet<_>, _>>()
                .unwrap();

            if expected_children != real_children {
                return Err(Error::InconsistentTree);
            }
        }

        Ok(RbxModel { meta, nodes: tree })
    }

    fn chomp_block(&mut self) -> Result<bool> {
        let name = self.chomp_blockname()?;

        if name == "END" {
            let end_data = <[u8; 12]>::chomp(&mut self.reader)?;
            assert_eq!(end_data, [0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0]);
            return Ok(false);
        }

        let data = self.chomp_lz4()?;
        let block_reader = &mut (&data as &[u8]);

        match &*name {
            "SSTR" => {
                let unknown = i32::chomp(block_reader)?;
                assert_eq!(unknown, 0);
                let num_strs = i32::chomp(block_reader)?;

                for _ in 0..num_strs {
                    let unknown1 = i32::chomp(block_reader)?;
                    let unknown2 = i32::chomp(block_reader)?;
                    let unknown3 = i32::chomp(block_reader)?;
                    let unknown4 = i32::chomp(block_reader)?;
                    assert_eq!(unknown1, 0);
                    assert_eq!(unknown2, 0);
                    assert_eq!(unknown3, 0);
                    assert_eq!(unknown4, 0);

                    self.raw_info
                        .shared_strs
                        .push(<Vec<u8>>::chomp(block_reader)?);
                }
            }
            "META" => {
                let num_pairs = i32::chomp(block_reader)?;
                for _ in 0..num_pairs {
                    let key = String::chomp(block_reader)?;
                    let value = String::chomp(block_reader)?;
                    self.raw_info.meta.insert(key, value);
                }
            }
            "INST" => {
                let index = i32::chomp(block_reader)?;
                let class_name = String::chomp(block_reader)?;
                let _is_service = bool::chomp(block_reader)?;
                let instance_count = i32::chomp(block_reader)?;
                let mut instance_ids =
                    i32::chomp_interleaved_transformed(block_reader, instance_count as usize)?;

                decode_cumulative(&mut instance_ids);

                for id in &instance_ids {
                    self.raw_info.instances.insert(*id, class_name.clone());
                }

                self.raw_info.class_ids.insert(index, instance_ids);
            }
            "PROP" => {
                let class_index = i32::chomp(block_reader)?;
                let property_name = String::chomp(block_reader)?;
                let prop_ty = u8::chomp(block_reader)?;

                let class_ids = self
                    .raw_info
                    .class_ids
                    .get(&class_index)
                    .ok_or(Error::UnknownClass(class_index))?;

                let num_props = class_ids.len();

                let mut properties = Vec::with_capacity(num_props);
                for _ in 0..num_props {
                    let prop = match prop_ty {
                        1 => RawProperty::RawString(<Vec<u8>>::chomp(block_reader)?),
                        2 => RawProperty::Bool(bool::chomp(block_reader)?),
                        3 => RawProperty::Int32(i32::chomp_transformed(block_reader)?),
                        4 => RawProperty::Float(f32::chomp_transformed(block_reader)?),
                        5 => RawProperty::Double(f64::chomp(block_reader)?),
                        6 => {
                            properties.extend(
                                UDim::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::UDim),
                            );
                            break;
                        }
                        7 => {
                            properties.extend(
                                UDim2::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::UDim2),
                            );
                            break;
                        }
                        8 => {
                            properties.extend(
                                Ray::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Ray),
                            );
                            break;
                        }
                        9 => RawProperty::Face(Faces::chomp(block_reader)?),
                        10 => RawProperty::Axis(Axes::chomp(block_reader)?),
                        11 => {
                            properties.extend(
                                BrickColor::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::BrickColor),
                            );
                            break;
                        }
                        12 => RawProperty::Color3(Color3::chomp_transformed(block_reader)?),
                        13 => RawProperty::Vector2(Vector2::chomp(block_reader)?),
                        14 => RawProperty::Vector3(Vector3::chomp(block_reader)?),
                        16 => {
                            properties.extend(
                                CFrame::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::CFrame),
                            );
                            break;
                        }
                        17 => todo!("Quaternions not yet supported"),
                        18 => {
                            properties.extend(
                                i32::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Enum),
                            );
                            break;
                        }
                        19 => {
                            let mut ids = i32::chomp_interleaved(block_reader, num_props)?;
                            decode_cumulative(&mut ids);
                            properties.extend(ids.into_iter().map(RawProperty::InstanceRef));
                            break;
                        }
                        20 => RawProperty::Vector3Int16(Vector3Int16::chomp(block_reader)?),
                        21 => RawProperty::NumberSequence(NumberSequence::chomp(block_reader)?),
                        22 => RawProperty::ColorSequence(ColorSequence::chomp(block_reader)?),
                        23 => RawProperty::NumberRange(NumberRange::chomp(block_reader)?),
                        24 => {
                            properties.extend(
                                Rect::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Rect),
                            );
                            break;
                        }
                        25 => RawProperty::CustomPhysicalProperties(bool::chomp(block_reader)?),
                        26 => RawProperty::Color3Uint8(Color3Uint8::chomp(block_reader)?),
                        27 => RawProperty::Int64(i64::chomp(block_reader)?),
                        28 => {
                            properties.extend(
                                i32::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::RawSharedString),
                            );
                            break;
                        }
                        30 => {
                            properties.extend(
                                Pivot::chomp_interleaved(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Pivot),
                            );
                            break;
                        }
                        _ => {
                            return Err(Error::UnknownProperty(prop_ty));
                        }
                    };

                    properties.push(prop)
                }

                assert_eq!(
                    *block_reader,
                    [],
                    "Property {} didn't consume whole buffer",
                    prop_ty
                );

                for (inst_id, property) in class_ids.iter().zip(properties.into_iter()) {
                    self.raw_info
                        .raw_props
                        .entry(*inst_id)
                        .or_default()
                        .insert(property_name.clone(), property);
                }
            }
            "PRNT" => {
                let unknown = u8::chomp(block_reader)?;
                assert_eq!(unknown, 0);
                let len = i32::chomp(block_reader)?;
                let mut instance_referents =
                    i32::chomp_interleaved_transformed(block_reader, len as usize)?;
                let mut parent_referents =
                    i32::chomp_interleaved_transformed(block_reader, len as usize)?;

                decode_cumulative(&mut instance_referents);
                decode_cumulative(&mut parent_referents);

                for (child_id, parent_id) in instance_referents
                    .into_iter()
                    .zip(parent_referents.into_iter())
                {
                    self.raw_info.parent_info.insert(child_id, parent_id);
                    self.raw_info
                        .child_info
                        .entry(parent_id)
                        .or_default()
                        .push(child_id);
                }
            }
            _ => return Err(Error::UnknownBlock(name)),
        }
        Ok(true)
    }

    fn chomp_blockname(&mut self) -> Result<String> {
        let data = <[u8; 4]>::chomp(&mut self.reader)?;

        let first_zero = data.iter().copied().position(|b| b == 0).unwrap_or(4) as usize;

        Ok(core::str::from_utf8(&data[..first_zero])
            .map_err(|_| Error::InvalidString)?
            .to_string())
    }

    fn chomp_lz4(&mut self) -> Result<Vec<u8>> {
        let compressed = i32::chomp(&mut self.reader)?;
        let uncompressed = i32::chomp(&mut self.reader)? as usize;

        assert_eq!(i32::chomp(&mut self.reader)?, 0i32);

        let mut data = vec![0; compressed as usize];
        self.reader.read_exact(&mut data)?;

        let out =
            lz4_flex::block::decompress(&data, uncompressed).map_err(|_| Error::InvalidLz4)?;

        assert_eq!(out.len(), uncompressed);

        Ok(out)
    }
}

/// Read a model from the provided IO reader
pub fn from_reader<R: Read>(reader: R) -> Result<RbxModel> {
    Deserializer::new(reader).deserialize()
}

/// Read a model from an existing file
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<RbxModel> {
    from_reader(std::fs::File::open(path)?)
}

/// Read a model from a raw byte slice
pub fn from_bytes(bytes: &[u8]) -> Result<RbxModel> {
    from_reader(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attrs() {
        let part = from_file("attrs.rbxm").unwrap();

        dbg!(part);
    }
}
