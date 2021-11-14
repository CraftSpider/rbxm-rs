//! Common error handling machinery for serialization/deserialization

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// A result used for all ser/de methods that return a result
pub type Result<T> = core::result::Result<T, Error>;

/// A common error handling type for ser/de.
#[derive(Debug)]
pub struct Error {
    #[cfg(all(feature = "std", feature = "unstable"))]
    backtrace: std::backtrace::Backtrace,
    /// The kind of this error, exposed for ease of matching by users
    pub kind: ErrorKind,
    _priv: (),
}

impl Error {
    pub(crate) fn from_kind(kind: ErrorKind) -> Error {
        Error {
            #[cfg(all(feature = "std", feature = "unstable"))]
            backtrace: std::backtrace::Backtrace::capture(),
            kind,
            _priv: (),
        }
    }

    pub(crate) fn bad_magic() -> Error {
        Error::from_kind(ErrorKind::BadMagic)
    }

    pub(crate) fn unknown_block(name: String) -> Error {
        Error::from_kind(ErrorKind::UnknownBlock(name))
    }

    pub(crate) fn unknown_class(id: i32) -> Error {
        Error::from_kind(ErrorKind::UnknownClass(id))
    }

    pub(crate) fn unknown_instance(inst: i32) -> Error {
        Error::from_kind(ErrorKind::UnknownInstance(inst))
    }

    pub(crate) fn unknown_cframe(id: u8) -> Error {
        Error::from_kind(ErrorKind::UnknownCFrame(id))
    }

    pub(crate) fn unknown_property(id: u8) -> Error {
        Error::from_kind(ErrorKind::UnknownProperty(id))
    }

    pub(crate) fn unknown_variant(id: i32) -> Error {
        Error::from_kind(ErrorKind::UnknownVariant(id))
    }

    #[cfg(feature = "mesh-format")]
    pub(crate) fn unknown_mesh(id: i32) -> Error {
        Error::from_kind(ErrorKind::UnknownMesh(id))
    }

    pub(crate) fn wrong_property_type(name: String) -> Error {
        Error::from_kind(ErrorKind::WrongPropertyType(name))
    }

    pub(crate) fn missing_property(name: String) -> Error {
        Error::from_kind(ErrorKind::MissingProperty(name))
    }

    pub(crate) fn unconsumed_properties(ty: String, remaining: Vec<String>) -> Error {
        Error::from_kind(ErrorKind::UnconsumedProperties(ty, remaining))
    }

    pub(crate) fn inconsistent_tree() -> Error {
        Error::from_kind(ErrorKind::InconsistentTree)
    }

    pub(crate) fn invalid_string() -> Error {
        Error::from_kind(ErrorKind::InvalidString)
    }

    pub(crate) fn invalid_lz4() -> Error {
        Error::from_kind(ErrorKind::InvalidLz4)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RBXM Ser/De Error: {}", self.kind)?;
        #[cfg(all(feature = "std", feature = "unstable"))]
        {
            writeln!(f, "{}", self.backtrace)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::from_kind(ErrorKind::IoError(err))
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::IoError(err) => Some(err),
            _ => None,
        }
    }

    #[cfg(feature = "unstable")]
    fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        Some(&self.backtrace)
    }
}

/// The underlying cause of a serde error
#[derive(Debug)]
pub enum ErrorKind {
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

impl fmt::Display for ErrorKind {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ErrorKind::BadMagic => "Invalid File Magic".to_string(),
            ErrorKind::UnknownBlock(block) => format!("Unrecognized data-block type `{}`", block),
            ErrorKind::UnknownClass(id) => format!("Reference to unknown class with ID `{}`", id),
            ErrorKind::UnknownInstance(id) => format!("Reference to unknown instance with ID `{}`", id),
            ErrorKind::UnknownCFrame(id) => format!("Unknown CFrame type `{}`", id),
            ErrorKind::UnknownProperty(id) => format!("Unknown property type `{}`", id),
            ErrorKind::UnknownVariant(id) => format!("Unknown enum variant with ID `{}`", id),
            ErrorKind::UnknownMesh(id) => format!("Unknown physics mesh kind with ID `{}`", id),

            ErrorKind::WrongPropertyType(prop_name) => {
                format!("Property {} was the wrong type", prop_name)
            }
            ErrorKind::MissingProperty(prop_name) => format!("Property {} was missing", prop_name),
            ErrorKind::UnconsumedProperties(class_name, prop_names) => format!(
                "Instance type {} had unexpected properties with names {:?}",
                class_name, prop_names
            ),
            ErrorKind::InconsistentTree => {
                String::from("RBXM parent->child relationships were inconsistent")
            }

            ErrorKind::IoError(err) => format!("Error in IO: {}", err),
            ErrorKind::InvalidString => "String contained invalid UTF data".to_string(),
            ErrorKind::InvalidLz4 => "LZ4 block couldn't be deserialized".to_string(),
        };
        write!(fmt, "{}", msg)
    }
}
