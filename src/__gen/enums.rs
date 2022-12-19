use rbxm_proc::EnumConvert;
#[doc = doc_link!("enum/AccessoryType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AccessoryType {
    Unknown = 0,
    Hat = 1,
    Hair = 2,
    Face = 3,
    Neck = 4,
    Shoulder = 5,
    Front = 6,
    Back = 7,
    Waist = 8,
    TShirt = 9,
    Shirt = 10,
    Pants = 11,
    Jacket = 12,
    Sweater = 13,
    Shorts = 14,
    LeftShoe = 15,
    RightShoe = 16,
    DressSkirt = 17,
    Eyebrow = 18,
    Eyelash = 19,
}
#[doc = doc_link!("enum/ActionType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ActionType {
    Nothing = 0,
    Pause = 1,
    Lose = 2,
    Draw = 3,
    Win = 4,
}
#[doc = doc_link!("enum/ActuatorRelativeTo")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ActuatorRelativeTo {
    Attachment0 = 0,
    Attachment1 = 1,
    World = 2,
}
#[doc = doc_link!("enum/ActuatorType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ActuatorType {
    None = 0,
    Motor = 1,
    Servo = 2,
}
#[doc = doc_link!("enum/AdPortalStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AdPortalStatus {
    Invalid = 0,
    Inactive = 1,
    Active = 2,
}
#[doc = doc_link!("enum/AdPortalType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AdPortalType {
    Forward = 0,
    Return = 1,
}
#[doc = doc_link!("enum/AdShape")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AdShape {
    HorizontalRectangle = 1,
}
#[doc = doc_link!("enum/AdornCullingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AdornCullingMode {
    Automatic = 0,
    Never = 1,
}
#[doc = doc_link!("enum/AlignType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AlignType {
    Parallel = 0,
    Perpendicular = 1,
}
#[doc = doc_link!("enum/AlphaMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AlphaMode {
    Overlay = 0,
    Transparency = 1,
}
#[doc = doc_link!("enum/AnalyticsEconomyAction")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnalyticsEconomyAction {
    Default = 0,
    Acquire = 1,
    Spend = 2,
}
#[doc = doc_link!("enum/AnalyticsLogLevel")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnalyticsLogLevel {
    Trace = 0,
    Debug = 1,
    Information = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}
#[doc = doc_link!("enum/AnalyticsProgressionStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnalyticsProgressionStatus {
    Default = 0,
    Begin = 1,
    Complete = 2,
    Abandon = 3,
    Fail = 4,
}
#[doc = doc_link!("enum/AnimationPriority")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnimationPriority {
    Idle = 0,
    Movement = 1,
    Action = 2,
    Action2 = 3,
    Action3 = 4,
    Action4 = 5,
    Core = 1000,
}
#[doc = doc_link!("enum/AnimatorRetargetingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnimatorRetargetingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/AppShellActionType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AppShellActionType {
    None = 0,
    OpenApp = 1,
    TapChatTab = 2,
    TapConversationEntry = 3,
    TapAvatarTab = 4,
    ReadConversation = 5,
    TapGamePageTab = 6,
    TapHomePageTab = 7,
    GamePageLoaded = 8,
    HomePageLoaded = 9,
    AvatarEditorPageLoaded = 10,
}
#[doc = doc_link!("enum/AppShellFeature")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AppShellFeature {
    None = 0,
    Chat = 1,
    AvatarEditor = 2,
    GamePage = 3,
    HomePage = 4,
    More = 5,
    Landing = 6,
}
#[doc = doc_link!("enum/AppUpdateStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AppUpdateStatus {
    Unknown = 0,
    NotSupported = 1,
    Failed = 2,
    NotAvailable = 3,
    Available = 4,
}
#[doc = doc_link!("enum/ApplyStrokeMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ApplyStrokeMode {
    Contextual = 0,
    Border = 1,
}
#[doc = doc_link!("enum/AspectType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AspectType {
    FitWithinMaxSize = 0,
    ScaleWithParentSize = 1,
}
#[doc = doc_link!("enum/AssetFetchStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AssetFetchStatus {
    Success = 0,
    Failure = 1,
}
#[doc = doc_link!("enum/AssetType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AssetType {
    Image = 1,
    TShirt = 2,
    Audio = 3,
    Mesh = 4,
    Lua = 5,
    Hat = 8,
    Place = 9,
    Model = 10,
    Shirt = 11,
    Pants = 12,
    Decal = 13,
    Head = 17,
    Face = 18,
    Gear = 19,
    Badge = 21,
    Animation = 24,
    Torso = 27,
    RightArm = 28,
    LeftArm = 29,
    LeftLeg = 30,
    RightLeg = 31,
    Package = 32,
    GamePass = 34,
    Plugin = 38,
    MeshPart = 40,
    HairAccessory = 41,
    FaceAccessory = 42,
    NeckAccessory = 43,
    ShoulderAccessory = 44,
    FrontAccessory = 45,
    BackAccessory = 46,
    WaistAccessory = 47,
    ClimbAnimation = 48,
    DeathAnimation = 49,
    FallAnimation = 50,
    IdleAnimation = 51,
    JumpAnimation = 52,
    RunAnimation = 53,
    SwimAnimation = 54,
    WalkAnimation = 55,
    PoseAnimation = 56,
    MoodAnimation = 78,
    EarAccessory = 57,
    EyeAccessory = 58,
    EmoteAnimation = 61,
    Video = 62,
    TShirtAccessory = 64,
    ShirtAccessory = 65,
    PantsAccessory = 66,
    JacketAccessory = 67,
    SweaterAccessory = 68,
    ShortsAccessory = 69,
    LeftShoeAccessory = 70,
    RightShoeAccessory = 71,
    DressSkirtAccessory = 72,
    EyebrowAccessory = 76,
    EyelashAccessory = 77,
    DynamicHead = 79,
    FontFamily = 73,
}
#[doc = doc_link!("enum/AssetTypeVerification")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AssetTypeVerification {
    Default = 1,
    ClientOnly = 2,
    Always = 3,
}
#[doc = doc_link!("enum/AutoIndentRule")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AutoIndentRule {
    Off = 0,
    Absolute = 1,
    Relative = 2,
}
#[doc = doc_link!("enum/AutomaticSize")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AutomaticSize {
    None = 0,
    X = 1,
    Y = 2,
    XY = 3,
}
#[doc = doc_link!("enum/AvatarAssetType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AvatarAssetType {
    TShirt = 2,
    Hat = 8,
    HairAccessory = 41,
    FaceAccessory = 42,
    NeckAccessory = 43,
    ShoulderAccessory = 44,
    FrontAccessory = 45,
    BackAccessory = 46,
    WaistAccessory = 47,
    Shirt = 11,
    Pants = 12,
    Gear = 19,
    Head = 17,
    Face = 18,
    Torso = 27,
    RightArm = 28,
    LeftArm = 29,
    LeftLeg = 30,
    RightLeg = 31,
    ClimbAnimation = 48,
    FallAnimation = 50,
    IdleAnimation = 51,
    JumpAnimation = 52,
    RunAnimation = 53,
    SwimAnimation = 54,
    WalkAnimation = 55,
    MoodAnimation = 78,
    EmoteAnimation = 61,
    TShirtAccessory = 64,
    ShirtAccessory = 65,
    PantsAccessory = 66,
    JacketAccessory = 67,
    SweaterAccessory = 68,
    ShortsAccessory = 69,
    LeftShoeAccessory = 70,
    RightShoeAccessory = 71,
    DressSkirtAccessory = 72,
    EyebrowAccessory = 76,
    EyelashAccessory = 77,
    DynamicHead = 79,
}
#[doc = doc_link!("enum/AvatarContextMenuOption")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AvatarContextMenuOption {
    Friend = 0,
    Chat = 1,
    Emote = 2,
    InspectMenu = 3,
}
#[doc = doc_link!("enum/AvatarItemType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AvatarItemType {
    Asset = 1,
    Bundle = 2,
}
#[doc = doc_link!("enum/AvatarPromptResult")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AvatarPromptResult {
    Success = 1,
    PermissionDenied = 2,
    Failed = 3,
}
#[doc = doc_link!("enum/Axis")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}
#[doc = doc_link!("enum/BinType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BinType {
    Script = 0,
    GameTool = 1,
    Grab = 2,
    Clone = 3,
    Hammer = 4,
}
#[doc = doc_link!("enum/BodyPart")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BodyPart {
    Head = 0,
    Torso = 1,
    LeftArm = 2,
    RightArm = 3,
    LeftLeg = 4,
    RightLeg = 5,
}
#[doc = doc_link!("enum/BodyPartR15")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BodyPartR15 {
    Head = 0,
    UpperTorso = 1,
    LowerTorso = 2,
    LeftFoot = 3,
    LeftLowerLeg = 4,
    LeftUpperLeg = 5,
    RightFoot = 6,
    RightLowerLeg = 7,
    RightUpperLeg = 8,
    LeftHand = 9,
    LeftLowerArm = 10,
    LeftUpperArm = 11,
    RightHand = 12,
    RightLowerArm = 13,
    RightUpperArm = 14,
    RootPart = 15,
    Unknown = 17,
}
#[doc = doc_link!("enum/BorderMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BorderMode {
    Outline = 0,
    Middle = 1,
    Inset = 2,
}
#[doc = doc_link!("enum/BreakReason")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BreakReason {
    Other = 0,
    Error = 1,
    UserBreakpoint = 3,
    SpecialBreakpoint = 2,
}
#[doc = doc_link!("enum/BreakpointRemoveReason")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BreakpointRemoveReason {
    Requested = 0,
    ScriptChanged = 1,
    ScriptRemoved = 2,
}
#[doc = doc_link!("enum/BulkMoveMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BulkMoveMode {
    FireAllEvents = 0,
    FireCFrameChanged = 1,
}
#[doc = doc_link!("enum/BundleType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BundleType {
    BodyParts = 1,
    Animations = 2,
    Shoes = 3,
    DynamicHead = 4,
    DynamicHeadAvatar = 5,
}
#[doc = doc_link!("enum/Button")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Button {
    Jump = 32,
    Dismount = 8,
}
#[doc = doc_link!("enum/ButtonStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ButtonStyle {
    Custom = 0,
    RobloxButtonDefault = 1,
    RobloxButton = 2,
    RobloxRoundButton = 3,
    RobloxRoundDefaultButton = 4,
    RobloxRoundDropdownButton = 5,
}
#[doc = doc_link!("enum/CageType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CageType {
    Inner = 0,
    Outer = 1,
}
#[doc = doc_link!("enum/CameraMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CameraMode {
    Classic = 0,
    LockFirstPerson = 1,
}
#[doc = doc_link!("enum/CameraPanMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CameraPanMode {
    Classic = 0,
    EdgeBump = 1,
}
#[doc = doc_link!("enum/CameraType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CameraType {
    Fixed = 0,
    Watch = 2,
    Attach = 1,
    Track = 3,
    Follow = 4,
    Custom = 5,
    Scriptable = 6,
    Orbital = 7,
}
#[doc = doc_link!("enum/CatalogCategoryFilter")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CatalogCategoryFilter {
    None = 1,
    Featured = 2,
    Collectibles = 3,
    CommunityCreations = 4,
    Premium = 5,
    Recommended = 6,
}
#[doc = doc_link!("enum/CatalogSortType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CatalogSortType {
    Relevance = 1,
    PriceHighToLow = 2,
    PriceLowToHigh = 3,
    RecentlyUpdated = 4,
    MostFavorited = 5,
}
#[doc = doc_link!("enum/CellBlock")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CellBlock {
    Solid = 0,
    VerticalWedge = 1,
    CornerWedge = 2,
    InverseCornerWedge = 3,
    HorizontalWedge = 4,
}
#[doc = doc_link!("enum/CellMaterial")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CellMaterial {
    Empty = 0,
    Grass = 1,
    Sand = 2,
    Brick = 3,
    Granite = 4,
    Asphalt = 5,
    Iron = 6,
    Aluminum = 7,
    Gold = 8,
    WoodPlank = 9,
    WoodLog = 10,
    Gravel = 11,
    CinderBlock = 12,
    MossyStone = 13,
    Cement = 14,
    RedPlastic = 15,
    BluePlastic = 16,
    Water = 17,
}
#[doc = doc_link!("enum/CellOrientation")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CellOrientation {
    NegZ = 0,
    X = 1,
    Z = 2,
    NegX = 3,
}
#[doc = doc_link!("enum/CenterDialogType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CenterDialogType {
    UnsolicitedDialog = 1,
    PlayerInitiatedDialog = 2,
    ModalDialog = 3,
    QuitDialog = 4,
}
#[doc = doc_link!("enum/ChatCallbackType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ChatCallbackType {
    OnCreatingChatWindow = 1,
    OnClientSendingMessage = 2,
    OnClientFormattingMessage = 3,
    OnServerReceivingMessage = 17,
}
#[doc = doc_link!("enum/ChatColor")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ChatColor {
    Blue = 0,
    Green = 1,
    Red = 2,
    White = 3,
}
#[doc = doc_link!("enum/ChatMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ChatMode {
    Menu = 0,
    TextAndMenu = 1,
}
#[doc = doc_link!("enum/ChatPrivacyMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ChatPrivacyMode {
    AllUsers = 0,
    NoOne = 1,
    Friends = 2,
}
#[doc = doc_link!("enum/ChatStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ChatStyle {
    Classic = 0,
    Bubble = 1,
    ClassicAndBubble = 2,
}
#[doc = doc_link!("enum/ChatVersion")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ChatVersion {
    LegacyChatService = 0,
    TextChatService = 1,
}
#[doc = doc_link!("enum/ClientAnimatorThrottlingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ClientAnimatorThrottlingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/CollisionFidelity")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CollisionFidelity {
    Default = 0,
    Hull = 1,
    Box = 2,
    PreciseConvexDecomposition = 3,
    DynamicPreciseConvexDecomposition = 4,
}
#[doc = doc_link!("enum/CommandPermission")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CommandPermission {
    Plugin = 0,
    LocalUser = 1,
}
#[doc = doc_link!("enum/CompletionItemKind")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}
#[doc = doc_link!("enum/CompletionItemTag")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CompletionItemTag {
    Deprecated = 1,
    IncorrectIndexType = 2,
    PluginPermissions = 3,
    CommandLinePermissions = 4,
    RobloxPermissions = 5,
    AddParens = 6,
    PutCursorInParens = 7,
    TypeCorrect = 8,
    ClientServerBoundaryViolation = 9,
}
#[doc = doc_link!("enum/CompletionTriggerKind")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CompletionTriggerKind {
    Invoked = 1,
    TriggerCharacter = 2,
    TriggerForIncompleteCompletions = 3,
}
#[doc = doc_link!("enum/ComputerCameraMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ComputerCameraMovementMode {
    Default = 0,
    Follow = 2,
    Classic = 1,
    Orbital = 3,
    CameraToggle = 4,
}
#[doc = doc_link!("enum/ComputerMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ComputerMovementMode {
    Default = 0,
    KeyboardMouse = 1,
    ClickToMove = 2,
}
#[doc = doc_link!("enum/ConnectionError")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ConnectionError {
    OK = 0,
    Unknown = 1,
    DisconnectErrors = 256,
    DisconnectBadhash = 257,
    DisconnectSecurityKeyMismatch = 258,
    DisconnectNewSecurityKeyMismatch = 272,
    DisconnectProtocolMismatch = 259,
    DisconnectReceivePacketError = 260,
    DisconnectReceivePacketStreamError = 261,
    DisconnectSendPacketError = 262,
    DisconnectIllegalTeleport = 263,
    DisconnectDuplicatePlayer = 264,
    DisconnectDuplicateTicket = 265,
    DisconnectTimeout = 266,
    DisconnectLuaKick = 267,
    DisconnectOnRemoteSysStats = 268,
    DisconnectHashTimeout = 269,
    DisconnectCloudEditKick = 270,
    DisconnectPlayerless = 271,
    DisconnectEvicted = 273,
    DisconnectDevMaintenance = 274,
    DisconnectRobloxMaintenance = 275,
    DisconnectRejoin = 276,
    DisconnectConnectionLost = 277,
    DisconnectIdle = 278,
    DisconnectRaknetErrors = 279,
    DisconnectWrongVersion = 280,
    DisconnectBySecurityPolicy = 281,
    DisconnectBlockedIP = 282,
    DisconnectClientFailure = 284,
    DisconnectClientRequest = 285,
    DisconnectOutOfMemory = 286,
    DisconnectModeratedGame = 287,
    DisconnectOutOfMemoryExitContinue = 288,
    DisconnectOutOfMemoryKeepPlayingExit = 289,
    ReplicatorTimeout = 290,
    PlayerRemoved = 291,
    PlacelaunchErrors = 512,
    PlacelaunchDisabled = 515,
    PlacelaunchError = 516,
    PlacelaunchGameEnded = 517,
    PlacelaunchGameFull = 518,
    PlacelaunchUserLeft = 522,
    PlacelaunchRestricted = 523,
    PlacelaunchUnauthorized = 524,
    PlacelaunchFlooded = 525,
    PlacelaunchHashExpired = 526,
    PlacelaunchHashException = 527,
    PlacelaunchPartyCannotFit = 528,
    PlacelaunchHttpError = 529,
    PlacelaunchUserPrivacyUnauthorized = 533,
    PlacelaunchCustomMessage = 610,
    PlacelaunchOtherError = 611,
    TeleportErrors = 768,
    TeleportFailure = 769,
    TeleportGameNotFound = 770,
    TeleportGameEnded = 771,
    TeleportGameFull = 772,
    TeleportUnauthorized = 773,
    TeleportFlooded = 774,
    TeleportIsTeleporting = 775,
}
#[doc = doc_link!("enum/ConnectionState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ConnectionState {
    Connected = 0,
    Disconnected = 1,
}
#[doc = doc_link!("enum/ContextActionPriority")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ContextActionPriority {
    Low = 1000,
    Medium = 2000,
    High = 3000,
}
#[doc = doc_link!("enum/ContextActionResult")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ContextActionResult {
    Pass = 1,
    Sink = 0,
}
#[doc = doc_link!("enum/ControlMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ControlMode {
    MouseLockSwitch = 1,
    Classic = 0,
}
#[doc = doc_link!("enum/CoreGuiType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CoreGuiType {
    PlayerList = 0,
    Health = 1,
    Backpack = 2,
    Chat = 3,
    All = 4,
    EmotesMenu = 5,
    SelfView = 6,
}
#[doc = doc_link!("enum/CreateOutfitFailure")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CreateOutfitFailure {
    InvalidName = 1,
    OutfitLimitReached = 2,
    Other = 3,
}
#[doc = doc_link!("enum/CreatorType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CreatorType {
    User = 0,
    Group = 1,
}
#[doc = doc_link!("enum/CurrencyType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CurrencyType {
    Default = 0,
    Robux = 1,
    Tix = 2,
}
#[doc = doc_link!("enum/CustomCameraMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CustomCameraMode {
    Default = 0,
    Follow = 2,
    Classic = 1,
}
#[doc = doc_link!("enum/DataStoreRequestType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DataStoreRequestType {
    GetAsync = 0,
    SetIncrementAsync = 1,
    UpdateAsync = 2,
    GetSortedAsync = 3,
    SetIncrementSortedAsync = 4,
    OnUpdate = 5,
}
#[doc = doc_link!("enum/DebuggerEndReason")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DebuggerEndReason {
    ClientRequest = 0,
    Timeout = 1,
    InvalidHost = 2,
    Disconnected = 3,
    ServerShutdown = 4,
    ServerProtocolMismatch = 5,
    ConfigurationFailed = 6,
    RpcError = 7,
}
#[doc = doc_link!("enum/DebuggerExceptionBreakMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DebuggerExceptionBreakMode {
    Never = 0,
    Unhandled = 2,
    Always = 1,
}
#[doc = doc_link!("enum/DebuggerFrameType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DebuggerFrameType {
    C = 0,
    Lua = 1,
}
#[doc = doc_link!("enum/DebuggerPauseReason")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DebuggerPauseReason {
    Unknown = 0,
    Requested = 1,
    Breakpoint = 2,
    Exception = 3,
    SingleStep = 4,
    Entrypoint = 5,
}
#[doc = doc_link!("enum/DebuggerStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DebuggerStatus {
    Success = 0,
    Timeout = 1,
    ConnectionLost = 2,
    InvalidResponse = 3,
    InternalError = 4,
    InvalidState = 5,
    RpcError = 6,
    InvalidArgument = 7,
    ConnectionClosed = 8,
}
#[doc = doc_link!("enum/DevCameraOcclusionMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevCameraOcclusionMode {
    Zoom = 0,
    Invisicam = 1,
}
#[doc = doc_link!("enum/DevComputerCameraMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevComputerCameraMovementMode {
    UserChoice = 0,
    Classic = 1,
    Follow = 2,
    Orbital = 3,
    CameraToggle = 4,
}
#[doc = doc_link!("enum/DevComputerMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevComputerMovementMode {
    UserChoice = 0,
    KeyboardMouse = 1,
    ClickToMove = 2,
    Scriptable = 3,
}
#[doc = doc_link!("enum/DevTouchCameraMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevTouchCameraMovementMode {
    UserChoice = 0,
    Classic = 1,
    Follow = 2,
    Orbital = 3,
}
#[doc = doc_link!("enum/DevTouchMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevTouchMovementMode {
    UserChoice = 0,
    Thumbstick = 1,
    DPad = 2,
    Thumbpad = 3,
    ClickToMove = 4,
    Scriptable = 5,
    DynamicThumbstick = 6,
}
#[doc = doc_link!("enum/DeveloperMemoryTag")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DeveloperMemoryTag {
    Internal = 0,
    HttpCache = 1,
    Instances = 2,
    Signals = 3,
    LuaHeap = 4,
    Script = 5,
    PhysicsCollision = 6,
    PhysicsParts = 7,
    GraphicsSolidModels = 8,
    GraphicsMeshParts = 10,
    GraphicsParticles = 11,
    GraphicsParts = 12,
    GraphicsSpatialHash = 13,
    GraphicsTerrain = 14,
    GraphicsTexture = 15,
    GraphicsTextureCharacter = 16,
    Sounds = 17,
    StreamingSounds = 18,
    TerrainVoxels = 19,
    Gui = 21,
    Animation = 22,
    Navigation = 23,
    GeometryCSG = 24,
}
#[doc = doc_link!("enum/DeviceType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DeviceType {
    Unknown = 0,
    Desktop = 1,
    Tablet = 2,
    Phone = 3,
}
#[doc = doc_link!("enum/DialogBehaviorType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogBehaviorType {
    SinglePlayer = 0,
    MultiplePlayers = 1,
}
#[doc = doc_link!("enum/DialogPurpose")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogPurpose {
    Quest = 0,
    Help = 1,
    Shop = 2,
}
#[doc = doc_link!("enum/DialogTone")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogTone {
    Neutral = 0,
    Friendly = 1,
    Enemy = 2,
}
#[doc = doc_link!("enum/DominantAxis")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DominantAxis {
    Width = 0,
    Height = 1,
}
#[doc = doc_link!("enum/DraftStatusCode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DraftStatusCode {
    OK = 0,
    DraftOutdated = 1,
    ScriptRemoved = 2,
    DraftCommitted = 3,
}
#[doc = doc_link!("enum/DraggerCoordinateSpace")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DraggerCoordinateSpace {
    Object = 0,
    World = 1,
}
#[doc = doc_link!("enum/DraggerMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DraggerMovementMode {
    Geometric = 0,
    Physical = 1,
}
#[doc = doc_link!("enum/EasingDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum EasingDirection {
    In = 0,
    Out = 1,
    InOut = 2,
}
#[doc = doc_link!("enum/EasingStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum EasingStyle {
    Linear = 0,
    Sine = 1,
    Back = 2,
    Quad = 3,
    Quart = 4,
    Quint = 5,
    Bounce = 6,
    Elastic = 7,
    Exponential = 8,
    Circular = 9,
    Cubic = 10,
}
#[doc = doc_link!("enum/ElasticBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ElasticBehavior {
    WhenScrollable = 0,
    Always = 1,
    Never = 2,
}
#[doc = doc_link!("enum/EnviromentalPhysicsThrottle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum EnviromentalPhysicsThrottle {
    DefaultAuto = 0,
    Disabled = 1,
    Always = 2,
    Skip2 = 3,
    Skip4 = 4,
    Skip8 = 5,
    Skip16 = 6,
}
#[doc = doc_link!("enum/ExperienceAuthScope")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ExperienceAuthScope {
    
}
#[doc = doc_link!("enum/ExplosionType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ExplosionType {
    NoCraters = 0,
    Craters = 1,
}
#[doc = doc_link!("enum/FacialAnimationFlags")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FacialAnimationFlags {
    None = 0,
    Place = 1,
    Server = 2,
    PlaceServer = 3,
}
#[doc = doc_link!("enum/FacialAnimationStreamingState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FacialAnimationStreamingState {
    None = 0,
    Audio = 1,
    Video = 2,
    Place = 4,
    Server = 8,
}
#[doc = doc_link!("enum/FieldOfViewMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FieldOfViewMode {
    Vertical = 0,
    Diagonal = 1,
    MaxAxis = 2,
}
#[doc = doc_link!("enum/FillDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FillDirection {
    Horizontal = 0,
    Vertical = 1,
}
#[doc = doc_link!("enum/FilterResult")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FilterResult {
    Rejected = 1,
    Accepted = 0,
}
#[doc = doc_link!("enum/Font")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Font {
    Legacy = 0,
    Arial = 1,
    ArialBold = 2,
    SourceSans = 3,
    SourceSansBold = 4,
    SourceSansSemibold = 16,
    SourceSansLight = 5,
    SourceSansItalic = 6,
    Bodoni = 7,
    Garamond = 8,
    Cartoon = 9,
    Code = 10,
    Highway = 11,
    SciFi = 12,
    Arcade = 13,
    Fantasy = 14,
    Antique = 15,
    Gotham = 17,
    GothamMedium = 18,
    GothamBold = 19,
    GothamBlack = 20,
    AmaticSC = 21,
    Bangers = 22,
    Creepster = 23,
    DenkOne = 24,
    Fondamento = 25,
    FredokaOne = 26,
    GrenzeGotisch = 27,
    IndieFlower = 28,
    JosefinSans = 29,
    Jura = 30,
    Kalam = 31,
    LuckiestGuy = 32,
    Merriweather = 33,
    Michroma = 34,
    Nunito = 35,
    Oswald = 36,
    PatrickHand = 37,
    PermanentMarker = 38,
    Roboto = 39,
    RobotoCondensed = 40,
    RobotoMono = 41,
    Sarpanch = 42,
    SpecialElite = 43,
    TitilliumWeb = 44,
    Ubuntu = 45,
    Unknown = 100,
}
#[doc = doc_link!("enum/FontSize")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FontSize {
    Size8 = 0,
    Size9 = 1,
    Size10 = 2,
    Size11 = 3,
    Size12 = 4,
    Size14 = 5,
    Size18 = 6,
    Size24 = 7,
    Size36 = 8,
    Size48 = 9,
    Size28 = 10,
    Size32 = 11,
    Size42 = 12,
    Size60 = 13,
    Size96 = 14,
}
#[doc = doc_link!("enum/FontStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FontStyle {
    Normal = 0,
    Italic = 1,
}
#[doc = doc_link!("enum/FontWeight")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Heavy = 900,
}
#[doc = doc_link!("enum/FormFactor")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FormFactor {
    Symmetric = 0,
    Brick = 1,
    Plate = 2,
    Custom = 3,
}
#[doc = doc_link!("enum/FrameStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FrameStyle {
    Custom = 0,
    ChatBlue = 1,
    RobloxSquare = 2,
    RobloxRound = 3,
    ChatGreen = 4,
    ChatRed = 5,
    DropShadow = 6,
}
#[doc = doc_link!("enum/FramerateManagerMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FramerateManagerMode {
    Automatic = 0,
    On = 1,
    Off = 2,
}
#[doc = doc_link!("enum/FriendRequestEvent")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FriendRequestEvent {
    Issue = 0,
    Revoke = 1,
    Accept = 2,
    Deny = 3,
}
#[doc = doc_link!("enum/FriendStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FriendStatus {
    Unknown = 0,
    NotFriend = 1,
    Friend = 2,
    FriendRequestSent = 3,
    FriendRequestReceived = 4,
}
#[doc = doc_link!("enum/FunctionalTestResult")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FunctionalTestResult {
    Passed = 0,
    Warning = 1,
    Error = 2,
}
#[doc = doc_link!("enum/GameAvatarType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum GameAvatarType {
    R6 = 0,
    R15 = 1,
    PlayerChoice = 2,
}
#[doc = doc_link!("enum/GearGenreSetting")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum GearGenreSetting {
    AllGenres = 0,
    MatchingGenreOnly = 1,
}
#[doc = doc_link!("enum/GearType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum GearType {
    MeleeWeapons = 0,
    RangedWeapons = 1,
    Explosives = 2,
    PowerUps = 3,
    NavigationEnhancers = 4,
    MusicalInstruments = 5,
    SocialItems = 6,
    BuildingTools = 7,
    Transport = 8,
}
#[doc = doc_link!("enum/Genre")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Genre {
    All = 0,
    TownAndCity = 1,
    Fantasy = 2,
    SciFi = 3,
    Ninja = 4,
    Scary = 5,
    Pirate = 6,
    Adventure = 7,
    Sports = 8,
    Funny = 9,
    WildWest = 10,
    War = 11,
    SkatePark = 12,
    Tutorial = 13,
}
#[doc = doc_link!("enum/GraphicsMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum GraphicsMode {
    Automatic = 1,
    Direct3D11 = 2,
    OpenGL = 4,
    Metal = 5,
    Vulkan = 6,
    NoGraphics = 9,
}
#[doc = doc_link!("enum/GuiType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum GuiType {
    Core = 0,
    Custom = 1,
    CustomBillboards = 3,
    PlayerNameplates = 2,
}
#[doc = doc_link!("enum/HandlesStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HandlesStyle {
    Resize = 0,
    Movement = 1,
}
#[doc = doc_link!("enum/HighlightDepthMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HighlightDepthMode {
    AlwaysOnTop = 0,
    Occluded = 1,
}
#[doc = doc_link!("enum/HorizontalAlignment")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HorizontalAlignment {
    Center = 0,
    Left = 1,
    Right = 2,
}
#[doc = doc_link!("enum/HoverAnimateSpeed")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HoverAnimateSpeed {
    VerySlow = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
    VeryFast = 4,
}
#[doc = doc_link!("enum/HttpCachePolicy")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HttpCachePolicy {
    None = 0,
    Full = 1,
    DataOnly = 2,
    Default = 3,
    InternalRedirectRefresh = 4,
}
#[doc = doc_link!("enum/HttpContentType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HttpContentType {
    ApplicationJson = 0,
    ApplicationXml = 1,
    ApplicationUrlEncoded = 2,
    TextPlain = 3,
    TextXml = 4,
}
#[doc = doc_link!("enum/HttpError")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HttpError {
    OK = 0,
    InvalidUrl = 1,
    DnsResolve = 2,
    ConnectFail = 3,
    OutOfMemory = 4,
    TimedOut = 5,
    TooManyRedirects = 6,
    InvalidRedirect = 7,
    NetFail = 8,
    Aborted = 9,
    SslConnectFail = 10,
    SslVerificationFail = 11,
    Unknown = 12,
}
#[doc = doc_link!("enum/HttpRequestType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HttpRequestType {
    Default = 0,
    MarketplaceService = 2,
    Players = 7,
    Chat = 15,
    Avatar = 16,
    Analytics = 23,
    Localization = 25,
}
#[doc = doc_link!("enum/HumanoidCollisionType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidCollisionType {
    OuterBox = 0,
    InnerBox = 1,
}
#[doc = doc_link!("enum/HumanoidDisplayDistanceType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidDisplayDistanceType {
    Viewer = 0,
    Subject = 1,
    None = 2,
}
#[doc = doc_link!("enum/HumanoidHealthDisplayType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidHealthDisplayType {
    DisplayWhenDamaged = 0,
    AlwaysOn = 1,
    AlwaysOff = 2,
}
#[doc = doc_link!("enum/HumanoidOnlySetCollisionsOnStateChange")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidOnlySetCollisionsOnStateChange {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/HumanoidRigType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidRigType {
    R6 = 0,
    R15 = 1,
}
#[doc = doc_link!("enum/HumanoidStateMachineMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidStateMachineMode {
    Default = 0,
    Legacy = 1,
    NoStateMachine = 2,
    LuaStateMachine = 3,
}
#[doc = doc_link!("enum/HumanoidStateType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidStateType {
    FallingDown = 0,
    Running = 8,
    RunningNoPhysics = 10,
    Climbing = 12,
    StrafingNoPhysics = 11,
    Ragdoll = 1,
    GettingUp = 2,
    Jumping = 3,
    Landed = 7,
    Flying = 6,
    Freefall = 5,
    Seated = 13,
    PlatformStanding = 14,
    Dead = 15,
    Swimming = 4,
    Physics = 16,
    None = 18,
}
#[doc = doc_link!("enum/IKCollisionsMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum IKCollisionsMode {
    NoCollisions = 0,
    OtherMechanismsAnchored = 1,
    IncludeContactedMechanisms = 2,
}
#[doc = doc_link!("enum/IKControlType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum IKControlType {
    Transform = 0,
    Position = 1,
    Rotation = 2,
    LookAt = 3,
}
#[doc = doc_link!("enum/IXPLoadingStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum IXPLoadingStatus {
    None = 0,
    Pending = 1,
    Initialized = 2,
    ErrorTimedOut = 6,
    ErrorConnection = 4,
    ErrorJsonParse = 5,
    ErrorInvalidUser = 3,
}
#[doc = doc_link!("enum/InOut")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InOut {
    Edge = 0,
    Inset = 1,
    Center = 2,
}
#[doc = doc_link!("enum/InfoType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InfoType {
    Asset = 0,
    Product = 1,
    GamePass = 2,
    Subscription = 3,
    Bundle = 4,
}
#[doc = doc_link!("enum/InitialDockState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InitialDockState {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
    Float = 4,
}
#[doc = doc_link!("enum/InputType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InputType {
    NoInput = 0,
    Constant = 12,
    Sin = 13,
}
#[doc = doc_link!("enum/InterpolationThrottlingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InterpolationThrottlingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/JointCreationMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum JointCreationMode {
    All = 0,
    Surface = 1,
    None = 2,
}
#[doc = doc_link!("enum/KeyCode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum KeyCode {
    Unknown = 0,
    Backspace = 8,
    Tab = 9,
    Clear = 12,
    Return = 13,
    Pause = 19,
    Escape = 27,
    Space = 32,
    QuotedDouble = 34,
    Hash = 35,
    Dollar = 36,
    Percent = 37,
    Ampersand = 38,
    Quote = 39,
    LeftParenthesis = 40,
    RightParenthesis = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Colon = 58,
    Semicolon = 59,
    LessThan = 60,
    Equals = 61,
    GreaterThan = 62,
    Question = 63,
    At = 64,
    LeftBracket = 91,
    BackSlash = 92,
    RightBracket = 93,
    Caret = 94,
    Underscore = 95,
    Backquote = 96,
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    LeftCurly = 123,
    Pipe = 124,
    RightCurly = 125,
    Tilde = 126,
    Delete = 127,
    KeypadZero = 256,
    KeypadOne = 257,
    KeypadTwo = 258,
    KeypadThree = 259,
    KeypadFour = 260,
    KeypadFive = 261,
    KeypadSix = 262,
    KeypadSeven = 263,
    KeypadEight = 264,
    KeypadNine = 265,
    KeypadPeriod = 266,
    KeypadDivide = 267,
    KeypadMultiply = 268,
    KeypadMinus = 269,
    KeypadPlus = 270,
    KeypadEnter = 271,
    KeypadEquals = 272,
    Up = 273,
    Down = 274,
    Right = 275,
    Left = 276,
    Insert = 277,
    Home = 278,
    End = 279,
    PageUp = 280,
    PageDown = 281,
    LeftShift = 304,
    RightShift = 303,
    LeftMeta = 310,
    RightMeta = 309,
    LeftAlt = 308,
    RightAlt = 307,
    LeftControl = 306,
    RightControl = 305,
    CapsLock = 301,
    NumLock = 300,
    ScrollLock = 302,
    LeftSuper = 311,
    RightSuper = 312,
    Mode = 313,
    Compose = 314,
    Help = 315,
    Print = 316,
    SysReq = 317,
    Break = 318,
    Menu = 319,
    Power = 320,
    Euro = 321,
    Undo = 322,
    F1 = 282,
    F2 = 283,
    F3 = 284,
    F4 = 285,
    F5 = 286,
    F6 = 287,
    F7 = 288,
    F8 = 289,
    F9 = 290,
    F10 = 291,
    F11 = 292,
    F12 = 293,
    F13 = 294,
    F14 = 295,
    F15 = 296,
    World0 = 160,
    World1 = 161,
    World2 = 162,
    World3 = 163,
    World4 = 164,
    World5 = 165,
    World6 = 166,
    World7 = 167,
    World8 = 168,
    World9 = 169,
    World10 = 170,
    World11 = 171,
    World12 = 172,
    World13 = 173,
    World14 = 174,
    World15 = 175,
    World16 = 176,
    World17 = 177,
    World18 = 178,
    World19 = 179,
    World20 = 180,
    World21 = 181,
    World22 = 182,
    World23 = 183,
    World24 = 184,
    World25 = 185,
    World26 = 186,
    World27 = 187,
    World28 = 188,
    World29 = 189,
    World30 = 190,
    World31 = 191,
    World32 = 192,
    World33 = 193,
    World34 = 194,
    World35 = 195,
    World36 = 196,
    World37 = 197,
    World38 = 198,
    World39 = 199,
    World40 = 200,
    World41 = 201,
    World42 = 202,
    World43 = 203,
    World44 = 204,
    World45 = 205,
    World46 = 206,
    World47 = 207,
    World48 = 208,
    World49 = 209,
    World50 = 210,
    World51 = 211,
    World52 = 212,
    World53 = 213,
    World54 = 214,
    World55 = 215,
    World56 = 216,
    World57 = 217,
    World58 = 218,
    World59 = 219,
    World60 = 220,
    World61 = 221,
    World62 = 222,
    World63 = 223,
    World64 = 224,
    World65 = 225,
    World66 = 226,
    World67 = 227,
    World68 = 228,
    World69 = 229,
    World70 = 230,
    World71 = 231,
    World72 = 232,
    World73 = 233,
    World74 = 234,
    World75 = 235,
    World76 = 236,
    World77 = 237,
    World78 = 238,
    World79 = 239,
    World80 = 240,
    World81 = 241,
    World82 = 242,
    World83 = 243,
    World84 = 244,
    World85 = 245,
    World86 = 246,
    World87 = 247,
    World88 = 248,
    World89 = 249,
    World90 = 250,
    World91 = 251,
    World92 = 252,
    World93 = 253,
    World94 = 254,
    World95 = 255,
    ButtonX = 1000,
    ButtonY = 1001,
    ButtonA = 1002,
    ButtonB = 1003,
    ButtonR1 = 1004,
    ButtonL1 = 1005,
    ButtonR2 = 1006,
    ButtonL2 = 1007,
    ButtonR3 = 1008,
    ButtonL3 = 1009,
    ButtonStart = 1010,
    ButtonSelect = 1011,
    DPadLeft = 1012,
    DPadRight = 1013,
    DPadUp = 1014,
    DPadDown = 1015,
    Thumbstick1 = 1016,
    Thumbstick2 = 1017,
}
#[doc = doc_link!("enum/KeyInterpolationMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum KeyInterpolationMode {
    Constant = 0,
    Linear = 1,
    Cubic = 2,
}
#[doc = doc_link!("enum/KeywordFilterType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum KeywordFilterType {
    Include = 0,
    Exclude = 1,
}
#[doc = doc_link!("enum/Language")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Language {
    Default = 0,
}
#[doc = doc_link!("enum/LeftRight")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LeftRight {
    Left = 0,
    Center = 1,
    Right = 2,
}
#[doc = doc_link!("enum/LevelOfDetailSetting")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LevelOfDetailSetting {
    High = 2,
    Medium = 1,
    Low = 0,
}
#[doc = doc_link!("enum/Limb")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Limb {
    Head = 0,
    Torso = 1,
    LeftArm = 2,
    RightArm = 3,
    LeftLeg = 4,
    RightLeg = 5,
    Unknown = 6,
}
#[doc = doc_link!("enum/LineJoinMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LineJoinMode {
    Round = 0,
    Bevel = 1,
    Miter = 2,
}
#[doc = doc_link!("enum/ListDisplayMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ListDisplayMode {
    Horizontal = 0,
    Vertical = 1,
}
#[doc = doc_link!("enum/ListenerType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ListenerType {
    Camera = 0,
    CFrame = 1,
    ObjectPosition = 2,
    ObjectCFrame = 3,
}
#[doc = doc_link!("enum/LoadCharacterLayeredClothing")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LoadCharacterLayeredClothing {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/LoadDynamicHeads")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LoadDynamicHeads {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/MarkupKind")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MarkupKind {
    PlainText = 0,
    Markdown = 1,
}
#[doc = doc_link!("enum/Material")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Material {
    Plastic = 256,
    Wood = 512,
    Slate = 800,
    Concrete = 816,
    CorrodedMetal = 1040,
    DiamondPlate = 1056,
    Foil = 1072,
    Grass = 1280,
    Ice = 1536,
    Marble = 784,
    Granite = 832,
    Brick = 848,
    Pebble = 864,
    Sand = 1296,
    Fabric = 1312,
    SmoothPlastic = 272,
    Metal = 1088,
    WoodPlanks = 528,
    Cobblestone = 880,
    Air = 1792,
    Water = 2048,
    Rock = 896,
    Glacier = 1552,
    Snow = 1328,
    Sandstone = 912,
    Mud = 1344,
    Basalt = 788,
    Ground = 1360,
    CrackedLava = 804,
    Neon = 288,
    Glass = 1568,
    Asphalt = 1376,
    LeafyGrass = 1284,
    Salt = 1392,
    Limestone = 820,
    Pavement = 836,
    ForceField = 1584,
}
#[doc = doc_link!("enum/MaterialPattern")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MaterialPattern {
    Regular = 0,
    Organic = 1,
}
#[doc = doc_link!("enum/MembershipType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MembershipType {
    None = 0,
    BuildersClub = 1,
    TurboBuildersClub = 2,
    OutrageousBuildersClub = 3,
    Premium = 4,
}
#[doc = doc_link!("enum/MeshPartDetailLevel")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MeshPartDetailLevel {
    DistanceBased = 0,
    Level00 = 1,
    Level01 = 2,
    Level02 = 3,
    Level03 = 4,
    Level04 = 5,
}
#[doc = doc_link!("enum/MeshPartHeadsAndAccessories")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MeshPartHeadsAndAccessories {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/MeshScaleUnit")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MeshScaleUnit {
    Stud = 0,
    Meter = 1,
    CM = 2,
    MM = 3,
    Foot = 4,
    Inch = 5,
}
#[doc = doc_link!("enum/MeshType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MeshType {
    Head = 0,
    Torso = 1,
    Wedge = 2,
    Prism = 7,
    Pyramid = 8,
    ParallelRamp = 9,
    RightAngleRamp = 10,
    CornerWedge = 11,
    Brick = 6,
    Sphere = 3,
    Cylinder = 4,
    FileMesh = 5,
}
#[doc = doc_link!("enum/MessageType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MessageType {
    MessageOutput = 0,
    MessageInfo = 1,
    MessageWarning = 2,
    MessageError = 3,
}
#[doc = doc_link!("enum/ModelLevelOfDetail")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ModelLevelOfDetail {
    Automatic = 0,
    StreamingMesh = 1,
    Disabled = 2,
}
#[doc = doc_link!("enum/ModelStreamingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ModelStreamingMode {
    Default = 0,
    Atomic = 1,
    Persistent = 2,
    PersistentPerPlayer = 3,
}
#[doc = doc_link!("enum/ModifierKey")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ModifierKey {
    Alt = 2,
    Ctrl = 1,
    Meta = 3,
    Shift = 0,
}
#[doc = doc_link!("enum/MouseBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MouseBehavior {
    Default = 0,
    LockCenter = 1,
    LockCurrentPosition = 2,
}
#[doc = doc_link!("enum/MoveState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MoveState {
    Stopped = 0,
    Coasting = 1,
    Pushing = 2,
    Stopping = 3,
    AirFree = 4,
}
#[doc = doc_link!("enum/NameOcclusion")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NameOcclusion {
    OccludeAll = 2,
    EnemyOcclusion = 1,
    NoOcclusion = 0,
}
#[doc = doc_link!("enum/NetworkOwnership")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NetworkOwnership {
    Automatic = 0,
    Manual = 1,
    OnContact = 2,
}
#[doc = doc_link!("enum/NewAnimationRuntimeSetting")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NewAnimationRuntimeSetting {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/NormalId")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NormalId {
    Top = 1,
    Bottom = 4,
    Back = 2,
    Front = 5,
    Right = 0,
    Left = 3,
}
#[doc = doc_link!("enum/OrientationAlignmentMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum OrientationAlignmentMode {
    OneAttachment = 0,
    TwoAttachment = 1,
}
#[doc = doc_link!("enum/OutfitSource")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum OutfitSource {
    All = 1,
    Created = 2,
    Purchased = 3,
}
#[doc = doc_link!("enum/OutfitType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum OutfitType {
    All = 1,
    Avatar = 2,
    DynamicHead = 3,
}
#[doc = doc_link!("enum/OutputLayoutMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum OutputLayoutMode {
    Horizontal = 0,
    Vertical = 1,
}
#[doc = doc_link!("enum/OverrideMouseIconBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum OverrideMouseIconBehavior {
    None = 0,
    ForceShow = 1,
    ForceHide = 2,
}
#[doc = doc_link!("enum/PackagePermission")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PackagePermission {
    None = 0,
    NoAccess = 1,
    Revoked = 2,
    UseView = 3,
    Edit = 4,
    Own = 5,
}
#[doc = doc_link!("enum/PartType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PartType {
    Ball = 0,
    Block = 1,
    Cylinder = 2,
}
#[doc = doc_link!("enum/ParticleEmitterShape")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleEmitterShape {
    Box = 0,
    Sphere = 1,
    Cylinder = 2,
    Disc = 3,
}
#[doc = doc_link!("enum/ParticleEmitterShapeInOut")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleEmitterShapeInOut {
    Outward = 0,
    Inward = 1,
    InAndOut = 2,
}
#[doc = doc_link!("enum/ParticleEmitterShapeStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleEmitterShapeStyle {
    Volume = 0,
    Surface = 1,
}
#[doc = doc_link!("enum/ParticleFlipbookLayout")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleFlipbookLayout {
    None = 0,
    Grid2x2 = 1,
    Grid4x4 = 2,
    Grid8x8 = 3,
}
#[doc = doc_link!("enum/ParticleFlipbookMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleFlipbookMode {
    Loop = 0,
    OneShot = 1,
    PingPong = 2,
    Random = 3,
}
#[doc = doc_link!("enum/ParticleFlipbookTextureCompatible")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleFlipbookTextureCompatible {
    NotCompatible = 0,
    Compatible = 1,
    Unknown = 2,
}
#[doc = doc_link!("enum/ParticleOrientation")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleOrientation {
    FacingCamera = 0,
    FacingCameraWorldUp = 1,
    VelocityParallel = 2,
    VelocityPerpendicular = 3,
}
#[doc = doc_link!("enum/PathStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PathStatus {
    Success = 0,
    ClosestNoPath = 1,
    ClosestOutOfRange = 2,
    FailStartNotEmpty = 3,
    FailFinishNotEmpty = 4,
    NoPath = 5,
}
#[doc = doc_link!("enum/PathWaypointAction")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PathWaypointAction {
    Walk = 0,
    Jump = 1,
    Custom = 2,
}
#[doc = doc_link!("enum/PermissionLevelShown")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PermissionLevelShown {
    Game = 0,
    RobloxGame = 1,
    RobloxScript = 2,
    Studio = 3,
    Roblox = 4,
}
#[doc = doc_link!("enum/PhysicsSimulationRate")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PhysicsSimulationRate {
    Fixed240Hz = 0,
    Fixed120Hz = 1,
    Fixed60Hz = 2,
}
#[doc = doc_link!("enum/PhysicsSteppingMethod")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PhysicsSteppingMethod {
    Default = 0,
    Fixed = 1,
    Adaptive = 2,
}
#[doc = doc_link!("enum/Platform")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Platform {
    Windows = 0,
    OSX = 1,
    IOS = 2,
    Android = 3,
    XBoxOne = 4,
    PS4 = 5,
    PS3 = 6,
    XBox360 = 7,
    WiiU = 8,
    NX = 9,
    Ouya = 10,
    AndroidTV = 11,
    Chromecast = 12,
    Linux = 13,
    SteamOS = 14,
    WebOS = 15,
    DOS = 16,
    BeOS = 17,
    UWP = 18,
    None = 20,
}
#[doc = doc_link!("enum/PlaybackState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PlaybackState {
    Begin = 0,
    Delayed = 1,
    Playing = 2,
    Paused = 3,
    Completed = 4,
    Cancelled = 5,
}
#[doc = doc_link!("enum/PlayerActions")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PlayerActions {
    CharacterForward = 0,
    CharacterBackward = 1,
    CharacterLeft = 2,
    CharacterRight = 3,
    CharacterJump = 4,
}
#[doc = doc_link!("enum/PlayerChatType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PlayerChatType {
    All = 0,
    Team = 1,
    Whisper = 2,
}
#[doc = doc_link!("enum/PoseEasingDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PoseEasingDirection {
    Out = 1,
    InOut = 2,
    In = 0,
}
#[doc = doc_link!("enum/PoseEasingStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PoseEasingStyle {
    Linear = 0,
    Constant = 1,
    Elastic = 2,
    Cubic = 3,
    Bounce = 4,
}
#[doc = doc_link!("enum/PositionAlignmentMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PositionAlignmentMode {
    OneAttachment = 0,
    TwoAttachment = 1,
}
#[doc = doc_link!("enum/PrivilegeType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PrivilegeType {
    Owner = 255,
    Admin = 240,
    Member = 128,
    Visitor = 10,
    Banned = 0,
}
#[doc = doc_link!("enum/ProductLocationRestriction")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProductLocationRestriction {
    AvatarShop = 0,
    AllowedGames = 1,
    AllGames = 2,
}
#[doc = doc_link!("enum/ProductPurchaseDecision")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProductPurchaseDecision {
    NotProcessedYet = 0,
    PurchaseGranted = 1,
}
#[doc = doc_link!("enum/PropertyStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PropertyStatus {
    Ok = 0,
    Warning = 1,
    Error = 2,
}
#[doc = doc_link!("enum/ProximityPromptExclusivity")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProximityPromptExclusivity {
    OnePerButton = 0,
    OneGlobally = 1,
    AlwaysShow = 2,
}
#[doc = doc_link!("enum/ProximityPromptInputType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProximityPromptInputType {
    Keyboard = 0,
    Gamepad = 1,
    Touch = 2,
}
#[doc = doc_link!("enum/ProximityPromptStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProximityPromptStyle {
    Default = 0,
    Custom = 1,
}
#[doc = doc_link!("enum/QualityLevel")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum QualityLevel {
    Automatic = 0,
    Level01 = 1,
    Level02 = 2,
    Level03 = 3,
    Level04 = 4,
    Level05 = 5,
    Level06 = 6,
    Level07 = 7,
    Level08 = 8,
    Level09 = 9,
    Level10 = 10,
    Level11 = 11,
    Level12 = 12,
    Level13 = 13,
    Level14 = 14,
    Level15 = 15,
    Level16 = 16,
    Level17 = 17,
    Level18 = 18,
    Level19 = 19,
    Level20 = 20,
    Level21 = 21,
}
#[doc = doc_link!("enum/R15CollisionType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum R15CollisionType {
    OuterBox = 0,
    InnerBox = 1,
}
#[doc = doc_link!("enum/RaycastFilterType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RaycastFilterType {
    Blacklist = 0,
    Whitelist = 1,
}
#[doc = doc_link!("enum/RejectCharacterDeletions")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RejectCharacterDeletions {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/RenderFidelity")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RenderFidelity {
    Automatic = 0,
    Precise = 1,
    Performance = 2,
}
#[doc = doc_link!("enum/RenderPriority")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RenderPriority {
    First = 0,
    Input = 100,
    Camera = 200,
    Character = 300,
    Last = 2000,
}
#[doc = doc_link!("enum/RenderingTestComparisonMethod")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RenderingTestComparisonMethod {
    psnr = 0,
    diff = 1,
}
#[doc = doc_link!("enum/ReplicateInstanceDestroySetting")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ReplicateInstanceDestroySetting {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/ResamplerMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ResamplerMode {
    Default = 0,
    Pixelated = 1,
}
#[doc = doc_link!("enum/ReservedHighlightId")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ReservedHighlightId {
    Standard = 0,
    Selection = 524288,
    Hover = 262144,
    Active = 131072,
}
#[doc = doc_link!("enum/ReturnKeyType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ReturnKeyType {
    Default = 0,
    Done = 1,
    Go = 2,
    Next = 3,
    Search = 4,
    Send = 5,
}
#[doc = doc_link!("enum/ReverbType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ReverbType {
    NoReverb = 0,
    GenericReverb = 1,
    PaddedCell = 2,
    Room = 3,
    Bathroom = 4,
    LivingRoom = 5,
    StoneRoom = 6,
    Auditorium = 7,
    ConcertHall = 8,
    Cave = 9,
    Arena = 10,
    Hangar = 11,
    CarpettedHallway = 12,
    Hallway = 13,
    StoneCorridor = 14,
    Alley = 15,
    Forest = 16,
    City = 17,
    Mountains = 18,
    Quarry = 19,
    Plain = 20,
    ParkingLot = 21,
    SewerPipe = 22,
    UnderWater = 23,
}
#[doc = doc_link!("enum/RibbonTool")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RibbonTool {
    Select = 0,
    Scale = 1,
    Rotate = 2,
    Move = 3,
    Transform = 4,
    ColorPicker = 5,
    MaterialPicker = 6,
    Group = 7,
    Ungroup = 8,
    None = 9,
}
#[doc = doc_link!("enum/RigType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RigType {
    R15 = 0,
    Rthro = 1,
    RthroNarrow = 2,
    Custom = 3,
    None = 4,
}
#[doc = doc_link!("enum/RollOffMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RollOffMode {
    Inverse = 0,
    Linear = 1,
    InverseTapered = 3,
    LinearSquare = 2,
}
#[doc = doc_link!("enum/RotationOrder")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RotationOrder {
    XYZ = 0,
    XZY = 1,
    YZX = 2,
    YXZ = 3,
    ZXY = 4,
    ZYX = 5,
}
#[doc = doc_link!("enum/RotationType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RotationType {
    MovementRelative = 0,
    CameraRelative = 1,
}
#[doc = doc_link!("enum/RunContext")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RunContext {
    Legacy = 0,
    Server = 1,
    Client = 2,
    Plugin = 3,
}
#[doc = doc_link!("enum/RuntimeUndoBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RuntimeUndoBehavior {
    Aggregate = 0,
    Snapshot = 1,
    Hybrid = 2,
}
#[doc = doc_link!("enum/SafeAreaCompatibility")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SafeAreaCompatibility {
    None = 0,
    FullscreenExtension = 1,
}
#[doc = doc_link!("enum/SaveFilter")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SaveFilter {
    SaveAll = 2,
    SaveWorld = 0,
    SaveGame = 1,
}
#[doc = doc_link!("enum/SavedQualitySetting")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SavedQualitySetting {
    Automatic = 0,
    QualityLevel1 = 1,
    QualityLevel2 = 2,
    QualityLevel3 = 3,
    QualityLevel4 = 4,
    QualityLevel5 = 5,
    QualityLevel6 = 6,
    QualityLevel7 = 7,
    QualityLevel8 = 8,
    QualityLevel9 = 9,
    QualityLevel10 = 10,
}
#[doc = doc_link!("enum/ScaleType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScaleType {
    Stretch = 0,
    Slice = 1,
    Tile = 2,
    Fit = 3,
    Crop = 4,
}
#[doc = doc_link!("enum/ScopeCheckResult")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScopeCheckResult {
    
}
#[doc = doc_link!("enum/ScreenInsets")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScreenInsets {
    None = 0,
    DeviceSafeInsets = 1,
    CoreUISafeInsets = 2,
}
#[doc = doc_link!("enum/ScreenOrientation")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScreenOrientation {
    LandscapeLeft = 0,
    LandscapeRight = 1,
    LandscapeSensor = 2,
    Portrait = 3,
    Sensor = 4,
}
#[doc = doc_link!("enum/ScrollBarInset")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScrollBarInset {
    None = 0,
    ScrollBar = 1,
    Always = 2,
}
#[doc = doc_link!("enum/ScrollingDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScrollingDirection {
    X = 1,
    Y = 2,
    XY = 4,
}
#[doc = doc_link!("enum/SelectionBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SelectionBehavior {
    Escape = 0,
    Stop = 1,
}
#[doc = doc_link!("enum/ServerAudioBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ServerAudioBehavior {
    Enabled = 0,
    Muted = 1,
    OnlineGame = 2,
}
#[doc = doc_link!("enum/SignalBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SignalBehavior {
    Default = 0,
    Immediate = 1,
    Deferred = 2,
    AncestryDeferred = 3,
}
#[doc = doc_link!("enum/SizeConstraint")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SizeConstraint {
    RelativeXY = 0,
    RelativeXX = 1,
    RelativeYY = 2,
}
#[doc = doc_link!("enum/SortDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SortDirection {
    Ascending = 0,
    Descending = 1,
}
#[doc = doc_link!("enum/SortOrder")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SortOrder {
    LayoutOrder = 2,
    Name = 0,
    Custom = 1,
}
#[doc = doc_link!("enum/SpecialKey")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SpecialKey {
    Insert = 0,
    Home = 1,
    End = 2,
    PageUp = 3,
    PageDown = 4,
    ChatHotkey = 5,
}
#[doc = doc_link!("enum/StartCorner")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StartCorner {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}
#[doc = doc_link!("enum/Status")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Status {
    Poison = 0,
    Confusion = 1,
}
#[doc = doc_link!("enum/StreamOutBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StreamOutBehavior {
    Default = 0,
    LowMemory = 1,
    Opportunistic = 2,
}
#[doc = doc_link!("enum/StreamingIntegrityMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StreamingIntegrityMode {
    Default = 0,
    Disabled = 1,
    MinimumRadiusPause = 2,
    PauseOutsideLoadedArea = 3,
}
#[doc = doc_link!("enum/StreamingPauseMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StreamingPauseMode {
    Default = 0,
    Disabled = 1,
    ClientPhysicsPause = 2,
}
#[doc = doc_link!("enum/StudioCloseMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StudioCloseMode {
    None = 0,
    CloseStudio = 1,
    CloseDoc = 2,
}
#[doc = doc_link!("enum/StudioDataModelType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StudioDataModelType {
    Edit = 0,
    PlayClient = 1,
    PlayServer = 2,
    Standalone = 3,
    None = 4,
}
#[doc = doc_link!("enum/StudioScriptEditorColorCategories")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StudioScriptEditorColorCategories {
    Default = 0,
    Operator = 1,
    Number = 2,
    String = 3,
    Comment = 4,
    Keyword = 5,
    Builtin = 6,
    Method = 7,
    Property = 8,
    Nil = 9,
    Bool = 10,
    Function = 11,
    Local = 12,
    _Self = 13,
    LuauKeyword = 14,
    FunctionName = 15,
    TODO = 16,
    Background = 17,
    SelectionText = 18,
    SelectionBackground = 19,
    FindSelectionBackground = 20,
    MatchingWordBackground = 21,
    Warning = 22,
    Error = 23,
    Whitespace = 24,
    ActiveLine = 25,
    DebuggerCurrentLine = 26,
    DebuggerErrorLine = 27,
    Ruler = 28,
    Bracket = 29,
    MenuPrimaryText = 30,
    MenuSecondaryText = 31,
    MenuSelectedText = 32,
    MenuBackground = 33,
    MenuSelectedBackground = 34,
    MenuScrollbarBackground = 35,
    MenuScrollbarHandle = 36,
    MenuBorder = 37,
    DocViewCodeBackground = 38,
}
#[doc = doc_link!("enum/StudioScriptEditorColorPresets")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StudioScriptEditorColorPresets {
    RobloxDefault = 0,
    Extra1 = 1,
    Extra2 = 2,
    Custom = 3,
}
#[doc = doc_link!("enum/StudioStyleGuideColor")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StudioStyleGuideColor {
    MainBackground = 0,
    Titlebar = 1,
    Dropdown = 2,
    Tooltip = 3,
    Notification = 4,
    ScrollBar = 5,
    ScrollBarBackground = 6,
    TabBar = 7,
    Tab = 8,
    FilterButtonDefault = 9,
    FilterButtonHover = 10,
    FilterButtonChecked = 11,
    FilterButtonAccent = 12,
    FilterButtonBorder = 13,
    FilterButtonBorderAlt = 14,
    RibbonTab = 15,
    RibbonTabTopBar = 16,
    Button = 17,
    MainButton = 18,
    RibbonButton = 19,
    ViewPortBackground = 20,
    InputFieldBackground = 21,
    Item = 22,
    TableItem = 23,
    CategoryItem = 24,
    GameSettingsTableItem = 25,
    GameSettingsTooltip = 26,
    EmulatorBar = 27,
    EmulatorDropDown = 28,
    ColorPickerFrame = 29,
    CurrentMarker = 30,
    Border = 31,
    DropShadow = 32,
    Shadow = 33,
    Light = 34,
    Dark = 35,
    Mid = 36,
    MainText = 37,
    SubText = 38,
    TitlebarText = 39,
    BrightText = 40,
    DimmedText = 41,
    LinkText = 42,
    WarningText = 43,
    ErrorText = 44,
    InfoText = 45,
    SensitiveText = 46,
    ScriptSideWidget = 47,
    ScriptBackground = 48,
    ScriptText = 49,
    ScriptSelectionText = 50,
    ScriptSelectionBackground = 51,
    ScriptFindSelectionBackground = 52,
    ScriptMatchingWordSelectionBackground = 53,
    ScriptOperator = 54,
    ScriptNumber = 55,
    ScriptString = 56,
    ScriptComment = 57,
    ScriptKeyword = 58,
    ScriptBuiltInFunction = 59,
    ScriptWarning = 60,
    ScriptError = 61,
    ScriptWhitespace = 62,
    ScriptRuler = 63,
    DocViewCodeBackground = 64,
    DebuggerCurrentLine = 65,
    DebuggerErrorLine = 66,
    ScriptEditorCurrentLine = 105,
    DiffFilePathText = 67,
    DiffTextHunkInfo = 68,
    DiffTextNoChange = 69,
    DiffTextAddition = 70,
    DiffTextDeletion = 71,
    DiffTextSeparatorBackground = 72,
    DiffTextNoChangeBackground = 73,
    DiffTextAdditionBackground = 74,
    DiffTextDeletionBackground = 75,
    DiffLineNum = 76,
    DiffLineNumSeparatorBackground = 77,
    DiffLineNumNoChangeBackground = 78,
    DiffLineNumAdditionBackground = 79,
    DiffLineNumDeletionBackground = 80,
    DiffFilePathBackground = 81,
    DiffFilePathBorder = 82,
    ChatIncomingBgColor = 83,
    ChatIncomingTextColor = 84,
    ChatOutgoingBgColor = 85,
    ChatOutgoingTextColor = 86,
    ChatModeratedMessageColor = 87,
    Separator = 88,
    ButtonBorder = 89,
    ButtonText = 90,
    InputFieldBorder = 91,
    CheckedFieldBackground = 92,
    CheckedFieldBorder = 93,
    CheckedFieldIndicator = 94,
    HeaderSection = 95,
    Midlight = 96,
    StatusBar = 97,
    DialogButton = 98,
    DialogButtonText = 99,
    DialogButtonBorder = 100,
    DialogMainButton = 101,
    DialogMainButtonText = 102,
    InfoBarWarningBackground = 103,
    InfoBarWarningText = 104,
    ScriptMethod = 106,
    ScriptProperty = 107,
    ScriptNil = 108,
    ScriptBool = 109,
    ScriptFunction = 110,
    ScriptLocal = 111,
    ScriptSelf = 112,
    ScriptLuauKeyword = 113,
    ScriptFunctionName = 114,
    ScriptTodo = 115,
    ScriptBracket = 116,
    AttributeCog = 117,
}
#[doc = doc_link!("enum/StudioStyleGuideModifier")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StudioStyleGuideModifier {
    Default = 0,
    Selected = 1,
    Pressed = 2,
    Disabled = 3,
    Hover = 4,
}
#[doc = doc_link!("enum/Style")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Style {
    AlternatingSupports = 0,
    BridgeStyleSupports = 1,
    NoSupports = 2,
}
#[doc = doc_link!("enum/SurfaceConstraint")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SurfaceConstraint {
    None = 0,
    Hinge = 1,
    SteppingMotor = 2,
    Motor = 3,
}
#[doc = doc_link!("enum/SurfaceGuiSizingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SurfaceGuiSizingMode {
    FixedSize = 0,
    PixelsPerStud = 1,
}
#[doc = doc_link!("enum/SurfaceType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SurfaceType {
    Smooth = 0,
    Glue = 1,
    Weld = 2,
    Studs = 3,
    Inlet = 4,
    Universal = 5,
    Hinge = 6,
    Motor = 7,
    SteppingMotor = 8,
    SmoothNoOutlines = 10,
}
#[doc = doc_link!("enum/SwipeDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SwipeDirection {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
    None = 4,
}
#[doc = doc_link!("enum/TableMajorAxis")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TableMajorAxis {
    RowMajor = 0,
    ColumnMajor = 1,
}
#[doc = doc_link!("enum/Technology")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Technology {
    Compatibility = 2,
    Voxel = 1,
    ShadowMap = 3,
    Legacy = 0,
    Future = 4,
}
#[doc = doc_link!("enum/TeleportMethod")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TeleportMethod {
    TeleportToSpawnByName = 0,
    TeleportToPlaceInstance = 1,
    TeleportToPrivateServer = 2,
    TeleportPartyAsync = 3,
    TeleportUnknown = 4,
}
#[doc = doc_link!("enum/TeleportResult")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TeleportResult {
    Success = 0,
    Failure = 1,
    GameNotFound = 2,
    GameEnded = 3,
    GameFull = 4,
    Unauthorized = 5,
    Flooded = 6,
    IsTeleporting = 7,
}
#[doc = doc_link!("enum/TeleportState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TeleportState {
    RequestedFromServer = 0,
    Started = 1,
    WaitingForServer = 2,
    Failed = 3,
    InProgress = 4,
}
#[doc = doc_link!("enum/TeleportType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TeleportType {
    ToPlace = 0,
    ToInstance = 1,
    ToReservedServer = 2,
}
#[doc = doc_link!("enum/TerrainAcquisitionMethod")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TerrainAcquisitionMethod {
    None = 0,
    Legacy = 1,
    Template = 2,
    Generate = 3,
    Import = 4,
    Convert = 5,
    EditAddTool = 6,
    EditSeaLevelTool = 7,
    EditReplaceTool = 8,
    RegionFillTool = 9,
    RegionPasteTool = 10,
    Other = 11,
}
#[doc = doc_link!("enum/TerrainFace")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TerrainFace {
    Top = 0,
    Side = 1,
    Bottom = 2,
}
#[doc = doc_link!("enum/TextChatMessageStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextChatMessageStatus {
    Unknown = 1,
    Success = 2,
    Sending = 3,
    TextFilterFailed = 4,
    Floodchecked = 5,
    InvalidPrivacySettings = 6,
    InvalidTextChannelPermissions = 7,
    MessageTooLong = 8,
}
#[doc = doc_link!("enum/TextFilterContext")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextFilterContext {
    PublicChat = 1,
    PrivateChat = 2,
}
#[doc = doc_link!("enum/TextInputType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextInputType {
    Default = 0,
    NoSuggestions = 1,
    Number = 2,
    Email = 3,
    Phone = 4,
    Password = 5,
    PasswordShown = 6,
    Username = 7,
    OneTimePassword = 8,
}
#[doc = doc_link!("enum/TextTruncate")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextTruncate {
    None = 0,
    AtEnd = 1,
}
#[doc = doc_link!("enum/TextXAlignment")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextXAlignment {
    Left = 0,
    Center = 2,
    Right = 1,
}
#[doc = doc_link!("enum/TextYAlignment")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextYAlignment {
    Top = 0,
    Center = 1,
    Bottom = 2,
}
#[doc = doc_link!("enum/TextureMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextureMode {
    Stretch = 0,
    Wrap = 1,
    Static = 2,
}
#[doc = doc_link!("enum/TextureQueryType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextureQueryType {
    NonHumanoid = 0,
    NonHumanoidOrphaned = 1,
    Humanoid = 2,
    HumanoidOrphaned = 3,
}
#[doc = doc_link!("enum/ThreadPoolConfig")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ThreadPoolConfig {
    Auto = 0,
    PerCore1 = 101,
    PerCore2 = 102,
    PerCore3 = 103,
    PerCore4 = 104,
    Threads1 = 1,
    Threads2 = 2,
    Threads3 = 3,
    Threads4 = 4,
    Threads8 = 8,
    Threads16 = 16,
}
#[doc = doc_link!("enum/ThrottlingPriority")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ThrottlingPriority {
    Extreme = 2,
    ElevatedOnServer = 1,
    Default = 0,
}
#[doc = doc_link!("enum/ThumbnailSize")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ThumbnailSize {
    Size48x48 = 0,
    Size180x180 = 1,
    Size420x420 = 2,
    Size60x60 = 3,
    Size100x100 = 4,
    Size150x150 = 5,
    Size352x352 = 6,
}
#[doc = doc_link!("enum/ThumbnailType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ThumbnailType {
    HeadShot = 0,
    AvatarBust = 1,
    AvatarThumbnail = 2,
}
#[doc = doc_link!("enum/TickCountSampleMethod")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TickCountSampleMethod {
    Fast = 0,
    Benchmark = 1,
    Precise = 2,
}
#[doc = doc_link!("enum/TopBottom")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TopBottom {
    Top = 0,
    Center = 1,
    Bottom = 2,
}
#[doc = doc_link!("enum/TouchCameraMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TouchCameraMovementMode {
    Default = 0,
    Follow = 2,
    Classic = 1,
    Orbital = 3,
}
#[doc = doc_link!("enum/TouchMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TouchMovementMode {
    Default = 0,
    Thumbstick = 1,
    DPad = 2,
    Thumbpad = 3,
    ClickToMove = 4,
    DynamicThumbstick = 5,
}
#[doc = doc_link!("enum/TrackerError")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TrackerError {
    Ok = 0,
    NoService = 1,
    InitFailed = 2,
    NoVideo = 3,
    VideoError = 4,
    VideoNoPermission = 5,
    NoAudio = 6,
    AudioError = 7,
    AudioNoPermission = 8,
}
#[doc = doc_link!("enum/TrackerExtrapolationFlagMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TrackerExtrapolationFlagMode {
    Auto = 3,
    ForceDisabled = 0,
    ExtrapolateFacsAndPose = 1,
    ExtrapolateFacsOnly = 2,
}
#[doc = doc_link!("enum/TrackerLodFlagMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TrackerLodFlagMode {
    Auto = 2,
    ForceFalse = 0,
    ForceTrue = 1,
}
#[doc = doc_link!("enum/TrackerLodValueMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TrackerLodValueMode {
    Auto = 2,
    Force0 = 0,
    Force1 = 1,
}
#[doc = doc_link!("enum/TrackerMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TrackerMode {
    None = 0,
    Audio = 1,
    Video = 2,
    AudioVideo = 3,
}
#[doc = doc_link!("enum/TriStateBoolean")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TriStateBoolean {
    Unknown = 0,
    True = 1,
    False = 2,
}
#[doc = doc_link!("enum/TweenStatus")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TweenStatus {
    Canceled = 0,
    Completed = 1,
}
#[doc = doc_link!("enum/UITheme")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UITheme {
    Light = 0,
    Dark = 1,
}
#[doc = doc_link!("enum/UiMessageType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UiMessageType {
    UiMessageError = 0,
    UiMessageInfo = 1,
}
#[doc = doc_link!("enum/UnionsScaleNonuniformly")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UnionsScaleNonuniformly {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/UsageContext")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UsageContext {
    Default = 0,
    Preview = 1,
}
#[doc = doc_link!("enum/UserCFrame")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UserCFrame {
    Head = 0,
    LeftHand = 1,
    RightHand = 2,
}
#[doc = doc_link!("enum/UserInputState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UserInputState {
    Begin = 0,
    Change = 1,
    End = 2,
    Cancel = 3,
    None = 4,
}
#[doc = doc_link!("enum/UserInputType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum UserInputType {
    MouseButton1 = 0,
    MouseButton2 = 1,
    MouseButton3 = 2,
    MouseWheel = 3,
    MouseMovement = 4,
    Touch = 7,
    Keyboard = 8,
    Focus = 9,
    Accelerometer = 10,
    Gyro = 11,
    Gamepad1 = 12,
    Gamepad2 = 13,
    Gamepad3 = 14,
    Gamepad4 = 15,
    Gamepad5 = 16,
    Gamepad6 = 17,
    Gamepad7 = 18,
    Gamepad8 = 19,
    TextInput = 20,
    InputMethod = 21,
    None = 22,
}
#[doc = doc_link!("enum/VRSessionState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VRSessionState {
    Idle = 1,
    Visible = 2,
    Focused = 3,
    Undefined = 0,
}
#[doc = doc_link!("enum/VRTouchpad")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VRTouchpad {
    Left = 0,
    Right = 1,
}
#[doc = doc_link!("enum/VRTouchpadMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VRTouchpadMode {
    Touch = 0,
    VirtualThumbstick = 1,
    ABXY = 2,
}
#[doc = doc_link!("enum/VelocityConstraintMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VelocityConstraintMode {
    Line = 0,
    Plane = 1,
    Vector = 2,
}
#[doc = doc_link!("enum/VerticalAlignment")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VerticalAlignment {
    Center = 0,
    Top = 1,
    Bottom = 2,
}
#[doc = doc_link!("enum/VerticalScrollBarPosition")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VerticalScrollBarPosition {
    Left = 1,
    Right = 0,
}
#[doc = doc_link!("enum/VibrationMotor")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VibrationMotor {
    Large = 0,
    Small = 1,
    LeftTrigger = 2,
    RightTrigger = 3,
    LeftHand = 4,
    RightHand = 5,
}
#[doc = doc_link!("enum/VirtualCursorMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VirtualCursorMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/VirtualInputMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VirtualInputMode {
    Recording = 1,
    Playing = 2,
    None = 0,
}
#[doc = doc_link!("enum/VoiceChatState")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VoiceChatState {
    Idle = 0,
    Joining = 1,
    JoiningRetry = 2,
    Joined = 3,
    Leaving = 4,
    Ended = 5,
    Failed = 6,
}
#[doc = doc_link!("enum/VolumetricAudio")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VolumetricAudio {
    Disabled = 0,
    Automatic = 1,
    Enabled = 2,
}
#[doc = doc_link!("enum/WaterDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum WaterDirection {
    NegX = 0,
    X = 1,
    NegY = 2,
    Y = 3,
    NegZ = 4,
    Z = 5,
}
#[doc = doc_link!("enum/WaterForce")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum WaterForce {
    None = 0,
    Small = 1,
    Medium = 2,
    Strong = 3,
    Max = 4,
}
#[doc = doc_link!("enum/WrapLayerAutoSkin")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum WrapLayerAutoSkin {
    Disabled = 0,
    EnabledPreserve = 1,
    EnabledOverride = 2,
}
#[doc = doc_link!("enum/WrapLayerDebugMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum WrapLayerDebugMode {
    None = 0,
    BoundCage = 1,
    LayerCage = 2,
    BoundCageAndLinks = 3,
    Reference = 4,
    Rbf = 5,
    OuterCage = 6,
    ReferenceMeshAfterMorph = 7,
    HSROuterDetail = 8,
    HSROuter = 9,
    HSRInner = 10,
    HSRInnerReverse = 11,
    LayerCageFittedToBase = 12,
    LayerCageFittedToPrev = 13,
}
#[doc = doc_link!("enum/WrapTargetDebugMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum WrapTargetDebugMode {
    None = 0,
    TargetCageOriginal = 1,
    TargetCageCompressed = 2,
    TargetCageInterface = 3,
    TargetLayerCageOriginal = 4,
    TargetLayerCageCompressed = 5,
    TargetLayerInterface = 6,
    Rbf = 7,
    OuterCageDetail = 8,
}
#[doc = doc_link!("enum/ZIndexBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ZIndexBehavior {
    Global = 0,
    Sibling = 1,
}
