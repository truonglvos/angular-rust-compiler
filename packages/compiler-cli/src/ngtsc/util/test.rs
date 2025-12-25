// Util Tests
//
// Tests for the utility module.

#[cfg(test)]
mod tests {
    use crate::ngtsc::util::*;

    mod identifier_validation_tests {
        use super::*;

        #[test]
        fn should_validate_simple_identifier() {
            assert!(is_valid_identifier("myVar"));
            assert!(is_valid_identifier("_private"));
            assert!(is_valid_identifier("$jquery"));
            assert!(is_valid_identifier("camelCase"));
        }

        #[test]
        fn should_reject_invalid_identifier() {
            assert!(!is_valid_identifier("123abc"));
            assert!(!is_valid_identifier("my-var"));
            assert!(!is_valid_identifier("my.var"));
            assert!(!is_valid_identifier(""));
        }
    }

    mod case_conversion_tests {
        use super::*;

        #[test]
        fn should_convert_to_camel_case() {
            assert_eq!(to_camel_case("my-component"), "myComponent");
            assert_eq!(to_camel_case("hello_world"), "helloWorld");
            assert_eq!(to_camel_case("already"), "already");
        }

        #[test]
        fn should_convert_to_kebab_case() {
            assert_eq!(to_kebab_case("myComponent"), "my-component");
            assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
            assert_eq!(to_kebab_case("already-kebab"), "already-kebab");
        }

        #[test]
        fn should_convert_to_pascal_case() {
            assert_eq!(to_pascal_case("my-component"), "MyComponent");
            assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        }
    }

    mod path_utilities_tests {
        use super::*;

        #[test]
        fn should_get_basename() {
            assert_eq!(get_basename("/path/to/file.ts"), "file.ts");
            assert_eq!(get_basename("file.ts"), "file.ts");
        }

        #[test]
        fn should_get_dirname() {
            assert_eq!(get_dirname("/path/to/file.ts"), "/path/to");
            assert_eq!(get_dirname("file.ts"), ".");
        }

        #[test]
        fn should_get_extension() {
            assert_eq!(get_extension("file.ts"), Some("ts"));
            assert_eq!(get_extension("file.d.ts"), Some("ts"));
            assert_eq!(get_extension("file"), None);
        }

        #[test]
        fn should_remove_extension() {
            assert_eq!(remove_extension("file.ts"), "file");
            assert_eq!(remove_extension("path/to/file.ts"), "path/to/file");
        }
    }
}
