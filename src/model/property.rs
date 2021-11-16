//! An enum for an unrecognized Instance property, which could be any of the valid property types.

use crate::model::data::*;

use alloc::string::String;
use alloc::vec::Vec;
use uuid::Uuid;

/// An enum of the different possible property types.
/// Used largely for diagnostics, being able to pass around expected/actual types without needing
/// actual property data.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum PropertyType {
    /// A string of bytes
    BinaryString,
    /// A string of UTF8 characters
    TextString,
    /// An interned string of bytes
    SharedBinaryString,
    /// An interned string of UTF8 characters
    SharedTextString,
    /// A boolean
    Bool,
    /// A 32-bit integer
    Int32,
    /// A 64-bit integer
    Int64,
    /// A 32-bit floating point
    Float,
    /// A 64-bit floating point
    Double,
    /// A [`UDim`]
    UDim,
    /// A [`UDim2`]
    UDim2,
    /// A [`Ray`]
    Ray,
    /// A set of [`Faces`]
    Faces,
    /// A set of [`Axes`]
    Axes,
    /// A [`BrickColor`]
    BrickColor,
    /// A [`Color3`]
    Color3,
    /// A [`Vector2`]
    Vector2,
    /// A [`Vector3`]
    Vector3,
    /// A [`CFrame`]
    CFrame,
    /// Any enumeration, see [`crate::model::enums`]
    Enum,
    /// An [`InstanceRef`]
    InstanceRef,
    /// A [`Vector3Int16`]
    Vector3Int16,
    /// A [`NumberSequence`]
    NumberSequence,
    /// A [`ColorSequence`]
    ColorSequence,
    /// A [`NumberRange`]
    NumberRange,
    /// A [`Rect`]
    Rect,
    /// A set of [`PhysicalProperties`]
    PhysicalProperties,
    /// A [`Color3Uint8`]
    Color3Uint8,
    /// A [`Pivot`]
    Pivot,
    /// A [`Uuid`]
    Uuid,
}

impl PropertyType {
    /// Get the human-readable name for this property type
    pub fn name(&self) -> &'static str {
        match self {
            PropertyType::BinaryString => "BinaryString",
            PropertyType::TextString => "TextString",
            PropertyType::SharedBinaryString => "SharedBinaryString",
            PropertyType::SharedTextString => "SharedTextString",
            PropertyType::Bool => "Bool",
            PropertyType::Int32 => "Int32",
            PropertyType::Int64 => "Int64",
            PropertyType::Float => "Float",
            PropertyType::Double => "Double",
            PropertyType::UDim => "UDim",
            PropertyType::UDim2 => "UDim2",
            PropertyType::Ray => "Ray",
            PropertyType::Faces => "Faces",
            PropertyType::Axes => "Axes",
            PropertyType::BrickColor => "BrickColor",
            PropertyType::Color3 => "Color3",
            PropertyType::Vector2 => "Vector2",
            PropertyType::Vector3 => "Vector3",
            PropertyType::CFrame => "CFrame",
            PropertyType::Enum => "Enum",
            PropertyType::InstanceRef => "InstanceRef",
            PropertyType::Vector3Int16 => "Vector3Int16",
            PropertyType::NumberSequence => "NumberSequence",
            PropertyType::ColorSequence => "ColorSequence",
            PropertyType::NumberRange => "NumberRange",
            PropertyType::Rect => "Rect",
            PropertyType::PhysicalProperties => "PhysicalProperties",
            PropertyType::Color3Uint8 => "Color3Uint8",
            PropertyType::Pivot => "Pivot",
            PropertyType::Uuid => "Uuid",
        }
    }
}

/// This represents a property with an unknown type, handling any of the possible types.
/// Should generally only be used if working with an [`Instance`](crate::model::Instance) not
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
    InstanceRef(InstanceRef),
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
    Uuid(Uuid),
}

impl Property {
    pub(crate) fn kind(&self) -> PropertyType {
        match self {
            Property::BinaryString(..) => PropertyType::BinaryString,
            Property::TextString(..) => PropertyType::TextString,
            Property::SharedBinaryString(..) => PropertyType::SharedBinaryString,
            Property::SharedTextString(..) => PropertyType::SharedTextString,
            Property::Bool(..) => PropertyType::Bool,
            Property::Int32(..) => PropertyType::Int32,
            Property::Int64(..) => PropertyType::Int64,
            Property::Float(..) => PropertyType::Float,
            Property::Double(..) => PropertyType::Double,
            Property::UDim(..) => PropertyType::UDim,
            Property::UDim2(..) => PropertyType::UDim2,
            Property::Ray(..) => PropertyType::Ray,
            Property::Faces(..) => PropertyType::Faces,
            Property::Axes(..) => PropertyType::Axes,
            Property::BrickColor(..) => PropertyType::BrickColor,
            Property::Color3(..) => PropertyType::Color3,
            Property::Vector2(..) => PropertyType::Vector2,
            Property::Vector3(..) => PropertyType::Vector3,
            Property::CFrame(..) => PropertyType::CFrame,
            Property::Enum(..) => PropertyType::Enum,
            Property::InstanceRef(..) => PropertyType::InstanceRef,
            Property::Vector3Int16(..) => PropertyType::Vector3Int16,
            Property::NumberSequence(..) => PropertyType::NumberSequence,
            Property::ColorSequence(..) => PropertyType::ColorSequence,
            Property::NumberRange(..) => PropertyType::NumberRange,
            Property::Rect(..) => PropertyType::Rect,
            Property::PhysicalProperties(..) => PropertyType::PhysicalProperties,
            Property::Color3Uint8(..) => PropertyType::Color3Uint8,
            Property::Pivot(..) => PropertyType::Pivot,
            Property::Uuid(..) => PropertyType::Uuid,
        }
    }
}
