use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::fs;

pub fn resolve_dependencies(entry_point: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut visited = HashSet::new();
    let mut results = Vec::new();
    let mut stack = vec![entry_point.to_path_buf()];

    while let Some(current_path) = stack.pop() {
        if !visited.insert(current_path.clone()) {
            continue;
        }

        results.push(current_path.clone());

        if let Ok(content) = fs::read_to_string(&current_path) {
            let allocator = Allocator::default();
            let source_type = SourceType::from_path(&current_path).unwrap_or_default();
            
            let ret = Parser::new(&allocator, &content, source_type).parse();
            
            // Very simple AST traversal to find imports
            for statement in ret.program.body {
                if let oxc_ast::ast::Statement::ImportDeclaration(import_decl) = statement {
                     let source = import_decl.source.value.as_str();
                     
                     // Skip node_modules (non-relative imports)
                     if source.starts_with(".") {
                         let dir = current_path.parent().unwrap_or(Path::new("."));
                         let mut resolved = dir.join(source);
                         
                         // Extension and index resolution
                         if !resolved.exists() {
                             // 1. Try .ts
                             let ts = resolved.with_extension("ts");
                             if ts.exists() {
                                 resolved = ts;
                             } else {
                                 // 2. Try .d.ts?? No, usually we care about source. 
                                 // 3. Try index.ts
                                 let index_ts = resolved.join("index.ts");
                                 if index_ts.exists() {
                                     resolved = index_ts;
                                 }
                             }
                         } else if resolved.is_dir() {
                             // If it resolves to a directory (and exists), check for index.ts
                             let index_ts = resolved.join("index.ts");
                             if index_ts.exists() {
                                 resolved = index_ts;
                             }
                         }

                         // Only process TS files (ignoring potentially resolved JS/assets involved in imports for now)
                         if resolved.exists() && 
                            (resolved.extension().map_or(false, |ext| ext == "ts") || resolved.extension().map_or(false, |ext| ext == "tsx")) {
                             stack.push(resolved);
                         }
                     }
                }
            }
        }
    }
    
    // Reverse to process dependencies first (simple topological sort approximation)
    results.reverse();
    Ok(results)
}
