use regex::Regex;
use once_cell::sync::Lazy;

static TS_DTS_JS_EXTENSION: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)\.ts$|\.d\.ts$|\.js$").unwrap());

/// Convert Windows-style separators to POSIX separators.
pub fn normalize_separators(path: &str) -> String {
    // In Rust, str replace is simple enough.
    path.replace('\\', "/")
}

/// Remove a .ts, .d.ts, or .js extension from a file name.
pub fn strip_extension(path: &str) -> String {
    TS_DTS_JS_EXTENSION.replace(path, "").to_string()
}

pub fn clean_path(path: &str) -> String {
    let normalized = normalize_separators(path);
    
    // Check for Windows-style absolute path (e.g., C:/, D:\)
    let is_windows_absolute = normalized.len() >= 2 && normalized.chars().nth(1) == Some(':');
    let is_unix_absolute = normalized.starts_with('/');
    
    // Extract drive prefix for Windows paths
    let drive_prefix = if is_windows_absolute {
        normalized[0..2].to_string() // "C:" or "D:" etc.
    } else {
        String::new()
    };
    
    // Get the path part after drive letter for Windows paths
    let path_part = if is_windows_absolute {
        &normalized[2..]
    } else {
        &normalized
    };
    
    let mut segments = Vec::new();
    
    for segment in path_part.split('/') {
        if segment.is_empty() || segment == "." {
            continue;
        }
        if segment == ".." {
            if !segments.is_empty() {
                segments.pop();
            }
        } else {
            segments.push(segment);
        }
    }
    
    let joined = segments.join("/");
    
    if is_windows_absolute {
        format!("{}/{}", drive_prefix, joined)
    } else if is_unix_absolute {
        format!("/{}", joined)
    } else {
        joined
    }
}

