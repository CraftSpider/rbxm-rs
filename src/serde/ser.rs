use crate::model::{
    Axis, Color3, Color3Uint8, ColorSequence, Face, Instance, NumberRange, NumberSequence,
    Property, RbxModel, Vector2, Vector3, Vector3Int16,
};
use crate::serde::internal::{Block, RawProperty};
use crate::serde::Result;

use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::rc::Rc;

macro_rules! float_match {
    ($var:expr, $($array:expr; $num:literal),+) => {
        if false { 0 }
        $(else if $var == $array {
            $num
        })*
        else {
            0
        };
    }
}

pub struct Serializer<W> {
    writer: W,
}

fn encode_i32(mut val: i32) -> u32 {
    let sign = (val < 0) as u32;
    if val < 0 {
        val = -val;
        val -= 1;
    }
    let mut out = u32::from_ne_bytes(val.to_ne_bytes());
    out <<= 1;
    out |= sign;
    out
}

fn encode_f32(val: f32) -> u32 {
    let mut raw = u32::from_ne_bytes(val.to_ne_bytes());

    let sign = (raw & 0b1000_0000_0000_0000 > 0) as u32;
    raw <<= 1;
    raw |= sign;
    raw
}

fn make_cumulative(mut slice: &mut [i32]) {
    for _ in (1..slice.len()).rev() {
        let (previous, last) = slice.split_at_mut(slice.len() - 1);
        last[0] -= previous.last().unwrap();
        slice = previous;
    }
}

fn write_bool<W: Write>(writer: &mut W, val: bool) -> Result<()> {
    writer.write_all(&[(val as u8)])?;
    Ok(())
}

fn write_u8<W: Write>(writer: &mut W, val: u8) -> Result<()> {
    writer.write_all(&[val])?;
    Ok(())
}

fn write_i16<W: Write>(writer: &mut W, val: i16) -> Result<()> {
    writer.write_all(&val.to_le_bytes())?;
    Ok(())
}

fn write_i32_raw<W: Write>(writer: &mut W, val: i32) -> Result<()> {
    writer.write_all(&val.to_le_bytes())?;
    Ok(())
}

fn write_i32_transformed<W: Write>(writer: &mut W, val: i32) -> Result<()> {
    writer.write_all(&encode_i32(val).to_be_bytes())?;
    Ok(())
}

fn write_i64<W: Write>(writer: &mut W, val: i64) -> Result<()> {
    writer.write_all(&val.to_le_bytes())?;
    Ok(())
}

fn write_f32_raw<W: Write>(writer: &mut W, val: f32) -> Result<()> {
    writer.write_all(&val.to_le_bytes())?;
    Ok(())
}

fn write_f32_transformed<W: Write>(writer: &mut W, val: f32) -> Result<()> {
    writer.write_all(&encode_f32(val).to_be_bytes())?;
    Ok(())
}

fn write_f64<W: Write>(writer: &mut W, val: f64) -> Result<()> {
    writer.write_all(&val.to_le_bytes())?;
    Ok(())
}

fn write_string<W: Write>(writer: &mut W, val: String) -> Result<()> {
    write_i32_raw(writer, val.len() as i32)?;
    writer.write_all(&val.into_bytes())?; // TODO: Should unicode be valid in this?
    Ok(())
}

fn write_binary_string<W: Write>(writer: &mut W, val: &[u8]) -> Result<()> {
    write_i32_raw(writer, val.len() as i32)?;
    writer.write_all(val)?;
    Ok(())
}

fn write_interleaved_i32_raw<W: Write>(writer: &mut W, val: &[i32]) -> Result<()> {
    let mut data = vec![0; val.len() * 4];

    for i in 0..val.len() {
        let bytes = val[i].to_be_bytes();
        for j in 0..4 {
            data[i + j * val.len()] = bytes[j];
        }
    }

    writer.write_all(&data)?;
    Ok(())
}

fn write_interleaved_i32_transformed<W: Write>(writer: &mut W, val: &[i32]) -> Result<()> {
    let mut data = vec![0; val.len() * 4];

    for i in 0..val.len() {
        let bytes = encode_i32(val[i]).to_be_bytes();
        for j in 0..4 {
            data[i + j * val.len()] = bytes[j];
        }
    }

    writer.write_all(&data)?;
    Ok(())
}

fn write_interleaved_f32<W: Write>(writer: &mut W, val: &[f32]) -> Result<()> {
    let mut data = vec![0; val.len() * 4];

    for i in 0..val.len() {
        let bytes = encode_f32(val[i]).to_be_bytes();
        for j in 0..4 {
            data[i + j * val.len()] = bytes[j];
        }
    }

    writer.write_all(&data)?;
    Ok(())
}

fn write_udims<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut scales = Vec::with_capacity(properties.len());
    let mut offsets = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::UDim(udim) = prop {
            scales.push(udim.scale);
            offsets.push(udim.offset);
        } else {
            unreachable!()
        }
    }

    write_interleaved_f32(writer, &scales)?;
    write_interleaved_i32_raw(writer, &offsets)?;
    Ok(())
}

fn write_udim2s<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut x_scales = Vec::with_capacity(properties.len());
    let mut y_scales = Vec::with_capacity(properties.len());
    let mut x_offsets = Vec::with_capacity(properties.len());
    let mut y_offsets = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::UDim2(udim) = prop {
            x_scales.push(udim.x_scale);
            y_scales.push(udim.y_scale);
            x_offsets.push(udim.x_offset);
            y_offsets.push(udim.y_offset);
        } else {
            unreachable!()
        }
    }

    write_interleaved_f32(writer, &x_scales)?;
    write_interleaved_f32(writer, &y_scales)?;
    write_interleaved_i32_raw(writer, &x_offsets)?;
    write_interleaved_i32_raw(writer, &y_offsets)?;
    Ok(())
}

fn write_rays<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut x_origs = Vec::with_capacity(properties.len());
    let mut y_origs = Vec::with_capacity(properties.len());
    let mut z_origs = Vec::with_capacity(properties.len());
    let mut x_dirs = Vec::with_capacity(properties.len());
    let mut y_dirs = Vec::with_capacity(properties.len());
    let mut z_dirs = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::Ray(ray) = prop {
            x_origs.push(ray.origin.x);
            y_origs.push(ray.origin.y);
            z_origs.push(ray.origin.z);
            x_dirs.push(ray.direction.x);
            y_dirs.push(ray.direction.y);
            z_dirs.push(ray.direction.z);
        } else {
            unreachable!()
        }
    }

    write_interleaved_f32(writer, &x_origs)?;
    write_interleaved_f32(writer, &y_origs)?;
    write_interleaved_f32(writer, &z_origs)?;
    write_interleaved_f32(writer, &x_dirs)?;
    write_interleaved_f32(writer, &y_dirs)?;
    write_interleaved_f32(writer, &z_dirs)?;
    Ok(())
}

fn write_face<W: Write>(writer: &mut W, val: &Face) -> Result<()> {
    let mut data = 0;

    if val.front {
        data |= 0b00000001;
    }
    if val.bottom {
        data |= 0b00000010;
    }
    if val.left {
        data |= 0b00000100;
    }
    if val.back {
        data |= 0b00001000;
    }
    if val.top {
        data |= 0b00010000;
    }
    if val.right {
        data |= 0b00100000;
    }

    write_u8(writer, data)?;
    Ok(())
}

fn write_axis<W: Write>(writer: &mut W, val: &Axis) -> Result<()> {
    let mut data = 0;

    if val.x {
        data |= 0b100
    }
    if val.y {
        data |= 0b010
    }
    if val.z {
        data |= 0b001
    }

    write_u8(writer, data)?;
    Ok(())
}

fn write_brick_colors<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut indices = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::BrickColor(color) = prop {
            indices.push(color.index);
        } else {
            unreachable!()
        }
    }

    write_interleaved_i32_raw(writer, &indices)?;
    Ok(())
}

fn write_color3_raw<W: Write>(writer: &mut W, val: &Color3) -> Result<()> {
    write_f32_raw(writer, val.r)?;
    write_f32_raw(writer, val.g)?;
    write_f32_raw(writer, val.b)?;
    Ok(())
}

fn write_color3<W: Write>(writer: &mut W, val: &Color3) -> Result<()> {
    write_f32_transformed(writer, val.r)?;
    write_f32_transformed(writer, val.g)?;
    write_f32_transformed(writer, val.b)?;
    Ok(())
}

fn write_vector2<W: Write>(writer: &mut W, val: &Vector2) -> Result<()> {
    write_f32_transformed(writer, val.x)?;
    write_f32_transformed(writer, val.y)?;
    Ok(())
}

fn write_vector3<W: Write>(writer: &mut W, val: &Vector3) -> Result<()> {
    write_f32_transformed(writer, val.x)?;
    write_f32_transformed(writer, val.y)?;
    write_f32_transformed(writer, val.z)?;
    Ok(())
}

fn write_cframes<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut angles = Vec::new();
    let mut positions = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::CFrame(cframe) = prop {
            let angle_ty = float_match! {
                cframe.angle,
                [[1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, 1f32]]; 2,
                [[1f32, 0f32, 0f32], [0f32, 0f32, -1f32], [0f32, 1f32, 0f32]]; 3,
                [[1f32, 0f32, 0f32], [0f32, -1f32, 0f32], [0f32, 0f32, -1f32]]; 5,
                [[1f32, 0f32, 0f32], [0f32, 0f32, 1f32], [0f32, -1f32, 0f32]]; 6,
                [[0f32, 1f32, 0f32], [1f32, 0f32, 0f32], [0f32, 0f32, -1f32]]; 7,
                [[0f32, 0f32, 1f32], [1f32, 0f32, 0f32], [0f32, 1f32, 0f32]]; 9,
                [[0f32, -1f32, 0f32], [1f32, 0f32, 0f32], [0f32, 0f32, 1f32]]; 10,
                [[0f32, 0f32, -1f32], [1f32, 0f32, 0f32], [0f32, -1f32, 0f32]]; 12,
                [[0f32, 1f32, 0f32], [0f32, 0f32, 1f32], [1f32, 0f32, 0f32]]; 13,
                [[0f32, 0f32, -1f32], [0f32, 1f32, 0f32], [1f32, 0f32, 0f32]]; 14,
                [[0f32, -1f32, 0f32], [0f32, 0f32, -1f32], [1f32, 0f32, 0f32]]; 16,
                [[0f32, 0f32, 1f32], [0f32, -1f32, 0f32], [1f32, 0f32, 0f32]]; 17,
                [[-1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, -1f32]]; 20,
                [[-1f32, 0f32, 0f32], [0f32, 0f32, 1f32], [0f32, 1f32, 0f32]]; 21,
                [[-1f32, 0f32, 0f32], [0f32, -1f32, 0f32], [0f32, 0f32, 1f32]]; 23,
                [[-1f32, 0f32, 0f32], [0f32, 0f32, -1f32], [0f32, -1f32, 0f32]]; 24,
                [[0f32, 1f32, 0f32], [-1f32, 0f32, 0f32], [0f32, 0f32, 1f32]]; 25,
                [[0f32, 0f32, -1f32], [-1f32, 0f32, 0f32], [0f32, 1f32, 0f32]]; 27,
                [[0f32, -1f32, 0f32], [-1f32, 0f32, 0f32], [0f32, 0f32, -1f32]]; 28,
                [[0f32, 0f32, 1f32], [-1f32, 0f32, 0f32], [0f32, -1f32, 0f32]]; 30,
                [[0f32, 1f32, 0f32], [0f32, 0f32, -1f32], [-1f32, 0f32, 0f32]]; 31,
                [[0f32, 0f32, 1f32], [0f32, 1f32, 0f32], [-1f32, 0f32, 0f32]]; 32,
                [[0f32, -1f32, 0f32], [0f32, 0f32, 1f32], [-1f32, 0f32, 0f32]]; 34,
                [[0f32, 0f32, -1f32], [0f32, -1f32, 0f32], [-1f32, 0f32, 0f32]]; 35
            };

            angles.push(angle_ty);

            if angle_ty != 0 {
                for i in cframe.angle.iter() {
                    for j in i {
                        write_f32_raw(&mut angles, *j)?;
                    }
                }
            }

            positions.push(&cframe.position);
        } else {
            unreachable!()
        }
    }

    writer.write_all(&angles)?;
    for pos in positions {
        write_vector3(writer, pos)?;
    }
    Ok(())
}

fn write_enums<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut vals = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::Enum(val) = prop {
            vals.push(*val);
        } else {
            unreachable!()
        }
    }

    write_interleaved_i32_raw(writer, &vals)?;
    Ok(())
}

fn write_instance_refs<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut vals = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::InstanceRef(val) = prop {
            vals.push(*val);
        } else {
            unreachable!()
        }
    }

    make_cumulative(&mut vals);
    write_interleaved_i32_raw(writer, &vals)?;
    Ok(())
}

fn write_vector3_i16<W: Write>(writer: &mut W, val: &Vector3Int16) -> Result<()> {
    write_i16(writer, val.x)?;
    write_i16(writer, val.y)?;
    write_i16(writer, val.z)?;
    Ok(())
}

fn write_number_sequence<W: Write>(writer: &mut W, val: &NumberSequence) -> Result<()> {
    write_i32_raw(writer, val.keypoints.len() as i32)?;
    for i in &val.keypoints {
        write_f32_raw(writer, i.time)?;
        write_f32_raw(writer, i.value)?;
        write_f32_raw(writer, i.envelope)?;
    }
    Ok(())
}

fn write_color_sequence<W: Write>(writer: &mut W, val: &ColorSequence) -> Result<()> {
    write_i32_raw(writer, val.keypoints.len() as i32)?;
    for i in &val.keypoints {
        write_f32_raw(writer, i.time)?;
        write_color3_raw(writer, &i.color)?;
        write_f32_raw(writer, i.envelope)?;
    }
    Ok(())
}

fn write_number_range<W: Write>(writer: &mut W, val: &NumberRange) -> Result<()> {
    write_f32_raw(writer, val.low)?;
    write_f32_raw(writer, val.high)?;
    Ok(())
}

fn write_rects<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut min_x = Vec::with_capacity(properties.len());
    let mut min_y = Vec::with_capacity(properties.len());
    let mut max_x = Vec::with_capacity(properties.len());
    let mut max_y = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::Rect(rect) = prop {
            min_x.push(rect.top_left.x);
            min_y.push(rect.top_left.y);
            max_x.push(rect.bottom_right.x);
            max_y.push(rect.bottom_right.y);
        } else {
            unreachable!()
        }
    }

    write_interleaved_f32(writer, &min_x)?;
    write_interleaved_f32(writer, &min_y)?;
    write_interleaved_f32(writer, &max_x)?;
    write_interleaved_f32(writer, &max_y)?;
    Ok(())
}

fn write_color3_u8<W: Write>(writer: &mut W, val: &Color3Uint8) -> Result<()> {
    write_u8(writer, val.r)?;
    write_u8(writer, val.g)?;
    write_u8(writer, val.b)?;
    Ok(())
}

fn write_shared_strings<W: Write>(writer: &mut W, properties: &[RawProperty]) -> Result<()> {
    let mut indices = Vec::with_capacity(properties.len());

    for prop in properties {
        if let RawProperty::SharedString(index) = prop {
            indices.push(*index);
        } else {
            unreachable!()
        }
    }

    write_interleaved_i32_raw(writer, &indices)?;
    Ok(())
}

fn walk_children(collection: &mut Vec<Rc<RefCell<Instance>>>, todo: &[Rc<RefCell<Instance>>]) {
    for i in todo {
        collection.push(i.clone());
        walk_children(collection, &i.borrow().children)
    }
}

fn break_model(model: &RbxModel) -> (i32, i32, Vec<Block>) {
    let mut insts = Vec::new();
    #[allow(clippy::mutable_key_type)]
    let mut inst_to_id = HashMap::new();

    walk_children(&mut insts, &model.roots);

    for (index, inst) in insts.iter().enumerate() {
        inst_to_id.insert(Rc::as_ptr(inst), index as i32);
    }

    let mut inst_blocks = HashMap::new();
    let mut prop_blocks = HashMap::new();
    let mut parents = HashMap::new();

    for (index, inst) in insts.iter().enumerate() {
        let next_index = inst_blocks.len();
        let class_name = inst.borrow().kind.clone();

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

        for (prop_name, prop_value) in &inst.borrow().properties {
            let prop_block = prop_blocks
                .entry((class_index, prop_name.clone()))
                .or_insert(Block::Property {
                    class_index,
                    property_name: prop_name.clone(),
                    properties: vec![],
                });

            if let Block::Property { properties, .. } = prop_block {
                let raw = match prop_value {
                    // TODO: BinaryString/RawString sometimes should be converted to a shared string
                    Property::BinaryString(blob) => RawProperty::RawString(blob.clone()),
                    Property::TextString(str) => RawProperty::RawString(str.clone().into_bytes()),
                    Property::InstanceRef(val) => {
                        RawProperty::InstanceRef(inst_to_id[&Rc::as_ptr(&val.upgrade().unwrap())])
                    }
                    prop => RawProperty::from_real(prop.clone()),
                };
                properties.push(raw)
            } else {
                unreachable!()
            }
        }

        let parent_index = match inst.borrow().parent.upgrade() {
            Some(parent) => inst_to_id[&Rc::as_ptr(&parent)],
            None => -1,
        };
        parents.insert(index as i32, parent_index);
    }

    let num_classes = inst_blocks.len();
    let num_insts = insts.len();

    let child_ref = parents.keys().copied().collect::<Vec<_>>();
    let parent_ref = parents.values().copied().collect::<Vec<_>>();

    let mut out = vec![Block::Meta(model.meta.clone())];
    // out.push(Block::SharedStr(Vec::new())); // TODO: Convert things to shared strings if needed
    out.extend(inst_blocks.into_iter().map(|(_, block)| block));
    out.extend(prop_blocks.into_iter().map(|(_, block)| block));
    out.push(Block::Parent {
        instance_referents: child_ref,
        parent_referents: parent_ref,
    });

    (num_classes as i32, num_insts as i32, out)
}

impl<W: Write> Serializer<W> {
    pub fn new(writer: W) -> Serializer<W> {
        Serializer { writer }
    }

    pub fn serialize(mut self, model: &RbxModel) -> Result<()> {
        let (num_classes, num_insts, blocks) = break_model(model);

        // Magic start value
        self.writer.write_all(&[
            0x3C, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x21, 0x89, 0xFF, 0x0D, 0x0A, 0x1A, 0x0A,
            0x00, 0x00,
        ])?;

        write_i32_raw(&mut self.writer, num_classes)?;
        write_i32_raw(&mut self.writer, num_insts)?;
        write_i32_raw(&mut self.writer, 0)?;
        write_i32_raw(&mut self.writer, 0)?;

        for b in blocks {
            self.write_block(b)?;
        }

        // Magic end value
        self.writer
            .write_all(&[0x3C, 0x2F, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x3E])?;

        Ok(())
    }

    fn write_block(&mut self, block: Block) -> Result<()> {
        let mut out_buffer = Vec::new();
        let writer = &mut out_buffer;

        let block_name = match block {
            Block::Meta(attrs) => {
                write_i32_raw(writer, attrs.len() as i32)?;
                for (key, value) in attrs {
                    write_string(writer, key)?;
                    write_string(writer, value)?;
                }
                b"META"
            }
            Block::SharedStr(strs) => {
                write_i32_raw(writer, 0)?;
                write_i32_raw(writer, strs.len() as i32)?;

                for blob in strs {
                    write_i32_raw(writer, 0)?;
                    write_i32_raw(writer, 0)?;
                    write_i32_raw(writer, 0)?;
                    write_i32_raw(writer, 0)?;

                    write_binary_string(writer, &blob)?;
                }

                b"SSTR"
            }
            Block::Instance {
                index,
                is_service,
                class_name,
                mut instance_ids,
            } => {
                write_i32_raw(writer, index)?;
                write_string(writer, class_name)?;
                write_bool(writer, is_service)?;
                write_i32_raw(writer, instance_ids.len() as i32)?;

                make_cumulative(&mut instance_ids);
                write_interleaved_i32_transformed(writer, &instance_ids)?;

                b"INST"
            }
            Block::Property {
                class_index,
                property_name,
                properties,
            } => {
                write_i32_raw(writer, class_index)?;
                write_string(writer, property_name)?;

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
                    RawProperty::SharedString(..) => 28,
                };

                write_u8(writer, prop_ty)?;

                for prop in &properties {
                    match prop {
                        RawProperty::RawString(blob) => write_binary_string(writer, blob)?,
                        RawProperty::Bool(val) => write_bool(writer, *val)?,
                        RawProperty::Int32(val) => write_i32_transformed(writer, *val)?,
                        RawProperty::Float(val) => write_f32_transformed(writer, *val)?,
                        RawProperty::Double(val) => write_f64(writer, *val)?,
                        RawProperty::UDim(..) => {
                            write_udims(writer, &properties)?;
                            break;
                        }
                        RawProperty::UDim2(..) => {
                            write_udim2s(writer, &properties)?;
                            break;
                        }
                        RawProperty::Ray(..) => {
                            write_rays(writer, &properties)?;
                            break;
                        }
                        RawProperty::Face(val) => write_face(writer, val)?,
                        RawProperty::Axis(val) => write_axis(writer, val)?,
                        RawProperty::BrickColor(..) => {
                            write_brick_colors(writer, &properties)?;
                            break;
                        }
                        RawProperty::Color3(val) => write_color3(writer, val)?,
                        RawProperty::Vector2(val) => write_vector2(writer, val)?,
                        RawProperty::Vector3(val) => write_vector3(writer, val)?,
                        RawProperty::CFrame(..) => {
                            write_cframes(writer, &properties)?;
                            break;
                        }
                        RawProperty::Quaternion => todo!("Quaternions not yet supported"),
                        RawProperty::Enum(..) => {
                            write_enums(writer, &properties)?;
                            break;
                        }
                        RawProperty::InstanceRef(..) => {
                            write_instance_refs(writer, &properties)?;
                            break;
                        }
                        RawProperty::Vector3Int16(val) => write_vector3_i16(writer, val)?,
                        RawProperty::NumberSequence(val) => write_number_sequence(writer, val)?,
                        RawProperty::ColorSequence(val) => write_color_sequence(writer, val)?,
                        RawProperty::NumberRange(val) => write_number_range(writer, val)?,
                        RawProperty::Rect(..) => {
                            write_rects(writer, &properties)?;
                            break;
                        }
                        RawProperty::CustomPhysicalProperties(val) => write_bool(writer, *val)?,
                        RawProperty::Color3Uint8(val) => write_color3_u8(writer, val)?,
                        RawProperty::Int64(val) => write_i64(writer, *val)?,
                        RawProperty::SharedString(..) => {
                            write_shared_strings(writer, &properties)?;
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

                write_u8(writer, 0)?;
                write_i32_raw(writer, instance_referents.len() as i32)?;
                write_interleaved_i32_transformed(writer, &instance_referents)?;
                write_interleaved_i32_transformed(writer, &parent_referents)?;
                b"PRNT"
            }
            Block::End => b"END\0",
        };

        let compressed_data = lz4::block::compress(&out_buffer, None, false).unwrap();

        self.writer.write_all(block_name)?;
        write_i32_raw(&mut self.writer, compressed_data.len() as i32)?;
        write_i32_raw(&mut self.writer, out_buffer.len() as i32)?;
        write_i32_raw(&mut self.writer, 0)?;
        self.writer.write_all(&compressed_data)?;

        Ok(())
    }
}

pub fn to_writer<W: Write>(writer: W, model: &RbxModel) -> Result<()> {
    Serializer::new(writer).serialize(model)
}

pub fn to_file<P: AsRef<Path>>(path: P, model: &RbxModel) -> Result<()> {
    to_writer(std::fs::File::create(path)?, model)
}

pub fn to_bytes(model: &RbxModel) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    to_writer(&mut out, model)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_i32() {
        assert_eq!(encode_i32(0), 0b0000_0000_0000_0000);
        assert_eq!(encode_i32(-1), 0b0000_0000_0000_0001);
        assert_eq!(encode_i32(2), 0b0000_0000_0000_0100);
        assert_eq!(encode_i32(16384), 0b1000_0000_0000_0000);
        assert_eq!(encode_i32(-16385), 0b1000_0000_0000_0001);
    }

    #[test]
    fn test_make_cumulative() {
        let mut array = [0, 1, 2, 7];
        make_cumulative(&mut array);
        assert_eq!(array, [0, 1, 1, 5]);
    }

    #[test]
    fn test_reser() {
        let model = crate::serde::from_file("./examples/BrickBase.rbxm").unwrap();

        to_file("./output/BrickBaseOut.rbxm", &model).unwrap();
    }
}
