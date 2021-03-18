use crate::model::*;
use crate::serde::internal::{make_kind, Block, RawProperty};
use crate::serde::{Error, Result};

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::rc::{Rc, Weak};

// Decode the special formats used to store some values

fn decode_i32(mut raw: u32) -> i32 {
    let sign = raw & 1;
    raw >>= 1;
    let mut out = i32::from_ne_bytes(raw.to_ne_bytes());
    if sign == 1 {
        out += 1;
        out = -out;
    }
    out
}

fn decode_f32(mut raw: u32) -> f32 {
    let sign = raw & 1;
    raw >>= 1;
    raw ^= sign * 0b1000_0000_0000_0000;
    f32::from_ne_bytes(raw.to_ne_bytes())
}

fn make_cumulative(mut slice: &mut [i32]) {
    for _ in 1..slice.len() {
        let (first, second) = slice.split_at_mut(1);
        second[0] += first[0];
        slice = second;
    }
}

// Consume ('chomp') some data from an input

fn chomp_bool<R: Read>(reader: &mut R) -> Result<bool> {
    let mut data = [0; 1];
    reader.read_exact(&mut data)?;
    Ok(data[0] != 0)
}

fn chomp_u8<R: Read>(reader: &mut R) -> Result<u8> {
    let mut data = [0; 1];
    reader.read_exact(&mut data)?;
    Ok(data[0])
}

fn chomp_i16<R: Read>(reader: &mut R) -> Result<i16> {
    let mut data = [0; 2];
    reader.read_exact(&mut data)?;
    Ok(i16::from_le_bytes(data))
}

fn chomp_i32_raw<R: Read>(reader: &mut R) -> Result<i32> {
    let mut data = [0; 4];
    reader.read_exact(&mut data)?;
    Ok(i32::from_le_bytes(data))
}

fn chomp_i32_transformed<R: Read>(reader: &mut R) -> Result<i32> {
    let mut data = [0; 4];
    reader.read_exact(&mut data)?;
    Ok(decode_i32(u32::from_be_bytes(data)))
}

fn chomp_i64<R: Read>(reader: &mut R) -> Result<i64> {
    let mut data = [0; 8];
    reader.read_exact(&mut data)?;
    Ok(i64::from_le_bytes(data))
}

fn chomp_f32_raw<R: Read>(reader: &mut R) -> Result<f32> {
    let mut data = [0; 4];
    reader.read_exact(&mut data)?;
    Ok(f32::from_le_bytes(data))
}

fn chomp_f32_transformed<R: Read>(reader: &mut R) -> Result<f32> {
    let mut data = [0; 4];
    reader.read_exact(&mut data)?;
    Ok(decode_f32(u32::from_be_bytes(data)))
}

fn chomp_f64<R: Read>(reader: &mut R) -> Result<f64> {
    let mut data = [0; 8];
    reader.read_exact(&mut data)?;
    Ok(f64::from_le_bytes(data))
}

fn chomp_string<R: Read>(reader: &mut R) -> Result<String> {
    let len = chomp_i32_raw(reader)?;
    let mut str = vec![0; len as usize];
    reader.read_exact(&mut str)?;
    String::from_utf8(str).map_err(|_| Error::InvalidString)
}

fn chomp_binary_string<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let len = chomp_i32_raw(reader)?;
    let mut str = vec![0; len as usize];
    reader.read_exact(&mut str)?;
    Ok(str)
}

fn chomp_interleaved_i32_raw<R: Read>(reader: &mut R, len: usize) -> Result<Vec<i32>> {
    let mut data = vec![0; len * 4];
    reader.read_exact(&mut data)?;

    let mut out = vec![0; len];
    for i in 0..len {
        let mut bytes = [0; 4];
        for j in 0..4 {
            bytes[j] = data[i + j * len];
        }
        out[i] = i32::from_be_bytes(bytes);
    }

    Ok(out)
}

fn chomp_interleaved_i32_transformed<R: Read>(reader: &mut R, len: usize) -> Result<Vec<i32>> {
    let mut data = vec![0; len * 4];
    reader.read_exact(&mut data)?;

    let mut out = vec![0; len];
    for i in 0..len {
        let mut bytes = [0; 4];
        for j in 0..4 {
            bytes[j] = data[i + j * len];
        }
        out[i] = decode_i32(u32::from_be_bytes(bytes));
    }
    Ok(out)
}

fn chomp_interleaved_f32<R: Read>(reader: &mut R, len: usize) -> Result<Vec<f32>> {
    let mut data = vec![0; len * 4];
    reader.read_exact(&mut data)?;

    let mut out = vec![0f32; len];
    for i in 0..len {
        let mut bytes = [0; 4];
        for j in 0..4 {
            // println!("[{}]", i + j * len);
            bytes[j] = data[i + j * len];
        }
        out[i] = decode_f32(u32::from_be_bytes(bytes));
    }

    Ok(out)
}

fn chomp_udims<R: Read>(reader: &mut R, len: usize) -> Result<Vec<UDim>> {
    let scales = chomp_interleaved_f32(reader, len)?;
    let offsets = chomp_interleaved_i32_raw(reader, len)?;

    let mut out = Vec::with_capacity(len);
    for (scale, offset) in scales.into_iter().zip(offsets.into_iter()) {
        out.push(UDim { scale, offset })
    }

    Ok(out)
}

fn chomp_udim2s<R: Read>(reader: &mut R, len: usize) -> Result<Vec<UDim2>> {
    let x_scales = chomp_interleaved_f32(reader, len)?;
    let y_scales = chomp_interleaved_f32(reader, len)?;
    let x_offsets = chomp_interleaved_i32_raw(reader, len)?;
    let y_offsets = chomp_interleaved_i32_raw(reader, len)?;

    let mut out = Vec::with_capacity(len);
    for (((x_scale, y_scale), x_offset), y_offset) in x_scales
        .into_iter()
        .zip(y_scales.into_iter())
        .zip(x_offsets.into_iter())
        .zip(y_offsets.into_iter())
    {
        out.push(UDim2 {
            x_scale,
            y_scale,
            x_offset,
            y_offset,
        })
    }
    Ok(out)
}

fn chomp_rays<R: Read>(reader: &mut R, len: usize) -> Result<Vec<Ray>> {
    let x_origs = chomp_interleaved_f32(reader, len)?;
    let y_origs = chomp_interleaved_f32(reader, len)?;
    let z_origs = chomp_interleaved_f32(reader, len)?;
    let x_dir = chomp_interleaved_f32(reader, len)?;
    let y_dir = chomp_interleaved_f32(reader, len)?;
    let z_dir = chomp_interleaved_f32(reader, len)?;

    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(Ray {
            origin: Vector3 {
                x: x_origs[i],
                y: y_origs[i],
                z: z_origs[i],
            },
            direction: Vector3 {
                x: x_dir[i],
                y: y_dir[i],
                z: z_dir[i],
            },
        })
    }

    Ok(out)
}

fn chomp_face<R: Read>(reader: &mut R) -> Result<Face> {
    let data = chomp_u8(reader)?;
    Ok(Face {
        front: data & 0b00000001 > 0,
        bottom: data & 0b00000010 > 0,
        left: data & 0b00000100 > 0,
        back: data & 0b00001000 > 0,
        top: data & 0b00010000 > 0,
        right: data & 0b00100000 > 0,
    })
}

fn chomp_axis<R: Read>(reader: &mut R) -> Result<Axis> {
    let data = chomp_u8(reader)?;
    Ok(Axis {
        x: data & 0b100 != 0,
        y: data & 0b010 != 0,
        z: data & 0b001 != 0,
    })
}

fn chomp_brick_colors<R: Read>(reader: &mut R, len: usize) -> Result<Vec<BrickColor>> {
    let mut colors = Vec::with_capacity(len);
    let indices = chomp_interleaved_i32_raw(reader, len)?;
    for index in indices {
        colors.push(BrickColor { index })
    }
    Ok(colors)
}

fn chomp_color3_raw<R: Read>(reader: &mut R) -> Result<Color3> {
    Ok(Color3 {
        r: chomp_f32_raw(reader)?,
        g: chomp_f32_raw(reader)?,
        b: chomp_f32_raw(reader)?,
    })
}

fn chomp_color3<R: Read>(reader: &mut R) -> Result<Color3> {
    Ok(Color3 {
        r: chomp_f32_transformed(reader)?,
        g: chomp_f32_transformed(reader)?,
        b: chomp_f32_transformed(reader)?,
    })
}

fn chomp_vector2<R: Read>(reader: &mut R) -> Result<Vector2> {
    Ok(Vector2 {
        x: chomp_f32_transformed(reader)?,
        y: chomp_f32_transformed(reader)?,
    })
}

fn chomp_vector3<R: Read>(reader: &mut R) -> Result<Vector3> {
    Ok(Vector3 {
        x: chomp_f32_transformed(reader)?,
        y: chomp_f32_transformed(reader)?,
        z: chomp_f32_transformed(reader)?,
    })
}

fn chomp_cframes<R: Read>(reader: &mut R, len: usize) -> Result<Vec<CFrame>> {
    let mut angles = Vec::with_capacity(len);
    for _ in 0..len {
        let angle_type = chomp_u8(reader)?;
        let angle: [[f32; 3]; 3] = match angle_type {
            0 => {
                let mut data = [[0f32; 3]; 3];
                for i in &mut data {
                    for j in i {
                        *j = chomp_f32_raw(reader)?;
                    }
                }
                data
            }
            2 => [[1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, 1f32]],
            3 => [[1f32, 0f32, 0f32], [0f32, 0f32, -1f32], [0f32, 1f32, 0f32]],
            5 => [[1f32, 0f32, 0f32], [0f32, -1f32, 0f32], [0f32, 0f32, -1f32]],
            6 => [[1f32, 0f32, 0f32], [0f32, 0f32, 1f32], [0f32, -1f32, 0f32]],
            7 => [[0f32, 1f32, 0f32], [1f32, 0f32, 0f32], [0f32, 0f32, -1f32]],
            9 => [[0f32, 0f32, 1f32], [1f32, 0f32, 0f32], [0f32, 1f32, 0f32]],
            10 => [[0f32, -1f32, 0f32], [1f32, 0f32, 0f32], [0f32, 0f32, 1f32]],
            12 => [[0f32, 0f32, -1f32], [1f32, 0f32, 0f32], [0f32, -1f32, 0f32]],
            13 => [[0f32, 1f32, 0f32], [0f32, 0f32, 1f32], [1f32, 0f32, 0f32]],
            14 => [[0f32, 0f32, -1f32], [0f32, 1f32, 0f32], [1f32, 0f32, 0f32]],
            16 => [[0f32, -1f32, 0f32], [0f32, 0f32, -1f32], [1f32, 0f32, 0f32]],
            17 => [[0f32, 0f32, 1f32], [0f32, -1f32, 0f32], [1f32, 0f32, 0f32]],
            20 => [[-1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, -1f32]],
            21 => [[-1f32, 0f32, 0f32], [0f32, 0f32, 1f32], [0f32, 1f32, 0f32]],
            23 => [[-1f32, 0f32, 0f32], [0f32, -1f32, 0f32], [0f32, 0f32, 1f32]],
            24 => [
                [-1f32, 0f32, 0f32],
                [0f32, 0f32, -1f32],
                [0f32, -1f32, 0f32],
            ],
            25 => [[0f32, 1f32, 0f32], [-1f32, 0f32, 0f32], [0f32, 0f32, 1f32]],
            27 => [[0f32, 0f32, -1f32], [-1f32, 0f32, 0f32], [0f32, 1f32, 0f32]],
            28 => [
                [0f32, -1f32, 0f32],
                [-1f32, 0f32, 0f32],
                [0f32, 0f32, -1f32],
            ],
            30 => [[0f32, 0f32, 1f32], [-1f32, 0f32, 0f32], [0f32, -1f32, 0f32]],
            31 => [[0f32, 1f32, 0f32], [0f32, 0f32, -1f32], [-1f32, 0f32, 0f32]],
            32 => [[0f32, 0f32, 1f32], [0f32, 1f32, 0f32], [-1f32, 0f32, 0f32]],
            34 => [[0f32, -1f32, 0f32], [0f32, 0f32, 1f32], [-1f32, 0f32, 0f32]],
            35 => [
                [0f32, 0f32, -1f32],
                [0f32, -1f32, 0f32],
                [-1f32, 0f32, 0f32],
            ],
            _ => return Err(Error::UnknownCFrame(angle_type)),
        };

        angles.push(angle);
    }
    let mut positions = Vec::with_capacity(len);
    for _ in 0..len {
        positions.push(chomp_vector3(reader)?);
    }

    let mut out = Vec::with_capacity(len);
    for (position, angle) in positions.into_iter().zip(angles.into_iter()) {
        out.push(CFrame { position, angle })
    }

    Ok(out)
}

fn chomp_vector3_i16<R: Read>(reader: &mut R) -> Result<Vector3Int16> {
    Ok(Vector3Int16 {
        x: chomp_i16(reader)?,
        y: chomp_i16(reader)?,
        z: chomp_i16(reader)?,
    })
}

fn chomp_number_sequence<R: Read>(reader: &mut R) -> Result<NumberSequence> {
    let num_keypoints = chomp_i32_raw(reader)?;
    let mut keypoints = Vec::with_capacity(num_keypoints as usize);
    for _ in 0..num_keypoints {
        let time = chomp_f32_raw(reader)?;
        let value = chomp_f32_raw(reader)?;
        let envelope = chomp_f32_raw(reader)?;
        keypoints.push(NumberKeypoint {
            time,
            value,
            envelope,
        })
    }

    Ok(NumberSequence { keypoints })
}

fn chomp_color_sequence<R: Read>(reader: &mut R) -> Result<ColorSequence> {
    let num_keypoints = chomp_i32_raw(reader)?;
    let mut keypoints = Vec::with_capacity(num_keypoints as usize);
    for _ in 0..num_keypoints {
        let time = chomp_f32_raw(reader)?;
        let color = chomp_color3_raw(reader)?;
        let envelope = chomp_f32_raw(reader)?;
        keypoints.push(ColorKeypoint {
            time,
            color,
            envelope,
        })
    }

    Ok(ColorSequence { keypoints })
}

fn chomp_number_range<R: Read>(reader: &mut R) -> Result<NumberRange> {
    Ok(NumberRange {
        low: chomp_f32_raw(reader)?,
        high: chomp_f32_raw(reader)?,
    })
}

fn chomp_rects<R: Read>(reader: &mut R, len: usize) -> Result<Vec<Rect>> {
    let min_x = chomp_interleaved_f32(reader, len)?;
    let min_y = chomp_interleaved_f32(reader, len)?;
    let max_x = chomp_interleaved_f32(reader, len)?;
    let max_y = chomp_interleaved_f32(reader, len)?;

    let mut out = Vec::with_capacity(len);
    for i in 0..len {
        out.push(Rect {
            top_left: Vector2 {
                x: min_x[i],
                y: min_y[i],
            },
            bottom_right: Vector2 {
                x: max_x[i],
                y: max_y[i],
            },
        })
    }
    Ok(out)
}

fn chomp_color3_u8<R: Read>(reader: &mut R) -> Result<Color3Uint8> {
    Ok(Color3Uint8 {
        r: chomp_u8(reader)?,
        g: chomp_u8(reader)?,
        b: chomp_u8(reader)?,
    })
}

fn make_model(num_classes: usize, num_instances: usize, blocks: Vec<Block>) -> Result<RbxModel> {
    let mut meta = HashMap::new();
    let mut shared = Vec::new();
    let mut classes = HashMap::new();
    let mut instances = HashMap::new();
    let mut raw_properties: HashMap<_, HashMap<_, _>> = HashMap::new();
    let mut parent_info = HashMap::new();
    let mut child_info: HashMap<_, Vec<_>> = HashMap::new();

    // Extract data from binary blocks. This is a sad in terms of number of maps, but the cleanest
    // way I could think of.
    for b in blocks {
        match b {
            Block::Meta(data) => meta.extend(data),
            Block::SharedStr(blobs) => shared.extend(blobs),
            Block::Instance {
                class_name,
                index,
                is_service: _,
                instance_ids,
            } => {
                for i in &instance_ids {
                    instances.insert(
                        *i,
                        (
                            class_name.clone(),
                            Rc::new(RefCell::new(Instance::uninit())),
                        ),
                    );
                }

                classes.insert(index, instance_ids);
            }
            Block::Property {
                class_index,
                property_name,
                properties,
            } => {
                let ids = classes
                    .get(&class_index)
                    .ok_or(Error::UnknownClass(class_index))?;

                for (id, prop) in ids.iter().zip(properties) {
                    raw_properties
                        .entry(*id)
                        .or_default()
                        .insert(property_name.clone(), prop);
                }
            }
            Block::Parent {
                instance_referents,
                parent_referents,
            } => {
                for (child_id, parent_id) in instance_referents
                    .into_iter()
                    .zip(parent_referents.into_iter())
                {
                    parent_info.insert(child_id, parent_id);
                    child_info.entry(parent_id).or_default().push(child_id);
                }
            }
            Block::End => continue,
        }
    }

    assert_eq!(classes.len(), num_classes);
    assert_eq!(instances.len(), num_instances);

    // Do reference resolution, and populate information
    let instances = instances
        .iter()
        .map(|(id, (class_name, inst))| {
            let parent_id = parent_info.remove(id).ok_or(Error::UnknownInstance(*id))?;

            let parent = if parent_id != -1 {
                Rc::downgrade(
                    &instances
                        .get(&parent_id)
                        .ok_or(Error::UnknownInstance(parent_id))?
                        .1,
                )
            } else {
                Weak::default()
            };

            let children = child_info
                .remove(id)
                .unwrap_or_default()
                .into_iter()
                .map(|child_id| {
                    Ok(Rc::clone(
                        &instances
                            .get(&child_id)
                            .ok_or(Error::UnknownInstance(child_id))?
                            .1,
                    ))
                })
                .collect::<Result<Vec<_>>>()?;

            let raw_props = raw_properties
                .remove(id)
                .ok_or(Error::UnknownInstance(*id))?;

            let props = raw_props
                .into_iter()
                .map(|(name, value)| {
                    let prop = match value {
                        RawProperty::RawString(blob) => String::from_utf8(blob)
                            .map(Property::TextString)
                            .unwrap_or_else(|err| Property::BinaryString(err.into_bytes())),
                        RawProperty::SharedString(shared_id) => {
                            let blob = &shared[shared_id as usize];
                            String::from_utf8(blob.clone())
                                .map(Property::TextString)
                                .unwrap_or_else(|err| Property::BinaryString(err.into_bytes()))
                        }
                        RawProperty::InstanceRef(ref_id) => {
                            let weak = Rc::downgrade(
                                &instances
                                    .get(&ref_id)
                                    .ok_or(Error::UnknownInstance(ref_id))?
                                    .1,
                            );
                            Property::InstanceRef(weak)
                        }
                        prop => prop.into_real(),
                    };

                    Ok((name, prop))
                })
                .collect::<Result<_>>()?;

            {
                let mut mut_inst = inst.borrow_mut();
                mut_inst.parent = parent;
                mut_inst.children = children;
                mut_inst.kind = make_kind(class_name, props)?;
            }

            Ok(inst)
        })
        .collect::<Result<Vec<_>>>()?;

    // Figure out our root instances
    let roots = instances
        .into_iter()
        .filter(|inst| inst.borrow().parent.ptr_eq(&Weak::default()))
        .map(|inst| Rc::clone(inst))
        .collect::<Vec<_>>();

    Ok(RbxModel { meta, roots })
}

pub struct Deserializer<R> {
    reader: R,
    blocks: Vec<Block>,
}

impl<'de, R: Read> Deserializer<R> {
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer {
            reader,
            blocks: Default::default(),
        }
    }

    pub fn deserialize(mut self) -> Result<RbxModel> {
        let mut magic = [0; 16];
        self.reader.read_exact(&mut magic)?;

        if magic
            != [
                0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x21, 0x89, 0xFF, 0x0D, 0x0A, 0x1A, 0x0A,
                0x00, 0x00,
            ]
        {
            return Err(Error::BadMagic);
        }

        let num_classes = chomp_i32_raw(&mut self.reader)?;
        let num_instances = chomp_i32_raw(&mut self.reader)?;

        let unknown = (
            chomp_i32_raw(&mut self.reader)?,
            chomp_i32_raw(&mut self.reader)?,
        );
        assert_eq!(unknown, (0, 0));

        loop {
            let block = self.chomp_block()?;
            match block {
                Block::End => break,
                _ => self.blocks.push(block),
            }
        }

        let mut magic_end = [0; 9];
        self.reader.read_exact(&mut magic_end)?;

        if magic_end != [0x3C, 0x2F, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x3E] {
            return Err(Error::BadMagic);
        }

        // println!("Unique Insts: {:?}, Total Insts: {:?}, Blocks: {:#?}", unique_tys, total_objs, self.blocks);

        make_model(num_classes as usize, num_instances as usize, self.blocks)
    }

    fn chomp_block(&mut self) -> Result<Block> {
        let name = self.chomp_blockname()?;

        if name == "END" {
            let mut end_data = [0; 12];
            self.reader.read_exact(&mut end_data)?;
            assert_eq!(end_data, [0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0]);
            return Ok(Block::End);
        }

        let data = self.chomp_lz4()?;
        let block_reader = &mut (&data as &[u8]);

        // println!("Data: {:?}", data);

        match &*name {
            "SSTR" => {
                let unknown = chomp_i32_raw(block_reader)?;
                assert_eq!(unknown, 0);
                let num_strs = chomp_i32_raw(block_reader)?;

                let mut blobs = Vec::with_capacity(num_strs as usize);
                for _ in 0..num_strs {
                    let unknown1 = chomp_i32_raw(block_reader)?;
                    let unknown2 = chomp_i32_raw(block_reader)?;
                    let unknown3 = chomp_i32_raw(block_reader)?;
                    let unknown4 = chomp_i32_raw(block_reader)?;
                    assert_eq!(unknown1, 0);
                    assert_eq!(unknown2, 0);
                    assert_eq!(unknown3, 0);
                    assert_eq!(unknown4, 0);

                    blobs.push(chomp_binary_string(block_reader)?);
                }

                Ok(Block::SharedStr(blobs))
            }
            "META" => {
                let num_pairs = chomp_i32_raw(block_reader)?;
                let mut map = HashMap::new();
                for _ in 0..num_pairs {
                    let key = chomp_string(block_reader)?;
                    let value = chomp_string(block_reader)?;
                    map.insert(key, value);
                }

                Ok(Block::Meta(map))
            }
            "INST" => {
                let index = chomp_i32_raw(block_reader)?;
                let class_name = chomp_string(block_reader)?;
                let is_service = chomp_bool(block_reader)?;
                let instance_count = chomp_i32_raw(block_reader)?;
                let mut instance_ids =
                    chomp_interleaved_i32_transformed(block_reader, instance_count as usize)?;

                make_cumulative(&mut instance_ids);

                Ok(Block::Instance {
                    index,
                    class_name,
                    is_service,
                    instance_ids,
                })
            }
            "PROP" => {
                let class_index = chomp_i32_raw(block_reader)?;
                let property_name = chomp_string(block_reader)?;
                let prop_ty = chomp_u8(block_reader)?;

                let num_props = self
                    .blocks
                    .iter()
                    .find(|b| {
                        if let Block::Instance { index, .. } = b {
                            *index == class_index
                        } else {
                            false
                        }
                    })
                    .map(|b| {
                        if let Block::Instance { instance_ids, .. } = b {
                            instance_ids.len()
                        } else {
                            unreachable!()
                        }
                    })
                    .ok_or(Error::UnknownClass(class_index))?;

                // println!("Property Name: {}\nProperty Ty: {}", property_name, prop_ty);

                let mut properties = Vec::with_capacity(num_props);
                for _ in 0..num_props {
                    let prop = match prop_ty {
                        1 => RawProperty::RawString(chomp_binary_string(block_reader)?),
                        2 => RawProperty::Bool(chomp_bool(block_reader)?),
                        3 => RawProperty::Int32(chomp_i32_transformed(block_reader)?),
                        4 => RawProperty::Float(chomp_f32_transformed(block_reader)?),
                        5 => RawProperty::Double(chomp_f64(block_reader)?),
                        6 => {
                            properties.extend(
                                chomp_udims(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::UDim),
                            );
                            break;
                        }
                        7 => {
                            properties.extend(
                                chomp_udim2s(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::UDim2),
                            );
                            break;
                        }
                        8 => {
                            properties.extend(
                                chomp_rays(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Ray),
                            );
                            break;
                        }
                        9 => RawProperty::Face(chomp_face(block_reader)?),
                        10 => RawProperty::Axis(chomp_axis(block_reader)?),
                        11 => {
                            properties.extend(
                                chomp_brick_colors(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::BrickColor),
                            );
                            break;
                        }
                        12 => RawProperty::Color3(chomp_color3(block_reader)?),
                        13 => RawProperty::Vector2(chomp_vector2(block_reader)?),
                        14 => RawProperty::Vector3(chomp_vector3(block_reader)?),
                        16 => {
                            properties.extend(
                                chomp_cframes(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::CFrame),
                            );
                            break;
                        }
                        17 => todo!("Quaternions not yet supported"),
                        18 => {
                            properties.extend(
                                chomp_interleaved_i32_raw(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Enum),
                            );
                            break;
                        }
                        19 => {
                            let mut ids = chomp_interleaved_i32_raw(block_reader, num_props)?;
                            make_cumulative(&mut ids);
                            properties.extend(ids.into_iter().map(RawProperty::InstanceRef));
                            break;
                        }
                        20 => RawProperty::Vector3Int16(chomp_vector3_i16(block_reader)?),
                        21 => RawProperty::NumberSequence(chomp_number_sequence(block_reader)?),
                        22 => RawProperty::ColorSequence(chomp_color_sequence(block_reader)?),
                        23 => RawProperty::NumberRange(chomp_number_range(block_reader)?),
                        24 => {
                            properties.extend(
                                chomp_rects(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::Rect),
                            );
                            break;
                        }
                        25 => RawProperty::CustomPhysicalProperties(chomp_bool(block_reader)?),
                        26 => RawProperty::Color3Uint8(chomp_color3_u8(block_reader)?),
                        27 => RawProperty::Int64(chomp_i64(block_reader)?),
                        28 => {
                            properties.extend(
                                chomp_interleaved_i32_raw(block_reader, num_props)?
                                    .into_iter()
                                    .map(RawProperty::SharedString),
                            );
                            break;
                        }
                        _ => {
                            println!("Unknown Property Data:\nClass: {}\nProperty: {}\nName: {}\nData: {:?}", class_index, prop_ty, property_name, data);
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

                Ok(Block::Property {
                    class_index,
                    property_name,
                    properties,
                })
            }
            "PRNT" => {
                let unknown = chomp_u8(block_reader)?;
                assert_eq!(unknown, 0);
                let len = chomp_i32_raw(block_reader)?;
                let mut instance_referents =
                    chomp_interleaved_i32_transformed(block_reader, len as usize)?;
                let mut parent_referents =
                    chomp_interleaved_i32_transformed(block_reader, len as usize)?;

                make_cumulative(&mut instance_referents);
                make_cumulative(&mut parent_referents);

                Ok(Block::Parent {
                    instance_referents,
                    parent_referents,
                })
            }
            _ => Err(Error::UnknownBlock(name)),
        }
    }

    fn chomp_blockname(&mut self) -> Result<String> {
        let mut data = [0; 4];
        self.reader.read_exact(&mut data)?;

        let first_zero = data.iter().copied().position(|b| b == 0).unwrap_or(4) as usize;

        Ok(std::str::from_utf8(&data[..first_zero])
            .map_err(|_| Error::InvalidString)?
            .to_owned())
    }

    fn chomp_lz4(&mut self) -> Result<Vec<u8>> {
        let compressed = chomp_i32_raw(&mut self.reader)?;
        let uncompressed = chomp_i32_raw(&mut self.reader)?;

        assert_eq!(chomp_i32_raw(&mut self.reader)?, 0i32);

        let mut data = vec![0; compressed as usize];
        self.reader.read_exact(&mut data)?;

        let out =
            lz4::block::decompress(&data, Some(uncompressed)).map_err(|_| Error::InvalidLz4)?;

        assert_eq!(out.len(), uncompressed as usize);

        Ok(out)
    }
}

pub fn from_reader<R: Read>(reader: R) -> Result<RbxModel> {
    Deserializer::new(reader).deserialize()
}

pub fn from_file<P: AsRef<Path>>(path: P) -> Result<RbxModel> {
    from_reader(std::fs::File::open(path)?)
}

pub fn from_bytes(bytes: &[u8]) -> Result<RbxModel> {
    from_reader(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_i32() {
        assert_eq!(decode_i32(0b0000_0000_0000_0000), 0);
        assert_eq!(decode_i32(0b0000_0000_0000_0001), -1);
        assert_eq!(decode_i32(0b0000_0000_0000_0100), 2);
        assert_eq!(decode_i32(0b1000_0000_0000_0000), 16384);
        assert_eq!(decode_i32(0b1000_0000_0000_0001), -16385);
    }

    #[test]
    fn test_make_cumulative() {
        let mut array = [0, 1, 1, 5];
        make_cumulative(&mut array);
        assert_eq!(array, [0, 1, 2, 7]);
    }

    #[test]
    fn test_brick() {
        from_file("./examples/BrickBase.rbxm").unwrap();
    }

    #[test]
    fn test_terrain() {
        from_file("./examples/TerrainBase.rbxm").unwrap();
    }

    #[test]
    fn test_cframes() {
        from_file("./examples/CFrameTest.rbxm").unwrap();
    }

    #[test]
    fn test_instances() {
        from_file("./examples/InstanceTest.rbxm").unwrap();
    }
}
