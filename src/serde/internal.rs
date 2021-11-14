use crate::model::instance::*;
use crate::model::*;
use crate::serde::{Error, ErrorKind, Result};
#[cfg(feature = "mesh-format")]
use crate::serde::encoding::{Chomp, Print};

use alloc::boxed::Box;
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
                    Some(_) => Err($crate::SerdeError::wrong_property_type(attrs.prop_name.to_string())),
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

macro_rules! prop_enum_impl {
    ($($ty:ty),+ $(,)?) => {
        $(
        impl FieldFromProperties for $ty {
            fn from_properties(attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
                match properties.remove(attrs.prop_name) {
                    Some(Property::Enum(val)) => <$ty>::try_from(val)
                        .map_err(|_| $crate::SerdeError::unknown_variant(val)),
                    Some(_) => Err($crate::SerdeError::wrong_property_type(attrs.prop_name.to_string())),
                    None => Err($crate::SerdeError::missing_property(attrs.prop_name.to_string())),
                }
            }
        }

        impl FieldToProperties for $ty {
            fn to_properties(self, attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) {
                properties.insert(attrs.prop_name.to_string(), Property::Enum(self.into()));
            }
        }
        )*
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RawProperty {
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
    UUID(Uuid),
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
            RawProperty::UUID(val) => Property::UUID(val),
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
            Property::UUID(val) => RawProperty::UUID(val),
        }
    }
}

pub(crate) trait FromProperty: Sized {
    fn from_properties(properties: &mut BTreeMap<String, Property>) -> Result<Self>;
}

pub(crate) trait ToProperty: Sized {
    fn to_properties(&self, properties: &mut BTreeMap<String, Property>);
}

pub(crate) struct FieldAttrs {
    pub field_name: &'static str,
    pub prop_name: &'static str,
    pub shared: bool,
}

pub(crate) trait FieldFromProperties: Sized {
    fn from_properties(
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>
    ) -> Result<Self>;
}

impl FieldFromProperties for String {
    fn from_properties(attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
        if attrs.shared {
            match properties.remove(attrs.prop_name) {
                Some(Property::SharedTextString(val)) => Ok(val),
                Some(_) => Err(Error::wrong_property_type(attrs.prop_name.to_string())),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        } else {
            match properties.remove(attrs.prop_name) {
                Some(Property::TextString(val)) => Ok(val),
                Some(_) => Err(Error::wrong_property_type(attrs.prop_name.to_string())),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        }
    }
}

impl FieldFromProperties for Vec<u8> {
    fn from_properties(attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
        if attrs.shared {
            match properties.remove(attrs.prop_name) {
                Some(Property::SharedBinaryString(val)) => Ok(val),
                Some(Property::SharedTextString(str)) => Ok(str.into_bytes()),
                Some(_) => Err(Error::wrong_property_type(attrs.prop_name.to_string())),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        } else {
            match properties.remove(attrs.prop_name) {
                Some(Property::BinaryString(val)) => Ok(val),
                Some(Property::TextString(str)) => Ok(str.into_bytes()),
                Some(_) => Err(Error::wrong_property_type(attrs.prop_name.to_string())),
                None => Err(Error::missing_property(attrs.prop_name.to_string())),
            }
        }
    }
}

#[cfg(feature = "mesh-format")]
impl FieldFromProperties for TriMesh {
    fn from_properties(attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
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
    fn from_properties(attrs: FieldAttrs, properties: &mut BTreeMap<String, Property>) -> Result<Self> {
        match T::from_properties(attrs, properties) {
            Ok(val) => Ok(Some(val)),
            Err(Error { kind: ErrorKind::MissingProperty(_), .. }) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub(crate) trait FieldToProperties: Sized {
    fn to_properties(
        self,
        attrs: FieldAttrs,
        properties: &mut BTreeMap<String, Property>
    );
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
    crate::tree::TreeKey : InstanceRef,
    Vector3Int16 : Vector3Int16,
    NumberSequence : NumberSequence,
    ColorSequence : ColorSequence,
    NumberRange : NumberRange,
    Rect : Rect,
    PhysicalProperties : PhysicalProperties,
    Color3Uint8 : Color3Uint8,
    Pivot : Pivot,
}

prop_enum_impl! {
    ActuatorRelativeTo, ActuatorType, AdornCullingMode, AlignType, AlphaMode, AnimationPriority,
    ApplyStrokeMode, AspectType, AutomaticSize, Axis, BinType, BodyPart, BorderMode, ButtonStyle,
    CameraType, DialogBehaviorType, DialogPurpose, DialogTone, DominantAxis, EasingDirection,
    EasingStyle, ElasticBehavior, ExplosionType, FieldOfViewMode, FillDirection, Font, FormFactor,
    FrameStyle, HandlesStyle, HorizontalAlignment, HumanoidCollisionType,
    HumanoidDisplayDistanceType, HumanoidHealthDisplayType, HumanoidRigType, InOut, InputType,
    KeyCode, LeftRight, LevelOfDetailSetting, LineJoinMode, Material, MeshType, ModelLevelOfDetail,
    NameOcclusion, NormalId, ParticleOrientation, PartType, PoseEasingDirection, PoseEasingStyle,
    ProximityPromptExclusivity, ProximityPromptStyle, RenderFidelity, RenderingTestComparisonMethod,
    RollOffMode, ScaleType, ScrollBarInset, ScrollingDirection, SizeConstraint, SortOrder,
    StartCorner, SurfaceGuiSizingMode, SurfaceType, TableMajorAxis, TerrainAcquisitionMethod,
    TextTruncate, TextureMode, TextXAlignment, TextYAlignment, TopBottom, TrussStyle,
    VerticalAlignment,VerticalScrollBarPosition, ZIndexBehavior
}

pub(crate) fn make_instance(
    kind: &str,
    mut properties: BTreeMap<String, Property>,
) -> Result<Instance> {
    let out = match kind {
        "Accoutrement" => Instance::Accoutrement(Accoutrement::from_properties(&mut properties)?),
        "Accessory" => Instance::Accessory(Accessory::from_properties(&mut properties)?),
        "Actor" => Instance::Actor(Actor::from_properties(&mut properties)?),
        "AlignOrientation" => {
            Instance::AlignOrientation(AlignOrientation::from_properties(&mut properties)?)
        }
        "AlignPosition" => {
            Instance::AlignPosition(AlignPosition::from_properties(&mut properties)?)
        }
        "AngularVelocity" => {
            Instance::AngularVelocity(AngularVelocity::from_properties(&mut properties)?)
        }
        "Animation" => Instance::Animation(Animation::from_properties(&mut properties)?),
        "AnimationController" => {
            Instance::AnimationController(AnimationController::from_properties(&mut properties)?)
        }
        "ArcHandles" => Instance::ArcHandles(ArcHandles::from_properties(&mut properties)?),
        "Atmosphere" => Instance::Atmosphere(Atmosphere::from_properties(&mut properties)?),
        "Backpack" => Instance::Backpack(Backpack::from_properties(&mut properties)?),
        "BallSocketConstraint" => {
            Instance::BallSocketConstraint(BallSocketConstraint::from_properties(&mut properties)?)
        }
        "Beam" => Instance::Beam(Beam::from_properties(&mut properties)?),
        "BillboardGui" => Instance::BillboardGui(BillboardGui::from_properties(&mut properties)?),
        "BinaryStringValue" => {
            Instance::BinaryStringValue(BinaryStringValue::from_properties(&mut properties)?)
        }
        "BindableEvent" => {
            Instance::BindableEvent(BindableEvent::from_properties(&mut properties)?)
        }
        "BindableFunction" => {
            Instance::BindableFunction(BindableFunction::from_properties(&mut properties)?)
        }
        "BlockMesh" => Instance::BlockMesh(BlockMesh::from_properties(&mut properties)?),
        "BloomEffect" => Instance::BloomEffect(BloomEffect::from_properties(&mut properties)?),
        "BlurEffect" => Instance::BlurEffect(BlurEffect::from_properties(&mut properties)?),
        "BodyAngularVelocity" => {
            Instance::BodyAngularVelocity(BodyAngularVelocity::from_properties(&mut properties)?)
        }
        "BodyColors" => Instance::BodyColors(BodyColors::from_properties(&mut properties)?),
        "BodyForce" => Instance::BodyForce(BodyForce::from_properties(&mut properties)?),
        "BodyGyro" => Instance::BodyGyro(BodyGyro::from_properties(&mut properties)?),
        "BodyPosition" => Instance::BodyPosition(BodyPosition::from_properties(&mut properties)?),
        "BodyThrust" => Instance::BodyThrust(BodyThrust::from_properties(&mut properties)?),
        "BodyVelocity" => Instance::BodyVelocity(BodyVelocity::from_properties(&mut properties)?),
        "BoolValue" => Instance::BoolValue(BoolValue::from_properties(&mut properties)?),
        "BoxHandleAdornment" => {
            Instance::BoxHandleAdornment(BoxHandleAdornment::from_properties(&mut properties)?)
        }
        "BrickColorValue" => {
            Instance::BrickColorValue(BrickColorValue::from_properties(&mut properties)?)
        }
        "Camera" => Instance::Camera(Camera::from_properties(&mut properties)?),
        "CFrameValue" => Instance::CFrameValue(CFrameValue::from_properties(&mut properties)?),
        "CharacterMesh" => {
            Instance::CharacterMesh(CharacterMesh::from_properties(&mut properties)?)
        }
        "ChorusSoundEffect" => {
            Instance::ChorusSoundEffect(ChorusSoundEffect::from_properties(&mut properties)?)
        }
        "ClickDetector" => {
            Instance::ClickDetector(ClickDetector::from_properties(&mut properties)?)
        }
        "Clouds" => Instance::Clouds(Clouds::from_properties(&mut properties)?),
        "Color3Value" => Instance::Color3Value(Color3Value::from_properties(&mut properties)?),
        "ColorCorrectionEffect" => Instance::ColorCorrectionEffect(
            ColorCorrectionEffect::from_properties(&mut properties)?,
        ),
        "CompressorSoundEffect" => Instance::CompressorSoundEffect(
            CompressorSoundEffect::from_properties(&mut properties)?,
        ),
        "ConeHandleAdornment" => {
            Instance::ConeHandleAdornment(ConeHandleAdornment::from_properties(&mut properties)?)
        }
        "Configuration" => {
            Instance::Configuration(Configuration::from_properties(&mut properties)?)
        }
        "CornerWedgePart" => {
            Instance::CornerWedgePart(CornerWedgePart::from_properties(&mut properties)?)
        }
        "CustomEvent" => Instance::CustomEvent(CustomEvent::from_properties(&mut properties)?),
        "CustomEventReceiver" => {
            Instance::CustomEventReceiver(CustomEventReceiver::from_properties(&mut properties)?)
        }
        "CylinderHandleAdornment" => Instance::CylinderHandleAdornment(
            CylinderHandleAdornment::from_properties(&mut properties)?,
        ),
        "CylinderMesh" => Instance::CylinderMesh(CylinderMesh::from_properties(&mut properties)?),
        "CylindricalConstraint" => Instance::CylindricalConstraint(
            CylindricalConstraint::from_properties(&mut properties)?,
        ),
        "Decal" => Instance::Decal(Decal::from_properties(&mut properties)?),
        "DepthOfFieldEffect" => {
            Instance::DepthOfFieldEffect(DepthOfFieldEffect::from_properties(&mut properties)?)
        }
        "Dialog" => Instance::Dialog(Dialog::from_properties(&mut properties)?),
        "DialogChoice" => Instance::DialogChoice(DialogChoice::from_properties(&mut properties)?),
        "DistortionSoundEffect" => Instance::DistortionSoundEffect(
            DistortionSoundEffect::from_properties(&mut properties)?,
        ),
        "DoubleConstrainedValue" => Instance::DoubleConstrainedValue(
            DoubleConstrainedValue::from_properties(&mut properties)?,
        ),
        "EchoSoundEffect" => {
            Instance::EchoSoundEffect(EchoSoundEffect::from_properties(&mut properties)?)
        }
        "EqualizerSoundEffect" => {
            Instance::EqualizerSoundEffect(EqualizerSoundEffect::from_properties(&mut properties)?)
        }
        "Explosion" => Instance::Explosion(Explosion::from_properties(&mut properties)?),
        "FileMesh" => Instance::FileMesh(FileMesh::from_properties(&mut properties)?),
        "Fire" => Instance::Fire(Fire::from_properties(&mut properties)?),
        "Flag" => Instance::Flag(Flag::from_properties(&mut properties)?),
        "FlagStand" => Instance::FlagStand(FlagStand::from_properties(&mut properties)?),
        "FlangeSoundEffect" => {
            Instance::FlangeSoundEffect(FlangeSoundEffect::from_properties(&mut properties)?)
        }
        "FloorWire" => Instance::FloorWire(FloorWire::from_properties(&mut properties)?),
        "Folder" => Instance::Folder(Folder::from_properties(&mut properties)?),
        "ForceField" => Instance::ForceField(ForceField::from_properties(&mut properties)?),
        "Frame" => Instance::Frame(Frame::from_properties(&mut properties)?),
        "FunctionalTest" => {
            Instance::FunctionalTest(FunctionalTest::from_properties(&mut properties)?)
        }
        "Glue" => Instance::Glue(Glue::from_properties(&mut properties)?),
        "GuiMain" => Instance::GuiMain(GuiMain::from_properties(&mut properties)?),
        "Handles" => Instance::Handles(Handles::from_properties(&mut properties)?),
        "Hat" => Instance::Hat(Hat::from_properties(&mut properties)?),
        "HingeConstraint" => {
            Instance::HingeConstraint(HingeConstraint::from_properties(&mut properties)?)
        }
        "Hint" => Instance::Hint(Hint::from_properties(&mut properties)?),
        "Hole" => Instance::Hole(Hole::from_properties(&mut properties)?),
        "HopperBin" => Instance::HopperBin(HopperBin::from_properties(&mut properties)?),
        "Humanoid" => Instance::Humanoid(Humanoid::from_properties(&mut properties)?),
        "HumanoidController" => {
            Instance::HumanoidController(HumanoidController::from_properties(&mut properties)?)
        }
        "HumanoidDescription" => Instance::HumanoidDescription(Box::new(
            HumanoidDescription::from_properties(&mut properties)?,
        )),
        "ImageButton" => Instance::ImageButton(ImageButton::from_properties(&mut properties)?),
        "ImageHandleAdornment" => {
            Instance::ImageHandleAdornment(ImageHandleAdornment::from_properties(&mut properties)?)
        }
        "ImageLabel" => Instance::ImageLabel(ImageLabel::from_properties(&mut properties)?),
        "IntConstrainedValue" => {
            Instance::IntConstrainedValue(IntConstrainedValue::from_properties(&mut properties)?)
        }
        "IntValue" => Instance::IntValue(IntValue::from_properties(&mut properties)?),
        "Keyframe" => Instance::Keyframe(Keyframe::from_properties(&mut properties)?),
        "KeyframeMarker" => {
            Instance::KeyframeMarker(KeyframeMarker::from_properties(&mut properties)?)
        }
        "KeyframeSequence" => {
            Instance::KeyframeSequence(KeyframeSequence::from_properties(&mut properties)?)
        }
        "LineForce" => Instance::LineForce(LineForce::from_properties(&mut properties)?),
        "LineHandleAdornment" => {
            Instance::LineHandleAdornment(LineHandleAdornment::from_properties(&mut properties)?)
        }
        "LocalizationTable" => {
            Instance::LocalizationTable(LocalizationTable::from_properties(&mut properties)?)
        }
        "LocalScript" => Instance::LocalScript(LocalScript::from_properties(&mut properties)?),
        "ManualGlue" => Instance::ManualGlue(ManualGlue::from_properties(&mut properties)?),
        "ManualWeld" => Instance::ManualWeld(ManualWeld::from_properties(&mut properties)?),
        "MeshPart" => Instance::MeshPart(MeshPart::from_properties(&mut properties)?),
        "Message" => Instance::Message(Message::from_properties(&mut properties)?),
        "Model" => Instance::Model(Model::from_properties(&mut properties)?),
        "ModuleScript" => Instance::ModuleScript(ModuleScript::from_properties(&mut properties)?),
        "Motor" => Instance::Motor(Motor::from_properties(&mut properties)?),
        "Motor6D" => Instance::Motor6D(Motor6D::from_properties(&mut properties)?),
        "MotorFeature" => Instance::MotorFeature(MotorFeature::from_properties(&mut properties)?),
        "NegateOperation" => {
            Instance::NegateOperation(NegateOperation::from_properties(&mut properties)?)
        }
        "NoCollisionConstraint" => Instance::NoCollisionConstraint(
            NoCollisionConstraint::from_properties(&mut properties)?,
        ),
        "NumberPose" => Instance::NumberPose(NumberPose::from_properties(&mut properties)?),
        "NumberValue" => Instance::NumberValue(NumberValue::from_properties(&mut properties)?),
        "ObjectValue" => Instance::ObjectValue(ObjectValue::from_properties(&mut properties)?),
        "Pants" => Instance::Pants(Pants::from_properties(&mut properties)?),
        "Part" => Instance::Part(Part::from_properties(&mut properties)?),
        "ParticleEmitter" => {
            Instance::ParticleEmitter(ParticleEmitter::from_properties(&mut properties)?)
        }
        "PartOperation" => {
            Instance::PartOperation(PartOperation::from_properties(&mut properties)?)
        }
        "PartOperationAsset" => {
            Instance::PartOperationAsset(PartOperationAsset::from_properties(&mut properties)?)
        }
        "PitchShiftSoundEffect" => Instance::PitchShiftSoundEffect(
            PitchShiftSoundEffect::from_properties(&mut properties)?,
        ),
        "PointLight" => Instance::PointLight(PointLight::from_properties(&mut properties)?),
        "Pose" => Instance::Pose(Pose::from_properties(&mut properties)?),
        "PrismaticConstraint" => {
            Instance::PrismaticConstraint(PrismaticConstraint::from_properties(&mut properties)?)
        }
        "ProximityPrompt" => {
            Instance::ProximityPrompt(ProximityPrompt::from_properties(&mut properties)?)
        }
        "RayValue" => Instance::RayValue(RayValue::from_properties(&mut properties)?),
        "ReflectionMetadata" => {
            Instance::ReflectionMetadata(ReflectionMetadata::from_properties(&mut properties)?)
        }
        "ReflectionMetadataCallbacks" => Instance::ReflectionMetadataCallbacks(
            ReflectionMetadataCallbacks::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataClass" => Instance::ReflectionMetadataClass(
            ReflectionMetadataClass::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataClasses" => Instance::ReflectionMetadataClasses(
            ReflectionMetadataClasses::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEnum" => Instance::ReflectionMetadataEnum(
            ReflectionMetadataEnum::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEnumItem" => Instance::ReflectionMetadataEnumItem(
            ReflectionMetadataEnumItem::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEnums" => Instance::ReflectionMetadataEnums(
            ReflectionMetadataEnums::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEvents" => Instance::ReflectionMetadataEvents(
            ReflectionMetadataEvents::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataFunctions" => Instance::ReflectionMetadataFunctions(
            ReflectionMetadataFunctions::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataMember" => Instance::ReflectionMetadataMember(
            ReflectionMetadataMember::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataProperties" => Instance::ReflectionMetadataProperties(
            ReflectionMetadataProperties::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataYieldFunctions" => Instance::ReflectionMetadataYieldFunctions(
            ReflectionMetadataYieldFunctions::from_properties(&mut properties)?,
        ),
        "RemoteEvent" => Instance::RemoteEvent(RemoteEvent::from_properties(&mut properties)?),
        "RemoteFunction" => {
            Instance::RemoteFunction(RemoteFunction::from_properties(&mut properties)?)
        }
        "RenderingTest" => {
            Instance::RenderingTest(RenderingTest::from_properties(&mut properties)?)
        }
        "ReverbSoundEffect" => {
            Instance::ReverbSoundEffect(ReverbSoundEffect::from_properties(&mut properties)?)
        }
        "RocketPropulsion" => {
            Instance::RocketPropulsion(RocketPropulsion::from_properties(&mut properties)?)
        }
        "RodConstraint" => {
            Instance::RodConstraint(RodConstraint::from_properties(&mut properties)?)
        }
        "RopeConstraint" => {
            Instance::RopeConstraint(RopeConstraint::from_properties(&mut properties)?)
        }
        "Rotate" => Instance::Rotate(Rotate::from_properties(&mut properties)?),
        "RotateP" => Instance::RotateP(RotateP::from_properties(&mut properties)?),
        "RotateV" => Instance::RotateV(RotateV::from_properties(&mut properties)?),
        "ScreenGui" => Instance::ScreenGui(ScreenGui::from_properties(&mut properties)?),
        "Script" => Instance::Script(Script::from_properties(&mut properties)?),
        "ScrollingFrame" => {
            Instance::ScrollingFrame(ScrollingFrame::from_properties(&mut properties)?)
        }
        "Seat" => Instance::Seat(Seat::from_properties(&mut properties)?),
        "SelectionBox" => Instance::SelectionBox(SelectionBox::from_properties(&mut properties)?),
        "SelectionPartLasso" => {
            Instance::SelectionPartLasso(SelectionPartLasso::from_properties(&mut properties)?)
        }
        "SelectionPointLasso" => {
            Instance::SelectionPointLasso(SelectionPointLasso::from_properties(&mut properties)?)
        }
        "SelectionSphere" => {
            Instance::SelectionSphere(SelectionSphere::from_properties(&mut properties)?)
        }
        "Shirt" => Instance::Shirt(Shirt::from_properties(&mut properties)?),
        "ShirtGraphic" => Instance::ShirtGraphic(ShirtGraphic::from_properties(&mut properties)?),
        "SkateboardController" => {
            Instance::SkateboardController(SkateboardController::from_properties(&mut properties)?)
        }
        "SkateboardPlatform" => {
            Instance::SkateboardPlatform(SkateboardPlatform::from_properties(&mut properties)?)
        }
        "Skin" => Instance::Skin(Skin::from_properties(&mut properties)?),
        "Sky" => Instance::Sky(Sky::from_properties(&mut properties)?),
        "Smoke" => Instance::Smoke(Smoke::from_properties(&mut properties)?),
        "Snap" => Instance::Snap(Snap::from_properties(&mut properties)?),
        "Sound" => Instance::Sound(Sound::from_properties(&mut properties)?),
        "SoundGroup" => Instance::SoundGroup(SoundGroup::from_properties(&mut properties)?),
        "Sparkles" => Instance::Sparkles(Sparkles::from_properties(&mut properties)?),
        "SpawnLocation" => {
            Instance::SpawnLocation(SpawnLocation::from_properties(&mut properties)?)
        }
        "SpecialMesh" => Instance::SpecialMesh(SpecialMesh::from_properties(&mut properties)?),
        "SphereHandleAdornment" => Instance::SphereHandleAdornment(
            SphereHandleAdornment::from_properties(&mut properties)?,
        ),
        "SpotLight" => Instance::SpotLight(SpotLight::from_properties(&mut properties)?),
        "SpringConstraint" => {
            Instance::SpringConstraint(SpringConstraint::from_properties(&mut properties)?)
        }
        "StandalonePluginScripts" => Instance::StandalonePluginScripts(
            StandalonePluginScripts::from_properties(&mut properties)?,
        ),
        "StarterGear" => Instance::StarterGear(StarterGear::from_properties(&mut properties)?),
        "StringValue" => Instance::StringValue(StringValue::from_properties(&mut properties)?),
        "SunRaysEffect" => {
            Instance::SunRaysEffect(SunRaysEffect::from_properties(&mut properties)?)
        }
        "SurfaceAppearance" => {
            Instance::SurfaceAppearance(SurfaceAppearance::from_properties(&mut properties)?)
        }
        "SurfaceGui" => Instance::SurfaceGui(SurfaceGui::from_properties(&mut properties)?),
        "SurfaceLight" => Instance::SurfaceLight(SurfaceLight::from_properties(&mut properties)?),
        "SurfaceSelection" => {
            Instance::SurfaceSelection(SurfaceSelection::from_properties(&mut properties)?)
        }
        "Team" => Instance::Team(Team::from_properties(&mut properties)?),
        "TeleportOptions" => {
            Instance::TeleportOptions(TeleportOptions::from_properties(&mut properties)?)
        }
        "Terrain" => Instance::Terrain(Terrain::from_properties(&mut properties)?),
        "TerrainRegion" => {
            Instance::TerrainRegion(TerrainRegion::from_properties(&mut properties)?)
        }
        "TextBox" => Instance::TextBox(TextBox::from_properties(&mut properties)?),
        "TextButton" => Instance::TextButton(TextButton::from_properties(&mut properties)?),
        "TextLabel" => Instance::TextLabel(TextLabel::from_properties(&mut properties)?),
        "Texture" => Instance::Texture(Texture::from_properties(&mut properties)?),
        "Tool" => Instance::Tool(Tool::from_properties(&mut properties)?),
        "Torque" => Instance::Torque(Torque::from_properties(&mut properties)?),
        "Trail" => Instance::Trail(Trail::from_properties(&mut properties)?),
        "TremoloSoundEffect" => {
            Instance::TremoloSoundEffect(TremoloSoundEffect::from_properties(&mut properties)?)
        }
        "TrussPart" => Instance::TrussPart(TrussPart::from_properties(&mut properties)?),
        "Tween" => Instance::Tween(Tween::from_properties(&mut properties)?),
        "UIAspectRatioConstraint" => Instance::UIAspectRatioConstraint(
            UIAspectRatioConstraint::from_properties(&mut properties)?,
        ),
        "UICorner" => Instance::UICorner(UICorner::from_properties(&mut properties)?),
        "UIGradient" => Instance::UIGradient(UIGradient::from_properties(&mut properties)?),
        "UIGridLayout" => Instance::UIGridLayout(UIGridLayout::from_properties(&mut properties)?),
        "UIListLayout" => Instance::UIListLayout(UIListLayout::from_properties(&mut properties)?),
        "UIPadding" => Instance::UIPadding(UIPadding::from_properties(&mut properties)?),
        "UIPageLayout" => Instance::UIPageLayout(UIPageLayout::from_properties(&mut properties)?),
        "UIScale" => Instance::UIScale(UIScale::from_properties(&mut properties)?),
        "UISizeConstraint" => {
            Instance::UISizeConstraint(UISizeConstraint::from_properties(&mut properties)?)
        }
        "UIStroke" => Instance::UIStroke(UIStroke::from_properties(&mut properties)?),
        "UITableLayout" => {
            Instance::UITableLayout(UITableLayout::from_properties(&mut properties)?)
        }
        "UITextSizeConstraint" => {
            Instance::UITextSizeConstraint(UITextSizeConstraint::from_properties(&mut properties)?)
        }
        "UnionOperation" => {
            Instance::UnionOperation(UnionOperation::from_properties(&mut properties)?)
        }
        "UniversalConstraint" => {
            Instance::UniversalConstraint(UniversalConstraint::from_properties(&mut properties)?)
        }
        "Vector3Value" => Instance::Vector3Value(Vector3Value::from_properties(&mut properties)?),
        "VectorForce" => Instance::VectorForce(VectorForce::from_properties(&mut properties)?),
        "VehicleController" => {
            Instance::VehicleController(VehicleController::from_properties(&mut properties)?)
        }
        "VehicleSeat" => Instance::VehicleSeat(VehicleSeat::from_properties(&mut properties)?),
        "VelocityMotor" => {
            Instance::VelocityMotor(VelocityMotor::from_properties(&mut properties)?)
        }
        "VideoFrame" => Instance::VideoFrame(VideoFrame::from_properties(&mut properties)?),
        "ViewportFrame" => {
            Instance::ViewportFrame(ViewportFrame::from_properties(&mut properties)?)
        }
        "WedgePart" => Instance::WedgePart(WedgePart::from_properties(&mut properties)?),
        "Weld" => Instance::Weld(Weld::from_properties(&mut properties)?),
        "WeldConstraint" => {
            Instance::WeldConstraint(WeldConstraint::from_properties(&mut properties)?)
        }
        "WorldModel" => Instance::WorldModel(WorldModel::from_properties(&mut properties)?),
        _ => {
            let kind = Instance::Other(
                kind.to_string(),
                properties
                    .iter()
                    .map(|(key, val)| (key.clone(), val.clone()))
                    .collect(),
            );
            properties.clear();
            kind
        }
    };

    if !properties.is_empty() {
        Err(Error::unconsumed_properties(
            out.class_name(),
            properties.into_iter().map(|(keys, _)| keys).collect(),
        ))
    } else {
        Ok(out)
    }
}

pub(crate) fn break_instance(kind: &Instance) -> BTreeMap<String, Property> {
    let mut properties = BTreeMap::new();

    match kind {
        Instance::Accoutrement(data) => data.to_properties(&mut properties),
        Instance::Accessory(data) => data.to_properties(&mut properties),
        Instance::Actor(data) => data.to_properties(&mut properties),
        Instance::AlignOrientation(data) => data.to_properties(&mut properties),
        Instance::AlignPosition(data) => data.to_properties(&mut properties),
        Instance::AngularVelocity(data) => data.to_properties(&mut properties),
        Instance::Animation(data) => data.to_properties(&mut properties),
        Instance::AnimationController(data) => data.to_properties(&mut properties),
        Instance::ArcHandles(data) => data.to_properties(&mut properties),
        Instance::Atmosphere(data) => data.to_properties(&mut properties),
        Instance::Backpack(data) => data.to_properties(&mut properties),
        Instance::BallSocketConstraint(data) => data.to_properties(&mut properties),
        Instance::Beam(data) => data.to_properties(&mut properties),
        Instance::BillboardGui(data) => data.to_properties(&mut properties),
        Instance::BinaryStringValue(data) => data.to_properties(&mut properties),
        Instance::BindableEvent(data) => data.to_properties(&mut properties),
        Instance::BindableFunction(data) => data.to_properties(&mut properties),
        Instance::BlockMesh(data) => data.to_properties(&mut properties),
        Instance::BloomEffect(data) => data.to_properties(&mut properties),
        Instance::BlurEffect(data) => data.to_properties(&mut properties),
        Instance::BodyAngularVelocity(data) => data.to_properties(&mut properties),
        Instance::BodyColors(data) => data.to_properties(&mut properties),
        Instance::BodyForce(data) => data.to_properties(&mut properties),
        Instance::BodyGyro(data) => data.to_properties(&mut properties),
        Instance::BodyPosition(data) => data.to_properties(&mut properties),
        Instance::BodyThrust(data) => data.to_properties(&mut properties),
        Instance::BodyVelocity(data) => data.to_properties(&mut properties),
        Instance::BoolValue(data) => data.to_properties(&mut properties),
        Instance::BoxHandleAdornment(data) => data.to_properties(&mut properties),
        Instance::BrickColorValue(data) => data.to_properties(&mut properties),
        Instance::Camera(data) => data.to_properties(&mut properties),
        Instance::CFrameValue(data) => data.to_properties(&mut properties),
        Instance::CharacterMesh(data) => data.to_properties(&mut properties),
        Instance::ChorusSoundEffect(data) => data.to_properties(&mut properties),
        Instance::ClickDetector(data) => data.to_properties(&mut properties),
        Instance::Clouds(data) => data.to_properties(&mut properties),
        Instance::Color3Value(data) => data.to_properties(&mut properties),
        Instance::ColorCorrectionEffect(data) => data.to_properties(&mut properties),
        Instance::CompressorSoundEffect(data) => data.to_properties(&mut properties),
        Instance::ConeHandleAdornment(data) => data.to_properties(&mut properties),
        Instance::Configuration(data) => data.to_properties(&mut properties),
        Instance::CornerWedgePart(data) => data.to_properties(&mut properties),
        Instance::CustomEvent(data) => data.to_properties(&mut properties),
        Instance::CustomEventReceiver(data) => data.to_properties(&mut properties),
        Instance::CylinderHandleAdornment(data) => data.to_properties(&mut properties),
        Instance::CylinderMesh(data) => data.to_properties(&mut properties),
        Instance::CylindricalConstraint(data) => data.to_properties(&mut properties),
        Instance::Decal(data) => data.to_properties(&mut properties),
        Instance::DepthOfFieldEffect(data) => data.to_properties(&mut properties),
        Instance::Dialog(data) => data.to_properties(&mut properties),
        Instance::DialogChoice(data) => data.to_properties(&mut properties),
        Instance::DistortionSoundEffect(data) => data.to_properties(&mut properties),
        Instance::DoubleConstrainedValue(data) => data.to_properties(&mut properties),
        Instance::EchoSoundEffect(data) => data.to_properties(&mut properties),
        Instance::EqualizerSoundEffect(data) => data.to_properties(&mut properties),
        Instance::Explosion(data) => data.to_properties(&mut properties),
        Instance::FileMesh(data) => data.to_properties(&mut properties),
        Instance::Fire(data) => data.to_properties(&mut properties),
        Instance::Flag(data) => data.to_properties(&mut properties),
        Instance::FlagStand(data) => data.to_properties(&mut properties),
        Instance::FlangeSoundEffect(data) => data.to_properties(&mut properties),
        Instance::FloorWire(data) => data.to_properties(&mut properties),
        Instance::Folder(data) => data.to_properties(&mut properties),
        Instance::ForceField(data) => data.to_properties(&mut properties),
        Instance::Frame(data) => data.to_properties(&mut properties),
        Instance::FunctionalTest(data) => data.to_properties(&mut properties),
        Instance::Glue(data) => data.to_properties(&mut properties),
        Instance::GuiMain(data) => data.to_properties(&mut properties),
        Instance::Handles(data) => data.to_properties(&mut properties),
        Instance::Hat(data) => data.to_properties(&mut properties),
        Instance::HingeConstraint(data) => data.to_properties(&mut properties),
        Instance::Hint(data) => data.to_properties(&mut properties),
        Instance::Hole(data) => data.to_properties(&mut properties),
        Instance::HopperBin(data) => data.to_properties(&mut properties),
        Instance::Humanoid(data) => data.to_properties(&mut properties),
        Instance::HumanoidController(data) => data.to_properties(&mut properties),
        Instance::HumanoidDescription(data) => data.to_properties(&mut properties),
        Instance::ImageButton(data) => data.to_properties(&mut properties),
        Instance::ImageHandleAdornment(data) => data.to_properties(&mut properties),
        Instance::ImageLabel(data) => data.to_properties(&mut properties),
        Instance::IntConstrainedValue(data) => data.to_properties(&mut properties),
        Instance::IntValue(data) => data.to_properties(&mut properties),
        Instance::Keyframe(data) => data.to_properties(&mut properties),
        Instance::KeyframeMarker(data) => data.to_properties(&mut properties),
        Instance::KeyframeSequence(data) => data.to_properties(&mut properties),
        Instance::LineForce(data) => data.to_properties(&mut properties),
        Instance::LineHandleAdornment(data) => data.to_properties(&mut properties),
        Instance::LocalizationTable(data) => data.to_properties(&mut properties),
        Instance::LocalScript(data) => data.to_properties(&mut properties),
        Instance::ManualGlue(data) => data.to_properties(&mut properties),
        Instance::ManualWeld(data) => data.to_properties(&mut properties),
        Instance::MeshPart(data) => data.to_properties(&mut properties),
        Instance::Message(data) => data.to_properties(&mut properties),
        Instance::Model(data) => data.to_properties(&mut properties),
        Instance::ModuleScript(data) => data.to_properties(&mut properties),
        Instance::Motor(data) => data.to_properties(&mut properties),
        Instance::Motor6D(data) => data.to_properties(&mut properties),
        Instance::MotorFeature(data) => data.to_properties(&mut properties),
        Instance::NegateOperation(data) => data.to_properties(&mut properties),
        Instance::NoCollisionConstraint(data) => data.to_properties(&mut properties),
        Instance::NumberPose(data) => data.to_properties(&mut properties),
        Instance::NumberValue(data) => data.to_properties(&mut properties),
        Instance::ObjectValue(data) => data.to_properties(&mut properties),
        Instance::Pants(data) => data.to_properties(&mut properties),
        Instance::Part(data) => data.to_properties(&mut properties),
        Instance::ParticleEmitter(data) => data.to_properties(&mut properties),
        Instance::PartOperation(data) => data.to_properties(&mut properties),
        Instance::PartOperationAsset(data) => data.to_properties(&mut properties),
        Instance::PitchShiftSoundEffect(data) => data.to_properties(&mut properties),
        Instance::PointLight(data) => data.to_properties(&mut properties),
        Instance::Pose(data) => data.to_properties(&mut properties),
        Instance::PrismaticConstraint(data) => data.to_properties(&mut properties),
        Instance::ProximityPrompt(data) => data.to_properties(&mut properties),
        Instance::RayValue(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadata(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataCallbacks(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataClass(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataClasses(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataEnum(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataEnumItem(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataEnums(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataEvents(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataFunctions(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataMember(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataProperties(data) => data.to_properties(&mut properties),
        Instance::ReflectionMetadataYieldFunctions(data) => data.to_properties(&mut properties),
        Instance::RemoteEvent(data) => data.to_properties(&mut properties),
        Instance::RemoteFunction(data) => data.to_properties(&mut properties),
        Instance::RenderingTest(data) => data.to_properties(&mut properties),
        Instance::ReverbSoundEffect(data) => data.to_properties(&mut properties),
        Instance::RocketPropulsion(data) => data.to_properties(&mut properties),
        Instance::RodConstraint(data) => data.to_properties(&mut properties),
        Instance::RopeConstraint(data) => data.to_properties(&mut properties),
        Instance::Rotate(data) => data.to_properties(&mut properties),
        Instance::RotateP(data) => data.to_properties(&mut properties),
        Instance::RotateV(data) => data.to_properties(&mut properties),
        Instance::ScreenGui(data) => data.to_properties(&mut properties),
        Instance::Script(data) => data.to_properties(&mut properties),
        Instance::ScrollingFrame(data) => data.to_properties(&mut properties),
        Instance::Seat(data) => data.to_properties(&mut properties),
        Instance::SelectionBox(data) => data.to_properties(&mut properties),
        Instance::SelectionPartLasso(data) => data.to_properties(&mut properties),
        Instance::SelectionPointLasso(data) => data.to_properties(&mut properties),
        Instance::SelectionSphere(data) => data.to_properties(&mut properties),
        Instance::Shirt(data) => data.to_properties(&mut properties),
        Instance::ShirtGraphic(data) => data.to_properties(&mut properties),
        Instance::SkateboardController(data) => data.to_properties(&mut properties),
        Instance::SkateboardPlatform(data) => data.to_properties(&mut properties),
        Instance::Skin(data) => data.to_properties(&mut properties),
        Instance::Sky(data) => data.to_properties(&mut properties),
        Instance::Smoke(data) => data.to_properties(&mut properties),
        Instance::Snap(data) => data.to_properties(&mut properties),
        Instance::Sound(data) => data.to_properties(&mut properties),
        Instance::SoundGroup(data) => data.to_properties(&mut properties),
        Instance::Sparkles(data) => data.to_properties(&mut properties),
        Instance::SpawnLocation(data) => data.to_properties(&mut properties),
        Instance::SpecialMesh(data) => data.to_properties(&mut properties),
        Instance::SphereHandleAdornment(data) => data.to_properties(&mut properties),
        Instance::SpotLight(data) => data.to_properties(&mut properties),
        Instance::SpringConstraint(data) => data.to_properties(&mut properties),
        Instance::StandalonePluginScripts(data) => data.to_properties(&mut properties),
        Instance::StarterGear(data) => data.to_properties(&mut properties),
        Instance::StringValue(data) => data.to_properties(&mut properties),
        Instance::SunRaysEffect(data) => data.to_properties(&mut properties),
        Instance::SurfaceAppearance(data) => data.to_properties(&mut properties),
        Instance::SurfaceGui(data) => data.to_properties(&mut properties),
        Instance::SurfaceLight(data) => data.to_properties(&mut properties),
        Instance::SurfaceSelection(data) => data.to_properties(&mut properties),
        Instance::Team(data) => data.to_properties(&mut properties),
        Instance::TeleportOptions(data) => data.to_properties(&mut properties),
        Instance::Terrain(data) => data.to_properties(&mut properties),
        Instance::TerrainRegion(data) => data.to_properties(&mut properties),
        Instance::TextBox(data) => data.to_properties(&mut properties),
        Instance::TextButton(data) => data.to_properties(&mut properties),
        Instance::TextLabel(data) => data.to_properties(&mut properties),
        Instance::Texture(data) => data.to_properties(&mut properties),
        Instance::Tool(data) => data.to_properties(&mut properties),
        Instance::Torque(data) => data.to_properties(&mut properties),
        Instance::Trail(data) => data.to_properties(&mut properties),
        Instance::TremoloSoundEffect(data) => data.to_properties(&mut properties),
        Instance::TrussPart(data) => data.to_properties(&mut properties),
        Instance::Tween(data) => data.to_properties(&mut properties),
        Instance::UIAspectRatioConstraint(data) => data.to_properties(&mut properties),
        Instance::UICorner(data) => data.to_properties(&mut properties),
        Instance::UIGradient(data) => data.to_properties(&mut properties),
        Instance::UIGridLayout(data) => data.to_properties(&mut properties),
        Instance::UIListLayout(data) => data.to_properties(&mut properties),
        Instance::UIPadding(data) => data.to_properties(&mut properties),
        Instance::UIPageLayout(data) => data.to_properties(&mut properties),
        Instance::UIScale(data) => data.to_properties(&mut properties),
        Instance::UISizeConstraint(data) => data.to_properties(&mut properties),
        Instance::UIStroke(data) => data.to_properties(&mut properties),
        Instance::UITableLayout(data) => data.to_properties(&mut properties),
        Instance::UITextSizeConstraint(data) => data.to_properties(&mut properties),
        Instance::UnionOperation(data) => data.to_properties(&mut properties),
        Instance::UniversalConstraint(data) => data.to_properties(&mut properties),
        Instance::Vector3Value(data) => data.to_properties(&mut properties),
        Instance::VectorForce(data) => data.to_properties(&mut properties),
        Instance::VehicleController(data) => data.to_properties(&mut properties),
        Instance::VehicleSeat(data) => data.to_properties(&mut properties),
        Instance::VelocityMotor(data) => data.to_properties(&mut properties),
        Instance::VideoFrame(data) => data.to_properties(&mut properties),
        Instance::ViewportFrame(data) => data.to_properties(&mut properties),
        Instance::WedgePart(data) => data.to_properties(&mut properties),
        Instance::Weld(data) => data.to_properties(&mut properties),
        Instance::WeldConstraint(data) => data.to_properties(&mut properties),
        Instance::WorldModel(data) => data.to_properties(&mut properties),
        Instance::Other(_, data) => properties.extend(data.clone()),
    }

    properties
}
