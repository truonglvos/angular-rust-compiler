// Incremental Source Module

pub mod api;
pub mod state;
pub mod strategy;
pub mod dependency_tracking;

// Re-exports
pub use api::{
    DependencyTracker, IncrementalBuild, IncrementalStrategy,
    IncrementalState, SemanticDepGraph, IncrementalResult,
};
pub use state::{FileState, IncrementalStateManager};
pub use strategy::{NoopIncrementalStrategy, TrackedIncrementalStrategy};
pub use dependency_tracking::FileDependencyGraph;
