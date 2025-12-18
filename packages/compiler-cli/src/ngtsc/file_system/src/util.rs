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
    let mut segments = Vec::new();
    let normalized = normalize_separators(path);
    let is_absolute = normalized.starts_with('/');
    
    for segment in normalized.split('/') {
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
    if is_absolute {
        format!("/{}", joined)
    } else {
        joined
    }
}
