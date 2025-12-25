// Entry Point Tests
//
// Tests for the entry point module.

#[cfg(test)]
mod tests {
    use crate::ngtsc::entry_point::*;

    mod entry_point_tests {
        use super::*;

        #[test]
        fn should_create_entry_point() {
            let entry = NgCompilerEntryPoint::new("/project");
            assert_eq!(entry.base_dir, "/project");
            assert!(entry.root_files.is_empty());
        }

        #[test]
        fn should_add_root_file() {
            let mut entry = NgCompilerEntryPoint::new("/project");
            entry.add_root_file("src/main.ts");
            entry.add_root_file("src/app.module.ts");

            assert_eq!(entry.get_root_files().len(), 2);
        }

        #[test]
        fn should_exclude_files() {
            let mut entry = NgCompilerEntryPoint::new("/project");
            entry.exclude("**/*.spec.ts");

            assert!(entry.is_excluded("**/*.spec.ts"));
            assert!(!entry.is_excluded("src/main.ts"));
        }
    }

    mod flat_module_generator_tests {
        use super::*;

        #[test]
        fn should_create_generator() {
            let gen = FlatModuleEntryPointGenerator::new("public-api", "my-lib");
            assert_eq!(gen.output_name(), "public-api");
            assert_eq!(gen.module_name(), "my-lib");
        }

        #[test]
        fn should_generate_flat_module() {
            let gen = FlatModuleEntryPointGenerator::new("index", "my-lib");

            let exports = vec![FlatModuleExport {
                symbols: vec!["MyComponent".to_string()],
                from: "./component".to_string(),
            }];

            let output = gen.generate(&exports);

            assert!(output.contains("MyComponent"));
        }
    }
}
