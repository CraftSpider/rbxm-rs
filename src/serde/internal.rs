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

macro_rules! chomp_prop {
    ($map:ident, $name:literal, $prop:ident) => {
        if let Some(Property::$prop(val)) = $map.remove($name) {
            val
        } else {
            todo!()
        };
    };
}

fn chomp_base(properties: &mut HashMap<String, Property>) -> Base {
    Base {
        name: chomp_prop!(properties, "Name", TextString),
        tags: chomp_prop!(properties, "Tags", TextString),
        attributes_serialize: chomp_prop!(properties, "AttributesSerialize", TextString),
        source_asset_id: chomp_prop!(properties, "SourceAssetId", Int64),
    }
}

fn chomp_base_part(properties: &mut HashMap<String, Property>) -> BasePart {
    BasePart {
        anchored: chomp_prop!(properties, "Anchored", Bool),
        locked: chomp_prop!(properties, "Locked", Bool),
        massless: chomp_prop!(properties, "Massless", Bool),
        can_touch: chomp_prop!(properties, "CanTouch", Bool),
        can_collide: chomp_prop!(properties, "CanCollide", Bool),
        cast_shadow: chomp_prop!(properties, "CastShadow", Bool),

        size: chomp_prop!(properties, "size", Vector3),
        cframe: chomp_prop!(properties, "CFrame", CFrame),
        velocity: chomp_prop!(properties, "Velocity", Vector3),
        rot_velocity: chomp_prop!(properties, "RotVelocity", Vector3),

        material: chomp_prop!(properties, "Material", Enum),
        transparency: chomp_prop!(properties, "Transparency", Float),
        reflectance: chomp_prop!(properties, "Reflectance", Float),

        collision_group_id: chomp_prop!(properties, "CollisionGroupId", Int32),
        custom_physical_properties: chomp_prop!(
            properties,
            "CustomPhysicalProperties",
            CustomPhysicalProperties
        ),
        root_priority: chomp_prop!(properties, "RootPriority", Int32),

        top_surface: chomp_prop!(properties, "TopSurface", Enum),
        top_surface_input: chomp_prop!(properties, "TopSurfaceInput", Enum),
        top_param_a: chomp_prop!(properties, "TopParamA", Float),
        top_param_b: chomp_prop!(properties, "TopParamB", Float),

        bottom_surface: chomp_prop!(properties, "BottomSurface", Enum),
        bottom_surface_input: chomp_prop!(properties, "BottomSurfaceInput", Enum),
        bottom_param_a: chomp_prop!(properties, "BottomParamA", Float),
        bottom_param_b: chomp_prop!(properties, "BottomParamB", Float),

        front_surface: chomp_prop!(properties, "FrontSurface", Enum),
        front_surface_input: chomp_prop!(properties, "FrontSurfaceInput", Enum),
        front_param_a: chomp_prop!(properties, "FrontParamA", Float),
        front_param_b: chomp_prop!(properties, "FrontParamB", Float),

        back_surface: chomp_prop!(properties, "BackSurface", Enum),
        back_surface_input: chomp_prop!(properties, "BackSurfaceInput", Enum),
        back_param_a: chomp_prop!(properties, "BackParamA", Float),
        back_param_b: chomp_prop!(properties, "BackParamB", Float),

        left_surface: chomp_prop!(properties, "LeftSurface", Enum),
        left_surface_input: chomp_prop!(properties, "LeftSurfaceInput", Enum),
        left_param_a: chomp_prop!(properties, "LeftParamA", Float),
        left_param_b: chomp_prop!(properties, "LeftParamB", Float),

        right_surface: chomp_prop!(properties, "RightSurface", Enum),
        right_surface_input: chomp_prop!(properties, "RightSurfaceInput", Enum),
        right_param_a: chomp_prop!(properties, "RightParamA", Float),
        right_param_b: chomp_prop!(properties, "RightParamB", Float),
    }
}

fn chomp_part(properties: &mut HashMap<String, Property>) -> Part {
    Part {
        base: chomp_base(properties),
        base_part: chomp_base_part(properties),
        form_factor_raw: chomp_prop!(properties, "formFactorRaw", Enum),
        color3_uint8: chomp_prop!(properties, "Color3uint8", Color3Uint8),
        shape: chomp_prop!(properties, "shape", Enum),
    }
}

pub fn make_kind(kind: &str, mut properties: HashMap<String, Property>) -> InstanceKind {
    match kind {
        "Part" => InstanceKind::Part(chomp_part(&mut properties)),
        _ => InstanceKind::Other(kind.to_owned(), properties),
    }
}

pub fn break_kind(_kind: &InstanceKind) -> HashMap<String, Property> {
    todo!()
}
