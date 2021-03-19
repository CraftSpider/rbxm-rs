use core::fmt;
use std::error::Error as StdError;
use std::io::Error as IoError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadMagic,

    UnknownBlock(String),
    UnknownClass(i32),
    UnknownInstance(i32),
    UnknownCFrame(u8),
    UnknownProperty(u8),
    UnknownVariant(i32),

    WrongPropertyType(String),
    MissingProperty(String),
    UnconsumedProperties(String, Vec<String>),

    IoError(IoError),
    InvalidString,
    InvalidLz4,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Error::BadMagic => "Invalid File Magic".to_string(),
            Error::UnknownBlock(block) => format!("Unrecognized data-block type `{}`", block),
            Error::UnknownClass(id) => format!("Reference to unknown class with ID `{}`", id),
            Error::UnknownInstance(id) => format!("Reference to unknown instance with ID `{}`", id),
            Error::UnknownCFrame(id) => format!("Unknown CFrame type `{}`", id),
            Error::UnknownProperty(id) => format!("Unknown property type `{}`", id),
            Error::UnknownVariant(id) => format!("Unknown enum variant with ID `{}`", id),

            Error::WrongPropertyType(prop_name) => format!("Property {} was the wrong type", prop_name),
            Error::MissingProperty(prop_name) => format!("Property {} was missing", prop_name),
            Error::UnconsumedProperties(class_name, prop_names) => format!("Instance type {} had unexpected properties with names {:?}", class_name, prop_names),

            Error::IoError(err) => format!("Error in IO: {}", err),
            Error::InvalidString => "String contained invalid UTF data".to_string(),
            Error::InvalidLz4 => "LZ4 block couldn't be deserialized".to_string(),
        };

        write!(fmt, "RBXM Ser/De Error: {}", msg)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::IoError(err)
    }
}
