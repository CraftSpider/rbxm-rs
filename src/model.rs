//! Representation of a Roblox model in memory. The entire model is an [`RbxModel`], which contains
//! [`Instances`](Instance). These should be matched on to retrieve kind-specific data. If the
//! system doesn't recognize the class of an instance, it uses a special kind called
//! [`Other`](Instance::Other), which contains the classname and a raw set of
//! [`Properties`](Property).

pub mod data;
pub mod enums;
pub mod error;
pub mod instance;
pub mod property;
mod rbx_model;

pub use data::*;
pub use enums::*;
pub use error::Error;
pub use instance::Instance;
pub use property::Property;
pub use rbx_model::RbxModel;
