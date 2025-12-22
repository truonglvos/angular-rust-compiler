// Docs Tests
//
// Tests for the docs extraction module.

#[cfg(test)]
mod tests {
    use super::src::*;
    
    mod entities_tests {
        use super::*;
        
        #[test]
        fn should_create_doc_entry() {
            let entry = DocEntry::new("TestClass", EntryType::Class);
            assert_eq!(entry.name, "TestClass");
            assert_eq!(entry.entry_type, EntryType::Class);
        }
        
        #[test]
        fn should_create_class_entry() {
            let entry = class_extractor::ClassExtractor::extract(
                "MyClass",
                "test.ts",
                10,
            );
            
            assert_eq!(entry.base.name, "MyClass");
            assert!(entry.members.is_empty());
            assert!(entry.constructor_params.is_empty());
        }
        
        #[test]
        fn should_create_function_entry() {
            let params = vec![
                ParameterEntry {
                    name: "value".to_string(),
                    type_annotation: "string".to_string(),
                    optional: false,
                    default_value: None,
                    description: String::new(),
                },
            ];
            
            let entry = function_extractor::FunctionExtractor::extract(
                "myFunction",
                params,
                "void",
            );
            
            assert_eq!(entry.base.name, "myFunction");
            assert_eq!(entry.params.len(), 1);
            assert_eq!(entry.return_type, "void");
        }
        
        #[test]
        fn should_create_enum_entry() {
            let members = vec![
                enum_extractor::EnumExtractor::extract_member("Value1", "0"),
                enum_extractor::EnumExtractor::extract_member("Value2", "1"),
            ];
            
            let entry = enum_extractor::EnumExtractor::extract("MyEnum", members);
            
            assert_eq!(entry.base.name, "MyEnum");
            assert_eq!(entry.members.len(), 2);
        }
    }
    
    mod jsdoc_extractor_tests {
        use super::*;
        
        #[test]
        fn should_parse_description() {
            let comment = "This is a description";
            let (description, tags) = jsdoc_extractor::JsDocExtractor::parse(comment);
            
            assert_eq!(description, "This is a description");
            assert!(tags.is_empty());
        }
        
        #[test]
        fn should_parse_param_tag() {
            let comment = r#"
                Description here
                @param name The name parameter
                @param value The value
            "#;
            
            let (description, tags) = jsdoc_extractor::JsDocExtractor::parse(comment);
            
            assert!(description.contains("Description"));
            assert_eq!(tags.len(), 2);
            assert_eq!(tags[0].name, "param");
            assert!(tags[0].text.contains("name"));
        }
        
        #[test]
        fn should_parse_returns_tag() {
            let comment = r#"
                Gets the value
                @returns The current value
            "#;
            
            let (_, tags) = jsdoc_extractor::JsDocExtractor::parse(comment);
            
            assert!(jsdoc_extractor::JsDocExtractor::has_tag(&tags, "returns"));
            let returns_text = jsdoc_extractor::JsDocExtractor::get_tag(&tags, "returns");
            assert!(returns_text.unwrap().contains("current value"));
        }
        
        #[test]
        fn should_parse_deprecated_tag() {
            let comment = r#"
                Old method
                @deprecated Use newMethod instead
            "#;
            
            let (_, tags) = jsdoc_extractor::JsDocExtractor::parse(comment);
            
            assert!(jsdoc_extractor::JsDocExtractor::has_tag(&tags, "deprecated"));
        }
    }
    
    mod decorator_extractor_tests {
        use super::*;
        
        #[test]
        fn should_identify_component_decorator() {
            assert!(decorator_extractor::DecoratorExtractor::is_angular_decorator("Component"));
            assert_eq!(
                decorator_extractor::DecoratorExtractor::get_decorator_type("Component"),
                Some(decorator_extractor::DecoratorType::Component)
            );
        }
        
        #[test]
        fn should_identify_directive_decorator() {
            assert!(decorator_extractor::DecoratorExtractor::is_angular_decorator("Directive"));
            assert_eq!(
                decorator_extractor::DecoratorExtractor::get_decorator_type("Directive"),
                Some(decorator_extractor::DecoratorType::Directive)
            );
        }
        
        #[test]
        fn should_identify_injectable_decorator() {
            assert!(decorator_extractor::DecoratorExtractor::is_angular_decorator("Injectable"));
        }
        
        #[test]
        fn should_not_identify_custom_decorator() {
            assert!(!decorator_extractor::DecoratorExtractor::is_angular_decorator("CustomDecorator"));
            assert!(decorator_extractor::DecoratorExtractor::get_decorator_type("CustomDecorator").is_none());
        }
    }
    
    mod type_extractor_tests {
        use super::*;
        
        #[test]
        fn should_identify_primitive_types() {
            assert!(type_extractor::TypeExtractor::is_primitive("string"));
            assert!(type_extractor::TypeExtractor::is_primitive("number"));
            assert!(type_extractor::TypeExtractor::is_primitive("boolean"));
            assert!(type_extractor::TypeExtractor::is_primitive("void"));
            assert!(type_extractor::TypeExtractor::is_primitive("any"));
        }
        
        #[test]
        fn should_not_identify_complex_as_primitive() {
            assert!(!type_extractor::TypeExtractor::is_primitive("MyClass"));
            assert!(!type_extractor::TypeExtractor::is_primitive("Observable<string>"));
        }
        
        #[test]
        fn should_identify_array_types() {
            assert!(type_extractor::TypeExtractor::is_array("string[]"));
            assert!(type_extractor::TypeExtractor::is_array("Array<number>"));
            assert!(!type_extractor::TypeExtractor::is_array("string"));
        }
        
        #[test]
        fn should_extract_array_element_type() {
            assert_eq!(
                type_extractor::TypeExtractor::get_array_element_type("string[]"),
                Some("string".to_string())
            );
            assert_eq!(
                type_extractor::TypeExtractor::get_array_element_type("Array<number>"),
                Some("number".to_string())
            );
        }
    }
    
    mod extractor_tests {
        use super::*;
        
        #[test]
        fn should_create_extractor_with_options() {
            let options = ExtractorOptions {
                include_private: false,
                include_internal: false,
                include_patterns: vec!["src/".to_string()],
                exclude_patterns: vec!["test/".to_string()],
            };
            
            let extractor = DocsExtractor::new(options);
            // Just test it doesn't panic
            let _ = extractor;
        }
        
        #[test]
        fn should_extract_from_empty_list() {
            let mut extractor = DocsExtractor::default();
            let result = extractor.extract(&[]);
            
            assert!(result.entries.is_empty());
            assert!(result.diagnostics.is_empty());
        }
        
        #[test]
        fn should_check_internal_entry() {
            let mut entry = DocEntry::new("internalFn", EntryType::Function);
            entry.jsdoc_tags.push(JsDocTag {
                name: "internal".to_string(),
                text: String::new(),
            });
            
            assert!(DocsExtractor::is_internal(&entry));
        }
        
        #[test]
        fn should_check_deprecated_entry() {
            let mut entry = DocEntry::new("oldFn", EntryType::Function);
            entry.deprecated = Some("Use newFn instead".to_string());
            
            assert!(DocsExtractor::is_deprecated(&entry));
        }
    }
}
