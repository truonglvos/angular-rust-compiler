// Incremental API
//
// Public API types for incremental compilation.

use std::collections::HashSet;

/// Tracks dependencies between files for incremental compilation.
pub trait DependencyTracker {
    /// Record that a file depends on another file.
    fn add_dependency(&mut self, from: &str, to: &str);

    /// Get all files that depend on the given file.
    fn get_dependents(&self, file: &str) -> HashSet<String>;

    /// Get all files that the given file depends on.
    fn get_dependencies(&self, file: &str) -> HashSet<String>;
}

/// A build state represents the state of a compilation.
pub trait IncrementalBuild {
    /// Get the prior build state, if available.
    fn prior_state(&self) -> Option<&IncrementalState>;

    /// Record the current state for future incremental builds.
    fn record_successful_analysis(&mut self, state: IncrementalState);
}

/// Strategy for determining which files need recompilation.
pub trait IncrementalStrategy {
    /// Determine if a file needs type-check.
    fn needs_type_check(&self, file: &str) -> bool;

    /// Determine if a file needs emit.
    fn needs_emit(&self, file: &str) -> bool;

    /// Mark a file as successfully analyzed.
    fn record_successful_analysis(&mut self, file: &str);
}

/// State of an incremental compilation.
#[derive(Debug, Clone, Default)]
pub struct IncrementalState {
    /// Set of analyzed files.
    pub analyzed_files: HashSet<String>,
    /// Set of emitted files.
    pub emitted_files: HashSet<String>,
    /// Semantic dependency graph.
    pub semantic_dep_graph: Option<SemanticDepGraph>,
}

impl IncrementalState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a file was analyzed.
    pub fn was_analyzed(&self, file: &str) -> bool {
        self.analyzed_files.contains(file)
    }

    /// Check if a file was emitted.
    pub fn was_emitted(&self, file: &str) -> bool {
        self.emitted_files.contains(file)
    }
}

/// Placeholder for semantic dependency graph.
#[derive(Debug, Clone, Default)]
pub struct SemanticDepGraph {
    /// Files in the graph.
    pub files: HashSet<String>,
}

/// Result of incremental analysis.
#[derive(Debug, Clone)]
pub enum IncrementalResult {
    /// Fresh build, no prior state.
    Fresh,
    /// Incremental build with prior state.
    Incremental {
        /// Files that need re-analysis.
        stale_files: HashSet<String>,
        /// Files that can be reused.
        reusable_files: HashSet<String>,
    },
}
