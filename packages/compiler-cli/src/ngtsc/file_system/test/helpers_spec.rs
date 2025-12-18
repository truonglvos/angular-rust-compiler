use crate::ngtsc::file_system::testing::test_helper::{run_in_each_file_system, init_mock_file_system};
use crate::ngtsc::file_system::src::helpers::{absolute_from, relative_from, set_file_system};
use std::sync::Arc;

#[test]
fn test_helpers_absolute_from() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        set_file_system(Arc::new(fs.clone()));
        
        let is_windows = os == "Windows";
        
        if is_windows {
            // Windows tests
            let p = absolute_from("C:/test.txt");
            assert_eq!(p.as_str(), "C:/test.txt");
            
            // Backslash normalization
            let p = absolute_from("C:\\test.txt");
            assert_eq!(p.as_str(), "C:/test.txt");
            
            // Drive letters
            let p = absolute_from("D:\\foo\\test.txt");
            assert_eq!(p.as_str(), "D:/foo/test.txt");
            
             // Non-absolute path should panic
             let result = std::panic::catch_unwind(|| absolute_from("test.txt"));
             assert!(result.is_err(), "Should have panicked for relative path on Windows");
             
        } else {
             // Unix/OSX tests
            let p = absolute_from("/test.txt");
            assert_eq!(p.as_str(), "/test.txt");
            
             // Non-absolute path should panic
             let result = std::panic::catch_unwind(|| absolute_from("test.txt"));
             assert!(result.is_err(), "Should have panicked for relative path on Unix");
             
             // Windows-style absolute is relative on Unix
             // Note: MockFileSystem usage of 'clean_path' might treat C:/... differently?
             // Actually PosixUtils is_root checks if starts with /. "C:/..." does not.
             let result = std::panic::catch_unwind(|| absolute_from("C:/test.txt"));
             assert!(result.is_err(), "Should have panicked for 'C:/foo' on Unix");
        }
    });
}

#[test]
fn test_helpers_relative_from() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        set_file_system(Arc::new(fs.clone()));
        
        let is_windows = os == "Windows";
        
        // Relative path
        let p = relative_from("a/b/c.txt");
        assert_eq!(p.as_str(), "a/b/c.txt");
        
        // Absolute path should panic
        if is_windows {
             let result = std::panic::catch_unwind(|| relative_from("C:/a/b/c.txt"));
             assert!(result.is_err());
        } else {
             let result = std::panic::catch_unwind(|| relative_from("/a/b/c.txt"));
             assert!(result.is_err());
        }
    });
}
