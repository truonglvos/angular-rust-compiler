use crate::ngtsc::file_system::src::compiler_host::{CompilerOptions, NgtscCompilerHost};
use crate::ngtsc::file_system::src::helpers::{absolute_from, set_file_system};
use crate::ngtsc::file_system::src::types::{FileSystem, ReadonlyFileSystem};
use crate::ngtsc::file_system::testing::test_helper::{
    init_mock_file_system, run_in_each_file_system,
};
use std::sync::Arc;

#[test]
fn test_compiler_host() {
    run_in_each_file_system(|os| {
        let fs = init_mock_file_system(os);
        let fs_arc = Arc::new(fs.clone());

        // IMPORTANT: Set the FileSystem BEFORE calling absolute_from
        set_file_system(fs_arc.clone());

        let directory = absolute_from("/a/b/c");
        let _ = fs.ensure_dir(&directory); // Assuming MockFS has ensure_dir or via trait
                                           // MockFileSystem implements FileSystem trait which has ensure_dir

        let options = CompilerOptions::default();
        let host = NgtscCompilerHost::new(fs_arc.clone(), options);

        // fileExists()
        {
            // should return `false` for an existing directory
            assert_eq!(host.file_exists(directory.as_str()), false);
        }

        // readFile()
        {
            // should return `None` (undefined) for an existing directory
            assert!(host.read_file(directory.as_str()).is_none());
        }

        // getSourceFile()
        {
            // should return `None` (undefined) for an existing directory
            assert!(host.get_source_file(directory.as_str()).is_none());
        }

        // useCaseSensitiveFileNames()
        {
            // should return the same as `FileSystem.isCaseSensitive()`
            assert_eq!(host.use_case_sensitive_file_names(), fs.is_case_sensitive());
        }

        // getCanonicalFileName()
        {
            // should return the original filename if FS is case-sensitive or lower case otherwise
            let res = host.get_canonical_file_name("AbCd.ts");
            if fs.is_case_sensitive() {
                assert_eq!(res, "AbCd.ts");
            } else {
                assert_eq!(res, "abcd.ts");
            }
        }
    });
}
