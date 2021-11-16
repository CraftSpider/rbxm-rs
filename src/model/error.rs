//! Error types for Roblox model related activities

use crate::tree;

use core::fmt;

/// The error type returned by model operations that may fail
#[derive(Debug)]
pub enum Error {
    /// Model path doesn't match expected syntax
    InvalidPath,
    /// Model path matches multiple instances
    AmbiguousPath,
    /// Model path matches no instances
    NotFound,
    /// The model node is already borrowed incompatibly with the operation
    CantBorrow,
}

impl From<tree::Error> for Error {
    fn from(e: tree::Error) -> Self {
        match e {
            tree::Error::Missing => Error::NotFound,
            tree::Error::CantBorrow => Error::CantBorrow,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidPath => write!(fmt, "Invalid path syntax"),
            Error::AmbiguousPath => write!(fmt, "Path matched multiple items"),
            Error::NotFound => write!(fmt, "Path didn't match any items"),
            Error::CantBorrow => {
                write!(fmt, "Path attempted to reference node already borrowed")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
