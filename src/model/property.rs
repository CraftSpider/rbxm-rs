//! An enum for an unrecognized Instance property, which could be any of the valid property types.

use crate::model::data::*;

use crate::tree::TreeKey;
use alloc::string::String;
use alloc::vec::Vec;
use uuid::Uuid;

/// This represents a property with an unknown type, handling any of the possible types.
/// Should generally only be used if working with an [`Instance`][crate::model::Instance] not
/// recognized by the crate.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Property {
    /// A raw byte-string, used for certain raw data such as collision or meshes
    BinaryString(Vec<u8>),
    /// A textual string, containing valid ASCII (Maybe UTF-8 or other encoding, help wanted)
    TextString(String),
    /// A raw-byte string which is encoded as interned/shared data
    SharedBinaryString(Vec<u8>),
    /// A textual string which is encoded as interned/shared data
    SharedTextString(String),
    /// A boolean value
    Bool(bool),
    /// A signed integral value, fitting in an i32
    Int32(i32),
    /// A signed integral value, fitting in an i64. Used for IDs
    Int64(i64),
    /// A single width floating-point value
    Float(f32),
    /// A double width floating-point value
    Double(f64),
    /// See [`UDim`]
    UDim(UDim),
    /// See [`UDim2`]
    UDim2(UDim2),
    /// See [`Ray`]
    Ray(Ray),
    /// See [`Faces`]
    Faces(Faces),
    /// See [`Axes`]
    Axes(Axes),
    /// See [`BrickColor`]
    BrickColor(BrickColor), // Deprecated
    /// See [`Color3`]
    Color3(Color3),
    /// See [`Vector2`]
    Vector2(Vector2),
    /// See [`Vector3`]
    Vector3(Vector3),
    /// See [`CFrame`]
    CFrame(CFrame),
    // Quaternion, // Not implemented properly by RBX?
    /// The discriminant for an enum, see [`model::enums`][crate::model::enums]. Which enum is not
    /// provided and must be known ahead of time.
    Enum(i32),
    /// A reference to an instance
    InstanceRef(TreeKey),
    /// See [`Vector3Int16`]
    Vector3Int16(Vector3Int16),
    /// See [`NumberSequence`]
    NumberSequence(NumberSequence),
    /// See [`ColorSequence`]
    ColorSequence(ColorSequence),
    /// See [`NumberRange`]
    NumberRange(NumberRange),
    /// See [`Rect`]
    Rect(Rect),
    /// A set of physical properties for physics objects.
    PhysicalProperties(PhysicalProperties),
    /// See [`Color3Uint8`]
    Color3Uint8(Color3Uint8),
    /// See [`Pivot`]
    Pivot(Pivot),
    /// A Universal Unique Identifier, or UUID
    UUID(Uuid),
}
