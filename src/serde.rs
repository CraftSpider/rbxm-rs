//! The ability to serialize/deserialize an RBXM file

pub mod de;
pub mod error;
pub(crate) mod internal;
pub mod ser;

pub use de::{from_file, from_reader};
pub use error::{Error, Result};
pub use ser::{to_file, to_writer};
