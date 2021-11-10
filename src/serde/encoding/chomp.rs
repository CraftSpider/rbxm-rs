
// Consume ('chomp') some data from an input

use crate::model::*;
use crate::serde::encoding::{decode_i32, decode_f32};
use crate::serde::{Result, Error};
use crate::serde::io::Read;

/// Types that can be read from a stream that is assumed to match the RBXM encoding format
pub trait Chomp<R: Read>: Sized {
    /// Consume type from stream
    fn chomp(reader: &mut R) -> Result<Self>;
}

/// Types that can be read in an encoded form from a stream that is assumed to match the RBXM
/// encoding format
pub trait ChompTransform<R: Read>: Sized {
    /// Consume and decode type from stream
    fn chomp_transformed(reader: &mut R) -> Result<Self>;
}

/// Types that can be read 'interleaved' from a stream
pub trait ChompInterleaved<R: Read>: Sized {
    /// Consume type interleaved from stream
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>>;
}

/// Types that can be read 'interleaved' in an encoded form
pub trait ChompInterleavedTransform<R: Read>: Sized {
    /// Consume type interleaved and decode from stream
    fn chomp_interleaved_transformed(reader: &mut R, count: usize) -> Result<Vec<Self>>;
}

impl<R: Read> Chomp<R> for bool {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 1];
        reader.read_exact(&mut data)?;
        Ok(data[0] != 0)
    }
}

impl<R: Read> Chomp<R> for u8 {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 1];
        reader.read_exact(&mut data)?;
        Ok(data[0])
    }
}

impl<R: Read> Chomp<R> for i16 {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 2];
        reader.read_exact(&mut data)?;
        Ok(i16::from_le_bytes(data))
    }
}

impl<R: Read> Chomp<R> for i32 {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(i32::from_le_bytes(data))
    }
}

impl<R: Read> ChompTransform<R> for i32 {
    fn chomp_transformed(reader: &mut R) -> Result<Self> {
        let mut data = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(decode_i32(u32::from_be_bytes(data)))
    }
}

impl<R: Read> ChompInterleaved<R> for i32 {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let mut data = vec![0; count * 4];
        reader.read_exact(&mut data)?;

        let mut out = vec![0; count];
        for i in 0..count {
            let mut bytes = [0; 4];
            for j in 0..4 {
                bytes[j] = data[i + j * count];
            }
            out[i] = i32::from_be_bytes(bytes);
        }

        Ok(out)
    }
}

impl<R: Read> ChompInterleavedTransform<R> for i32 {
    fn chomp_interleaved_transformed(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let mut data = vec![0; count * 4];
        reader.read_exact(&mut data)?;

        let mut out = vec![0; count];
        for i in 0..count {
            let mut bytes = [0; 4];
            for j in 0..4 {
                bytes[j] = data[i + j * count];
            }
            out[i] = decode_i32(u32::from_be_bytes(bytes));
        }
        Ok(out)
    }
}

impl<R: Read> Chomp<R> for i64 {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 8];
        reader.read_exact(&mut data)?;
        Ok(i64::from_le_bytes(data))
    }
}

impl<R: Read> Chomp<R> for f32 {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(f32::from_le_bytes(data))
    }
}

impl<R: Read> ChompTransform<R> for f32 {
    fn chomp_transformed(reader: &mut R) -> Result<Self> {
        let mut data = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(decode_f32(u32::from_be_bytes(data)))
    }
}

impl<R: Read> ChompInterleaved<R> for f32 {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let mut data = vec![0; count * 4];
        reader.read_exact(&mut data)?;

        let mut out = vec![0f32; count];
        for i in 0..count {
            let mut bytes = [0; 4];
            for j in 0..4 {
                bytes[j] = data[i + j * count];
            }
            out[i] = decode_f32(u32::from_be_bytes(bytes));
        }

        Ok(out)
    }
}

impl<R: Read> Chomp<R> for f64 {
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; 8];
        reader.read_exact(&mut data)?;
        Ok(f64::from_le_bytes(data))
    }
}

impl<R: Read, const N: usize> Chomp<R> for [u8; N] {
    #[inline]
    fn chomp(reader: &mut R) -> Result<Self> {
        let mut data = [0; N];
        reader.read_exact(&mut data)?;
        Ok(data)
    }
}

impl<R: Read> Chomp<R> for String {
    fn chomp(reader: &mut R) -> Result<Self> {
        let len = i32::chomp(reader)?;
        let mut str = vec![0; len as usize];
        reader.read_exact(&mut str)?;
        String::from_utf8(str).map_err(|_| Error::InvalidString)
    }
}

impl<R: Read> Chomp<R> for Vec<u8> {
    fn chomp(reader: &mut R) -> Result<Self> {
        let len = i32::chomp(reader)?;
        let mut str = vec![0; len as usize];
        reader.read_exact(&mut str)?;
        Ok(str)
    }
}

impl<R: Read> ChompInterleaved<R> for UDim {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let scales = f32::chomp_interleaved(reader, count)?;
        let offsets = i32::chomp_interleaved(reader, count)?;

        let mut out = Vec::with_capacity(count);
        for (scale, offset) in scales.into_iter().zip(offsets.into_iter()) {
            out.push(UDim { scale, offset })
        }

        Ok(out)
    }
}

impl<R: Read> ChompInterleaved<R> for UDim2 {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let x_scales = f32::chomp_interleaved(reader, count)?;
        let y_scales = f32::chomp_interleaved(reader, count)?;
        let x_offsets = i32::chomp_interleaved(reader, count)?;
        let y_offsets = i32::chomp_interleaved(reader, count)?;

        let mut out = Vec::with_capacity(count);
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
}

impl<R: Read> ChompInterleaved<R> for Ray {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let x_origs = f32::chomp_interleaved(reader, count)?;
        let y_origs = f32::chomp_interleaved(reader, count)?;
        let z_origs = f32::chomp_interleaved(reader, count)?;
        let x_dir = f32::chomp_interleaved(reader, count)?;
        let y_dir = f32::chomp_interleaved(reader, count)?;
        let z_dir = f32::chomp_interleaved(reader, count)?;

        let mut out = Vec::with_capacity(count);
        for i in 0..count {
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
}

impl<R: Read> Chomp<R> for Faces {
    fn chomp(reader: &mut R) -> Result<Self> {
        let data = u8::chomp(reader)?;
        Ok(Faces {
            front: data & 0b0000_0001 > 0,
            bottom: data & 0b0000_0010 > 0,
            left: data & 0b0000_0100 > 0,
            back: data & 0b0000_1000 > 0,
            top: data & 0b0001_0000 > 0,
            right: data & 0b0010_0000 > 0,
        })
    }
}

impl<R: Read> Chomp<R> for Axes {
    fn chomp(reader: &mut R) -> Result<Self> {
        let data = u8::chomp(reader)?;
        Ok(Axes {
            x: data & 0b100 != 0,
            y: data & 0b010 != 0,
            z: data & 0b001 != 0,
        })
    }
}

impl<R: Read> ChompInterleaved<R> for BrickColor {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let mut colors = Vec::with_capacity(count);
        let indices = i32::chomp_interleaved(reader, count)?;
        for index in indices {
            colors.push(BrickColor { index })
        }
        Ok(colors)
    }
}

impl<R: Read> Chomp<R> for Color3 {
    fn chomp(reader: &mut R) -> Result<Self> {
        Ok(Color3 {
            r: f32::chomp(reader)?,
            g: f32::chomp(reader)?,
            b: f32::chomp(reader)?,
        })
    }
}

impl<R: Read> Chomp<R> for Color3Uint8 {
    fn chomp(reader: &mut R) -> Result<Self> {
        Ok(Color3Uint8 {
            r: u8::chomp(reader)?,
            g: u8::chomp(reader)?,
            b: u8::chomp(reader)?,
        })
    }
}

impl<R: Read> ChompTransform<R> for Color3 {
    fn chomp_transformed(reader: &mut R) -> Result<Self> {
        Ok(Color3 {
            r: f32::chomp_transformed(reader)?,
            g: f32::chomp_transformed(reader)?,
            b: f32::chomp_transformed(reader)?,
        })
    }
}

impl<R: Read> Chomp<R> for Vector2 {
    fn chomp(reader: &mut R) -> Result<Self> {
        Ok(Vector2 {
            x: f32::chomp_transformed(reader)?,
            y: f32::chomp_transformed(reader)?,
        })
    }
}

impl<R: Read> Chomp<R> for Vector3 {
    fn chomp(reader: &mut R) -> Result<Self> {
        Ok(Vector3 {
            x: f32::chomp_transformed(reader)?,
            y: f32::chomp_transformed(reader)?,
            z: f32::chomp_transformed(reader)?,
        })
    }
}

impl<R: Read> Chomp<R> for Vector3Int16 {
    fn chomp(reader: &mut R) -> Result<Self> {
        Ok(Vector3Int16 {
            x: i16::chomp(reader)?,
            y: i16::chomp(reader)?,
            z: i16::chomp(reader)?,
        })
    }
}

impl<R: Read> ChompInterleaved<R> for CFrame {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let mut angles = Vec::with_capacity(count);
        for _ in 0..count {
            let angle_type = u8::chomp(reader)?;
            let angle: [[f32; 3]; 3] = match angle_type {
                0 => {
                    let mut data = [[0f32; 3]; 3];
                    for i in &mut data {
                        for j in i {
                            *j = f32::chomp(reader)?;
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
        let mut positions = Vec::with_capacity(count);
        for _ in 0..count {
            positions.push(Vector3::chomp(reader)?);
        }

        let mut out = Vec::with_capacity(count);
        for (position, angle) in positions.into_iter().zip(angles.into_iter()) {
            out.push(CFrame { position, angle })
        }

        Ok(out)
    }
}

impl<R: Read> Chomp<R> for NumberSequence {
    fn chomp(reader: &mut R) -> Result<Self> {
        let num_keypoints = i32::chomp(reader)?;
        let mut keypoints = Vec::with_capacity(num_keypoints as usize);
        for _ in 0..num_keypoints {
            let time = f32::chomp(reader)?;
            let value = f32::chomp(reader)?;
            let envelope = f32::chomp(reader)?;
            keypoints.push(NumberKeypoint {
                time,
                value,
                envelope,
            })
        }

        Ok(NumberSequence { keypoints })
    }
}

impl<R: Read> Chomp<R> for ColorSequence {
    fn chomp(reader: &mut R) -> Result<Self> {
        let num_keypoints = i32::chomp(reader)?;
        let mut keypoints = Vec::with_capacity(num_keypoints as usize);
        for _ in 0..num_keypoints {
            let time = f32::chomp(reader)?;
            let color = Color3::chomp(reader)?;
            let envelope = f32::chomp(reader)?;
            keypoints.push(ColorKeypoint {
                time,
                color,
                envelope,
            })
        }

        Ok(ColorSequence { keypoints })
    }
}

impl<R: Read> Chomp<R> for NumberRange {
    fn chomp(reader: &mut R) -> Result<Self> {
        Ok(NumberRange {
            low: f32::chomp(reader)?,
            high: f32::chomp(reader)?,
        })
    }
}

impl<R: Read> ChompInterleaved<R> for Rect {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let min_x = f32::chomp_interleaved(reader, count)?;
        let min_y = f32::chomp_interleaved(reader, count)?;
        let max_x = f32::chomp_interleaved(reader, count)?;
        let max_y = f32::chomp_interleaved(reader, count)?;

        let mut out = Vec::with_capacity(count);
        for i in 0..count {
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
}

impl<R: Read> ChompInterleaved<R> for Pivot {
    fn chomp_interleaved(reader: &mut R, count: usize) -> Result<Vec<Self>> {
        let _first = u8::chomp(reader)?;

        let cframes = CFrame::chomp_interleaved(reader, count)?;

        let _second = u8::chomp(reader)?;

        let mut unknown = Vec::new();

        for _ in 0..count {
            unknown.push(u8::chomp(reader)?);
        }

        let mut out = Vec::new();

        for (cframe, unknown) in cframes.into_iter().zip(unknown.into_iter()) {
            out.push(Pivot { cframe, unknown })
        }

        Ok(out)
    }
}

#[cfg(feature = "mesh-format")]
impl<R: Read> Chomp<R> for ConvexHull {
    fn chomp(reader: &mut R) -> Result<Self> {
        let unknown_1_len = i32::chomp(reader)?;
        let mut unknown_1 = vec![0; unknown_1_len as usize];
        reader.read_exact(&mut unknown_1)?;

        let unknown_2_len = i32::chomp(reader)?;
        let mut unknown_2 = vec![0; unknown_2_len as usize];
        reader.read_exact(&mut unknown_2)?;

        let vert_len = i32::chomp(reader)? / 3;
        let vert_width = i32::chomp(reader)?;
        assert_eq!(vert_width, 4, "Unexpected vertice width");
        let mut vertices = Vec::with_capacity(vert_len as usize);
        for _ in 0..vert_len {
            vertices.push(Vector3 {
                x: f32::chomp(reader)?,
                y: f32::chomp(reader)?,
                z: f32::chomp(reader)?,
            });
        }

        let faces_len = i32::chomp(reader)? / 3;
        let mut faces = Vec::with_capacity(faces_len as usize);
        for _ in 0..faces_len {
            faces.push((
                i32::chomp(reader)? as usize,
                i32::chomp(reader)? as usize,
                i32::chomp(reader)? as usize,
            ));
        }

        Ok(ConvexHull {
            unknown_1,
            unknown_2,
            vertices,
            faces,
        })
    }
}

#[cfg(feature = "mesh-format")]
impl<R: Read> Chomp<R> for TriMesh {
    fn chomp(reader: &mut R) -> Result<Self> {
        let magic = <[u8; 6]>::chomp(reader)?;
        if &magic != b"CSGPHS" {
            return Err(Error::BadMagic);
        }

        let kind = i32::chomp(reader)?;

        match kind {
            0 => {
                let magic = <[u8; 5]>::chomp(reader)?;
                if &magic != b"BLOCK" {
                    Err(Error::BadMagic)
                } else {
                    Ok(TriMesh::Box)
                }
            }
            6 => {
                let volume = f32::chomp(reader)?;
                let center_of_gravity = Vector3 {
                    x: f32::chomp(reader)?,
                    y: f32::chomp(reader)?,
                    z: f32::chomp(reader)?,
                };

                let inertia_tensor = {
                    let xx = f32::chomp(reader)?;
                    let xy = f32::chomp(reader)?;
                    let xz = f32::chomp(reader)?;
                    let yy = f32::chomp(reader)?;
                    let yz = f32::chomp(reader)?;
                    let zz = f32::chomp(reader)?;

                    [[xx, xy, xz], [-xy, yy, yz], [-xz, -yz, zz]]
                };

                let mut meshes = Vec::new();
                // Read hulls till we run out of reader
                loop {
                    match ConvexHull::chomp(reader) {
                        Ok(mesh) => meshes.push(mesh),
                        Err(Error::IoError(_)) => break,
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
            val => Err(Error::UnknownMesh(val)),
        }
    }
}
