//! Docs Source
//!
//! Core documentation extraction logic.

pub mod entities;
pub mod extractor;
pub mod class_extractor;
pub mod constant_extractor;
pub mod decorator_extractor;
pub mod enum_extractor;
pub mod function_extractor;
pub mod generics_extractor;
pub mod interface_extractor;
pub mod jsdoc_extractor;
pub mod type_extractor;

pub use entities::*;
pub use extractor::*;
