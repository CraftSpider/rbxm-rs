use rbxm_proc::{Inherits, PropertyConvert};
#[doc = doc_link!("class/Accoutrement")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Accoutrement {
    pub base: crate::model::classes::Base,
    pub attachment_point: crate::model::data::CFrame,
}
#[doc = doc_link!("class/Accessory")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Accessory {
    pub accoutrement: Accoutrement,
    pub accessory_type: super::enums::AccessoryType,
}
#[doc = doc_link!("class/Hat")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Hat {
    pub accoutrement: Accoutrement,
    
}
#[doc = doc_link!("class/AdPortal")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AdPortal {
    pub base: crate::model::classes::Base,
    pub portal_type: super::enums::AdPortalType,
}
#[doc = doc_link!("class/AdService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AdService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AdvancedDragger")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AdvancedDragger {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnalyticsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnalyticsService {
    pub base: crate::model::classes::Base,
    pub api_key: String,
}
#[doc = doc_link!("class/Animation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Animation {
    pub base: crate::model::classes::Base,
    pub animation_id: crate::model::data::Content,
}
#[doc = doc_link!("class/AnimationClip")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationClip {
    pub base: crate::model::classes::Base,
    #[propname = "Loop"] pub loop_seq: bool,
    pub priority: super::enums::AnimationPriority,
}
#[doc = doc_link!("class/CurveAnimation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CurveAnimation {
    pub animation_clip: AnimationClip,
    
}
#[doc = doc_link!("class/KeyframeSequence")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct KeyframeSequence {
    pub animation_clip: AnimationClip,
    pub authored_hip_height: f32,
}
#[doc = doc_link!("class/AnimationClipProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationClipProvider {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnimationController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationController {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnimationFromVideoCreatorService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationFromVideoCreatorService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnimationFromVideoCreatorStudioService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationFromVideoCreatorStudioService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnimationRigData")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationRigData {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnimationStreamTrack")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationStreamTrack {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AnimationTrack")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationTrack {
    pub base: crate::model::classes::Base,
    pub looped: bool,
    pub priority: super::enums::AnimationPriority,
}
#[doc = doc_link!("class/Animator")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Animator {
    pub base: crate::model::classes::Base,
    pub prefer_lod_enabled: bool,
}
#[doc = doc_link!("class/AppUpdateService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AppUpdateService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AssetCounterService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetCounterService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AssetDeliveryProxy")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetDeliveryProxy {
    pub base: crate::model::classes::Base,
    pub interface: String,
    pub port: i32,
    pub start_server: bool,
}
#[doc = doc_link!("class/AssetImportService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetImportService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AssetImportSession")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetImportSession {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AssetManagerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetManagerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AssetPatchSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetPatchSettings {
    pub base: crate::model::classes::Base,
    pub content_id: String,
    pub output_path: String,
    pub patch_id: String,
}
#[doc = doc_link!("class/AssetService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Atmosphere")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Atmosphere {
    pub base: crate::model::classes::Base,
    pub color: crate::model::data::Color3,
    pub decay: crate::model::data::Color3,
    pub density: f32,
    pub glare: f32,
    pub haze: f32,
    pub offset: f32,
}
#[doc = doc_link!("class/Attachment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Attachment {
    pub base: crate::model::classes::Base,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub visible: bool,
}
#[doc = doc_link!("class/Bone")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Bone {
    pub attachment: Attachment,
    
}
#[doc = doc_link!("class/AvatarEditorService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AvatarEditorService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AvatarImportService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AvatarImportService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Backpack")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Backpack {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BackpackItem")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BackpackItem {
    pub base: crate::model::classes::Base,
    pub texture_id: crate::model::data::Content,
}
#[doc = doc_link!("class/HopperBin")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HopperBin {
    pub backpack_item: BackpackItem,
    pub active: bool,
    pub bin_type: super::enums::BinType,
}
#[doc = doc_link!("class/Tool")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Tool {
    pub backpack_item: BackpackItem,
    pub can_be_dropped: bool,
    pub enabled: bool,
    pub grip: crate::model::data::CFrame,
    pub manual_activation_only: bool,
    pub requires_handle: bool,
    pub tool_tip: String,
}
#[doc = doc_link!("class/Flag")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Flag {
    pub tool: Tool,
    pub team_color: crate::model::data::BrickColor,
}
#[doc = doc_link!("class/BadgeService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BadgeService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BasePlayerGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BasePlayerGui {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CoreGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CoreGui {
    pub base_player_gui: BasePlayerGui,
    pub selection_image_object: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/PlayerGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PlayerGui {
    pub base_player_gui: BasePlayerGui,
    pub screen_orientation: super::enums::ScreenOrientation,
    pub selection_image_object: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/StarterGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StarterGui {
    pub base_player_gui: BasePlayerGui,
    pub reset_player_gui_on_spawn: bool,
    pub screen_orientation: super::enums::ScreenOrientation,
    pub show_development_gui: bool,
    pub virtual_cursor_mode: super::enums::VirtualCursorMode,
}
#[doc = doc_link!("class/BaseWrap")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BaseWrap {
    pub base: crate::model::classes::Base,
    pub cage_mesh_id: crate::model::data::Content,
    pub cage_origin: crate::model::data::CFrame,
    pub hsr_asset_id: crate::model::data::Content,
    pub import_origin: crate::model::data::CFrame,
}
#[doc = doc_link!("class/WrapLayer")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WrapLayer {
    pub base_wrap: BaseWrap,
    pub auto_skin: super::enums::WrapLayerAutoSkin,
    pub bind_offset: crate::model::data::CFrame,
    pub enabled: bool,
    pub order: i32,
    pub puffiness: f32,
    pub reference_mesh_id: crate::model::data::Content,
    pub reference_origin: crate::model::data::CFrame,
    pub shrink_factor: f32,
}
#[doc = doc_link!("class/WrapTarget")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WrapTarget {
    pub base_wrap: BaseWrap,
    pub stiffness: f32,
}
#[doc = doc_link!("class/Beam")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Beam {
    pub base: crate::model::classes::Base,
    pub attachment_0: crate::model::data::InstanceRef,
    pub attachment_1: crate::model::data::InstanceRef,
    pub brightness: f32,
    pub color: crate::model::data::ColorSequence,
    pub curve_size_0: f32,
    pub curve_size_1: f32,
    pub enabled: bool,
    pub face_camera: bool,
    pub light_emission: f32,
    pub light_influence: f32,
    pub segments: i32,
    pub texture: crate::model::data::Content,
    pub texture_length: f32,
    pub texture_mode: super::enums::TextureMode,
    pub texture_speed: f32,
    pub transparency: crate::model::data::NumberSequence,
    pub width_0: f32,
    pub width_1: f32,
    pub z_offset: f32,
}
#[doc = doc_link!("class/BindableEvent")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BindableEvent {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BindableFunction")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BindableFunction {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BodyMover")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyMover {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BodyAngularVelocity")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyAngularVelocity {
    pub body_mover: BodyMover,
    pub angular_velocity: crate::model::data::Vector3,
    pub max_torque: crate::model::data::Vector3,
    pub p: f32,
}
#[doc = doc_link!("class/BodyForce")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyForce {
    pub body_mover: BodyMover,
    pub force: crate::model::data::Vector3,
}
#[doc = doc_link!("class/BodyGyro")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyGyro {
    pub body_mover: BodyMover,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub d: f32,
    pub max_torque: crate::model::data::Vector3,
    pub p: f32,
}
#[doc = doc_link!("class/BodyPosition")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyPosition {
    pub body_mover: BodyMover,
    pub d: f32,
    pub max_force: crate::model::data::Vector3,
    pub p: f32,
    pub position: crate::model::data::Vector3,
}
#[doc = doc_link!("class/BodyThrust")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyThrust {
    pub body_mover: BodyMover,
    pub force: crate::model::data::Vector3,
    pub location: crate::model::data::Vector3,
}
#[doc = doc_link!("class/BodyVelocity")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyVelocity {
    pub body_mover: BodyMover,
    pub max_force: crate::model::data::Vector3,
    pub p: f32,
    pub velocity: crate::model::data::Vector3,
}
#[doc = doc_link!("class/RocketPropulsion")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RocketPropulsion {
    pub body_mover: BodyMover,
    pub cartoon_factor: f32,
    pub max_speed: f32,
    pub max_thrust: f32,
    pub max_torque: crate::model::data::Vector3,
    pub target: crate::model::data::InstanceRef,
    pub target_offset: crate::model::data::Vector3,
    pub target_radius: f32,
    pub thrust_d: f32,
    pub thrust_p: f32,
    pub turn_d: f32,
    pub turn_p: f32,
}
#[doc = doc_link!("class/Breakpoint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Breakpoint {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BrowserService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BrowserService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BulkImportService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BulkImportService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CacheableContentProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CacheableContentProvider {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HSRDataContentProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HSRDataContentProvider {
    pub cacheable_content_provider: CacheableContentProvider,
    
}
#[doc = doc_link!("class/MeshContentProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MeshContentProvider {
    pub cacheable_content_provider: CacheableContentProvider,
    
}
#[doc = doc_link!("class/SolidModelContentProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SolidModelContentProvider {
    pub cacheable_content_provider: CacheableContentProvider,
    
}
#[doc = doc_link!("class/CalloutService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CalloutService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Camera")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Camera {
    pub base: crate::model::classes::Base,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub camera_subject: crate::model::data::InstanceRef,
    pub camera_type: super::enums::CameraType,
    pub field_of_view: f32,
    pub field_of_view_mode: super::enums::FieldOfViewMode,
    pub focus: crate::model::data::CFrame,
    pub head_locked: bool,
    pub head_scale: f32,
}
#[doc = doc_link!("class/ChangeHistoryService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ChangeHistoryService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CharacterAppearance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CharacterAppearance {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BodyColors")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BodyColors {
    pub character_appearance: CharacterAppearance,
    pub head_color: crate::model::data::BrickColor,
    pub head_color_3: crate::model::data::Color3,
    pub left_arm_color: crate::model::data::BrickColor,
    pub left_arm_color_3: crate::model::data::Color3,
    pub left_leg_color: crate::model::data::BrickColor,
    pub left_leg_color_3: crate::model::data::Color3,
    pub right_arm_color: crate::model::data::BrickColor,
    pub right_arm_color_3: crate::model::data::Color3,
    pub right_leg_color: crate::model::data::BrickColor,
    pub right_leg_color_3: crate::model::data::Color3,
    pub torso_color: crate::model::data::BrickColor,
    pub torso_color_3: crate::model::data::Color3,
}
#[doc = doc_link!("class/CharacterMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CharacterMesh {
    pub character_appearance: CharacterAppearance,
    pub base_texture_id: i64,
    pub body_part: super::enums::BodyPart,
    pub mesh_id: i64,
    pub overlay_texture_id: i64,
}
#[doc = doc_link!("class/Clothing")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Clothing {
    pub character_appearance: CharacterAppearance,
    pub color_3: crate::model::data::Color3,
}
#[doc = doc_link!("class/Pants")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Pants {
    pub clothing: Clothing,
    pub pants_template: crate::model::data::Content,
}
#[doc = doc_link!("class/Shirt")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Shirt {
    pub clothing: Clothing,
    pub shirt_template: crate::model::data::Content,
}
#[doc = doc_link!("class/ShirtGraphic")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ShirtGraphic {
    pub character_appearance: CharacterAppearance,
    pub color_3: crate::model::data::Color3,
    pub graphic: crate::model::data::Content,
}
#[doc = doc_link!("class/Skin")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Skin {
    pub character_appearance: CharacterAppearance,
    pub skin_color: crate::model::data::BrickColor,
}
#[doc = doc_link!("class/Chat")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Chat {
    pub base: crate::model::classes::Base,
    pub bubble_chat_enabled: bool,
    pub load_default_chat: bool,
}
#[doc = doc_link!("class/ClickDetector")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ClickDetector {
    pub base: crate::model::classes::Base,
    pub cursor_icon: crate::model::data::Content,
    pub max_activation_distance: f32,
}
#[doc = doc_link!("class/Clouds")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Clouds {
    pub base: crate::model::classes::Base,
    pub color: crate::model::data::Color3,
    pub cover: f32,
    pub density: f32,
    pub enabled: bool,
}
#[doc = doc_link!("class/ClusterPacketCache")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ClusterPacketCache {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CollectionService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CollectionService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CommandInstance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CommandInstance {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CommandService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CommandService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Configuration")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Configuration {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ConfigureServerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ConfigureServerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Constraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Constraint {
    pub base: crate::model::classes::Base,
    pub attachment_0: crate::model::data::InstanceRef,
    pub attachment_1: crate::model::data::InstanceRef,
    pub color: crate::model::data::BrickColor,
    pub enabled: bool,
    pub visible: bool,
}
#[doc = doc_link!("class/AlignOrientation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AlignOrientation {
    pub constraint: Constraint,
    pub align_type: super::enums::AlignType,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub max_angular_velocity: f32,
    pub max_torque: f32,
    pub mode: super::enums::OrientationAlignmentMode,
    pub primary_axis_only: bool,
    pub reaction_torque_enabled: bool,
    pub responsiveness: f32,
    pub rigidity_enabled: bool,
}
#[doc = doc_link!("class/AlignPosition")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AlignPosition {
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub max_force: f32,
    pub max_velocity: f32,
    pub mode: super::enums::PositionAlignmentMode,
    pub position: crate::model::data::Vector3,
    pub reaction_force_enabled: bool,
    pub responsiveness: f32,
    pub rigidity_enabled: bool,
}
#[doc = doc_link!("class/AngularVelocity")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AngularVelocity {
    pub constraint: Constraint,
    pub angular_velocity: crate::model::data::Vector3,
    pub max_torque: f32,
    pub reaction_torque_enabled: bool,
    pub relative_to: super::enums::ActuatorRelativeTo,
}
#[doc = doc_link!("class/AnimationConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnimationConstraint {
    pub constraint: Constraint,
    pub max_force: f32,
    pub max_torque: f32,
    pub transform: crate::model::data::CFrame,
}
#[doc = doc_link!("class/BallSocketConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BallSocketConstraint {
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub max_friction_torque: f32,
    pub radius: f32,
    pub restitution: f32,
    pub twist_limits_enabled: bool,
    pub twist_lower_angle: f32,
    pub twist_upper_angle: f32,
    pub upper_angle: f32,
}
#[doc = doc_link!("class/HingeConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HingeConstraint {
    pub constraint: Constraint,
    pub actuator_type: super::enums::ActuatorType,
    pub angular_responsiveness: f32,
    pub angular_speed: f32,
    pub angular_velocity: f32,
    pub limits_enabled: bool,
    pub lower_angle: f32,
    pub motor_max_acceleration: f32,
    pub motor_max_torque: f32,
    pub radius: f32,
    pub restitution: f32,
    pub servo_max_torque: f32,
    pub target_angle: f32,
    pub upper_angle: f32,
}
#[doc = doc_link!("class/LineForce")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LineForce {
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub inverse_square_law: bool,
    pub magnitude: f32,
    pub max_force: f32,
    pub reaction_force_enabled: bool,
}
#[doc = doc_link!("class/LinearVelocity")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LinearVelocity {
    pub constraint: Constraint,
    pub line_direction: crate::model::data::Vector3,
    pub line_velocity: f32,
    pub max_force: f32,
    pub plane_velocity: crate::model::data::Vector2,
    pub primary_tangent_axis: crate::model::data::Vector3,
    pub relative_to: super::enums::ActuatorRelativeTo,
    pub secondary_tangent_axis: crate::model::data::Vector3,
    pub vector_velocity: crate::model::data::Vector3,
    pub velocity_constraint_mode: super::enums::VelocityConstraintMode,
}
#[doc = doc_link!("class/PlaneConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PlaneConstraint {
    pub constraint: Constraint,
    
}
#[doc = doc_link!("class/Plane")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Plane {
    pub plane_constraint: PlaneConstraint,
    
}
#[doc = doc_link!("class/RigidConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RigidConstraint {
    pub constraint: Constraint,
    
}
#[doc = doc_link!("class/RodConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RodConstraint {
    pub constraint: Constraint,
    pub length: f32,
    pub limit_angle_0: f32,
    pub limit_angle_1: f32,
    pub limits_enabled: bool,
    pub thickness: f32,
}
#[doc = doc_link!("class/RopeConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RopeConstraint {
    pub constraint: Constraint,
    pub length: f32,
    pub restitution: f32,
    pub thickness: f32,
    pub winch_enabled: bool,
    pub winch_force: f32,
    pub winch_responsiveness: f32,
    pub winch_speed: f32,
    pub winch_target: f32,
}
#[doc = doc_link!("class/SlidingBallConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SlidingBallConstraint {
    pub constraint: Constraint,
    pub actuator_type: super::enums::ActuatorType,
    pub limits_enabled: bool,
    pub linear_responsiveness: f32,
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
#[doc = doc_link!("class/CylindricalConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CylindricalConstraint {
    pub sliding_ball_constraint: SlidingBallConstraint,
    pub angular_actuator_type: super::enums::ActuatorType,
    pub angular_limits_enabled: bool,
    pub angular_responsiveness: f32,
    pub angular_restitution: f32,
    pub angular_speed: f32,
    pub angular_velocity: f32,
    pub inclination_angle: f32,
    pub lower_angle: f32,
    pub motor_max_angular_acceleration: f32,
    pub motor_max_torque: f32,
    pub rotation_axis_visible: bool,
    pub servo_max_torque: f32,
    pub target_angle: f32,
    pub upper_angle: f32,
}
#[doc = doc_link!("class/PrismaticConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PrismaticConstraint {
    pub sliding_ball_constraint: SlidingBallConstraint,
    
}
#[doc = doc_link!("class/SpringConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SpringConstraint {
    pub constraint: Constraint,
    pub coils: f32,
    pub damping: f32,
    pub free_length: f32,
    pub limits_enabled: bool,
    pub max_force: f32,
    pub max_length: f32,
    pub min_length: f32,
    pub radius: f32,
    pub stiffness: f32,
    pub thickness: f32,
}
#[doc = doc_link!("class/Torque")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Torque {
    pub constraint: Constraint,
    pub relative_to: super::enums::ActuatorRelativeTo,
    pub torque: crate::model::data::Vector3,
}
#[doc = doc_link!("class/TorsionSpringConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TorsionSpringConstraint {
    pub constraint: Constraint,
    pub coils: f32,
    pub damping: f32,
    pub limit_enabled: bool,
    pub limits_enabled: bool,
    pub max_angle: f32,
    pub max_torque: f32,
    pub radius: f32,
    pub restitution: f32,
    pub stiffness: f32,
}
#[doc = doc_link!("class/UniversalConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UniversalConstraint {
    pub constraint: Constraint,
    pub limits_enabled: bool,
    pub max_angle: f32,
    pub radius: f32,
    pub restitution: f32,
}
#[doc = doc_link!("class/VectorForce")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VectorForce {
    pub constraint: Constraint,
    pub apply_at_center_of_mass: bool,
    pub force: crate::model::data::Vector3,
    pub relative_to: super::enums::ActuatorRelativeTo,
}
#[doc = doc_link!("class/ContentProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ContentProvider {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ContextActionService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ContextActionService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Controller")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Controller {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HumanoidController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HumanoidController {
    pub controller: Controller,
    
}
#[doc = doc_link!("class/SkateboardController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SkateboardController {
    pub controller: Controller,
    
}
#[doc = doc_link!("class/VehicleController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VehicleController {
    pub controller: Controller,
    
}
#[doc = doc_link!("class/ControllerBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ControllerBase {
    pub base: crate::model::classes::Base,
    pub move_speed_factor: f32,
    pub rigidity_enabled: bool,
}
#[doc = doc_link!("class/AirController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AirController {
    pub controller_base: ControllerBase,
    pub cancel_air_momentum: bool,
    pub maintain_angular_momentum: bool,
    pub maintain_linear_momentum: bool,
    pub move_max_force: f32,
    pub orientation_max_torque: f32,
    pub orientation_speed_factor: f32,
    pub vector_force: crate::model::data::Vector3,
}
#[doc = doc_link!("class/ClimbController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ClimbController {
    pub controller_base: ControllerBase,
    pub acceleration_time: f32,
    pub move_max_force: f32,
    pub orientation_max_torque: f32,
    pub orientation_speed_factor: f32,
}
#[doc = doc_link!("class/GroundController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GroundController {
    pub controller_base: ControllerBase,
    pub acceleration_lean: f32,
    pub acceleration_time: f32,
    pub align_speed: f32,
    pub align_torque: f32,
    pub deceleration_time: f32,
    pub friction: f32,
    pub friction_weight: f32,
    pub ground_offset: f32,
    pub stand_force: f32,
    pub stand_speed: f32,
    pub turning_factor: f32,
}
#[doc = doc_link!("class/SwimController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SwimController {
    pub controller_base: ControllerBase,
    pub acceleration_time: f32,
    pub pitch_max_torque: f32,
    pub pitch_speed_factor: f32,
    pub roll_max_torque: f32,
    pub roll_speed_factor: f32,
}
#[doc = doc_link!("class/ControllerManager")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ControllerManager {
    pub base: crate::model::classes::Base,
    pub base_move_speed: f32,
    pub base_turn_speed: f32,
    pub facing_direction: crate::model::data::Vector3,
    pub moving_direction: crate::model::data::Vector3,
}
#[doc = doc_link!("class/ControllerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ControllerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CookiesService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CookiesService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CorePackages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CorePackages {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CoreScriptSyncService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CoreScriptSyncService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CrossDMScriptChangeListener")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CrossDMScriptChangeListener {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CustomEvent")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CustomEvent {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CustomEventReceiver")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CustomEventReceiver {
    pub base: crate::model::classes::Base,
    pub source: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/DataModelMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataModelMesh {
    pub base: crate::model::classes::Base,
    pub offset: crate::model::data::Vector3,
    pub scale: crate::model::data::Vector3,
    pub vertex_color: crate::model::data::Vector3,
}
#[doc = doc_link!("class/BevelMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BevelMesh {
    pub data_model_mesh: DataModelMesh,
    
}
#[doc = doc_link!("class/BlockMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BlockMesh {
    pub bevel_mesh: BevelMesh,
    
}
#[doc = doc_link!("class/CylinderMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CylinderMesh {
    pub bevel_mesh: BevelMesh,
    
}
#[doc = doc_link!("class/FileMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FileMesh {
    pub data_model_mesh: DataModelMesh,
    pub mesh_id: crate::model::data::Content,
    pub texture_id: crate::model::data::Content,
}
#[doc = doc_link!("class/SpecialMesh")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SpecialMesh {
    pub file_mesh: FileMesh,
    pub mesh_type: super::enums::MeshType,
}
#[doc = doc_link!("class/DataModelPatchService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataModelPatchService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataModelSession")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataModelSession {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStoreIncrementOptions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreIncrementOptions {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStoreInfo")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreInfo {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStoreKey")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreKey {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStoreKeyInfo")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreKeyInfo {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStoreObjectVersionInfo")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreObjectVersionInfo {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStoreOptions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreOptions {
    pub base: crate::model::classes::Base,
    pub all_scopes: bool,
}
#[doc = doc_link!("class/DataStoreService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreService {
    pub base: crate::model::classes::Base,
    pub legacy_naming_scheme: bool,
}
#[doc = doc_link!("class/DataStoreSetOptions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreSetOptions {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Debris")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Debris {
    pub base: crate::model::classes::Base,
    pub max_items: i32,
}
#[doc = doc_link!("class/DebugSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebugSettings {
    pub base: crate::model::classes::Base,
    pub is_script_stack_tracing_enabled: bool,
    pub report_sound_warnings: bool,
    pub tick_count_precise_override: super::enums::TickCountSampleMethod,
}
#[doc = doc_link!("class/DebuggablePluginWatcher")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggablePluginWatcher {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DebuggerBreakpoint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerBreakpoint {
    pub base: crate::model::classes::Base,
    pub condition: String,
    pub continue_execution: bool,
    pub is_enabled: bool,
    pub log_expression: String,
    pub is_context_dependent_breakpoint: bool,
}
#[doc = doc_link!("class/DebuggerConnection")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerConnection {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LocalDebuggerConnection")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LocalDebuggerConnection {
    pub debugger_connection: DebuggerConnection,
    
}
#[doc = doc_link!("class/DebuggerConnectionManager")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerConnectionManager {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DebuggerLuaResponse")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerLuaResponse {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DebuggerManager")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerManager {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DebuggerUIService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerUIService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DebuggerVariable")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerVariable {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DebuggerWatch")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DebuggerWatch {
    pub base: crate::model::classes::Base,
    pub expression: String,
}
#[doc = doc_link!("class/DeviceIdService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DeviceIdService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Dialog")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Dialog {
    pub base: crate::model::classes::Base,
    pub behavior_type: super::enums::DialogBehaviorType,
    pub conversation_distance: f32,
    pub goodbye_choice_active: bool,
    pub goodbye_dialog: String,
    pub in_use: bool,
    pub initial_prompt: String,
    pub purpose: super::enums::DialogPurpose,
    pub tone: super::enums::DialogTone,
    pub trigger_distance: f32,
    pub trigger_offset: crate::model::data::Vector3,
}
#[doc = doc_link!("class/DialogChoice")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DialogChoice {
    pub base: crate::model::classes::Base,
    pub goodbye_choice_active: bool,
    pub goodbye_dialog: String,
    pub response_dialog: String,
    pub user_dialog: String,
}
#[doc = doc_link!("class/DraftsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DraftsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Dragger")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Dragger {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DraggerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DraggerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DynamicTextureAlpha")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DynamicTextureAlpha {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DynamicTextureLayerAlpha")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DynamicTextureLayerAlpha {
    pub base: crate::model::classes::Base,
    pub z_index: i32,
}
#[doc = doc_link!("class/EulerRotationCurve")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct EulerRotationCurve {
    pub base: crate::model::classes::Base,
    pub rotation_order: super::enums::RotationOrder,
}
#[doc = doc_link!("class/EventIngestService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct EventIngestService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ExperienceAuthService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ExperienceAuthService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ExperienceInviteOptions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ExperienceInviteOptions {
    pub base: crate::model::classes::Base,
    pub invite_message_id: String,
    pub invite_user: i64,
    pub launch_data: String,
    pub prompt_message: String,
}
#[doc = doc_link!("class/Explosion")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Explosion {
    pub base: crate::model::classes::Base,
    pub blast_pressure: f32,
    pub blast_radius: f32,
    pub destroy_joint_radius_percent: f32,
    pub explosion_type: super::enums::ExplosionType,
    pub position: crate::model::data::Vector3,
    pub time_scale: f32,
    pub visible: bool,
}
#[doc = doc_link!("class/FaceAnimatorService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FaceAnimatorService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/FaceControls")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FaceControls {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/FaceInstance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FaceInstance {
    pub base: crate::model::classes::Base,
    pub face: super::enums::NormalId,
}
#[doc = doc_link!("class/Decal")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Decal {
    pub face_instance: FaceInstance,
    pub color_3: crate::model::data::Color3,
    pub texture: crate::model::data::Content,
    pub transparency: f32,
    pub z_index: i32,
}
#[doc = doc_link!("class/Texture")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Texture {
    pub decal: Decal,
    pub offset_studs_u: f32,
    pub offset_studs_v: f32,
    pub studs_per_tile_u: f32,
    pub studs_per_tile_v: f32,
}
#[doc = doc_link!("class/FacialAnimationRecordingService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FacialAnimationRecordingService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/FacialAnimationStreamingService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FacialAnimationStreamingService {
    pub base: crate::model::classes::Base,
    pub enable_flags: super::enums::FacialAnimationFlags,
    pub enabled: bool,
}
#[doc = doc_link!("class/FacialAnimationStreamingServiceV2")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FacialAnimationStreamingServiceV2 {
    pub base: crate::model::classes::Base,
    pub service_state: i32,
}
#[doc = doc_link!("class/Feature")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Feature {
    pub base: crate::model::classes::Base,
    pub face_id: super::enums::NormalId,
    pub in_out: super::enums::InOut,
    pub left_right: super::enums::LeftRight,
    pub top_bottom: super::enums::TopBottom,
}
#[doc = doc_link!("class/Hole")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Hole {
    pub feature: Feature,
    
}
#[doc = doc_link!("class/MotorFeature")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MotorFeature {
    pub feature: Feature,
    
}
#[doc = doc_link!("class/File")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct File {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Fire")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Fire {
    pub base: crate::model::classes::Base,
    pub color: crate::model::data::Color3,
    pub enabled: bool,
    pub secondary_color: crate::model::data::Color3,
    pub time_scale: f32,
}
#[doc = doc_link!("class/FlagStandService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FlagStandService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/FloatCurve")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FloatCurve {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/FlyweightService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FlyweightService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CSGDictionaryService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CSGDictionaryService {
    pub flyweight_service: FlyweightService,
    
}
#[doc = doc_link!("class/NonReplicatedCSGDictionaryService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NonReplicatedCSGDictionaryService {
    pub flyweight_service: FlyweightService,
    
}
#[doc = doc_link!("class/Folder")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Folder {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ForceField")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ForceField {
    pub base: crate::model::classes::Base,
    pub visible: bool,
}
#[doc = doc_link!("class/FriendService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FriendService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/FunctionalTest")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FunctionalTest {
    pub base: crate::model::classes::Base,
    pub description: String,
}
#[doc = doc_link!("class/GamePassService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GamePassService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/GameSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GameSettings {
    pub base: crate::model::classes::Base,
    pub video_capture_enabled: bool,
}
#[doc = doc_link!("class/GamepadService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GamepadService {
    pub base: crate::model::classes::Base,
    pub gamepad_cursor_enabled: bool,
}
#[doc = doc_link!("class/Geometry")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Geometry {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/GetTextBoundsParams")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GetTextBoundsParams {
    pub base: crate::model::classes::Base,
    pub font: crate::model::data::Font,
    pub size: f32,
    pub text: String,
    pub width: f32,
}
#[doc = doc_link!("class/GlobalDataStore")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GlobalDataStore {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataStore")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStore {
    pub global_data_store: GlobalDataStore,
    
}
#[doc = doc_link!("class/OrderedDataStore")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct OrderedDataStore {
    pub global_data_store: GlobalDataStore,
    
}
#[doc = doc_link!("class/GoogleAnalyticsConfiguration")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GoogleAnalyticsConfiguration {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/GroupService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GroupService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/GuiBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiBase {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/GuiBase2d")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiBase2d {
    pub gui_base: GuiBase,
    pub auto_localize: bool,
    pub root_localization_table: crate::model::data::InstanceRef,
    pub selection_behavior_down: super::enums::SelectionBehavior,
    pub selection_behavior_left: super::enums::SelectionBehavior,
    pub selection_behavior_right: super::enums::SelectionBehavior,
    pub selection_behavior_up: super::enums::SelectionBehavior,
    pub selection_group: bool,
}
#[doc = doc_link!("class/GuiObject")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiObject {
    pub gui_base_2_d: GuiBase2d,
    pub active: bool,
    pub anchor_point: crate::model::data::Vector2,
    pub automatic_size: super::enums::AutomaticSize,
    pub background_color_3: crate::model::data::Color3,
    pub background_transparency: f32,
    pub border_color_3: crate::model::data::Color3,
    pub border_mode: super::enums::BorderMode,
    pub border_size_pixel: i32,
    pub clips_descendants: bool,
    pub draggable: bool,
    pub layout_order: i32,
    pub next_selection_down: crate::model::data::InstanceRef,
    pub next_selection_left: crate::model::data::InstanceRef,
    pub next_selection_right: crate::model::data::InstanceRef,
    pub next_selection_up: crate::model::data::InstanceRef,
    pub position: crate::model::data::UDim2,
    pub rotation: f32,
    pub selectable: bool,
    pub selection_image_object: crate::model::data::InstanceRef,
    pub selection_order: i32,
    pub size: crate::model::data::UDim2,
    pub size_constraint: super::enums::SizeConstraint,
    pub visible: bool,
    pub z_index: i32,
}
#[doc = doc_link!("class/CanvasGroup")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CanvasGroup {
    pub gui_object: GuiObject,
    pub group_color_3: crate::model::data::Color3,
    pub group_transparency: f32,
}
#[doc = doc_link!("class/Frame")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Frame {
    pub gui_object: GuiObject,
    pub style: super::enums::FrameStyle,
}
#[doc = doc_link!("class/GuiButton")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiButton {
    pub gui_object: GuiObject,
    pub auto_button_color: bool,
    pub modal: bool,
    pub selected: bool,
    pub style: super::enums::ButtonStyle,
}
#[doc = doc_link!("class/ImageButton")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImageButton {
    pub gui_button: GuiButton,
    pub hover_image: crate::model::data::Content,
    pub image: crate::model::data::Content,
    pub image_color_3: crate::model::data::Color3,
    pub image_rect_offset: crate::model::data::Vector2,
    pub image_rect_size: crate::model::data::Vector2,
    pub image_transparency: f32,
    pub pressed_image: crate::model::data::Content,
    pub resample_mode: super::enums::ResamplerMode,
    pub scale_type: super::enums::ScaleType,
    pub slice_center: crate::model::data::Rect,
    pub slice_scale: f32,
    pub tile_size: crate::model::data::UDim2,
}
#[doc = doc_link!("class/TextButton")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextButton {
    pub gui_button: GuiButton,
    pub font: super::enums::Font,
    pub font_face: crate::model::data::Font,
    pub line_height: f32,
    pub max_visible_graphemes: i32,
    pub rich_text: bool,
    pub text: String,
    pub text_color_3: crate::model::data::Color3,
    pub text_scaled: bool,
    pub text_size: f32,
    pub text_stroke_color_3: crate::model::data::Color3,
    pub text_stroke_transparency: f32,
    pub text_transparency: f32,
    pub text_truncate: super::enums::TextTruncate,
    pub text_wrapped: bool,
    pub text_x_alignment: super::enums::TextXAlignment,
    pub text_y_alignment: super::enums::TextYAlignment,
}
#[doc = doc_link!("class/GuiLabel")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiLabel {
    pub gui_object: GuiObject,
    
}
#[doc = doc_link!("class/ImageLabel")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImageLabel {
    pub gui_label: GuiLabel,
    pub image: crate::model::data::Content,
    pub image_color_3: crate::model::data::Color3,
    pub image_rect_offset: crate::model::data::Vector2,
    pub image_rect_size: crate::model::data::Vector2,
    pub image_transparency: f32,
    pub resample_mode: super::enums::ResamplerMode,
    pub scale_type: super::enums::ScaleType,
    pub slice_center: crate::model::data::Rect,
    pub slice_scale: f32,
    pub tile_size: crate::model::data::UDim2,
}
#[doc = doc_link!("class/TextLabel")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextLabel {
    pub gui_label: GuiLabel,
    pub font: super::enums::Font,
    pub font_face: crate::model::data::Font,
    pub line_height: f32,
    pub max_visible_graphemes: i32,
    pub rich_text: bool,
    pub text: String,
    pub text_color_3: crate::model::data::Color3,
    pub text_scaled: bool,
    pub text_size: f32,
    pub text_stroke_color_3: crate::model::data::Color3,
    pub text_stroke_transparency: f32,
    pub text_transparency: f32,
    pub text_truncate: super::enums::TextTruncate,
    pub text_wrapped: bool,
    pub text_x_alignment: super::enums::TextXAlignment,
    pub text_y_alignment: super::enums::TextYAlignment,
}
#[doc = doc_link!("class/ScrollingFrame")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScrollingFrame {
    pub gui_object: GuiObject,
    pub automatic_canvas_size: super::enums::AutomaticSize,
    pub bottom_image: crate::model::data::Content,
    pub canvas_position: crate::model::data::Vector2,
    pub canvas_size: crate::model::data::UDim2,
    pub elastic_behavior: super::enums::ElasticBehavior,
    pub horizontal_scroll_bar_inset: super::enums::ScrollBarInset,
    pub mid_image: crate::model::data::Content,
    pub scroll_bar_image_color_3: crate::model::data::Color3,
    pub scroll_bar_image_transparency: f32,
    pub scroll_bar_thickness: i32,
    pub scrolling_direction: super::enums::ScrollingDirection,
    pub scrolling_enabled: bool,
    pub top_image: crate::model::data::Content,
    pub vertical_scroll_bar_inset: super::enums::ScrollBarInset,
    pub vertical_scroll_bar_position: super::enums::VerticalScrollBarPosition,
}
#[doc = doc_link!("class/TextBox")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextBox {
    pub gui_object: GuiObject,
    pub clear_text_on_focus: bool,
    pub cursor_position: i32,
    pub font: super::enums::Font,
    pub font_face: crate::model::data::Font,
    pub line_height: f32,
    pub max_visible_graphemes: i32,
    pub multi_line: bool,
    pub placeholder_color_3: crate::model::data::Color3,
    pub placeholder_text: String,
    pub rich_text: bool,
    pub selection_start: i32,
    pub show_native_input: bool,
    pub text: String,
    pub text_color_3: crate::model::data::Color3,
    pub text_editable: bool,
    pub text_scaled: bool,
    pub text_size: f32,
    pub text_stroke_color_3: crate::model::data::Color3,
    pub text_stroke_transparency: f32,
    pub text_transparency: f32,
    pub text_truncate: super::enums::TextTruncate,
    pub text_wrapped: bool,
    pub text_x_alignment: super::enums::TextXAlignment,
    pub text_y_alignment: super::enums::TextYAlignment,
}
#[doc = doc_link!("class/VideoFrame")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VideoFrame {
    pub gui_object: GuiObject,
    pub looped: bool,
    pub video: crate::model::data::Content,
    pub volume: f32,
}
#[doc = doc_link!("class/ViewportFrame")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ViewportFrame {
    pub gui_object: GuiObject,
    pub ambient: crate::model::data::Color3,
    pub image_color_3: crate::model::data::Color3,
    pub image_transparency: f32,
    pub light_color: crate::model::data::Color3,
    pub light_direction: crate::model::data::Vector3,
}
#[doc = doc_link!("class/LayerCollector")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LayerCollector {
    pub gui_base_2_d: GuiBase2d,
    pub enabled: bool,
    pub reset_on_spawn: bool,
    pub z_index_behavior: super::enums::ZIndexBehavior,
}
#[doc = doc_link!("class/BillboardGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BillboardGui {
    pub layer_collector: LayerCollector,
    pub active: bool,
    pub adornee: crate::model::data::InstanceRef,
    pub always_on_top: bool,
    pub brightness: f32,
    pub clips_descendants: bool,
    pub distance_lower_limit: f32,
    pub distance_step: f32,
    pub distance_upper_limit: f32,
    pub extents_offset: crate::model::data::Vector3,
    pub extents_offset_world_space: crate::model::data::Vector3,
    pub light_influence: f32,
    pub max_distance: f32,
    pub player_to_hide_from: crate::model::data::InstanceRef,
    pub size: crate::model::data::UDim2,
    pub size_offset: crate::model::data::Vector2,
    pub studs_offset: crate::model::data::Vector3,
    pub studs_offset_world_space: crate::model::data::Vector3,
}
#[doc = doc_link!("class/PluginGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginGui {
    pub layer_collector: LayerCollector,
    pub title: String,
}
#[doc = doc_link!("class/DockWidgetPluginGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DockWidgetPluginGui {
    pub plugin_gui: PluginGui,
    
}
#[doc = doc_link!("class/QWidgetPluginGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct QWidgetPluginGui {
    pub plugin_gui: PluginGui,
    
}
#[doc = doc_link!("class/ScreenGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScreenGui {
    pub layer_collector: LayerCollector,
    pub clip_to_device_safe_area: bool,
    pub display_order: i32,
    pub ignore_gui_inset: bool,
    pub on_top_of_core_blur: bool,
    pub safe_area_compatibility: super::enums::SafeAreaCompatibility,
    pub screen_insets: super::enums::ScreenInsets,
}
#[doc = doc_link!("class/GuiMain")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiMain {
    pub screen_gui: ScreenGui,
    
}
#[doc = doc_link!("class/SurfaceGuiBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SurfaceGuiBase {
    pub layer_collector: LayerCollector,
    pub active: bool,
    pub adornee: crate::model::data::InstanceRef,
    pub face: super::enums::NormalId,
}
#[doc = doc_link!("class/AdGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AdGui {
    pub surface_gui_base: SurfaceGuiBase,
    pub ad_shape: super::enums::AdShape,
}
#[doc = doc_link!("class/SurfaceGui")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SurfaceGui {
    pub surface_gui_base: SurfaceGuiBase,
    pub always_on_top: bool,
    pub brightness: f32,
    pub canvas_size: crate::model::data::Vector2,
    pub clips_descendants: bool,
    pub light_influence: f32,
    pub pixels_per_stud: f32,
    pub sizing_mode: super::enums::SurfaceGuiSizingMode,
    pub tool_punch_through_distance: f32,
    pub z_offset: f32,
}
#[doc = doc_link!("class/GuiBase3d")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiBase3d {
    pub gui_base: GuiBase,
    pub color_3: crate::model::data::Color3,
    pub transparency: f32,
    pub visible: bool,
}
#[doc = doc_link!("class/FloorWire")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FloorWire {
    pub gui_base_3_d: GuiBase3d,
    pub cycle_offset: f32,
    pub from: crate::model::data::InstanceRef,
    pub studs_between_textures: f32,
    pub texture: crate::model::data::Content,
    pub texture_size: crate::model::data::Vector2,
    pub to: crate::model::data::InstanceRef,
    pub velocity: f32,
    pub wire_radius: f32,
}
#[doc = doc_link!("class/InstanceAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct InstanceAdornment {
    pub gui_base_3_d: GuiBase3d,
    pub adornee: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/SelectionBox")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SelectionBox {
    pub instance_adornment: InstanceAdornment,
    pub line_thickness: f32,
    pub surface_color_3: crate::model::data::Color3,
    pub surface_transparency: f32,
}
#[doc = doc_link!("class/PVAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PVAdornment {
    pub gui_base_3_d: GuiBase3d,
    pub adornee: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/HandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HandleAdornment {
    pub pv_adornment: PVAdornment,
    pub adorn_culling_mode: super::enums::AdornCullingMode,
    pub always_on_top: bool,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub size_relative_offset: crate::model::data::Vector3,
    pub z_index: i32,
}
#[doc = doc_link!("class/BoxHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BoxHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub size: crate::model::data::Vector3,
}
#[doc = doc_link!("class/ConeHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ConeHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub height: f32,
    pub radius: f32,
}
#[doc = doc_link!("class/CylinderHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CylinderHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub angle: f32,
    pub height: f32,
    pub inner_radius: f32,
    pub radius: f32,
}
#[doc = doc_link!("class/ImageHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImageHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub image: crate::model::data::Content,
    pub size: crate::model::data::Vector2,
}
#[doc = doc_link!("class/LineHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LineHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub length: f32,
    pub thickness: f32,
}
#[doc = doc_link!("class/SphereHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SphereHandleAdornment {
    pub handle_adornment: HandleAdornment,
    pub radius: f32,
}
#[doc = doc_link!("class/WireframeHandleAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WireframeHandleAdornment {
    pub handle_adornment: HandleAdornment,
    
}
#[doc = doc_link!("class/ParabolaAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ParabolaAdornment {
    pub pv_adornment: PVAdornment,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub range: f32,
    pub thickness: f32,
}
#[doc = doc_link!("class/SelectionSphere")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SelectionSphere {
    pub pv_adornment: PVAdornment,
    pub surface_color_3: crate::model::data::Color3,
    pub surface_transparency: f32,
}
#[doc = doc_link!("class/PartAdornment")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PartAdornment {
    pub gui_base_3_d: GuiBase3d,
    pub adornee: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/HandlesBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HandlesBase {
    pub part_adornment: PartAdornment,
    
}
#[doc = doc_link!("class/ArcHandles")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ArcHandles {
    pub handles_base: HandlesBase,
    pub axes: crate::model::data::Axes,
}
#[doc = doc_link!("class/Handles")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Handles {
    pub handles_base: HandlesBase,
    pub faces: crate::model::data::Faces,
    pub style: super::enums::HandlesStyle,
}
#[doc = doc_link!("class/SurfaceSelection")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SurfaceSelection {
    pub part_adornment: PartAdornment,
    pub target_surface: super::enums::NormalId,
}
#[doc = doc_link!("class/SelectionLasso")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SelectionLasso {
    pub gui_base_3_d: GuiBase3d,
    pub humanoid: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/SelectionPartLasso")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SelectionPartLasso {
    pub selection_lasso: SelectionLasso,
    pub part: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/SelectionPointLasso")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SelectionPointLasso {
    pub selection_lasso: SelectionLasso,
    pub point: crate::model::data::Vector3,
}
#[doc = doc_link!("class/GuiService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuiService {
    pub base: crate::model::classes::Base,
    pub auto_select_gui_enabled: bool,
    pub gui_navigation_enabled: bool,
    pub selected_object: crate::model::data::InstanceRef,
    pub touch_controls_enabled: bool,
}
#[doc = doc_link!("class/GuidRegistryService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GuidRegistryService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HapticService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HapticService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HeightmapImporterService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HeightmapImporterService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HiddenSurfaceRemovalAsset")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HiddenSurfaceRemovalAsset {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Highlight")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Highlight {
    pub base: crate::model::classes::Base,
    pub adornee: crate::model::data::InstanceRef,
    pub depth_mode: super::enums::HighlightDepthMode,
    pub enabled: bool,
    pub fill_color: crate::model::data::Color3,
    pub fill_transparency: f32,
    pub line_thickness: i32,
    pub outline_color: crate::model::data::Color3,
    pub outline_transparency: f32,
    pub reserved_id: super::enums::ReservedHighlightId,
}
#[doc = doc_link!("class/Hopper")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Hopper {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HttpRbxApiService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HttpRbxApiService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HttpRequest")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HttpRequest {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/HttpService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HttpService {
    pub base: crate::model::classes::Base,
    pub http_enabled: bool,
}
#[doc = doc_link!("class/Humanoid")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Humanoid {
    pub base: crate::model::classes::Base,
    pub auto_jump_enabled: bool,
    pub auto_rotate: bool,
    pub automatic_scaling_enabled: bool,
    pub break_joints_on_death: bool,
    pub camera_offset: crate::model::data::Vector3,
    pub collision_type: super::enums::HumanoidCollisionType,
    pub display_distance_type: super::enums::HumanoidDisplayDistanceType,
    pub display_name: String,
    pub evaluate_state_machine: bool,
    pub health_display_distance: f32,
    pub health_display_type: super::enums::HumanoidHealthDisplayType,
    pub hip_height: f32,
    pub jump_height: f32,
    pub jump_power: f32,
    pub max_health: f32,
    pub max_slope_angle: f32,
    pub name_display_distance: f32,
    pub name_occlusion: super::enums::NameOcclusion,
    pub platform_stand: bool,
    pub requires_neck: bool,
    pub rig_type: super::enums::HumanoidRigType,
    pub sit: bool,
    pub target_point: crate::model::data::Vector3,
    pub use_jump_power: bool,
    pub walk_speed: f32,
    pub walk_to_part: crate::model::data::InstanceRef,
    pub walk_to_point: crate::model::data::Vector3,
}
#[doc = doc_link!("class/HumanoidDescription")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct HumanoidDescription {
    pub base: crate::model::classes::Base,
    pub accessory_blob: String,
    pub back_accessory: String,
    pub body_type_scale: f32,
    pub climb_animation: i64,
    pub depth_scale: f32,
    pub face: i64,
    pub face_accessory: String,
    pub fall_animation: i64,
    pub front_accessory: String,
    pub graphic_t_shirt: i64,
    pub hair_accessory: String,
    pub hat_accessory: String,
    pub head: i64,
    pub head_color: crate::model::data::Color3,
    pub head_scale: f32,
    pub height_scale: f32,
    pub idle_animation: i64,
    pub jump_animation: i64,
    pub left_arm: i64,
    pub left_arm_color: crate::model::data::Color3,
    pub left_leg: i64,
    pub left_leg_color: crate::model::data::Color3,
    pub mood_animation: i64,
    pub neck_accessory: String,
    pub number_emotes_loaded: i32,
    pub pants: i64,
    pub proportion_scale: f32,
    pub right_arm: i64,
    pub right_arm_color: crate::model::data::Color3,
    pub right_leg: i64,
    pub right_leg_color: crate::model::data::Color3,
    pub run_animation: i64,
    pub shirt: i64,
    pub shoulders_accessory: String,
    pub swim_animation: i64,
    pub torso: i64,
    pub torso_color: crate::model::data::Color3,
    pub waist_accessory: String,
    pub walk_animation: i64,
    pub width_scale: f32,
}
#[doc = doc_link!("class/IKControl")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct IKControl {
    pub base: crate::model::classes::Base,
    pub chain_root: crate::model::data::InstanceRef,
    pub enabled: bool,
    pub end_effector: crate::model::data::InstanceRef,
    pub end_effector_offset: crate::model::data::CFrame,
    pub offset: crate::model::data::CFrame,
    pub pole: crate::model::data::InstanceRef,
    pub priority: i32,
    pub target: crate::model::data::InstanceRef,
    #[propname = "Type"] pub ty: super::enums::IKControlType,
    pub weight: f32,
}
#[doc = doc_link!("class/ILegacyStudioBridge")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ILegacyStudioBridge {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LegacyStudioBridge")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LegacyStudioBridge {
    pub i_legacy_studio_bridge: ILegacyStudioBridge,
    
}
#[doc = doc_link!("class/IXPService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct IXPService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ImporterBaseSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterBaseSettings {
    pub base: crate::model::classes::Base,
    pub import_name: String,
    pub should_import: bool,
}
#[doc = doc_link!("class/ImporterAnimationSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterAnimationSettings {
    pub importer_base_settings: ImporterBaseSettings,
    
}
#[doc = doc_link!("class/ImporterFacsSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterFacsSettings {
    pub importer_base_settings: ImporterBaseSettings,
    
}
#[doc = doc_link!("class/ImporterGroupSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterGroupSettings {
    pub importer_base_settings: ImporterBaseSettings,
    pub anchored: bool,
    pub import_as_model_asset: bool,
    pub insert_in_workspace: bool,
}
#[doc = doc_link!("class/ImporterJointSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterJointSettings {
    pub importer_base_settings: ImporterBaseSettings,
    
}
#[doc = doc_link!("class/ImporterMaterialSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterMaterialSettings {
    pub importer_base_settings: ImporterBaseSettings,
    pub diffuse_file_path: String,
    pub metalness_file_path: String,
    pub normal_file_path: String,
    pub roughness_file_path: String,
}
#[doc = doc_link!("class/ImporterMeshSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterMeshSettings {
    pub importer_base_settings: ImporterBaseSettings,
    pub anchored: bool,
    pub cage_mesh_intersected_preview: bool,
    pub cage_non_manifold_preview: bool,
    pub cage_overlapping_vertices_preview: bool,
    pub cage_uv_mis_matched_preview: bool,
    pub double_sided: bool,
    pub ignore_vertex_colors: bool,
    pub irrelevant_cage_modified_preview: bool,
    pub mesh_hole_detected_preview: bool,
    pub outer_cage_far_extended_from_mesh_preview: bool,
    pub use_imported_pivot: bool,
}
#[doc = doc_link!("class/ImporterRootSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ImporterRootSettings {
    pub importer_base_settings: ImporterBaseSettings,
    pub add_model_to_inventory: bool,
    pub anchored: bool,
    pub import_as_model_asset: bool,
    pub insert_in_workspace: bool,
    pub insert_with_scene_position: bool,
    pub invert_negative_faces: bool,
    pub merge_meshes: bool,
    pub rig_type: super::enums::RigType,
    pub scale_unit: super::enums::MeshScaleUnit,
    pub use_scene_origin_as_pivot: bool,
    pub world_forward: super::enums::NormalId,
    pub world_up: super::enums::NormalId,
}
#[doc = doc_link!("class/IncrementalPatchBuilder")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct IncrementalPatchBuilder {
    pub base: crate::model::classes::Base,
    pub add_paths_to_bundle: bool,
    pub high_compression: bool,
    pub zstd_compression: bool,
}
#[doc = doc_link!("class/InputObject")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct InputObject {
    pub base: crate::model::classes::Base,
    pub delta: crate::model::data::Vector3,
    pub key_code: super::enums::KeyCode,
    pub position: crate::model::data::Vector3,
    pub user_input_state: super::enums::UserInputState,
    pub user_input_type: super::enums::UserInputType,
}
#[doc = doc_link!("class/InsertService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct InsertService {
    pub base: crate::model::classes::Base,
    pub allow_client_insert_models: bool,
}
#[doc = doc_link!("class/JointInstance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct JointInstance {
    pub base: crate::model::classes::Base,
    pub c_0: crate::model::data::CFrame,
    pub c_1: crate::model::data::CFrame,
    pub enabled: bool,
    pub part_0: crate::model::data::InstanceRef,
    pub part_1: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/DynamicRotate")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DynamicRotate {
    pub joint_instance: JointInstance,
    pub base_angle: f32,
}
#[doc = doc_link!("class/RotateP")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RotateP {
    pub dynamic_rotate: DynamicRotate,
    
}
#[doc = doc_link!("class/RotateV")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RotateV {
    pub dynamic_rotate: DynamicRotate,
    
}
#[doc = doc_link!("class/Glue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Glue {
    pub joint_instance: JointInstance,
    pub f_0: crate::model::data::Vector3,
    pub f_1: crate::model::data::Vector3,
    pub f_2: crate::model::data::Vector3,
    pub f_3: crate::model::data::Vector3,
}
#[doc = doc_link!("class/ManualSurfaceJointInstance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ManualSurfaceJointInstance {
    pub joint_instance: JointInstance,
    
}
#[doc = doc_link!("class/ManualGlue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ManualGlue {
    pub manual_surface_joint_instance: ManualSurfaceJointInstance,
    
}
#[doc = doc_link!("class/ManualWeld")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ManualWeld {
    pub manual_surface_joint_instance: ManualSurfaceJointInstance,
    
}
#[doc = doc_link!("class/Motor")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Motor {
    pub joint_instance: JointInstance,
    pub desired_angle: f32,
    pub max_velocity: f32,
}
#[doc = doc_link!("class/Motor6D")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Motor6D {
    pub motor: Motor,
    
}
#[doc = doc_link!("class/Rotate")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Rotate {
    pub joint_instance: JointInstance,
    
}
#[doc = doc_link!("class/Snap")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Snap {
    pub joint_instance: JointInstance,
    
}
#[doc = doc_link!("class/VelocityMotor")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VelocityMotor {
    pub joint_instance: JointInstance,
    pub current_angle: f32,
    pub desired_angle: f32,
    pub hole: crate::model::data::InstanceRef,
    pub max_velocity: f32,
}
#[doc = doc_link!("class/Weld")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Weld {
    pub joint_instance: JointInstance,
    
}
#[doc = doc_link!("class/JointsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct JointsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/KeyboardService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct KeyboardService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Keyframe")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Keyframe {
    pub base: crate::model::classes::Base,
    pub time: f32,
}
#[doc = doc_link!("class/KeyframeMarker")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct KeyframeMarker {
    pub base: crate::model::classes::Base,
    pub value: String,
}
#[doc = doc_link!("class/KeyframeSequenceProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct KeyframeSequenceProvider {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LSPFileSyncService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LSPFileSyncService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LanguageService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LanguageService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Light")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Light {
    pub base: crate::model::classes::Base,
    pub brightness: f32,
    pub color: crate::model::data::Color3,
    pub enabled: bool,
    pub shadows: bool,
}
#[doc = doc_link!("class/PointLight")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PointLight {
    pub light: Light,
    pub range: f32,
}
#[doc = doc_link!("class/SpotLight")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SpotLight {
    pub light: Light,
    pub angle: f32,
    pub face: super::enums::NormalId,
    pub range: f32,
}
#[doc = doc_link!("class/SurfaceLight")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SurfaceLight {
    pub light: Light,
    pub angle: f32,
    pub face: super::enums::NormalId,
    pub range: f32,
}
#[doc = doc_link!("class/Lighting")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Lighting {
    pub base: crate::model::classes::Base,
    pub ambient: crate::model::data::Color3,
    pub brightness: f32,
    pub environment_diffuse_scale: f32,
    pub environment_specular_scale: f32,
    pub exposure_compensation: f32,
    pub fog_color: crate::model::data::Color3,
    pub fog_end: f32,
    pub fog_start: f32,
    pub geographic_latitude: f32,
    pub global_shadows: bool,
    pub outdoor_ambient: crate::model::data::Color3,
    pub outlines: bool,
    pub shadow_softness: f32,
    pub technology: super::enums::Technology,
    pub time_of_day: String,
}
#[doc = doc_link!("class/LocalStorageService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LocalStorageService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/AppStorageService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AppStorageService {
    pub local_storage_service: LocalStorageService,
    
}
#[doc = doc_link!("class/UserStorageService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UserStorageService {
    pub local_storage_service: LocalStorageService,
    
}
#[doc = doc_link!("class/LocalizationService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LocalizationService {
    pub base: crate::model::classes::Base,
    pub is_text_scraper_running: bool,
}
#[doc = doc_link!("class/LocalizationTable")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LocalizationTable {
    pub base: crate::model::classes::Base,
    pub source_locale_id: String,
}
#[doc = doc_link!("class/CloudLocalizationTable")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CloudLocalizationTable {
    pub localization_table: LocalizationTable,
    
}
#[doc = doc_link!("class/LodDataEntity")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LodDataEntity {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LodDataService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LodDataService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LogService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LogService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LoginService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LoginService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LuaSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LuaSettings {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LuaSourceContainer")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LuaSourceContainer {
    pub base: crate::model::classes::Base,
    pub current_editor: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/BaseScript")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BaseScript {
    pub lua_source_container: LuaSourceContainer,
    pub disabled: bool,
    pub linked_source: crate::model::data::Content,
    pub run_context: super::enums::RunContext,
}
#[doc = doc_link!("class/CoreScript")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CoreScript {
    pub base_script: BaseScript,
    
}
#[doc = doc_link!("class/Script")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Script {
    pub base_script: BaseScript,
    pub source: String,
}
#[doc = doc_link!("class/LocalScript")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LocalScript {
    pub script: Script,
    
}
#[doc = doc_link!("class/ModuleScript")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ModuleScript {
    pub lua_source_container: LuaSourceContainer,
    pub linked_source: crate::model::data::Content,
    pub source: String,
}
#[doc = doc_link!("class/LuaWebService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LuaWebService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/LuauScriptAnalyzerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct LuauScriptAnalyzerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MarkerCurve")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MarkerCurve {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MarketplaceService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MarketplaceService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MaterialService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MaterialService {
    pub base: crate::model::classes::Base,
    pub asphalt_name: String,
    pub basalt_name: String,
    pub brick_name: String,
    pub cobblestone_name: String,
    pub concrete_name: String,
    pub corroded_metal_name: String,
    pub cracked_lava_name: String,
    pub diamond_plate_name: String,
    pub fabric_name: String,
    pub foil_name: String,
    pub glacier_name: String,
    pub granite_name: String,
    pub grass_name: String,
    pub ground_name: String,
    pub ice_name: String,
    pub leafy_grass_name: String,
    pub limestone_name: String,
    pub marble_name: String,
    pub metal_name: String,
    pub mud_name: String,
    pub pavement_name: String,
    pub pebble_name: String,
    pub plastic_name: String,
    pub rock_name: String,
    pub salt_name: String,
    pub sand_name: String,
    pub sandstone_name: String,
    pub slate_name: String,
    pub smooth_plastic_name: String,
    pub snow_name: String,
    pub wood_name: String,
    pub wood_planks_name: String,
}
#[doc = doc_link!("class/MaterialVariant")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MaterialVariant {
    pub base: crate::model::classes::Base,
    pub base_material: super::enums::Material,
    pub color_map: crate::model::data::Content,
    pub custom_physical_properties: crate::model::data::PhysicalProperties,
    pub material_pattern: super::enums::MaterialPattern,
    pub metalness_map: crate::model::data::Content,
    pub normal_map: crate::model::data::Content,
    pub roughness_map: crate::model::data::Content,
    pub studs_per_tile: f32,
}
#[doc = doc_link!("class/MemStorageConnection")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MemStorageConnection {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MemStorageService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MemStorageService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MemoryStoreQueue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MemoryStoreQueue {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MemoryStoreService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MemoryStoreService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MemoryStoreSortedMap")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MemoryStoreSortedMap {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Message")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Message {
    pub base: crate::model::classes::Base,
    pub text: String,
}
#[doc = doc_link!("class/Hint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Hint {
    pub message: Message,
    
}
#[doc = doc_link!("class/MessageBusConnection")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MessageBusConnection {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MessageBusService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MessageBusService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MessagingService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MessagingService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MetaBreakpoint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MetaBreakpoint {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MetaBreakpointContext")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MetaBreakpointContext {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MetaBreakpointManager")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MetaBreakpointManager {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Mouse")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Mouse {
    pub base: crate::model::classes::Base,
    pub icon: crate::model::data::Content,
    pub target_filter: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/PlayerMouse")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PlayerMouse {
    pub mouse: Mouse,
    
}
#[doc = doc_link!("class/PluginMouse")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginMouse {
    pub mouse: Mouse,
    
}
#[doc = doc_link!("class/MouseService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MouseService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/MultipleDocumentInterfaceInstance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MultipleDocumentInterfaceInstance {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/NetworkMarker")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NetworkMarker {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/NetworkPeer")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NetworkPeer {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/NetworkClient")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NetworkClient {
    pub network_peer: NetworkPeer,
    
}
#[doc = doc_link!("class/NetworkServer")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NetworkServer {
    pub network_peer: NetworkPeer,
    
}
#[doc = doc_link!("class/NetworkReplicator")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NetworkReplicator {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ClientReplicator")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ClientReplicator {
    pub network_replicator: NetworkReplicator,
    
}
#[doc = doc_link!("class/ServerReplicator")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ServerReplicator {
    pub network_replicator: NetworkReplicator,
    
}
#[doc = doc_link!("class/NetworkSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NetworkSettings {
    pub base: crate::model::classes::Base,
    pub http_proxy_enabled: bool,
    pub http_proxy_url: String,
    pub incoming_replication_lag: f64,
    pub print_join_size_breakdown: bool,
    pub print_physics_errors: bool,
    pub print_stream_instance_quota: bool,
    pub randomize_join_instance_order: bool,
    pub render_streamed_regions: bool,
    pub show_active_animation_asset: bool,
}
#[doc = doc_link!("class/NoCollisionConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NoCollisionConstraint {
    pub base: crate::model::classes::Base,
    pub enabled: bool,
    pub part_0: crate::model::data::InstanceRef,
    pub part_1: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/NotificationService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NotificationService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PVInstance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PVInstance {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BasePart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BasePart {
    pub pv_instance: PVInstance,
    pub anchored: bool,
    pub back_param_a: f32,
    pub back_param_b: f32,
    pub back_surface: super::enums::SurfaceType,
    pub back_surface_input: super::enums::InputType,
    pub bottom_param_a: f32,
    pub bottom_param_b: f32,
    pub bottom_surface: super::enums::SurfaceType,
    pub bottom_surface_input: super::enums::InputType,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub can_collide: bool,
    pub can_query: bool,
    pub can_touch: bool,
    pub cast_shadow: bool,
    pub custom_physical_properties: crate::model::data::PhysicalProperties,
    pub front_param_a: f32,
    pub front_param_b: f32,
    pub front_surface: super::enums::SurfaceType,
    pub front_surface_input: super::enums::InputType,
    pub left_param_a: f32,
    pub left_param_b: f32,
    pub left_surface: super::enums::SurfaceType,
    pub left_surface_input: super::enums::InputType,
    pub locked: bool,
    pub massless: bool,
    pub material: super::enums::Material,
    pub pivot_offset: crate::model::data::CFrame,
    pub reflectance: f32,
    pub right_param_a: f32,
    pub right_param_b: f32,
    pub right_surface: super::enums::SurfaceType,
    pub right_surface_input: super::enums::InputType,
    pub root_priority: i32,
    pub rot_velocity: crate::model::data::Vector3,
    pub top_param_a: f32,
    pub top_param_b: f32,
    pub top_surface: super::enums::SurfaceType,
    pub top_surface_input: super::enums::InputType,
    pub transparency: f32,
    pub velocity: crate::model::data::Vector3,
}
#[doc = doc_link!("class/CornerWedgePart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CornerWedgePart {
    pub base_part: BasePart,
    
}
#[doc = doc_link!("class/FormFactorPart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FormFactorPart {
    pub base_part: BasePart,
    
}
#[doc = doc_link!("class/Part")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Part {
    pub form_factor_part: FormFactorPart,
    
}
#[doc = doc_link!("class/FlagStand")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FlagStand {
    pub part: Part,
    pub team_color: crate::model::data::BrickColor,
}
#[doc = doc_link!("class/Platform")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Platform {
    pub part: Part,
    
}
#[doc = doc_link!("class/Seat")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Seat {
    pub part: Part,
    pub disabled: bool,
}
#[doc = doc_link!("class/SkateboardPlatform")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SkateboardPlatform {
    pub part: Part,
    pub steer: i32,
    pub sticky_wheels: bool,
    pub throttle: i32,
}
#[doc = doc_link!("class/SpawnLocation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SpawnLocation {
    pub part: Part,
    pub allow_team_change_on_touch: bool,
    pub duration: i32,
    pub enabled: bool,
    pub neutral: bool,
    pub team_color: crate::model::data::BrickColor,
}
#[doc = doc_link!("class/WedgePart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WedgePart {
    pub form_factor_part: FormFactorPart,
    
}
#[doc = doc_link!("class/Terrain")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Terrain {
    pub base_part: BasePart,
    pub decoration: bool,
    pub material_colors: Vec<u8>,
    pub shorelines_upgraded: bool,
    pub water_color: crate::model::data::Color3,
    pub water_reflectance: f32,
    pub water_transparency: f32,
    pub water_wave_size: f32,
    pub water_wave_speed: f32,
}
#[doc = doc_link!("class/TriangleMeshPart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TriangleMeshPart {
    pub base_part: BasePart,
    
}
#[doc = doc_link!("class/MeshPart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct MeshPart {
    pub triangle_mesh_part: TriangleMeshPart,
    pub double_sided: bool,
    pub has_joint_offset: bool,
    pub has_skinned_mesh: bool,
    pub joint_offset: crate::model::data::Vector3,
    pub mesh_id: crate::model::data::Content,
    pub texture_id: crate::model::data::Content,
}
#[doc = doc_link!("class/PartOperation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PartOperation {
    pub triangle_mesh_part: TriangleMeshPart,
    pub render_fidelity: super::enums::RenderFidelity,
    pub smoothing_angle: f32,
    pub use_part_color: bool,
}
#[doc = doc_link!("class/NegateOperation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NegateOperation {
    pub part_operation: PartOperation,
    
}
#[doc = doc_link!("class/UnionOperation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UnionOperation {
    pub part_operation: PartOperation,
    
}
#[doc = doc_link!("class/TrussPart")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TrussPart {
    pub base_part: BasePart,
    
}
#[doc = doc_link!("class/VehicleSeat")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
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
#[doc = doc_link!("class/Model")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Model {
    pub pv_instance: PVInstance,
    pub level_of_detail: super::enums::ModelLevelOfDetail,
    pub model_streaming_mode: super::enums::ModelStreamingMode,
    pub primary_part: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/Actor")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Actor {
    pub model: Model,
    
}
#[doc = doc_link!("class/Status")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Status {
    pub model: Model,
    
}
#[doc = doc_link!("class/WorldRoot")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WorldRoot {
    pub model: Model,
    
}
#[doc = doc_link!("class/Workspace")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Workspace {
    pub world_root: WorldRoot,
    pub animation_weighted_blend_fix: super::enums::NewAnimationRuntimeSetting,
    pub client_animator_throttling: super::enums::ClientAnimatorThrottlingMode,
    pub fallen_parts_destroy_height: f32,
    pub global_wind: crate::model::data::Vector3,
    pub gravity: f32,
    pub humanoid_only_set_collisions_on_state_change: super::enums::HumanoidOnlySetCollisionsOnStateChange,
    pub interpolation_throttling: super::enums::InterpolationThrottlingMode,
    pub mesh_part_heads_and_accessories: super::enums::MeshPartHeadsAndAccessories,
    pub physics_stepping_method: super::enums::PhysicsSteppingMethod,
    pub reject_character_deletions: super::enums::RejectCharacterDeletions,
    pub replicate_instance_destroy_setting: super::enums::ReplicateInstanceDestroySetting,
    pub retargeting: super::enums::AnimatorRetargetingMode,
    pub signal_behavior: super::enums::SignalBehavior,
    pub stream_out_behavior: super::enums::StreamOutBehavior,
    pub streaming_enabled: bool,
    pub streaming_integrity_mode: super::enums::StreamingIntegrityMode,
    pub streaming_min_radius: i32,
    pub streaming_target_radius: i32,
    pub touches_use_collision_groups: bool,
    pub unions_scale_nonuniformly: super::enums::UnionsScaleNonuniformly,
}
#[doc = doc_link!("class/WorldModel")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WorldModel {
    pub world_root: WorldRoot,
    
}
#[doc = doc_link!("class/PackageLink")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PackageLink {
    pub base: crate::model::classes::Base,
    pub auto_update: bool,
}
#[doc = doc_link!("class/PackageService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PackageService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PackageUIService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PackageUIService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Pages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Pages {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CatalogPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CatalogPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/DataStoreKeyPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreKeyPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/DataStoreListingPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreListingPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/DataStorePages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStorePages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/DataStoreVersionPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataStoreVersionPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/FriendPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FriendPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/InventoryPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct InventoryPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/EmotesPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct EmotesPages {
    pub inventory_pages: InventoryPages,
    
}
#[doc = doc_link!("class/OutfitPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct OutfitPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/StandardPages")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StandardPages {
    pub pages: Pages,
    
}
#[doc = doc_link!("class/PartOperationAsset")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PartOperationAsset {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ParticleEmitter")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ParticleEmitter {
    pub base: crate::model::classes::Base,
    pub acceleration: crate::model::data::Vector3,
    pub brightness: f32,
    pub color: crate::model::data::ColorSequence,
    pub drag: f32,
    pub emission_direction: super::enums::NormalId,
    pub enabled: bool,
    pub flipbook_framerate: crate::model::data::NumberRange,
    pub flipbook_incompatible: String,
    pub flipbook_layout: super::enums::ParticleFlipbookLayout,
    pub flipbook_mode: super::enums::ParticleFlipbookMode,
    pub flipbook_start_random: bool,
    pub lifetime: crate::model::data::NumberRange,
    pub light_emission: f32,
    pub light_influence: f32,
    pub locked_to_part: bool,
    pub orientation: super::enums::ParticleOrientation,
    pub rate: f32,
    pub rot_speed: crate::model::data::NumberRange,
    pub rotation: crate::model::data::NumberRange,
    pub shape: super::enums::ParticleEmitterShape,
    pub shape_in_out: super::enums::ParticleEmitterShapeInOut,
    pub shape_partial: f32,
    pub shape_style: super::enums::ParticleEmitterShapeStyle,
    pub size: crate::model::data::NumberSequence,
    pub speed: crate::model::data::NumberRange,
    pub spread_angle: crate::model::data::Vector2,
    pub squash: crate::model::data::NumberSequence,
    pub texture: crate::model::data::Content,
    pub time_scale: f32,
    pub transparency: crate::model::data::NumberSequence,
    pub velocity_inheritance: f32,
    pub z_offset: f32,
}
#[doc = doc_link!("class/Path")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Path {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PathfindingLink")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PathfindingLink {
    pub base: crate::model::classes::Base,
    pub attachment_0: crate::model::data::InstanceRef,
    pub attachment_1: crate::model::data::InstanceRef,
    pub is_bidirectional: bool,
    pub label: String,
}
#[doc = doc_link!("class/PathfindingModifier")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PathfindingModifier {
    pub base: crate::model::classes::Base,
    pub label: String,
    pub pass_through: bool,
}
#[doc = doc_link!("class/PathfindingService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PathfindingService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PausedState")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PausedState {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PausedStateBreakpoint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PausedStateBreakpoint {
    pub paused_state: PausedState,
    
}
#[doc = doc_link!("class/PausedStateException")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PausedStateException {
    pub paused_state: PausedState,
    
}
#[doc = doc_link!("class/PermissionsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PermissionsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PhysicsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PhysicsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PhysicsSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PhysicsSettings {
    pub base: crate::model::classes::Base,
    pub allow_sleep: bool,
    pub are_anchors_shown: bool,
    pub are_assemblies_shown: bool,
    pub are_awake_parts_highlighted: bool,
    pub are_body_types_shown: bool,
    pub are_contact_islands_shown: bool,
    pub are_contact_points_shown: bool,
    pub are_joint_coordinates_shown: bool,
    pub are_mechanisms_shown: bool,
    pub are_model_coords_shown: bool,
    pub are_owners_shown: bool,
    pub are_part_coords_shown: bool,
    pub are_regions_shown: bool,
    pub are_terrain_replication_regions_shown: bool,
    pub are_timesteps_shown: bool,
    pub are_unaligned_parts_shown: bool,
    pub are_world_coords_shown: bool,
    pub disable_cs_gv_2: bool,
    pub is_interpolation_throttle_shown: bool,
    pub is_receive_age_shown: bool,
    pub is_tree_shown: bool,
    pub physics_environmental_throttle: super::enums::EnviromentalPhysicsThrottle,
    pub show_decomposition_geometry: bool,
    pub throttle_adjust_time: f64,
    pub use_cs_gv_2: bool,
}
#[doc = doc_link!("class/Player")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Player {
    pub base: crate::model::classes::Base,
    pub auto_jump_enabled: bool,
    pub camera_max_zoom_distance: f32,
    pub camera_min_zoom_distance: f32,
    pub camera_mode: super::enums::CameraMode,
    pub can_load_character_appearance: bool,
    pub character: crate::model::data::InstanceRef,
    pub character_appearance: String,
    pub character_appearance_id: i64,
    pub dev_camera_occlusion_mode: super::enums::DevCameraOcclusionMode,
    pub dev_computer_camera_mode: super::enums::DevComputerCameraMovementMode,
    pub dev_computer_movement_mode: super::enums::DevComputerMovementMode,
    pub dev_enable_mouse_lock: bool,
    pub dev_touch_camera_mode: super::enums::DevTouchCameraMovementMode,
    pub dev_touch_movement_mode: super::enums::DevTouchMovementMode,
    pub display_name: String,
    pub gameplay_paused: bool,
    pub has_verified_badge: bool,
    pub health_display_distance: f32,
    pub name_display_distance: f32,
    pub neutral: bool,
    pub platform_name: String,
    pub replication_focus: crate::model::data::InstanceRef,
    pub respawn_location: crate::model::data::InstanceRef,
    pub simulation_radius: f32,
    pub team_color: crate::model::data::BrickColor,
    pub teleported_in: bool,
    pub user_id: i64,
    pub vr_device: String,
    #[propname = "userId"] pub _user_id: i64,
}
#[doc = doc_link!("class/PlayerEmulatorService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PlayerEmulatorService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PlayerScripts")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PlayerScripts {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Players")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Players {
    pub base: crate::model::classes::Base,
    pub max_players_internal: i32,
    pub preferred_players_internal: i32,
    pub respawn_time: f32,
}
#[doc = doc_link!("class/Plugin")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Plugin {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginAction")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginAction {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginDebugService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginDebugService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginDragEvent")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginDragEvent {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginGuiService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginGuiService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginManagementService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginManagementService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginManager")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginManager {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginManagerInterface")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginManagerInterface {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginMenu")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginMenu {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginPolicyService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginPolicyService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginToolbar")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginToolbar {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PluginToolbarButton")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PluginToolbarButton {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PointsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PointsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/PolicyService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PolicyService {
    pub base: crate::model::classes::Base,
    pub is_luobu_server: super::enums::TriStateBoolean,
    pub luobu_whitelisted: super::enums::TriStateBoolean,
}
#[doc = doc_link!("class/PoseBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PoseBase {
    pub base: crate::model::classes::Base,
    pub easing_direction: super::enums::PoseEasingDirection,
    pub easing_style: super::enums::PoseEasingStyle,
    pub weight: f32,
}
#[doc = doc_link!("class/NumberPose")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NumberPose {
    pub pose_base: PoseBase,
    pub value: f64,
}
#[doc = doc_link!("class/Pose")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Pose {
    pub pose_base: PoseBase,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
}
#[doc = doc_link!("class/PostEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PostEffect {
    pub base: crate::model::classes::Base,
    pub enabled: bool,
}
#[doc = doc_link!("class/BloomEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BloomEffect {
    pub post_effect: PostEffect,
    pub intensity: f32,
    pub size: f32,
    pub threshold: f32,
}
#[doc = doc_link!("class/BlurEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BlurEffect {
    pub post_effect: PostEffect,
    pub size: f32,
}
#[doc = doc_link!("class/ColorCorrectionEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ColorCorrectionEffect {
    pub post_effect: PostEffect,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub tint_color: crate::model::data::Color3,
}
#[doc = doc_link!("class/DepthOfFieldEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DepthOfFieldEffect {
    pub post_effect: PostEffect,
    pub far_intensity: f32,
    pub focus_distance: f32,
    pub in_focus_radius: f32,
    pub near_intensity: f32,
}
#[doc = doc_link!("class/SunRaysEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SunRaysEffect {
    pub post_effect: PostEffect,
    pub intensity: f32,
    pub spread: f32,
}
#[doc = doc_link!("class/ProcessInstancePhysicsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ProcessInstancePhysicsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ProximityPrompt")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ProximityPrompt {
    pub base: crate::model::classes::Base,
    pub action_text: String,
    pub auto_localize: bool,
    pub clickable_prompt: bool,
    pub enabled: bool,
    pub exclusivity: super::enums::ProximityPromptExclusivity,
    pub gamepad_key_code: super::enums::KeyCode,
    pub hold_duration: f32,
    pub keyboard_key_code: super::enums::KeyCode,
    pub max_activation_distance: f32,
    pub object_text: String,
    pub requires_line_of_sight: bool,
    pub root_localization_table: crate::model::data::InstanceRef,
    pub style: super::enums::ProximityPromptStyle,
    pub ui_offset: crate::model::data::Vector2,
}
#[doc = doc_link!("class/ProximityPromptService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ProximityPromptService {
    pub base: crate::model::classes::Base,
    pub enabled: bool,
    pub max_prompts_visible: i32,
}
#[doc = doc_link!("class/PublishService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PublishService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RbxAnalyticsService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RbxAnalyticsService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadata")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadata {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataCallbacks")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataCallbacks {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataClasses")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataClasses {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataEnums")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataEnums {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataEvents")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataEvents {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataFunctions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataFunctions {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataItem")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataItem {
    pub base: crate::model::classes::Base,
    pub browsable: bool,
    pub class_category: String,
    pub client_only: bool,
    pub constraint: String,
    pub deprecated: bool,
    pub editing_disabled: bool,
    pub editor_type: String,
    pub f_flag: String,
    pub is_backend: bool,
    pub property_order: i32,
    pub script_context: String,
    pub server_only: bool,
    pub slider_scaling: String,
    pub ui_maximum: f64,
    pub ui_minimum: f64,
    pub ui_num_ticks: f64,
}
#[doc = doc_link!("class/ReflectionMetadataClass")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataClass {
    pub reflection_metadata_item: ReflectionMetadataItem,
    pub explorer_image_index: i32,
    pub explorer_order: i32,
    pub insertable: bool,
    pub preferred_parent: String,
}
#[doc = doc_link!("class/ReflectionMetadataEnum")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataEnum {
    pub reflection_metadata_item: ReflectionMetadataItem,
    
}
#[doc = doc_link!("class/ReflectionMetadataEnumItem")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataEnumItem {
    pub reflection_metadata_item: ReflectionMetadataItem,
    
}
#[doc = doc_link!("class/ReflectionMetadataMember")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataMember {
    pub reflection_metadata_item: ReflectionMetadataItem,
    
}
#[doc = doc_link!("class/ReflectionMetadataProperties")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataProperties {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReflectionMetadataYieldFunctions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReflectionMetadataYieldFunctions {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RemoteDebuggerServer")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RemoteDebuggerServer {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RemoteEvent")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RemoteEvent {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RemoteFunction")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RemoteFunction {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RenderSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RenderSettings {
    pub base: crate::model::classes::Base,
    pub auto_frm_level: i32,
    pub eager_bulk_execution: bool,
    pub edit_quality_level: super::enums::QualityLevel,
    pub export_merge_by_material: bool,
    pub frame_rate_manager: super::enums::FramerateManagerMode,
    pub graphics_mode: super::enums::GraphicsMode,
    pub mesh_cache_size: i32,
    pub mesh_part_detail_level: super::enums::MeshPartDetailLevel,
    pub quality_level: super::enums::QualityLevel,
    pub reload_assets: bool,
    pub render_csg_triangles_debug: bool,
    pub show_bounding_boxes: bool,
}
#[doc = doc_link!("class/RenderingTest")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RenderingTest {
    pub base: crate::model::classes::Base,
    #[propname = "CFrame"] pub cframe: crate::model::data::CFrame,
    pub comparison_diff_threshold: i32,
    pub comparison_method: super::enums::RenderingTestComparisonMethod,
    pub comparison_psnr_threshold: f32,
    pub description: String,
    pub field_of_view: f32,
    pub quality_level: i32,
    pub should_skip: bool,
    pub ticket: String,
}
#[doc = doc_link!("class/ReplicatedFirst")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReplicatedFirst {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ReplicatedStorage")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReplicatedStorage {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RobloxPluginGuiService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RobloxPluginGuiService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RobloxReplicatedStorage")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RobloxReplicatedStorage {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RotationCurve")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RotationCurve {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RtMessagingService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RtMessagingService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RunService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RunService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RuntimeScriptService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RuntimeScriptService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScreenshotHud")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScreenshotHud {
    pub base: crate::model::classes::Base,
    pub camera_button_icon: crate::model::data::Content,
    pub camera_button_position: crate::model::data::UDim2,
    pub close_button_position: crate::model::data::UDim2,
    pub close_when_screenshot_taken: bool,
    pub experience_name_overlay_enabled: bool,
    pub overlay_font: super::enums::Font,
    pub username_overlay_enabled: bool,
    pub visible: bool,
}
#[doc = doc_link!("class/ScriptBuilder")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptBuilder {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/CoreScriptBuilder")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CoreScriptBuilder {
    pub script_builder: ScriptBuilder,
    
}
#[doc = doc_link!("class/ScriptChangeService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptChangeService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptCloneWatcher")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptCloneWatcher {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptCloneWatcherHelper")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptCloneWatcherHelper {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptContext")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptContext {
    pub base: crate::model::classes::Base,
    pub scripts_disabled: bool,
}
#[doc = doc_link!("class/ScriptDebugger")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptDebugger {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptDocument")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptDocument {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptEditorService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptEditorService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptRegistrationService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptRegistrationService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ScriptService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ScriptService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Selection")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Selection {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ServerScriptService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ServerScriptService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ServerStorage")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ServerStorage {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ServiceProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ServiceProvider {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/DataModel")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DataModel {
    pub service_provider: ServiceProvider,
    
}
#[doc = doc_link!("class/GenericSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GenericSettings {
    pub service_provider: ServiceProvider,
    
}
#[doc = doc_link!("class/AnalysticsSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AnalysticsSettings {
    pub generic_settings: GenericSettings,
    
}
#[doc = doc_link!("class/GlobalSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct GlobalSettings {
    pub generic_settings: GenericSettings,
    
}
#[doc = doc_link!("class/UserSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UserSettings {
    pub generic_settings: GenericSettings,
    
}
#[doc = doc_link!("class/SessionService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SessionService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ShorelineUpgraderService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ShorelineUpgraderService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Sky")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Sky {
    pub base: crate::model::classes::Base,
    pub celestial_bodies_shown: bool,
    pub moon_angular_size: f32,
    pub moon_texture_id: crate::model::data::Content,
    pub skybox_bk: crate::model::data::Content,
    pub skybox_dn: crate::model::data::Content,
    pub skybox_ft: crate::model::data::Content,
    pub skybox_lf: crate::model::data::Content,
    pub skybox_rt: crate::model::data::Content,
    pub skybox_up: crate::model::data::Content,
    pub star_count: i32,
    pub sun_angular_size: f32,
    pub sun_texture_id: crate::model::data::Content,
}
#[doc = doc_link!("class/Smoke")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Smoke {
    pub base: crate::model::classes::Base,
    pub color: crate::model::data::Color3,
    pub enabled: bool,
    pub time_scale: f32,
}
#[doc = doc_link!("class/SnippetService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SnippetService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/SocialService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SocialService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Sound")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Sound {
    pub base: crate::model::classes::Base,
    pub emitter_size: f32,
    pub loop_region: crate::model::data::NumberRange,
    pub looped: bool,
    pub max_distance: f32,
    pub min_distance: f32,
    pub pitch: f32,
    pub play_on_remove: bool,
    pub playback_region: crate::model::data::NumberRange,
    pub playback_regions_enabled: bool,
    pub roll_off_mode: super::enums::RollOffMode,
    pub sound_group: crate::model::data::InstanceRef,
    pub sound_id: crate::model::data::Content,
    pub volume: f32,
}
#[doc = doc_link!("class/SoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SoundEffect {
    pub base: crate::model::classes::Base,
    pub enabled: bool,
    pub priority: i32,
}
#[doc = doc_link!("class/AssetSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct AssetSoundEffect {
    pub sound_effect: SoundEffect,
    
}
#[doc = doc_link!("class/ChorusSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ChorusSoundEffect {
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub mix: f32,
    pub rate: f32,
}
#[doc = doc_link!("class/CompressorSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CompressorSoundEffect {
    pub sound_effect: SoundEffect,
    pub attack: f32,
    pub gain_makeup: f32,
    pub ratio: f32,
    pub release: f32,
    pub side_chain: crate::model::data::InstanceRef,
    pub threshold: f32,
}
#[doc = doc_link!("class/CustomSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CustomSoundEffect {
    pub sound_effect: SoundEffect,
    
}
#[doc = doc_link!("class/ChannelSelectorSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ChannelSelectorSoundEffect {
    pub custom_sound_effect: CustomSoundEffect,
    pub channel: i32,
}
#[doc = doc_link!("class/DistortionSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DistortionSoundEffect {
    pub sound_effect: SoundEffect,
    pub level: f32,
}
#[doc = doc_link!("class/EchoSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct EchoSoundEffect {
    pub sound_effect: SoundEffect,
    pub delay: f32,
    pub dry_level: f32,
    pub feedback: f32,
    pub wet_level: f32,
}
#[doc = doc_link!("class/EqualizerSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct EqualizerSoundEffect {
    pub sound_effect: SoundEffect,
    pub high_gain: f32,
    pub low_gain: f32,
    pub mid_gain: f32,
}
#[doc = doc_link!("class/FlangeSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct FlangeSoundEffect {
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub mix: f32,
    pub rate: f32,
}
#[doc = doc_link!("class/PitchShiftSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct PitchShiftSoundEffect {
    pub sound_effect: SoundEffect,
    pub octave: f32,
}
#[doc = doc_link!("class/ReverbSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ReverbSoundEffect {
    pub sound_effect: SoundEffect,
    pub decay_time: f32,
    pub density: f32,
    pub diffusion: f32,
    pub dry_level: f32,
    pub wet_level: f32,
}
#[doc = doc_link!("class/TremoloSoundEffect")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TremoloSoundEffect {
    pub sound_effect: SoundEffect,
    pub depth: f32,
    pub duty: f32,
    pub frequency: f32,
}
#[doc = doc_link!("class/SoundGroup")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SoundGroup {
    pub base: crate::model::classes::Base,
    pub volume: f32,
}
#[doc = doc_link!("class/SoundService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SoundService {
    pub base: crate::model::classes::Base,
    pub ambient_reverb: super::enums::ReverbType,
    pub distance_factor: f32,
    pub doppler_scale: f32,
    pub respect_filtering_enabled: bool,
    pub rolloff_scale: f32,
    pub volumetric_audio: super::enums::VolumetricAudio,
}
#[doc = doc_link!("class/Sparkles")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Sparkles {
    pub base: crate::model::classes::Base,
    pub enabled: bool,
    pub sparkle_color: crate::model::data::Color3,
    pub time_scale: f32,
}
#[doc = doc_link!("class/SpawnerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SpawnerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Speaker")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Speaker {
    pub base: crate::model::classes::Base,
    pub roll_off_mode: super::enums::RollOffMode,
    pub sound_group: crate::model::data::InstanceRef,
    pub source: crate::model::data::InstanceRef,
    pub volume: f32,
}
#[doc = doc_link!("class/StackFrame")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StackFrame {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StandalonePluginScripts")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StandalonePluginScripts {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StarterGear")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StarterGear {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StarterPack")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StarterPack {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StarterPlayer")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StarterPlayer {
    pub base: crate::model::classes::Base,
    pub allow_custom_animations: bool,
    pub auto_jump_enabled: bool,
    pub camera_max_zoom_distance: f32,
    pub camera_min_zoom_distance: f32,
    pub camera_mode: super::enums::CameraMode,
    pub character_jump_height: f32,
    pub character_jump_power: f32,
    pub character_max_slope_angle: f32,
    pub character_use_jump_power: bool,
    pub character_walk_speed: f32,
    pub dev_camera_occlusion_mode: super::enums::DevCameraOcclusionMode,
    pub dev_computer_camera_movement_mode: super::enums::DevComputerCameraMovementMode,
    pub dev_computer_movement_mode: super::enums::DevComputerMovementMode,
    pub dev_touch_camera_movement_mode: super::enums::DevTouchCameraMovementMode,
    pub dev_touch_movement_mode: super::enums::DevTouchMovementMode,
    pub enable_dynamic_heads: super::enums::LoadDynamicHeads,
    pub enable_mouse_lock_option: bool,
    pub health_display_distance: f32,
    pub humanoid_state_machine_mode: super::enums::HumanoidStateMachineMode,
    pub load_character_appearance: bool,
    pub name_display_distance: f32,
    pub user_emotes_enabled: bool,
}
#[doc = doc_link!("class/StarterPlayerScripts")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StarterPlayerScripts {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StarterCharacterScripts")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StarterCharacterScripts {
    pub starter_player_scripts: StarterPlayerScripts,
    
}
#[doc = doc_link!("class/Stats")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Stats {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StatsItem")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StatsItem {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/RunningAverageItemDouble")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RunningAverageItemDouble {
    pub stats_item: StatsItem,
    
}
#[doc = doc_link!("class/RunningAverageItemInt")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RunningAverageItemInt {
    pub stats_item: StatsItem,
    
}
#[doc = doc_link!("class/RunningAverageTimeIntervalItem")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RunningAverageTimeIntervalItem {
    pub stats_item: StatsItem,
    
}
#[doc = doc_link!("class/TotalCountTimeIntervalItem")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TotalCountTimeIntervalItem {
    pub stats_item: StatsItem,
    
}
#[doc = doc_link!("class/StopWatchReporter")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StopWatchReporter {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Studio")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Studio {
    pub base: crate::model::classes::Base,
    pub command_bar_local_state: bool,
    pub default_script_file_dir: String,
    pub deprecated_objects_shown: bool,
    pub display_language: String,
    pub font: String,
    pub icon_override_dir: String,
    pub local_assets_folder: String,
    pub lua_debugger_enabled: bool,
    pub permission_level_shown: super::enums::PermissionLevelShown,
    pub plugin_debugging_enabled: bool,
    pub plugins_dir: String,
    pub rulers: String,
    pub runtime_undo_behavior: super::enums::RuntimeUndoBehavior,
    pub script_timeout_length: i32,
    pub show_core_packages_in_explorer: bool,
    pub theme: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/StudioAssetService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioAssetService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StudioData")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioData {
    pub base: crate::model::classes::Base,
    pub enable_script_collab_by_default_on_load: bool,
}
#[doc = doc_link!("class/StudioDeviceEmulatorService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioDeviceEmulatorService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StudioHighDpiService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioHighDpiService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StudioPublishService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioPublishService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StudioScriptDebugEventListener")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioScriptDebugEventListener {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StudioService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/StudioTheme")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StudioTheme {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/SurfaceAppearance")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct SurfaceAppearance {
    pub base: crate::model::classes::Base,
    pub alpha_mode: super::enums::AlphaMode,
    pub color_map: crate::model::data::Content,
    pub metalness_map: crate::model::data::Content,
    pub normal_map: crate::model::data::Content,
    pub roughness_map: crate::model::data::Content,
    pub texture_pack: crate::model::data::Content,
}
#[doc = doc_link!("class/TaskScheduler")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TaskScheduler {
    pub base: crate::model::classes::Base,
    pub thread_pool_config: super::enums::ThreadPoolConfig,
}
#[doc = doc_link!("class/Team")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Team {
    pub base: crate::model::classes::Base,
    pub auto_assignable: bool,
    pub child_order: i32,
    pub team_color: crate::model::data::BrickColor,
}
#[doc = doc_link!("class/TeamCreateService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TeamCreateService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Teams")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Teams {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TeleportAsyncResult")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TeleportAsyncResult {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TeleportOptions")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TeleportOptions {
    pub base: crate::model::classes::Base,
    pub reserved_server_access_code: String,
    pub server_instance_id: String,
    pub should_reserve_server: bool,
}
#[doc = doc_link!("class/TeleportService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TeleportService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TemporaryCageMeshProvider")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TemporaryCageMeshProvider {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TemporaryScriptService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TemporaryScriptService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TerrainDetail")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TerrainDetail {
    pub base: crate::model::classes::Base,
    pub color_map: crate::model::data::Content,
    pub face: super::enums::TerrainFace,
    pub material_pattern: super::enums::MaterialPattern,
    pub metalness_map: crate::model::data::Content,
    pub normal_map: crate::model::data::Content,
    pub roughness_map: crate::model::data::Content,
    pub studs_per_tile: f32,
}
#[doc = doc_link!("class/TerrainRegion")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TerrainRegion {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TestService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TestService {
    pub base: crate::model::classes::Base,
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
#[doc = doc_link!("class/TextBoxService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextBoxService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TextChannel")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextChannel {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TextChatCommand")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextChatCommand {
    pub base: crate::model::classes::Base,
    pub enabled: bool,
    pub primary_alias: String,
    pub secondary_alias: String,
}
#[doc = doc_link!("class/TextChatConfigurations")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextChatConfigurations {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BubbleChatConfiguration")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BubbleChatConfiguration {
    pub text_chat_configurations: TextChatConfigurations,
    pub adornee_name: String,
    pub background_color_3: crate::model::data::Color3,
    pub background_transparency: f64,
    pub bubble_duration: f32,
    pub bubbles_spacing: f32,
    pub enabled: bool,
    pub font: super::enums::Font,
    pub font_face: crate::model::data::Font,
    pub local_player_studs_offset: crate::model::data::Vector3,
    pub max_distance: f32,
    pub minimize_distance: f32,
    pub text_color_3: crate::model::data::Color3,
    pub text_size: i64,
    pub vertical_studs_offset: f32,
}
#[doc = doc_link!("class/ChatInputBarConfiguration")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ChatInputBarConfiguration {
    pub text_chat_configurations: TextChatConfigurations,
    pub absolute_position_write: crate::model::data::Vector2,
    pub absolute_size_write: crate::model::data::Vector2,
    pub background_color_3: crate::model::data::Color3,
    pub background_transparency: f64,
    pub enabled: bool,
    pub font_face: crate::model::data::Font,
    pub placeholder_color_3: crate::model::data::Color3,
    pub target_text_channel: crate::model::data::InstanceRef,
    pub text_color_3: crate::model::data::Color3,
    pub text_size: i64,
    pub text_stroke_color_3: crate::model::data::Color3,
    pub text_stroke_transparency: f64,
}
#[doc = doc_link!("class/ChatWindowConfiguration")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ChatWindowConfiguration {
    pub text_chat_configurations: TextChatConfigurations,
    pub absolute_position_write: crate::model::data::Vector2,
    pub absolute_size_write: crate::model::data::Vector2,
    pub background_color_3: crate::model::data::Color3,
    pub background_transparency: f64,
    pub enabled: bool,
    pub font_face: crate::model::data::Font,
    pub height_scale: f32,
    pub horizontal_alignment: super::enums::HorizontalAlignment,
    pub text_color_3: crate::model::data::Color3,
    pub text_size: i64,
    pub text_stroke_color_3: crate::model::data::Color3,
    pub text_stroke_transparency: f64,
    pub vertical_alignment: super::enums::VerticalAlignment,
    pub width_scale: f32,
}
#[doc = doc_link!("class/TextChatMessage")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextChatMessage {
    pub base: crate::model::classes::Base,
    pub message_id: String,
    pub metadata: String,
    pub prefix_text: String,
    pub status: super::enums::TextChatMessageStatus,
    pub text: String,
    pub text_channel: crate::model::data::InstanceRef,
    pub text_source: crate::model::data::InstanceRef,
    pub timestamp: i64,
}
#[doc = doc_link!("class/TextChatMessageProperties")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextChatMessageProperties {
    pub base: crate::model::classes::Base,
    pub prefix_text: String,
    pub text: String,
}
#[doc = doc_link!("class/TextChatService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextChatService {
    pub base: crate::model::classes::Base,
    pub chat_version: super::enums::ChatVersion,
    pub create_default_commands: bool,
    pub create_default_text_channels: bool,
}
#[doc = doc_link!("class/TextFilterResult")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextFilterResult {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TextService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TextSource")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TextSource {
    pub base: crate::model::classes::Base,
    pub can_send: bool,
}
#[doc = doc_link!("class/ThirdPartyUserService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ThirdPartyUserService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ThreadState")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ThreadState {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TimerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TimerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/ToastNotificationService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ToastNotificationService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TouchInputService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TouchInputService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TouchTransmitter")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TouchTransmitter {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TracerService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TracerService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TrackerLodController")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TrackerLodController {
    pub base: crate::model::classes::Base,
    pub audio_mode: super::enums::TrackerLodFlagMode,
    pub video_extrapolation_mode: super::enums::TrackerExtrapolationFlagMode,
    pub video_lod_mode: super::enums::TrackerLodValueMode,
    pub video_mode: super::enums::TrackerLodFlagMode,
}
#[doc = doc_link!("class/TrackerStreamAnimation")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TrackerStreamAnimation {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Trail")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Trail {
    pub base: crate::model::classes::Base,
    pub attachment_0: crate::model::data::InstanceRef,
    pub attachment_1: crate::model::data::InstanceRef,
    pub brightness: f32,
    pub color: crate::model::data::ColorSequence,
    pub enabled: bool,
    pub face_camera: bool,
    pub lifetime: f32,
    pub light_emission: f32,
    pub light_influence: f32,
    pub max_length: f32,
    pub min_length: f32,
    pub texture: crate::model::data::Content,
    pub texture_length: f32,
    pub texture_mode: super::enums::TextureMode,
    pub transparency: crate::model::data::NumberSequence,
    pub width_scale: crate::model::data::NumberSequence,
}
#[doc = doc_link!("class/Translator")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Translator {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/TweenBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TweenBase {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Tween")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Tween {
    pub tween_base: TweenBase,
    
}
#[doc = doc_link!("class/TweenService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct TweenService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/UGCValidationService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UGCValidationService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/UIBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIBase {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/UIComponent")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIComponent {
    pub ui_base: UIBase,
    
}
#[doc = doc_link!("class/UIConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIConstraint {
    pub ui_component: UIComponent,
    
}
#[doc = doc_link!("class/UIAspectRatioConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIAspectRatioConstraint {
    pub ui_constraint: UIConstraint,
    pub aspect_ratio: f32,
    pub aspect_type: super::enums::AspectType,
    pub dominant_axis: super::enums::DominantAxis,
}
#[doc = doc_link!("class/UISizeConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UISizeConstraint {
    pub ui_constraint: UIConstraint,
    pub max_size: crate::model::data::Vector2,
    pub min_size: crate::model::data::Vector2,
}
#[doc = doc_link!("class/UITextSizeConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UITextSizeConstraint {
    pub ui_constraint: UIConstraint,
    pub max_text_size: i32,
    pub min_text_size: i32,
}
#[doc = doc_link!("class/UICorner")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UICorner {
    pub ui_component: UIComponent,
    pub corner_radius: crate::model::data::UDim,
}
#[doc = doc_link!("class/UIGradient")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIGradient {
    pub ui_component: UIComponent,
    pub color: crate::model::data::ColorSequence,
    pub enabled: bool,
    pub offset: crate::model::data::Vector2,
    pub rotation: f32,
    pub transparency: crate::model::data::NumberSequence,
}
#[doc = doc_link!("class/UILayout")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UILayout {
    pub ui_component: UIComponent,
    
}
#[doc = doc_link!("class/UIGridStyleLayout")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIGridStyleLayout {
    pub ui_layout: UILayout,
    pub fill_direction: super::enums::FillDirection,
    pub horizontal_alignment: super::enums::HorizontalAlignment,
    pub sort_order: super::enums::SortOrder,
    pub vertical_alignment: super::enums::VerticalAlignment,
}
#[doc = doc_link!("class/UIGridLayout")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIGridLayout {
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub cell_padding: crate::model::data::UDim2,
    pub cell_size: crate::model::data::UDim2,
    pub fill_direction_max_cells: i32,
    pub start_corner: super::enums::StartCorner,
}
#[doc = doc_link!("class/UIListLayout")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIListLayout {
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub padding: crate::model::data::UDim,
}
#[doc = doc_link!("class/UIPageLayout")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIPageLayout {
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub animated: bool,
    pub circular: bool,
    pub easing_direction: super::enums::EasingDirection,
    pub easing_style: super::enums::EasingStyle,
    pub gamepad_input_enabled: bool,
    pub padding: crate::model::data::UDim,
    pub scroll_wheel_input_enabled: bool,
    pub touch_input_enabled: bool,
    pub tween_time: f32,
}
#[doc = doc_link!("class/UITableLayout")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UITableLayout {
    pub ui_grid_style_layout: UIGridStyleLayout,
    pub fill_empty_space_columns: bool,
    pub fill_empty_space_rows: bool,
    pub major_axis: super::enums::TableMajorAxis,
    pub padding: crate::model::data::UDim2,
}
#[doc = doc_link!("class/UIPadding")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIPadding {
    pub ui_component: UIComponent,
    pub padding_bottom: crate::model::data::UDim,
    pub padding_left: crate::model::data::UDim,
    pub padding_right: crate::model::data::UDim,
    pub padding_top: crate::model::data::UDim,
}
#[doc = doc_link!("class/UIScale")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIScale {
    pub ui_component: UIComponent,
    pub scale: f32,
}
#[doc = doc_link!("class/UIStroke")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UIStroke {
    pub ui_component: UIComponent,
    pub apply_stroke_mode: super::enums::ApplyStrokeMode,
    pub color: crate::model::data::Color3,
    pub enabled: bool,
    pub line_join_mode: super::enums::LineJoinMode,
    pub thickness: f32,
    pub transparency: f32,
}
#[doc = doc_link!("class/UnvalidatedAssetService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UnvalidatedAssetService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/UserGameSettings")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UserGameSettings {
    pub base: crate::model::classes::Base,
    pub all_tutorials_disabled: bool,
    pub camera_mode: super::enums::CustomCameraMode,
    pub chat_visible: bool,
    pub computer_camera_movement_mode: super::enums::ComputerCameraMovementMode,
    pub computer_movement_mode: super::enums::ComputerMovementMode,
    pub control_mode: super::enums::ControlMode,
    pub fullscreen: bool,
    pub gamepad_camera_sensitivity: f32,
    pub graphics_quality_level: i32,
    pub has_ever_used_vr: bool,
    pub master_volume: f32,
    pub micro_profiler_web_server_enabled: bool,
    pub mouse_sensitivity: f32,
    pub on_screen_profiler_enabled: bool,
    pub onboardings_completed: String,
    pub performance_stats_visible: bool,
    pub rcc_profiler_record_frame_rate: i32,
    pub rcc_profiler_record_time_frame: i32,
    pub rotation_type: super::enums::RotationType,
    pub saved_quality_level: super::enums::SavedQualitySetting,
    pub touch_camera_movement_mode: super::enums::TouchCameraMovementMode,
    pub touch_movement_mode: super::enums::TouchMovementMode,
    pub used_core_gui_is_visible_toggle: bool,
    pub used_custom_gui_is_visible_toggle: bool,
    pub used_hide_hud_shortcut: bool,
    pub vr_enabled: bool,
    pub vr_rotation_intensity: i32,
    pub vr_smooth_rotation_enabled: bool,
    pub vignette_enabled: bool,
}
#[doc = doc_link!("class/UserInputService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UserInputService {
    pub base: crate::model::classes::Base,
    pub legacy_input_events_enabled: bool,
    pub modal_enabled: bool,
    pub mouse_behavior: super::enums::MouseBehavior,
    pub mouse_icon_enabled: bool,
    pub override_mouse_icon_behavior: super::enums::OverrideMouseIconBehavior,
}
#[doc = doc_link!("class/UserService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct UserService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VRService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VRService {
    pub base: crate::model::classes::Base,
    pub gui_input_user_c_frame: super::enums::UserCFrame,
}
#[doc = doc_link!("class/ValueBase")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ValueBase {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/BinaryStringValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BinaryStringValue {
    pub value_base: ValueBase,
    
}
#[doc = doc_link!("class/BoolValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BoolValue {
    pub value_base: ValueBase,
    pub value: bool,
}
#[doc = doc_link!("class/BrickColorValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct BrickColorValue {
    pub value_base: ValueBase,
    pub value: crate::model::data::BrickColor,
}
#[doc = doc_link!("class/CFrameValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct CFrameValue {
    pub value_base: ValueBase,
    pub value: crate::model::data::CFrame,
}
#[doc = doc_link!("class/Color3Value")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Color3Value {
    pub value_base: ValueBase,
    pub value: crate::model::data::Color3,
}
#[doc = doc_link!("class/DoubleConstrainedValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct DoubleConstrainedValue {
    pub value_base: ValueBase,
    pub max_value: f64,
    pub min_value: f64,
}
#[doc = doc_link!("class/IntConstrainedValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct IntConstrainedValue {
    pub value_base: ValueBase,
    pub max_value: i64,
    pub min_value: i64,
}
#[doc = doc_link!("class/IntValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct IntValue {
    pub value_base: ValueBase,
    pub value: i64,
}
#[doc = doc_link!("class/NumberValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct NumberValue {
    pub value_base: ValueBase,
    pub value: f64,
}
#[doc = doc_link!("class/ObjectValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct ObjectValue {
    pub value_base: ValueBase,
    pub value: crate::model::data::InstanceRef,
}
#[doc = doc_link!("class/RayValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct RayValue {
    pub value_base: ValueBase,
    pub value: crate::model::data::Ray,
}
#[doc = doc_link!("class/StringValue")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct StringValue {
    pub value_base: ValueBase,
    pub value: String,
}
#[doc = doc_link!("class/Vector3Value")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Vector3Value {
    pub value_base: ValueBase,
    pub value: crate::model::data::Vector3,
}
#[doc = doc_link!("class/Vector3Curve")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Vector3Curve {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VersionControlService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VersionControlService {
    pub base: crate::model::classes::Base,
    pub script_collab_enabled: bool,
}
#[doc = doc_link!("class/VideoCaptureService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VideoCaptureService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VirtualInputManager")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VirtualInputManager {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VirtualUser")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VirtualUser {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VisibilityCheckDispatcher")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VisibilityCheckDispatcher {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VisibilityService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VisibilityService {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/Visit")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct Visit {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VoiceChatInternal")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VoiceChatInternal {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/VoiceChatService")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VoiceChatService {
    pub base: crate::model::classes::Base,
    pub enable_default_voice: bool,
    pub voice_chat_enabled_for_place_on_rcc: bool,
    pub voice_chat_enabled_for_universe_on_rcc: bool,
}
#[doc = doc_link!("class/VoiceSource")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct VoiceSource {
    pub base: crate::model::classes::Base,
    
}
#[doc = doc_link!("class/WeldConstraint")]
#[derive(Debug, Clone, Inherits, PropertyConvert)]
#[non_exhaustive]
pub struct WeldConstraint {
    pub base: crate::model::classes::Base,
    
}
