#[derive(Debug, Clone)]
pub struct UDim {
    pub scale: f32,
    pub offset: i32,
}

#[derive(Debug, Clone)]
pub struct UDim2 {
    pub x_scale: f32,
    pub y_scale: f32,
    pub x_offset: i32,
    pub y_offset: i32,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub front: bool,
    pub back: bool,
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Clone)]
pub struct Axis {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

#[derive(Debug, Clone)]
pub struct BrickColor {
    pub index: i32,
}

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct CFrame {
    pub position: Vector3,
    pub angle: [[f32; 3]; 3],
}

#[derive(Debug, Clone)]
pub struct Vector3Int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Debug, Clone)]
pub struct NumberKeypoint {
    pub time: f32,
    pub value: f32,
    pub envelope: f32,
}

#[derive(Debug, Clone)]
pub struct NumberSequence {
    pub keypoints: Vec<NumberKeypoint>,
}

#[derive(Debug, Clone)]
pub struct ColorKeypoint {
    pub time: f32,
    pub color: Color3,
    pub envelope: f32,
}

#[derive(Debug, Clone)]
pub struct ColorSequence {
    pub keypoints: Vec<ColorKeypoint>,
}

#[derive(Debug, Clone)]
pub struct NumberRange {
    pub low: f32,
    pub high: f32,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub top_left: Vector2,
    pub bottom_right: Vector2,
}

#[derive(Debug, Clone)]
pub struct Color3 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub struct Color3Uint8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
