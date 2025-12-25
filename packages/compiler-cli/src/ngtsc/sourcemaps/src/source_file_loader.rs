// Source File Loader
//
// Loads source files and their source maps.

use super::content_origin::ContentOrigin;
use super::raw_source_map::SourceMap;

/// Source file loader.
pub struct SourceFileLoader {
    root_dir: String,
}

impl SourceFileLoader {
    pub fn new(root_dir: impl Into<String>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    /// Load a source file.
    pub fn load_source_file(&self, path: &str) -> Result<LoadedFile, String> {
        let full_path = format!("{}/{}", self.root_dir, path);

        let content = std::fs::read_to_string(&full_path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;

        let source_map = self.find_source_map(&content, path);

        Ok(LoadedFile {
            path: path.to_string(),
            content,
            source_map,
            origin: ContentOrigin::File(full_path),
        })
    }

    /// Find source map for content.
    fn find_source_map(&self, content: &str, _path: &str) -> Option<SourceMap> {
        // Check for inline source map
        if content.contains("//# sourceMappingURL=data:") {
            // Would parse inline source map
            return None;
        }

        // Check for external source map
        if let Some(url_line) = content
            .lines()
            .find(|l| l.starts_with("//# sourceMappingURL="))
        {
            let _url = url_line.trim_start_matches("//# sourceMappingURL=");
            // Would load external source map
            return None;
        }

        None
    }
}

/// Loaded source file.
#[derive(Debug, Clone)]
pub struct LoadedFile {
    pub path: String,
    pub content: String,
    pub source_map: Option<SourceMap>,
    pub origin: ContentOrigin,
}
