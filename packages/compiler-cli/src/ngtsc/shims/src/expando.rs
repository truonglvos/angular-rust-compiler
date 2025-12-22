// Expando
//
// Expando utilities for shims.

use std::collections::HashMap;

/// Expando data for shim files.
#[derive(Debug, Clone, Default)]
pub struct ShimExpando {
    /// Additional properties.
    properties: HashMap<String, String>,
}

impl ShimExpando {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }
    
    pub fn get(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
    
    pub fn has(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
}
