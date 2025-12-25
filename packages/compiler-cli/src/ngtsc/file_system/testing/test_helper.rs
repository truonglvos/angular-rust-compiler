use crate::ngtsc::file_system::testing::mock_file_system::MockFileSystem;
use crate::ngtsc::file_system::testing::mock_file_system_posix::PosixUtils;
use crate::ngtsc::file_system::testing::mock_file_system_windows::WindowsUtils;
use std::sync::Arc;

pub fn init_mock_file_system(os: &str) -> MockFileSystem {
    match os {
        "OS/X" => MockFileSystem::new(Arc::new(PosixUtils {
            is_case_sensitive: false,
        })),
        "Unix" => MockFileSystem::new(Arc::new(PosixUtils {
            is_case_sensitive: true,
        })),
        "Windows" => MockFileSystem::new(Arc::new(WindowsUtils)),
        _ => panic!("Unsupported OS: {}", os),
    }
}

pub fn run_in_each_file_system<F>(mut test_fn: F)
where
    F: FnMut(&str),
{
    for os in &["OS/X", "Unix", "Windows"] {
        test_fn(os);
    }
}
