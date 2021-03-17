use crate::model::*;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) enum Block {
    Meta(HashMap<String, String>),
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

#[derive(Debug, Clone)]
pub enum RawProperty {
    RawString(Vec<u8>), // This may or may not be a 'real' string, it can also be just a data blob
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
    #[allow(unused)]
    Quaternion, // Not implemented properly by RBX?
    Enum(i32),
    InstanceRef(i32),
    Vector3Int16(Vector3Int16),
    NumberSequence(NumberSequence),
    ColorSequence(ColorSequence),
    NumberRange(NumberRange),
    Rect(Rect),
    CustomPhysicalProperties(bool),
    Color3Uint8(Color3Uint8),
    SharedString(i32),
}

impl RawProperty {
    pub(crate) fn into_real(self) -> Property {
        match self {
            RawProperty::RawString(..) => unreachable!(),
            RawProperty::Bool(val) => Property::Bool(val),
            RawProperty::Int32(val) => Property::Int32(val),
            RawProperty::Float(val) => Property::Float(val),
            RawProperty::Double(val) => Property::Double(val),
            RawProperty::UDim(val) => Property::UDim(val),
            RawProperty::UDim2(val) => Property::UDim2(val),
            RawProperty::Ray(val) => Property::Ray(val),
            RawProperty::Face(val) => Property::Face(val),
            RawProperty::Axis(val) => Property::Axis(val),
            RawProperty::BrickColor(val) => Property::BrickColor(val),
            RawProperty::Color3(val) => Property::Color3(val),
            RawProperty::Vector2(val) => Property::Vector2(val),
            RawProperty::Vector3(val) => Property::Vector3(val),
            RawProperty::CFrame(val) => Property::CFrame(val),
            RawProperty::Quaternion => todo!(),
            RawProperty::Enum(val) => Property::Enum(val),
            RawProperty::InstanceRef(..) => unreachable!(),
            RawProperty::Vector3Int16(val) => Property::Vector3Int16(val),
            RawProperty::NumberSequence(val) => Property::NumberSequence(val),
            RawProperty::ColorSequence(val) => Property::ColorSequence(val),
            RawProperty::NumberRange(val) => Property::NumberRange(val),
            RawProperty::Rect(val) => Property::Rect(val),
            RawProperty::CustomPhysicalProperties(val) => Property::CustomPhysicalProperties(val),
            RawProperty::Color3Uint8(val) => Property::Color3Uint8(val),
            RawProperty::Int64(val) => Property::Int64(val),
            RawProperty::SharedString(..) => unreachable!(),
        }
    }

    pub(crate) fn from_real(prop: Property) -> RawProperty {
        match prop {
            Property::BinaryString(..) => unreachable!(),
            Property::TextString(..) => unreachable!(),
            Property::Bool(val) => RawProperty::Bool(val),
            Property::Int32(val) => RawProperty::Int32(val),
            Property::Float(val) => RawProperty::Float(val),
            Property::Double(val) => RawProperty::Double(val),
            Property::UDim(val) => RawProperty::UDim(val),
            Property::UDim2(val) => RawProperty::UDim2(val),
            Property::Ray(val) => RawProperty::Ray(val),
            Property::Face(val) => RawProperty::Face(val),
            Property::Axis(val) => RawProperty::Axis(val),
            Property::BrickColor(val) => RawProperty::BrickColor(val),
            Property::Color3(val) => RawProperty::Color3(val),
            Property::Vector2(val) => RawProperty::Vector2(val),
            Property::Vector3(val) => RawProperty::Vector3(val),
            Property::CFrame(val) => RawProperty::CFrame(val),
            // Property::Quaternion => todo!(),
            Property::Enum(val) => RawProperty::Enum(val),
            Property::InstanceRef(..) => unreachable!(),
            Property::Vector3Int16(val) => RawProperty::Vector3Int16(val),
            Property::NumberSequence(val) => RawProperty::NumberSequence(val),
            Property::ColorSequence(val) => RawProperty::ColorSequence(val),
            Property::NumberRange(val) => RawProperty::NumberRange(val),
            Property::Rect(val) => RawProperty::Rect(val),
            Property::CustomPhysicalProperties(val) => RawProperty::CustomPhysicalProperties(val),
            Property::Color3Uint8(val) => RawProperty::Color3Uint8(val),
            Property::Int64(val) => RawProperty::Int64(val),
        }
    }
}

// TODO: Use this as a ser/de intermediate, to allow Instance to be nicer for the user
pub struct RawInstance {
    pub children: Vec<i32>,
    pub parent: i32,
    pub kind: String,
    pub properties: HashMap<String, Property>,
}
