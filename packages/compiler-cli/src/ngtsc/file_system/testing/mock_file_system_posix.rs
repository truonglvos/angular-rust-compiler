use crate::ngtsc::file_system::src::types::AbsoluteFsPath;
use crate::ngtsc::file_system::src::util::clean_path;
use crate::ngtsc::file_system::testing::mock_file_system::PathStrategy;
use std::path::{Path, PathBuf};

pub struct PosixUtils {
    pub is_case_sensitive: bool,
}

impl PathStrategy for PosixUtils {
    fn split_path(&self, path: &str) -> Vec<String> {
        path.split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect() // Root logic might need adjustment
    }
    fn normalize(&self, path: &str) -> String {
        path.replace('\\', "/")
    }
    fn dirname(&self, path: &str) -> String {
        let p = Path::new(path);
        match p.parent() {
            Some(parent) => {
                let s = parent.to_str().unwrap().replace('\\', "/");
                if s.is_empty() {
                    "/".to_string()
                } else {
                    s
                }
            }
            None => "/".to_string(),
        }
    }
    fn join(&self, base_path: &str, paths: &[&str]) -> String {
        let mut full_path = base_path.to_string();
        for p in paths {
            if !full_path.ends_with('/') && !p.starts_with('/') {
                full_path.push('/');
            }
            full_path.push_str(p);
        }
        clean_path(&full_path)
    }
    fn relative(&self, from: &str, to: &str) -> String {
        let from_path = Path::new(from);
        let to_path = Path::new(to);

        // Rudimentary relative impl
        let from_comps: Vec<_> = from_path.components().collect();
        let to_comps: Vec<_> = to_path.components().collect();
        let mut i = 0;
        while i < from_comps.len() && i < to_comps.len() && from_comps[i] == to_comps[i] {
            i += 1;
        }
        let mut res = PathBuf::new();
        for _ in 0..(from_comps.len() - i) {
            res.push("..");
        }
        for j in i..to_comps.len() {
            res.push(to_comps[j]);
        }
        let s = res.to_str().unwrap_or("");
        if s.is_empty() {
            ".".to_string()
        } else {
            s.replace('\\', "/")
        }
    }
    fn basename(&self, path: &str, ext: Option<&str>) -> String {
        let name = Path::new(path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if let Some(ext) = ext {
            if name.ends_with(ext) {
                return name[..name.len() - ext.len()].to_string();
            }
        }
        name.to_string()
    }
    fn is_case_sensitive(&self) -> bool {
        self.is_case_sensitive
    }
    fn resolve(&self, cwd: &str, paths: &[&str]) -> AbsoluteFsPath {
        let joined = self.join(cwd, paths);
        AbsoluteFsPath::new(joined)
    }
    fn is_root(&self, path: &str) -> bool {
        path == "/"
    }
}
