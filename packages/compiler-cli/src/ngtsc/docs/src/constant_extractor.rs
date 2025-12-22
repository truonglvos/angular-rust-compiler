// Constant Extractor
//
// Extracts documentation from constant declarations.

use super::entities::*;

/// Extracts constant documentation.
pub struct ConstantExtractor;

impl ConstantExtractor {
    /// Extract constant entry.
    pub fn extract(
        name: &str,
        type_annotation: &str,
        value: Option<&str>,
    ) -> DocEntry {
        let mut entry = DocEntry::new(name, EntryType::Constant);
        entry.metadata.insert("type".to_string(), type_annotation.to_string());
        if let Some(v) = value {
            entry.metadata.insert("value".to_string(), v.to_string());
        }
        entry
    }
}
