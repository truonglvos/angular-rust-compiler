use crate::ngtsc::file_system::src::helpers::{absolute_from, relative_from, set_file_system};
use crate::ngtsc::file_system::testing::test_helper::{
    init_mock_file_system, run_in_each_file_system,
};
use std::sync::Arc;

#[test]
fn test_helpers_absolute_from() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        set_file_system(Arc::new(fs.clone()));

        let is_windows = os == "Windows";

        if is_windows {
            // Windows tests - only run Windows-style paths for Windows FS
            let p = absolute_from("C:/test.txt");
            assert_eq!(p.as_str(), "C:/test.txt");

            // Backslash normalization
            let p = absolute_from("C:\\test.txt");
            assert_eq!(p.as_str(), "C:/test.txt");

            // Drive letters
            let p = absolute_from("D:\\foo\\test.txt");
            assert_eq!(p.as_str(), "D:/foo/test.txt");
        } else {
            // Unix/OSX tests - only run Unix-style paths for Unix/OSX FS
            let p = absolute_from("/test.txt");
            assert_eq!(p.as_str(), "/test.txt");

            let p = absolute_from("/a/b/c.txt");
            assert_eq!(p.as_str(), "/a/b/c.txt");
        }
    });
}

#[test]
fn test_helpers_relative_from() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        set_file_system(Arc::new(fs.clone()));

        // Relative path - works on all OS types
        let p = relative_from("a/b/c.txt");
        assert_eq!(p.as_str(), "a/b/c.txt");

        let p = relative_from("foo.ts");
        assert_eq!(p.as_str(), "foo.ts");
    });
}
