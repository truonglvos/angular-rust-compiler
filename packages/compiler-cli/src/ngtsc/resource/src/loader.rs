// Resource Loader
//
// Loads external resources (templates, styles).

use std::collections::HashMap;

/// Resource load error.
#[derive(Debug, Clone)]
pub struct ResourceError {
    pub url: String,
    pub message: String,
}

impl ResourceError {
    pub fn not_found(url: &str) -> Self {
        Self {
            url: url.to_string(),
            message: format!("Resource not found: {}", url),
        }
    }

    pub fn load_failed(url: &str, reason: &str) -> Self {
        Self {
            url: url.to_string(),
            message: format!("Failed to load {}: {}", url, reason),
        }
    }
}

/// Resource loader trait.
pub trait ResourceLoader {
    fn can_preload(&self, url: &str) -> bool;
    fn preload(&self, url: &str) -> Result<(), ResourceError>;
    fn load(&self, url: &str) -> Result<String, ResourceError>;
}

/// In-memory resource loader.
#[derive(Default)]
pub struct InMemoryResourceLoader {
    resources: HashMap<String, String>,
}

impl InMemoryResourceLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, url: &str, content: &str) {
        self.resources.insert(url.to_string(), content.to_string());
    }
}

impl ResourceLoader for InMemoryResourceLoader {
    fn can_preload(&self, url: &str) -> bool {
        self.resources.contains_key(url)
    }

    fn preload(&self, url: &str) -> Result<(), ResourceError> {
        if self.resources.contains_key(url) {
            Ok(())
        } else {
            Err(ResourceError::not_found(url))
        }
    }

    fn load(&self, url: &str) -> Result<String, ResourceError> {
        self.resources
            .get(url)
            .cloned()
            .ok_or_else(|| ResourceError::not_found(url))
    }
}

/// File-based resource loader.
pub struct FileResourceLoader {
    root_dir: String,
}

impl FileResourceLoader {
    pub fn new(root_dir: impl Into<String>) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }
}

impl ResourceLoader for FileResourceLoader {
    fn can_preload(&self, url: &str) -> bool {
        let path = format!("{}/{}", self.root_dir, url);
        std::path::Path::new(&path).exists()
    }

    fn preload(&self, _url: &str) -> Result<(), ResourceError> {
        Ok(())
    }

    fn load(&self, url: &str) -> Result<String, ResourceError> {
        let path = format!("{}/{}", self.root_dir, url);
        std::fs::read_to_string(&path).map_err(|e| ResourceError::load_failed(url, &e.to_string()))
    }
}
