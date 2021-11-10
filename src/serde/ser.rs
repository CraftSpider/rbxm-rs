//! The serialization implementation for an RBXM

use crate::model::*;
#[cfg(feature = "mesh-format")]
use crate::serde::internal::{break_kind, RawProperty};
use crate::serde::Result;
use crate::serde::io::Write;
use crate::serde::encoding::{Print, PrintTransform, PrintInterleaved, PrintInterleavedTransform};

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub(crate) enum Block {
    Meta(BTreeMap<String, String>),
    SharedStr(Vec<Vec<u8>>),
    Instance {
        index: i32,
        class_name: String,
        is_service: bool,
        instance_ids: Vec<i32>,
    },
    Property {
        class_index: i32,
        property_name: String,
        properties: Vec<RawProperty>,
    },
    Parent {
        instance_referents: Vec<i32>,
        parent_referents: Vec<i32>,
    },
    End,
}

/// Necessary state for serializing a value
pub struct Serializer<W> {
    writer: W,
}

fn make_cumulative(mut slice: &mut [i32]) {
    for _ in (1..slice.len()).rev() {
        let (previous, last) = slice.split_at_mut(slice.len() - 1);
        last[0] -= previous.last().unwrap();
        slice = previous;
    }
}

fn break_model(model: &RbxModel) -> (i32, i32, Vec<Block>) {
    #[allow(clippy::mutable_key_type)]
    let key_to_id: BTreeMap<_, _> = model
        .nodes
        .unordered_keys()
        .enumerate()
        .map(|(idx, key)| (key, idx))
        .collect();

    let mut inst_blocks = BTreeMap::new();
    let mut prop_blocks = BTreeMap::new();
    let mut parents = BTreeMap::new();
    let mut shared_strs = Vec::new();

    for node in model.nodes.unordered_iter() {
        let index = key_to_id[&node.key()];
        let inst = &**node;
        let next_index = inst_blocks.len();
        let class_name = inst.class_name();

        let inst_block = inst_blocks
            .entry(class_name.clone())
            .or_insert(Block::Instance {
                index: next_index as i32,
                class_name,
                is_service: false,
                instance_ids: vec![],
            });

        let class_index = if let Block::Instance {
            index: class_index,
            instance_ids,
            ..
        } = inst_block
        {
            instance_ids.push(index as i32);
            *class_index
        } else {
            unreachable!()
        };

        for (prop_name, prop_value) in break_kind(inst) {
            let prop_block = prop_blocks
                .entry((class_index, prop_name.clone()))
                .or_insert(Block::Property {
                    class_index,
                    property_name: prop_name.clone(),
                    properties: vec![],
                });
            if let Block::Property { properties, .. } = prop_block {
                let raw = match prop_value {
                    Property::BinaryString(blob) => RawProperty::RawString(blob.clone()),
                    Property::TextString(str) => RawProperty::RawString(str.clone().into_bytes()),
                    Property::SharedBinaryString(blob) => {
                        let pos = shared_strs.iter().position(|data| data == &blob);

                        match pos {
                            Some(pos) => RawProperty::RawSharedString(pos as i32),
                            None => {
                                shared_strs.push(blob);
                                RawProperty::RawSharedString((shared_strs.len() - 1) as i32)
                            }
                        }
                    }
                    Property::SharedTextString(str) => {
                        let blob = str.into_bytes();
                        let pos = shared_strs.iter().position(|data| data == &blob);

                        match pos {
                            Some(pos) => RawProperty::RawSharedString(pos as i32),
                            None => {
                                shared_strs.push(blob);
                                RawProperty::RawSharedString((shared_strs.len() - 1) as i32)
                            }
                        }
                    }
                    Property::InstanceRef(val) => RawProperty::InstanceRef(key_to_id[&val] as i32),
                    prop => RawProperty::from_real(prop.clone()),
                };
                properties.push(raw)
            } else {
                unreachable!()
            }
        }

        let parent_index = match node.parent() {
            Ok(Some(parent)) => key_to_id[&parent.key()] as i32,
            Ok(None) => -1,
            Err(e) => panic!("Failed to get expected node parent: {}", e),
        };

        parents.insert(index as i32, parent_index);
    }

    let num_classes = inst_blocks.len();
    let num_insts = model.nodes.len();

    let child_ref = parents.keys().copied().collect::<Vec<_>>();
    let parent_ref = parents.values().copied().collect::<Vec<_>>();

    let mut out = vec![Block::Meta(model.meta.clone())];
    if !shared_strs.is_empty() {
        out.push(Block::SharedStr(shared_strs))
    }
    out.extend(inst_blocks.into_iter().map(|(_, block)| block));
    out.extend(prop_blocks.into_iter().map(|(_, block)| block));
    out.push(Block::Parent {
        instance_referents: child_ref,
        parent_referents: parent_ref,
    });
    out.push(Block::End);

    (num_classes as i32, num_insts as i32, out)
}

impl<W: Write> Serializer<W> {
    /// Create a new serializer from a writer and if necessary any other state
    pub fn new(writer: W) -> Serializer<W> {
        Serializer { writer }
    }

    /// Serialize a model to the output stream
    pub fn serialize(mut self, model: &RbxModel) -> Result<()> {
        let (num_classes, num_insts, blocks) = break_model(model);

        // Magic start value
        self.writer.write_all(&[
            0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x21, 0x89, 0xFF, 0x0D, 0x0A, 0x1A, 0x0A,
            0x00, 0x00,
        ])?;

        i32::print(&mut self.writer, num_classes)?;
        i32::print(&mut self.writer, num_insts)?;
        i32::print(&mut self.writer, 0)?;
        i32::print(&mut self.writer, 0)?;

        for b in blocks {
            self.write_block(b)?;
        }

        // Magic end value, maybe part of END?
        self.writer
            .write_all(&[0x3C, 0x2F, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x3E])?;

        Ok(())
    }

    fn write_block(&mut self, block: Block) -> Result<()> {
        let mut out_buffer = Vec::new();
        let writer = &mut out_buffer;

        let block_name = match block {
            Block::Meta(attrs) => {
                i32::print(writer, attrs.len() as i32)?;
                for (key, value) in attrs {
                    String::print(writer, key)?;
                    String::print(writer, value)?;
                }
                b"META"
            }
            Block::SharedStr(strs) => {
                i32::print(writer, 0)?;
                i32::print(writer, strs.len() as i32)?;

                for blob in strs {
                    i32::print(writer, 0)?;
                    i32::print(writer, 0)?;
                    i32::print(writer, 0)?;
                    i32::print(writer, 0)?;

                    <&[u8]>::print(writer, &blob)?;
                }

                b"SSTR"
            }
            Block::Instance {
                index,
                is_service,
                class_name,
                mut instance_ids,
            } => {
                i32::print(writer, index)?;
                String::print(writer, class_name)?;
                bool::print(writer, is_service)?;
                i32::print(writer, instance_ids.len() as i32)?;

                make_cumulative(&mut instance_ids);
                i32::print_interleaved_transformed(writer, &instance_ids)?;

                b"INST"
            }
            Block::Property {
                class_index,
                property_name,
                properties,
            } => {
                i32::print(writer, class_index)?;
                String::print(writer, property_name)?;

                let prop_ty = match &properties[0] {
                    RawProperty::RawString(..) => 1,
                    RawProperty::Bool(..) => 2,
                    RawProperty::Int32(..) => 3,
                    RawProperty::Float(..) => 4,
                    RawProperty::Double(..) => 5,
                    RawProperty::UDim(..) => 6,
                    RawProperty::UDim2(..) => 7,
                    RawProperty::Ray(..) => 8,
                    RawProperty::Face(..) => 9,
                    RawProperty::Axis(..) => 10,
                    RawProperty::BrickColor(..) => 11,
                    RawProperty::Color3(..) => 12,
                    RawProperty::Vector2(..) => 13,
                    RawProperty::Vector3(..) => 14,
                    RawProperty::CFrame(..) => 16,
                    RawProperty::Quaternion => 17,
                    RawProperty::Enum(..) => 18,
                    RawProperty::InstanceRef(..) => 19,
                    RawProperty::Vector3Int16(..) => 20,
                    RawProperty::NumberSequence(..) => 21,
                    RawProperty::ColorSequence(..) => 22,
                    RawProperty::NumberRange(..) => 23,
                    RawProperty::Rect(..) => 24,
                    RawProperty::CustomPhysicalProperties(..) => 25,
                    RawProperty::Color3Uint8(..) => 26,
                    RawProperty::Int64(..) => 27,
                    RawProperty::RawSharedString(..) => 28,
                    RawProperty::Pivot(..) => 30,
                };

                u8::print(writer, prop_ty)?;

                for prop in &properties {
                    match prop {
                        RawProperty::RawString(blob) => <&[u8]>::print(writer, blob)?,
                        RawProperty::Bool(val) => bool::print(writer, *val)?,
                        RawProperty::Int32(val) => i32::print_transformed(writer, *val)?,
                        RawProperty::Float(val) => f32::print_transformed(writer, *val)?,
                        RawProperty::Double(val) => f64::print(writer, *val)?,
                        RawProperty::UDim(..) => {
                            UDim::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::UDim(u) = i {
                                    u
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::UDim2(..) => {
                            UDim2::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::UDim2(u) = i {
                                    u
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::Ray(..) => {
                            Ray::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::Ray(r) = i {
                                    r
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::Face(val) => Faces::print(writer, val.clone())?,
                        RawProperty::Axis(val) => Axes::print(writer, val.clone())?,
                        RawProperty::BrickColor(..) => {
                            BrickColor::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::BrickColor(bc) = i {
                                    bc
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::Color3(val) => Color3::print_transformed(writer, val.clone())?,
                        RawProperty::Vector2(val) => Vector2::print(writer, val.clone())?,
                        RawProperty::Vector3(val) => Vector3::print(writer, val.clone())?,
                        RawProperty::CFrame(..) => {
                            CFrame::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::CFrame(c) = i {
                                        c
                                    } else {
                                        unreachable!()
                                    }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::Quaternion => todo!("Quaternions not yet supported"),
                        RawProperty::Enum(..) => {
                            i32::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::Enum(e) = i {
                                    e
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::InstanceRef(..) => {
                            i32::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::InstanceRef(e) = i {
                                    e
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::Vector3Int16(val) => Vector3Int16::print(writer, val.clone())?,
                        RawProperty::NumberSequence(val) => NumberSequence::print(writer, val.clone())?,
                        RawProperty::ColorSequence(val) => ColorSequence::print(writer, val.clone())?,
                        RawProperty::NumberRange(val) => NumberRange::print(writer, val.clone())?,
                        RawProperty::Rect(..) => {
                            Rect::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::Rect(c) = i {
                                    c
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::CustomPhysicalProperties(val) => bool::print(writer, *val)?,
                        RawProperty::Color3Uint8(val) => Color3Uint8::print(writer, val.clone())?,
                        RawProperty::Int64(val) => i64::print(writer, *val)?,
                        RawProperty::RawSharedString(..) => {
                            i32::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::InstanceRef(e) = i {
                                    e
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                        RawProperty::Pivot(..) => {
                            CFrame::print_interleaved(
                                writer,
                                &properties.iter().cloned().map(|i| if let RawProperty::CFrame(c) = i {
                                    c
                                } else {
                                    unreachable!()
                                }).collect::<Vec<_>>()
                            )?;
                            break;
                        }
                    }
                }

                b"PROP"
            }
            Block::Parent {
                mut instance_referents,
                mut parent_referents,
            } => {
                make_cumulative(&mut instance_referents);
                make_cumulative(&mut parent_referents);

                u8::print(writer, 0)?;
                i32::print(writer, instance_referents.len() as i32)?;
                i32::print_interleaved_transformed(writer, &instance_referents)?;
                i32::print_interleaved_transformed(writer, &parent_referents)?;
                b"PRNT"
            }
            Block::End => {
                // END is special
                self.writer.write_all(b"END\0")?;
                i32::print(&mut self.writer, 0)?;
                i32::print(&mut self.writer, 9)?;
                i32::print(&mut self.writer, 0)?;
                return Ok(());
            }
        };

        let compressed_data = lz4_flex::block::compress(&out_buffer);

        self.writer.write_all(block_name)?;
        i32::print(&mut self.writer, compressed_data.len() as i32)?;
        i32::print(&mut self.writer, out_buffer.len() as i32)?;
        i32::print(&mut self.writer, 0)?;
        self.writer.write_all(&compressed_data)?;

        Ok(())
    }
}

/// Write a model out to a provided IO writer
pub fn to_writer<W: Write>(writer: W, model: &RbxModel) -> Result<()> {
    Serializer::new(writer).serialize(model)
}

/// Write a model out to a file, creating it if necessary
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[cfg(feature = "std")]
pub fn to_file<P: AsRef<std::path::Path>>(path: P, model: &RbxModel) -> Result<()> {
    to_writer(std::fs::File::create(path)?, model)
}

/// Write a model to a vector as raw bytes, and return it
pub fn to_bytes(model: &RbxModel) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    to_writer(&mut out, model)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_cumulative() {
        let mut array = [0, 1, 2, 7];
        make_cumulative(&mut array);
        assert_eq!(array, [0, 1, 1, 5]);
    }
}
