//! Base data types used primarily by Instance properties.

use crate::model::{FontStyle, FontWeight, Property};
use crate::tree::TreeKey;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

/// A set of named attributes for an instance. Wrapper type for a mapping of
/// string to property,
#[derive(Clone, Default)]
pub struct Attributes {
    pub(crate) backing: BTreeMap<String, Property>,
}

impl Deref for Attributes {
    type Target = BTreeMap<String, Property>;

    fn deref(&self) -> &Self::Target {
        &self.backing
    }
}

impl DerefMut for Attributes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.backing
    }
}

impl fmt::Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.backing)
    }
}

/// A dimensional component representing a scale and an offset
///
#[doc = doc_link!("datatype/UDim")]
#[derive(Debug, Clone, Default)]
pub struct UDim {
    /// The scale of this component
    pub scale: f32,
    /// The offset of this component
    pub offset: i32,
}

impl UDim {
    /// Create a new `UDim` from components
    #[must_use]
    pub const fn new(scale: f32, offset: i32) -> UDim {
        UDim { scale, offset }
    }
}

/// A type of coordinate representing a scale and an offset in XY space, most often
/// used in GUI objects
///
#[doc = doc_link!("datatype/UDim2")]
#[derive(Debug, Clone, Default)]
pub struct UDim2 {
    /// The x component of this coordinate
    pub x: UDim,
    /// The y component of this coordinate
    pub y: UDim,
}

impl UDim2 {
    /// Create a new `UDim2` from an x, y pair of [`UDims`](UDim)
    #[must_use]
    pub const fn new(x: UDim, y: UDim) -> UDim2 {
        UDim2 { x, y }
    }

    /// Create a new `UDim2` from x and y scale and offset components.
    #[must_use]
    pub const fn new_components(x_scale: f32, x_offset: i32, y_scale: f32, y_offset: i32) -> UDim2 {
        UDim2 {
            x: UDim::new(x_scale, x_offset),
            y: UDim::new(y_scale, y_offset),
        }
    }
}

/// A ray in 3D space, from origin extending along a unit axis
///
#[doc = doc_link!("datatype/Ray")]
#[derive(Debug, Clone, Default)]
pub struct Ray {
    /// The origin point
    pub origin: Vector3,
    /// The unit direction
    pub direction: Vector3,
}

impl Ray {
    /// Create a new ray from components. The direction vector will be converted to a unit vector
    /// if necessary.
    #[must_use]
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction: direction.unit(),
        }
    }
}

/// A set of faces an Instance is applied to
///
#[doc = doc_link!("datatype/Faces")]
#[derive(Debug, Clone, Default)]
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

impl Faces {
    /// Get a value with no faces selected
    #[must_use]
    pub const fn none() -> Faces {
        Faces {
            front: false,
            back: false,
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }

    /// Get a value with the front and back faces selected
    #[must_use]
    pub const fn front_back() -> Faces {
        Faces {
            front: true,
            back: true,
            ..Faces::none()
        }
    }

    /// Get a value with the top and bottom faces selected
    #[must_use]
    pub const fn top_bottom() -> Faces {
        Faces {
            top: true,
            bottom: true,
            ..Faces::none()
        }
    }

    /// Get a value with the left and right faces selected
    #[must_use]
    pub const fn left_right() -> Faces {
        Faces {
            left: true,
            right: true,
            ..Faces::none()
        }
    }

    /// Get a value with all faces selected
    #[must_use]
    pub const fn all() -> Faces {
        Faces {
            front: true,
            back: true,
            top: true,
            bottom: true,
            left: true,
            right: true,
        }
    }
}

/// A set of XYZ axes an Instance is applied to
///
#[doc = doc_link!("datatype/Axes")]
#[derive(Debug, Clone, Default)]
pub struct Axes {
    /// Applied to X axis
    pub x: bool,
    /// Applied to Y axis
    pub y: bool,
    /// Applied to Z axis
    pub z: bool,
}

impl Axes {
    /// Get a value with no axes selected
    #[must_use]
    pub const fn none() -> Axes {
        Axes {
            x: false,
            y: false,
            z: false,
        }
    }

    /// Get a value with all axes selected
    #[must_use]
    pub const fn all() -> Axes {
        Axes {
            x: true,
            y: true,
            z: true,
        }
    }
}

/// A color for an Instance, picked from a static palette. This type is deprecated, and only used
/// in a few places.
///
#[doc = doc_link!("datatype/BrickColor")]
#[derive(Debug, Clone, Default)]
pub struct BrickColor {
    /// The palette index of this BrickColor
    pub index: i32,
}

/// A 2D vector, most often used in GUI
///
#[doc = doc_link!("datatype/Vector2")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vector2 {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
}

impl Vector2 {
    /// Zero-vector, all components zeroed
    pub const ZERO: Vector2 = Vector2::new(0.0, 0.0);
    /// Unit vector in the X direction
    pub const UNIT_X: Vector2 = Vector2::new(1.0, 0.0);
    /// Unit vector in the Y direction
    pub const UNIT_Y: Vector2 = Vector2::new(0.0, 1.0);
    /// One-vector, all components are 1.0.
    pub const ONE: Vector2 = Vector2::new(1.0, 1.0);

    /// Create a new `Vector2` from components
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Get the length of this vector
    #[must_use]
    #[inline]
    pub fn len(&self) -> f32 {
        num::Float::sqrt(self.len_squared())
    }

    /// Get the length squared of this vector
    #[must_use]
    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Div for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

/// A 3D vector, used for most physical things
///
#[doc = doc_link!("datatype/Vector3")]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Vector3 {
    /// X component
    pub x: f32,
    /// Y component
    pub y: f32,
    /// Z component
    pub z: f32,
}

impl Vector3 {
    /// Zero-vector, all components zeroed
    pub const ZERO: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    /// Unit vector in the X direction
    pub const UNIT_X: Vector3 = Vector3::new(1.0, 0.0, 0.0);
    /// Unit vector in the Y direction
    pub const UNIT_Y: Vector3 = Vector3::new(0.0, 1.0, 0.0);
    /// Unit vector in the Z direction
    pub const UNIT_Z: Vector3 = Vector3::new(0.0, 0.0, 1.0);
    /// One-vector, all components are 1.0.
    pub const ONE: Vector3 = Vector3::new(1.0, 1.0, 1.0);

    /// Create a new `Vector3` from components
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Get the length of this vector
    #[must_use]
    #[inline]
    pub fn len(&self) -> f32 {
        num::Float::sqrt(self.len_squared())
    }

    /// Get the length squared of this vector
    #[must_use]
    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get the vector pointing the same direction as this vector, with a length of one, also known
    /// as a unit vector.
    #[must_use]
    pub fn unit(&self) -> Vector3 {
        let len = self.len();
        Vector3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

/// A representation of a point in space plus a rotation. This is basically a [`Vector3`] and a
/// rotation matrix.
///
#[doc = doc_link!("datatype/CFrame")]
#[derive(Debug, Clone)]
pub struct CFrame {
    /// The position in space this CFrame represents
    pub position: Vector3,
    /// The rotation angle this CFrame represents
    pub angle: [[f32; 3]; 3],
}

impl CFrame {
    /// Create a new `CFrame` from components
    #[must_use]
    pub const fn new(position: Vector3, angle: [[f32; 3]; 3]) -> CFrame {
        CFrame { position, angle }
    }
}

impl Default for CFrame {
    fn default() -> Self {
        CFrame {
            position: Vector3::default(),
            angle: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
        }
    }
}

/// A reference to another instance in the model, which may be null
#[derive(Debug, Clone)]
pub enum InstanceRef {
    /// A null instance reference
    Null,
    /// A reference to another instance in the model
    Item(TreeKey),
}

/// A 3D vector, with an underlying unsigned integer datatype.
///
#[doc = doc_link!("datatype/Vector3int16")]
#[derive(Debug, Clone, Default)]
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
#[doc = doc_link!("datatype/NumberSequenceKeypoint")]
#[derive(Debug, Clone, Default)]
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
#[doc = doc_link!("datatype/NumberSequence")]
#[derive(Debug, Clone, Default)]
pub struct NumberSequence {
    /// The keypoints contained in this sequence
    pub keypoints: Vec<NumberKeypoint>,
}

/// A keypoint in a [`ColorSequence`], a color value at a time, and the amount of variance that
/// might occur at that time.
///
#[doc = doc_link!("datatype/ColorSequenceKeypoint")]
#[derive(Debug, Clone, Default)]
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
#[doc = doc_link!("datatype/ColorSequence")]
#[derive(Debug, Clone, Default)]
pub struct ColorSequence {
    /// The keypoints contained in this sequence
    pub keypoints: Vec<ColorKeypoint>,
}

/// A range of possible values
///
#[doc = doc_link!("datatype/NumberRange")]
#[derive(Debug, Clone, Default)]
pub struct NumberRange {
    /// Low point of the range
    pub low: f32,
    /// High point of the range
    pub high: f32,
}

/// A rectangle in a 2D plane
///
#[doc = doc_link!("datatype/Rect")]
#[derive(Debug, Clone, Default)]
pub struct Rect {
    /// The top-left corner
    pub top_left: Vector2,
    /// The bottom-right corner
    pub bottom_right: Vector2,
}

/// A color with floating point RGB components, in the range of \[0-1\].
///
#[doc = doc_link!("datatype/Color3")]
#[derive(Debug, Clone, Default)]
pub struct Color3 {
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
}

impl Color3 {
    /// Create a new `Color3` from components
    ///
    /// # Panics
    ///
    /// In debug mode, if any component isn't within the range `[0.0, 1.0]`.
    #[must_use]
    pub fn new(r: f32, g: f32, b: f32) -> Color3 {
        debug_assert!((0.0..=1.0).contains(&r), "r value not in (0.0..=1.0)");
        debug_assert!((0.0..=1.0).contains(&g), "g value not in (0.0..=1.0)");
        debug_assert!((0.0..=1.0).contains(&b), "b value not in (0.0..=1.0)");
        Color3 { r, g, b }
    }
}

impl From<Color3Uint8> for Color3 {
    fn from(col: Color3Uint8) -> Self {
        Color3 {
            r: col.r as f32 / 255.,
            g: col.g as f32 / 255.,
            b: col.b as f32 / 255.,
        }
    }
}

/// A color with u8 RGB components, spanning the whole byte range. This isn't actually exposed
/// to the lua engine, instead shown as a Color3
#[derive(Debug, Clone, Default)]
pub struct Color3Uint8 {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
}

impl Color3Uint8 {
    /// Create a new `Color3Uint8` from an (r, g, b) set
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Color3Uint8 {
        Color3Uint8 { r, g, b }
    }
}

impl From<Color3> for Color3Uint8 {
    fn from(col: Color3) -> Self {
        Color3Uint8 {
            r: (col.r * 255.) as u8,
            g: (col.g * 255.) as u8,
            b: (col.b * 255.) as u8,
        }
    }
}

impl From<(u8, u8, u8)> for Color3Uint8 {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color3Uint8 { r, g, b }
    }
}

/// A pivot-point representing the 'center of mass' of a model, which it rotates around
#[derive(Debug, Clone, Default)]
pub struct Pivot {
    /// The position and orientation of the pivot in space
    pub cframe: CFrame,
    /// Unknown extra data
    // FIXME(CraftSpider)
    pub unknown: u8,
}

/// A set of physical properties, possibly user defined or not
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum PhysicalProperties {
    /// No custom physical properties, use default from part material
    Default,
    /// Custom physical properties
    Custom {
        /// The density of this part
        density: f32,
        /// The elasticity of collisions
        elasticity: f32,
        /// How much this parts elasticity affects the collision
        elasticity_weight: f32,
        /// The friction of collisions
        friction: f32,
        /// How much this parts friction affects the collision
        friction_weight: f32,
    },
}

impl Default for PhysicalProperties {
    fn default() -> Self {
        PhysicalProperties::Default
    }
}

/// A font property
#[derive(Debug, Clone)]
pub struct FontFace {
    /// The font family, such as Calibri or Times
    pub family: String,
    /// The font weight - from thin to heavy
    pub weight: FontWeight,
    /// The font style, regular or italic
    pub style: FontStyle,
    /// The cached font-face ID
    pub cached_face_id: String,
}

/// A full triangle mesh, used for collision or display
#[cfg(feature = "mesh-format")]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TriMesh {
    /// Default tri-mesh data
    Default,
    /// A box mesh, no special collision
    Box,
    /// A convex-hull based mesh
    Hull {
        /// Total volume of the mesh
        volume: f32,
        /// The center of gravity for the whole mesh
        center_of_gravity: Vector3,
        /// The inertia tensor of the whole mesh
        inertia_tensor: [[f32; 3]; 3],
        /// The set of convex hulls that make up this mesh
        meshes: Vec<ConvexHull>,
    },
}

#[cfg(feature = "mesh-format")]
impl Default for TriMesh {
    fn default() -> Self {
        TriMesh::Default
    }
}

/// A single convex hull, with relevant data
#[cfg(feature = "mesh-format")]
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct ConvexHull {
    /// Unknown, possibly triangle indices
    // FIXME(CraftSpider)
    pub unknown_1: Vec<u8>,
    /// Unknown, possibly transform offsets
    // FIXME(CraftSpider)
    pub unknown_2: Vec<u8>,
    /// Triangle vertices
    pub vertices: Vec<Vector3>,
    /// Face indices into the vertex list
    pub faces: Vec<(usize, usize, usize)>,
}

#[cfg(feature = "mesh-format")]
impl ConvexHull {
    /// Create a new ConvexHull from a set of vertices and faces
    pub fn new(vertices: Vec<Vector3>, faces: Vec<(usize, usize, usize)>) -> ConvexHull {
        ConvexHull {
            unknown_1: Vec::new(),
            unknown_2: Vec::new(),
            vertices,
            faces,
        }
    }
}
