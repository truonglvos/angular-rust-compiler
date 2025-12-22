// Private Export Checker
//
// Checks for private exports in entry points.

use std::collections::HashSet;

/// Private export checker.
pub struct PrivateExportChecker {
    /// Known private symbols.
    private_symbols: HashSet<String>,
}

impl PrivateExportChecker {
    pub fn new() -> Self {
        Self {
            private_symbols: HashSet::new(),
        }
    }
    
    /// Check if a symbol is private.
    pub fn is_private(&self, symbol: &str) -> bool {
        symbol.starts_with('_') || self.private_symbols.contains(symbol)
    }
    
    /// Add a private symbol.
    pub fn add_private(&mut self, symbol: impl Into<String>) {
        self.private_symbols.insert(symbol.into());
    }
    
    /// Check exports for private symbols.
    pub fn check_exports(&self, exports: &[String]) -> Vec<PrivateExportViolation> {
        exports
            .iter()
            .filter(|e| self.is_private(e))
            .map(|e| PrivateExportViolation {
                symbol: e.clone(),
                message: format!("Symbol '{}' is private and cannot be exported", e),
            })
            .collect()
    }
}

impl Default for PrivateExportChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// A private export violation.
#[derive(Debug, Clone)]
pub struct PrivateExportViolation {
    /// Symbol name.
    pub symbol: String,
    /// Error message.
    pub message: String,
}
