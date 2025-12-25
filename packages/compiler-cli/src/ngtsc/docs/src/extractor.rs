// Docs Extractor
//
// Main documentation extractor that coordinates other extractors.

use super::entities::*;
use std::collections::HashMap;

/// Documentation extraction options.
#[derive(Debug, Clone, Default)]
pub struct ExtractorOptions {
    /// Whether to include private members.
    pub include_private: bool,
    /// Whether to include internal APIs.
    pub include_internal: bool,
    /// File patterns to include.
    pub include_patterns: Vec<String>,
    /// File patterns to exclude.
    pub exclude_patterns: Vec<String>,
}

/// Result of documentation extraction.
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    /// All extracted entries.
    pub entries: Vec<DocEntry>,
    /// Class entries.
    pub classes: Vec<ClassEntry>,
    /// Function entries.
    pub functions: Vec<FunctionEntry>,
    /// Enum entries.
    pub enums: Vec<EnumEntry>,
    /// Diagnostics.
    pub diagnostics: Vec<String>,
}

impl ExtractionResult {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            classes: Vec::new(),
            functions: Vec::new(),
            enums: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}

impl Default for ExtractionResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Main documentation extractor.
pub struct DocsExtractor {
    /// Extraction options.
    options: ExtractorOptions,
    /// Extracted entries by file.
    entries_by_file: HashMap<String, Vec<DocEntry>>,
}

impl DocsExtractor {
    pub fn new(options: ExtractorOptions) -> Self {
        Self {
            options,
            entries_by_file: HashMap::new(),
        }
    }

    /// Extract documentation from source files.
    pub fn extract(&mut self, source_files: &[String]) -> ExtractionResult {
        let mut result = ExtractionResult::new();

        for file in source_files {
            if self.should_include_file(file) {
                self.extract_file(file, &mut result);
            }
        }

        result
    }

    /// Check if a file should be included.
    fn should_include_file(&self, file: &str) -> bool {
        // Check exclude patterns first
        for pattern in &self.options.exclude_patterns {
            if file.contains(pattern) {
                return false;
            }
        }

        // If no include patterns, include all
        if self.options.include_patterns.is_empty() {
            return true;
        }

        // Check include patterns
        for pattern in &self.options.include_patterns {
            if file.contains(pattern) {
                return true;
            }
        }

        false
    }

    /// Extract documentation from a single file.
    fn extract_file(&mut self, _file: &str, _result: &mut ExtractionResult) {
        // In a real implementation, this would:
        // 1. Parse the TypeScript file
        // 2. Walk the AST
        // 3. Extract classes, functions, etc. using sub-extractors
        // 4. Add entries to result
    }

    /// Check if an entry is internal.
    pub fn is_internal(entry: &DocEntry) -> bool {
        entry.jsdoc_tags.iter().any(|tag| tag.name == "internal")
    }

    /// Check if an entry is deprecated.
    pub fn is_deprecated(entry: &DocEntry) -> bool {
        entry.deprecated.is_some() || entry.jsdoc_tags.iter().any(|tag| tag.name == "deprecated")
    }
}

impl Default for DocsExtractor {
    fn default() -> Self {
        Self::new(ExtractorOptions::default())
    }
}
