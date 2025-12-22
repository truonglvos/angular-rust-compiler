// Semantic Dependency Graph API
//
// Types for tracking semantic dependencies between symbols.

use std::collections::HashSet;

/// A symbol in the semantic graph.
pub trait SemanticSymbol: std::fmt::Debug {
    /// Get the unique identifier for this symbol.
    fn identifier(&self) -> &str;
    
    /// Get the file path containing this symbol.
    fn file_path(&self) -> &str;
    
    /// Check if the public API of this symbol has changed.
    fn is_public_api_affected(&self, previous: &dyn SemanticSymbol) -> bool;
    
    /// Check if the type-check API of this symbol has changed.
    fn is_type_check_api_affected(&self, previous: &dyn SemanticSymbol) -> bool;
}

/// A reference to a semantic symbol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SemanticReference {
    /// Symbol identifier.
    pub symbol_id: String,
    /// The public name used to reference this symbol.
    pub public_name: String,
}

impl SemanticReference {
    pub fn new(symbol_id: impl Into<String>, public_name: impl Into<String>) -> Self {
        Self {
            symbol_id: symbol_id.into(),
            public_name: public_name.into(),
        }
    }
}

/// Semantic dependency graph for tracking symbol relationships.
#[derive(Debug, Clone, Default)]
pub struct SemanticDependencyGraph {
    /// Symbols by identifier.
    symbols: std::collections::HashMap<String, SymbolData>,
    /// Files by path.
    files: HashSet<String>,
}

/// Data for a symbol in the graph.
#[derive(Debug, Clone)]
pub struct SymbolData {
    /// Symbol identifier.
    pub id: String,
    /// File containing this symbol.
    pub file: String,
    /// Symbols this symbol depends on (public API).
    pub public_api_deps: HashSet<String>,
    /// Symbols this symbol depends on (type-check API).
    pub type_check_api_deps: HashSet<String>,
}

impl SemanticDependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a symbol to the graph.
    pub fn add_symbol(&mut self, id: impl Into<String>, file: impl Into<String>) {
        let id = id.into();
        let file = file.into();
        self.files.insert(file.clone());
        self.symbols.insert(id.clone(), SymbolData {
            id,
            file,
            public_api_deps: HashSet::new(),
            type_check_api_deps: HashSet::new(),
        });
    }
    
    /// Add a public API dependency.
    pub fn add_public_api_dep(&mut self, from: &str, to: &str) {
        if let Some(symbol) = self.symbols.get_mut(from) {
            symbol.public_api_deps.insert(to.to_string());
        }
    }
    
    /// Add a type-check API dependency.
    pub fn add_type_check_api_dep(&mut self, from: &str, to: &str) {
        if let Some(symbol) = self.symbols.get_mut(from) {
            symbol.type_check_api_deps.insert(to.to_string());
        }
    }
    
    /// Get symbols affected by changes to a set of symbols.
    pub fn get_affected_symbols(&self, changed: &HashSet<String>) -> HashSet<String> {
        let mut affected = HashSet::new();
        
        for (id, data) in &self.symbols {
            for dep in &data.public_api_deps {
                if changed.contains(dep) {
                    affected.insert(id.clone());
                    break;
                }
            }
        }
        
        affected
    }
    
    /// Get all files in the graph.
    pub fn files(&self) -> &HashSet<String> {
        &self.files
    }
}
