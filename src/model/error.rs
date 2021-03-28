//! Error types for Roblox model related activities

use core::fmt;

/// The error type returned by model operations that may fail
#[derive(Debug)]
pub enum ModelError {
    /// Model path doesn't match expected syntax
    InvalidPath,
    /// Model path matches multiple instances
    AmbiguousPath,
    /// Model path matches no instances
    NotFound,
}

impl fmt::Display for ModelError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelError::InvalidPath => write!(fmt, "Invalid path syntax"),
            ModelError::AmbiguousPath => write!(fmt, "Path matched multiple items"),
            ModelError::NotFound => write!(fmt, "Path didn't match any items"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ModelError {}

/// The error type returned by instance operations that may fail
#[derive(Debug)]
pub enum InstanceError {
    /// An instance couldn't be mutably borrowed
    FailedBorrowMut,
    /// An instance already has another instance as a child
    AlreadyHasChild,
    /// An instance doesn't have another instance as a child
    NoSuchChild,
}

impl fmt::Display for InstanceError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstanceError::FailedBorrowMut => write!(
                fmt,
                "Instance could not be mutably borrowed, as it is currently immutably borrowed"
            ),
            InstanceError::AlreadyHasChild => write!(fmt, "Instance is already a child of parent"),
            InstanceError::NoSuchChild => write!(fmt, "Instance is not a child of parent"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InstanceError {}
