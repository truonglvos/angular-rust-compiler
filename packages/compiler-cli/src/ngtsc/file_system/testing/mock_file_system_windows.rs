use std::path::{Path, PathBuf};
use crate::ngtsc::file_system::testing::mock_file_system::PathStrategy;
use crate::ngtsc::file_system::src::util::clean_path;
use crate::ngtsc::file_system::src::types::AbsoluteFsPath;

pub struct WindowsUtils;

impl WindowsUtils {
    /// Check if a path is Windows-style absolute (e.g., C:/, D:\)
    fn is_windows_absolute(path: &str) -> bool {
        path.len() >= 2 && path.chars().nth(1) == Some(':')
    }
}

impl PathStrategy for WindowsUtils {
    fn split_path(&self, path: &str) -> Vec<String> {
        path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect() // Root logic (drive letter) needed
    }
    fn normalize(&self, path: &str) -> String {
        // Windows normalization: backslashes to slashes, ensure drive letter if absolute?
        // MockFS implementation usually assumes paths are already mostly reasonable or normalizes inputs.
        // Copying from previous mock_file_system.rs logic:
        let p = path.replace('\\', "/");
        if !p.contains(':') && p.starts_with('/') {
            // Emulate drive letter if missing for absolute path?
            // "C:" + p
             format!("C:{}", p)
        } else {
            p
        }
    }
    fn dirname(&self, path: &str) -> String {
        // Simple dirname
        let idx = path.rfind('/');
        if let Some(i) = idx {
            if i == 2 && path.chars().nth(1) == Some(':') {
                return path[0..3].to_string(); // C:/
            }
             if i == 0 { return "/".to_string() } // Should not happen on windows usually if normalized
             path[0..i].to_string()
        } else {
             // C: -> C: ?
             path.to_string()
        }
    }
    fn join(&self, base_path: &str, paths: &[&str]) -> String {
        let mut full_path = base_path.to_string();
        for p in paths {
            // If this path segment is already Windows-absolute, use it as the new base
            if Self::is_windows_absolute(p) {
                full_path = p.to_string();
                continue;
            }
            if !full_path.ends_with('/') {
                full_path.push('/');
            }
            full_path.push_str(p);
        }
        clean_path(&full_path)
    }
    fn relative(&self, from: &str, to: &str) -> String {
        // Similar to posix relative but case insensitive?
         let from_lower = from.to_lowercase();
         let to_lower = to.to_lowercase();
         let from_path = Path::new(&from_lower);
         let to_path = Path::new(&to_lower);
         
         let from_comps: Vec<_> = from_path.components().collect();
         let to_comps: Vec<_> = to_path.components().collect();
         
         let mut i = 0;
         while i < from_comps.len() && i < to_comps.len() && from_comps[i] == to_comps[i] { i += 1; }
         
         let mut res = PathBuf::new();
         for _ in 0..(from_comps.len() - i) { res.push(".."); }
         // Use original casing for the suffix?
         // We need to grab original components from `to`.
         // But we advanced `i` using keys.
         // Let's re-split original `to`
         let to_orig_comps: Vec<_> = Path::new(to).components().collect();
         for j in i..to_orig_comps.len() { res.push(to_orig_comps[j]); }
         
         let s = res.to_str().unwrap_or("");
         if s.is_empty() { ".".to_string() } else { s.replace('\\', "/") }
    }
    fn basename(&self, path: &str, ext: Option<&str>) -> String {
        let name = Path::new(path).file_name().and_then(|n| n.to_str()).unwrap_or("");
        if let Some(ext) = ext {
            if name.to_lowercase().ends_with(&ext.to_lowercase()) {
                 return name[..name.len()-ext.len()].to_string();
            }
        }
        name.to_string()
    }
    fn is_case_sensitive(&self) -> bool { false }
    fn resolve(&self, cwd: &str, paths: &[&str]) -> AbsoluteFsPath {
        // If the first path is already Windows-absolute, use it directly
        if paths.len() > 0 && Self::is_windows_absolute(paths[0]) {
            let joined = self.join(paths[0], &paths[1..]);
            return AbsoluteFsPath::new(joined);
        }
        let joined = self.join(cwd, paths);
        AbsoluteFsPath::new(joined)
    }
    fn is_root(&self, path: &str) -> bool {
        // C:/ or C:\
        path.len() == 3 && path.chars().nth(1) == Some(':') && path.ends_with('/')
    }
}
