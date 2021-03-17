use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Instance {
    pub children: Vec<Rc<RefCell<Instance>>>,
    pub parent: Weak<RefCell<Instance>>,
    pub kind: String,
    pub properties: HashMap<String, Property>,
}

impl Instance {
    pub fn new(kind: String, parent: Option<&Rc<RefCell<Instance>>>) -> Instance {
        Instance {
            children: vec![],
            parent: parent.map(|rc| Rc::downgrade(rc)).unwrap_or_default(),
            kind,
            // TODO: Default properties, merge them into Kind and make it exhaustive?
            //       Idea: InstanceKind with known properties, etc, then an InstanceKind::Other
            //       which just has a name and HashMap, for unknown kinds
            properties: HashMap::new(),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum InstanceKind {
    Part(Part),
    Other(HashMap<String, Property>),
}

#[derive(Debug, Clone)]
pub struct Base {
    name: String,
    tags: String,
    source_asset_id: i64,
    attributes_serialize: String,
}

#[derive(Debug, Clone)]
pub struct BasePart {
    anchored: bool,
    locked: bool,
    massless: bool,
    can_touch: bool,
    can_collide: bool,
    cast_shadow: bool,

    cframe: CFrame,
    velocity: Vector3,
    rot_velocity: Vector3,

    size: Vector3,
    material: i32,
    transparency: f32,
    reflectance: f32,

    collision_group_id: i32,
    custom_physical_properties: bool,
    root_priority: i32,

    top_surface: i32,
    top_surface_input: i32,
    top_param_a: f32,
    top_param_b: f32,

    bottom_surface: i32,
    bottom_surface_input: i32,
    bottom_param_a: f32,
    bottom_param_b: f32,

    front_surface: i32,
    front_surface_input: i32,
    front_param_a: f32,
    front_param_b: f32,

    back_surface: i32,
    back_surface_input: i32,
    back_param_a: f32,
    back_param_b: f32,

    left_surface: i32,
    left_surface_input: i32,
    left_param_a: f32,
    left_param_b: f32,

    right_surface: i32,
    right_surface_input: i32,
    right_param_a: f32,
    right_param_b: f32,
}

#[derive(Debug, Clone)]
pub struct Part {
    base: Base,
    base_part: BasePart,
    form_factor_raw: i32,
    color3_uint8: Color3Uint8, // Suspect this is part of BasePart
    shape: i32,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Property {
    BinaryString(Vec<u8>),
    TextString(String),
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    UDim(UDim),
    UDim2(UDim2),
    Ray(Ray),
    Face(Face),
    Axis(Axis),
    BrickColor(BrickColor), // Deprecated
    Color3(Color3),
    Vector2(Vector2),
    Vector3(Vector3),
    CFrame(CFrame),
    // Quaternion, // Not implemented properly by RBX?
    Enum(i32),
    InstanceRef(Weak<RefCell<Instance>>),
    Vector3Int16(Vector3Int16),
    NumberSequence(NumberSequence),
    ColorSequence(ColorSequence),
    NumberRange(NumberRange),
    Rect(Rect),
    CustomPhysicalProperties(bool),
    Color3Uint8(Color3Uint8),
}

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
