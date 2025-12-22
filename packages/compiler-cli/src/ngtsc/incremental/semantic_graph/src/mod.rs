// Semantic Graph Source Module

pub mod api;
pub mod graph;
pub mod util;

// Re-exports
pub use api::{SemanticSymbol, SemanticReference, SemanticDependencyGraph, SymbolData};
pub use graph::SemanticGraph;
pub use util::{references_equal, has_dependency_changes};
