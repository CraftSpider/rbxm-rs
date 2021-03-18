use crate::model::data::*;
use crate::model::Instance;

use std::cell::RefCell;
use std::rc::Weak;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Property {
    BinaryString(Vec<u8>),
    TextString(String),
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    UDim(UDim),
    UDim2(UDim2),
    Ray(Ray),
    Face(Face),
    Axis(Axis),
    BrickColor(BrickColor), // Deprecated
    Color3(Color3),
    Vector2(Vector2),
    Vector3(Vector3),
    CFrame(CFrame),
    // Quaternion, // Not implemented properly by RBX?
    Enum(i32),
    InstanceRef(Weak<RefCell<Instance>>),
    Vector3Int16(Vector3Int16),
    NumberSequence(NumberSequence),
    ColorSequence(ColorSequence),
    NumberRange(NumberRange),
    Rect(Rect),
    CustomPhysicalProperties(bool),
    Color3Uint8(Color3Uint8),
}
