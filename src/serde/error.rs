//! Common error handling machinery for serialization/deserialization

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// A result used for all ser/de methods that return a result
pub type Result<T> = core::result::Result<T, Error>;

/// A common error handling type for ser/de.
#[derive(Debug)]
pub enum Error {
    /// RBXM file had invalid magic bytes at the start/end
    BadMagic,

    /// An RBXM block name wasn't recognized (A block name is always four or
    /// fewer uppercase characters)
    UnknownBlock(String),
    /// A class index wasn't recognized, generally a PROP block references an invalid INST block
    UnknownClass(i32),
    /// An instance ID wasn't recognized, generally a PRNT block reference an invalid instance ID
    UnknownInstance(i32),
    /// A CFrame ID (indicating certain special values) was unrecognized
    UnknownCFrame(u8),
    /// A property type ID wasn't recognized
    UnknownProperty(u8),
    /// An enum had an ID that wasn't recognized as a valid variant
    UnknownVariant(i32),
    /// A Mesh kind ID (indicating mesh type) was unrecognized
    UnknownMesh(i32),

    /// A property successfully parse, but was an unexpected type for the Instance it
    /// was attached to
    WrongPropertyType(String),
    /// An instance successfully parsed, but was missing an expected property
    MissingProperty(String),
    /// An instance successfully parsed, but contained more properties than expected
    UnconsumedProperties(String, Vec<String>),
    /// A parent->child relationship was encoded in the RBXM incorrectly
    InconsistentTree,

    /// The input experienced an underlying IO error
    IoError(
        #[cfg(feature = "std")] std::io::Error,
        #[cfg(not(feature = "std"))] &'static str,
    ),
    /// A string value contained invalid bytes
    InvalidString,
    /// An LZ4 block contained invalid bytes
    InvalidLz4,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::BadMagic => "Invalid File Magic".to_string(),
            Error::UnknownBlock(block) => format!("Unrecognized data-block type `{}`", block),
            Error::UnknownClass(id) => format!("Reference to unknown class with ID `{}`", id),
            Error::UnknownInstance(id) => format!("Reference to unknown instance with ID `{}`", id),
            Error::UnknownCFrame(id) => format!("Unknown CFrame type `{}`", id),
            Error::UnknownProperty(id) => format!("Unknown property type `{}`", id),
            Error::UnknownVariant(id) => format!("Unknown enum variant with ID `{}`", id),
            Error::UnknownMesh(id) => format!("Unknown physics mesh kind with ID `{}`", id),

            Error::WrongPropertyType(prop_name) => {
                format!("Property {} was the wrong type", prop_name)
            }
            Error::MissingProperty(prop_name) => format!("Property {} was missing", prop_name),
            Error::UnconsumedProperties(class_name, prop_names) => format!(
                "Instance type {} had unexpected properties with names {:?}",
                class_name, prop_names
            ),
            Error::InconsistentTree => {
                String::from("RBXM parent->child relationships were inconsistent")
            }

            Error::IoError(err) => format!("Error in IO: {}", err),
            Error::InvalidString => "String contained invalid UTF data".to_string(),
            Error::InvalidLz4 => "LZ4 block couldn't be deserialized".to_string(),
        };

        write!(fmt, "RBXM Ser/De Error: {}", msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            _ => None,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}
