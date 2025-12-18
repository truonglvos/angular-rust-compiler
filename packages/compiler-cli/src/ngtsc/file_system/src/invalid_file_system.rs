use crate::ngtsc::file_system::src::types::{
    AbsoluteFsPath, FileStats, FileSystem, PathManipulation, PathSegment, ReadonlyFileSystem,
};
use std::io::{self, Error, ErrorKind};

pub struct InvalidFileSystem;

impl PathManipulation for InvalidFileSystem {
    fn extname(&self, _path: &str) -> String {
        make_error()
    }
    fn is_root(&self, _path: &AbsoluteFsPath) -> bool {
        make_error()
    }
    fn is_rooted(&self, _path: &str) -> bool {
        make_error()
    }
    fn dirname(&self, _file: &str) -> String {
        make_error()
    }
    fn join(&self, _base_path: &str, _paths: &[&str]) -> String {
        make_error()
    }
    fn relative(&self, _from: &str, _to: &str) -> String {
        make_error()
    }
    fn basename(&self, _file_path: &str, _extension: Option<&str>) -> PathSegment {
        make_error()
    }
    fn normalize(&self, _path: &str) -> String {
        make_error()
    }
    fn resolve(&self, _paths: &[&str]) -> AbsoluteFsPath {
        make_error()
    }
    fn pwd(&self) -> AbsoluteFsPath {
        make_error()
    }
    fn chdir(&self, _path: &AbsoluteFsPath) {
        make_error()
    }
}

impl ReadonlyFileSystem for InvalidFileSystem {
    fn is_case_sensitive(&self) -> bool {
        make_error()
    }
    fn exists(&self, _path: &AbsoluteFsPath) -> bool {
        make_error()
    }
    fn read_file(&self, _path: &AbsoluteFsPath) -> io::Result<String> {
        Err(make_io_error())
    }
    fn read_file_buffer(&self, _path: &AbsoluteFsPath) -> io::Result<Vec<u8>> {
        Err(make_io_error())
    }
    fn readdir(&self, _path: &AbsoluteFsPath) -> io::Result<Vec<PathSegment>> {
        Err(make_io_error())
    }
    fn lstat(&self, _path: &AbsoluteFsPath) -> io::Result<FileStats> {
        Err(make_io_error())
    }
    fn stat(&self, _path: &AbsoluteFsPath) -> io::Result<FileStats> {
        Err(make_io_error())
    }
    fn realpath(&self, _file_path: &AbsoluteFsPath) -> io::Result<AbsoluteFsPath> {
        Err(make_io_error())
    }
    fn get_default_lib_location(&self) -> AbsoluteFsPath {
        make_error()
    }
}

impl FileSystem for InvalidFileSystem {
    fn write_file(&self, _path: &AbsoluteFsPath, _data: &[u8], _exclusive: Option<bool>) -> io::Result<()> {
        Err(make_io_error())
    }
    fn remove_file(&self, _path: &AbsoluteFsPath) -> io::Result<()> {
        Err(make_io_error())
    }
    fn symlink(&self, _target: &AbsoluteFsPath, _path: &AbsoluteFsPath) -> io::Result<()> {
        Err(make_io_error())
    }
    fn copy_file(&self, _from: &AbsoluteFsPath, _to: &AbsoluteFsPath) -> io::Result<()> {
        Err(make_io_error())
    }
    fn move_file(&self, _from: &AbsoluteFsPath, _to: &AbsoluteFsPath) -> io::Result<()> {
        Err(make_io_error())
    }
    fn ensure_dir(&self, _path: &AbsoluteFsPath) -> io::Result<()> {
        Err(make_io_error())
    }
    fn remove_deep(&self, _path: &AbsoluteFsPath) -> io::Result<()> {
        Err(make_io_error())
    }
}

fn make_error() -> ! {
    panic!("FileSystem has not been configured. Please call `set_file_system()` before calling this method.");
}

fn make_io_error() -> Error {
    Error::new(ErrorKind::Other, "FileSystem has not been configured. Please call `set_file_system()` before calling this method.")
}
