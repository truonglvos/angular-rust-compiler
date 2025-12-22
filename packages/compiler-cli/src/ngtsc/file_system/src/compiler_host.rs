use crate::ngtsc::file_system::src::helpers::absolute_from;
use crate::ngtsc::file_system::src::types::{AbsoluteFsPath, FileSystem};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct CompilerOptions {
    pub new_line: Option<String>, // Approximating ts.NewLineKind as string or option for now
    // Add other fields as needed
    pub base_url: Option<String>,
    pub paths: Option<std::collections::HashMap<String, Vec<String>>>,
    pub root_dirs: Option<Vec<String>>,
}

pub struct NgtscCompilerHost {
    fs: Arc<dyn FileSystem + Sync + Send>,
    options: CompilerOptions,
}

impl NgtscCompilerHost {
    pub fn new(fs: Arc<dyn FileSystem + Sync + Send>, options: CompilerOptions) -> Self {
        Self { fs, options }
    }

    pub fn get_source_file(&self, file_name: &str) -> Option<String> {
         self.read_file(file_name)
    }

    pub fn get_default_lib_file_name(&self) -> String {
        self.fs.join(self.get_default_lib_location().as_str(), &["lib.d.ts"])
    }

    pub fn get_default_lib_location(&self) -> String {
        self.fs.get_default_lib_location().into_string()
    }

    pub fn write_file(
        &self,
        file_name: &str,
        data: &str,
        _write_byte_order_mark: bool,
    ) {
        let path = absolute_from(file_name);
        let _ = self.fs.ensure_dir(&AbsoluteFsPath::new(self.fs.dirname(path.as_str())));
        let _ = self.fs.write_file(&path, data.as_bytes(), None);
    }

    pub fn get_current_directory(&self) -> String {
        self.fs.pwd().into_string()
    }

    pub fn get_canonical_file_name(&self, file_name: &str) -> String {
        if self.use_case_sensitive_file_names() {
            file_name.to_string()
        } else {
            file_name.to_lowercase()
        }
    }

    pub fn use_case_sensitive_file_names(&self) -> bool {
        self.fs.is_case_sensitive()
    }

    pub fn get_new_line(&self) -> String {
        self.options.new_line.clone().unwrap_or_else(|| "\n".to_string())
    }

    pub fn file_exists(&self, file_name: &str) -> bool {
        let abs_path = self.fs.resolve(&[file_name]);
        self.fs.exists(&abs_path) && self.fs.stat(&abs_path).map(|s| s.is_file).unwrap_or(false)
    }

    pub fn read_file(&self, file_name: &str) -> Option<String> {
        let abs_path = self.fs.resolve(&[file_name]);
        if !self.file_exists(abs_path.as_str()) {
            return None;
        }
        self.fs.read_file(&abs_path).ok()
    }

    pub fn realpath(&self, path: &str) -> String {
        let abs = self.fs.resolve(&[path]);
         self.fs.realpath(&abs).map(|p| p.into_string()).unwrap_or_else(|_| path.to_string())
    }
}
