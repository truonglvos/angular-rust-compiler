// Shim Adapter
//
// Generates shim files for type-checking.

/// Shim type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShimType {
    Factory,
    Summary,
    Ngsummary,
}

/// Shim generator.
pub struct ShimGenerator {
    base_content: String,
}

impl ShimGenerator {
    pub fn new() -> Self {
        Self {
            base_content: String::new(),
        }
    }
    
    pub fn base_content(&self) -> &str {
        &self.base_content
    }
    
    pub fn generate(&self, original_file: &str, shim_type: ShimType) -> ShimFile {
        let suffix = match shim_type {
            ShimType::Factory => ".ngfactory",
            ShimType::Summary => ".ngsummary",
            ShimType::Ngsummary => ".ngsummary",
        };
        
        let file_name = original_file.replace(".ts", &format!("{}.ts", suffix));
        
        ShimFile {
            file_name,
            content: self.generate_content(original_file, shim_type),
            shim_type,
        }
    }
    
    fn generate_content(&self, _original: &str, shim_type: ShimType) -> String {
        match shim_type {
            ShimType::Factory => "// Factory shim\nexport {};\n".to_string(),
            ShimType::Summary | ShimType::Ngsummary => "// Summary shim\nexport {};\n".to_string(),
        }
    }
}

impl Default for ShimGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generated shim file.
#[derive(Debug, Clone)]
pub struct ShimFile {
    pub file_name: String,
    pub content: String,
    pub shim_type: ShimType,
}
