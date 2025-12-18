use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io;
use std::path::{Path, PathBuf};
use crate::ngtsc::file_system::src::types::{AbsoluteFsPath, FileStats, FileSystem, PathManipulation, PathSegment, ReadonlyFileSystem};
use crate::ngtsc::file_system::src::util::clean_path;

// Import strategies from sibling modules
use super::mock_file_system_posix::PosixUtils;
use super::mock_file_system_windows::WindowsUtils;
use super::mock_file_system_native::NativeUtils;

#[derive(Clone, Debug)]
pub enum Entity {
    Folder(Box<Folder>),
    File(Vec<u8>),
    SymLink(AbsoluteFsPath),
}

pub type Folder = HashMap<String, Entity>;

/// Strategy for path manipulation (Posix vs Windows)
pub trait PathStrategy: Send + Sync {
    fn split_path(&self, path: &str) -> Vec<String>;
    fn normalize(&self, path: &str) -> String;
    fn dirname(&self, path: &str) -> String;
    fn join(&self, base_path: &str, paths: &[&str]) -> String;
    fn relative(&self, from: &str, to: &str) -> String;
    fn basename(&self, path: &str, ext: Option<&str>) -> String;
    fn is_case_sensitive(&self) -> bool;
    fn resolve(&self, cwd: &str, paths: &[&str]) -> AbsoluteFsPath;
    fn is_root(&self, path: &str) -> bool;
}

#[derive(Clone)]
pub struct MockFileSystem {
    strategy: Arc<dyn PathStrategy>,
    cwd: Arc<Mutex<AbsoluteFsPath>>,
    file_tree: Arc<Mutex<Folder>>,
}

impl MockFileSystem {
    pub fn new(strategy: Arc<dyn PathStrategy>) -> Self {
        MockFileSystem {
            strategy,
            cwd: Arc::new(Mutex::new(AbsoluteFsPath::new("/".to_string()))),
            file_tree: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn new_posix() -> Self {
        Self::new(Arc::new(PosixUtils { is_case_sensitive: true }))
    }
    
    pub fn new_windows() -> Self {
        Self::new(Arc::new(WindowsUtils))
    }
    
    pub fn new_native() -> Self {
        Self::new(Arc::new(NativeUtils))
    }

    
    pub fn init_with_files(&self, files: Vec<(&str, &str)>) {
        for (path, content) in files {
            // Ensure path is absolute for the strategy?
             // If implicit absolute, fine.
            let _ = self.write_file(&AbsoluteFsPath::new(path.to_string()), content.as_bytes(), None);
        }
    }
    
    // Internal helper to lookup an entity. Returns Clone of entity to avoid borrow issues with Mutex.
    fn get_entity(&self, path: &AbsoluteFsPath) -> Option<Entity> {
        let tree = self.file_tree.lock().unwrap();
        let segments = self.strategy.split_path(path.as_str());
        let mut current = &*tree; // &HashMap
        
        for (i, segment) in segments.iter().enumerate() {
            match current.get(segment) {
                Some(Entity::Folder(map)) => {
                    if i == segments.len() - 1 { return Some(Entity::Folder(map.clone())); }
                    current = map;
                },
                Some(Entity::File(content)) => {
                    if i == segments.len() - 1 { return Some(Entity::File(content.clone())); }
                    return None; // File mid-path
                },
                Some(Entity::SymLink(target)) => {
                     if i == segments.len() - 1 { return Some(Entity::SymLink(target.clone())); }
                     return None; 
                },
                None => return None,
            }
        }
        // Root
        Some(Entity::Folder(Box::new(tree.clone())))
    }
}

impl FileSystem for MockFileSystem {
    fn write_file(&self, path: &AbsoluteFsPath, data: &[u8], exclusive: Option<bool>) -> io::Result<()> {
        let mut tree = self.file_tree.lock().unwrap();
        let segments = self.strategy.split_path(path.as_str());
        
        if segments.is_empty() { return Err(io::Error::new(io::ErrorKind::Other, "Cannot write to root")); }
        
        let file_name = segments.last().unwrap().clone();
        let dir_segments = &segments[..segments.len()-1];
        
        let mut current = &mut *tree;
        for segment in dir_segments {
            if !current.contains_key(segment) {
                 return Err(io::Error::new(io::ErrorKind::NotFound, format!("Directory {} not found", segment)));
            }
            
            let next = current.get_mut(segment).unwrap();
            match next {
                Entity::Folder(map) => current = map,
                _ => return Err(io::Error::new(io::ErrorKind::Other, "Not a directory")),
            }
        }
        
        if exclusive.unwrap_or(false) && current.contains_key(&file_name) {
             return Err(io::Error::new(io::ErrorKind::AlreadyExists, "File exists"));
        }
        
        current.insert(file_name, Entity::File(data.to_vec()));
        Ok(())
    }

    fn remove_file(&self, path: &AbsoluteFsPath) -> io::Result<()> {
        let mut tree = self.file_tree.lock().unwrap();
        let segments = self.strategy.split_path(path.as_str());
         if segments.is_empty() { return Err(io::Error::new(io::ErrorKind::Other, "Root")); }
        
        let file_name = segments.last().unwrap().clone();
        let dir_segments = &segments[..segments.len()-1];
        
        let mut current = &mut *tree;
        for segment in dir_segments {
             match current.get_mut(segment) {
                 Some(Entity::Folder(map)) => current = map,
                 _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Path not found")),
             }
        }
        
        if let Some(entry) = current.get(&file_name) {
             if let Entity::Folder(_) = entry {
                 return Err(io::Error::new(io::ErrorKind::Other, "Is a directory"));
             }
             current.remove(&file_name);
             Ok(())
        } else {
             Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
        }
    }

    fn symlink(&self, target: &AbsoluteFsPath, path: &AbsoluteFsPath) -> io::Result<()> {
        let mut tree = self.file_tree.lock().unwrap();
        let segments = self.strategy.split_path(path.as_str());
        if segments.is_empty() { return Err(io::Error::new(io::ErrorKind::Other, "Root")); }
        
        let file_name = segments.last().unwrap().clone();
        let dir_segments = &segments[..segments.len()-1];
        
        let mut current = &mut *tree;
        for segment in dir_segments {
             match current.get_mut(segment) {
                 Some(Entity::Folder(map)) => current = map,
                 _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Parent not found")),
             }
        }
        
        current.insert(file_name, Entity::SymLink(target.clone()));
        Ok(())
    }

    fn copy_file(&self, from: &AbsoluteFsPath, to: &AbsoluteFsPath) -> io::Result<()> {
        let content = self.read_file_buffer(from)?;
        self.write_file(to, &content, None)
    }

    fn move_file(&self, from: &AbsoluteFsPath, to: &AbsoluteFsPath) -> io::Result<()> {
        let content = self.read_file_buffer(from)?;
        self.write_file(to, &content, None)?;
        self.remove_file(from)
    }

    fn ensure_dir(&self, path: &AbsoluteFsPath) -> io::Result<()> {
        let mut tree = self.file_tree.lock().unwrap();
        let segments = self.strategy.split_path(path.as_str());
        
        let mut current = &mut *tree;
        for segment in segments {
            if !current.contains_key(&segment) {
                 current.insert(segment.clone(), Entity::Folder(Box::new(HashMap::new())));
            }
             
            let next = current.get_mut(&segment).unwrap();
            match next {
                Entity::Folder(map) => current = map,
                _ => return Err(io::Error::new(io::ErrorKind::Other, "Path component is not a directory")),
            }
        }
        Ok(())
    }

    fn remove_deep(&self, path: &AbsoluteFsPath) -> io::Result<()> {
         let mut tree = self.file_tree.lock().unwrap();
        let segments = self.strategy.split_path(path.as_str());
        if segments.is_empty() { return Ok(()); } // Cleaning root?
        
        let file_name = segments.last().unwrap().clone();
        let dir_segments = &segments[..segments.len()-1];
        
        let mut current = &mut *tree;
        for segment in dir_segments {
             match current.get_mut(segment) {
                 Some(Entity::Folder(map)) => current = map,
                 _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Path not found")),
             }
        }
        current.remove(&file_name);
        Ok(())
    }
}

impl ReadonlyFileSystem for MockFileSystem {
    fn is_case_sensitive(&self) -> bool { self.strategy.is_case_sensitive() }
    
    fn exists(&self, path: &AbsoluteFsPath) -> bool {
        self.get_entity(path).is_some()
    }

    fn read_file(&self, path: &AbsoluteFsPath) -> io::Result<String> {
        let bytes = self.read_file_buffer(path)?;
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn read_file_buffer(&self, path: &AbsoluteFsPath) -> io::Result<Vec<u8>> {
        match self.get_entity(path) {
            Some(Entity::File(content)) => Ok(content),
            Some(Entity::SymLink(_)) => Err(io::Error::new(io::ErrorKind::Other, "Is a symlink")), // Should follow?
            Some(Entity::Folder(_)) => Err(io::Error::new(io::ErrorKind::Other, "Is a directory")),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "File not found")),
        }
    }

    fn readdir(&self, path: &AbsoluteFsPath) -> io::Result<Vec<PathSegment>> {
        match self.get_entity(path) {
            Some(Entity::Folder(map)) => {
                Ok(map.keys().map(|k| PathSegment::new(k.clone())).collect())
            },
            Some(_) => Err(io::Error::new(io::ErrorKind::Other, "Not a directory")),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "Directory not found")),
        }
    }
    
    fn lstat(&self, path: &AbsoluteFsPath) -> io::Result<FileStats> {
        match self.get_entity(path) {
             Some(Entity::File(_)) => Ok(FileStats { is_file: true, is_directory: false, is_symbolic_link: false }),
             Some(Entity::Folder(_)) => Ok(FileStats { is_file: false, is_directory: true, is_symbolic_link: false }),
             Some(Entity::SymLink(_)) => Ok(FileStats { is_file: false, is_directory: false, is_symbolic_link: true }),
             None => Err(io::Error::new(io::ErrorKind::NotFound, "Path not found")),
        }
    }
    
    fn stat(&self, path: &AbsoluteFsPath) -> io::Result<FileStats> {
        // stat follows symlinks
        match self.get_entity(path) {
             Some(Entity::File(_)) => Ok(FileStats { is_file: true, is_directory: false, is_symbolic_link: false }),
             Some(Entity::Folder(_)) => Ok(FileStats { is_file: false, is_directory: true, is_symbolic_link: false }),
             Some(Entity::SymLink(_)) => {
                 // Should resolve target and stat that. 
                 // For now returning symlink stats as placeholder if resolution not implemented
                 Ok(FileStats { is_file: false, is_directory: false, is_symbolic_link: true })
             },
             None => Err(io::Error::new(io::ErrorKind::NotFound, "Path not found")),
        }
    }
    
    fn realpath(&self, path: &AbsoluteFsPath) -> io::Result<AbsoluteFsPath> {
        // Mock implementation: just return path if exists
        if self.exists(path) {
            Ok(path.clone())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Path not found"))
        }
    }
    
    fn get_default_lib_location(&self) -> AbsoluteFsPath {
        AbsoluteFsPath::new("/".to_string())
    }
}

impl PathManipulation for MockFileSystem {
    fn dirname(&self, file: &str) -> String { self.strategy.dirname(file) }
    fn join(&self, base_path: &str, paths: &[&str]) -> String { self.strategy.join(base_path, paths) }
    fn resolve(&self, paths: &[&str]) -> AbsoluteFsPath { 
        let cwd = self.cwd.lock().unwrap();
        self.strategy.resolve(cwd.as_str(), paths) 
    }
    fn basename(&self, path: &str, ext: Option<&str>) -> PathSegment { PathSegment::new(self.strategy.basename(path, ext)) }
    fn extname(&self, path: &str) -> String { 
         PathSegment::new(path.rsplit('.').next().map(|e| format!(".{}",e)).unwrap_or_default())
         .to_string()
    }
    fn is_root(&self, path: &AbsoluteFsPath) -> bool { self.strategy.is_root(path.as_str()) }
    fn is_rooted(&self, path: &str) -> bool { path.starts_with('/') } 
    fn normalize(&self, path: &str) -> String { self.strategy.normalize(path) }
    fn relative(&self, from: &str, to: &str) -> String { self.strategy.relative(from, to) }
    fn pwd(&self) -> AbsoluteFsPath { self.cwd.lock().unwrap().clone() }
    fn chdir(&self, path: &AbsoluteFsPath) { *self.cwd.lock().unwrap() = path.clone(); }
}
