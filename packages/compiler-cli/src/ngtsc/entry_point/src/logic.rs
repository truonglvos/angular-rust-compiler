// Logic
//
// Entry point analysis logic.

use std::collections::HashSet;

/// Entry point for compilation.
#[derive(Debug, Clone)]
pub struct NgCompilerEntryPoint {
    /// Root files to compile.
    pub root_files: Vec<String>,
    /// Excluded files.
    pub excluded: HashSet<String>,
    /// Base directory.
    pub base_dir: String,
}

impl NgCompilerEntryPoint {
    pub fn new(base_dir: impl Into<String>) -> Self {
        Self {
            root_files: Vec::new(),
            excluded: HashSet::new(),
            base_dir: base_dir.into(),
        }
    }

    /// Add a root file.
    pub fn add_root_file(&mut self, file: impl Into<String>) {
        self.root_files.push(file.into());
    }

    /// Exclude a file.
    pub fn exclude(&mut self, file: impl Into<String>) {
        self.excluded.insert(file.into());
    }

    /// Check if a file is excluded.
    pub fn is_excluded(&self, file: &str) -> bool {
        self.excluded.contains(file)
    }

    /// Get all root files.
    pub fn get_root_files(&self) -> &[String] {
        &self.root_files
    }
}

/// Entry point analysis result.
#[derive(Debug, Clone)]
pub struct EntryPointAnalysis {
    /// Is this a valid entry point.
    pub is_valid: bool,
    /// Exports from this entry point.
    pub exports: Vec<String>,
    /// Dependencies.
    pub dependencies: Vec<String>,
}

/// Analyze an entry point.
pub fn analyze_entry_point(_path: &str) -> EntryPointAnalysis {
    EntryPointAnalysis {
        is_valid: true,
        exports: Vec::new(),
        dependencies: Vec::new(),
    }
}
