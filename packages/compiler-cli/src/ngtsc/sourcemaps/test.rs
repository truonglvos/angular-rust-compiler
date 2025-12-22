// Sourcemaps Tests
//
// Tests for the sourcemaps module.

#[cfg(test)]
mod tests {
    use crate::ngtsc::sourcemaps::*;
    
    mod vlq_encoding_tests {
        use super::*;
        
        #[test]
        fn should_encode_zero() {
            let encoded = encode_vlq(0);
            assert_eq!(encoded, "A");
        }
        
        #[test]
        fn should_encode_positive_numbers() {
            assert_eq!(encode_vlq(1), "C");
        }
    }
    
    mod source_map_tests {
        use super::*;
        
        #[test]
        fn should_create_source_map() {
            let map = SourceMap {
                version: 3,
                file: "output.js".to_string(),
                source_root: None,
                sources: vec!["input.ts".to_string()],
                sources_content: Some(vec![Some("const x = 1;".to_string())]),
                names: vec![],
                mappings: "AAAA".to_string(),
            };
            
            assert_eq!(map.version, 3);
            assert_eq!(map.sources.len(), 1);
        }
    }
    
    mod source_map_builder_tests {
        use super::*;
        
        #[test]
        fn should_create_builder() {
            let builder = SourceMapBuilder::new("output.js".to_string());
            assert!(builder.version() == 3);
        }
        
        #[test]
        fn should_add_source() {
            let mut builder = SourceMapBuilder::new("output.js".to_string());
            builder.add_source("input.ts", Some("code"));
            
            let map = builder.build();
            assert!(map.sources.contains(&"input.ts".to_string()));
        }
        
        #[test]
        fn should_add_name() {
            let mut builder = SourceMapBuilder::new("output.js".to_string());
            let idx = builder.add_name("myFunction");
            
            assert_eq!(idx, 0);
        }
    }
}
