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
pub use error::{InstanceError, ModelError};
pub use instance::{Instance, InstanceKind};
pub use property::Property;
pub use rbx_model::RbxModel;

type OwnedInstance = alloc::rc::Rc<core::cell::RefCell<Instance>>;
type InstanceRef = alloc::rc::Weak<core::cell::RefCell<Instance>>;
