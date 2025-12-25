// Function Extractor
//
// Extracts documentation from function declarations.

use super::entities::*;

/// Extracts function documentation.
pub struct FunctionExtractor;

impl FunctionExtractor {
    /// Extract function entry.
    pub fn extract(name: &str, params: Vec<ParameterEntry>, return_type: &str) -> FunctionEntry {
        FunctionEntry {
            base: DocEntry::new(name, EntryType::Function),
            params,
            return_type: return_type.to_string(),
            type_params: Vec::new(),
        }
    }

    /// Extract parameter entry.
    pub fn extract_param(
        name: &str,
        type_annotation: &str,
        optional: bool,
        default_value: Option<&str>,
    ) -> ParameterEntry {
        ParameterEntry {
            name: name.to_string(),
            type_annotation: type_annotation.to_string(),
            optional,
            default_value: default_value.map(|s| s.to_string()),
            description: String::new(),
        }
    }
}
