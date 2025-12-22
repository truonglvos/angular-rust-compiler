// Shim Utilities
//
// Utility functions for shims.

/// Check if a file is a shim.
pub fn is_shim(file_path: &str) -> bool {
    file_path.contains(".ngfactory") || file_path.contains(".ngsummary")
}

/// Get the original file for a shim.
pub fn get_original_file(shim_path: &str) -> Option<String> {
    if shim_path.contains(".ngfactory") {
        Some(shim_path.replace(".ngfactory", ""))
    } else if shim_path.contains(".ngsummary") {
        Some(shim_path.replace(".ngsummary", ""))
    } else {
        None
    }
}

/// Get shim file name from original.
pub fn get_shim_file_name(original: &str, suffix: &str) -> String {
    original.replace(".ts", &format!("{}.ts", suffix))
}
