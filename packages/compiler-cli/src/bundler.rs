use std::path::{Path, PathBuf};
use crate::config::angular::{AngularConfig, BuildOptions};
use glob::glob;
use crate::compile::parallel::parallel_compile;
use anyhow::Result;
use std::collections::HashMap;

pub struct BundleResult {
    pub bundle_js: String,
    pub styles_css: Option<String>,
    pub scripts_js: Option<String>,
    pub index_html: Option<String>,
    pub files: HashMap<String, String>,
}

pub fn bundle_project(project_path: &Path) -> Result<BundleResult> {
    // 1. Load configuration
    let config = AngularConfig::load(project_path)?;
    let (name, project) = config.projects.iter().next().ok_or_else(|| anyhow::anyhow!("No project found"))?;

    let build_options = project.architect.as_ref()
        .and_then(|a| a.get("build"))
        .and_then(|t| t.options.as_ref());
        
    let root_dir = project_path.parent().unwrap_or_else(|| Path::new("."));

    // 2. Resolve Entry Point (Still useful for bootstrapping, though we compile everything)
    // We don't strictly need main_file here if we glob everything, but logic might use it?
    // Actually we don't use main_file for anything other than resolve_dependencies which we are removing.
    
    // 3. Resolve Files (Glob all .ts files)
    let pattern = root_dir.join("src/**/*.ts");
    let pattern_str = pattern.to_string_lossy();
    
    let files: Vec<PathBuf> = glob(&pattern_str)
        .map_err(|e| anyhow::anyhow!("Failed to read glob pattern: {}", e))?
        .filter_map(Result::ok)
        .filter(|p| !p.to_string_lossy().ends_with(".spec.ts"))
        .collect();

    // 4. Compile
    let compiled_contents = parallel_compile(&files, project_path)?;

    // 5. Bundle (Concatenate) & Populate Virtual FS
    let mut bundle_js = String::new();
    let mut files_map = HashMap::new();

    // Add preamble for polyfills if needed
    bundle_js.push_str("import 'zone.js';\n");

    let import_regex = regex::Regex::new(r#"(from\s+['"])([\.\/][^'"]+)(['"])"#).unwrap();

    for (path, content) in compiled_contents {
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        // Calculate relative path for Virtual FS (relative to project root/CWD?)
        // The project_path passed in is "angular.json" path.
        // We want paths relative to that directory usually.
        let relative_file_path = path.strip_prefix(project_path.parent().unwrap_or(Path::new(".")))
            .unwrap_or(&path);
        let relative_path_str = relative_file_path.to_string_lossy().to_string();
        
        // Populate files map with original content
        // We ensure keys start with / or ./ to be clear? 
        // Vite expects /src/... for absolute virtual files.
        // Let's store as `src/app/main.js` (relative).
        files_map.insert(relative_path_str, content.clone());

        if extension == "js" {
            // Rewrite imports to be root-relative for the aggregated bundle
            let file_dir = relative_file_path.parent().unwrap_or(Path::new("."));

            let rewritten_content = import_regex.replace_all(&content, |caps: &regex::Captures| {
                let prefix = &caps[1];
                let import_path = &caps[2];
                let suffix = &caps[3];

                if import_path.starts_with('.') {
                    // unexpected relative path
                    let joined = file_dir.join(import_path);
                    let mut new_path = joined.to_string_lossy().to_string();
                    if !new_path.starts_with('.') && !new_path.starts_with('/') {
                        new_path = format!("./{}", new_path);
                    }
                    format!("{}{}{}", prefix, new_path, suffix)
                } else {
                    caps[0].to_string()
                }
            });

            bundle_js.push_str(&format!("// File: {}\n", path.display()));
            bundle_js.push_str(&rewritten_content);
            bundle_js.push_str("\n");
        }
    }

    // 6. Process Styles
    let mut styles_css = None;
    if let Some(options) = build_options {
        if let Some(styles) = &options.styles {
             let mut combined_css = String::new();
             for style in styles {
                let path = root_dir.join(style);
                if path.exists() {
                    let content = std::fs::read_to_string(&path)?;
                    // Add to files map too?
                    files_map.insert(style.clone(), content.clone());

                    combined_css.push_str(&format!("/* {} */\n", style));
                    combined_css.push_str(&content);
                    combined_css.push_str("\n");
                }
             }
             if !combined_css.is_empty() {
                 styles_css = Some(combined_css);
             }
        }
    }

    // 7. Process Scripts
    let mut scripts_js = None;
    if let Some(options) = build_options {
        if let Some(scripts) = &options.scripts {
            let mut combined_js = String::new();
            for script in scripts {
                 let path = root_dir.join(script);
                 if path.exists() {
                    let content = std::fs::read_to_string(&path)?;
                    // Add to files map
                    files_map.insert(script.clone(), content.clone());

                    combined_js.push_str(&format!("// {} \n", script));
                    combined_js.push_str(&content);
                    combined_js.push_str("\n");
                 }
            }
            if !combined_js.is_empty() {
                scripts_js = Some(combined_js);
            }
        }
    }

    // 8. Process Index HTML
    let mut index_html = None;
    if let Some(options) = build_options {
        if let Some(index) = &options.index {
            let src_path = root_dir.join(index);
            if src_path.exists() {
                let mut content = std::fs::read_to_string(&src_path)?;
                
                // Inject styles
                if styles_css.is_some() {
                     let link_tag = r#"<link rel="stylesheet" href="styles.css">"#;
                     if let Some(pos) = content.find("</head>") {
                         content.insert_str(pos, &format!("{}\n", link_tag));
                     } else {
                         content.push_str(&format!("\n{}", link_tag));
                     }
                }

                // Inject bundle
                let script_tag = r#"<script src="bundle.js" type="module"></script>"#;
                if let Some(pos) = content.find("</body>") {
                    content.insert_str(pos, &format!("{}\n", script_tag));
                } else {
                     content.push_str(&format!("\n{}", script_tag));
                }

                // Inject scripts
                if scripts_js.is_some() {
                    let script_tag = r#"<script src="scripts.js" defer></script>"#;
                    if let Some(pos) = content.find("</body>") {
                        content.insert_str(pos, &format!("{}\n", script_tag));
                    } else {
                         content.push_str(&format!("\n{}", script_tag));
                    }
                }
                index_html = Some(content);
            }
        }
    }

    Ok(BundleResult {
        bundle_js,
        styles_css,
        scripts_js,
        index_html,
        files: files_map,
    })
}
