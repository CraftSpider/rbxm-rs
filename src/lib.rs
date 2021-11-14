//! A crate providing the ability to read and manipulate Roblox models from the RBXM format.
//!
//! # Structure
//!
//! The crate is divided into two main pieces: Model and Serde.
//! The model components are the actual representation of a Roblox model in Rust.
//! The serde components are the ability to read and write that representation to a file format
//! that can be understood by Roblox.
//!
//! # Examples
//!
//! Parse a model from a file, find an [`Instance`] by path, and print the instance's class.
//!
//! ```ignore
//! use rbxm::{SerdeError, ModelError, from_file};
//!
//! let model = match rbxm::from_file("./examples/BrickBase.rbxm") {
//!     Ok(model) => model,
//!     Err(SerdeError::IoError(err)) => panic!("IO Error: {}", err),
//!     Err(err) => panic!("RBXM Parsing Error: {}", err),
//! };
//!
//! let part = match model.get_path("Part") {
//!     Ok(part) => part,
//!     Err(ModelError::NotFound) => panic!("Couldn't find instance named \"Part\""),
//!     Err(ModelError::AmbiguousPath) => panic!("Found more than one instance named \"Part\""),
//!     Err(err) => panic!("Model Error: {}", err),
//! };
//!
//! println!("Part Class: {}", part.class_name())
//! ```
//!

#![deny(clippy::all)]
#![deny(
    missing_docs,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    missing_abi,
    noop_method_call,
    pointer_structural_match,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    unused_lifetimes,
    clippy::cargo,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal
)]
#![cfg_attr(docsrs, features(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "std", feature = "unstable"), feature(backtrace))]

#[macro_use]
extern crate alloc;

mod __external {}

pub mod model;
pub mod serde;
mod tree;

pub use model::{InstanceError, ModelError, RbxModel};
pub use serde::Error as SerdeError;
#[cfg(feature = "std")]
pub use serde::{from_file, to_file};
