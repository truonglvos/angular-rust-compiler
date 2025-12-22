// Resource Tests
//
// Tests for the resource loading module.

#[cfg(test)]
mod tests {
    use crate::ngtsc::resource::*;
    
    mod in_memory_loader_tests {
        use super::*;
        
        #[test]
        fn should_create_empty_loader() {
            let loader = InMemoryResourceLoader::new();
            assert!(!loader.can_preload("nonexistent.html"));
        }
        
        #[test]
        fn should_add_and_load_resource() {
            let mut loader = InMemoryResourceLoader::new();
            loader.add("template.html", "<div>Hello</div>");
            
            assert!(loader.can_preload("template.html"));
            
            let content = loader.load("template.html").unwrap();
            assert_eq!(content, "<div>Hello</div>");
        }
        
        #[test]
        fn should_return_error_for_missing_resource() {
            let loader = InMemoryResourceLoader::new();
            let result = loader.load("missing.html");
            
            assert!(result.is_err());
            let err = result.unwrap_err();
            assert!(err.message.contains("not found"));
        }
        
        #[test]
        fn should_preload_without_error() {
            let mut loader = InMemoryResourceLoader::new();
            loader.add("style.css", "body { margin: 0; }");
            
            assert!(loader.preload("style.css").is_ok());
        }
    }
    
    mod resource_error_tests {
        use super::*;
        
        #[test]
        fn should_create_not_found_error() {
            let err = ResourceError::not_found("template.html");
            assert!(err.message.contains("not found"));
            assert_eq!(err.url, "template.html");
        }
        
        #[test]
        fn should_create_load_failed_error() {
            let err = ResourceError::load_failed("style.css", "Permission denied");
            assert!(err.message.contains("Failed to load"));
            assert!(err.message.contains("Permission denied"));
        }
    }
}
