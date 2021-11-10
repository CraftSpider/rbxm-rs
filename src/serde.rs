//! The ability to serialize/deserialize an RBXM file

pub mod de;
pub mod encoding;
pub mod error;
pub(crate) mod internal;
pub mod io;
pub mod ser;

#[cfg(feature = "std")]
pub use de::from_file;
pub use de::from_reader;
pub use error::{Error, Result};
#[cfg(feature = "std")]
pub use ser::to_file;
pub use ser::to_writer;
