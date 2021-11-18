//! Enums primarily used by Instance properties

#![allow(missing_docs)]

use rbxm_proc::EnumConvert;

/// The [`CFrame`](crate::model::CFrame) that this Instance is relative to
///
#[doc = doc_link!("enum/ActuatorRelativeTo")]
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
#[doc = doc_link!("enum/ActuatorType")]
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
#[doc = doc_link!("enum/AdornCullingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AdornCullingMode {
    /// Cull automatically
    Automatic = 0,
    /// Never cull
    Never = 1,
}

/// The axis relationship between two alignment orientations
///
#[doc = doc_link!("enum/AlignType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AlignType {
    /// Axis are parallel
    Parallel = 0,
    /// Axis are perpendicular
    Perpendicular = 1,
}

/// How the alpha channel of a color map is used
///
#[doc = doc_link!("enum/AlphaMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AlphaMode {
    /// Overlays the color map over the underlying part color
    Overlay = 0,
    /// Overlays the color map over the underlying color3
    Transparency = 1,
}

/// The priority of animations played at the same time
///
#[doc = doc_link!("enum/AnimationPriority")]
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

/// Whether to enable animation retargeting ([What is Animation Retargeting?][1])
///
#[doc = doc_link!("enum/AnimatorRetargetingMode")]
///
/// [1]: https://docs.unrealengine.com/4.26/en-US/AnimatingObjects/SkeletalMeshAnimation/AnimationRetargeting/
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AnimatorRetargetingMode {
    /// Use the Roblox default mode
    Default = 0,
    /// Disable animation retargeting
    Disabled = 1,
    /// Enable animation retargeting
    Enabled = 2,
}

/// How to apply this UI stroke
///
#[doc = doc_link!("enum/ApplyStrokeMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ApplyStrokeMode {
    /// Contextual application
    Contextual = 0,
    /// Border application
    Border = 1,
}

/// Controls how an aspect ratio constraint applies
///
#[doc = doc_link!("enum/AspectType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum AspectType {
    /// Match the parent's current size while maintaining aspect ratio
    FitWithinMaxSize = 0,
    /// Match the parent's maximum size while maintaining aspect ratio
    ScaleWithParentSize = 1,
}

/// Whether the element should automatically increase in size to the maximum allowed by the parent
///
#[doc = doc_link!("enum/AutomaticSize")]
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
#[doc = doc_link!("enum/Axis")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Axis {
    /// X axis
    X = 0,
    /// Y axis
    Y = 1,
    /// Z axis
    Z = 2,
}

/// The type of a [`HopperBin`](super::instance::HopperBin)
///
#[doc = doc_link!("enum/BinType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BinType {
    /// A tool that relies on a script for behavior
    Script = 0,
    /// A tool that relies on game for behavior
    GameTool = 1,
    /// A grab tool
    Grab = 2,
    /// A clone tool
    Clone = 3,
    /// A hammer tool
    Hammer = 4,
}

/// The body part that a [`CharacterMesh`](super::instance::CharacterMesh) affects
///
#[doc = doc_link!("enum/BodyPart")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BodyPart {
    /// Mesh affects the head
    Head = 0,
    /// Mesh affects the torso
    Torso = 1,
    /// Mesh affects the left arm
    LeftArm = 2,
    /// Mesh affects the right arm
    RightArm = 3,
    /// Mesh affects the left leg
    LeftLeg = 4,
    /// Mesh affects the right leg
    RightLeg = 5,
}

/// How the border of a [`GuiObject`](super::instance::GuiObject) is laid out
///
#[doc = doc_link!("enum/BorderMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum BorderMode {
    /// Border outlines the GUI, inner size not affected by thickness
    Outline = 0,
    /// Border sits in the middle of outside and inside, inner size is cut by half of thickness
    Middle = 1,
    /// Border is inset in the GUI, inner size is cut by thickness
    Inset = 2,
}

impl Default for BorderMode {
    fn default() -> Self {
        BorderMode::Outline
    }
}

/// The style of a [`GuiButton`](super::instance::GuiButton)
///
#[doc = doc_link!("enum/ButtonStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ButtonStyle {
    /// Custom button style
    Custom = 0,
    /// Use the Roblox default style
    RobloxButtonDefault = 1,
    /// Use the Roblox style
    RobloxButton = 2,
    /// Use the round Roblox style
    RobloxRoundButton = 3,
    /// Use the round Roblox default style
    RobloxRoundDefaultButton = 4,
    /// Use the Roblox round dropdown style
    RobloxRoundDropdownButton = 5,
}

/// Mode for the user camera
///
#[doc = doc_link!("enum/CameraMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CameraMode {
    /// Classic camera, can be zoomed between first and third person
    Classic = 0,
    /// Camera is locked into first-person mode
    LockFirstPersion = 1,
}

/// Control the behavior of a [`Camera`](super::instance::Camera)
///
#[doc = doc_link!("enum/CameraType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum CameraType {
    /// Fixed position camera
    Fixed = 0,
    /// Fixed relative to a subject
    Attach = 1,
    /// Watch a subject from the current position
    Watch = 2,
    /// Move with the subject, but don't rotate the camera
    Track = 3,
    /// Move with the subject, rotating as they do
    Follow = 4,
    /// Default mode of Roblox core scripts
    Custom = 5,
    /// No default behavior, controlled dynamically
    Scriptable = 6,
    /// Camera stays at a fixed Y position, but can rotate around the subject
    Orbital = 7,
}

/// Client animator throttling behavior
///
#[doc = doc_link!("enum/ClientAnimatorThrottlingMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ClientAnimatorThrottlingMode {
    /// Default settings
    Default = 0,
    /// Disable throttling
    Disabled = 1,
    /// Enable throttling
    Enabled = 2,
}

/// How the camera handles objects between it and the subject
///
#[doc = doc_link!("enum/DevCameraOcclusionMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevCameraOcclusionMode {
    /// Zoom in until there's nothing obscuring the subject
    Zoom = 0,
    /// Any objects between the player and the subject will become translucent
    Invisicam = 1,
}

/// Override for the player's camera movement mode if they're on a computer
///
#[doc = doc_link!("enum/DevComputerCameraMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevComputerCameraMovementMode {
    /// Use the user's settings
    UserChoice = 0,
    /// Force classic camera movement
    Classic = 1,
    /// Force follow camera movement
    Follow = 2,
    /// Force orbital camera movement
    Orbital = 3,
    /// Force camera toggle mode
    CameraToggle = 4,
}

/// Override for the player's movement mode if they're on a computer
///
#[doc = doc_link!("enum/DevComputerMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevComputerMovementMode {
    /// Use the user's settings
    UserChoice = 0,
    /// Force keyboard+mouse movement
    KeyboardMouse = 1,
    /// Force click-based movement
    ClickToMove = 2,
    /// Disable all default movement, all character movement will be handled by custom scripts
    Scriptable = 3,
}

/// Override for the player's camera movement mode if they're on mobile
///
#[doc = doc_link!("enum/DevTouchCameraMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevTouchCameraMovementMode {
    /// Use the user's settings
    UserChoice = 0,
    /// Force classic camera movement
    Classic = 1,
    /// Force follow camera movement
    Follow = 2,
    /// Force orbital camera movement
    Orbital = 3,
}

/// Override for the player's movement mode if they're on mobile
///
#[doc = doc_link!("enum/DevTouchMovementMode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DevTouchMovementMode {
    /// Use the user's settings
    UserChoice = 0,
    /// Force virtual thumbstick based movement (stick that moves with the player's finger)
    Thumbstick = 1,
    /// Force virtual dpad based movement
    DPad = 2,
    /// Force virtual thumbpad based movement (stick that stays stationary)
    Thumbpad = 3,
    /// Force click-based movement
    ClickToMove = 4,
    /// Disable all default movement, all character movement will be handled by custom scripts
    Scriptable = 5,
    /// Force control via portrait-mode dynamic stick controls
    DynamicThumbstick = 6,
}

/// Controls whether multiple people can use a dialog at once
///
#[doc = doc_link!("enum/DialogBehaviorType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogBehaviorType {
    /// Only one player can use dialog at a time. Everyone can see the current dialog
    SinglePlayer = 0,
    /// Many players can use the dialog at once. Everyone sees their own dialog
    MultiplePlayers = 1,
}

/// Set the icon used by the dialog
///
#[doc = doc_link!("enum/DialogPurpose")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogPurpose {
    /// Shows an exclamation point - `!`
    Quest = 0,
    /// Shows a question mark - `?`
    Help = 1,
    /// Shows a dollar sign - `$`
    Shop = 2,
}

/// Set the color of the bar along the side of a dialog
///
#[doc = doc_link!("enum/DialogTone")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DialogTone {
    /// Dialog has a blue bar
    Neutral = 0,
    /// Dialog has a green bar
    Friendly = 1,
    /// Dialog has a red bar
    Enemy = 2,
}

/// Choose the axis used when setting a new GUI size in an aspect ratio constraint
///
#[doc = doc_link!("enum/DominantAxis")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum DominantAxis {
    /// Constrain along X axis
    Width = 0,
    /// Constrain along Y axis
    Height = 1,
}

/// Choose the direction of a [`Tween`](super::instance::Tween)
///
#[doc = doc_link!("enum/EasingDirection")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum EasingDirection {
    /// Ease 'in', start slowly and speed up
    In = 0,
    /// Ease 'out', start quickly and slow down
    Out = 1,
    /// Easy both in and out, speeding up towards the middle and slowing at either end
    InOut = 2,
}

/// Choose how a [`Tween`](super::instance::Tween) will move an object over its lifetime
///
#[doc = doc_link!("enum/EasingStyle")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum EasingStyle {
    /// Linear motion, LERP
    Linear = 0,
    /// Sine motion, smooth curve
    Sine = 1,
    /// Back motion, slightly overshoots then retreats
    Back = 2,
    /// Quad motion, smooth curve
    Quad = 3,
    /// Quart motion, starts very fast and slows rapidly
    Quart = 4,
    /// Quint motion, starts very fast and slows rapidly
    Quint = 5,
    /// Bounce motion, 'hit' the endpoint and bounce
    Bounce = 6,
    /// Elastic motion, like a rubber band is pulling it in
    Elastic = 7,
    /// Exponential motion, like quart or quint but even more so
    Exponential = 8,
    /// Circular motion, very smooth
    Circular = 9,
    /// Cubic motion, smooth curve
    Cubic = 10,
}

/// When a scrollbar in a [`ScrollingFrame`](super::instance::ScrollingFrame) will behave
/// elastically
///
#[doc = doc_link!("enum/ElasticBehavior")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ElasticBehavior {
    /// Elastic when there is content scrollable
    WhenScrollable = 0,
    /// Elastic all the time, content or not
    Always = 1,
    /// Elastic never
    Never = 2,
}

/// Whether an explosion should generate craters
///
#[doc = doc_link!("enum/ExplosionType")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ExplosionType {
    /// Explosion won't generate craters
    NoCraters = 0,
    /// Explosion will generate craters
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
pub enum GameAvatarType {
    R6 = 0,
    R15 = 1,
    PlayerChoice = 2,
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

/// The button pressed in a user input. Note that these map to the physical layout of a qwerty
/// keyboard, for most buttons. If the user is using DVORAK, then `WASD` will instead map to `,AOE`.
///
/// Not all keyboards will have every button on this list.
#[doc = doc_link!("enum/KeyCode")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum KeyCode {
    /// Unknown key pressed by the user
    Unknown = 0,
    /// Backspace key
    Backspace = 8,
    /// Tab key
    Tab = 9,
    /// Clear key
    Clear = 12,
    /// Return key
    Return = 13,
    /// Pause key
    Pause = 19,
    /// Esc key
    Escape = 27,
    /// Spacebar
    Space = 32,
    /// Double-quote key (shift+apostrophe)
    QuotedDouble = 34,
    /// Hash/Pound sign key (shift+3)
    Hash = 35,
    /// Dollar sign key (shift+4)
    Dollar = 36,
    /// Percent key (shift+5)
    Percent = 37,
    /// Ampersand key (shift+7)
    Ampersand = 38,
    /// Single-quote/apostrophe key
    Quote = 39,
    /// Left parenthesis key (shift+9)
    LeftParenthesis = 40,
    /// Right parenthesis key (shift+0)
    RightParenthesis = 41,
    /// Asterisk key (shift+8)
    Asterisk = 42,
    /// Plus key (shift+equals)
    Plus = 43,
    /// Comma key
    Comma = 44,
    /// Minus key
    Minus = 45,
    /// Period key
    Period = 46,
    /// Slash key
    Slash = 47,
    /// Zero key
    Zero = 48,
    /// One key
    One = 49,
    /// Two key
    Two = 50,
    /// Three key
    Three = 51,
    /// Four key
    Four = 52,
    /// Five key
    Five = 53,
    /// Six key
    Six = 54,
    /// Seven key
    Seven = 55,
    /// Eight key
    Eight = 56,
    /// Nine key
    Nine = 57,
    /// Colon key (shift+semicolon)
    Colon = 58,
    /// Semicolon key
    Semicolon = 59,
    /// Less-than/Left angle bracket key (shift+comma)
    LessThan = 60,
    /// Equals key
    Equals = 61,
    /// Greater-than/Right angle bracket key (shift+period)
    GreaterThan = 62,
    /// Question-mark key (shift+slash)
    Question = 63,
    /// At key (shift+2)
    At = 64,
    /// Left-bracket key
    LeftBracket = 91,
    /// Backslash key
    BackSlash = 92,
    /// Right-bracket key
    RightBracket = 93,
    /// Caret key (shift+6)
    Caret = 94,
    /// Underscore key (shift+minus)
    Underscore = 95,
    /// Backquote key
    Backquote = 96,
    /// A key
    A = 97,
    /// B key
    B = 98,
    /// C key
    C = 99,
    /// D key
    D = 100,
    /// E key
    E = 101,
    /// F key
    F = 102,
    /// G key
    G = 103,
    /// H key
    H = 104,
    /// I key
    I = 105,
    /// J key
    J = 106,
    /// K key
    K = 107,
    /// L key
    L = 108,
    /// M key
    M = 109,
    /// N key
    N = 110,
    /// O key
    O = 111,
    /// P key
    P = 112,
    /// Q key
    Q = 113,
    /// R key
    R = 114,
    /// S key
    S = 115,
    /// T key
    T = 116,
    /// U key
    U = 117,
    /// V key
    V = 118,
    /// W key
    W = 119,
    /// X key
    X = 120,
    /// Y key
    Y = 121,
    /// Z key
    Z = 122,
    /// Left curly-brace key (shift+left bracket)
    LeftCurly = 123,
    /// Pipe key (shift+backslash)
    Pipe = 124,
    /// Right curly-brace key (shift+right bracket)
    RightCurly = 125,
    /// Tilde key (shift+backquote)
    Tilde = 126,
    /// Delete key
    Delete = 127,
    /// World 0 Key
    World0 = 160,
    /// World 1 Key
    World1 = 161,
    /// World 2 Key
    World2 = 162,
    /// World 3 Key
    World3 = 163,
    /// World 4 Key
    World4 = 164,
    /// World 5 Key
    World5 = 165,
    /// World 6 Key
    World6 = 166,
    /// World 7 Key
    World7 = 167,
    /// World 8 Key
    World8 = 168,
    /// World 9 Key
    World9 = 169,
    /// World 10 Key
    World10 = 170,
    /// World 11 Key
    World11 = 171,
    /// World 12 Key
    World12 = 172,
    /// World 13 Key
    World13 = 173,
    /// World 14 Key
    World14 = 174,
    /// World 15 Key
    World15 = 175,
    /// World 16 Key
    World16 = 176,
    /// World 17 Key
    World17 = 177,
    /// World 18 Key
    World18 = 178,
    /// World 19 Key
    World19 = 179,
    /// World 20 Key
    World20 = 180,
    /// World 21 Key
    World21 = 181,
    /// World 22 Key
    World22 = 182,
    /// World 23 Key
    World23 = 183,
    /// World 24 Key
    World24 = 184,
    /// World 25 Key
    World25 = 185,
    /// World 26 Key
    World26 = 186,
    /// World 27 Key
    World27 = 187,
    /// World 28 Key
    World28 = 188,
    /// World 29 Key
    World29 = 189,
    /// World 30 Key
    World30 = 190,
    /// World 31 Key
    World31 = 191,
    /// World 32 Key
    World32 = 192,
    /// World 33 Key
    World33 = 193,
    /// World 34 Key
    World34 = 194,
    /// World 35 Key
    World35 = 195,
    /// World 36 Key
    World36 = 196,
    /// World 37 Key
    World37 = 197,
    /// World 38 Key
    World38 = 198,
    /// World 39 Key
    World39 = 199,
    /// World 40 Key
    World40 = 200,
    /// World 41 Key
    World41 = 201,
    /// World 42 Key
    World42 = 202,
    /// World 43 Key
    World43 = 203,
    /// World 44 Key
    World44 = 204,
    /// World 45 Key
    World45 = 205,
    /// World 46 Key
    World46 = 206,
    /// World 47 Key
    World47 = 207,
    /// World 48 Key
    World48 = 208,
    /// World 49 Key
    World49 = 209,
    /// World 50 Key
    World50 = 210,
    /// World 51 Key
    World51 = 211,
    /// World 52 Key
    World52 = 212,
    /// World 53 Key
    World53 = 213,
    /// World 54 Key
    World54 = 214,
    /// World 55 Key
    World55 = 215,
    /// World 56 Key
    World56 = 216,
    /// World 57 Key
    World57 = 217,
    /// World 58 Key
    World58 = 218,
    /// World 59 Key
    World59 = 219,
    /// World 60 Key
    World60 = 220,
    /// World 61 Key
    World61 = 221,
    /// World 62 Key
    World62 = 222,
    /// World 63 Key
    World63 = 223,
    /// World 64 Key
    World64 = 224,
    /// World 65 Key
    World65 = 225,
    /// World 66 Key
    World66 = 226,
    /// World 67 Key
    World67 = 227,
    /// World 68 Key
    World68 = 228,
    /// World 69 Key
    World69 = 229,
    /// World 70 Key
    World70 = 230,
    /// World 71 Key
    World71 = 231,
    /// World 72 Key
    World72 = 232,
    /// World 73 Key
    World73 = 233,
    /// World 74 Key
    World74 = 234,
    /// World 75 Key
    World75 = 235,
    /// World 76 Key
    World76 = 236,
    /// World 77 Key
    World77 = 237,
    /// World 78 Key
    World78 = 238,
    /// World 79 Key
    World79 = 239,
    /// World 80 Key
    World80 = 240,
    /// World 81 Key
    World81 = 241,
    /// World 82 Key
    World82 = 242,
    /// World 83 Key
    World83 = 243,
    /// World 84 Key
    World84 = 244,
    /// World 85 Key
    World85 = 245,
    /// World 86 Key
    World86 = 246,
    /// World 87 Key
    World87 = 247,
    /// World 88 Key
    World88 = 248,
    /// World 89 Key
    World89 = 249,
    /// World 90 Key
    World90 = 250,
    /// World 91 Key
    World91 = 251,
    /// World 92 Key
    World92 = 252,
    /// World 93 Key
    World93 = 253,
    /// World 94 Key
    World94 = 254,
    /// World 95 Key
    World95 = 255,
    /// Keypad 0 key
    KeypadZero = 256,
    /// Keypad 1 key
    KeypadOne = 257,
    /// Keypad 2 key
    KeypadTwo = 258,
    /// Keypad 3 key
    KeypadThree = 259,
    /// Keypad 4 key
    KeypadFour = 260,
    /// Keypad 5 key
    KeypadFive = 261,
    /// Keypad 6 key
    KeypadSix = 262,
    /// Keypad 7 key
    KeypadSeven = 263,
    /// Keypad 8 key
    KeypadEight = 264,
    /// Keypad 9 key
    KeypadNine = 265,
    /// Keypad period key
    KeypadPeriod = 266,
    /// Keypad slash key
    KeypadDivide = 267,
    /// Keypad asterisk key
    KeypadMultiply = 268,
    /// Keypad minus key
    KeypadMinus = 269,
    /// Keypad plus key
    KeypadPlus = 270,
    /// Keypad enter key
    KeypadEnter = 271,
    /// Keypad equals key
    KeypadEquals = 272,
    /// Up arrow key
    Up = 273,
    /// Down arrow key
    Down = 274,
    /// Right arrow key
    Right = 275,
    /// Left arrow key
    Left = 276,
    /// Insert key
    Insert = 277,
    /// Home key
    Home = 278,
    /// End key
    End = 279,
    /// Page up key
    PageUp = 280,
    /// Page down key
    PageDown = 281,
    /// Function 1 key
    F1 = 282,
    /// Function 2 key
    F2 = 283,
    /// Function 3 key
    F3 = 284,
    /// Function 4 key
    F4 = 285,
    /// Function 5 key
    F5 = 286,
    /// Function 6 key
    F6 = 287,
    /// Function 7 key
    F7 = 288,
    /// Function 8 key
    F8 = 289,
    /// Function 9 key
    F9 = 290,
    /// Function 10 key
    F10 = 291,
    /// Function 11 key
    F11 = 292,
    /// Function 12 key
    F12 = 293,
    /// Function 13 key
    F13 = 294,
    /// Function 14 key
    F14 = 295,
    /// Function 15 key
    F15 = 296,
    /// Num-lock key
    NumLock = 300,
    /// Caps-lock key
    CapsLock = 301,
    /// Scroll-lock key
    ScrollLock = 302,
    /// Right shift key
    RightShift = 303,
    /// Left shift key
    LeftShift = 304,
    /// Right control key
    RightControl = 305,
    /// Left control key
    LeftControl = 306,
    /// Right alt key
    RightAlt = 307,
    /// Left alt key
    LeftAlt = 308,
    /// Right meta/windows key
    RightMeta = 309,
    /// Left meta/windows key
    LeftMeta = 310,
    /// Left super key
    LeftSuper = 311,
    /// Right super key
    RightSuper = 312,
    /// Mode key
    Mode = 313,
    /// Compose key
    Compose = 314,
    /// Help key
    Help = 315,
    /// Print key
    Print = 316,
    /// SysReq key
    SysReq = 317,
    /// Break key
    Break = 318,
    /// Menu key
    Menu = 319,
    /// Power key
    Power = 320,
    /// Euro key
    Euro = 321,
    /// Undo key
    Undo = 322,
    /// X Button (controller)
    ButtonX = 1000,
    /// Y Button (controller)
    ButtonY = 1001,
    /// A Button (controller)
    ButtonA = 1002,
    /// B Button (controller)
    ButtonB = 1003,
    /// R1 Button (controller)
    ButtonR1 = 1004,
    /// L1 Button (controller)
    ButtonL1 = 1005,
    /// R2 Button (controller)
    ButtonR2 = 1006,
    /// L2 Button (controller)
    ButtonL2 = 1007,
    /// R3 Button (controller)
    ButtonR3 = 1008,
    /// L3 Button (controller)
    ButtonL3 = 1009,
    /// Start Button (controller)
    ButtonStart = 1010,
    /// Select Button (controller)
    ButtonSelect = 1011,
    /// D-pad left Button (controller)
    DPadLeft = 1012,
    /// D-pad right Button (controller)
    DPadRight = 1013,
    /// D-pad up Button (controller)
    DPadUp = 1014,
    /// D-pad down Button (controller)
    DPadDown = 1015,
    /// Thumbstick 1 Button (controller)
    Thumbstick1 = 1016,
    /// Thumbstick 2 Button (controller)
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
pub enum LoadCharacterLayeredClothing {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

/// The material of some [`part`](super::instance::BasePart) or [`terrain`](super::instance::Terrain)
///
#[doc = doc_link!("enum/Material")]
#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum Material {
    /// Plastic, the most common and default material for parts
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Plastic = 256,
    /// Smooth plastic, like plastic, but shinier and less bumpy
    ///
    /// [`BasePart`](super::instance::BasePart) only
    SmoothPlastic = 272,
    /// Neon, glowing bright and without without bumps.
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Neon = 288,
    /// Wood, like from a tree. Very grainy
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Wood = 512,
    /// Wooden planks, like you'd cut from a tree.
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    WoodPlanks = 528,
    /// Marble, white and fancy looking
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Marble = 784,
    /// Basalt, a dark and rough stone
    ///
    /// [`Terrain`](super::instance::BasePart) only
    Basalt = 788,
    /// Slate, a grainy neutral colored stone
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Slate = 800,
    /// Stone with lava shining through the cracks
    ///
    /// [`Terrain`](super::instance::Terrain) only
    CrackedLava = 804,
    /// Concrete, great for both buildings and sidewalks
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Concrete = 816,
    /// Limestone, a softer stone that is easily eroded
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Limestone = 820,
    /// Granite, a hard igneous rock
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Granite = 832,
    /// Pavement, that thing cars drive on
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Pavement = 836,
    /// Bricks, another good building choice
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Brick = 848,
    /// Pebbles, small loose stones all packed together
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Pebble = 864,
    /// Cobblestone, larger pieces of rock fit tightly in a loose pattern
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Cobblestone = 880,
    /// Rock, just plain stone
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Rock = 896,
    /// Sandstone, the firmament of the desert
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Sandstone = 912,
    /// Corroded metal, long past its prime, covered in rust
    ///
    /// [`BasePart`](super::instance::BasePart) only
    CorrodedMetal = 1040,
    /// Diamond plate, great for industrial installations
    ///
    /// [`BasePart`](super::instance::BasePart) only
    DiamondPlate = 1056,
    /// Shiny metal foil, keeps food fresh
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Foil = 1072,
    /// Clean metal, fresh off some forging installation
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Metal = 1088,
    /// Grass, covering lawns everywhwere
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Grass = 1280,
    /// Leafy grass, like it's autumn or something
    ///
    /// [`Terrain`](super::instance::Terrain) only
    LeafyGrass = 1284,
    /// Sand, the best and worst part of the beach
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Sand = 1296,
    /// Fabric, the stuff clothes are made of
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Fabric = 1312,
    /// Snow, a common sight in the winter
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Snow = 1328,
    /// Mud, the result of rain and dirt mixing too much
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Mud = 1344,
    /// Ground, that stuff you stand on
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Ground = 1360,
    /// Asphalt, a deep black pavement
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Asphalt = 1376,
    /// Salt, don't lick it or you'll need a drink
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Salt = 1392,
    /// Ice, slippery frozen water
    ///
    /// [`BasePart`](super::instance::BasePart) or [`Terrain`](super::instance::Terrain)
    Ice = 1536,
    /// Glacier, a very large sheet of ice
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Glacier = 1552,
    /// Glass, makes a better window than a door
    ///
    /// [`BasePart`](super::instance::BasePart) only
    Glass = 1568,
    /// Force-field, can protect a ship from space debris
    ///
    /// [`BasePart`](super::instance::BasePart) only
    ForceField = 1584,
    /// Air, mostly empty space
    ///
    /// [`Terrain`](super::instance::Terrain) only
    Air = 1792,
    /// Water, filler of oceans and puddled
    ///
    /// [`Terrain`](super::instance::Terrain) only
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
pub enum R15CollisionType {
    OuterBox = 0,
    InnerBox = 1,
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
pub enum ScreenOrientation {
    LandscapeLeft = 0,
    LandscapeRight = 1,
    LandscapeSensor = 2,
    Portrait = 3,
    Sensor = 4,
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
pub enum VirtualCursorMode {
    Default = 0,
    Disabled = 1,
    Enabled = 2,
}

#[derive(Debug, Copy, Clone, EnumConvert)]
pub enum ZIndexBehavior {
    Global = 0,
    Sibling = 1,
}
