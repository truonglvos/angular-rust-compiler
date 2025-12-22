// Generics Extractor
//
// Extracts type parameter information.

use super::entities::*;

/// Extracts type parameter documentation.
pub struct GenericsExtractor;

impl GenericsExtractor {
    /// Extract type parameter entry.
    pub fn extract(
        name: &str,
        constraint: Option<&str>,
        default: Option<&str>,
    ) -> TypeParameterEntry {
        TypeParameterEntry {
            name: name.to_string(),
            constraint: constraint.map(|s| s.to_string()),
            default: default.map(|s| s.to_string()),
        }
    }
    
    /// Extract multiple type parameters.
    pub fn extract_all(params: &[(String, Option<String>, Option<String>)]) -> Vec<TypeParameterEntry> {
        params.iter().map(|(name, constraint, default)| {
            TypeParameterEntry {
                name: name.clone(),
                constraint: constraint.clone(),
                default: default.clone(),
            }
        }).collect()
    }
}
