// Enum Extractor
//
// Extracts documentation from enum declarations.

use super::entities::*;

/// Extracts enum documentation.
pub struct EnumExtractor;

impl EnumExtractor {
    /// Extract enum entry.
    pub fn extract(name: &str, members: Vec<EnumMemberEntry>) -> EnumEntry {
        EnumEntry {
            base: DocEntry::new(name, EntryType::Enum),
            members,
        }
    }

    /// Extract enum member.
    pub fn extract_member(name: &str, value: &str) -> EnumMemberEntry {
        EnumMemberEntry {
            name: name.to_string(),
            value: value.to_string(),
            description: String::new(),
        }
    }
}
