// Semantic Graph Source Module

pub mod api;
pub mod graph;
pub mod util;

// Re-exports
pub use api::{SemanticDependencyGraph, SemanticReference, SemanticSymbol, SymbolData};
pub use graph::SemanticGraph;
pub use util::{has_dependency_changes, references_equal};
