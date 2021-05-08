//! The deserialization implementation for an RBXM

use crate::model::*;
use crate::serde::internal::{make_kind, RawProperty};
use crate::serde::{Error, Result};

use alloc::collections::BTreeMap;
use alloc::rc::{Rc, Weak};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;

/// A no_std minimal implementation of [`std::io::Read`]
pub trait Read {
    /// Read an exact buffer size
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>;
}

#[cfg(feature = "std")]
impl<T: std::io::Read> Read for T {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        <Self as std::io::Read>::read_exact(self, buf).map_err(Error::IoError)
    }
}

#[cfg(not(feature = "std"))]
impl Read for &[u8] {
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        if buf.len() < self.len() {
            for i in 0..buf.len() {
                buf[i] = self[i];
            }
            *self = &self[buf.len()..];
            Ok(())
        } else {
            Err(Error::IoError())
        }
    }
}

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
    raw ^= sign * (1 << 31);
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

#[inline]
fn chomp_bytes<R: Read, const LEN: usize>(reader: &mut R) -> Result<[u8; LEN]> {
    let mut data = [0; LEN];
    reader.read_exact(&mut data)?;
    Ok(data)
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
            x: UDim {
                scale: x_scale,
                offset: x_offset,
            },
            y: UDim {
                scale: y_scale,
                offset: y_offset,
            },
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

fn chomp_face<R: Read>(reader: &mut R) -> Result<Faces> {
    let data = chomp_u8(reader)?;
    Ok(Faces {
        front: data & 0b00000001 > 0,
        bottom: data & 0b00000010 > 0,
        left: data & 0b00000100 > 0,
        back: data & 0b00001000 > 0,
        top: data & 0b00010000 > 0,
        right: data & 0b00100000 > 0,
    })
}

fn chomp_axis<R: Read>(reader: &mut R) -> Result<Axes> {
    let data = chomp_u8(reader)?;
    Ok(Axes {
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

#[cfg(feature = "mesh-format")]
fn chomp_inner_mesh<R: Read>(reader: &mut R) -> Result<ConvexHull> {
    let unknown_1_len = chomp_i32_raw(reader)?;
    let mut unknown_1 = vec![0; unknown_1_len as usize];
    reader.read_exact(&mut unknown_1)?;

    let unknown_2_len = chomp_i32_raw(reader)?;
    let mut unknown_2 = vec![0; unknown_2_len as usize];
    reader.read_exact(&mut unknown_2)?;

    let vert_len = chomp_i32_raw(reader)? / 3;
    let vert_width = chomp_i32_raw(reader)?;
    assert_eq!(vert_width, 4, "Unexpected vertice width");
    let mut vertices = Vec::with_capacity(vert_len as usize);
    for _ in 0..vert_len {
        vertices.push(Vector3 {
            x: chomp_f32_raw(reader)?,
            y: chomp_f32_raw(reader)?,
            z: chomp_f32_raw(reader)?,
        });
    }

    let faces_len = chomp_i32_raw(reader)? / 3;
    let mut faces = Vec::with_capacity(faces_len as usize);
    for _ in 0..faces_len {
        faces.push((
            chomp_i32_raw(reader)? as usize,
            chomp_i32_raw(reader)? as usize,
            chomp_i32_raw(reader)? as usize,
        ));
    }

    Ok(ConvexHull {
        unknown_1,
        unknown_2,
        vertices,
        faces,
    })
}

#[cfg(feature = "mesh-format")]
pub(crate) fn chomp_mesh<R: Read>(reader: &mut R) -> Result<TriMesh> {
    let magic = chomp_bytes::<_, 6>(reader)?;
    if &magic != b"CSGPHS" {
        return Err(Error::BadMagic);
    }

    let kind = chomp_i32_raw(reader)?;

    match kind {
        0 => {
            let magic = chomp_bytes::<_, 5>(reader)?;
            if &magic != b"BLOCK" {
                Err(Error::BadMagic)
            } else {
                Ok(TriMesh::Box)
            }
        }
        6 => {
            let volume = chomp_f32_raw(reader)?;
            let center_of_gravity = Vector3 {
                x: chomp_f32_raw(reader)?,
                y: chomp_f32_raw(reader)?,
                z: chomp_f32_raw(reader)?,
            };

            let inertia_tensor = {
                let xx = chomp_f32_raw(reader)?;
                let xy = chomp_f32_raw(reader)?;
                let xz = chomp_f32_raw(reader)?;
                let yy = chomp_f32_raw(reader)?;
                let yz = chomp_f32_raw(reader)?;
                let zz = chomp_f32_raw(reader)?;

                [
                    [ xx,  xy,  xz],
                    [-xy,  yy,  yz],
                    [-xz, -yz,  zz],
                ]
            };

            let mut meshes = Vec::new();
            // Read hulls till we run out of reader
            loop {
                match chomp_inner_mesh(reader) {
                    Ok(mesh) => meshes.push(mesh),
                    Err(Error::IoError(..)) => break,
                    Err(e) => return Err(e),
                }
            }

            Ok(TriMesh::Hull {
                volume,
                center_of_gravity,
                inertia_tensor,
                meshes,
            })
        }
        val => Err(Error::UnknownMesh(val))
    }
}

#[derive(Default)]
struct RawInfo {
    meta: BTreeMap<String, String>,
    shared_strs: Vec<Vec<u8>>,
    class_ids: BTreeMap<i32, Vec<i32>>,
    instances: BTreeMap<i32, (String, Rc<RefCell<Instance>>)>,
    raw_props: BTreeMap<i32, BTreeMap<String, RawProperty>>,
    parent_info: BTreeMap<i32, i32>,
    child_info: BTreeMap<i32, Vec<i32>>,
}

/// Necessary state for deserializing a value
pub struct Deserializer<R> {
    reader: R,
    raw_info: RawInfo,
}

impl<'de, R: Read> Deserializer<R> {
    /// Create a new deserializer from a reader and if necessary any other state
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer {
            reader,
            raw_info: RawInfo::default(),
        }
    }

    /// Deserialize a model from the input stream
    pub fn deserialize(mut self) -> Result<RbxModel> {
        let magic = chomp_bytes::<_, 16>(&mut self.reader)?;

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

        while self.chomp_block()? {}

        let magic_end = chomp_bytes::<_, 9>(&mut self.reader)?;

        if magic_end != [0x3C, 0x2F, 0x72, 0x6F, 0x62, 0x6C, 0x6F, 0x78, 0x3E] {
            return Err(Error::BadMagic);
        }

        assert_eq!(self.raw_info.class_ids.len(), num_classes as usize);
        assert_eq!(self.raw_info.instances.len(), num_instances as usize);

        self.make_model()
    }

    fn make_model(self) -> Result<RbxModel> {
        let RawInfo {
            meta,
            shared_strs,
            class_ids: _,
            instances,
            mut raw_props,
            mut parent_info,
            mut child_info,
        } = self.raw_info;

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

                let raw_props = raw_props.remove(id).ok_or(Error::UnknownInstance(*id))?;

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

    fn chomp_block(&mut self) -> Result<bool> {
        let name = self.chomp_blockname()?;

        if name == "END" {
            let end_data = chomp_bytes::<_, 12>(&mut self.reader)?;
            assert_eq!(end_data, [0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0]);
            return Ok(false);
        }

        let data = self.chomp_lz4()?;
        let block_reader = &mut (&data as &[u8]);

        match &*name {
            "SSTR" => {
                let unknown = chomp_i32_raw(block_reader)?;
                assert_eq!(unknown, 0);
                let num_strs = chomp_i32_raw(block_reader)?;

                for _ in 0..num_strs {
                    let unknown1 = chomp_i32_raw(block_reader)?;
                    let unknown2 = chomp_i32_raw(block_reader)?;
                    let unknown3 = chomp_i32_raw(block_reader)?;
                    let unknown4 = chomp_i32_raw(block_reader)?;
                    assert_eq!(unknown1, 0);
                    assert_eq!(unknown2, 0);
                    assert_eq!(unknown3, 0);
                    assert_eq!(unknown4, 0);

                    self.raw_info
                        .shared_strs
                        .push(chomp_binary_string(block_reader)?);
                }
            }
            "META" => {
                let num_pairs = chomp_i32_raw(block_reader)?;
                for _ in 0..num_pairs {
                    let key = chomp_string(block_reader)?;
                    let value = chomp_string(block_reader)?;
                    self.raw_info.meta.insert(key, value);
                }
            }
            "INST" => {
                let index = chomp_i32_raw(block_reader)?;
                let class_name = chomp_string(block_reader)?;
                let _is_service = chomp_bool(block_reader)?;
                let instance_count = chomp_i32_raw(block_reader)?;
                let mut instance_ids =
                    chomp_interleaved_i32_transformed(block_reader, instance_count as usize)?;

                make_cumulative(&mut instance_ids);

                for id in &instance_ids {
                    self.raw_info
                        .instances
                        .insert(*id, (class_name.clone(), Instance::uninit()));
                }

                self.raw_info.class_ids.insert(index, instance_ids);
            }
            "PROP" => {
                let class_index = chomp_i32_raw(block_reader)?;
                let property_name = chomp_string(block_reader)?;
                let prop_ty = chomp_u8(block_reader)?;

                let class_ids = self
                    .raw_info
                    .class_ids
                    .get(&class_index)
                    .ok_or(Error::UnknownClass(class_index))?;

                let num_props = class_ids.len();

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
                                    .map(RawProperty::RawSharedString),
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
                let unknown = chomp_u8(block_reader)?;
                assert_eq!(unknown, 0);
                let len = chomp_i32_raw(block_reader)?;
                let mut instance_referents =
                    chomp_interleaved_i32_transformed(block_reader, len as usize)?;
                let mut parent_referents =
                    chomp_interleaved_i32_transformed(block_reader, len as usize)?;

                make_cumulative(&mut instance_referents);
                make_cumulative(&mut parent_referents);

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
        let data = chomp_bytes::<_, 4>(&mut self.reader)?;

        let first_zero = data.iter().copied().position(|b| b == 0).unwrap_or(4) as usize;

        Ok(core::str::from_utf8(&data[..first_zero])
            .map_err(|_| Error::InvalidString)?
            .to_string())
    }

    fn chomp_lz4(&mut self) -> Result<Vec<u8>> {
        let compressed = chomp_i32_raw(&mut self.reader)?;
        let uncompressed = chomp_i32_raw(&mut self.reader)? as usize;

        assert_eq!(chomp_i32_raw(&mut self.reader)?, 0i32);

        let mut data = vec![0; compressed as usize];
        self.reader.read_exact(&mut data)?;

        let out =
            lz4_flex::block::decompress::decompress(&data, uncompressed).map_err(|_| Error::InvalidLz4)?;

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
    fn test_decode_i32() {
        assert_eq!(decode_i32(0b0000_0000_0000_0000_0000_0000_0000_0000), 0);
        assert_eq!(decode_i32(0b0000_0000_0000_0000_0000_0000_0000_0001), -1);
        assert_eq!(decode_i32(0b0000_0000_0000_0000_0000_0000_0000_0100), 2);
        assert_eq!(decode_i32(0b0000_0000_0000_0000_1000_0000_0000_0000), 16384);
        assert_eq!(
            decode_i32(0b0000_0000_0000_0000_1000_0000_0000_0001),
            -16385
        );
    }

    #[test]
    fn test_decode_f32() {
        assert_eq!(decode_f32(0b0000_0000_0000_0000_0000_0000_0000_0000), 0f32);
        assert_eq!(decode_f32(0b0111_1111_0000_0000_0000_0000_0000_0001), -1f32);
        assert_eq!(decode_f32(0b1000_0000_0000_0000_0000_0000_0000_0000), 2f32);
    }

    #[test]
    fn test_make_cumulative() {
        let mut array = [0, 1, 1, 5];
        make_cumulative(&mut array);
        assert_eq!(array, [0, 1, 2, 7]);
    }
}
