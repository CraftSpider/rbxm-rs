//! Enums primarily used by Instance properties

#![allow(missing_docs)]

use rbxm_proc::EnumConvert;

/// The [`CFrame`][crate::model::CFrame] that this Instance is relative to
///
/// **Reference Link**: [enum/ActuatorRelativeTo](https://developer.roblox.com/en-us/api-reference/enum/ActuatorRelativeTo)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ActuatorRelativeTo {
    /// Relative to the first attachment
    Attachment0 = 0,
    /// Relative to the second attachment
    Attachment1 = 1,
    /// Relative to the absolute game world
    World = 2,
}

/// The type of actuator this Instance is
///
/// **Reference Link**: [enum/ActuatorType](https://developer.roblox.com/en-us/api-reference/enum/ActuatorType)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ActuatorType {
    /// No actuati,on
    None = 0,
    /// Motor actuation
    Motor = 1,
    /// Servo actuation
    Servo = 2,
}

/// When to cull this adornment
///
/// **Reference Link**: [enum/AdornCullingMode](https://developer.roblox.com/en-us/api-reference/enum/AdornCullingMode)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AdornCullingMode {
    /// Cull automatically
    Automatic = 0,
    /// Never cull
    Never = 1,
}

/// The axis relationship between two alignment orientations
///
/// **Reference Link**: [enum/AlignType](https://developer.roblox.com/en-us/api-reference/enum/AlignType)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AlignType {
    /// Axis are parallel
    Parallel = 0,
    /// Axis are perpendicular
    Perpendicular = 1,
}

/// How the alpha channel of a color map is used
///
/// **Reference Link**: [enum/AlphaMode](https://developer.roblox.com/en-us/api-reference/enum/AlphaMode)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AlphaMode {
    /// Overlays the color map over the underlying part color
    Overlay = 0,
    /// Overlays the color map over the underlying color3
    Transparency = 1,
}

/// The priority of animations played at the same time
///
/// **Reference Link**: [enum/AnimationPriority](https://developer.roblox.com/en-us/api-reference/enum/AnimationPriority)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnimationPriority {
    /// Second lowest priority
    Idle = 0,
    /// Second highest priority
    Movement = 1,
    /// Highest priority
    Action = 2,
    /// Lowest priority
    Core = 1000,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnimatorRetargetingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

/// How to apply this UI stroke
///
/// **Reference Link**: [enum/ApplyStrokeMode](https://developer.roblox.com/en-us/api-reference/enum/ApplyStrokeMode)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ApplyStrokeMode {
    /// Contextual application
    Contextual = 0,
    /// Border application
    Border = 1,
}

/// Controls how an aspect ratio constraint applies
///
/// **Reference Link**: [enum/AspectType](https://developer.roblox.com/en-us/api-reference/enum/AspectType)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AspectType {
    /// Match the parent's current size while maintaining aspect ratio
    FitWithinMaxSize = 0,
    /// Match the parent's maximum size while maintaining aspect ratio
    ScaleWithParentSize = 1,
}

/// Whether the element should automatically increase in size to the maximum allowed by the parent
///
/// **Reference Link**: [enum/AutomaticSize](https://developer.roblox.com/en-us/api-reference/enum/AutomaticSize)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AutomaticSize {
    /// Do not use automatic size
    None = 0,
    /// Resize along the X axis
    X = 1,
    /// Resize along the Y axis
    Y = 2,
    /// Resize along both X and Y axes
    XY = 3,
}

/// Represents a single X, Y, or Z axis
///
/// **Reference Link**: [enum/Axis](https://developer.roblox.com/en-us/api-reference/enum/Axis)
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Axis {
    /// X axis
    X = 0,
    /// Y axis
    Y = 1,
    /// Z axis
    Z = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BinType {
    Script = 0,
    GameTool = 1,
    Grab = 2,
    Clone = 3,
    Hammer = 4,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BodyPart {
    Head = 0,
    Torso = 1,
    LeftArm = 2,
    RightArm = 3,
    LeftLeg = 4,
    RightLeg = 5,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BorderMode {
    Outline = 0,
    Middle = 1,
    Inset = 2,
}

impl Default for BorderMode {
    fn default() -> Self {
        BorderMode::Outline
    }
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ButtonStyle {
    Custom = 0,
    RobloxButtonDefault = 1,
    RobloxButton = 2,
    RobloxRoundButton = 3,
    RobloxRoundDefaultButton = 4,
    RobloxRoundDropdownButton = 5,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CameraType {
    Fixed = 0,
    Attach = 1,
    Watch = 2,
    Track = 3,
    Follow = 4,
    Custom = 5,
    Scriptable = 6,
    Orbital = 7,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ClientAnimatorThrottlingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogBehaviorType {
    SinglePlayer = 0,
    MultiplePlayers = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogPurpose {
    Quest = 0,
    Help = 1,
    Shop = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogTone {
    Neutral = 0,
    Friendly = 1,
    Enemy = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DominantAxis {
    Width = 0,
    Height = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum EasingDirection {
    In = 0,
    Out = 1,
    InOut = 2,
}

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

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ElasticBehavior {
    WhenScrollable = 0,
    Always = 1,
    Never = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ExplosionType {
    NoCraters = 0,
    Craters = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FieldOfViewMode {
    Vertical = 0,
    Diagonal = 1,
    MaxAxis = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FillDirection {
    Horizontal = 0,
    Vertical = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Font {
    Legacy = 0,
    Arial = 1,
    ArialBold = 2,
    SourceSans = 3,
    SourceSansBold = 4,
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
    SourceSansSemibold = 16,
    Gotham = 17,
    GothamSemibold = 18,
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
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum FormFactor {
    Symmetric = 0,
    Brick = 1,
    Plate = 2,
    Custom = 3,
}

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

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HandlesStyle {
    Resize = 0,
    Movement = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HorizontalAlignment {
    Center = 0,
    Left = 1,
    Right = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidCollisionType {
    OuterBox = 0,
    InnerBox = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidDisplayDistanceType {
    Viewer = 0,
    Subject = 1,
    None = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidHealthDisplayType {
    DisplayWhenDamaged = 0,
    AlwaysOn = 1,
    AlwaysOff = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidOnlySetCollisionsOnStateChange {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum HumanoidRigType {
    R6 = 0,
    R15 = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InOut {
    Edge = 0,
    Inset = 1,
    Center = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InputType {
    NoInput = 0,
    Constant = 12,
    Sin = 13,
}

impl Default for InputType {
    fn default() -> Self {
        InputType::NoInput
    }
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum InterpolationThrottlingMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

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
    NumLock = 300,
    CapsLock = 301,
    ScrollLock = 302,
    RightShift = 303,
    LeftShift = 304,
    RightControl = 305,
    LeftControl = 306,
    RightAlt = 307,
    LeftAlt = 308,
    RightMeta = 309,
    LeftMeta = 310,
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

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LeftRight {
    Left = 0,
    Center = 1,
    Right = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LevelOfDetailSetting {
    Low = 0,
    Medium = 1,
    High = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum LineJoinMode {
    Round = 0,
    Bevel = 1,
    Miter = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Material {
    Plastic = 256,
    SmoothPlastic = 272,
    Neon = 288,
    Wood = 512,
    WoodPlanks = 528,
    Marble = 784,
    Basalt = 788,
    Slate = 800,
    CrackedLava = 804,
    Concrete = 816,
    Limestone = 820,
    Granite = 832,
    Pavement = 836,
    Brick = 848,
    Pebble = 864,
    Cobblestone = 880,
    Rock = 896,
    Sandstone = 912,
    CorrodedMetal = 1040,
    DiamondPlate = 1056,
    Foil = 1072,
    Metal = 1088,
    Grass = 1280,
    LeafyGrass = 1284,
    Sand = 1296,
    Fabric = 1312,
    Snow = 1328,
    Mud = 1344,
    Ground = 1360,
    Asphalt = 1376,
    Salt = 1392,
    Ice = 1536,
    Glacier = 1552,
    Glass = 1568,
    ForceField = 1584,
    Air = 1792,
    Water = 2048,
}

impl Default for Material {
    fn default() -> Self {
        Material::Plastic
    }
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MeshPartHeadsAndAccessories {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum MeshType {
    Head = 0,
    Torso = 1,
    Wedge = 2,
    Sphere = 3,
    Cylinder = 4,
    FileMesh = 5,
    Brick = 6,
    Prism = 7,
    Pyramid = 8,
    ParallelRamp = 9,
    RightAngleRame = 10,
    CornerWedge = 11,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ModelLevelOfDetail {
    Automatic = 0,
    StreamingMesh = 1,
    Disabled = 2,
}

impl Default for ModelLevelOfDetail {
    fn default() -> Self {
        ModelLevelOfDetail::Automatic
    }
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NameOcclusion {
    NoOcclusion = 0,
    EnemyOcclusion = 1,
    OccludeAll = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NewAnimationRuntimeSettings {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum NormalId {
    Right = 0,
    Top = 1,
    Back = 2,
    Left = 3,
    Bottom = 4,
    Front = 5,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ParticleOrientation {
    FacingCamera = 0,
    FacingCameraWorldUp = 1,
    VelocityParallel = 2,
    VelocityPerpendicular = 3,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PartType {
    Ball = 0,
    Block = 1,
    Cylinder = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PhysicsSimulationRate {
    Fixed240Hz = 0,
    Fixed120Hz = 1,
    Fixed60Hz = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PhysicsSteppingMethod {
    Default = 0,
    Fixed = 1,
    Adaptive = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PoseEasingDirection {
    In = 0,
    Out = 1,
    InOut = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum PoseEasingStyle {
    Linear = 0,
    Constant = 1,
    Elastic = 2,
    Cubic = 3,
    Bounce = 4,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProximityPromptExclusivity {
    OnePerButton = 0,
    OneGlobally = 1,
    AlwaysShow = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ProximityPromptStyle {
    Default = 0,
    Custom = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RenderFidelity {
    Automatic = 0,
    Precise = 1,
    Performance = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RenderingTestComparisonMethod {
    Psnr = 0,
    Diff = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ResamplerMode {
    Default = 0,
    Pixelated = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum RollOffMode {
    Inverse = 0,
    Linear = 1,
    LinearSquare = 2,
    InverseTapered = 3,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScaleType {
    Stretch = 0,
    Slice = 1,
    Tile = 2,
    Fit = 3,
    Crop = 4,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScrollBarInset {
    None = 0,
    ScrollBar = 1,
    Always = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ScrollingDirection {
    X = 1,
    Y = 2,
    XY = 4,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SignalBehavior {
    Default = 0,
    Immediate = 1,
    Deferred = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SizeConstraint {
    RelativeXY = 0,
    RelativeXX = 1,
    RelativeYY = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SortOrder {
    Name = 0,
    Custom = 1,
    LayoutOrder = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StartCorner {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StreamingPauseMode {
    Default = 0,
    Disabled = 1,
    ClientPhysicsPause = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum StreamOutBehavior {
    Default = 0,
    LowMemory = 1,
    Opportunistic = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum SurfaceGuiSizingMode {
    FixedSize = 0,
    PixelsPerStud = 1,
}

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

impl Default for SurfaceType {
    fn default() -> Self {
        SurfaceType::Smooth
    }
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TableMajorAxis {
    RowMajor = 0,
    ColumnMajor = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Technology {
    Legacy = 0,
    Voxel = 1,
    Compatibility = 2,
    ShadowMap = 3,
    Future = 4,
}

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

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextTruncate {
    None = 0,
    AtEnd = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextureMode {
    Stretch = 0,
    Wrap = 1,
    Static = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextXAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TextYAlignment {
    Top = 0,
    Center = 1,
    Bottom = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TopBottom {
    Top = 0,
    Center = 1,
    Bottom = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum TrussStyle {
    AlternatingSupports = 0,
    BridgeStyleSupports = 1,
    NoSupports = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VerticalAlignment {
    Center = 0,
    Top = 1,
    Bottom = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum VerticalScrollBarPosition {
    Right = 0,
    Left = 1,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ZIndexBehavior {
    Global = 0,
    Sibling = 1,
}
