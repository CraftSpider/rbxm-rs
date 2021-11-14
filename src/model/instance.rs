//! Crate for items related to Roblox Instances, the base components of all models.

#![allow(missing_docs)]

use super::InstanceRef;
use crate::model::data::*;
use crate::model::enums::*;
use crate::model::Property;
use crate::serde::internal::{FromProperty, ToProperty};
use rbxm_proc::{Inherits, PropertyConvert};

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Represent the kind of an [`Instance`]. This is not meant to be matched exhaustively, more often
/// only checking if an Instance is of a specific kind, otherwise performing some default activity
/// or erroring out.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Instance {
    Accessory(Accessory),
    Accoutrement(Accoutrement),
    Actor(Actor),
    AlignOrientation(AlignOrientation),
    AlignPosition(AlignPosition),
    AngularVelocity(AngularVelocity),
    Animation(Animation),
    AnimationController(AnimationController),
    ArcHandles(ArcHandles),
    Atmosphere(Atmosphere),
    Backpack(Backpack),
    BallSocketConstraint(BallSocketConstraint),
    Beam(Beam),
    BillboardGui(BillboardGui),
    BinaryStringValue(BinaryStringValue),
    BindableEvent(BindableEvent),
    BindableFunction(BindableFunction),
    BlockMesh(BlockMesh),
    BloomEffect(BloomEffect),
    BlurEffect(BlurEffect),
    BodyAngularVelocity(BodyAngularVelocity),
    BodyColors(BodyColors),
    BodyForce(BodyForce),
    BodyGyro(BodyGyro),
    BodyPosition(BodyPosition),
    BodyThrust(BodyThrust),
    BodyVelocity(BodyVelocity),
    BoolValue(BoolValue),
    BoxHandleAdornment(BoxHandleAdornment),
    BrickColorValue(BrickColorValue),
    Camera(Camera),
    CFrameValue(CFrameValue),
    CharacterMesh(CharacterMesh),
    ChorusSoundEffect(ChorusSoundEffect),
    ClickDetector(ClickDetector),
    Clouds(Clouds),
    Color3Value(Color3Value),
    ColorCorrectionEffect(ColorCorrectionEffect),
    CompressorSoundEffect(CompressorSoundEffect),
    ConeHandleAdornment(ConeHandleAdornment),
    Configuration(Configuration),
    CornerWedgePart(CornerWedgePart),
    CustomEvent(CustomEvent),
    CustomEventReceiver(CustomEventReceiver),
    CylinderHandleAdornment(CylinderHandleAdornment),
    CylinderMesh(CylinderMesh),
    CylindricalConstraint(CylindricalConstraint),
    Decal(Decal),
    DepthOfFieldEffect(DepthOfFieldEffect),
    Dialog(Dialog),
    DialogChoice(DialogChoice),
    DistortionSoundEffect(DistortionSoundEffect),
    DoubleConstrainedValue(DoubleConstrainedValue),
    EchoSoundEffect(EchoSoundEffect),
    EqualizerSoundEffect(EqualizerSoundEffect),
    Explosion(Explosion),
    FileMesh(FileMesh),
    Fire(Fire),
    Flag(Flag),
    FlagStand(FlagStand),
    FlangeSoundEffect(FlangeSoundEffect),
    FloorWire(FloorWire),
    Folder(Folder),
    ForceField(ForceField),
    Frame(Frame),
    FunctionalTest(FunctionalTest),
    Glue(Glue),
    GuiMain(GuiMain),
    Handles(Handles),
    Hat(Hat),
    HingeConstraint(HingeConstraint),
    Hint(Hint),
    Hole(Hole),
    HopperBin(HopperBin),
    Humanoid(Humanoid),
    HumanoidController(HumanoidController),
    HumanoidDescription(Box<HumanoidDescription>),
    ImageButton(ImageButton),
    ImageHandleAdornment(ImageHandleAdornment),
    ImageLabel(ImageLabel),
    IntConstrainedValue(IntConstrainedValue),
    IntValue(IntValue),
    Keyframe(Keyframe),
    KeyframeMarker(KeyframeMarker),
    KeyframeSequence(KeyframeSequence),
    LineForce(LineForce),
    LineHandleAdornment(LineHandleAdornment),
    LocalizationTable(LocalizationTable),
    LocalScript(LocalScript),
    ManualGlue(ManualGlue),
    ManualWeld(ManualWeld),
    MeshPart(MeshPart),
    Message(Message),
    Model(Model),
    ModuleScript(ModuleScript),
    Motor(Motor),
    Motor6D(Motor6D),
    MotorFeature(MotorFeature),
    NegateOperation(NegateOperation),
    NoCollisionConstraint(NoCollisionConstraint),
    NumberPose(NumberPose),
    NumberValue(NumberValue),
    ObjectValue(ObjectValue),
    Pants(Pants),
    Part(Part),
    ParticleEmitter(ParticleEmitter),
    PartOperation(PartOperation),
    PartOperationAsset(PartOperationAsset),
    PitchShiftSoundEffect(PitchShiftSoundEffect),
    PointLight(PointLight),
    Pose(Pose),
    PrismaticConstraint(PrismaticConstraint),
    ProximityPrompt(ProximityPrompt),
    RayValue(RayValue),
    ReflectionMetadata(ReflectionMetadata),
    ReflectionMetadataCallbacks(ReflectionMetadataCallbacks),
    ReflectionMetadataClass(ReflectionMetadataClass),
    ReflectionMetadataClasses(ReflectionMetadataClasses),
    ReflectionMetadataEnum(ReflectionMetadataEnum),
    ReflectionMetadataEnumItem(ReflectionMetadataEnumItem),
    ReflectionMetadataEnums(ReflectionMetadataEnums),
    ReflectionMetadataEvents(ReflectionMetadataEvents),
    ReflectionMetadataFunctions(ReflectionMetadataFunctions),
    ReflectionMetadataMember(ReflectionMetadataMember),
    ReflectionMetadataProperties(ReflectionMetadataProperties),
    ReflectionMetadataYieldFunctions(ReflectionMetadataYieldFunctions),
    RemoteEvent(RemoteEvent),
    RemoteFunction(RemoteFunction),
    RenderingTest(RenderingTest),
    ReverbSoundEffect(ReverbSoundEffect),
    RocketPropulsion(RocketPropulsion),
    RodConstraint(RodConstraint),
    RopeConstraint(RopeConstraint),
    Rotate(Rotate),
    RotateP(RotateP),
    RotateV(RotateV),
    ScreenGui(ScreenGui),
    Script(Script),
    ScrollingFrame(ScrollingFrame),
    Seat(Seat),
    SelectionBox(SelectionBox),
    SelectionPartLasso(SelectionPartLasso),
    SelectionPointLasso(SelectionPointLasso),
    SelectionSphere(SelectionSphere),
    Shirt(Shirt),
    ShirtGraphic(ShirtGraphic),
    SkateboardController(SkateboardController),
    SkateboardPlatform(SkateboardPlatform),
    Skin(Skin),
    Sky(Sky),
    Smoke(Smoke),
    Snap(Snap),
    Sound(Sound),
    SoundGroup(SoundGroup),
    Sparkles(Sparkles),
    SpawnLocation(SpawnLocation),
    SpecialMesh(SpecialMesh),
    SphereHandleAdornment(SphereHandleAdornment),
    SpotLight(SpotLight),
    SpringConstraint(SpringConstraint),
    StandalonePluginScripts(StandalonePluginScripts),
    StarterGear(StarterGear),
    StringValue(StringValue),
    SunRaysEffect(SunRaysEffect),
    SurfaceAppearance(SurfaceAppearance),
    SurfaceGui(SurfaceGui),
    SurfaceLight(SurfaceLight),
    SurfaceSelection(SurfaceSelection),
    Team(Team),
    TeleportOptions(TeleportOptions),
    Terrain(Terrain),
    TerrainRegion(TerrainRegion),
    TextBox(TextBox),
    TextButton(TextButton),
    TextLabel(TextLabel),
    Texture(Texture),
    Tool(Tool),
    Torque(Torque),
    Trail(Trail),
    TremoloSoundEffect(TremoloSoundEffect),
    TrussPart(TrussPart),
    Tween(Tween),
    UIAspectRatioConstraint(UIAspectRatioConstraint),
    UICorner(UICorner),
    UIGradient(UIGradient),
    UIGridLayout(UIGridLayout),
    UIListLayout(UIListLayout),
    UIPadding(UIPadding),
    UIPageLayout(UIPageLayout),
    UIScale(UIScale),
    UISizeConstraint(UISizeConstraint),
    UIStroke(UIStroke),
    UITableLayout(UITableLayout),
    UITextSizeConstraint(UITextSizeConstraint),
    UnionOperation(UnionOperation),
    UniversalConstraint(UniversalConstraint),
    Vector3Value(Vector3Value),
    VectorForce(VectorForce),
    VehicleController(VehicleController),
    VehicleSeat(VehicleSeat),
    VelocityMotor(VelocityMotor),
    VideoFrame(VideoFrame),
    ViewportFrame(ViewportFrame),
    WedgePart(WedgePart),
    Weld(Weld),
    WeldConstraint(WeldConstraint),
    WorldModel(WorldModel),

    Other(String, BTreeMap<String, Property>),
}

#[warn(missing_docs)]
impl Instance {
    /// Get the name of the class for this kind
    pub fn class_name(&self) -> String {
        String::from(match self {
            Instance::Accessory(..) => "Accessory",
            Instance::Accoutrement(..) => "Accoutrement",
            Instance::Actor(..) => "Actor",
            Instance::AlignOrientation(..) => "AlignOrientation",
            Instance::AlignPosition(..) => "AlignPosition",
            Instance::AngularVelocity(..) => "AngularVelocity",
            Instance::Animation(..) => "Animation",
            Instance::AnimationController(..) => "AnimationController",
            Instance::ArcHandles(..) => "ArcHandles",
            Instance::Atmosphere(..) => "Atmosphere",
            Instance::Backpack(..) => "Backpack",
            Instance::BallSocketConstraint(..) => "BallSocketConstraint",
            Instance::Beam(..) => "Beam",
            Instance::BillboardGui(..) => "BillboardGui",
            Instance::BinaryStringValue(..) => "BinaryStringValue",
            Instance::BindableEvent(..) => "BindableEvent",
            Instance::BindableFunction(..) => "BindableFunction",
            Instance::BlockMesh(..) => "BlockMesh",
            Instance::BloomEffect(..) => "BloomEffect",
            Instance::BlurEffect(..) => "BlurEffect",
            Instance::BodyAngularVelocity(..) => "BodyAngularVelocity",
            Instance::BodyColors(..) => "BodyColors",
            Instance::BodyForce(..) => "BodyForce",
            Instance::BodyGyro(..) => "BodyGyro",
            Instance::BodyPosition(..) => "BodyPosition",
            Instance::BodyThrust(..) => "BodyThrust",
            Instance::BodyVelocity(..) => "BodyVelocity",
            Instance::BoolValue(..) => "BoolValue",
            Instance::BoxHandleAdornment(..) => "BoxHandleAdornment",
            Instance::BrickColorValue(..) => "BrickColorValue",
            Instance::Camera(..) => "Camera",
            Instance::CFrameValue(..) => "CFrameValue",
            Instance::CharacterMesh(..) => "CharacterMesh",
            Instance::ChorusSoundEffect(..) => "ChorusSoundEffect",
            Instance::ClickDetector(..) => "ClickDetector",
            Instance::Clouds(..) => "Clouds",
            Instance::Color3Value(..) => "Color3Value",
            Instance::ColorCorrectionEffect(..) => "ColorCorrectionEffect",
            Instance::CompressorSoundEffect(..) => "CompressorSoundEffect",
            Instance::ConeHandleAdornment(..) => "ConeHandleAdornment",
            Instance::Configuration(..) => "Configuration",
            Instance::CornerWedgePart(..) => "CornerWedgePart",
            Instance::CustomEvent(..) => "CustomEvent",
            Instance::CustomEventReceiver(..) => "CustomEventReceiver",
            Instance::CylinderHandleAdornment(..) => "CylinderHandleAdornment",
            Instance::CylinderMesh(..) => "CylinderMesh",
            Instance::CylindricalConstraint(..) => "CylindricalConstraint",
            Instance::Decal(..) => "Decal",
            Instance::DepthOfFieldEffect(..) => "DepthOfFieldEffect",
            Instance::Dialog(..) => "Dialog",
            Instance::DialogChoice(..) => "DialogChoice",
            Instance::DistortionSoundEffect(..) => "DistortionSoundEffect",
            Instance::DoubleConstrainedValue(..) => "DoubleConstrainedValue",
            Instance::EchoSoundEffect(..) => "EchoSoundEffect",
            Instance::EqualizerSoundEffect(..) => "EqualizerSoundEffect",
            Instance::Explosion(..) => "Explosion",
            Instance::FileMesh(..) => "FileMesh",
            Instance::Fire(..) => "Fire",
            Instance::Flag(..) => "Flag",
            Instance::FlagStand(..) => "FlagStand",
            Instance::FlangeSoundEffect(..) => "FlangeSoundEffect",
            Instance::FloorWire(..) => "FloorWire",
            Instance::Folder(..) => "Folder",
            Instance::ForceField(..) => "ForceField",
            Instance::Frame(..) => "Frame",
            Instance::FunctionalTest(..) => "FunctionalTest",
            Instance::Glue(..) => "Glue",
            Instance::GuiMain(..) => "GuiMain",
            Instance::Handles(..) => "Handles",
            Instance::Hat(..) => "Hat",
            Instance::HingeConstraint(..) => "HingeConstraint",
            Instance::Hint(..) => "Hint",
            Instance::Hole(..) => "Hole",
            Instance::HopperBin(..) => "HopperBin",
            Instance::Humanoid(..) => "Humanoid",
            Instance::HumanoidController(..) => "HumanoidController",
            Instance::HumanoidDescription(..) => "HumanoidDescription",
            Instance::ImageButton(..) => "ImageButton",
            Instance::ImageHandleAdornment(..) => "ImageHandleAdornment",
            Instance::ImageLabel(..) => "ImageLabel",
            Instance::IntConstrainedValue(..) => "IntConstrainedValue",
            Instance::IntValue(..) => "IntValue",
            Instance::Keyframe(..) => "Keyframe",
            Instance::KeyframeMarker(..) => "KeyframeMarker",
            Instance::KeyframeSequence(..) => "KeyframeSequence",
            Instance::LineForce(..) => "LineForce",
            Instance::LineHandleAdornment(..) => "LineHandleAdornment",
            Instance::LocalizationTable(..) => "LocalizationTable",
            Instance::LocalScript(..) => "LocalScript",
            Instance::ManualGlue(..) => "ManualGlue",
            Instance::ManualWeld(..) => "ManualWeld",
            Instance::MeshPart(..) => "MeshPart",
            Instance::Message(..) => "Message",
            Instance::Model(..) => "Model",
            Instance::ModuleScript(..) => "ModuleScript",
            Instance::Motor(..) => "Motor",
            Instance::Motor6D(..) => "Motor6D",
            Instance::MotorFeature(..) => "MotorFeature",
            Instance::NegateOperation(..) => "NegateOperation",
            Instance::NoCollisionConstraint(..) => "NoCollisionConstraint",
            Instance::NumberPose(..) => "NumberPose",
            Instance::NumberValue(..) => "NumberValue",
            Instance::ObjectValue(..) => "ObjectValue",
            Instance::Pants(..) => "Pants",
            Instance::Part(..) => "Part",
            Instance::ParticleEmitter(..) => "ParticleEmitter",
            Instance::PartOperation(..) => "PartOperation",
            Instance::PartOperationAsset(..) => "PartOperationAsset",
            Instance::PitchShiftSoundEffect(..) => "PitchShiftSoundEffect",
            Instance::PointLight(..) => "PointLight",
            Instance::Pose(..) => "Pose",
            Instance::PrismaticConstraint(..) => "PrismaticConstraint",
            Instance::ProximityPrompt(..) => "ProximityPrompt",
            Instance::RayValue(..) => "RayValue",
            Instance::ReflectionMetadata(..) => "ReflectionMetadata",
            Instance::ReflectionMetadataCallbacks(..) => "ReflectionMetadataCallbacks",
            Instance::ReflectionMetadataClass(..) => "ReflectionMetadataClass",
            Instance::ReflectionMetadataClasses(..) => "ReflectionMetadataClasses",
            Instance::ReflectionMetadataEnum(..) => "ReflectionMetadataEnum",
            Instance::ReflectionMetadataEnumItem(..) => "ReflectionMetadataEnumItem",
            Instance::ReflectionMetadataEnums(..) => "ReflectionMetadataEnums",
            Instance::ReflectionMetadataEvents(..) => "ReflectionMetadataEvents",
            Instance::ReflectionMetadataFunctions(..) => "ReflectionMetadataFunctions",
            Instance::ReflectionMetadataMember(..) => "ReflectionMetadataMember",
            Instance::ReflectionMetadataProperties(..) => "ReflectionMetadataProperties",
            Instance::ReflectionMetadataYieldFunctions(..) => "ReflectionMetadataYieldFunctions",
            Instance::RemoteEvent(..) => "RemoteEvent",
            Instance::RemoteFunction(..) => "RemoteFunction",
            Instance::RenderingTest(..) => "RenderingTest",
            Instance::ReverbSoundEffect(..) => "ReverbSoundEffect",
            Instance::RocketPropulsion(..) => "RocketPropulsion",
            Instance::RodConstraint(..) => "RodConstraint",
            Instance::RopeConstraint(..) => "RopeConstraint",
            Instance::Rotate(..) => "Rotate",
            Instance::RotateP(..) => "RotateP",
            Instance::RotateV(..) => "RotateV",
            Instance::ScreenGui(..) => "ScreenGui",
            Instance::Script(..) => "Script",
            Instance::ScrollingFrame(..) => "ScrollingFrame",
            Instance::Seat(..) => "Seat",
            Instance::SelectionBox(..) => "SelectionBox",
            Instance::SelectionPartLasso(..) => "SelectionPartLasso",
            Instance::SelectionPointLasso(..) => "SelectionPointLasso",
            Instance::SelectionSphere(..) => "SelectionSphere",
            Instance::Shirt(..) => "Shirt",
            Instance::ShirtGraphic(..) => "ShirtGraphic",
            Instance::SkateboardController(..) => "SkateboardController",
            Instance::SkateboardPlatform(..) => "SkateboardPlatform",
            Instance::Skin(..) => "Skin",
            Instance::Sky(..) => "Sky",
            Instance::Smoke(..) => "Smoke",
            Instance::Snap(..) => "Snap",
            Instance::Sound(..) => "Sound",
            Instance::SoundGroup(..) => "SoundGroup",
            Instance::Sparkles(..) => "Sparkles",
            Instance::SpawnLocation(..) => "SpawnLocation",
            Instance::SpecialMesh(..) => "SpecialMesh",
            Instance::SphereHandleAdornment(..) => "SphereHandleAdornment",
            Instance::SpotLight(..) => "SpotLight",
            Instance::SpringConstraint(..) => "SpringConstraint",
            Instance::StandalonePluginScripts(..) => "StandalonePluginScripts",
            Instance::StarterGear(..) => "StarterGear",
            Instance::StringValue(..) => "StringValue",
            Instance::SunRaysEffect(..) => "SunRaysEffect",
            Instance::SurfaceAppearance(..) => "SurfaceAppearance",
            Instance::SurfaceGui(..) => "SurfaceGui",
            Instance::SurfaceLight(..) => "SurfaceLight",
            Instance::SurfaceSelection(..) => "SurfaceSelection",
            Instance::Team(..) => "Team",
            Instance::TeleportOptions(..) => "TeleportOptions",
            Instance::Terrain(..) => "Terrain",
            Instance::TerrainRegion(..) => "TerrainRegion",
            Instance::TextBox(..) => "TextBox",
            Instance::TextButton(..) => "TextButton",
            Instance::TextLabel(..) => "TextLabel",
            Instance::Texture(..) => "Texture",
            Instance::Tool(..) => "Tool",
            Instance::Torque(..) => "Torque",
            Instance::Trail(..) => "Trail",
            Instance::TremoloSoundEffect(..) => "TremoloSoundEffect",
            Instance::TrussPart(..) => "TrussPart",
            Instance::Tween(..) => "Tween",
            Instance::UIAspectRatioConstraint(..) => "UIAspectRatioConstraint",
            Instance::UICorner(..) => "UICorner",
            Instance::UIGradient(..) => "UIGradient",
            Instance::UIGridLayout(..) => "UIGridLayout",
            Instance::UIListLayout(..) => "UIListLayout",
            Instance::UIPadding(..) => "UIPadding",
            Instance::UIPageLayout(..) => "UIPageLayout",
            Instance::UIScale(..) => "UIScale",
            Instance::UISizeConstraint(..) => "UISizeConstraint",
            Instance::UIStroke(..) => "UIStroke",
            Instance::UITableLayout(..) => "UITableLayout",
            Instance::UITextSizeConstraint(..) => "UITextSizeConstraint",
            Instance::UnionOperation(..) => "UnionOperation",
            Instance::UniversalConstraint(..) => "UniversalConstraint",
            Instance::Vector3Value(..) => "Vector3Value",
            Instance::VectorForce(..) => "VectorForce",
            Instance::VehicleController(..) => "VehicleController",
            Instance::VehicleSeat(..) => "VehicleSeat",
            Instance::VelocityMotor(..) => "VelocityMotor",
            Instance::VideoFrame(..) => "VideoFrame",
            Instance::ViewportFrame(..) => "ViewportFrame",
            Instance::WedgePart(..) => "WedgePart",
            Instance::Weld(..) => "Weld",
            Instance::WeldConstraint(..) => "WeldConstraint",
            Instance::WorldModel(..) => "WorldModel",
            Instance::Other(name, ..) => name,
        })
    }

    /// Get the name of this kind
    ///
    /// # Panics
    ///
    /// If the instance is of an unrecognized type which doesn't have a name.
    pub fn name(&self) -> &str {
        match self {
            Instance::Accessory(data) => &data.name,
            Instance::Accoutrement(data) => &data.name,
            Instance::Actor(data) => &data.name,
            Instance::AlignOrientation(data) => &data.name,
            Instance::AlignPosition(data) => &data.name,
            Instance::AngularVelocity(data) => &data.name,
            Instance::Animation(data) => &data.name,
            Instance::AnimationController(data) => &data.name,
            Instance::ArcHandles(data) => &data.name,
            Instance::Atmosphere(data) => &data.name,
            Instance::Backpack(data) => &data.name,
            Instance::BallSocketConstraint(data) => &data.name,
            Instance::Beam(data) => &data.name,
            Instance::BillboardGui(data) => &data.name,
            Instance::BinaryStringValue(data) => &data.name,
            Instance::BindableEvent(data) => &data.name,
            Instance::BindableFunction(data) => &data.name,
            Instance::BlockMesh(data) => &data.name,
            Instance::BloomEffect(data) => &data.name,
            Instance::BlurEffect(data) => &data.name,
            Instance::BodyAngularVelocity(data) => &data.name,
            Instance::BodyColors(data) => &data.name,
            Instance::BodyForce(data) => &data.name,
            Instance::BodyGyro(data) => &data.name,
            Instance::BodyPosition(data) => &data.name,
            Instance::BodyThrust(data) => &data.name,
            Instance::BodyVelocity(data) => &data.name,
            Instance::BoolValue(data) => &data.name,
            Instance::BoxHandleAdornment(data) => &data.name,
            Instance::BrickColorValue(data) => &data.name,
            Instance::Camera(data) => &data.name,
            Instance::CFrameValue(data) => &data.name,
            Instance::CharacterMesh(data) => &data.name,
            Instance::ChorusSoundEffect(data) => &data.name,
            Instance::ClickDetector(data) => &data.name,
            Instance::Clouds(data) => &data.name,
            Instance::Color3Value(data) => &data.name,
            Instance::ColorCorrectionEffect(data) => &data.name,
            Instance::CompressorSoundEffect(data) => &data.name,
            Instance::ConeHandleAdornment(data) => &data.name,
            Instance::Configuration(data) => &data.name,
            Instance::CornerWedgePart(data) => &data.name,
            Instance::CustomEvent(data) => &data.name,
            Instance::CustomEventReceiver(data) => &data.name,
            Instance::CylinderHandleAdornment(data) => &data.name,
            Instance::CylinderMesh(data) => &data.name,
            Instance::CylindricalConstraint(data) => &data.name,
            Instance::Decal(data) => &data.name,
            Instance::DepthOfFieldEffect(data) => &data.name,
            Instance::Dialog(data) => &data.name,
            Instance::DialogChoice(data) => &data.name,
            Instance::DistortionSoundEffect(data) => &data.name,
            Instance::DoubleConstrainedValue(data) => &data.name,
            Instance::EchoSoundEffect(data) => &data.name,
            Instance::EqualizerSoundEffect(data) => &data.name,
            Instance::Explosion(data) => &data.name,
            Instance::FileMesh(data) => &data.name,
            Instance::Fire(data) => &data.name,
            Instance::Flag(data) => &data.name,
            Instance::FlagStand(data) => &data.name,
            Instance::FlangeSoundEffect(data) => &data.name,
            Instance::FloorWire(data) => &data.name,
            Instance::Folder(data) => &data.name,
            Instance::ForceField(data) => &data.name,
            Instance::Frame(data) => &data.name,
            Instance::FunctionalTest(data) => &data.name,
            Instance::Glue(data) => &data.name,
            Instance::GuiMain(data) => &data.name,
            Instance::Handles(data) => &data.name,
            Instance::Hat(data) => &data.name,
            Instance::HingeConstraint(data) => &data.name,
            Instance::Hint(data) => &data.name,
            Instance::Hole(data) => &data.name,
            Instance::HopperBin(data) => &data.name,
            Instance::Humanoid(data) => &data.name,
            Instance::HumanoidController(data) => &data.name,
            Instance::HumanoidDescription(data) => &data.name,
            Instance::ImageButton(data) => &data.name,
            Instance::ImageHandleAdornment(data) => &data.name,
            Instance::ImageLabel(data) => &data.name,
            Instance::IntConstrainedValue(data) => &data.name,
            Instance::IntValue(data) => &data.name,
            Instance::Keyframe(data) => &data.name,
            Instance::KeyframeMarker(data) => &data.name,
            Instance::KeyframeSequence(data) => &data.name,
            Instance::LineForce(data) => &data.name,
            Instance::LineHandleAdornment(data) => &data.name,
            Instance::LocalizationTable(data) => &data.name,
            Instance::LocalScript(data) => &data.name,
            Instance::ManualGlue(data) => &data.name,
            Instance::ManualWeld(data) => &data.name,
            Instance::MeshPart(data) => &data.name,
            Instance::Message(data) => &data.name,
            Instance::Model(data) => &data.name,
            Instance::ModuleScript(data) => &data.name,
            Instance::Motor(data) => &data.name,
            Instance::Motor6D(data) => &data.name,
            Instance::MotorFeature(data) => &data.name,
            Instance::NegateOperation(data) => &data.name,
            Instance::NoCollisionConstraint(data) => &data.name,
            Instance::NumberPose(data) => &data.name,
            Instance::NumberValue(data) => &data.name,
            Instance::ObjectValue(data) => &data.name,
            Instance::Pants(data) => &data.name,
            Instance::Part(data) => &data.name,
            Instance::ParticleEmitter(data) => &data.name,
            Instance::PartOperation(data) => &data.name,
            Instance::PartOperationAsset(data) => &data.name,
            Instance::PitchShiftSoundEffect(data) => &data.name,
            Instance::PointLight(data) => &data.name,
            Instance::Pose(data) => &data.name,
            Instance::PrismaticConstraint(data) => &data.name,
            Instance::ProximityPrompt(data) => &data.name,
            Instance::RayValue(data) => &data.name,
            Instance::ReflectionMetadata(data) => &data.name,
            Instance::ReflectionMetadataCallbacks(data) => &data.name,
            Instance::ReflectionMetadataClass(data) => &data.name,
            Instance::ReflectionMetadataClasses(data) => &data.name,
            Instance::ReflectionMetadataEnum(data) => &data.name,
            Instance::ReflectionMetadataEnumItem(data) => &data.name,
            Instance::ReflectionMetadataEnums(data) => &data.name,
            Instance::ReflectionMetadataEvents(data) => &data.name,
            Instance::ReflectionMetadataFunctions(data) => &data.name,
            Instance::ReflectionMetadataMember(data) => &data.name,
            Instance::ReflectionMetadataProperties(data) => &data.name,
            Instance::ReflectionMetadataYieldFunctions(data) => &data.name,
            Instance::RemoteEvent(data) => &data.name,
            Instance::RemoteFunction(data) => &data.name,
            Instance::RenderingTest(data) => &data.name,
            Instance::ReverbSoundEffect(data) => &data.name,
            Instance::RocketPropulsion(data) => &data.name,
            Instance::RodConstraint(data) => &data.name,
            Instance::RopeConstraint(data) => &data.name,
            Instance::Rotate(data) => &data.name,
            Instance::RotateP(data) => &data.name,
            Instance::RotateV(data) => &data.name,
            Instance::ScreenGui(data) => &data.name,
            Instance::Script(data) => &data.name,
            Instance::ScrollingFrame(data) => &data.name,
            Instance::Seat(data) => &data.name,
            Instance::SelectionBox(data) => &data.name,
            Instance::SelectionPartLasso(data) => &data.name,
            Instance::SelectionPointLasso(data) => &data.name,
            Instance::SelectionSphere(data) => &data.name,
            Instance::Shirt(data) => &data.name,
            Instance::ShirtGraphic(data) => &data.name,
            Instance::SkateboardController(data) => &data.name,
            Instance::SkateboardPlatform(data) => &data.name,
            Instance::Skin(data) => &data.name,
            Instance::Sky(data) => &data.name,
            Instance::Smoke(data) => &data.name,
            Instance::Snap(data) => &data.name,
            Instance::Sound(data) => &data.name,
            Instance::SoundGroup(data) => &data.name,
            Instance::Sparkles(data) => &data.name,
            Instance::SpawnLocation(data) => &data.name,
            Instance::SpecialMesh(data) => &data.name,
            Instance::SphereHandleAdornment(data) => &data.name,
            Instance::SpotLight(data) => &data.name,
            Instance::SpringConstraint(data) => &data.name,
            Instance::StandalonePluginScripts(data) => &data.name,
            Instance::StarterGear(data) => &data.name,
            Instance::StringValue(data) => &data.name,
            Instance::SunRaysEffect(data) => &data.name,
            Instance::SurfaceAppearance(data) => &data.name,
            Instance::SurfaceGui(data) => &data.name,
            Instance::SurfaceLight(data) => &data.name,
            Instance::SurfaceSelection(data) => &data.name,
            Instance::Team(data) => &data.name,
            Instance::TeleportOptions(data) => &data.name,
            Instance::Terrain(data) => &data.name,
            Instance::TerrainRegion(data) => &data.name,
            Instance::TextBox(data) => &data.name,
            Instance::TextButton(data) => &data.name,
            Instance::TextLabel(data) => &data.name,
            Instance::Texture(data) => &data.name,
            Instance::Tool(data) => &data.name,
            Instance::Torque(data) => &data.name,
            Instance::Trail(data) => &data.name,
            Instance::TremoloSoundEffect(data) => &data.name,
            Instance::TrussPart(data) => &data.name,
            Instance::Tween(data) => &data.name,
            Instance::UIAspectRatioConstraint(data) => &data.name,
            Instance::UICorner(data) => &data.name,
            Instance::UIGradient(data) => &data.name,
            Instance::UIGridLayout(data) => &data.name,
            Instance::UIListLayout(data) => &data.name,
            Instance::UIPadding(data) => &data.name,
            Instance::UIPageLayout(data) => &data.name,
            Instance::UIScale(data) => &data.name,
            Instance::UISizeConstraint(data) => &data.name,
            Instance::UIStroke(data) => &data.name,
            Instance::UITableLayout(data) => &data.name,
            Instance::UITextSizeConstraint(data) => &data.name,
            Instance::UnionOperation(data) => &data.name,
            Instance::UniversalConstraint(data) => &data.name,
            Instance::Vector3Value(data) => &data.name,
            Instance::VectorForce(data) => &data.name,
            Instance::VehicleController(data) => &data.name,
            Instance::VehicleSeat(data) => &data.name,
            Instance::VelocityMotor(data) => &data.name,
            Instance::VideoFrame(data) => &data.name,
            Instance::ViewportFrame(data) => &data.name,
            Instance::WedgePart(data) => &data.name,
            Instance::Weld(data) => &data.name,
            Instance::WeldConstraint(data) => &data.name,
            Instance::WorldModel(data) => &data.name,
            Instance::Other(_, props) => {
                if let Property::TextString(name) = props.get("Name").unwrap() {
                    name
                } else {
                    panic!("Instance didn't have a Name")
                }
            }
        }
    }
}

/// Information common to all instances, presumably part of Instance itself.
#[derive(Debug, Clone, Default, PropertyConvert)]
pub struct Base {
    /// The name of this instance
    pub name: String,
    /// Custom tags applied to the instance
    // FIXME(CraftSpider) This is most likely actually a map of some kind
    pub tags: String,
    /// The ID of the asset source for this instance
    // FIXME(CraftSpider)
    pub source_asset_id: i64,
    /// Serialized custom attributes on the instance
    // FIXME(CraftSpider) Deserialize this correctly, probably into a custom type?
    pub attributes_serialize: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Accessory {
    #[delegate]
    pub accoutrement: Accoutrement,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Accoutrement {
    #[delegate]
    pub base: Base,
    pub attachment_point: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Actor {
    #[delegate]
    pub model: Model,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct AlignOrientation {
    #[delegate]
    pub constraint: Constraint,

    pub primary_axis_only: bool,
    pub reaction_torque_enabled: bool,
    pub rigidity_enabled: bool,

    pub align_type: AlignType,

    pub max_angular_velocity: f32,
    pub max_torque: f32,
    pub responsiveness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct AlignPosition {
    #[delegate]
    pub constraint: Constraint,

    pub apply_at_center_of_mass: bool,
    pub reaction_force_enabled: bool,
    pub rigidity_enabled: bool,

    pub max_force: f32,
    pub max_velocity: f32,
    pub responsiveness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct AngularVelocity {
    #[delegate]
    pub constraint: Constraint,
    pub reaction_torque_enabled: bool,
    pub relative_to: ActuatorRelativeTo,
    pub max_torque: f32,
    pub angular_velocity: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Animation {
    #[delegate]
    pub base: Base,
    pub animation_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct AnimationController {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ArcHandles {
    #[delegate]
    pub part_adornment: PartAdornment,
    pub axes: Axes,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Atmosphere {
    #[delegate]
    pub base: Base,
    pub color: Color3,
    pub decay: Color3,
    pub density: f32,
    pub glare: f32,
    pub haze: f32,
    pub offset: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Backpack {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BackpackItem {
    #[delegate]
    pub base: Base,
    pub texture_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BallSocketConstraint {
    #[delegate]
    pub constraint: Constraint,

    pub limits_enabled: bool,
    pub twist_limits_enabled: bool,

    #[propname = "MaxFrictionTorqueXml"]
    pub max_friction_torque: f32,
    pub radius: f32,
    pub restitution: f32,
    pub upper_angle: f32,
    pub twist_upper_angle: f32,
    pub twist_lower_angle: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BasePart {
    #[delegate]
    pub base: Base,

    pub anchored: bool,
    pub locked: bool,
    pub massless: bool,
    pub can_touch: bool,
    pub can_collide: bool,
    pub cast_shadow: bool,
    pub can_query: bool,

    #[propname = "size"]
    pub size: Vector3,
    pub c_frame: CFrame,
    pub velocity: Vector3,
    pub rot_velocity: Vector3,
    pub pivot_offset: CFrame,

    pub material: Material,
    pub color3uint8: Color3Uint8,
    pub transparency: f32,
    pub reflectance: f32,

    pub collision_group_id: i32,
    pub custom_physical_properties: PhysicalProperties,
    pub root_priority: i32,

    pub top_surface: SurfaceType,
    pub top_surface_input: InputType,
    pub top_param_a: f32,
    pub top_param_b: f32,

    pub bottom_surface: SurfaceType,
    pub bottom_surface_input: InputType,
    pub bottom_param_a: f32,
    pub bottom_param_b: f32,

    pub front_surface: SurfaceType,
    pub front_surface_input: InputType,
    pub front_param_a: f32,
    pub front_param_b: f32,

    pub back_surface: SurfaceType,
    pub back_surface_input: InputType,
    pub back_param_a: f32,
    pub back_param_b: f32,

    pub left_surface: SurfaceType,
    pub left_surface_input: InputType,
    pub left_param_a: f32,
    pub left_param_b: f32,

    pub right_surface: SurfaceType,
    pub right_surface_input: InputType,
    pub right_param_a: f32,
    pub right_param_b: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BaseScript {
    #[delegate]
    pub source_container: LuaSourceContainer,
    pub disabled: bool,
    pub linked_source: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Beam {
    #[delegate]
    pub base: Base,

    pub enabled: bool,
    pub face_camera: bool,

    pub color: ColorSequence,
    pub transparency: NumberSequence,

    pub curve_size_0: f32,
    pub curve_size_1: f32,
    pub width_0: f32,
    pub width_1: f32,
    pub z_offset: f32,
    pub light_emission: f32,
    pub light_influence: f32,
    pub segments: i32,
    pub texture: String,
    pub texture_length: f32,
    pub texture_speed: f32,
    pub texture_mode: TextureMode,

    pub attachment_0: InstanceRef,
    pub attachment_1: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BevelMesh {
    #[delegate]
    pub data_model_mesh: DataModelMesh,
    pub bulge: f32,
    pub bevel: f32,
    #[propname = "Bevel Roundness"]
    pub bevel_roundness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BillboardGui {
    #[delegate]
    pub layer_collector: LayerCollector,
    pub active: bool,
    pub always_on_top: bool,
    pub clips_descendants: bool,

    pub distance_lower_limit: f32,
    pub distance_step: f32,
    pub distance_upper_limit: f32,
    pub max_distance: f32,
    pub light_influence: f32,

    pub size: UDim2,
    pub size_offset: Vector2,
    pub studs_offset: Vector3,
    pub studs_offset_world_space: Vector3,
    pub extents_offset: Vector3,
    pub extents_offset_world_space: Vector3,

    pub adornee: InstanceRef,
    pub player_to_hide_from: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BinaryStringValue {
    #[delegate]
    pub base: Base,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BindableEvent {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BindableFunction {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BlockMesh {
    #[delegate]
    pub bevel_mesh: BevelMesh,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BloomEffect {
    #[delegate]
    pub post_effect: PostEffect,
    pub intensity: f32,
    pub size: f32,
    pub threshold: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BlurEffect {
    #[delegate]
    pub post_effect: PostEffect,
    pub size: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyAngularVelocity {
    #[delegate]
    pub base: Base,
    pub p: f32,
    pub angular_velocity: Vector3,
    pub max_torque: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyColors {
    #[delegate]
    pub base: Base,
    pub head_color3: Color3,
    pub torso_color3: Color3,
    pub left_arm_color3: Color3,
    pub right_arm_color3: Color3,
    pub left_leg_color3: Color3,
    pub right_leg_color3: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyForce {
    #[delegate]
    pub base: Base,
    pub force: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyGyro {
    #[delegate]
    pub base: Base,
    pub d: f32,
    pub p: f32,
    pub c_frame: CFrame,
    pub max_torque: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyPosition {
    #[delegate]
    pub base: Base,
    pub d: f32,
    pub p: f32,
    pub position: Vector3,
    pub max_force: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyThrust {
    #[delegate]
    pub base: Base,
    pub force: Vector3,
    pub location: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyVelocity {
    #[delegate]
    pub base: Base,
    pub p: f32,
    pub velocity: Vector3,
    pub max_force: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BoolValue {
    #[delegate]
    pub base: Base,
    pub value: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BoxHandleAdornment {
    #[delegate]
    pub handle_adornment: HandleAdornment,
    pub size: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BrickColorValue {
    #[delegate]
    pub base: Base,
    pub value: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Camera {
    #[delegate]
    pub base: Base,
    pub head_locked: bool,
    pub c_frame: CFrame,
    pub camera_subject: InstanceRef,
    pub camera_type: CameraType,
    pub field_of_view: f32,
    pub field_of_view_mode: FieldOfViewMode,
    pub focus: CFrame,
    pub head_scale: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CFrameValue {
    #[delegate]
    pub base: Base,
    pub value: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CharacterMesh {
    #[delegate]
    pub base: Base,
    pub body_part: BodyPart,
    pub base_texture_id: i64,
    pub overlay_texture_id: i64,
    pub mesh_id: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ChorusSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub mix: f32,
    pub rate: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ClickDetector {
    #[delegate]
    pub base: Base,
    pub cursor_icon: String,
    pub max_activation_distance: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Clothing {
    #[delegate]
    pub base: Base,
    pub color3: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Clouds {
    #[delegate]
    pub base: Base,
    pub cover: f32,
    pub density: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Color3Value {
    #[delegate]
    pub base: Base,
    pub value: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ColorCorrectionEffect {
    #[delegate]
    pub post_effect: PostEffect,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub tint_color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CompressorSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub attack: f32,
    pub gain_makeup: f32,
    pub ratio: f32,
    pub release: f32,
    pub threshold: f32,
    pub side_chain: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ConeHandleAdornment {
    #[delegate]
    pub handle_adornment: HandleAdornment,
    pub height: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Configuration {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CornerWedgePart {
    #[delegate]
    pub base_part: BasePart,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Constraint {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub visible: bool,
    pub color: BrickColor,
    pub attachment_0: InstanceRef,
    pub attachment_1: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CustomEvent {
    #[delegate]
    pub base: Base,
    pub persisted_current_value: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CustomEventReceiver {
    #[delegate]
    pub base: Base,
    pub source: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CylinderHandleAdornment {
    #[delegate]
    pub handle_adornment: HandleAdornment,
    pub angle: f32,
    pub height: f32,
    pub inner_radius: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CylinderMesh {
    #[delegate]
    pub bevel_mesh: BevelMesh,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CylindricalConstraint {
    #[delegate]
    pub sliding_ball_constraint: SlidingBallConstraint,
    pub angular_limits_enabled: bool,
    pub rotation_axis_visible: bool,
    pub angular_actuator_type: ActuatorType,
    pub angular_restitution: f32,
    pub angular_speed: f32,
    pub angular_velocity: f32,
    pub inclination_angle: f32,
    pub lower_angle: f32,
    pub motor_max_angular_acceleration: f32,
    pub motor_max_torque: f32,
    pub target_angle: f32,
    pub upper_angle: f32,
    pub servo_max_torque: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DataModelMesh {
    #[delegate]
    pub base: Base,
    pub offset: Vector3,
    pub scale: Vector3,
    pub vertex_color: Vector3,
    #[propname = "LODX"]
    pub lodx: LevelOfDetailSetting,
    #[propname = "LODY"]
    pub lody: LevelOfDetailSetting,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Decal {
    #[delegate]
    pub face_instance: FaceInstance,
    pub color3: Color3,
    pub texture: String,
    pub transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DepthOfFieldEffect {
    #[delegate]
    pub post_effect: PostEffect,
    pub far_intensity: f32,
    pub focus_distance: f32,
    pub in_focus_radius: f32,
    pub near_intensity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Dialog {
    #[delegate]
    pub base: Base,
    pub goodbye_choice_active: bool,
    pub behavior_type: DialogBehaviorType,
    pub conversation_distance: f32,
    pub goodbye_dialog: String,
    pub initial_prompt: String,
    pub purpose: DialogPurpose,
    pub tone: DialogTone,
    pub trigger_distance: f32,
    pub trigger_offset: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DialogChoice {
    #[delegate]
    pub base: Base,
    pub goodbye_choice_active: bool,
    pub goodbye_dialog: String,
    pub response_dialog: String,
    pub user_dialog: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DistortionSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub level: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DoubleConstrainedValue {
    #[delegate]
    pub base: Base,
    pub min_value: f64,
    pub max_value: f64,
    #[propname = "value"]
    pub value: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DynamicRotate {
    #[delegate]
    pub joint_instance: JointInstance,
    pub base_angle: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct EchoSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub delay: f32,
    pub dry_level: f32,
    pub feedback: f32,
    pub wet_level: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct EqualizerSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub low_gain: f32,
    pub mid_gain: f32,
    pub high_gain: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Explosion {
    #[delegate]
    pub base: Base,
    pub visible: bool,
    pub blast_pressure: f32,
    pub blast_radius: f32,
    pub destroy_joint_radius_percent: f32,
    pub explosion_type: ExplosionType,
    pub position: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FaceInstance {
    #[delegate]
    pub base: Base,
    pub face: NormalId,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Feature {
    #[delegate]
    pub base: Base,
    #[propname = "FaceId"]
    pub face: NormalId,
    pub in_out: InOut,
    pub left_right: LeftRight,
    pub top_bottom: TopBottom,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FileMesh {
    #[delegate]
    pub data_model_mesh: DataModelMesh,
    pub mesh_id: String,
    pub texture_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Fire {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub color: Color3,
    pub secondary_color: Color3,
    #[propname = "heat_xml"]
    pub heat: f32,
    #[propname = "size_xml"]
    pub size: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Flag {
    #[delegate]
    pub tool: Tool,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FlagStand {
    #[delegate]
    pub part: Part,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FlangeSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub mix: f32,
    pub rate: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FloorWire {
    #[delegate]
    pub gui_base: GuiBase3D,
    pub cycle_offset: f32,
    pub studs_between_textures: f32,
    pub texture: String,
    pub texture_size: Vector2,
    pub velocity: f32,
    pub wire_radius: f32,
    pub from: InstanceRef,
    pub to: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Folder {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ForceField {
    #[delegate]
    pub base: Base,
    pub visible: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Frame {
    #[delegate]
    pub gui_object: GuiObject,
    pub style: FrameStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FunctionalTest {
    #[delegate]
    pub base: Base,
    pub has_migrated_settings_to_test_service: bool,
    pub description: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Glue {
    #[delegate]
    pub joint_instance: JointInstance,
    pub f0: Vector3,
    pub f1: Vector3,
    pub f2: Vector3,
    pub f3: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiBase2D {
    #[delegate]
    pub base: Base,
    pub auto_localize: bool,
    pub root_localization_table: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiBase3D {
    #[delegate]
    pub base: Base,
    pub visible: bool,
    pub color3: Color3,
    pub transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiButton {
    #[delegate]
    pub gui_object: GuiObject,
    pub auto_button_color: bool,
    pub modal: bool,
    pub selected: bool,
    pub style: ButtonStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiMain {
    #[delegate]
    pub screen_gui: ScreenGui,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiObject {
    #[delegate]
    pub gui_base: GuiBase2D,
    pub active: bool,
    pub clips_descendants: bool,
    pub draggable: bool,
    pub selectable: bool,
    pub visible: bool,

    pub border_mode: BorderMode,
    pub border_size_pixel: i32,
    pub border_color3: Color3,
    pub background_transparency: f32,
    pub background_color3: Color3,

    pub anchor_point: Vector2,
    pub position: UDim2,
    pub automatic_size: AutomaticSize,
    pub size_constraint: SizeConstraint,
    pub size: UDim2,
    pub layout_order: i32,
    pub z_index: i32,
    pub rotation: f32,

    pub selection_image_object: InstanceRef,
    pub next_selection_left: InstanceRef,
    pub next_selection_right: InstanceRef,
    pub next_selection_up: InstanceRef,
    pub next_selection_down: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HandleAdornment {
    #[delegate]
    pub pv_adornment: PVAdornment,
    pub always_on_top: bool,
    pub adorn_culling_mode: AdornCullingMode,
    pub z_index: i32,
    pub size_relative_offset: Vector3,
    pub c_frame: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Handles {
    #[delegate]
    pub part_adornment: PartAdornment,
    pub faces: Faces,
    pub style: HandlesStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Hat {
    #[delegate]
    pub accoutrement: Accoutrement,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HingeConstraint {
    #[delegate]
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub angular_speed: f32,
    pub angular_velocity: f32,
    pub lower_angle: f32,
    pub upper_angle: f32,
    pub target_angle: f32,
    pub motor_max_acceleration: f32,
    pub motor_max_torque: f32,
    pub radius: f32,
    pub restitution: f32,
    pub servo_max_torque: f32,
    pub actuator_type: ActuatorType,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Hint {
    #[delegate]
    pub message: Message,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Hole {
    #[delegate]
    pub feature: Feature,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HopperBin {
    #[delegate]
    pub backpack_item: BackpackItem,
    pub active: bool,
    pub bin_type: BinType,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Humanoid {
    #[delegate]
    pub base: Base,
    pub auto_jump_enabled: bool,
    pub auto_rotate: bool,
    pub automatic_scaling_enabled: bool,
    pub break_joints_on_death: bool,
    pub requires_neck: bool,
    pub use_jump_power: bool,

    pub rig_type: HumanoidRigType,
    pub name_occlusion: NameOcclusion,
    pub health_display_type: HumanoidHealthDisplayType,
    pub collision_type: HumanoidCollisionType,
    pub display_distance_type: HumanoidDisplayDistanceType,

    pub name_display_distance: f32,
    pub health_display_distance: f32,
    pub walk_speed: f32,
    pub max_health: f32,
    #[propname = "Health_XML"]
    pub health: f32,
    pub max_slope_angle: f32,
    pub jump_power: f32,
    pub jump_height: f32,
    pub hip_height: f32,
    pub internal_head_scale: f32,
    pub internal_body_scale: Vector3,

    pub display_name: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HumanoidController {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HumanoidDescription {
    #[delegate]
    pub base: Base,

    pub body_type_scale: f32,
    pub depth_scale: f32,
    pub head_scale: f32,
    pub height_scale: f32,
    pub proportion_scale: f32,
    pub width_scale: f32,

    pub climb_animation: i64,
    pub fall_animation: i64,
    pub idle_animation: i64,
    pub jump_animation: i64,
    pub walk_animation: i64,
    pub swim_animation: i64,
    pub run_animation: i64,

    pub face: i64,
    pub graphic_t_shirt: i64,
    pub pants: i64,
    pub shirt: i64,
    pub front_accessory: String,
    pub back_accessory: String,
    pub face_accessory: String,
    pub hair_accessory: String,
    pub hat_accessory: String,
    pub neck_accessory: String,
    pub shoulders_accessory: String,
    pub waist_accessory: String,
    pub emotes_data_internal: Vec<u8>,
    pub equipped_emotes_data_internal: Vec<u8>,

    pub head: i64,
    pub head_color: Color3,
    pub left_arm: i64,
    pub left_arm_color: Color3,
    pub left_leg: i64,
    pub left_leg_color: Color3,
    pub right_arm: i64,
    pub right_arm_color: Color3,
    pub right_leg: i64,
    pub right_leg_color: Color3,
    pub torso: i64,
    pub torso_color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ImageButton {
    #[delegate]
    pub gui_button: GuiButton,
    pub hover_image: String,
    pub image: String,
    pub pressed_image: String,
    pub image_color3: Color3,
    pub image_rect_offset: Vector2,
    pub image_rect_size: Vector2,
    pub image_transparency: f32,
    pub scale_type: ScaleType,
    pub slice_center: Rect,
    pub slice_scale: f32,
    pub tile_size: UDim2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ImageHandleAdornment {
    #[delegate]
    pub handle_adornment: HandleAdornment,
    pub image: String,
    pub size: Vector2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ImageLabel {
    #[delegate]
    pub gui_object: GuiObject,
    pub image: String,
    pub image_color3: Color3,
    pub image_rect_offset: Vector2,
    pub image_rect_size: Vector2,
    pub image_transparency: f32,
    pub scale_type: ScaleType,
    pub slice_center: Rect,
    pub slice_scale: f32,
    pub tile_size: UDim2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct InstanceAdornment {
    #[delegate]
    pub gui_base: GuiBase3D,
    pub adornee: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct IntConstrainedValue {
    #[delegate]
    pub base: Base,
    pub max_value: i64,
    pub min_value: i64,
    #[propname = "value"]
    pub value: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct IntValue {
    #[delegate]
    pub base: Base,
    pub value: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct JointInstance {
    #[delegate]
    pub base: Base,
    // pub active: bool,
    pub enabled: bool,
    pub c0: CFrame,
    pub c1: CFrame,
    pub part_0: InstanceRef,
    pub part_1: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Keyframe {
    #[delegate]
    pub base: Base,
    pub time: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct KeyframeMarker {
    #[delegate]
    pub base: Base,
    pub value: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct KeyframeSequence {
    #[delegate]
    pub base: Base,
    #[propname = "Loop"]
    pub loop_seq: bool,
    pub priority: AnimationPriority,
    pub authored_hip_height: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LayerCollector {
    #[delegate]
    pub gui_base: GuiBase2D,
    pub enabled: bool,
    pub reset_on_spawn: bool,
    pub z_index_behavior: ZIndexBehavior,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Light {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub shadows: bool,
    pub brightness: f32,
    pub color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LineForce {
    #[delegate]
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub inverse_square_law: bool,
    pub reaction_force_enabled: bool,
    pub magnitude: f32,
    pub max_force: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LineHandleAdornment {
    #[delegate]
    pub handle_adornment: HandleAdornment,
    pub length: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LocalizationTable {
    #[delegate]
    pub base: Base,
    pub contents: Vec<u8>,
    pub source_locale_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LocalScript {
    #[delegate]
    pub script: Script,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LuaSourceContainer {
    #[delegate]
    pub base: Base,
    pub script_guid: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ManualGlue {
    #[delegate]
    pub joint_instance: JointInstance,
    pub surface_0: i32,
    pub surface_1: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ManualWeld {
    #[delegate]
    pub joint_instance: JointInstance,
    pub surface_0: i32,
    pub surface_1: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct MeshPart {
    #[delegate]
    pub triangle_mesh_part: TriangleMeshPart,
    pub double_sided: bool,
    pub has_joint_offset: bool,
    pub has_skinned_mesh: bool,
    pub joint_offset: Vector3,
    pub mesh_id: String,
    // Deprecated mesh ID
    #[propname = "MeshID"]
    pub mesh_id2: String,
    pub render_fidelity: RenderFidelity,
    #[propname = "TextureID"]
    pub texture_id: String,
    pub pivot_offset: Option<CFrame>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Message {
    #[delegate]
    pub base: Base,
    pub text: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Model {
    #[delegate]
    pub base: Base,
    pub level_of_detail: ModelLevelOfDetail,
    pub model_in_primary: Option<CFrame>,
    #[cfg(feature = "mesh-format")]
    #[shared]
    pub model_mesh_data: TriMesh,
    #[cfg(not(feature = "mesh-format"))]
    #[shared]
    pub model_mesh_data: Vec<u8>,
    pub model_mesh_size: Vector3,
    pub model_mesh_c_frame: CFrame,
    pub primary_part: InstanceRef,
    pub needs_pivot_migration: bool,
    pub world_pivot_data: Pivot,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ModuleScript {
    #[delegate]
    pub lua_source_container: LuaSourceContainer,
    pub linked_source: String,
    pub source: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Motor {
    #[delegate]
    pub joint_instance: JointInstance,
    pub desired_angle: f32,
    pub max_velocity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Motor6D {
    #[delegate]
    pub motor: Motor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct MotorFeature {
    #[delegate]
    pub feature: Feature,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NegateOperation {
    #[delegate]
    pub part_operation: PartOperation,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NoCollisionConstraint {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub part_0: InstanceRef,
    pub part_1: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NumberPose {
    #[delegate]
    pub pose_base: PoseBase,
    pub value: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NumberValue {
    #[delegate]
    pub base: Base,
    pub value: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ObjectValue {
    #[delegate]
    pub base: Base,
    pub value: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Pants {
    #[delegate]
    pub clothing: Clothing,
    pub pants_template: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Part {
    #[delegate]
    pub base_part: BasePart,
    #[propname = "formFactorRaw"]
    pub form_factor: FormFactor,
    #[propname = "shape"]
    pub shape: PartType,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PartAdornment {
    #[delegate]
    pub gui_base: GuiBase3D,
    pub adornee: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ParticleEmitter {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub locked_to_part: bool,
    pub acceleration: Vector3,
    pub color: ColorSequence,
    pub drag: f32,
    pub emission_direction: NormalId,
    pub lifetime: NumberRange,
    pub light_emission: f32,
    pub light_influence: f32,
    pub orientation: ParticleOrientation,
    pub rate: f32,
    pub rot_speed: NumberRange,
    pub rotation: NumberRange,
    pub size: NumberSequence,
    pub speed: NumberRange,
    pub spread_angle: Vector2,
    pub texture: String,
    pub time_scale: f32,
    pub transparency: NumberSequence,
    pub velocity_inheritance: f32,
    pub z_offset: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PartOperation {
    #[delegate]
    pub triangle_mesh_part: TriangleMeshPart,
    pub use_part_color: bool,
    pub smoothing_angle: f32,
    pub render_fidelity: RenderFidelity,

    pub form_factor: FormFactor,
    pub asset_id: String,
    pub mesh_data: String,
    pub child_data: String,
    #[shared]
    pub mesh_data_2: String,
    #[shared]
    pub child_data_2: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PartOperationAsset {
    #[delegate]
    pub base: Base,
    pub mesh_data: Vec<u8>,
    pub child_data: Vec<u8>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PitchShiftSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub octave: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PointLight {
    #[delegate]
    pub light: Light,
    pub range: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Pose {
    #[delegate]
    pub pose_base: PoseBase,
    pub c_frame: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PoseBase {
    #[delegate]
    pub base: Base,
    pub easing_direction: PoseEasingDirection,
    pub easing_style: PoseEasingStyle,
    pub weight: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PostEffect {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PrismaticConstraint {
    #[delegate]
    pub sliding_ball_constraint: SlidingBallConstraint,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ProximityPrompt {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub auto_localize: bool,
    pub clickable_prompt: bool,
    pub requires_line_of_sight: bool,
    pub action_text: String,
    pub exclusivity: ProximityPromptExclusivity,
    pub gamepad_key_code: KeyCode,
    pub hold_duration: f32,
    pub keyboard_key_code: KeyCode,
    pub max_activation_distance: f32,
    pub object_text: String,
    pub root_localization_table: InstanceRef,
    pub style: ProximityPromptStyle,
    #[propname = "UIOffset"]
    pub ui_offset: Vector2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PVAdornment {
    #[delegate]
    pub gui_base: GuiBase3D,
    pub adornee: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RayValue {
    #[delegate]
    pub base: Base,
    pub value: Ray,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadata {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataCallbacks {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataClass {
    #[delegate]
    pub reflection_meta_item: ReflectionMetadataItem,
    pub insertable: bool,
    pub explorer_image_index: i32,
    pub explorer_order: i32,
    pub preferred_parent: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataClasses {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEnum {
    #[delegate]
    pub reflection_meta_item: ReflectionMetadataItem,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEnumItem {
    #[delegate]
    pub reflection_meta_item: ReflectionMetadataItem,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEnums {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEvents {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataFunctions {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataItem {
    #[delegate]
    pub base: Base,
    pub browsable: bool,
    pub client_only: bool,
    pub server_only: bool,
    pub deprecated: bool,
    pub editing_disabled: bool,
    pub is_backend: bool,
    pub class_category: String,
    pub constraint: String,
    pub script_context: String,
    #[propname = "summary"]
    pub summary: String,
    pub property_order: i32,
    #[propname = "UIMaximum"]
    pub ui_maximum: f64,
    #[propname = "UIMinimum"]
    pub ui_minimum: f64,
    #[propname = "UINumTicks"]
    pub ui_num_ticks: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataMember {
    #[delegate]
    pub reflection_meta_item: ReflectionMetadataItem,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataProperties {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataYieldFunctions {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RemoteEvent {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RemoteFunction {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RenderingTest {
    #[delegate]
    pub base: Base,
    pub should_skip: bool,
    pub c_frame: CFrame,
    pub comparison_diff_threshold: i32,
    pub comparison_method: RenderingTestComparisonMethod,
    pub comparison_psnr_threshold: f32,
    pub description: String,
    pub field_of_view: f32,
    pub quality_level: i32,
    pub ticket: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReverbSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub decay_time: f32,
    pub density: f32,
    pub diffusion: f32,
    pub dry_level: f32,
    pub wet_level: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RocketPropulsion {
    #[delegate]
    pub base: Base,
    pub cartoon_factor: f32,
    pub max_speed: f32,
    pub max_thrust: f32,
    pub max_torque: Vector3,
    pub target: InstanceRef,
    pub target_offset: Vector3,
    pub target_radius: f32,
    pub thrust_d: f32,
    pub thrust_p: f32,
    pub turn_d: f32,
    pub turn_p: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RodConstraint {
    #[delegate]
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub length: f32,
    pub limit_angle_0: f32,
    pub limit_angle_1: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RopeConstraint {
    #[delegate]
    pub constraint: Constraint,
    pub length: f32,
    pub restitution: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Rotate {
    #[delegate]
    pub joint_instance: JointInstance,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RotateP {
    #[delegate]
    pub dynamic_rotate: DynamicRotate,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RotateV {
    #[delegate]
    pub dynamic_rotate: DynamicRotate,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ScreenGui {
    #[delegate]
    pub layer_collector: LayerCollector,
    pub ignore_gui_inset: bool,
    pub display_order: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Script {
    #[delegate]
    pub base_script: BaseScript,
    pub source: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ScrollingFrame {
    #[delegate]
    pub gui_object: GuiObject,
    pub scrolling_enabled: bool,
    pub automatic_canvas_size: AutomaticSize,
    pub bottom_image: String,
    pub canvas_position: Vector2,
    pub canvas_size: UDim2,
    pub elastic_behavior: ElasticBehavior,
    pub horizontal_scroll_bar_inset: ScrollBarInset,
    pub mid_image: String,
    pub scroll_bar_image_color3: Color3,
    pub scroll_bar_image_transparency: f32,
    pub scroll_bar_thickness: i32,
    pub scrolling_direction: ScrollingDirection,
    pub top_image: String,
    pub vertical_scroll_bar_inset: ScrollBarInset,
    pub vertical_scroll_bar_position: VerticalScrollBarPosition,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Seat {
    #[delegate]
    pub part: Part,
    pub disabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionBox {
    #[delegate]
    pub instance_adornment: InstanceAdornment,
    pub line_thickness: f32,
    pub surface_color3: Color3,
    pub surface_transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionLasso {
    #[delegate]
    pub gui_base: GuiBase3D,
    pub humanoid: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionPartLasso {
    #[delegate]
    pub selection_lasso: SelectionLasso,
    pub part: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionPointLasso {
    #[delegate]
    pub selection_lasso: SelectionLasso,
    pub point: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionSphere {
    #[delegate]
    pub pv_adornment: PVAdornment,
    pub surface_color3: Color3,
    pub surface_transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Shirt {
    #[delegate]
    pub clothing: Clothing,
    pub shirt_template: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ShirtGraphic {
    #[delegate]
    pub base: Base,
    pub color3: Color3,
    pub graphic: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SkateboardController {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SkateboardPlatform {
    #[delegate]
    pub part: Part,
    pub sticky_wheels: bool,
    pub steer: i32,
    pub throttle: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Skin {
    #[delegate]
    pub base: Base,
    pub skin_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Sky {
    #[delegate]
    pub base: Base,
    pub celestial_bodies_shown: bool,
    pub moon_angular_size: f32,
    pub sun_angular_size: f32,
    pub star_count: i32,
    pub moon_texture_id: String,
    pub sun_texture_id: String,
    pub skybox_bk: String,
    pub skybox_dn: String,
    pub skybox_ft: String,
    pub skybox_lf: String,
    pub skybox_rt: String,
    pub skybox_up: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SlidingBallConstraint {
    #[delegate]
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub actuator_type: ActuatorType,
    pub lower_limit: f32,
    pub motor_max_acceleration: f32,
    pub motor_max_force: f32,
    pub restitution: f32,
    pub servo_max_force: f32,
    pub size: f32,
    pub speed: f32,
    pub target_position: f32,
    pub upper_limit: f32,
    pub velocity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Smoke {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub color: Color3,
    #[propname = "opacity_xml"]
    pub opacity: f32,
    #[propname = "riseVelocity_xml"]
    pub rise_velocity: f32,
    #[propname = "size_xml"]
    pub size: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Snap {
    #[delegate]
    pub joint_instance: JointInstance,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Sound {
    #[delegate]
    pub base: Base,
    pub looped: bool,
    pub playing: bool,
    pub play_on_remove: bool,
    pub playback_speed: f32,
    pub roll_off_mode: RollOffMode,
    pub sound_group: InstanceRef,
    #[propname = "xmlRead_MaxDistance_3"]
    pub max_distance: f32,
    pub time_position: f64,
    pub sound_id: String,
    pub volume: f32,
    pub emitter_size: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SoundEffect {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SoundGroup {
    #[delegate]
    pub base: Base,
    pub volume: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Sparkles {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub sparkle_color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpawnLocation {
    #[delegate]
    pub part: Part,
    pub allow_team_change_on_touch: bool,
    pub enabled: bool,
    pub neutral: bool,
    pub duration: i32,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpecialMesh {
    #[delegate]
    pub file_mesh: FileMesh,
    pub mesh_type: MeshType,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SphereHandleAdornment {
    #[delegate]
    pub handle_adornment: HandleAdornment,
    pub radius: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpotLight {
    #[delegate]
    pub light: Light,
    pub angle: f32,
    pub face: NormalId,
    pub range: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpringConstraint {
    #[delegate]
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub coils: f32,
    pub damping: f32,
    pub free_length: f32,
    pub max_force: f32,
    pub max_length: f32,
    pub min_length: f32,
    pub radius: f32,
    pub stiffness: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StandalonePluginScripts {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StarterGear {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StringValue {
    #[delegate]
    pub base: Base,
    pub value: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SunRaysEffect {
    #[delegate]
    pub post_effect: PostEffect,
    pub intensity: f32,
    pub spread: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceAppearance {
    #[delegate]
    pub base: Base,
    pub alpha_mode: AlphaMode,
    pub color_map: String,
    pub metalness_map: String,
    pub normal_map: String,
    pub roughness_map: String,
    pub texture_pack: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceGui {
    #[delegate]
    pub layer_collector: LayerCollector,
    pub active: bool,
    pub always_on_top: bool,
    pub clips_descendants: bool,
    pub adornee: InstanceRef,
    pub canvas_size: Vector2,
    pub face: NormalId,
    pub light_influence: f32,
    pub pixels_per_stud: f32,
    pub sizing_mode: SurfaceGuiSizingMode,
    pub tool_punch_through_distance: f32,
    pub z_offset: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceLight {
    #[delegate]
    pub light: Light,
    pub angle: f32,
    pub face: NormalId,
    pub range: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceSelection {
    #[delegate]
    pub part_adornment: PartAdornment,
    pub target_surface: NormalId,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Team {
    #[delegate]
    pub base: Base,
    pub auto_assignable: bool,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TeleportOptions {
    #[delegate]
    pub base: Base,
    pub should_reserve_server: bool,
    pub reserved_server_access_code: String,
    pub server_instance_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Terrain {
    #[delegate]
    pub base_part: BasePart,
    pub decoration: bool,
    pub acquisition_method: TerrainAcquisitionMethod,
    pub water_wave_size: f32,
    pub water_wave_speed: f32,
    pub water_color: Color3,
    pub water_reflectance: f32,
    pub water_transparency: f32,

    pub physics_grid: Vec<u8>,
    pub smooth_grid: Vec<u8>,
    pub material_colors: Vec<u8>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TerrainRegion {
    #[delegate]
    pub base: Base,
    pub smooth_grid: Vec<u8>,
    pub extents_max: Vector3Int16,
    pub extents_min: Vector3Int16,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TextBox {
    #[delegate]
    pub gui_object: GuiObject,
    pub clear_text_on_focus: bool,
    pub rich_text: bool,
    pub show_native_input: bool,
    pub text_editable: bool,
    pub text_scaled: bool,
    pub text_wrapped: bool,
    pub multi_line: bool,
    pub font: Font,
    pub line_height: f32,
    pub max_visible_graphemes: i32,
    pub placeholder_color3: Color3,
    pub placeholder_text: String,
    pub text: String,
    pub text_color3: Color3,
    pub text_size: f32,
    pub text_stroke_color3: Color3,
    pub text_stroke_transparency: f32,
    pub text_transparency: f32,
    pub text_truncate: TextTruncate,
    pub text_x_alignment: TextXAlignment,
    pub text_y_alignment: TextYAlignment,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TextButton {
    #[delegate]
    pub gui_button: GuiButton,
    pub rich_text: bool,
    pub text_scaled: bool,
    pub text_wrapped: bool,
    pub font: Font,
    pub line_height: f32,
    pub max_visible_graphemes: i32,
    pub text: String,
    pub text_color3: Color3,
    pub text_size: f32,
    pub text_stroke_color3: Color3,
    pub text_stroke_transparency: f32,
    pub text_transparency: f32,
    pub text_truncate: TextTruncate,
    pub text_x_alignment: TextXAlignment,
    pub text_y_alignment: TextYAlignment,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TextLabel {
    #[delegate]
    pub gui_object: GuiObject,
    pub rich_text: bool,
    pub text_scaled: bool,
    pub text_wrapped: bool,
    pub font: Font,
    pub line_height: f32,
    pub max_visible_graphemes: i32,
    pub text: String,
    pub text_color3: Color3,
    pub text_size: f32,
    pub text_stroke_color3: Color3,
    pub text_stroke_transparency: f32,
    pub text_transparency: f32,
    pub text_truncate: TextTruncate,
    pub text_x_alignment: TextXAlignment,
    pub text_y_alignment: TextYAlignment,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Texture {
    #[delegate]
    pub decal: Decal,
    pub offset_studs_u: f32,
    pub offset_studs_v: f32,
    pub studs_per_tile_u: f32,
    pub studs_per_tile_v: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Tool {
    #[delegate]
    pub backpack_item: BackpackItem,
    pub enabled: bool,
    pub can_be_dropped: bool,
    pub manual_activation_only: bool,
    pub requires_handle: bool,
    pub grip: CFrame,
    pub tool_tip: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Torque {
    #[delegate]
    pub constraint: Constraint,
    pub relative_to: ActuatorRelativeTo,
    pub torque: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Trail {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub face_camera: bool,
    pub attachment_0: InstanceRef,
    pub attachment_1: InstanceRef,
    pub color: ColorSequence,
    pub lifetime: f32,
    pub light_emission: f32,
    pub light_influence: f32,
    pub max_length: f32,
    pub min_length: f32,
    pub texture: String,
    pub texture_length: f32,
    pub texture_mode: TextureMode,
    pub transparency: NumberSequence,
    pub width_scale: NumberSequence,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TremoloSoundEffect {
    #[delegate]
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub duty: f32,
    pub frequency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TriangleMeshPart {
    #[delegate]
    pub base_part: BasePart,
    #[propname = "LODData"]
    pub lod_data: String,
    #[cfg(feature = "mesh-format")]
    #[shared]
    pub physical_config_data: TriMesh,
    #[cfg(not(feature = "mesh-format"))]
    #[shared]
    pub physical_config_data: Vec<u8>,
    pub physics_data: Vec<u8>,
    pub initial_size: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TrussPart {
    #[delegate]
    pub base_part: BasePart,
    #[propname = "style"]
    pub style: TrussStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Tween {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIAspectRatioConstraint {
    #[delegate]
    pub base: Base,
    pub aspect_ratio: f32,
    pub aspect_type: AspectType,
    pub dominant_axis: DominantAxis,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UICorner {
    #[delegate]
    pub base: Base,
    pub corner_radius: UDim,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIGradient {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub color: ColorSequence,
    pub offset: Vector2,
    pub rotation: f32,
    pub transparency: NumberSequence,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIGridLayout {
    #[delegate]
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub cell_padding: UDim2,
    pub cell_size: UDim2,
    pub fill_direction_max_cells: i32,
    pub start_corner: StartCorner,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIGridStyleLayout {
    #[delegate]
    pub base: Base,
    pub fill_direction: FillDirection,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIListLayout {
    #[delegate]
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub padding: UDim,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIPadding {
    #[delegate]
    pub base: Base,
    pub padding_top: UDim,
    pub padding_bottom: UDim,
    pub padding_left: UDim,
    pub padding_right: UDim,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIPageLayout {
    #[delegate]
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub animated: bool,
    pub circular: bool,
    pub gamepad_input_enabled: bool,
    pub scroll_wheel_input_enabled: bool,
    pub touch_input_enabled: bool,
    pub easing_direction: EasingDirection,
    pub easing_style: EasingStyle,
    pub padding: UDim,
    pub tween_time: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIScale {
    #[delegate]
    pub base: Base,
    pub scale: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UISizeConstraint {
    #[delegate]
    pub base: Base,
    pub max_size: Vector2,
    pub min_size: Vector2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIStroke {
    #[delegate]
    pub base: Base,
    pub enabled: bool,
    pub apply_stroke_mode: ApplyStrokeMode,
    pub color: Color3,
    pub line_join_mode: LineJoinMode,
    pub thickness: f32,
    pub transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UITableLayout {
    #[delegate]
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub fill_empty_space_columns: bool,
    pub fill_empty_space_rows: bool,
    pub major_axis: TableMajorAxis,
    pub padding: UDim2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UITextSizeConstraint {
    #[delegate]
    pub base: Base,
    pub min_text_size: i32,
    pub max_text_size: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UnionOperation {
    #[delegate]
    pub part_operation: PartOperation,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UniversalConstraint {
    #[delegate]
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub max_angle: f32,
    pub radius: f32,
    pub restitution: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Vector3Value {
    #[delegate]
    pub base: Base,
    pub value: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VectorForce {
    #[delegate]
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub force: Vector3,
    pub relative_to: ActuatorRelativeTo,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VehicleController {
    #[delegate]
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VehicleSeat {
    #[delegate]
    pub base_part: BasePart,
    pub disabled: bool,
    pub heads_up_display: bool,
    pub max_speed: f32,
    pub steer: i32,
    pub steer_float: f32,
    pub throttle: i32,
    pub throttle_float: f32,
    pub torque: f32,
    pub turn_speed: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VelocityMotor {
    #[delegate]
    pub joint_instance: JointInstance,
    pub current_angle: f32,
    pub desired_angle: f32,
    pub hole: InstanceRef,
    pub max_velocity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VideoFrame {
    #[delegate]
    pub gui_object: GuiObject,
    pub looped: bool,
    pub playing: bool,
    pub time_position: f64,
    pub video: String,
    pub volume: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ViewportFrame {
    #[delegate]
    pub gui_object: GuiObject,
    pub ambient: Color3,
    pub image_color3: Color3,
    pub image_transparency: f32,
    pub light_color: Color3,
    pub light_direction: Vector3,
    pub camera_c_frame: CFrame,
    pub camera_field_of_view: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct WedgePart {
    #[delegate]
    pub base_part: BasePart,
    #[propname = "formFactorRaw"]
    pub form_factor: FormFactor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Weld {
    #[delegate]
    pub joint_instance: JointInstance,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct WeldConstraint {
    #[delegate]
    pub base: Base,
    #[propname = "Part0Internal"]
    pub part_0: InstanceRef,
    #[propname = "Part1Internal"]
    pub part_1: InstanceRef,
    pub state: i32,
    #[propname = "CFrame0"]
    pub c_frame: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct WorldModel {
    #[delegate]
    pub model: Model,
}
