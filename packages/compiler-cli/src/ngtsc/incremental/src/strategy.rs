// Incremental Strategy
//
// Strategies for incremental compilation.

use std::collections::HashSet;
use super::api::{IncrementalStrategy, IncrementalState};

/// Full rebuild strategy - no incremental support.
pub struct NoopIncrementalStrategy;

impl IncrementalStrategy for NoopIncrementalStrategy {
    fn needs_type_check(&self, _file: &str) -> bool {
        true
    }
    
    fn needs_emit(&self, _file: &str) -> bool {
        true
    }
    
    fn record_successful_analysis(&mut self, _file: &str) {
        // No-op
    }
}

/// Tracked incremental strategy - uses prior state.
pub struct TrackedIncrementalStrategy {
    /// Prior state.
    prior_state: Option<IncrementalState>,
    /// Files that need re-analysis.
    stale_files: HashSet<String>,
    /// Files that have been analyzed this build.
    analyzed_files: HashSet<String>,
}

impl TrackedIncrementalStrategy {
    pub fn new() -> Self {
        Self {
            prior_state: None,
            stale_files: HashSet::new(),
            analyzed_files: HashSet::new(),
        }
    }
    
    /// Initialize with prior state.
    pub fn with_prior_state(prior: IncrementalState) -> Self {
        Self {
            prior_state: Some(prior),
            stale_files: HashSet::new(),
            analyzed_files: HashSet::new(),
        }
    }
    
    /// Mark a file as stale.
    pub fn mark_stale(&mut self, file: impl Into<String>) {
        self.stale_files.insert(file.into());
    }
    
    /// Mark multiple files as stale.
    pub fn mark_stale_batch(&mut self, files: impl IntoIterator<Item = String>) {
        self.stale_files.extend(files);
    }
    
    /// Check if any files are stale.
    pub fn has_stale_files(&self) -> bool {
        !self.stale_files.is_empty()
    }
    
    /// Get all stale files.
    pub fn stale_files(&self) -> &HashSet<String> {
        &self.stale_files
    }
}

impl IncrementalStrategy for TrackedIncrementalStrategy {
    fn needs_type_check(&self, file: &str) -> bool {
        // Check if file is in stale set
        if self.stale_files.contains(file) {
            return true;
        }
        
        // Check if prior state has this file
        if let Some(prior) = &self.prior_state {
            if prior.was_analyzed(file) {
                return false;
            }
        }
        
        true
    }
    
    fn needs_emit(&self, file: &str) -> bool {
        // Check if file was already emitted
        if let Some(prior) = &self.prior_state {
            if prior.was_emitted(file) && !self.stale_files.contains(file) {
                return false;
            }
        }
        
        true
    }
    
    fn record_successful_analysis(&mut self, file: &str) {
        self.analyzed_files.insert(file.to_string());
        self.stale_files.remove(file);
    }
}

impl Default for TrackedIncrementalStrategy {
    fn default() -> Self {
        Self::new()
    }
}
