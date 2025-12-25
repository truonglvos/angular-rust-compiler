// Incremental Source Module

pub mod api;
pub mod dependency_tracking;
pub mod state;
pub mod strategy;

// Re-exports
pub use api::{
    DependencyTracker, IncrementalBuild, IncrementalResult, IncrementalState, IncrementalStrategy,
    SemanticDepGraph,
};
pub use dependency_tracking::FileDependencyGraph;
pub use state::{FileState, IncrementalStateManager};
pub use strategy::{NoopIncrementalStrategy, TrackedIncrementalStrategy};
