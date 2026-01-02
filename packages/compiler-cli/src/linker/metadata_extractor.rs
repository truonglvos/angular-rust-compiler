//! NgModule Metadata Extraction
//!
//! This module extracts metadata from linked JavaScript to enable dynamic
//! NgModule resolution. When an NgModule is linked, we parse the output to
//! extract:
//! - NgModule exports (list of directive/pipe names)
//! - Directive selectors, inputs, outputs, hostAttrs
//! - Pipe names
//!
//! This metadata is used during template compilation to match directives
//! from imported NgModules without hard-coding specific modules.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Extracted metadata for a directive or component
#[derive(Debug, Clone)]
pub struct ExtractedDirective {
    pub name: String,
    pub selector: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub host_attrs: Vec<String>,
    pub is_component: bool,
}

/// Extracted metadata for an NgModule
#[derive(Debug, Clone)]
pub struct ExtractedNgModule {
    pub name: String,
    pub exports: Vec<String>,
}

/// Global metadata cache, keyed by module path
static METADATA_CACHE: OnceLock<RwLock<MetadataCache>> = OnceLock::new();

#[derive(Debug, Default)]
pub struct MetadataCache {
    pub modules: HashMap<String, ExtractedNgModule>,
    pub directives: HashMap<String, ExtractedDirective>,
}

impl MetadataCache {
    fn new() -> Self {
        Self {
            modules: HashMap::new(),
            directives: HashMap::new(),
        }
    }
}

/// Get the global metadata cache
pub fn get_metadata_cache() -> &'static RwLock<MetadataCache> {
    METADATA_CACHE.get_or_init(|| RwLock::new(MetadataCache::new()))
}

/// Extract NgModule and directive metadata from linked JavaScript code.
/// This parses the linked output to find ɵmod and ɵcmp/ɵdir definitions.
///
/// # Arguments
/// * `module_path` - The module path (e.g., "@angular/material/button")
/// * `linked_code` - The linked JavaScript code
///
/// # Returns
/// A tuple of (Vec<ExtractedNgModule>, Vec<ExtractedDirective>)
pub fn extract_metadata_from_linked(
    module_path: &str,
    linked_code: &str,
) -> (Vec<ExtractedNgModule>, Vec<ExtractedDirective>) {
    let mut modules = Vec::new();
    let mut directives = Vec::new();

    // Parse patterns for NgModule exports
    // Pattern: SomeName.ɵmod = ɵɵdefineNgModule({...exports: [Directive1, Directive2]...})
    let ng_module_pattern =
        regex::Regex::new(r"(\w+)\.ɵmod\s*=\s*[^(]+\(\{[^}]*exports:\s*\[([^\]]*)\]").ok();

    // Pattern: SomeName.ɵcmp = ɵɵdefineComponent({...selectors: [[...]]...})
    // or: SomeName.ɵdir = ɵɵdefineDirective({...selectors: [[...]]...})
    // Now we only handle array format: [["button", "mat-button", ""], ["a", "mat-button", ""]]
    // The old string format is no longer used after fixing emit.rs and handler.rs

    // Pattern for hostAttrs: hostAttrs: [1, "mdc-button"]
    let host_attrs_pattern = regex::Regex::new(r#"hostAttrs:\s*\[([^\]]*)\]"#).ok();

    // Parse NgModules
    if let Some(re) = ng_module_pattern {
        for caps in re.captures_iter(linked_code) {
            if let (Some(name), Some(exports_str)) = (caps.get(1), caps.get(2)) {
                let name = name.as_str().to_string();
                let exports: Vec<String> = exports_str
                    .as_str()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if !exports.is_empty() {
                    modules.push(ExtractedNgModule { name, exports });
                }
            }
        }
    }

    // Parse Directives/Components
    // Use a more robust approach: find directive definitions and manually extract selectors
    let directive_name_pattern = regex::Regex::new(r#"(\w+)\.(ɵcmp|ɵdir)\s*="#).ok();

    if let Some(name_re) = directive_name_pattern {
        for caps in name_re.captures_iter(linked_code) {
            if let (Some(name_match), Some(kind_match)) = (caps.get(1), caps.get(2)) {
                let name = name_match.as_str().to_string();
                let is_component = kind_match.as_str() == "ɵcmp";

                // Find the selectors: [[...]] pattern for this directive
                // Search from the directive definition
                let directive_start = name_match.start();
                let search_start = directive_start;
                let search_end = std::cmp::min(search_start + 5000, linked_code.len());
                let search_range = &linked_code[search_start..search_end];

                // Look for selectors: [[ pattern
                // Format: selectors: [["button", "mat-button", ""], ["a", "mat-button", ""]]
                if let Some(selectors_pos) = search_range.find("selectors:") {
                    let after_selectors = &search_range[selectors_pos..];

                    // Find the opening [
                    if let Some(open_pos) = after_selectors.find('[') {
                        let after_open = &after_selectors[open_pos + 1..];

                        // Find the closing ] for the entire selectors array
                        // We need to match all nested arrays: [[...], [...], ...]
                        let mut bracket_count = 1; // Start at 1 because we found the opening [
                        let mut found_closing = false;
                        let mut end_pos = 0;

                        for (i, ch) in after_open.char_indices() {
                            match ch {
                                '[' => bracket_count += 1,
                                ']' => {
                                    bracket_count -= 1;
                                    if bracket_count == 0 {
                                        end_pos = i;
                                        found_closing = true;
                                        break;
                                    }
                                }
                                _ => {}
                            }
                        }

                        if found_closing {
                            let selectors_array_str = &after_open[..end_pos];
                            // Parse the entire selectors array: [["button", "mat-button", ""], ["a", "mat-button", ""]]
                            let selector = parse_selectors_array(selectors_array_str);

                            // eprintln!("DEBUG: [metadata_extractor] Extracted selector for {}: '{}' (from raw: '{}')", name, selector, selectors_array_str);

                            // Extract hostAttrs for this directive
                            let host_attrs = extract_host_attrs_for_class(
                                &name,
                                linked_code,
                                &host_attrs_pattern,
                            );

                            if !selector.is_empty() {
                                directives.push(ExtractedDirective {
                                    name,
                                    selector,
                                    inputs: vec![],
                                    outputs: vec![],
                                    host_attrs,
                                    is_component,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Store in cache
    if !modules.is_empty() || !directives.is_empty() {
        if let Ok(mut cache) = get_metadata_cache().write() {
            for module in &modules {
                cache
                    .modules
                    .insert(format!("{}:{}", module_path, module.name), module.clone());
            }
            for directive in &directives {
                cache.directives.insert(
                    format!("{}:{}", module_path, directive.name),
                    directive.clone(),
                );
            }
        }
    }

    (modules, directives)
}

/// Parse selectors array from JavaScript: [["button", "mat-button", ""], ["a", "mat-button", ""]]
/// Returns a single selector string: "button[mat-button], a[mat-button]"
fn parse_selectors_array(selectors_array_str: &str) -> String {
    let trimmed = selectors_array_str.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let mut selector_strings = Vec::new();
    let mut current_pos = 0;

    // Parse each selector array: ["button", "mat-button", ""]
    while current_pos < trimmed.len() {
        // Find the next [
        if let Some(open_pos) = trimmed[current_pos..].find('[') {
            let array_start = current_pos + open_pos + 1;
            let after_open = &trimmed[array_start..];

            // Find the matching ]
            let mut bracket_count = 1;
            let mut found_closing = false;
            let mut end_pos = 0;

            for (i, ch) in after_open.char_indices() {
                match ch {
                    '[' => bracket_count += 1,
                    ']' => {
                        bracket_count -= 1;
                        if bracket_count == 0 {
                            end_pos = i;
                            found_closing = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if found_closing {
                let selector_array_str = &after_open[..end_pos];
                let selector = parse_single_selector_array(selector_array_str);
                if !selector.is_empty() {
                    selector_strings.push(selector);
                }
                current_pos = array_start + end_pos + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    selector_strings.join(", ")
}

/// Parse a single selector array: "button", "mat-button", ""
/// Returns: "button[mat-button]"
fn parse_single_selector_array(selector_array_str: &str) -> String {
    let parts: Vec<&str> = selector_array_str
        .split(',')
        .map(|s| s.trim().trim_matches('"').trim_matches('\''))
        .filter(|s| !s.is_empty())
        .collect();

    if parts.is_empty() {
        return String::new();
    }

    // First part is tag name, rest are attribute pairs (name, value)
    let tag = parts[0];
    let mut attrs = Vec::new();

    // Attributes come in pairs: [name, value, name, value, ...]
    for i in (1..parts.len()).step_by(2) {
        let attr_name = parts[i];
        let attr_value = if i + 1 < parts.len() {
            parts[i + 1]
        } else {
            ""
        };

        if !attr_name.is_empty() {
            if attr_value.is_empty() {
                attrs.push(attr_name.to_string());
            } else {
                attrs.push(format!("{}={}", attr_name, attr_value));
            }
        }
    }

    if attrs.is_empty() {
        tag.to_string()
    } else {
        format!("{}[{}]", tag, attrs.join("]["))
    }
}

/// Extract hostAttrs for a specific class from the linked code
fn extract_host_attrs_for_class(
    class_name: &str,
    linked_code: &str,
    pattern: &Option<regex::Regex>,
) -> Vec<String> {
    // Find the class definition block and extract hostAttrs
    // This is a simplified extraction - in practice we'd use AST parsing

    let class_marker = format!("{}.ɵcmp", class_name);
    if let Some(start) = linked_code.find(&class_marker) {
        // Find the hostAttrs within a reasonable range
        let search_range = &linked_code[start..std::cmp::min(start + 2000, linked_code.len())];

        if let Some(re) = pattern {
            if let Some(caps) = re.captures(search_range) {
                if let Some(attrs) = caps.get(1) {
                    return attrs
                        .as_str()
                        .split(',')
                        .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            }
        }
    }

    vec![]
}

/// Look up an NgModule's exported directives from the cache
pub fn get_module_exports(module_path: &str, module_name: &str) -> Option<Vec<ExtractedDirective>> {
    let cache = get_metadata_cache().read().ok()?;
    let module = cache
        .modules
        .get(&format!("{}:{}", module_path, module_name))?;

    let mut result = Vec::new();
    for export_name in &module.exports {
        if let Some(directive) = cache
            .directives
            .get(&format!("{}:{}", module_path, export_name))
        {
            result.push(directive.clone());
        }
    }

    Some(result)
}

/// Look up a directive's metadata from the cache
pub fn get_directive_metadata(
    module_path: &str,
    directive_name: &str,
) -> Option<ExtractedDirective> {
    let cache = get_metadata_cache().read().ok()?;
    cache
        .directives
        .get(&format!("{}:{}", module_path, directive_name))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_selector_array() {
        assert_eq!(
            parse_single_selector_array(r#""button", "mat-button", """#),
            "button[mat-button]"
        );
        assert_eq!(parse_single_selector_array(r#""div""#), "div");
    }

    #[test]
    fn test_parse_selectors_array() {
        assert_eq!(
            parse_selectors_array(r#"["button", "mat-button", ""], ["a", "mat-button", ""]"#),
            "button[mat-button], a[mat-button]"
        );
        assert_eq!(
            parse_selectors_array(
                r#"["button", "matButton", ""], ["a", "matButton", ""], ["button", "mat-button", ""]"#
            ),
            "button[matButton], a[matButton], button[mat-button]"
        );
    }

    #[test]
    fn test_extract_metadata_from_linked() {
        let code = r#"
            MatButtonModule.ɵmod = ɵɵdefineNgModule({
                type: MatButtonModule,
                exports: [MatButton, MatFabButton]
            });
            
            MatButton.ɵcmp = ɵɵdefineComponent({
                selectors: [["button", "mat-button", ""]],
                hostAttrs: [1, "mdc-button"]
            });
        "#;

        let (modules, directives) = extract_metadata_from_linked("@angular/material/button", code);

        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "MatButtonModule");
        assert!(modules[0].exports.contains(&"MatButton".to_string()));

        assert_eq!(directives.len(), 1);
        assert_eq!(directives[0].name, "MatButton");
        assert_eq!(directives[0].selector, "button[mat-button]");
    }
}
