use oxc_allocator::Allocator;
use oxc_ast::ast::Statement;
use oxc_parser::Parser;
use oxc_span::SourceType;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve_dependencies(entry_point: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut visited = HashSet::new();
    let mut results = Vec::new();
    let mut stack = vec![entry_point.to_path_buf()];

    eprintln!("Resolving dependencies starting from: {:?}", entry_point);

    while let Some(current_path) = stack.pop() {
        if !visited.insert(current_path.clone()) {
            continue;
        }

        eprintln!("Processing: {:?}", current_path);
        results.push(current_path.clone());

        if let Ok(content) = fs::read_to_string(&current_path) {
            let allocator = Allocator::default();
            let source_type = SourceType::from_path(&current_path).unwrap_or_default();

            let ret = Parser::new(&allocator, &content, source_type).parse();

            for statement in ret.program.body {
                let source = match statement {
                    Statement::ImportDeclaration(decl) => Some(decl.source.value.as_str()),
                    Statement::ExportNamedDeclaration(decl) => {
                        decl.source.as_ref().map(|s| s.value.as_str())
                    }
                    Statement::ExportAllDeclaration(decl) => Some(decl.source.value.as_str()),
                    _ => None,
                };

                if let Some(source) = source {
                    if source.starts_with(".") {
                        let dir = current_path.parent().unwrap_or(Path::new("."));
                        let base_resolved = dir.join(source);

                        // Resolution logic
                        let candidates = vec![
                            base_resolved.clone(),
                            base_resolved.with_extension("ts"),
                            base_resolved.with_extension("tsx"),
                            base_resolved.with_extension("js"),
                            base_resolved.join("index.ts"),
                            base_resolved.join("index.tsx"),
                            base_resolved.join("index.js"),
                        ];

                        let mut found = None;
                        for candidate in candidates {
                            if candidate.exists() && candidate.is_file() {
                                found = Some(candidate);
                                break;
                            }
                        }

                        if let Some(resolved) = found {
                            eprintln!("  Resolved '{}' to {:?}", source, resolved);
                            stack.push(resolved);
                        } else {
                            eprintln!("  Failed to resolve '{}' from {:?}", source, current_path);
                        }
                    }
                }
            }
        } else {
            eprintln!("  Failed to read file: {:?}", current_path);
        }
    }

    // Reverse to process dependencies first (simple topological sort approximation)
    results.reverse();
    Ok(results)
}
