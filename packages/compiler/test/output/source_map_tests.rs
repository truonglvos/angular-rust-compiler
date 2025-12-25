use angular_compiler::output::source_map::{to_base64_string, SourceMapGenerator};

#[path = "source_map_util.rs"]
mod source_map_util;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_a_valid_source_map() {
        let mut map = SourceMapGenerator::new(Some("out.js".to_string()));
        map.add_source("a.js".to_string(), None)
            .add_line()
            .add_mapping(0, Some("a.js".to_string()), Some(0), Some(0))
            .unwrap()
            .add_mapping(4, Some("a.js".to_string()), Some(0), Some(6))
            .unwrap()
            .add_mapping(5, Some("a.js".to_string()), Some(0), Some(7))
            .unwrap()
            .add_mapping(8, Some("a.js".to_string()), Some(0), Some(22))
            .unwrap()
            .add_mapping(9, Some("a.js".to_string()), Some(0), Some(23))
            .unwrap()
            .add_mapping(10, Some("a.js".to_string()), Some(0), Some(24))
            .unwrap()
            .add_line()
            .add_mapping(0, Some("a.js".to_string()), Some(1), Some(0))
            .unwrap()
            .add_mapping(4, Some("a.js".to_string()), Some(1), Some(6))
            .unwrap()
            .add_mapping(5, Some("a.js".to_string()), Some(1), Some(7))
            .unwrap()
            .add_mapping(8, Some("a.js".to_string()), Some(1), Some(10))
            .unwrap()
            .add_mapping(9, Some("a.js".to_string()), Some(1), Some(11))
            .unwrap()
            .add_mapping(10, Some("a.js".to_string()), Some(1), Some(12))
            .unwrap()
            .add_line()
            .add_mapping(0, Some("a.js".to_string()), Some(3), Some(0))
            .unwrap()
            .add_mapping(2, Some("a.js".to_string()), Some(3), Some(2))
            .unwrap()
            .add_mapping(3, Some("a.js".to_string()), Some(3), Some(3))
            .unwrap()
            .add_mapping(10, Some("a.js".to_string()), Some(3), Some(10))
            .unwrap()
            .add_mapping(11, Some("a.js".to_string()), Some(3), Some(11))
            .unwrap()
            .add_mapping(21, Some("a.js".to_string()), Some(3), Some(11))
            .unwrap()
            .add_mapping(22, Some("a.js".to_string()), Some(3), Some(12))
            .unwrap()
            .add_line()
            .add_mapping(4, Some("a.js".to_string()), Some(4), Some(4))
            .unwrap()
            .add_mapping(11, Some("a.js".to_string()), Some(4), Some(11))
            .unwrap()
            .add_mapping(12, Some("a.js".to_string()), Some(4), Some(12))
            .unwrap()
            .add_mapping(15, Some("a.js".to_string()), Some(4), Some(15))
            .unwrap()
            .add_mapping(16, Some("a.js".to_string()), Some(4), Some(16))
            .unwrap()
            .add_mapping(21, Some("a.js".to_string()), Some(4), Some(21))
            .unwrap()
            .add_mapping(22, Some("a.js".to_string()), Some(4), Some(22))
            .unwrap()
            .add_mapping(23, Some("a.js".to_string()), Some(4), Some(23))
            .unwrap()
            .add_line()
            .add_mapping(0, Some("a.js".to_string()), Some(5), Some(0))
            .unwrap()
            .add_mapping(1, Some("a.js".to_string()), Some(5), Some(1))
            .unwrap()
            .add_mapping(2, Some("a.js".to_string()), Some(5), Some(2))
            .unwrap()
            .add_mapping(3, Some("a.js".to_string()), Some(5), Some(2))
            .unwrap();

        let json = map.to_json().unwrap();
        // Generated with https://sokra.github.io/source-map-visualization using a TS source map
        // Note: The TS test string is 'AAAA,IAAM,CAAC,GAAe,CAAC,CAAC;AACxB,IAAM,CAAC,GAAG,CAAC,CAAC;AAEZ,EAAE,CAAC,OAAO,CAAC,UAAA,CAAC;IACR,OAAO,CAAC,GAAG,CAAC,KAAK,CAAC,CAAC;AACvB,CAAC,CAAC,CAAA'
        // But verifying exact VLQ encoding in Rust might be brittle if my VLQ impl differs slightly (e.g. variable length integers).
        // However, the standard is fixed.

        // Let's verify our decoder against the expected string first!
        // Wait, I don't have the expected string directly available easily without copy paste.
        // I will copy paste it from the TS test.
        assert_eq!(json.mappings, "AAAA,IAAM,CAAC,GAAe,CAAC,CAAC;AACxB,IAAM,CAAC,GAAG,CAAC,CAAC;AAEZ,EAAE,CAAC,OAAO,CAAC,UAAA,CAAC;IACR,OAAO,CAAC,GAAG,CAAC,KAAK,CAAC,CAAC;AACvB,CAAC,CAAC,CAAA");
    }

    #[test]
    fn should_include_the_files_and_their_contents() {
        let mut map = SourceMapGenerator::new(Some("out.js".to_string()));
        map.add_source("inline.ts".to_string(), Some("inline".to_string()))
            .add_source("inline.ts".to_string(), Some("inline".to_string())) // make sure the sources are dedup
            .add_source("url.ts".to_string(), None)
            .add_line()
            .add_mapping(0, Some("inline.ts".to_string()), Some(0), Some(0))
            .unwrap();

        let json = map.to_json().unwrap();
        assert_eq!(json.file, Some("out.js".to_string()));
        // Note: Sort order might differ if implementation uses HashMap iteration without sorting keys properly in to_json.
        // `source_map.rs` sorts keys: `sorted_keys.sort();`. So it should differ alphabetically.
        // "inline.ts" comes before "url.ts".
        assert_eq!(json.sources, vec!["inline.ts", "url.ts"]);
        // TypeScript test: expect(map.sourcesContent).toEqual(['inline', null]);
        // Rust SourceMap has sources_content as Vec<Option<String>>, not Option<Vec<...>>
        assert_eq!(json.sources_content, vec![Some("inline".to_string()), None]);
    }

    #[test]
    fn should_not_generate_source_maps_when_there_is_no_mapping() {
        let mut smg = SourceMapGenerator::new(Some("out.js".to_string()));
        smg.add_source("inline.ts".to_string(), Some("inline".to_string()))
            .add_line();

        assert!(smg.to_json().is_none());
        assert_eq!(smg.to_js_comment(), "");
    }

    #[test]
    fn should_return_the_b64_encoded_value() {
        let cases = vec![
            ("", ""),
            ("a", "YQ=="),
            ("Foo", "Rm9v"),
            ("Foo1", "Rm9vMQ=="),
            ("Foo12", "Rm9vMTI="),
            ("Foo123", "Rm9vMTIz"),
        ];

        for (src, b64) in cases {
            assert_eq!(to_base64_string(src), b64);
        }
    }

    #[test]
    fn should_error_when_mappings_are_added_out_of_order() {
        let mut gen = SourceMapGenerator::new(Some("out.js".to_string()));
        gen.add_source("in.js".to_string(), None).add_line();
        gen.add_mapping(10, Some("in.js".to_string()), Some(0), Some(0))
            .unwrap();
        let res = gen.add_mapping(0, Some("in.js".to_string()), Some(0), Some(0));
        match res {
            Err(ref msg) => assert_eq!(msg, "Mapping should be added in output order"),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn should_error_when_adding_segments_before_any_line_is_created() {
        let mut gen = SourceMapGenerator::new(Some("out.js".to_string()));
        let res = gen.add_source("in.js".to_string(), None).add_mapping(
            0,
            Some("in.js".to_string()),
            Some(0),
            Some(0),
        );
        match res {
            Err(ref msg) => assert_eq!(msg, "A line must be added before mappings can be added"),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn should_error_when_adding_segments_referencing_unknown_sources() {
        let mut gen = SourceMapGenerator::new(Some("out.js".to_string()));
        let res = gen
            .add_source("in.js".to_string(), None)
            .add_line()
            .add_mapping(0, Some("in_.js".to_string()), Some(0), Some(0));
        match res {
            Err(ref msg) => assert_eq!(msg, "Unknown source file \"in_.js\""),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    // Note: Rust type system prevents "adding segments without column" (usize required) -> skipped

    #[test]
    fn should_error_when_adding_segments_with_a_source_url_but_no_position() {
        let mut gen = SourceMapGenerator::new(Some("out.js".to_string()));
        let res = gen
            .add_source("in.js".to_string(), None)
            .add_line()
            .add_mapping(0, Some("in.js".to_string()), None, None);
        match res {
            Err(ref msg) => assert_eq!(
                msg,
                "The source location must be provided when a source url is provided"
            ),
            Ok(_) => panic!("Expected error but got Ok"),
        }

        let mut gen2 = SourceMapGenerator::new(Some("out.js".to_string()));
        let res2 = gen2
            .add_source("in.js".to_string(), None)
            .add_line()
            .add_mapping(0, Some("in.js".to_string()), Some(0), None);
        match res2 {
            Err(ref msg) => assert_eq!(
                msg,
                "The source location must be provided when a source url is provided"
            ),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }
}
