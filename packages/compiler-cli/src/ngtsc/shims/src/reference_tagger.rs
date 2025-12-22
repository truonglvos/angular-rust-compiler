// Reference Tagger
//
// Tags references in shim files.

use std::collections::HashSet;

/// Reference tagger for shims.
pub struct ReferenceTagger {
    tagged: HashSet<String>,
}

impl ReferenceTagger {
    pub fn new() -> Self {
        Self {
            tagged: HashSet::new(),
        }
    }
    
    /// Tag a reference.
    pub fn tag(&mut self, reference: &str) {
        self.tagged.insert(reference.to_string());
    }
    
    /// Check if reference is tagged.
    pub fn is_tagged(&self, reference: &str) -> bool {
        self.tagged.contains(reference)
    }
    
    /// Get all tagged references.
    pub fn all_tagged(&self) -> impl Iterator<Item = &str> {
        self.tagged.iter().map(|s| s.as_str())
    }
    
    /// Clear all tags.
    pub fn clear(&mut self) {
        self.tagged.clear();
    }
}

impl Default for ReferenceTagger {
    fn default() -> Self {
        Self::new()
    }
}
