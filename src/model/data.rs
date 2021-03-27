//! Base data types used primarily by Instance properties.

/// A dimensional component representing a scale and an offset
///
/// **Reference Link**: [datatype/UDim](https://developer.roblox.com/en-us/api-reference/datatype/UDim)
#[derive(Debug, Clone)]
pub struct UDim {
    /// The scale of this component
    pub scale: f32,
    /// The offset of this component
    pub offset: i32,
}

/// A type of coordinate representing a scale and an offset in XY space, most often
/// used in GUI objects
///
/// **Reference Link**: [datatype/UDim2](https://developer.roblox.com/en-us/api-reference/datatype/UDim2)
#[derive(Debug, Clone)]
pub struct UDim2 {
    /// The x component of this coordinate
    pub x: UDim,
    /// The y component of this coordinate
    pub y: UDim,
}

/// A ray in 3D space, from origin extending along a unit axis
///
/// **Reference Link**: [datatype/Ray](https://developer.roblox.com/en-us/api-reference/datatype/Ray)
#[derive(Debug, Clone)]
pub struct Ray {
    /// The origin point
    pub origin: Vector3,
    /// The unit direction
    pub direction: Vector3,
}

/// A set of faces an Instance is applied to
///
/// **Reference Link**: [datatype/Faces](https://developer.roblox.com/en-us/api-reference/datatype/Faces)
#[derive(Debug, Clone)]
pub struct Faces {
    /// Applied to front face
    pub front: bool,
    /// Applied to back face
    pub back: bool,
    /// Applied to top face
    pub top: bool,
    /// Applied to bottom face
    pub bottom: bool,
    /// Applied to left face
    pub left: bool,
    /// Applied to right face
    pub right: bool,
}

/// A set of XYZ axes an Instance is applied to
///
/// **Reference Link**: [datatype/Axes](https://developer.roblox.com/en-us/api-reference/datatype/Axes)
#[derive(Debug, Clone)]
pub struct Axes {
    /// Applied to X axis
    pub x: bool,
    /// Applied to Y axis
    pub y: bool,
    /// Applied to Z axis
    pub z: bool,
}

/// A color for an Instance, picked from a static palette. This type is deprecated, and only used
/// in a few places.
///
/// **Reference Link**: [datatype/BrickColor](https://developer.roblox.com/en-us/api-reference/datatype/BrickColor)
#[derive(Debug, Clone)]
pub struct BrickColor {
    /// The palette index of this BrickColor
    pub index: i32,
}

/// A 2D vector, most often used in GUI
///
/// **Reference Link**: [datatype/Vector2](https://developer.roblox.com/en-us/api-reference/datatype/Vector2)
#[derive(Debug, Clone)]
pub struct Vector2 {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
}

/// A 3D vector, used for most physical things
///
/// **Reference Link**: [datatype/Vector3](https://developer.roblox.com/en-us/api-reference/datatype/Vector3)
#[derive(Debug, Clone)]
pub struct Vector3 {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
    /// Z component
    pub z: f32,
}

/// A representation of a point in space plus a rotation. This is basically a [`Vector3`] and a
/// rotation matrix.
///
/// **Reference Link**: [datatype/CFrame](https://developer.roblox.com/en-us/api-reference/datatype/CFrame)
#[derive(Debug, Clone)]
pub struct CFrame {
    /// The position in space this CFrame represents
    pub position: Vector3,
    /// The rotation angle this CFrame represents
    pub angle: [[f32; 3]; 3],
}

/// A 3D vector, with an underlying unsigned integer datatype.
///
/// **Reference Link**: [datatype/Vector3int16](https://developer.roblox.com/en-us/api-reference/datatype/Vector3int16)
#[derive(Debug, Clone)]
pub struct Vector3Int16 {
    /// X component
    pub x: i16,
    /// Y component
    pub y: i16,
    /// Z component
    pub z: i16,
}

/// A keypoint in a [`NumberSequence`], a value at a time, and the amount of variance that might
/// occur at that time.
///
/// **Reference Link**: [datatype/NumberSequenceKeypoint](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequenceKeypoint)
#[derive(Debug, Clone)]
pub struct NumberKeypoint {
    /// The time of this keypoint
    pub time: f32,
    /// The value at the given time
    pub value: f32,
    /// The possible variance of this value
    pub envelope: f32,
}

/// A sequence of values, often used for particles or over-time effects
///
/// **Reference Link**: [datatype/NumberSequence](https://developer.roblox.com/en-us/api-reference/datatype/NumberSequence)
#[derive(Debug, Clone)]
pub struct NumberSequence {
    /// The keypoints contained in this sequence
    pub keypoints: Vec<NumberKeypoint>,
}

/// A keypoint in a [`ColorSequence`], a color value at a time, and the amount of variance that
/// might occur at that time.
///
/// **Reference Link**: [datatype/ColorSequenceKeypoint](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequenceKeypoint)
#[derive(Debug, Clone)]
pub struct ColorKeypoint {
    /// The time of this keypoint
    pub time: f32,
    /// The color value at the given time
    pub color: Color3,
    /// The possible variance of this value
    pub envelope: f32,
}

/// A sequence of color values, often used for particles or over-time effects
///
/// **Reference Links**: [datatype/ColorSequence](https://developer.roblox.com/en-us/api-reference/datatype/ColorSequence)
#[derive(Debug, Clone)]
pub struct ColorSequence {
    /// The keypoints contained in this sequence
    pub keypoints: Vec<ColorKeypoint>,
}

/// A range of possible values
///
/// **Reference Links**: [datatype/NumberRange](https://developer.roblox.com/en-us/api-reference/datatype/NumberRange)
#[derive(Debug, Clone)]
pub struct NumberRange {
    /// Low point of the range
    pub low: f32,
    /// High point of the range
    pub high: f32,
}

/// A rectangle in a 2D plane
///
/// **Reference Links**: [datatype/Rect](https://developer.roblox.com/en-us/api-reference/datatype/Rect)
#[derive(Debug, Clone)]
pub struct Rect {
    /// The top-left corner
    pub top_left: Vector2,
    /// The bottom-right corner
    pub bottom_right: Vector2,
}

/// A color with floating point RGB components, in the range of \[0-1\].
///
/// **Reference Links**: [datatype/Color3](https://developer.roblox.com/en-us/api-reference/datatype/Color3)
#[derive(Debug, Clone)]
pub struct Color3 {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
}

/// A color with u8 RGB components, spanning the whole byte range. This isn't actually exposed
/// to the lua engine, instead shown as a Color3
#[derive(Debug, Clone)]
pub struct Color3Uint8 {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
}
