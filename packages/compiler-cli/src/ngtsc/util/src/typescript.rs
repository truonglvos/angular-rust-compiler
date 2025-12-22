// TypeScript Utilities
//
// TypeScript-specific utilities.

/// Check if file is TypeScript.
pub fn is_typescript_file(path: &str) -> bool {
    path.ends_with(".ts") || path.ends_with(".tsx")
}

/// Check if file is declaration file.
pub fn is_declaration_file(path: &str) -> bool {
    path.ends_with(".d.ts")
}

/// Check if file is JavaScript.
pub fn is_javascript_file(path: &str) -> bool {
    path.ends_with(".js") || path.ends_with(".jsx") || path.ends_with(".mjs")
}

/// Check if file is source file.
pub fn is_source_file(path: &str) -> bool {
    is_typescript_file(path) && !is_declaration_file(path)
}

/// Get output path for TypeScript.
pub fn get_output_path(source_path: &str, out_dir: &str) -> String {
    let base = source_path.replace(".ts", ".js").replace(".tsx", ".js");
    format!("{}/{}", out_dir, super::path::get_basename(&base))
}

/// Get declaration path.
pub fn get_declaration_path(source_path: &str) -> String {
    source_path.replace(".ts", ".d.ts").replace(".tsx", ".d.ts")
}
