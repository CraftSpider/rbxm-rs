use crate::model::*;
use crate::serde::encoding::{encode_f32, encode_i32};
use crate::serde::io::Write;
use crate::serde::Result;

macro_rules! float_match {
    ($var:expr, $($array:expr; $num:literal),+) => {
        if false { unreachable!() }
        $(else if $var == $array {
            $num
        })*
        else {
            0
        }
    }
}

/// Types that can be written to a stream in the RBXM encoding format
pub trait Print<W: Write>: Sized {
    /// Write type to stream
    fn print(writer: &mut W, val: Self) -> Result<()>;
}

/// Types that can be written to a stream in an encoded form in the RBXM encoding format
pub trait PrintTransform<W: Write>: Sized {
    /// Encode and write type to stream
    fn print_transformed(writer: &mut W, val: Self) -> Result<()>;
}

/// Types that can be written 'interleaved' to a stream
pub trait PrintInterleaved<W: Write>: Sized {
    /// Write type interleaved to stream
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()>;
}

/// Types that can be written 'interleaved' in an encoded form to a stream
pub trait PrintInterleavedTransform<W: Write>: Sized {
    /// Encode and write the type interleaved to stream
    fn print_interleaved_transformed(writer: &mut W, vals: &[Self]) -> Result<()>;
}

impl<W: Write> Print<W> for bool {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&[(val as u8)])?;
        Ok(())
    }
}

impl<W: Write> Print<W> for u8 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&[val])?;
        Ok(())
    }
}

impl<W: Write> Print<W> for i16 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&val.to_le_bytes())?;
        Ok(())
    }
}

impl<W: Write> Print<W> for i32 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&val.to_le_bytes())?;
        Ok(())
    }
}

impl<W: Write> PrintTransform<W> for i32 {
    fn print_transformed(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&encode_i32(val).to_be_bytes())?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for i32 {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut data = vec![0; vals.len() * 4];

        for i in 0..vals.len() {
            let bytes = vals[i].to_be_bytes();
            for j in 0..4 {
                data[i + j * vals.len()] = bytes[j];
            }
        }

        writer.write_all(&data)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleavedTransform<W> for i32 {
    fn print_interleaved_transformed(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut data = vec![0; vals.len() * 4];

        for i in 0..vals.len() {
            let bytes = encode_i32(vals[i]).to_be_bytes();
            for j in 0..4 {
                data[i + j * vals.len()] = bytes[j];
            }
        }

        writer.write_all(&data)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for i64 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&val.to_le_bytes())?;
        Ok(())
    }
}

impl<W: Write> Print<W> for f32 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&val.to_le_bytes())?;
        Ok(())
    }
}

impl<W: Write> PrintTransform<W> for f32 {
    fn print_transformed(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&encode_f32(val).to_be_bytes())?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for f32 {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut data = vec![0; vals.len() * 4];

        for i in 0..vals.len() {
            let bytes = encode_f32(vals[i]).to_be_bytes();
            for j in 0..4 {
                data[i + j * vals.len()] = bytes[j];
            }
        }

        writer.write_all(&data)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for f64 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(&val.to_le_bytes())?;
        Ok(())
    }
}

impl<W: Write> Print<W> for String {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        i32::print(writer, val.len() as i32)?;
        writer.write_all(&val.into_bytes())?;
        Ok(())
    }
}

impl<W: Write> Print<W> for &[u8] {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        i32::print(writer, val.len() as i32)?;
        writer.write_all(val)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for UDim {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut scales = Vec::with_capacity(vals.len());
        let mut offsets = Vec::with_capacity(vals.len());

        for udim in vals {
            scales.push(udim.scale);
            offsets.push(udim.offset);
        }

        f32::print_interleaved(writer, &scales)?;
        i32::print_interleaved(writer, &offsets)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for UDim2 {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut x_scales = Vec::with_capacity(vals.len());
        let mut y_scales = Vec::with_capacity(vals.len());
        let mut x_offsets = Vec::with_capacity(vals.len());
        let mut y_offsets = Vec::with_capacity(vals.len());

        for udim in vals {
            x_scales.push(udim.x.scale);
            y_scales.push(udim.y.scale);
            x_offsets.push(udim.x.offset);
            y_offsets.push(udim.y.offset);
        }

        f32::print_interleaved(writer, &x_scales)?;
        f32::print_interleaved(writer, &y_scales)?;
        i32::print_interleaved(writer, &x_offsets)?;
        i32::print_interleaved(writer, &y_offsets)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for Ray {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut x_origs = Vec::with_capacity(vals.len());
        let mut y_origs = Vec::with_capacity(vals.len());
        let mut z_origs = Vec::with_capacity(vals.len());
        let mut x_dirs = Vec::with_capacity(vals.len());
        let mut y_dirs = Vec::with_capacity(vals.len());
        let mut z_dirs = Vec::with_capacity(vals.len());

        for ray in vals {
            x_origs.push(ray.origin.x);
            y_origs.push(ray.origin.y);
            z_origs.push(ray.origin.z);
            x_dirs.push(ray.direction.x);
            y_dirs.push(ray.direction.y);
            z_dirs.push(ray.direction.z);
        }

        f32::print_interleaved(writer, &x_origs)?;
        f32::print_interleaved(writer, &y_origs)?;
        f32::print_interleaved(writer, &z_origs)?;
        f32::print_interleaved(writer, &x_dirs)?;
        f32::print_interleaved(writer, &y_dirs)?;
        f32::print_interleaved(writer, &z_dirs)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Faces {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        let mut data = 0;

        if val.front {
            data |= 0b0000_0001;
        }
        if val.bottom {
            data |= 0b0000_0010;
        }
        if val.left {
            data |= 0b0000_0100;
        }
        if val.back {
            data |= 0b0000_1000;
        }
        if val.top {
            data |= 0b0001_0000;
        }
        if val.right {
            data |= 0b0010_0000;
        }

        u8::print(writer, data)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Axes {
    fn print(writer: &mut W, val: Self) -> Result<()> {
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

        u8::print(writer, data)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for BrickColor {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut indices = Vec::with_capacity(vals.len());

        for color in vals {
            indices.push(color.index);
        }

        i32::print_interleaved(writer, &indices)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Color3 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        f32::print(writer, val.r)?;
        f32::print(writer, val.g)?;
        f32::print(writer, val.b)?;
        Ok(())
    }
}

impl<W: Write> PrintTransform<W> for Color3 {
    fn print_transformed(writer: &mut W, val: Self) -> Result<()> {
        f32::print_transformed(writer, val.r)?;
        f32::print_transformed(writer, val.g)?;
        f32::print_transformed(writer, val.b)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Color3Uint8 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        u8::print(writer, val.r)?;
        u8::print(writer, val.g)?;
        u8::print(writer, val.b)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Vector2 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        f32::print_transformed(writer, val.x)?;
        f32::print_transformed(writer, val.y)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Vector3 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        f32::print_transformed(writer, val.x)?;
        f32::print_transformed(writer, val.y)?;
        f32::print_transformed(writer, val.z)?;
        Ok(())
    }
}

impl<W: Write> Print<W> for Vector3Int16 {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        i16::print(writer, val.x)?;
        i16::print(writer, val.y)?;
        i16::print(writer, val.z)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for CFrame {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut angles = Vec::new();
        let mut positions = Vec::with_capacity(vals.len());

        for cframe in vals {
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

            if angle_ty == 0 {
                for i in cframe.angle.iter() {
                    for j in i {
                        f32::print(&mut angles, *j)?;
                    }
                }
            }

            positions.push(&cframe.position);
        }

        writer.write_all(&angles)?;
        for pos in positions {
            Vector3::print(writer, pos.clone())?;
        }
        Ok(())
    }
}

impl<W: Write> Print<W> for NumberSequence {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        i32::print(writer, val.keypoints.len() as i32)?;
        for i in &val.keypoints {
            f32::print(writer, i.time)?;
            f32::print(writer, i.value)?;
            f32::print(writer, i.envelope)?;
        }
        Ok(())
    }
}

impl<W: Write> Print<W> for ColorSequence {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        i32::print(writer, val.keypoints.len() as i32)?;
        for i in &val.keypoints {
            f32::print(writer, i.time)?;
            Color3::print(writer, i.color.clone())?;
            f32::print(writer, i.envelope)?;
        }
        Ok(())
    }
}

impl<W: Write> Print<W> for NumberRange {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        f32::print(writer, val.low)?;
        f32::print(writer, val.high)?;
        Ok(())
    }
}

impl<W: Write> PrintInterleaved<W> for Rect {
    fn print_interleaved(writer: &mut W, vals: &[Self]) -> Result<()> {
        let mut min_x = Vec::with_capacity(vals.len());
        let mut min_y = Vec::with_capacity(vals.len());
        let mut max_x = Vec::with_capacity(vals.len());
        let mut max_y = Vec::with_capacity(vals.len());

        for rect in vals {
            min_x.push(rect.top_left.x);
            min_y.push(rect.top_left.y);
            max_x.push(rect.bottom_right.x);
            max_y.push(rect.bottom_right.y);
        }

        f32::print_interleaved(writer, &min_x)?;
        f32::print_interleaved(writer, &min_y)?;
        f32::print_interleaved(writer, &max_x)?;
        f32::print_interleaved(writer, &max_y)?;
        Ok(())
    }
}

#[cfg(feature = "mesh-format")]
impl<W: Write> Print<W> for ConvexHull {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        let ConvexHull {
            unknown_1,
            unknown_2,
            vertices,
            faces,
        } = val;

        i32::print(writer, unknown_1.len() as i32)?;
        writer.write_all(&unknown_1)?;

        i32::print(writer, unknown_2.len() as i32)?;
        writer.write_all(&unknown_2)?;

        i32::print(writer, (vertices.len() * 3) as i32)?;
        i32::print(writer, 4)?;
        for vertex in vertices {
            f32::print(writer, vertex.x)?;
            f32::print(writer, vertex.y)?;
            f32::print(writer, vertex.z)?;
        }

        i32::print(writer, (faces.len() * 3) as i32)?;
        for face in faces {
            i32::print(writer, face.0 as i32)?;
            i32::print(writer, face.1 as i32)?;
            i32::print(writer, face.2 as i32)?;
        }
        Ok(())
    }
}

#[cfg(feature = "mesh-format")]
impl<W: Write> Print<W> for TriMesh {
    fn print(writer: &mut W, val: Self) -> Result<()> {
        writer.write_all(b"CSGPHS")?;
        match val {
            TriMesh::Box => {
                // Kind
                i32::print(writer, 0)?;
                // Magic content
                writer.write_all(b"BLOCK")?;
            }
            TriMesh::Hull {
                volume,
                center_of_gravity,
                inertia_tensor,
                meshes,
            } => {
                // Kind
                i32::print(writer, 6)?;
                // Content
                f32::print(writer, volume)?;

                f32::print(writer, center_of_gravity.x)?;
                f32::print(writer, center_of_gravity.y)?;
                f32::print(writer, center_of_gravity.z)?;

                f32::print(writer, inertia_tensor[0][0])?;
                f32::print(writer, inertia_tensor[0][1])?;
                f32::print(writer, inertia_tensor[0][2])?;
                f32::print(writer, inertia_tensor[1][1])?;
                f32::print(writer, inertia_tensor[1][2])?;
                f32::print(writer, inertia_tensor[2][2])?;

                for mesh in meshes {
                    ConvexHull::print(writer, mesh)?;
                }
            }
        }
        Ok(())
    }
}
