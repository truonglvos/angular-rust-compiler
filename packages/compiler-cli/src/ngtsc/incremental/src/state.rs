// Incremental State
//
// Tracks the state of compilation for incremental builds.

use std::collections::HashMap;
use super::api::IncrementalState;

/// State for a single file in an incremental build.
#[derive(Debug, Clone)]
pub struct FileState {
    /// File path.
    pub path: String,
    /// Hash of file contents.
    pub hash: String,
    /// Whether the file was successfully analyzed.
    pub analyzed: bool,
    /// Whether the file was successfully emitted.
    pub emitted: bool,
    /// Analysis timestamp.
    pub analysis_time: Option<u64>,
}

impl FileState {
    pub fn new(path: impl Into<String>, hash: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            hash: hash.into(),
            analyzed: false,
            emitted: false,
            analysis_time: None,
        }
    }
}

/// Manages incremental build state.
pub struct IncrementalStateManager {
    /// Current state.
    current: IncrementalState,
    /// File states by path.
    file_states: HashMap<String, FileState>,
    /// Prior state from last successful build.
    prior_state: Option<IncrementalState>,
}

impl IncrementalStateManager {
    pub fn new() -> Self {
        Self {
            current: IncrementalState::new(),
            file_states: HashMap::new(),
            prior_state: None,
        }
    }
    
    /// Initialize with prior state.
    pub fn with_prior_state(prior: IncrementalState) -> Self {
        Self {
            current: IncrementalState::new(),
            file_states: HashMap::new(),
            prior_state: Some(prior),
        }
    }
    
    /// Get prior state.
    pub fn prior(&self) -> Option<&IncrementalState> {
        self.prior_state.as_ref()
    }
    
    /// Get current state.
    pub fn current(&self) -> &IncrementalState {
        &self.current
    }
    
    /// Record that a file was analyzed.
    pub fn record_analyzed(&mut self, file: &str) {
        self.current.analyzed_files.insert(file.to_string());
        if let Some(state) = self.file_states.get_mut(file) {
            state.analyzed = true;
        }
    }
    
    /// Record that a file was emitted.
    pub fn record_emitted(&mut self, file: &str) {
        self.current.emitted_files.insert(file.to_string());
        if let Some(state) = self.file_states.get_mut(file) {
            state.emitted = true;
        }
    }
    
    /// Check if a file needs re-analysis.
    pub fn needs_analysis(&self, file: &str, current_hash: &str) -> bool {
        if let Some(prior) = &self.prior_state {
            if !prior.analyzed_files.contains(file) {
                return true;
            }
            // Check if file changed
            if let Some(state) = self.file_states.get(file) {
                return state.hash != current_hash;
            }
        }
        true
    }
    
    /// Finalize the current state.
    pub fn finalize(self) -> IncrementalState {
        self.current
    }
}

impl Default for IncrementalStateManager {
    fn default() -> Self {
        Self::new()
    }
}
