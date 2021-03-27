//! Representation of a Roblox model in memory. The entire model is an [`RbxModel`], which contains
//! [`Instances`][Instance]. These have a kind/class, which should be matched on to retrieve
//! kind-specific data. If the system doesn't recognize the class of an instance, it uses a special
//! kind called [`Other`][InstanceKind::Other], which contains the classname and a raw set of
//! [`Properties`][Property].

pub mod data;
pub mod enums;
pub mod error;
pub mod instance;
pub mod property;
mod rbx_model;

pub use data::*;
pub use enums::*;
pub use error::{ModelError, InstanceError};
pub use instance::{Instance, InstanceKind};
pub use property::Property;
pub use rbx_model::RbxModel;

type OwnedInstance = std::rc::Rc<std::cell::RefCell<Instance>>;
type InstanceRef = std::rc::Weak<std::cell::RefCell<Instance>>;
