// Reference Graph
//
// Tracks references between source files.

use std::collections::{HashMap, HashSet};

/// Reference graph for entry point analysis.
#[derive(Debug, Clone, Default)]
pub struct ReferenceGraph {
    /// Outgoing references from each file.
    references: HashMap<String, HashSet<String>>,
}

impl ReferenceGraph {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a reference from one file to another.
    pub fn add_reference(&mut self, from: &str, to: &str) {
        self.references
            .entry(from.to_string())
            .or_default()
            .insert(to.to_string());
    }
    
    /// Get all references from a file.
    pub fn get_references(&self, file: &str) -> Option<&HashSet<String>> {
        self.references.get(file)
    }
    
    /// Get transitive references from a file.
    pub fn get_transitive_references(&self, file: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut to_visit = vec![file.to_string()];
        
        while let Some(current) = to_visit.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            
            if let Some(refs) = self.references.get(&current) {
                for r in refs {
                    if !visited.contains(r) {
                        to_visit.push(r.clone());
                    }
                }
            }
        }
        
        visited.remove(file);
        visited
    }
    
    /// Check if there's a path from source to target.
    pub fn has_path(&self, from: &str, to: &str) -> bool {
        self.get_transitive_references(from).contains(to)
    }
    
    /// Get all files.
    pub fn files(&self) -> Vec<&str> {
        self.references.keys().map(|s| s.as_str()).collect()
    }
}
