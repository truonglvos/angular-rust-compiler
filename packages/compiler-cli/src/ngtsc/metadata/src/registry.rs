//! Metadata registry for Angular decorators.
//!
//! This module provides traits and implementations for reading metadata from TypeScript/JavaScript AST.
//! Matches TypeScript's registry.ts

use super::api::DecoratorMetadata;
use oxc_ast::ast::Program;
use std::path::Path;

/// Trait for reading Angular decorator metadata from source files.
/// This is the primary interface for metadata extraction.
///
/// Note: The trait uses `'static` lifetime for backward compatibility.
/// For lifetime-aware usage, use `OxcMetadataReader::get_directive_metadata_with_lifetime`.
pub trait MetadataReader {
    /// Extract all Angular decorator metadata from a program AST.
    /// Returns metadata with `'static` lifetime (decorator references are cleared).
    fn get_directive_metadata(
        &self,
        program: &Program,
        path: &Path,
    ) -> Vec<DecoratorMetadata<'static>>;
}

/// OXC-based implementation of MetadataReader.
/// Uses the OXC parser to analyze TypeScript/JavaScript files.
pub struct OxcMetadataReader;

impl OxcMetadataReader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OxcMetadataReader {
    fn default() -> Self {
        Self::new()
    }
}
