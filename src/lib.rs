#![deny(clippy::all)]

pub mod model;
pub mod serde;

pub use model::Error as ModelError;
pub use model::Instance;
pub use model::RbxModel;
pub use serde::Error as SerdeError;
