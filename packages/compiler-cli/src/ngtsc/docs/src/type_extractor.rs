// Type Extractor
//
// Extracts type information from TypeScript nodes.

/// Extracts type annotations.
pub struct TypeExtractor;

impl TypeExtractor {
    /// Convert type to string representation.
    pub fn type_to_string(type_str: &str) -> String {
        type_str.to_string()
    }
    
    /// Check if type is primitive.
    pub fn is_primitive(type_str: &str) -> bool {
        matches!(type_str, "string" | "number" | "boolean" | "null" | "undefined" | "void" | "any" | "unknown" | "never")
    }
    
    /// Check if type is array.
    pub fn is_array(type_str: &str) -> bool {
        type_str.ends_with("[]") || type_str.starts_with("Array<")
    }
    
    /// Get element type of array.
    pub fn get_array_element_type(type_str: &str) -> Option<String> {
        if type_str.ends_with("[]") {
            Some(type_str[..type_str.len() - 2].to_string())
        } else if type_str.starts_with("Array<") && type_str.ends_with('>') {
            Some(type_str[6..type_str.len() - 1].to_string())
        } else {
            None
        }
    }
}
