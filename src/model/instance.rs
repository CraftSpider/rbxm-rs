//! Crate for items related to Roblox Instances, the base components of all models.
//!
//! As the types here are based off of Lua classes with an inheritance structure,
//! `Deref`/`DerefMut` is used to emulate inheritance by allowing access to fields on the parent.
//!
//! This was chosen despite normally being bad practice due to the fact that these structs are
//! intentionally copying an inheritance structure, which would also normally be bad practice in
//! Rust, but is also the cleanest way to map the structure we are given.

#![allow(missing_docs)]

use super::InstanceRef;
use crate::model::data::*;
use crate::model::enums::*;
use crate::model::Property;
use crate::serde::internal::{FromProperties, ToProperties};
use rbxm_proc::{Inherits, InstanceExtra, PropertyConvert};

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use uuid::Uuid;

/// An Instance in a Roblox model. This is not meant to be matched exhaustively, more often
/// only checking if an Instance is of a specific kind, otherwise performing some default activity
/// or erroring out.
#[non_exhaustive]
#[derive(Debug, Clone, InstanceExtra)]
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
    Attachment(Attachment),
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
    Chat(Chat),
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
    Debris(Debris),
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
    HttpService(HttpService),
    Humanoid(Humanoid),
    HumanoidController(HumanoidController),
    HumanoidDescription(Box<HumanoidDescription>),
    ImageButton(ImageButton),
    ImageHandleAdornment(ImageHandleAdornment),
    ImageLabel(ImageLabel),
    InsertService(InsertService),
    IntConstrainedValue(IntConstrainedValue),
    IntValue(IntValue),
    Keyframe(Keyframe),
    KeyframeMarker(KeyframeMarker),
    KeyframeSequence(KeyframeSequence),
    Lighting(Lighting),
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
    PlayerEmulatorService(PlayerEmulatorService),
    Players(Players),
    PointLight(PointLight),
    Pose(Pose),
    PrismaticConstraint(PrismaticConstraint),
    ProximityPrompt(ProximityPrompt),
    ProximityPromptService(ProximityPromptService),
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
    ServerScriptService(ServerScriptService),
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
    SoundService(SoundService),
    Sparkles(Sparkles),
    SpawnLocation(SpawnLocation),
    SpecialMesh(SpecialMesh),
    SphereHandleAdornment(SphereHandleAdornment),
    SpotLight(SpotLight),
    SpringConstraint(SpringConstraint),
    StandalonePluginScripts(StandalonePluginScripts),
    StarterGear(StarterGear),
    StarterGui(StarterGui),
    StarterPack(StarterPack),
    StarterPlayer(StarterPlayer),
    StringValue(StringValue),
    StudioData(StudioData),
    SunRaysEffect(SunRaysEffect),
    SurfaceAppearance(SurfaceAppearance),
    SurfaceGui(SurfaceGui),
    SurfaceLight(SurfaceLight),
    SurfaceSelection(SurfaceSelection),
    Team(Team),
    Teams(Teams),
    TeleportOptions(TeleportOptions),
    Terrain(Terrain),
    TerrainRegion(TerrainRegion),
    TestService(TestService),
    TextBox(TextBox),
    TextButton(TextButton),
    TextLabel(TextLabel),
    Texture(Texture),
    TimerService(TimerService),
    Tool(Tool),
    Torque(Torque),
    Trail(Trail),
    TremoloSoundEffect(TremoloSoundEffect),
    TrussPart(TrussPart),
    Tween(Tween),
    TweenService(TweenService),
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
    Workspace(Workspace),
    WorldModel(WorldModel),

    /// Any unhandled class type falls here, with its name and properties preserved but
    /// uninterpreted
    // This must remain the last item in the enum for code generation
    Other(String, BTreeMap<String, Property>),
}

/// Information common to all instances, presumably part of Instance itself.
///
#[doc = doc_link!("class/Instance")]
#[derive(Debug, Clone, PropertyConvert)]
#[non_exhaustive]
pub struct Base {
    /// The name of this instance
    pub name: String,
    /// Custom tags applied to the instance
    // FIXME(CraftSpider) This is most likely actually a list of some kind
    pub tags: String,
    /// The ID of the asset source for this instance
    pub source_asset_id: i64,
    /// Serialized custom attributes on the instance
    #[propname = "AttributesSerialize"]
    pub attributes: Attributes,
    /// A UUID identifying this instance in a world. Generally not present in model files
    pub unique_id: Option<Uuid>,
}

impl Base {
    fn new_named(name: String) -> Base {
        Base {
            name,
            tags: String::new(),
            source_asset_id: 0,
            attributes: Attributes::default(),
            unique_id: None,
        }
    }
}

/// A character accessory, like a hat or armband. The newer successor to [`Hat`]
///
#[doc = doc_link!("class/Accessory")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Accessory {
    pub accoutrement: Accoutrement,
}

impl Accessory {
    /// Create a new accessory instance
    #[must_use]
    pub fn new() -> Accessory {
        Accessory::new_named(String::from("Accessory"))
    }

    /// Create a new accessory with the given name
    #[must_use]
    pub fn new_named(name: String) -> Accessory {
        Accessory {
            accoutrement: Accoutrement::new_named(name),
        }
    }
}

impl Default for Accessory {
    fn default() -> Self {
        Accessory::new()
    }
}

/// An item worn by a character, like a [`Hat`] or [`Accessory`]
///
#[doc = doc_link!("class/Accoutrement")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Accoutrement {
    pub base: Base,
    pub attachment_point: CFrame,
}

impl Accoutrement {
    /// Create a new Accoutrement
    #[must_use]
    pub fn new() -> Accoutrement {
        Accoutrement::new_named(String::from("Accoutrement"))
    }

    /// Create a new Accoutrement with the given name
    #[must_use]
    pub fn new_named(name: String) -> Accoutrement {
        Accoutrement {
            base: Base::new_named(name),
            attachment_point: CFrame::default(),
        }
    }
}

impl Default for Accoutrement {
    fn default() -> Self {
        Accoutrement::new()
    }
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Actor {
    pub model: Model,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct AlignOrientation {
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
    pub constraint: Constraint,
    pub reaction_torque_enabled: bool,
    pub relative_to: ActuatorRelativeTo,
    pub max_torque: f32,
    pub angular_velocity: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Animation {
    pub base: Base,
    pub animation_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct AnimationController {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ArcHandles {
    pub part_adornment: PartAdornment,
    pub axes: Axes,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Atmosphere {
    pub base: Base,
    pub color: Color3,
    pub decay: Color3,
    pub density: f32,
    pub glare: f32,
    pub haze: f32,
    pub offset: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Attachment {
    pub base: Base,
    pub axis: Option<Vector3>,
    #[propname = "CFrame"]
    pub cframe: Option<CFrame>,
    pub orientation: Option<Vector3>,
    pub position: Option<Vector3>,
    pub secondary_axis: Option<Vector3>,
    pub visible: bool,
    pub world_axis: Option<Vector3>,
    #[propname = "WorldCFrame"]
    pub world_cframe: Option<CFrame>,
    pub world_position: Option<Vector3>,
    pub world_secondary_axis: Option<Vector3>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Backpack {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BackpackItem {
    pub base: Base,
    pub texture_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BallSocketConstraint {
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
#[non_exhaustive]
pub struct BasePart {
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
    #[propname = "CFrame"]
    pub cframe: CFrame,
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

impl BasePart {
    fn new_named(name: String) -> BasePart {
        BasePart {
            base: Base::new_named(name),

            anchored: false,
            locked: false,
            massless: false,
            can_touch: true,
            can_collide: true,
            cast_shadow: true,
            can_query: true,

            size: Vector3::new(1.0, 1.0, 1.0),
            cframe: CFrame::default(),
            velocity: Vector3::default(),
            rot_velocity: Vector3::default(),
            pivot_offset: CFrame::default(),

            material: Material::Plastic,
            color3uint8: Color3Uint8::default(),
            transparency: 0.0,
            reflectance: 0.0,

            collision_group_id: 0,
            custom_physical_properties: PhysicalProperties::Default,
            root_priority: 0,

            top_surface: SurfaceType::Smooth,
            top_surface_input: InputType::NoInput,
            top_param_a: 0.0,
            top_param_b: 0.0,

            bottom_surface: SurfaceType::Smooth,
            bottom_surface_input: InputType::NoInput,
            bottom_param_a: 0.0,
            bottom_param_b: 0.0,

            front_surface: SurfaceType::Smooth,
            front_surface_input: InputType::NoInput,
            front_param_a: 0.0,
            front_param_b: 0.0,

            back_surface: SurfaceType::Smooth,
            back_surface_input: InputType::NoInput,
            back_param_a: 0.0,
            back_param_b: 0.0,

            left_surface: SurfaceType::Smooth,
            left_surface_input: InputType::NoInput,
            left_param_a: 0.0,
            left_param_b: 0.0,

            right_surface: SurfaceType::Smooth,
            right_surface_input: InputType::NoInput,
            right_param_a: 0.0,
            right_param_b: 0.0,
        }
    }
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BaseScript {
    pub source_container: LuaSourceContainer,
    pub disabled: bool,
    pub linked_source: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Beam {
    pub base: Base,

    pub enabled: bool,
    pub face_camera: bool,

    pub color: ColorSequence,
    pub transparency: NumberSequence,
    pub brightness: f32,

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
    pub data_model_mesh: DataModelMesh,
    pub bulge: f32,
    pub bevel: f32,
    #[propname = "Bevel Roundness"]
    pub bevel_roundness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BillboardGui {
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
    pub base: Base,
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BindableEvent {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BindableFunction {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BlockMesh {
    pub bevel_mesh: BevelMesh,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BloomEffect {
    pub post_effect: PostEffect,
    pub intensity: f32,
    pub size: f32,
    pub threshold: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BlurEffect {
    pub post_effect: PostEffect,
    pub size: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyAngularVelocity {
    pub base: Base,
    pub p: f32,
    pub angular_velocity: Vector3,
    pub max_torque: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyColors {
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
    pub base: Base,
    pub force: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyGyro {
    pub base: Base,
    pub d: f32,
    pub p: f32,
    #[propname = "CFrame"]
    pub cframe: CFrame,
    pub max_torque: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyPosition {
    pub base: Base,
    pub d: f32,
    pub p: f32,
    pub position: Vector3,
    pub max_force: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyThrust {
    pub base: Base,
    pub force: Vector3,
    pub location: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BodyVelocity {
    pub base: Base,
    pub p: f32,
    pub velocity: Vector3,
    pub max_force: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BoolValue {
    pub base: Base,
    pub value: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BoxHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub size: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct BrickColorValue {
    pub base: Base,
    pub value: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Camera {
    pub base: Base,
    pub head_locked: bool,
    #[propname = "CFrame"]
    pub cframe: CFrame,
    pub camera_subject: InstanceRef,
    pub camera_type: CameraType,
    pub field_of_view: f32,
    pub field_of_view_mode: FieldOfViewMode,
    pub focus: CFrame,
    pub head_scale: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CFrameValue {
    pub base: Base,
    pub value: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CharacterMesh {
    pub base: Base,
    pub body_part: BodyPart,
    pub base_texture_id: i64,
    pub overlay_texture_id: i64,
    pub mesh_id: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Chat {
    pub base: Base,
    pub bubble_chat_enabled: bool,
    pub load_default_chat: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ChorusSoundEffect {
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub mix: f32,
    pub rate: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ClickDetector {
    pub base: Base,
    pub cursor_icon: String,
    pub max_activation_distance: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Clothing {
    pub base: Base,
    pub color3: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Clouds {
    pub base: Base,
    pub cover: f32,
    pub density: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Color3Value {
    pub base: Base,
    pub value: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ColorCorrectionEffect {
    pub post_effect: PostEffect,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub tint_color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CompressorSoundEffect {
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
    pub handle_adornment: HandleAdornment,
    pub height: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Configuration {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CornerWedgePart {
    pub base_part: BasePart,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Constraint {
    pub base: Base,
    pub enabled: bool,
    pub visible: bool,
    pub color: BrickColor,
    pub attachment_0: InstanceRef,
    pub attachment_1: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CustomEvent {
    pub base: Base,
    pub persisted_current_value: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CustomEventReceiver {
    pub base: Base,
    pub source: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CylinderHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub angle: f32,
    pub height: f32,
    pub inner_radius: f32,
    pub radius: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CylinderMesh {
    pub bevel_mesh: BevelMesh,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct CylindricalConstraint {
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
pub struct Debris {
    pub base: Base,
    pub max_items: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Decal {
    pub face_instance: FaceInstance,
    pub color3: Color3,
    pub texture: String,
    pub transparency: f32,
    pub z_index: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DepthOfFieldEffect {
    pub post_effect: PostEffect,
    pub far_intensity: f32,
    pub focus_distance: f32,
    pub in_focus_radius: f32,
    pub near_intensity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Dialog {
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
    pub base: Base,
    pub goodbye_choice_active: bool,
    pub goodbye_dialog: String,
    pub response_dialog: String,
    pub user_dialog: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DistortionSoundEffect {
    pub sound_effect: SoundEffect,
    pub level: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DoubleConstrainedValue {
    pub base: Base,
    pub min_value: f64,
    pub max_value: f64,
    #[propname = "value"]
    pub value: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct DynamicRotate {
    pub joint_instance: JointInstance,
    pub base_angle: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct EchoSoundEffect {
    pub sound_effect: SoundEffect,
    pub delay: f32,
    pub dry_level: f32,
    pub feedback: f32,
    pub wet_level: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct EqualizerSoundEffect {
    pub sound_effect: SoundEffect,
    pub low_gain: f32,
    pub mid_gain: f32,
    pub high_gain: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Explosion {
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
    pub base: Base,
    pub face: NormalId,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Feature {
    pub base: Base,
    #[propname = "FaceId"]
    pub face: NormalId,
    pub in_out: InOut,
    pub left_right: LeftRight,
    pub top_bottom: TopBottom,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FileMesh {
    pub data_model_mesh: DataModelMesh,
    pub mesh_id: String,
    pub texture_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Fire {
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
    pub tool: Tool,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FlagStand {
    pub part: Part,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FlangeSoundEffect {
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub mix: f32,
    pub rate: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FloorWire {
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
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ForceField {
    pub base: Base,
    pub visible: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Frame {
    pub gui_object: GuiObject,
    pub style: FrameStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct FunctionalTest {
    pub base: Base,
    pub has_migrated_settings_to_test_service: bool,
    pub description: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Glue {
    pub joint_instance: JointInstance,
    pub f0: Vector3,
    pub f1: Vector3,
    pub f2: Vector3,
    pub f3: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiBase2D {
    pub base: Base,
    pub auto_localize: bool,
    pub root_localization_table: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiBase3D {
    pub base: Base,
    pub visible: bool,
    pub color3: Color3,
    pub transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiButton {
    pub gui_object: GuiObject,
    pub auto_button_color: bool,
    pub modal: bool,
    pub selected: bool,
    pub style: ButtonStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiMain {
    pub screen_gui: ScreenGui,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct GuiObject {
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
    pub pv_adornment: PVAdornment,
    pub always_on_top: bool,
    pub adorn_culling_mode: AdornCullingMode,
    pub z_index: i32,
    pub size_relative_offset: Vector3,
    #[propname = "CFrame"]
    pub cframe: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Handles {
    pub part_adornment: PartAdornment,
    pub faces: Faces,
    pub style: HandlesStyle,
}

/// A hat worn by a player
///
#[doc = doc_link!("class/Hat")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Hat {
    pub accoutrement: Accoutrement,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HingeConstraint {
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
    pub message: Message,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Hole {
    pub feature: Feature,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HopperBin {
    pub backpack_item: BackpackItem,
    pub active: bool,
    pub bin_type: BinType,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HttpService {
    pub base: Base,
    pub http_enabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Humanoid {
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
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct HumanoidDescription {
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
    pub gui_button: GuiButton,
    pub hover_image: String,
    pub image: String,
    pub pressed_image: String,
    pub image_color3: Color3,
    pub image_rect_offset: Vector2,
    pub image_rect_size: Vector2,
    pub image_transparency: f32,
    pub resample_mode: ResamplerMode,
    pub scale_type: ScaleType,
    pub slice_center: Rect,
    pub slice_scale: f32,
    pub tile_size: UDim2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ImageHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub image: String,
    pub size: Vector2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ImageLabel {
    pub gui_object: GuiObject,
    pub image: String,
    pub image_color3: Color3,
    pub image_rect_offset: Vector2,
    pub image_rect_size: Vector2,
    pub image_transparency: f32,
    pub resample_mode: ResamplerMode,
    pub scale_type: ScaleType,
    pub slice_center: Rect,
    pub slice_scale: f32,
    pub tile_size: UDim2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct InsertService {
    pub base: Base,
    pub allow_client_insert_models: bool,
    pub allow_insert_free_models: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct InstanceAdornment {
    pub gui_base: GuiBase3D,
    pub adornee: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct IntConstrainedValue {
    pub base: Base,
    pub max_value: i64,
    pub min_value: i64,
    #[propname = "value"]
    pub value: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct IntValue {
    pub base: Base,
    pub value: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct JointInstance {
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
    pub base: Base,
    pub time: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct KeyframeMarker {
    pub base: Base,
    pub value: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct KeyframeSequence {
    pub base: Base,
    #[propname = "Loop"]
    pub loop_seq: bool,
    pub priority: AnimationPriority,
    pub authored_hip_height: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LayerCollector {
    pub gui_base: GuiBase2D,
    pub enabled: bool,
    pub reset_on_spawn: bool,
    pub z_index_behavior: ZIndexBehavior,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Light {
    pub base: Base,
    pub enabled: bool,
    pub shadows: bool,
    pub brightness: f32,
    pub color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Lighting {
    pub base: Base,
    pub ambient: Color3,
    pub brightness: f32,
    #[propname = "ColorShift_Bottom"]
    pub color_shift_bottom: Color3,
    #[propname = "ColorShift_Top"]
    pub color_shift_top: Color3,
    pub environment_diffuse_scale: f32,
    pub environment_specular_scale: f32,
    pub exposure_compensation: f32,
    pub fog_color: Color3,
    pub fog_end: f32,
    pub fog_start: f32,
    pub geographic_latitude: f32,
    pub global_shadows: bool,
    pub outdoor_ambient: Color3,
    pub outlines: bool,
    pub shadow_softness: f32,
    pub technology: Technology,
    pub time_of_day: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LineForce {
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub inverse_square_law: bool,
    pub reaction_force_enabled: bool,
    pub magnitude: f32,
    pub max_force: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LineHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub length: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LocalizationTable {
    pub base: Base,
    pub contents: Vec<u8>,
    pub source_locale_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LocalScript {
    pub script: Script,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct LuaSourceContainer {
    pub base: Base,
    pub script_guid: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ManualGlue {
    pub joint_instance: JointInstance,
    pub surface_0: i32,
    pub surface_1: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ManualWeld {
    pub joint_instance: JointInstance,
    pub surface_0: i32,
    pub surface_1: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct MeshPart {
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
    pub base: Base,
    pub text: String,
}

/// A model represents a group of objects in space, for example all the parts that make up a chair.
/// If looking for a non-geometric collection, use [`Folder`]
///
#[doc = doc_link!("class/Model")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Model {
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
    #[propname = "ModelMeshCFrame"]
    pub model_mesh_cframe: CFrame,
    pub primary_part: InstanceRef,
    pub needs_pivot_migration: bool,
    pub world_pivot_data: Pivot,
}

impl Model {
    /// Create a new model with default values
    #[must_use]
    pub fn new() -> Model {
        Model::new_named(String::from("Model"))
    }

    /// Create a new model with the given name
    #[must_use]
    pub fn new_named(name: String) -> Model {
        Model {
            base: Base::new_named(name),
            level_of_detail: ModelLevelOfDetail::default(),
            model_in_primary: None,
            model_mesh_data: Default::default(),
            model_mesh_size: Vector3::default(),
            model_mesh_cframe: CFrame::default(),
            primary_part: InstanceRef::Null,
            needs_pivot_migration: false,
            world_pivot_data: Pivot::default(),
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::new()
    }
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ModuleScript {
    pub lua_source_container: LuaSourceContainer,
    pub linked_source: String,
    pub source: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Motor {
    pub joint_instance: JointInstance,
    pub desired_angle: f32,
    pub max_velocity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Motor6D {
    pub motor: Motor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct MotorFeature {
    pub feature: Feature,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NegateOperation {
    pub part_operation: PartOperation,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NoCollisionConstraint {
    pub base: Base,
    pub enabled: bool,
    pub part_0: InstanceRef,
    pub part_1: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NumberPose {
    pub pose_base: PoseBase,
    pub value: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct NumberValue {
    pub base: Base,
    pub value: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ObjectValue {
    pub base: Base,
    pub value: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Pants {
    pub clothing: Clothing,
    pub pants_template: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Part {
    pub base_part: BasePart,
    #[propname = "formFactorRaw"]
    pub form_factor: FormFactor,
    #[propname = "shape"]
    pub shape: PartType,
}

impl Part {
    /// Create a new part
    #[must_use]
    pub fn new() -> Part {
        Part::new_named(String::from("Part"))
    }

    /// Create a new part with the given name
    #[must_use]
    pub fn new_named(name: String) -> Part {
        Part {
            base_part: BasePart::new_named(name),
            form_factor: FormFactor::Brick,
            shape: PartType::Block,
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Part::new()
    }
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PartAdornment {
    pub gui_base: GuiBase3D,
    pub adornee: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ParticleEmitter {
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
    pub base: Base,
    pub mesh_data: Vec<u8>,
    pub child_data: Vec<u8>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PitchShiftSoundEffect {
    pub sound_effect: SoundEffect,
    pub octave: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PlayerEmulatorService {
    pub base: Base,
    pub custom_policies_enabled: bool,
    pub emulated_country_code: String,
    pub emulated_game_locale: String,
    pub player_emulation_enabled: bool,
    pub serialized_emulated_policy_info: Vec<u8>,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Players {
    pub base: Base,
    pub character_auto_loads: bool,
    #[propname = "MaxPlayersInternal"]
    pub max_players: i32,
    #[propname = "PreferredPlayersInternal"]
    pub preferred_players: i32,
    pub respawn_time: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PointLight {
    pub light: Light,
    pub range: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Pose {
    pub pose_base: PoseBase,
    #[propname = "CFrame"]
    pub cframe: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PoseBase {
    pub base: Base,
    pub easing_direction: PoseEasingDirection,
    pub easing_style: PoseEasingStyle,
    pub weight: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PostEffect {
    pub base: Base,
    pub enabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PrismaticConstraint {
    pub sliding_ball_constraint: SlidingBallConstraint,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ProximityPrompt {
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
pub struct ProximityPromptService {
    pub base: Base,
    pub enabled: bool,
    pub max_prompts_visible: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct PVAdornment {
    pub gui_base: GuiBase3D,
    pub adornee: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RayValue {
    pub base: Base,
    pub value: Ray,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadata {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataCallbacks {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataClass {
    pub reflection_meta_item: ReflectionMetadataItem,
    pub insertable: bool,
    pub explorer_image_index: i32,
    pub explorer_order: i32,
    pub preferred_parent: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataClasses {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEnum {
    pub reflection_meta_item: ReflectionMetadataItem,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEnumItem {
    pub reflection_meta_item: ReflectionMetadataItem,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEnums {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataEvents {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataFunctions {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataItem {
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
    pub reflection_meta_item: ReflectionMetadataItem,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataProperties {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ReflectionMetadataYieldFunctions {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RemoteEvent {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RemoteFunction {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RenderingTest {
    pub base: Base,
    pub should_skip: bool,
    #[propname = "CFrame"]
    pub cframe: CFrame,
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
    pub sound_effect: SoundEffect,
    pub decay_time: f32,
    pub density: f32,
    pub diffusion: f32,
    pub dry_level: f32,
    pub wet_level: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RocketPropulsion {
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
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub length: f32,
    pub limit_angle_0: f32,
    pub limit_angle_1: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RopeConstraint {
    pub constraint: Constraint,
    pub length: f32,
    pub restitution: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Rotate {
    pub joint_instance: JointInstance,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RotateP {
    pub dynamic_rotate: DynamicRotate,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct RotateV {
    pub dynamic_rotate: DynamicRotate,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ScreenGui {
    pub layer_collector: LayerCollector,
    pub ignore_gui_inset: bool,
    pub display_order: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Script {
    pub base_script: BaseScript,
    pub source: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ScrollingFrame {
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
    pub part: Part,
    pub disabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionBox {
    pub instance_adornment: InstanceAdornment,
    pub line_thickness: f32,
    pub surface_color3: Color3,
    pub surface_transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionLasso {
    pub gui_base: GuiBase3D,
    pub humanoid: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionPartLasso {
    pub selection_lasso: SelectionLasso,
    pub part: InstanceRef,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionPointLasso {
    pub selection_lasso: SelectionLasso,
    pub point: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SelectionSphere {
    pub pv_adornment: PVAdornment,
    pub surface_color3: Color3,
    pub surface_transparency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ServerScriptService {
    pub base: Base,
    pub load_string_enabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Shirt {
    pub clothing: Clothing,
    pub shirt_template: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ShirtGraphic {
    pub base: Base,
    pub color3: Color3,
    pub graphic: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SkateboardController {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SkateboardPlatform {
    pub part: Part,
    pub sticky_wheels: bool,
    pub steer: i32,
    pub throttle: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Skin {
    pub base: Base,
    pub skin_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Sky {
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
    pub joint_instance: JointInstance,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Sound {
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
    pub base: Base,
    pub enabled: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SoundGroup {
    pub base: Base,
    pub volume: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SoundService {
    pub base: Base,
    pub ambient_reverb: ReverbType,
    pub distance_factor: f32,
    pub doppler_scale: f32,
    pub respect_filtering_enabled: bool,
    pub rolloff_scale: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Sparkles {
    pub base: Base,
    pub enabled: bool,
    pub sparkle_color: Color3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpawnLocation {
    pub part: Part,
    pub allow_team_change_on_touch: bool,
    pub enabled: bool,
    pub neutral: bool,
    pub duration: i32,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpecialMesh {
    pub file_mesh: FileMesh,
    pub mesh_type: MeshType,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SphereHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub radius: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpotLight {
    pub light: Light,
    pub angle: f32,
    pub face: NormalId,
    pub range: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SpringConstraint {
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
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StarterGear {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StarterGui {
    pub base: Base,
    pub reset_player_gui_on_spawn: bool,
    pub screen_orientation: ScreenOrientation,
    pub show_development_gui: bool,
    pub virtual_cursor_mode: VirtualCursorMode,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StarterPack {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StarterPlayer {
    pub base: Base,
    pub allow_custom_animations: bool,
    pub auto_jump_enabled: bool,
    pub camera_max_zoom_distance: f32,
    pub camera_min_zoom_distance: f32,
    pub camera_mode: CameraMode,
    pub character_jump_height: f32,
    pub character_jump_power: f32,
    pub character_max_slope_angle: f32,
    pub character_use_jump_power: bool,
    pub character_walk_speed: f32,
    pub dev_camera_occlusion_mode: DevCameraOcclusionMode,
    pub dev_computer_camera_movement_mode: DevComputerCameraMovementMode,
    pub dev_computer_movement_mode: DevComputerMovementMode,
    pub dev_touch_camera_movement_mode: DevTouchCameraMovementMode,
    pub dev_touch_movement_mode: DevTouchMovementMode,
    pub enable_mouse_lock_option: bool,
    #[propname = "GameSettingsAssetIDFace"]
    pub game_settings_asset_id_face: i64,
    #[propname = "GameSettingsAssetIDHead"]
    pub game_settings_asset_id_head: i64,
    #[propname = "GameSettingsAssetIDLeftArm"]
    pub game_settings_asset_id_left_arm: i64,
    #[propname = "GameSettingsAssetIDLeftLeg"]
    pub game_settings_asset_id_left_leg: i64,
    #[propname = "GameSettingsAssetIDPants"]
    pub game_settings_asset_id_pants: i64,
    #[propname = "GameSettingsAssetIDRightArm"]
    pub game_settings_asset_id_right_arm: i64,
    #[propname = "GameSettingsAssetIDRightLeg"]
    pub game_settings_asset_id_right_leg: i64,
    #[propname = "GameSettingsAssetIDShirt"]
    pub game_settings_asset_id_shirt: i64,
    #[propname = "GameSettingsAssetIDTeeShirt"]
    pub game_settings_asset_id_tee_shirt: i64,
    #[propname = "GameSettingsAssetIDTorso"]
    pub game_settings_asset_id_torso: i64,
    pub game_settings_avatar: GameAvatarType,
    pub game_settings_r15_collision: R15CollisionType,
    pub game_settings_scale_range_body_type: NumberRange,
    pub game_settings_scale_range_head: NumberRange,
    pub game_settings_scale_range_height: NumberRange,
    pub game_settings_scale_range_proportion: NumberRange,
    pub game_settings_scale_range_width: NumberRange,
    pub health_display_distance: f32,
    pub load_character_appearance: bool,
    pub load_character_layered_clothing: LoadCharacterLayeredClothing,
    pub name_display_distance: f32,
    pub user_emotes_enabled: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StringValue {
    pub base: Base,
    pub value: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct StudioData {
    pub base: Base,
    pub commit_inflight_author_id: i64,
    pub commit_inflight_guid: String,
    pub commit_inflight_place_version: i32,
    pub enable_script_collab_by_default_on_load: bool,
    pub enable_team_create_streaming_on_load: bool,
    pub src_place_id: i64,
    pub src_universe_id: i64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SunRaysEffect {
    pub post_effect: PostEffect,
    pub intensity: f32,
    pub spread: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceAppearance {
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
    pub layer_collector: LayerCollector,
    pub active: bool,
    pub always_on_top: bool,
    pub clips_descendants: bool,
    pub adornee: InstanceRef,
    pub canvas_size: Vector2,
    pub face: NormalId,
    pub light_influence: f32,
    pub pixels_per_stud: f32,
    pub brightness: f32,
    pub sizing_mode: SurfaceGuiSizingMode,
    pub tool_punch_through_distance: f32,
    pub z_offset: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceLight {
    pub light: Light,
    pub angle: f32,
    pub face: NormalId,
    pub range: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct SurfaceSelection {
    pub part_adornment: PartAdornment,
    pub target_surface: NormalId,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Team {
    pub base: Base,
    pub auto_assignable: bool,
    pub team_color: BrickColor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Teams {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TeleportOptions {
    pub base: Base,
    pub should_reserve_server: bool,
    pub reserved_server_access_code: String,
    pub server_instance_id: String,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Terrain {
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
    pub base: Base,
    pub smooth_grid: Vec<u8>,
    pub extents_max: Vector3Int16,
    pub extents_min: Vector3Int16,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TestService {
    pub base: Base,
    pub auto_runs: bool,
    pub description: String,
    pub execute_with_studio_run: bool,
    pub is_30_fps_throttle_enabled: bool,
    pub is_physics_environmental_throttled: bool,
    pub is_sleep_allowed: bool,
    pub number_of_players: i32,
    pub simulate_seconds_lag: f64,
    pub timeout: f64,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TextBox {
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
    pub decal: Decal,
    pub offset_studs_u: f32,
    pub offset_studs_v: f32,
    pub studs_per_tile_u: f32,
    pub studs_per_tile_v: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TimerService {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Tool {
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
    pub constraint: Constraint,
    pub relative_to: ActuatorRelativeTo,
    pub torque: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Trail {
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
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub duty: f32,
    pub frequency: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TriangleMeshPart {
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
    pub base_part: BasePart,
    #[propname = "style"]
    pub style: TrussStyle,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Tween {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct TweenService {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIAspectRatioConstraint {
    pub base: Base,
    pub aspect_ratio: f32,
    pub aspect_type: AspectType,
    pub dominant_axis: DominantAxis,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UICorner {
    pub base: Base,
    pub corner_radius: UDim,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIGradient {
    pub base: Base,
    pub enabled: bool,
    pub color: ColorSequence,
    pub offset: Vector2,
    pub rotation: f32,
    pub transparency: NumberSequence,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIGridLayout {
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub cell_padding: UDim2,
    pub cell_size: UDim2,
    pub fill_direction_max_cells: i32,
    pub start_corner: StartCorner,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIGridStyleLayout {
    pub base: Base,
    pub fill_direction: FillDirection,
    pub horizontal_alignment: HorizontalAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub sort_order: SortOrder,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIListLayout {
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub padding: UDim,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIPadding {
    pub base: Base,
    pub padding_top: UDim,
    pub padding_bottom: UDim,
    pub padding_left: UDim,
    pub padding_right: UDim,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIPageLayout {
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
    pub base: Base,
    pub scale: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UISizeConstraint {
    pub base: Base,
    pub max_size: Vector2,
    pub min_size: Vector2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UIStroke {
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
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub fill_empty_space_columns: bool,
    pub fill_empty_space_rows: bool,
    pub major_axis: TableMajorAxis,
    pub padding: UDim2,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UITextSizeConstraint {
    pub base: Base,
    pub min_text_size: i32,
    pub max_text_size: i32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UnionOperation {
    pub part_operation: PartOperation,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct UniversalConstraint {
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub max_angle: f32,
    pub radius: f32,
    pub restitution: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Vector3Value {
    pub base: Base,
    pub value: Vector3,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VectorForce {
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub force: Vector3,
    pub relative_to: ActuatorRelativeTo,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VehicleController {
    pub base: Base,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VehicleSeat {
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
    pub joint_instance: JointInstance,
    pub current_angle: f32,
    pub desired_angle: f32,
    pub hole: InstanceRef,
    pub max_velocity: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct VideoFrame {
    pub gui_object: GuiObject,
    pub looped: bool,
    pub playing: bool,
    pub time_position: f64,
    pub video: String,
    pub volume: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct ViewportFrame {
    pub gui_object: GuiObject,
    pub ambient: Color3,
    pub image_color3: Color3,
    pub image_transparency: f32,
    pub light_color: Color3,
    pub light_direction: Vector3,
    #[propname = "CameraCFrame"]
    pub camera_cframe: CFrame,
    pub camera_field_of_view: f32,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct WedgePart {
    pub base_part: BasePart,
    #[propname = "formFactorRaw"]
    pub form_factor: FormFactor,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Weld {
    pub joint_instance: JointInstance,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct WeldConstraint {
    pub base: Base,
    #[propname = "Part0Internal"]
    pub part_0: InstanceRef,
    #[propname = "Part1Internal"]
    pub part_1: InstanceRef,
    pub state: i32,
    #[propname = "CFrame0"]
    pub cframe: CFrame,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct Workspace {
    pub model: Model,
    pub allow_third_party_sales: bool,
    pub animation_weighted_blend_fix: NewAnimationRuntimeSettings,
    pub client_animator_throttling: ClientAnimatorThrottlingMode,
    pub current_camera: InstanceRef,
    pub distributed_game_time: f64,
    pub fallen_parts_destroy_height: f32,
    pub gravity: f32,
    pub humanoid_only_set_collisions_on_state_change: HumanoidOnlySetCollisionsOnStateChange,
    pub interpolation_throttling: InterpolationThrottlingMode,
    pub mesh_part_heads_and_accessories: MeshPartHeadsAndAccessories,
    pub physics_stepping_method: PhysicsSteppingMethod,
    pub retargeting: AnimatorRetargetingMode,
    pub signal_behavior: SignalBehavior,
    pub stream_out_behavior: StreamOutBehavior,
    pub streaming_enabled: bool,
    pub streaming_min_radius: i32,
    pub streaming_pause_mode: StreamingPauseMode,
    pub streaming_target_radius: i32,
    pub touches_use_collision_groups: bool,
    pub collision_groups: Vec<u8>,
    pub explicit_auto_joints: bool,
    pub terrain_welds_fixed: bool,
}

#[derive(Debug, Clone, Inherits, PropertyConvert)]
pub struct WorldModel {
    pub model: Model,
}
