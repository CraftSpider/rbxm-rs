use crate::model::instance::*;
use crate::model::*;
use crate::serde::{Error, Result};
use std::collections::HashMap;

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
    RawSharedString(i32),
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
            RawProperty::RawSharedString(..) => unreachable!(),
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
            Property::Faces(val) => RawProperty::Face(val),
            Property::Axes(val) => RawProperty::Axis(val),
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

pub(crate) trait FromProperty: Sized {
    fn from_properties(properties: &mut HashMap<String, Property>) -> Result<Self>;
}

pub(crate) trait ToProperty: Sized {
    fn to_properties(&self, properties: &mut HashMap<String, Property>);
}

pub fn make_kind(kind: &str, mut properties: HashMap<String, Property>) -> Result<InstanceKind> {
    let out = match kind {
        "Accoutrement" => {
            InstanceKind::Accoutrement(Accoutrement::from_properties(&mut properties)?)
        }
        "Accessory" => InstanceKind::Accessory(Accessory::from_properties(&mut properties)?),
        "Actor" => InstanceKind::Actor(Actor::from_properties(&mut properties)?),
        "AlignOrientation" => {
            InstanceKind::AlignOrientation(AlignOrientation::from_properties(&mut properties)?)
        }
        "AlignPosition" => {
            InstanceKind::AlignPosition(AlignPosition::from_properties(&mut properties)?)
        }
        "AngularVelocity" => {
            InstanceKind::AngularVelocity(AngularVelocity::from_properties(&mut properties)?)
        }
        "Animation" => InstanceKind::Animation(Animation::from_properties(&mut properties)?),
        "AnimationController" => InstanceKind::AnimationController(
            AnimationController::from_properties(&mut properties)?,
        ),
        "ArcHandles" => InstanceKind::ArcHandles(ArcHandles::from_properties(&mut properties)?),
        "Atmosphere" => InstanceKind::Atmosphere(Atmosphere::from_properties(&mut properties)?),
        "Backpack" => InstanceKind::Backpack(Backpack::from_properties(&mut properties)?),
        "BallSocketConstraint" => InstanceKind::BallSocketConstraint(
            BallSocketConstraint::from_properties(&mut properties)?,
        ),
        "Beam" => InstanceKind::Beam(Beam::from_properties(&mut properties)?),
        "BillboardGui" => {
            InstanceKind::BillboardGui(BillboardGui::from_properties(&mut properties)?)
        }
        "BinaryStringValue" => {
            InstanceKind::BinaryStringValue(BinaryStringValue::from_properties(&mut properties)?)
        }
        "BindableEvent" => {
            InstanceKind::BindableEvent(BindableEvent::from_properties(&mut properties)?)
        }
        "BindableFunction" => {
            InstanceKind::BindableFunction(BindableFunction::from_properties(&mut properties)?)
        }
        "BlockMesh" => InstanceKind::BlockMesh(BlockMesh::from_properties(&mut properties)?),
        "BloomEffect" => InstanceKind::BloomEffect(BloomEffect::from_properties(&mut properties)?),
        "BlurEffect" => InstanceKind::BlurEffect(BlurEffect::from_properties(&mut properties)?),
        "BodyAngularVelocity" => InstanceKind::BodyAngularVelocity(
            BodyAngularVelocity::from_properties(&mut properties)?,
        ),
        "BodyColors" => InstanceKind::BodyColors(BodyColors::from_properties(&mut properties)?),
        "BodyForce" => InstanceKind::BodyForce(BodyForce::from_properties(&mut properties)?),
        "BodyGyro" => InstanceKind::BodyGyro(BodyGyro::from_properties(&mut properties)?),
        "BodyPosition" => {
            InstanceKind::BodyPosition(BodyPosition::from_properties(&mut properties)?)
        }
        "BodyThrust" => InstanceKind::BodyThrust(BodyThrust::from_properties(&mut properties)?),
        "BodyVelocity" => {
            InstanceKind::BodyVelocity(BodyVelocity::from_properties(&mut properties)?)
        }
        "BoolValue" => InstanceKind::BoolValue(BoolValue::from_properties(&mut properties)?),
        "BoxHandleAdornment" => {
            InstanceKind::BoxHandleAdornment(BoxHandleAdornment::from_properties(&mut properties)?)
        }
        "BrickColorValue" => {
            InstanceKind::BrickColorValue(BrickColorValue::from_properties(&mut properties)?)
        }
        "Camera" => InstanceKind::Camera(Camera::from_properties(&mut properties)?),
        "CFrameValue" => InstanceKind::CFrameValue(CFrameValue::from_properties(&mut properties)?),
        "CharacterMesh" => {
            InstanceKind::CharacterMesh(CharacterMesh::from_properties(&mut properties)?)
        }
        "ChorusSoundEffect" => {
            InstanceKind::ChorusSoundEffect(ChorusSoundEffect::from_properties(&mut properties)?)
        }
        "ClickDetector" => {
            InstanceKind::ClickDetector(ClickDetector::from_properties(&mut properties)?)
        }
        "Clouds" => InstanceKind::Clouds(Clouds::from_properties(&mut properties)?),
        "Color3Value" => InstanceKind::Color3Value(Color3Value::from_properties(&mut properties)?),
        "ColorCorrectionEffect" => InstanceKind::ColorCorrectionEffect(
            ColorCorrectionEffect::from_properties(&mut properties)?,
        ),
        "CompressorSoundEffect" => InstanceKind::CompressorSoundEffect(
            CompressorSoundEffect::from_properties(&mut properties)?,
        ),
        "ConeHandleAdornment" => InstanceKind::ConeHandleAdornment(
            ConeHandleAdornment::from_properties(&mut properties)?,
        ),
        "Configuration" => {
            InstanceKind::Configuration(Configuration::from_properties(&mut properties)?)
        }
        "CornerWedgePart" => {
            InstanceKind::CornerWedgePart(CornerWedgePart::from_properties(&mut properties)?)
        }
        "CustomEvent" => InstanceKind::CustomEvent(CustomEvent::from_properties(&mut properties)?),
        "CustomEventReceiver" => InstanceKind::CustomEventReceiver(
            CustomEventReceiver::from_properties(&mut properties)?,
        ),
        "CylinderHandleAdornment" => InstanceKind::CylinderHandleAdornment(
            CylinderHandleAdornment::from_properties(&mut properties)?,
        ),
        "CylinderMesh" => {
            InstanceKind::CylinderMesh(CylinderMesh::from_properties(&mut properties)?)
        }
        "CylindricalConstraint" => InstanceKind::CylindricalConstraint(
            CylindricalConstraint::from_properties(&mut properties)?,
        ),
        "Decal" => InstanceKind::Decal(Decal::from_properties(&mut properties)?),
        "DepthOfFieldEffect" => {
            InstanceKind::DepthOfFieldEffect(DepthOfFieldEffect::from_properties(&mut properties)?)
        }
        "Dialog" => InstanceKind::Dialog(Dialog::from_properties(&mut properties)?),
        "DialogChoice" => {
            InstanceKind::DialogChoice(DialogChoice::from_properties(&mut properties)?)
        }
        "DistortionSoundEffect" => InstanceKind::DistortionSoundEffect(
            DistortionSoundEffect::from_properties(&mut properties)?,
        ),
        "DoubleConstrainedValue" => InstanceKind::DoubleConstrainedValue(
            DoubleConstrainedValue::from_properties(&mut properties)?,
        ),
        "EchoSoundEffect" => {
            InstanceKind::EchoSoundEffect(EchoSoundEffect::from_properties(&mut properties)?)
        }
        "EqualizerSoundEffect" => InstanceKind::EqualizerSoundEffect(
            EqualizerSoundEffect::from_properties(&mut properties)?,
        ),
        "Explosion" => InstanceKind::Explosion(Explosion::from_properties(&mut properties)?),
        "FileMesh" => InstanceKind::FileMesh(FileMesh::from_properties(&mut properties)?),
        "Fire" => InstanceKind::Fire(Fire::from_properties(&mut properties)?),
        "Flag" => InstanceKind::Flag(Flag::from_properties(&mut properties)?),
        "FlagStand" => InstanceKind::FlagStand(FlagStand::from_properties(&mut properties)?),
        "FlangeSoundEffect" => {
            InstanceKind::FlangeSoundEffect(FlangeSoundEffect::from_properties(&mut properties)?)
        }
        "FloorWire" => InstanceKind::FloorWire(FloorWire::from_properties(&mut properties)?),
        "Folder" => InstanceKind::Folder(Folder::from_properties(&mut properties)?),
        "ForceField" => InstanceKind::ForceField(ForceField::from_properties(&mut properties)?),
        "Frame" => InstanceKind::Frame(Frame::from_properties(&mut properties)?),
        "FunctionalTest" => {
            InstanceKind::FunctionalTest(FunctionalTest::from_properties(&mut properties)?)
        }
        "Glue" => InstanceKind::Glue(Glue::from_properties(&mut properties)?),
        "GuiMain" => InstanceKind::GuiMain(GuiMain::from_properties(&mut properties)?),
        "Handles" => InstanceKind::Handles(Handles::from_properties(&mut properties)?),
        "Hat" => InstanceKind::Hat(Hat::from_properties(&mut properties)?),
        "HingeConstraint" => {
            InstanceKind::HingeConstraint(HingeConstraint::from_properties(&mut properties)?)
        }
        "Hint" => InstanceKind::Hint(Hint::from_properties(&mut properties)?),
        "Hole" => InstanceKind::Hole(Hole::from_properties(&mut properties)?),
        "HopperBin" => InstanceKind::HopperBin(HopperBin::from_properties(&mut properties)?),
        "Humanoid" => InstanceKind::Humanoid(Humanoid::from_properties(&mut properties)?),
        "HumanoidController" => {
            InstanceKind::HumanoidController(HumanoidController::from_properties(&mut properties)?)
        }
        "HumanoidDescription" => InstanceKind::HumanoidDescription(Box::new(
            HumanoidDescription::from_properties(&mut properties)?,
        )),
        "ImageButton" => InstanceKind::ImageButton(ImageButton::from_properties(&mut properties)?),
        "ImageHandleAdornment" => InstanceKind::ImageHandleAdornment(
            ImageHandleAdornment::from_properties(&mut properties)?,
        ),
        "ImageLabel" => InstanceKind::ImageLabel(ImageLabel::from_properties(&mut properties)?),
        "IntConstrainedValue" => InstanceKind::IntConstrainedValue(
            IntConstrainedValue::from_properties(&mut properties)?,
        ),
        "IntValue" => InstanceKind::IntValue(IntValue::from_properties(&mut properties)?),
        "Keyframe" => InstanceKind::Keyframe(Keyframe::from_properties(&mut properties)?),
        "KeyframeMarker" => {
            InstanceKind::KeyframeMarker(KeyframeMarker::from_properties(&mut properties)?)
        }
        "KeyframeSequence" => {
            InstanceKind::KeyframeSequence(KeyframeSequence::from_properties(&mut properties)?)
        }
        "LineForce" => InstanceKind::LineForce(LineForce::from_properties(&mut properties)?),
        "LineHandleAdornment" => InstanceKind::LineHandleAdornment(
            LineHandleAdornment::from_properties(&mut properties)?,
        ),
        "LocalizationTable" => {
            InstanceKind::LocalizationTable(LocalizationTable::from_properties(&mut properties)?)
        }
        "LocalScript" => InstanceKind::LocalScript(LocalScript::from_properties(&mut properties)?),
        "ManualGlue" => InstanceKind::ManualGlue(ManualGlue::from_properties(&mut properties)?),
        "ManualWeld" => InstanceKind::ManualWeld(ManualWeld::from_properties(&mut properties)?),
        "MeshPart" => InstanceKind::MeshPart(MeshPart::from_properties(&mut properties)?),
        "Message" => InstanceKind::Message(Message::from_properties(&mut properties)?),
        "Model" => InstanceKind::Model(Model::from_properties(&mut properties)?),
        "ModuleScript" => {
            InstanceKind::ModuleScript(ModuleScript::from_properties(&mut properties)?)
        }
        "Motor" => InstanceKind::Motor(Motor::from_properties(&mut properties)?),
        "Motor6D" => InstanceKind::Motor6D(Motor6D::from_properties(&mut properties)?),
        "MotorFeature" => {
            InstanceKind::MotorFeature(MotorFeature::from_properties(&mut properties)?)
        }
        "NegateOperation" => {
            InstanceKind::NegateOperation(NegateOperation::from_properties(&mut properties)?)
        }
        "NoCollisionConstraint" => InstanceKind::NoCollisionConstraint(
            NoCollisionConstraint::from_properties(&mut properties)?,
        ),
        "NumberPose" => InstanceKind::NumberPose(NumberPose::from_properties(&mut properties)?),
        "NumberValue" => InstanceKind::NumberValue(NumberValue::from_properties(&mut properties)?),
        "ObjectValue" => InstanceKind::ObjectValue(ObjectValue::from_properties(&mut properties)?),
        "Pants" => InstanceKind::Pants(Pants::from_properties(&mut properties)?),
        "Part" => InstanceKind::Part(Part::from_properties(&mut properties)?),
        "ParticleEmitter" => {
            InstanceKind::ParticleEmitter(ParticleEmitter::from_properties(&mut properties)?)
        }
        "PartOperation" => {
            InstanceKind::PartOperation(PartOperation::from_properties(&mut properties)?)
        }
        "PartOperationAsset" => {
            InstanceKind::PartOperationAsset(PartOperationAsset::from_properties(&mut properties)?)
        }
        "PitchShiftSoundEffect" => InstanceKind::PitchShiftSoundEffect(
            PitchShiftSoundEffect::from_properties(&mut properties)?,
        ),
        "PointLight" => InstanceKind::PointLight(PointLight::from_properties(&mut properties)?),
        "Pose" => InstanceKind::Pose(Pose::from_properties(&mut properties)?),
        "PrismaticConstraint" => InstanceKind::PrismaticConstraint(
            PrismaticConstraint::from_properties(&mut properties)?,
        ),
        "ProximityPrompt" => {
            InstanceKind::ProximityPrompt(ProximityPrompt::from_properties(&mut properties)?)
        }
        "RayValue" => InstanceKind::RayValue(RayValue::from_properties(&mut properties)?),
        "ReflectionMetadata" => {
            InstanceKind::ReflectionMetadata(ReflectionMetadata::from_properties(&mut properties)?)
        }
        "ReflectionMetadataCallbacks" => InstanceKind::ReflectionMetadataCallbacks(
            ReflectionMetadataCallbacks::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataClass" => InstanceKind::ReflectionMetadataClass(
            ReflectionMetadataClass::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataClasses" => InstanceKind::ReflectionMetadataClasses(
            ReflectionMetadataClasses::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEnum" => InstanceKind::ReflectionMetadataEnum(
            ReflectionMetadataEnum::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEnumItem" => InstanceKind::ReflectionMetadataEnumItem(
            ReflectionMetadataEnumItem::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEnums" => InstanceKind::ReflectionMetadataEnums(
            ReflectionMetadataEnums::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataEvents" => InstanceKind::ReflectionMetadataEvents(
            ReflectionMetadataEvents::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataFunctions" => InstanceKind::ReflectionMetadataFunctions(
            ReflectionMetadataFunctions::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataMember" => InstanceKind::ReflectionMetadataMember(
            ReflectionMetadataMember::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataProperties" => InstanceKind::ReflectionMetadataProperties(
            ReflectionMetadataProperties::from_properties(&mut properties)?,
        ),
        "ReflectionMetadataYieldFunctions" => InstanceKind::ReflectionMetadataYieldFunctions(
            ReflectionMetadataYieldFunctions::from_properties(&mut properties)?,
        ),
        "RemoteEvent" => InstanceKind::RemoteEvent(RemoteEvent::from_properties(&mut properties)?),
        "RemoteFunction" => {
            InstanceKind::RemoteFunction(RemoteFunction::from_properties(&mut properties)?)
        }
        "RenderingTest" => {
            InstanceKind::RenderingTest(RenderingTest::from_properties(&mut properties)?)
        }
        "ReverbSoundEffect" => {
            InstanceKind::ReverbSoundEffect(ReverbSoundEffect::from_properties(&mut properties)?)
        }
        "RocketPropulsion" => {
            InstanceKind::RocketPropulsion(RocketPropulsion::from_properties(&mut properties)?)
        }
        "RodConstraint" => {
            InstanceKind::RodConstraint(RodConstraint::from_properties(&mut properties)?)
        }
        "RopeConstraint" => {
            InstanceKind::RopeConstraint(RopeConstraint::from_properties(&mut properties)?)
        }
        "Rotate" => InstanceKind::Rotate(Rotate::from_properties(&mut properties)?),
        "RotateP" => InstanceKind::RotateP(RotateP::from_properties(&mut properties)?),
        "RotateV" => InstanceKind::RotateV(RotateV::from_properties(&mut properties)?),
        "ScreenGui" => InstanceKind::ScreenGui(ScreenGui::from_properties(&mut properties)?),
        "Script" => InstanceKind::Script(Script::from_properties(&mut properties)?),
        "ScrollingFrame" => {
            InstanceKind::ScrollingFrame(ScrollingFrame::from_properties(&mut properties)?)
        }
        "Seat" => InstanceKind::Seat(Seat::from_properties(&mut properties)?),
        "SelectionBox" => {
            InstanceKind::SelectionBox(SelectionBox::from_properties(&mut properties)?)
        }
        "SelectionPartLasso" => {
            InstanceKind::SelectionPartLasso(SelectionPartLasso::from_properties(&mut properties)?)
        }
        "SelectionPointLasso" => InstanceKind::SelectionPointLasso(
            SelectionPointLasso::from_properties(&mut properties)?,
        ),
        "SelectionSphere" => {
            InstanceKind::SelectionSphere(SelectionSphere::from_properties(&mut properties)?)
        }
        "Shirt" => InstanceKind::Shirt(Shirt::from_properties(&mut properties)?),
        "ShirtGraphic" => {
            InstanceKind::ShirtGraphic(ShirtGraphic::from_properties(&mut properties)?)
        }
        "SkateboardController" => InstanceKind::SkateboardController(
            SkateboardController::from_properties(&mut properties)?,
        ),
        "SkateboardPlatform" => {
            InstanceKind::SkateboardPlatform(SkateboardPlatform::from_properties(&mut properties)?)
        }
        "Skin" => InstanceKind::Skin(Skin::from_properties(&mut properties)?),
        "Sky" => InstanceKind::Sky(Sky::from_properties(&mut properties)?),
        "Smoke" => InstanceKind::Smoke(Smoke::from_properties(&mut properties)?),
        "Snap" => InstanceKind::Snap(Snap::from_properties(&mut properties)?),
        "Sound" => InstanceKind::Sound(Sound::from_properties(&mut properties)?),
        "SoundGroup" => InstanceKind::SoundGroup(SoundGroup::from_properties(&mut properties)?),
        "Sparkles" => InstanceKind::Sparkles(Sparkles::from_properties(&mut properties)?),
        "SpawnLocation" => {
            InstanceKind::SpawnLocation(SpawnLocation::from_properties(&mut properties)?)
        }
        "SpecialMesh" => InstanceKind::SpecialMesh(SpecialMesh::from_properties(&mut properties)?),
        "SphereHandleAdornment" => InstanceKind::SphereHandleAdornment(
            SphereHandleAdornment::from_properties(&mut properties)?,
        ),
        "SpotLight" => InstanceKind::SpotLight(SpotLight::from_properties(&mut properties)?),
        "SpringConstraint" => {
            InstanceKind::SpringConstraint(SpringConstraint::from_properties(&mut properties)?)
        }
        "StandalonePluginScripts" => InstanceKind::StandalonePluginScripts(
            StandalonePluginScripts::from_properties(&mut properties)?,
        ),
        "StarterGear" => InstanceKind::StarterGear(StarterGear::from_properties(&mut properties)?),
        "StringValue" => InstanceKind::StringValue(StringValue::from_properties(&mut properties)?),
        "SunRaysEffect" => {
            InstanceKind::SunRaysEffect(SunRaysEffect::from_properties(&mut properties)?)
        }
        "SurfaceAppearance" => {
            InstanceKind::SurfaceAppearance(SurfaceAppearance::from_properties(&mut properties)?)
        }
        "SurfaceGui" => InstanceKind::SurfaceGui(SurfaceGui::from_properties(&mut properties)?),
        "SurfaceLight" => {
            InstanceKind::SurfaceLight(SurfaceLight::from_properties(&mut properties)?)
        }
        "SurfaceSelection" => {
            InstanceKind::SurfaceSelection(SurfaceSelection::from_properties(&mut properties)?)
        }
        "Team" => InstanceKind::Team(Team::from_properties(&mut properties)?),
        "TeleportOptions" => {
            InstanceKind::TeleportOptions(TeleportOptions::from_properties(&mut properties)?)
        }
        "Terrain" => InstanceKind::Terrain(Terrain::from_properties(&mut properties)?),
        "TerrainRegion" => {
            InstanceKind::TerrainRegion(TerrainRegion::from_properties(&mut properties)?)
        }
        "TextBox" => InstanceKind::TextBox(TextBox::from_properties(&mut properties)?),
        "TextButton" => InstanceKind::TextButton(TextButton::from_properties(&mut properties)?),
        "TextLabel" => InstanceKind::TextLabel(TextLabel::from_properties(&mut properties)?),
        "Texture" => InstanceKind::Texture(Texture::from_properties(&mut properties)?),
        "Tool" => InstanceKind::Tool(Tool::from_properties(&mut properties)?),
        "Torque" => InstanceKind::Torque(Torque::from_properties(&mut properties)?),
        "Trail" => InstanceKind::Trail(Trail::from_properties(&mut properties)?),
        "TremoloSoundEffect" => {
            InstanceKind::TremoloSoundEffect(TremoloSoundEffect::from_properties(&mut properties)?)
        }
        "TrussPart" => InstanceKind::TrussPart(TrussPart::from_properties(&mut properties)?),
        "Tween" => InstanceKind::Tween(Tween::from_properties(&mut properties)?),
        "UIAspectRatioConstraint" => InstanceKind::UIAspectRatioConstraint(
            UIAspectRatioConstraint::from_properties(&mut properties)?,
        ),
        "UICorner" => InstanceKind::UICorner(UICorner::from_properties(&mut properties)?),
        "UIGradient" => InstanceKind::UIGradient(UIGradient::from_properties(&mut properties)?),
        "UIGridLayout" => {
            InstanceKind::UIGridLayout(UIGridLayout::from_properties(&mut properties)?)
        }
        "UIListLayout" => {
            InstanceKind::UIListLayout(UIListLayout::from_properties(&mut properties)?)
        }
        "UIPadding" => InstanceKind::UIPadding(UIPadding::from_properties(&mut properties)?),
        "UIPageLayout" => {
            InstanceKind::UIPageLayout(UIPageLayout::from_properties(&mut properties)?)
        }
        "UIScale" => InstanceKind::UIScale(UIScale::from_properties(&mut properties)?),
        "UISizeConstraint" => {
            InstanceKind::UISizeConstraint(UISizeConstraint::from_properties(&mut properties)?)
        }
        "UIStroke" => InstanceKind::UIStroke(UIStroke::from_properties(&mut properties)?),
        "UITableLayout" => {
            InstanceKind::UITableLayout(UITableLayout::from_properties(&mut properties)?)
        }
        "UITextSizeConstraint" => InstanceKind::UITextSizeConstraint(
            UITextSizeConstraint::from_properties(&mut properties)?,
        ),
        "UnionOperation" => {
            InstanceKind::UnionOperation(UnionOperation::from_properties(&mut properties)?)
        }
        "UniversalConstraint" => InstanceKind::UniversalConstraint(
            UniversalConstraint::from_properties(&mut properties)?,
        ),
        "Vector3Value" => {
            InstanceKind::Vector3Value(Vector3Value::from_properties(&mut properties)?)
        }
        "VectorForce" => InstanceKind::VectorForce(VectorForce::from_properties(&mut properties)?),
        "VehicleController" => {
            InstanceKind::VehicleController(VehicleController::from_properties(&mut properties)?)
        }
        "VehicleSeat" => InstanceKind::VehicleSeat(VehicleSeat::from_properties(&mut properties)?),
        "VelocityMotor" => {
            InstanceKind::VelocityMotor(VelocityMotor::from_properties(&mut properties)?)
        }
        "VideoFrame" => InstanceKind::VideoFrame(VideoFrame::from_properties(&mut properties)?),
        "ViewportFrame" => {
            InstanceKind::ViewportFrame(ViewportFrame::from_properties(&mut properties)?)
        }
        "WedgePart" => InstanceKind::WedgePart(WedgePart::from_properties(&mut properties)?),
        "Weld" => InstanceKind::Weld(Weld::from_properties(&mut properties)?),
        "WeldConstraint" => {
            InstanceKind::WeldConstraint(WeldConstraint::from_properties(&mut properties)?)
        }
        "WorldModel" => InstanceKind::WorldModel(WorldModel::from_properties(&mut properties)?),
        _ => InstanceKind::Other(kind.to_owned(), properties.drain().collect()),
    };

    if !properties.is_empty() {
        Err(Error::UnconsumedProperties(
            out.class_name(),
            properties.into_iter().map(|(keys, _)| keys).collect(),
        ))
    } else {
        Ok(out)
    }
}

pub fn break_kind(kind: &InstanceKind) -> HashMap<String, Property> {
    let mut properties = HashMap::new();

    match kind {
        InstanceKind::Accoutrement(data) => data.to_properties(&mut properties),
        InstanceKind::Accessory(data) => data.to_properties(&mut properties),
        InstanceKind::Actor(data) => data.to_properties(&mut properties),
        InstanceKind::AlignOrientation(data) => data.to_properties(&mut properties),
        InstanceKind::AlignPosition(data) => data.to_properties(&mut properties),
        InstanceKind::AngularVelocity(data) => data.to_properties(&mut properties),
        InstanceKind::Animation(data) => data.to_properties(&mut properties),
        InstanceKind::AnimationController(data) => data.to_properties(&mut properties),
        InstanceKind::ArcHandles(data) => data.to_properties(&mut properties),
        InstanceKind::Atmosphere(data) => data.to_properties(&mut properties),
        InstanceKind::Backpack(data) => data.to_properties(&mut properties),
        InstanceKind::BallSocketConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::Beam(data) => data.to_properties(&mut properties),
        InstanceKind::BillboardGui(data) => data.to_properties(&mut properties),
        InstanceKind::BinaryStringValue(data) => data.to_properties(&mut properties),
        InstanceKind::BindableEvent(data) => data.to_properties(&mut properties),
        InstanceKind::BindableFunction(data) => data.to_properties(&mut properties),
        InstanceKind::BlockMesh(data) => data.to_properties(&mut properties),
        InstanceKind::BloomEffect(data) => data.to_properties(&mut properties),
        InstanceKind::BlurEffect(data) => data.to_properties(&mut properties),
        InstanceKind::BodyAngularVelocity(data) => data.to_properties(&mut properties),
        InstanceKind::BodyColors(data) => data.to_properties(&mut properties),
        InstanceKind::BodyForce(data) => data.to_properties(&mut properties),
        InstanceKind::BodyGyro(data) => data.to_properties(&mut properties),
        InstanceKind::BodyPosition(data) => data.to_properties(&mut properties),
        InstanceKind::BodyThrust(data) => data.to_properties(&mut properties),
        InstanceKind::BodyVelocity(data) => data.to_properties(&mut properties),
        InstanceKind::BoolValue(data) => data.to_properties(&mut properties),
        InstanceKind::BoxHandleAdornment(data) => data.to_properties(&mut properties),
        InstanceKind::BrickColorValue(data) => data.to_properties(&mut properties),
        InstanceKind::Camera(data) => data.to_properties(&mut properties),
        InstanceKind::CFrameValue(data) => data.to_properties(&mut properties),
        InstanceKind::CharacterMesh(data) => data.to_properties(&mut properties),
        InstanceKind::ChorusSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::ClickDetector(data) => data.to_properties(&mut properties),
        InstanceKind::Clouds(data) => data.to_properties(&mut properties),
        InstanceKind::Color3Value(data) => data.to_properties(&mut properties),
        InstanceKind::ColorCorrectionEffect(data) => data.to_properties(&mut properties),
        InstanceKind::CompressorSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::ConeHandleAdornment(data) => data.to_properties(&mut properties),
        InstanceKind::Configuration(data) => data.to_properties(&mut properties),
        InstanceKind::CornerWedgePart(data) => data.to_properties(&mut properties),
        InstanceKind::CustomEvent(data) => data.to_properties(&mut properties),
        InstanceKind::CustomEventReceiver(data) => data.to_properties(&mut properties),
        InstanceKind::CylinderHandleAdornment(data) => data.to_properties(&mut properties),
        InstanceKind::CylinderMesh(data) => data.to_properties(&mut properties),
        InstanceKind::CylindricalConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::Decal(data) => data.to_properties(&mut properties),
        InstanceKind::DepthOfFieldEffect(data) => data.to_properties(&mut properties),
        InstanceKind::Dialog(data) => data.to_properties(&mut properties),
        InstanceKind::DialogChoice(data) => data.to_properties(&mut properties),
        InstanceKind::DistortionSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::DoubleConstrainedValue(data) => data.to_properties(&mut properties),
        InstanceKind::EchoSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::EqualizerSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::Explosion(data) => data.to_properties(&mut properties),
        InstanceKind::FileMesh(data) => data.to_properties(&mut properties),
        InstanceKind::Fire(data) => data.to_properties(&mut properties),
        InstanceKind::Flag(data) => data.to_properties(&mut properties),
        InstanceKind::FlagStand(data) => data.to_properties(&mut properties),
        InstanceKind::FlangeSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::FloorWire(data) => data.to_properties(&mut properties),
        InstanceKind::Folder(data) => data.to_properties(&mut properties),
        InstanceKind::ForceField(data) => data.to_properties(&mut properties),
        InstanceKind::Frame(data) => data.to_properties(&mut properties),
        InstanceKind::FunctionalTest(data) => data.to_properties(&mut properties),
        InstanceKind::Glue(data) => data.to_properties(&mut properties),
        InstanceKind::GuiMain(data) => data.to_properties(&mut properties),
        InstanceKind::Handles(data) => data.to_properties(&mut properties),
        InstanceKind::Hat(data) => data.to_properties(&mut properties),
        InstanceKind::HingeConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::Hint(data) => data.to_properties(&mut properties),
        InstanceKind::Hole(data) => data.to_properties(&mut properties),
        InstanceKind::HopperBin(data) => data.to_properties(&mut properties),
        InstanceKind::Humanoid(data) => data.to_properties(&mut properties),
        InstanceKind::HumanoidController(data) => data.to_properties(&mut properties),
        InstanceKind::HumanoidDescription(data) => data.to_properties(&mut properties),
        InstanceKind::ImageButton(data) => data.to_properties(&mut properties),
        InstanceKind::ImageHandleAdornment(data) => data.to_properties(&mut properties),
        InstanceKind::ImageLabel(data) => data.to_properties(&mut properties),
        InstanceKind::IntConstrainedValue(data) => data.to_properties(&mut properties),
        InstanceKind::IntValue(data) => data.to_properties(&mut properties),
        InstanceKind::Keyframe(data) => data.to_properties(&mut properties),
        InstanceKind::KeyframeMarker(data) => data.to_properties(&mut properties),
        InstanceKind::KeyframeSequence(data) => data.to_properties(&mut properties),
        InstanceKind::LineForce(data) => data.to_properties(&mut properties),
        InstanceKind::LineHandleAdornment(data) => data.to_properties(&mut properties),
        InstanceKind::LocalizationTable(data) => data.to_properties(&mut properties),
        InstanceKind::LocalScript(data) => data.to_properties(&mut properties),
        InstanceKind::ManualGlue(data) => data.to_properties(&mut properties),
        InstanceKind::ManualWeld(data) => data.to_properties(&mut properties),
        InstanceKind::MeshPart(data) => data.to_properties(&mut properties),
        InstanceKind::Message(data) => data.to_properties(&mut properties),
        InstanceKind::Model(data) => data.to_properties(&mut properties),
        InstanceKind::ModuleScript(data) => data.to_properties(&mut properties),
        InstanceKind::Motor(data) => data.to_properties(&mut properties),
        InstanceKind::Motor6D(data) => data.to_properties(&mut properties),
        InstanceKind::MotorFeature(data) => data.to_properties(&mut properties),
        InstanceKind::NegateOperation(data) => data.to_properties(&mut properties),
        InstanceKind::NoCollisionConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::NumberPose(data) => data.to_properties(&mut properties),
        InstanceKind::NumberValue(data) => data.to_properties(&mut properties),
        InstanceKind::ObjectValue(data) => data.to_properties(&mut properties),
        InstanceKind::Pants(data) => data.to_properties(&mut properties),
        InstanceKind::Part(data) => data.to_properties(&mut properties),
        InstanceKind::ParticleEmitter(data) => data.to_properties(&mut properties),
        InstanceKind::PartOperation(data) => data.to_properties(&mut properties),
        InstanceKind::PartOperationAsset(data) => data.to_properties(&mut properties),
        InstanceKind::PitchShiftSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::PointLight(data) => data.to_properties(&mut properties),
        InstanceKind::Pose(data) => data.to_properties(&mut properties),
        InstanceKind::PrismaticConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::ProximityPrompt(data) => data.to_properties(&mut properties),
        InstanceKind::RayValue(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadata(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataCallbacks(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataClass(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataClasses(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataEnum(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataEnumItem(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataEnums(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataEvents(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataFunctions(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataMember(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataProperties(data) => data.to_properties(&mut properties),
        InstanceKind::ReflectionMetadataYieldFunctions(data) => data.to_properties(&mut properties),
        InstanceKind::RemoteEvent(data) => data.to_properties(&mut properties),
        InstanceKind::RemoteFunction(data) => data.to_properties(&mut properties),
        InstanceKind::RenderingTest(data) => data.to_properties(&mut properties),
        InstanceKind::ReverbSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::RocketPropulsion(data) => data.to_properties(&mut properties),
        InstanceKind::RodConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::RopeConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::Rotate(data) => data.to_properties(&mut properties),
        InstanceKind::RotateP(data) => data.to_properties(&mut properties),
        InstanceKind::RotateV(data) => data.to_properties(&mut properties),
        InstanceKind::ScreenGui(data) => data.to_properties(&mut properties),
        InstanceKind::Script(data) => data.to_properties(&mut properties),
        InstanceKind::ScrollingFrame(data) => data.to_properties(&mut properties),
        InstanceKind::Seat(data) => data.to_properties(&mut properties),
        InstanceKind::SelectionBox(data) => data.to_properties(&mut properties),
        InstanceKind::SelectionPartLasso(data) => data.to_properties(&mut properties),
        InstanceKind::SelectionPointLasso(data) => data.to_properties(&mut properties),
        InstanceKind::SelectionSphere(data) => data.to_properties(&mut properties),
        InstanceKind::Shirt(data) => data.to_properties(&mut properties),
        InstanceKind::ShirtGraphic(data) => data.to_properties(&mut properties),
        InstanceKind::SkateboardController(data) => data.to_properties(&mut properties),
        InstanceKind::SkateboardPlatform(data) => data.to_properties(&mut properties),
        InstanceKind::Skin(data) => data.to_properties(&mut properties),
        InstanceKind::Sky(data) => data.to_properties(&mut properties),
        InstanceKind::Smoke(data) => data.to_properties(&mut properties),
        InstanceKind::Snap(data) => data.to_properties(&mut properties),
        InstanceKind::Sound(data) => data.to_properties(&mut properties),
        InstanceKind::SoundGroup(data) => data.to_properties(&mut properties),
        InstanceKind::Sparkles(data) => data.to_properties(&mut properties),
        InstanceKind::SpawnLocation(data) => data.to_properties(&mut properties),
        InstanceKind::SpecialMesh(data) => data.to_properties(&mut properties),
        InstanceKind::SphereHandleAdornment(data) => data.to_properties(&mut properties),
        InstanceKind::SpotLight(data) => data.to_properties(&mut properties),
        InstanceKind::SpringConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::StandalonePluginScripts(data) => data.to_properties(&mut properties),
        InstanceKind::StarterGear(data) => data.to_properties(&mut properties),
        InstanceKind::StringValue(data) => data.to_properties(&mut properties),
        InstanceKind::SunRaysEffect(data) => data.to_properties(&mut properties),
        InstanceKind::SurfaceAppearance(data) => data.to_properties(&mut properties),
        InstanceKind::SurfaceGui(data) => data.to_properties(&mut properties),
        InstanceKind::SurfaceLight(data) => data.to_properties(&mut properties),
        InstanceKind::SurfaceSelection(data) => data.to_properties(&mut properties),
        InstanceKind::Team(data) => data.to_properties(&mut properties),
        InstanceKind::TeleportOptions(data) => data.to_properties(&mut properties),
        InstanceKind::Terrain(data) => data.to_properties(&mut properties),
        InstanceKind::TerrainRegion(data) => data.to_properties(&mut properties),
        InstanceKind::TextBox(data) => data.to_properties(&mut properties),
        InstanceKind::TextButton(data) => data.to_properties(&mut properties),
        InstanceKind::TextLabel(data) => data.to_properties(&mut properties),
        InstanceKind::Texture(data) => data.to_properties(&mut properties),
        InstanceKind::Tool(data) => data.to_properties(&mut properties),
        InstanceKind::Torque(data) => data.to_properties(&mut properties),
        InstanceKind::Trail(data) => data.to_properties(&mut properties),
        InstanceKind::TremoloSoundEffect(data) => data.to_properties(&mut properties),
        InstanceKind::TrussPart(data) => data.to_properties(&mut properties),
        InstanceKind::Tween(data) => data.to_properties(&mut properties),
        InstanceKind::UIAspectRatioConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::UICorner(data) => data.to_properties(&mut properties),
        InstanceKind::UIGradient(data) => data.to_properties(&mut properties),
        InstanceKind::UIGridLayout(data) => data.to_properties(&mut properties),
        InstanceKind::UIListLayout(data) => data.to_properties(&mut properties),
        InstanceKind::UIPadding(data) => data.to_properties(&mut properties),
        InstanceKind::UIPageLayout(data) => data.to_properties(&mut properties),
        InstanceKind::UIScale(data) => data.to_properties(&mut properties),
        InstanceKind::UISizeConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::UIStroke(data) => data.to_properties(&mut properties),
        InstanceKind::UITableLayout(data) => data.to_properties(&mut properties),
        InstanceKind::UITextSizeConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::UnionOperation(data) => data.to_properties(&mut properties),
        InstanceKind::UniversalConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::Vector3Value(data) => data.to_properties(&mut properties),
        InstanceKind::VectorForce(data) => data.to_properties(&mut properties),
        InstanceKind::VehicleController(data) => data.to_properties(&mut properties),
        InstanceKind::VehicleSeat(data) => data.to_properties(&mut properties),
        InstanceKind::VelocityMotor(data) => data.to_properties(&mut properties),
        InstanceKind::VideoFrame(data) => data.to_properties(&mut properties),
        InstanceKind::ViewportFrame(data) => data.to_properties(&mut properties),
        InstanceKind::WedgePart(data) => data.to_properties(&mut properties),
        InstanceKind::Weld(data) => data.to_properties(&mut properties),
        InstanceKind::WeldConstraint(data) => data.to_properties(&mut properties),
        InstanceKind::WorldModel(data) => data.to_properties(&mut properties),
        InstanceKind::Other(_, data) => properties.extend(data.clone()),
    }

    properties
}
