// Dependency Tracking
//
// Tracks dependencies between files for incremental compilation.

use std::collections::{HashMap, HashSet};
use super::api::DependencyTracker;

/// File dependency graph.
pub struct FileDependencyGraph {
    /// Forward dependencies: file -> files it depends on.
    forward: HashMap<String, HashSet<String>>,
    /// Reverse dependencies: file -> files that depend on it.
    reverse: HashMap<String, HashSet<String>>,
}

impl FileDependencyGraph {
    pub fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }
    
    /// Clear all dependencies.
    pub fn clear(&mut self) {
        self.forward.clear();
        self.reverse.clear();
    }
    
    /// Get all files in the graph.
    pub fn all_files(&self) -> HashSet<String> {
        let mut files = HashSet::new();
        files.extend(self.forward.keys().cloned());
        files.extend(self.reverse.keys().cloned());
        files
    }
    
    /// Get transitive dependencies (all files this file depends on, recursively).
    pub fn get_transitive_dependencies(&self, file: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = vec![file.to_string()];
        
        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            
            if let Some(deps) = self.forward.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        queue.push(dep.clone());
                    }
                }
            }
        }
        
        visited.remove(file);
        visited
    }
    
    /// Get transitive dependents (all files that depend on this file, recursively).
    pub fn get_transitive_dependents(&self, file: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = vec![file.to_string()];
        
        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());
            
            if let Some(deps) = self.reverse.get(&current) {
                for dep in deps {
                    if !visited.contains(dep) {
                        queue.push(dep.clone());
                    }
                }
            }
        }
        
        visited.remove(file);
        visited
    }
}

impl DependencyTracker for FileDependencyGraph {
    fn add_dependency(&mut self, from: &str, to: &str) {
        self.forward
            .entry(from.to_string())
            .or_insert_with(HashSet::new)
            .insert(to.to_string());
        
        self.reverse
            .entry(to.to_string())
            .or_insert_with(HashSet::new)
            .insert(from.to_string());
    }
    
    fn get_dependents(&self, file: &str) -> HashSet<String> {
        self.reverse
            .get(file)
            .cloned()
            .unwrap_or_default()
    }
    
    fn get_dependencies(&self, file: &str) -> HashSet<String> {
        self.forward
            .get(file)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for FileDependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
