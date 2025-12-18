use crate::ngtsc::file_system::src::helpers::{
    absolute_from_source_file, dirname, is_local_relative_path, relative, resolve, to_relative_import,
};
use crate::ngtsc::file_system::src::types::{AbsoluteFsPath, BrandedPath, PathSegment};
use crate::ngtsc::file_system::src::util::strip_extension;
use std::collections::HashMap;
use std::marker::PhantomData;

/// A path that's relative to the logical root of a TypeScript project (one of the project's
/// rootDirs).
///
/// Paths in the type system use POSIX format.
pub type LogicalProjectPath = BrandedPath<LogicalProjectPathBrand>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicalProjectPathBrand;

pub struct LogicalFileSystem {
    /// The root directories of the project, sorted with the longest path first.
    root_dirs: Vec<AbsoluteFsPath>,

    /// The same root directories as `root_dirs` but with each one converted to its
    /// canonical form for matching in case-insensitive file-systems.
    canonical_root_dirs: Vec<AbsoluteFsPath>,

    /// A cache of file paths to project paths, because computation of these paths is slightly
    /// expensive.
    cache: HashMap<AbsoluteFsPath, Option<LogicalProjectPath>>,

    // In TS this is `private compilerHost: Pick<ts.CompilerHost, 'getCanonicalFileName'>`.
    // We can just store a closure or function pointer, or since we are 1:1 porting within our compiler context,
    // we assume we have a way to canonicalize.
    // Minimally we need `get_canonical_file_name`.
    // Let's store a generic or use a boxed closure.
    // For now, let's use a function pointer or trait object if strictly needed.
    // But simplest is to pass a canonicalizer.
    canonicalizer: std::sync::Arc<dyn Fn(&str) -> String + Send + Sync>,
}

impl LogicalFileSystem {
    pub fn new(
        root_dirs: Vec<AbsoluteFsPath>,
        canonicalizer: std::sync::Arc<dyn Fn(&str) -> String + Send + Sync>,
    ) -> Self {
        // Make a copy and sort it by length in reverse order (longest first).
        let mut sorted_root_dirs = root_dirs.clone();
        sorted_root_dirs.sort_by(|a, b| b.as_str().len().cmp(&a.as_str().len()));

        let canonical_root_dirs = sorted_root_dirs
            .iter()
            .map(|dir| AbsoluteFsPath::new(canonicalizer(dir.as_str())))
            .collect();

        Self {
            root_dirs: sorted_root_dirs,
            canonical_root_dirs,
            cache: HashMap::new(),
            canonicalizer,
        }
    }

    /// Get the logical path in the project of a `ts.SourceFile`.
    pub fn logical_path_of_sf(&mut self, sf: &oxc_ast::ast::Program) -> Option<LogicalProjectPath> {
        self.logical_path_of_file(&absolute_from_source_file(sf))
    }

    /// Get the logical path in the project of a source file.
    pub fn logical_path_of_file(&mut self, physical_file: &AbsoluteFsPath) -> Option<LogicalProjectPath> {
        if !self.cache.contains_key(physical_file) {
            let canonical_file_path = AbsoluteFsPath::new((self.canonicalizer)(physical_file.as_str()));
            let mut logical_file: Option<LogicalProjectPath> = None;

            for i in 0..self.root_dirs.len() {
                let root_dir = &self.root_dirs[i];
                let canonical_root_dir = &self.canonical_root_dirs[i];

                if is_within_base_path(canonical_root_dir, &canonical_file_path) {
                    let log_file = self.create_logical_project_path(physical_file, root_dir);
                    // The logical project does not include any special "node_modules" nested directories.
                    if log_file.as_str().contains("/node_modules/") {
                        logical_file = None;
                    } else {
                        logical_file = Some(log_file);
                        break;
                    }
                }
            }
            self.cache.insert(physical_file.clone(), logical_file);
        }
        self.cache.get(physical_file).unwrap().clone()
    }

    fn create_logical_project_path(
        &self,
        file: &AbsoluteFsPath,
        root_dir: &AbsoluteFsPath,
    ) -> LogicalProjectPath {
        // file.slice(root_dir.length)
        let relative_path_str = &file.as_str()[root_dir.as_str().len()..];
        let logical_path = strip_extension(relative_path_str);
        
        let normalized = if logical_path.starts_with('/') {
            logical_path
        } else {
            format!("/{}", logical_path)
        };
        
        LogicalProjectPath::new(normalized)
    }
}

fn is_within_base_path(base: &AbsoluteFsPath, path: &AbsoluteFsPath) -> bool {
    is_local_relative_path(&relative(base.as_str(), path.as_str()))
}

pub struct LogicalProjectPathHelper;
impl LogicalProjectPathHelper {
     /// Get the relative path between two `LogicalProjectPath`s.
    pub fn relative_path_between(from: &LogicalProjectPath, to: &LogicalProjectPath) -> PathSegment {
         let from_abs = resolve(from.as_str(), &[]);
         let from_dir = dirname(from_abs.as_str());
         let to_abs = resolve(to.as_str(), &[]);
         let relative_path = relative(&from_dir, to_abs.as_str());
         PathSegment::new(to_relative_import(&relative_path))
    }
}
