use crate::ngtsc::file_system::src::helpers::{get_file_system, set_file_system};
use crate::ngtsc::file_system::src::logical::{
    LogicalFileSystem, LogicalProjectPath, LogicalProjectPathHelper,
};
use crate::ngtsc::file_system::src::types::AbsoluteFsPath;
use crate::ngtsc::file_system::testing::test_helper::{
    init_mock_file_system, run_in_each_file_system,
};
use std::sync::Arc;

fn absolute_from(path: &str) -> AbsoluteFsPath {
    AbsoluteFsPath::new(path.to_string())
}

#[test]
fn test_logical_paths() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        set_file_system(Arc::new(fs.clone()));

        let is_case_sensitive = get_file_system().is_case_sensitive();
        // Emulate NgtscCompilerHost canonicalization logic
        let canonicalizer: Arc<dyn Fn(&str) -> String + Send + Sync> = if is_case_sensitive {
            Arc::new(|p| p.to_string())
        } else {
            Arc::new(|p| p.to_lowercase())
        };

        // Single root
        {
            let mut lfs =
                LogicalFileSystem::new(vec![absolute_from("/test")], canonicalizer.clone());
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/test/foo/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo/foo"
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/test/bar/bar.ts"))
                    .unwrap()
                    .as_str(),
                "/bar/bar"
            );
            assert!(lfs
                .logical_path_of_file(&absolute_from("/not-test/bar.ts"))
                .is_none());
        }

        // Multi root
        {
            let mut lfs = LogicalFileSystem::new(
                vec![absolute_from("/test/foo"), absolute_from("/test/bar")],
                canonicalizer.clone(),
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/test/foo/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo"
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/test/bar/bar.ts"))
                    .unwrap()
                    .as_str(),
                "/bar"
            );
        }

        // Nested roots
        {
            let mut lfs = LogicalFileSystem::new(
                vec![absolute_from("/test"), absolute_from("/test/dist")],
                canonicalizer.clone(),
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/test/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo"
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/test/dist/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo"
            );
        }

        // Root prefix
        {
            let mut root_fs =
                LogicalFileSystem::new(vec![absolute_from("/")], canonicalizer.clone());
            assert_eq!(
                root_fs
                    .logical_path_of_file(&absolute_from("/foo/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo/foo"
            );

            let mut non_root_fs =
                LogicalFileSystem::new(vec![absolute_from("/test/")], canonicalizer.clone());
            assert_eq!(
                non_root_fs
                    .logical_path_of_file(&absolute_from("/test/foo/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo/foo"
            );
        }

        // Casing
        {
            let mut lfs =
                LogicalFileSystem::new(vec![absolute_from("/Test")], canonicalizer.clone());
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/Test/foo/Foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo/Foo"
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/Test/foo/foo.ts"))
                    .unwrap()
                    .as_str(),
                "/foo/foo"
            );
            assert_eq!(
                lfs.logical_path_of_file(&absolute_from("/Test/bar/bAR.ts"))
                    .unwrap()
                    .as_str(),
                "/bar/bAR"
            );
        }

        // Case sensitivity matching rootDirs
        {
            let mut lfs =
                LogicalFileSystem::new(vec![absolute_from("/Test")], canonicalizer.clone());
            if is_case_sensitive {
                assert!(lfs
                    .logical_path_of_file(&absolute_from("/test/car/CAR.ts"))
                    .is_none());
            } else {
                assert_eq!(
                    lfs.logical_path_of_file(&absolute_from("/test/car/CAR.ts"))
                        .unwrap()
                        .as_str(),
                    "/car/CAR"
                );
            }
        }
    });
}

#[test]
fn test_utilities() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        set_file_system(Arc::new(fs.clone()));

        // Adjacent
        {
            let res = LogicalProjectPathHelper::relative_path_between(
                &LogicalProjectPath::new("/foo".to_string()),
                &LogicalProjectPath::new("/bar".to_string()),
            );
            assert_eq!(res.as_str(), "./bar");
        }

        // Non-adjacent
        {
            let res = LogicalProjectPathHelper::relative_path_between(
                &LogicalProjectPath::new("/foo/index".to_string()),
                &LogicalProjectPath::new("/bar/index".to_string()),
            );
            assert_eq!(res.as_str(), "../bar/index");
        }

        // Casing relative
        {
            let res = LogicalProjectPathHelper::relative_path_between(
                &LogicalProjectPath::new("/fOO".to_string()),
                &LogicalProjectPath::new("/bAR".to_string()),
            );
            assert_eq!(res.as_str(), "./bAR");
        }
    });
}
