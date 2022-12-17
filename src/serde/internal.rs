use crate::model::property::PropertyType;
use crate::model::*;
use crate::serde::encoding::{Chomp, Print};
use crate::serde::{Error, ErrorKind, Result};

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use uuid::Uuid;

macro_rules! prop_ty_impl {
    ($($ty:ty : $variant:ident),+ $(,)?) => {
        $(
        impl FieldFromProperties for $ty {
            fn from_properties(attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
                match properties.remove(attrs.prop_name) {
                    Some(Property::$variant(val)) => Ok(val),
                    Some(prop) => Err($crate::SerdeError::wrong_property_type(
                        attrs.prop_name.to_string(),
                        Some((PropertyType::$variant, prop.kind()))
                    )),
                    None => Err($crate::SerdeError::missing_property(attrs.prop_name.to_string())),
                }
            }
        }

        impl FieldToProperties for $ty {
            fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
                properties.insert(attrs.prop_name.to_string(), Property::$variant(self));
            }
        }
        )*
    }
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
    Face(Faces),
    Axis(Axes),
    BrickColor(BrickColor), // Deprecated
    Color3(Color3),
    Vector2(Vector2),
    Vector3(Vector3),
    CFrame(CFrame),
    Enum(i32),
    InstanceRef(i32),
    Vector3Int16(Vector3Int16),
    NumberSequence(NumberSequence),
    ColorSequence(ColorSequence),
    NumberRange(NumberRange),
    Rect(Rect),
    PhysicalProperties(PhysicalProperties),
    Color3Uint8(Color3Uint8),
    RawSharedString(i32),
    // TODO: This is called 'OptionalCoordinateFrame' in XML
    Pivot(Pivot),
    Uuid(Uuid),
    Font(FontFace),
}

impl RawProperty {
    pub(crate) fn encode_ty(&self) -> u8 {
        match self {
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
            // RawProperty::Quaternion => 17,
            RawProperty::Enum(..) => 18,
            RawProperty::InstanceRef(..) => 19,
            RawProperty::Vector3Int16(..) => 20,
            RawProperty::NumberSequence(..) => 21,
            RawProperty::ColorSequence(..) => 22,
            RawProperty::NumberRange(..) => 23,
            RawProperty::Rect(..) => 24,
            RawProperty::PhysicalProperties(..) => 25,
            RawProperty::Color3Uint8(..) => 26,
            RawProperty::Int64(..) => 27,
            RawProperty::RawSharedString(..) => 28,
            RawProperty::Pivot(..) => 30,
            RawProperty::Uuid(..) => 31,
            RawProperty::Font(..) => 32,
        }
    }

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
            RawProperty::Face(val) => Property::Faces(val),
            RawProperty::Axis(val) => Property::Axes(val),
            RawProperty::BrickColor(val) => Property::BrickColor(val),
            RawProperty::Color3(val) => Property::Color3(val),
            RawProperty::Vector2(val) => Property::Vector2(val),
            RawProperty::Vector3(val) => Property::Vector3(val),
            RawProperty::CFrame(val) => Property::CFrame(val),
            // RawProperty::Quaternion => unimplemented!("Quaternions not supported"),
            RawProperty::Enum(val) => Property::Enum(val),
            RawProperty::InstanceRef(..) => unreachable!(),
            RawProperty::Vector3Int16(val) => Property::Vector3Int16(val),
            RawProperty::NumberSequence(val) => Property::NumberSequence(val),
            RawProperty::ColorSequence(val) => Property::ColorSequence(val),
            RawProperty::NumberRange(val) => Property::NumberRange(val),
            RawProperty::Rect(val) => Property::Rect(val),
            RawProperty::PhysicalProperties(val) => Property::PhysicalProperties(val),
            RawProperty::Color3Uint8(val) => Property::Color3Uint8(val),
            RawProperty::Int64(val) => Property::Int64(val),
            RawProperty::RawSharedString(..) => unreachable!(),
            RawProperty::Pivot(val) => Property::Pivot(val),
            RawProperty::Uuid(val) => Property::Uuid(val),
            RawProperty::Font(val) => Property::Font(val),
        }
    }

    pub(crate) fn from_real(prop: Property) -> RawProperty {
        match prop {
            Property::BinaryString(..) => unreachable!(),
            Property::TextString(..) => unreachable!(),
            Property::SharedBinaryString(..) => unreachable!(),
            Property::SharedTextString(..) => unreachable!(),
            Property::Bool(val) => RawProperty::Bool(val),
            Property::Int32(val) => RawProperty::Int32(val),
            Property::Float(val) => RawProperty::Float(val),
            Property::Double(val) => RawProperty::Double(val),
            Property::UDim(val) => RawProperty::UDim(val),
            Property::UDim2(val) => RawProperty::UDim2(val),
            Property::Ray(val) => RawProperty::Ray(val),
            Property::Faces(val) => RawProperty::Face(val),
            Property::Axes(val) => RawProperty::Axis(val),
            Property::BrickColor(val) => RawProperty::BrickColor(val),
            Property::Color3(val) => RawProperty::Color3(val),
            Property::Vector2(val) => RawProperty::Vector2(val),
            Property::Vector3(val) => RawProperty::Vector3(val),
            Property::CFrame(val) => RawProperty::CFrame(val),
            // Property::Quaternion => unimplemented!("Quaternions not supported"),
            Property::Enum(val) => RawProperty::Enum(val),
            Property::InstanceRef(..) => unreachable!(),
            Property::Vector3Int16(val) => RawProperty::Vector3Int16(val),
            Property::NumberSequence(val) => RawProperty::NumberSequence(val),
            Property::ColorSequence(val) => RawProperty::ColorSequence(val),
            Property::NumberRange(val) => RawProperty::NumberRange(val),
            Property::Rect(val) => RawProperty::Rect(val),
            Property::PhysicalProperties(val) => RawProperty::PhysicalProperties(val),
            Property::Color3Uint8(val) => RawProperty::Color3Uint8(val),
            Property::Int64(val) => RawProperty::Int64(val),
            Property::Pivot(val) => RawProperty::Pivot(val),
            Property::Uuid(val) => RawProperty::Uuid(val),
            Property::Font(val) => RawProperty::Font(val),
        }
    }
}

pub trait FromProperties: Sized {
    fn from_properties(properties: &mut BTreeMap<String, Property>) -> Result<Self>;
}

pub trait ToProperties: Sized {
    fn to_properties(&self, properties: &mut BTreeMap<String, Property>);
}

pub struct FieldAttrs {
    pub field_name: &'static str,
    pub prop_name: &'static str,
    pub shared: bool,
}

pub trait FieldFromProperties: Sized {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>,
    ) -> Result<Self>;
}

impl<T: FromProperties> FieldFromProperties for T {
    fn from_properties(_: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
        T::from_properties(properties)
    }
}

impl FieldFromProperties for String {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>,
    ) -> Result<Self> {
        if attrs.shared {
            match properties.remove(attrs.prop_name) {
                Some(Property::SharedTextString(val)) => Ok(val),
                Some(prop) => Err(Error::wrong_property_type(
                    attrs.prop_name.to_string(),
                    Some((PropertyType::SharedTextString, prop.kind())),
                )),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        } else {
            match properties.remove(attrs.prop_name) {
                Some(Property::TextString(val)) => Ok(val),
                Some(prop) => Err(Error::wrong_property_type(
                    attrs.prop_name.to_string(),
                    Some((PropertyType::TextString, prop.kind())),
                )),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        }
    }
}

impl FieldFromProperties for Vec<u8> {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>,
    ) -> Result<Self> {
        if attrs.shared {
            match properties.remove(attrs.prop_name) {
                Some(Property::SharedBinaryString(val)) => Ok(val),
                Some(Property::SharedTextString(str)) => Ok(str.into_bytes()),
                Some(prop) => Err(Error::wrong_property_type(
                    attrs.prop_name.to_string(),
                    Some((PropertyType::SharedBinaryString, prop.kind())),
                )),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        } else {
            match properties.remove(attrs.prop_name) {
                Some(Property::BinaryString(val)) => Ok(val),
                Some(Property::TextString(str)) => Ok(str.into_bytes()),
                Some(prop) => Err(Error::wrong_property_type(
                    attrs.prop_name.to_string(),
                    Some((PropertyType::BinaryString, prop.kind())),
                )),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        }
    }
}

impl FieldFromProperties for Attributes {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>,
    ) -> Result<Self> {
        match <Vec<u8>>::from_properties(attrs, properties) {
            Ok(bytes) => {
                if bytes.is_empty() {
                    return Ok(Attributes::default());
                }
                let mut reader = &*bytes;
                let out = Attributes::chomp(&mut reader);
                debug_assert_eq!(
                    *reader,
                    [],
                    "Attributes didn't consume whole serialized buffer"
                );
                out
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(feature = "mesh-format")]
impl FieldFromProperties for TriMesh {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>,
    ) -> Result<Self> {
        match <Vec<u8>>::from_properties(attrs, properties) {
            Ok(bytes) => {
                // Special Case: Empty bytes, default mesh data
                if bytes.is_empty() {
                    return Ok(TriMesh::Box);
                }
                let mut reader = &*bytes;
                let out = TriMesh::chomp(&mut reader);
                debug_assert_eq!(*reader, [], "TriMesh didn't consume whole physics buffer");
                out
            }
            Err(e) => Err(e),
        }
    }
}

impl<T: FieldFromProperties> FieldFromProperties for Option<T> {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>,
    ) -> Result<Self> {
        match T::from_properties(attrs, properties) {
            Ok(val) => Ok(Some(val)),
            Err(Error {
                kind: ErrorKind::MissingProperty(_),
                ..
            }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub trait FieldToProperties: Sized {
    fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>);
}

impl<T: ToProperties> FieldToProperties for T {
    fn to_properties(self, _: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
        T::to_properties(&self, properties)
    }
}

impl FieldToProperties for String {
    fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
        let prop = if attrs.shared {
            Property::SharedTextString(self)
        } else {
            Property::TextString(self)
        };

        properties.insert(attrs.field_name.to_string(), prop);
    }
}

impl FieldToProperties for Vec<u8> {
    fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
        let prop = if attrs.shared {
            Property::SharedBinaryString(self)
        } else {
            Property::BinaryString(self)
        };

        properties.insert(attrs.field_name.to_string(), prop);
    }
}

impl FieldToProperties for Attributes {
    fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
        let mut out = Vec::new();
        Attributes::print(&mut out, self).unwrap();
        out.to_properties(attrs, properties);
    }
}

#[cfg(feature = "mesh-format")]
impl FieldToProperties for TriMesh {
    fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
        let mut out = Vec::new();
        TriMesh::print(&mut out, self).unwrap();
        out.to_properties(attrs, properties)
    }
}

impl<T: FieldToProperties> FieldToProperties for Option<T> {
    fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
        if let Some(val) = self {
            T::to_properties(val, attrs, properties);
        }
    }
}

prop_ty_impl! {
    bool : Bool,
    i32 : Int32,
    i64 : Int64,
    f32 : Float,
    f64 : Double,
    UDim : UDim,
    UDim2 : UDim2,
    Ray : Ray,
    Faces : Faces,
    Axes : Axes,
    BrickColor : BrickColor,
    Color3 : Color3,
    Vector2 : Vector2,
    Vector3 : Vector3,
    CFrame : CFrame,
    InstanceRef : InstanceRef,
    Vector3Int16 : Vector3Int16,
    NumberSequence : NumberSequence,
    ColorSequence : ColorSequence,
    NumberRange : NumberRange,
    Rect : Rect,
    PhysicalProperties : PhysicalProperties,
    Color3Uint8 : Color3Uint8,
    Pivot : Pivot,
    Uuid : Uuid,
    FontFace : Font,
}
