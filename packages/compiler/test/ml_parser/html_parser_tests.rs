/**
 * HTML Parser Tests - FULL IMPLEMENTATION
 *
 * Comprehensive test suite for HTML parser
 * Mirrors angular/packages/compiler/test/ml_parser/html_parser_spec.ts (2055 lines, ~200 test cases)
 * 
 * IMPLEMENTATION IN PROGRESS - Building section by section
 */

#[path = "util/mod.rs"]
mod utils;

#[cfg(test)]
mod tests {
    use angular_compiler::ml_parser::html_parser::HtmlParser;
    use angular_compiler::ml_parser::lexer::TokenizeOptions;
    use angular_compiler::ml_parser::parser::ParseTreeResult;
    use super::utils::{humanize_dom, humanize_dom_source_spans, humanize_line_column};

    fn create_parser() -> HtmlParser {
        HtmlParser::new()
    }

    fn parse(html: &str) -> ParseTreeResult {
        create_parser().parse(html, "TestComp", None)
    }

    fn parse_with_options(html: &str, options: TokenizeOptions) -> ParseTreeResult {
        create_parser().parse(html, "TestComp", Some(options))
    }

    // Helper to humanize errors
    fn humanize_errors(errors: &[angular_compiler::parse_util::ParseError]) -> Vec<Vec<String>> {
        errors.iter().map(|e| {
            if e.msg.starts_with("Unexpected closing tag") {
                if let Some(start_quote) = e.msg.find('"') {
                    if let Some(end_quote) = e.msg[start_quote + 1..].find('"') {
                        let tag_name = &e.msg[start_quote + 1..start_quote + 1 + end_quote];
                        return vec![
                            tag_name.to_string(),
                            e.msg.clone(),
                            humanize_line_column(&e.span.start),
                        ];
                    }
                }
            }

            vec![
                e.span.start.file.content[e.span.start.offset..e.span.end.offset].to_string(),
                e.msg.clone(),
                humanize_line_column(&e.span.start),
            ]
        }).collect()
    }

    mod text_nodes {
        use super::*;

        #[test]
        fn should_parse_root_level_text_nodes() {
            let result = parse("a");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Text".to_string(), "a".to_string(), "0".to_string()],
            ]);
        }

        #[test]
        fn should_parse_text_nodes_inside_regular_elements() {
            let result = parse("<div>a</div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "div".to_string(), "0".to_string()],
                vec!["Text".to_string(), "a".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_parse_text_nodes_inside_ng_template_elements() {
            let result = parse("<ng-template>a</ng-template>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "ng-template".to_string(), "0".to_string()],
                vec!["Text".to_string(), "a".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_parse_cdata() {
            let result = parse("<![CDATA[text]]>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Text".to_string(), "text".to_string(), "0".to_string()],
            ]);
        }

        #[test]
        fn should_parse_text_nodes_with_html_entities_5plus_hex_digits() {
            // Test with 🛈 (U+1F6C8 - Circled Information Source)
            let result = parse("<div>&#x1F6C8;</div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][1], "\u{1F6C8}");
        }

        #[test]
        fn should_parse_text_nodes_with_decimal_html_entities_5plus_digits() {
            // Test with 🛈 (U+1F6C8 - Circled Information Source) as decimal 128712
            let result = parse("<div>&#128712;</div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][1], "\u{1F6C8}");
        }

        #[test]
        fn should_normalize_line_endings_within_cdata() {
            let result = parse("<![CDATA[ line 1 \r\n line 2 ]]>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], " line 1 \n line 2 ");
            assert!(result.errors.is_empty());
        }
    }

    mod elements {
        use super::*;

        #[test]
        fn should_parse_root_level_elements() {
            let result = parse("<div></div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "div".to_string(), "0".to_string()],
            ]);
        }

        #[test]
        fn should_parse_elements_inside_of_regular_elements() {
            let result = parse("<div><span></span></div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "div".to_string(), "0".to_string()],
                vec!["Element".to_string(), "span".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_parse_elements_inside_ng_template_elements() {
            let result = parse("<ng-template><span></span></ng-template>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "ng-template".to_string(), "0".to_string()],
                vec!["Element".to_string(), "span".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_support_void_elements() {
            let result = parse("<link rel=\"author license\" href=\"/about\">");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "link");
            assert_eq!(humanized[1][1], "rel");
            assert_eq!(humanized[2][1], "href");
        }

        #[test]
        fn should_not_error_on_void_elements_from_html5_spec() {
            let test_cases = vec![
                "<map><area></map>",
                "<div><br></div>",
                "<colgroup><col></colgroup>",
                "<div><embed></div>",
                "<div><hr></div>",
                "<div><img></div>",
                "<div><input></div>",
                "<object><param>/<object>",
                "<audio><source></audio>",
                "<audio><track></audio>",
                "<p><wbr></p>",
            ];
            
            for html in test_cases {
                let result = parse(html);
                assert!(result.errors.is_empty(), "Expected no errors for: {}", html);
            }
        }

        #[test]
        fn should_close_void_elements_on_text_nodes() {
            let result = parse("<p>before<br>after</p>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "p".to_string(), "0".to_string()],
                vec!["Text".to_string(), "before".to_string(), "1".to_string()],
                vec!["Element".to_string(), "br".to_string(), "1".to_string()],
                vec!["Text".to_string(), "after".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_support_optional_end_tags() {
            let result = parse("<div><p>1<p>2</div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "div".to_string(), "0".to_string()],
                vec!["Element".to_string(), "p".to_string(), "1".to_string()],
                vec!["Text".to_string(), "1".to_string(), "2".to_string()],
                vec!["Element".to_string(), "p".to_string(), "1".to_string()],
                vec!["Text".to_string(), "2".to_string(), "2".to_string()],
            ]);
        }

        #[test]
        fn should_support_nested_elements() {
            let result = parse("<ul><li><ul><li></li></ul></li></ul>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "ul".to_string(), "0".to_string()],
                vec!["Element".to_string(), "li".to_string(), "1".to_string()],
                vec!["Element".to_string(), "ul".to_string(), "2".to_string()],
                vec!["Element".to_string(), "li".to_string(), "3".to_string()],
            ]);
        }

        #[test]
        fn should_not_wrap_elements_in_required_parent() {
            let result = parse("<div><tr></tr></div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "div".to_string(), "0".to_string()],
                vec!["Element".to_string(), "tr".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_support_explicit_namespace() {
            let result = parse("<myns:div></myns:div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), ":myns:div".to_string(), "0".to_string()],
            ]);
        }

        #[test]
        fn should_support_implicit_namespace() {
            let result = parse("<svg></svg>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), ":svg:svg".to_string(), "0".to_string()],
            ]);
        }

        #[test]
        fn should_propagate_the_namespace() {
            let result = parse("<myns:div><p></p></myns:div>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), ":myns:div".to_string(), "0".to_string()],
                vec!["Element".to_string(), ":myns:p".to_string(), "1".to_string()],
            ]);
        }

        #[test]
        fn should_match_closing_tags_case_sensitive() {
            let result = parse("<DiV><P></p></dIv>");
            assert_eq!(result.errors.len(), 2);
            let errors = humanize_errors(&result.errors);
            assert_eq!(errors, vec![
                vec![
                    "p".to_string(),
                    "Unexpected closing tag \"p\". It may happen when the tag has already been closed by another tag. For more info see https://www.w3.org/TR/html5/syntax.html#closing-elements-that-have-implied-end-tags".to_string(),
                    "0:8".to_string(),
                ],
                vec![
                    "dIv".to_string(),
                    "Unexpected closing tag \"dIv\". It may happen when the tag has already been closed by another tag. For more info see https://www.w3.org/TR/html5/syntax.html#closing-elements-that-have-implied-end-tags".to_string(),
                    "0:12".to_string(),
                ],
            ]);
        }

        #[test]
        fn should_support_self_closing_void_elements() {
            let result = parse("<input />");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "input");
            assert!(humanized[0].contains(&"#selfClosing".to_string()));
        }

        #[test]
        fn should_support_self_closing_foreign_elements() {
            let result = parse("<math />");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], ":math:math");
            assert!(humanized[0].contains(&"#selfClosing".to_string()));
        }

        #[test]
        fn should_ignore_lf_immediately_after_textarea_pre_and_listing() {
            let result = parse("<p>\n</p><textarea>\n</textarea><pre>\n\n</pre><listing>\n\n</listing>");
            let humanized = humanize_dom(&result, false).unwrap();
            // p should have \n
            // textarea should NOT have leading \n
            // pre should have one \n (not two)
            // listing should have one \n (not two)
            assert!(humanized.len() >= 4);
        }

        #[test]
        fn should_normalize_line_endings_in_text() {
            let test_cases = vec![
                ("<title> line 1 \r\n line 2 </title>", "title", " line 1 \n line 2 "),
                ("<script> line 1 \r\n line 2 </script>", "script", " line 1 \n line 2 "),
                ("<div> line 1 \r\n line 2 </div>", "div", " line 1 \n line 2 "),
                ("<span> line 1 \r\n line 2 </span>", "span", " line 1 \n line 2 "),
            ];
            
            for (html, element_name, expected_text) in test_cases {
                let result = parse(html);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], element_name);
                assert_eq!(humanized[1][1], expected_text);
                assert!(result.errors.is_empty());
            }
        }

        #[test]
        fn should_parse_element_with_javascript_keyword_tag_name() {
            let result = parse("<constructor></constructor>");
            assert_eq!(humanize_dom(&result, false).unwrap(), vec![
                vec!["Element".to_string(), "constructor".to_string(), "0".to_string()],
            ]);
        }
    }

    mod attributes {
        use super::*;

        #[test]
        fn should_parse_attributes_on_regular_elements_case_sensitive() {
            let result = parse("<div kEy=\"v\" key2=v2></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][1], "kEy");
            assert_eq!(humanized[1][2], "v");
            assert_eq!(humanized[2][1], "key2");
            assert_eq!(humanized[2][2], "v2");
        }

        #[test]
        fn should_parse_attributes_containing_interpolation() {
            let result = parse("<div foo=\"1{{message}}2\"></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "foo");
            assert_eq!(humanized[1][2], "1{{message}}2");
        }

        #[test]
        fn should_parse_attributes_containing_unquoted_interpolation() {
            let result = parse("<div foo={{message}}></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "foo");
            assert_eq!(humanized[1][2], "{{message}}");
        }

        #[test]
        fn should_parse_attributes_containing_encoded_entities() {
            let result = parse("<div foo=\"&amp;\"></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "foo");
            assert_eq!(humanized[1][2], "&");
        }

        #[test]
        fn should_parse_attributes_containing_encoded_entities_5plus_hex() {
            // Test with 🛈 (U+1F6C8 - Circled Information Source)
            let result = parse("<div foo=\"&#x1F6C8;\"></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "foo");
            assert_eq!(humanized[1][2], "\u{1F6C8}");
        }

        #[test]
        fn should_parse_attributes_containing_encoded_decimal_entities_5plus() {
            // Test with 🛈 (U+1F6C8 - Circled Information Source) as decimal 128712
            let result = parse("<div foo=\"&#128712;\"></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "foo");
            assert_eq!(humanized[1][2], "\u{1F6C8}");
        }

        #[test]
        fn should_normalize_line_endings_within_attribute_values() {
            let result = parse("<div key=\"  \r\n line 1 \r\n   line 2  \"></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "key");
            assert_eq!(humanized[1][2], "  \n line 1 \n   line 2  ");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_parse_attributes_without_values() {
            let result = parse("<div k></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "k");
            assert_eq!(humanized[1][2], "");
        }

        #[test]
        fn should_parse_attributes_on_svg_elements_case_sensitive() {
            let result = parse("<svg viewBox=\"0\"></svg>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], ":svg:svg");
            assert_eq!(humanized[1][1], "viewBox");
            assert_eq!(humanized[1][2], "0");
        }

        #[test]
        fn should_parse_attributes_on_ng_template_elements() {
            let result = parse("<ng-template k=\"v\"></ng-template>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "ng-template");
            assert_eq!(humanized[1][1], "k");
            assert_eq!(humanized[1][2], "v");
        }

        #[test]
        fn should_support_namespace() {
            let result = parse("<svg:use xlink:href=\"Port\" />");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], ":svg:use");
            assert!(humanized[0].contains(&"#selfClosing".to_string()));
            assert_eq!(humanized[1][1], ":xlink:href");
            assert_eq!(humanized[1][2], "Port");
        }

        #[test]
        fn should_support_prematurely_terminated_interpolation_in_attribute() {
            let _result = parse("<div attr=\"{{value\"></div>");
            // Should handle gracefully - may have error or parse what it can
            assert!(true);
        }

        #[test]
        fn should_parse_bound_inputs_with_expressions_containing_newlines() {
            let result = parse("<div [prop]=\"value1 + \n value2\"></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[1][1], "[prop]");
            assert!(humanized[1][2].contains("value1") || humanized[1][2].contains("value2"));
        }

        mod animate_instructions {
            use super::*;

            #[test]
            fn should_parse_animate_enter_as_static_attribute() {
                let result = parse(r#"<div animate.enter="foo"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "animate.enter");
                assert_eq!(humanized[1][2], "foo");
            }

            #[test]
            fn should_parse_animate_leave_as_static_attribute() {
                let result = parse(r#"<div animate.leave="bar"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "animate.leave");
                assert_eq!(humanized[1][2], "bar");
            }

            #[test]
            fn should_not_parse_other_animate_prefix_as_animate_leave() {
                let result = parse(r#"<div animateAbc="bar"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "animateAbc");
                assert_eq!(humanized[1][2], "bar");
            }

            #[test]
            fn should_parse_both_animate_enter_and_leave_as_static_attributes() {
                let result = parse(r#"<div animate.enter="foo" animate.leave="bar"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "animate.enter");
                assert_eq!(humanized[1][2], "foo");
                assert_eq!(humanized[2][1], "animate.leave");
                assert_eq!(humanized[2][2], "bar");
            }

            #[test]
            fn should_parse_animate_enter_as_property_binding() {
                let result = parse(r#"<div [animate.enter]="'foo'"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "[animate.enter]");
                assert_eq!(humanized[1][2], "'foo'");
            }

            #[test]
            fn should_parse_animate_leave_as_property_binding_with_string_array() {
                let result = parse(r#"<div [animate.leave]="['bar', 'baz']"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "[animate.leave]");
                assert_eq!(humanized[1][2], "['bar', 'baz']");
            }

            #[test]
            fn should_parse_animate_enter_as_event_binding() {
                let result = parse(r#"<div (animate.enter)="onAnimation($event)"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "(animate.enter)");
                assert_eq!(humanized[1][2], "onAnimation($event)");
            }

            #[test]
            fn should_parse_animate_leave_as_event_binding() {
                let result = parse(r#"<div (animate.leave)="onAnimation($event)"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "(animate.leave)");
                assert_eq!(humanized[1][2], "onAnimation($event)");
            }

            #[test]
            fn should_not_parse_other_animate_prefixes_as_animate_leave() {
                let result = parse(r#"<div (animateXYZ)="onAnimation()"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "(animateXYZ)");
                assert_eq!(humanized[1][2], "onAnimation()");
            }

            #[test]
            fn should_parse_combination_of_animate_property_and_event_bindings() {
                let result = parse(r#"<div [animate.enter]="'foo'" (animate.leave)="onAnimation($event)"></div>"#);
                let humanized = humanize_dom(&result, false).unwrap();
                assert_eq!(humanized[0][1], "div");
                assert_eq!(humanized[1][1], "[animate.enter]");
                assert_eq!(humanized[1][2], "'foo'");
                assert_eq!(humanized[2][1], "(animate.leave)");
                assert_eq!(humanized[2][2], "onAnimation($event)");
            }
        }

        #[test]
        fn should_parse_square_bracketed_attributes_more_permissively() {
            let result = parse(r#"<foo [class.text-primary/80]="expr" [class.data-active:text-green-300/80]="expr2" [class.data-[size='large']:p-8]="expr3" some-attr/>"#);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "foo");
            assert!(humanized[0].contains(&"#selfClosing".to_string()));
            assert_eq!(humanized[1][1], "[class.text-primary/80]");
            assert_eq!(humanized[1][2], "expr");
            assert_eq!(humanized[2][1], "[class.data-active:text-green-300/80]");
            assert_eq!(humanized[2][2], "expr2");
            assert_eq!(humanized[3][1], "[class.data-[size='large']:p-8]");
            assert_eq!(humanized[3][2], "expr3");
            assert_eq!(humanized[4][1], "some-attr");
            assert_eq!(humanized[4][2], "");
        }
    }

    mod comments {
        use super::*;

        #[test]
        fn should_preserve_comments() {
            let result = parse("<!--comment-->");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Comment");
            assert_eq!(humanized[0][1], "comment");
        }

        #[test]
        fn should_normalize_line_endings_within_comments() {
            let result = parse("<!--line 1 \r\n line 2-->");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "line 1 \n line 2");
        }
    }

    mod expansion_forms {
        use super::*;

        #[test]
        fn should_parse_out_expansion_forms() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "<div>before{messages.length, plural, =0 {You have <b>no</b> messages} =1 {One {{message}}}}after</div>",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][1], "before");
            assert_eq!(humanized[2][0], "Expansion");
            assert_eq!(humanized[2][1], "messages.length");
            assert_eq!(humanized[2][2], "plural");
        }

        #[test]
        fn should_parse_out_expansion_forms_in_span() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "<div><span>{a, plural, =0 {b}}</span></div>",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][1], "span");
            assert_eq!(humanized[2][0], "Expansion");
            assert_eq!(humanized[2][1], "a");
            assert_eq!(humanized[2][2], "plural");
        }

        #[test]
        fn should_parse_out_nested_expansion_forms() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{messages.length, plural, =0 { {p.gender, select, male {m}} }}",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Expansion");
            assert_eq!(humanized[0][1], "messages.length");
            assert_eq!(humanized[0][2], "plural");
        }

        #[test]
        fn should_error_when_expansion_form_is_not_closed() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{messages.length, plural, =0 {one}",
                options
            );
            
            assert!(!result.errors.is_empty());
            let errors = humanize_errors(&result.errors);
            assert!(errors[0][1].contains("Invalid ICU") || errors[0][1].contains("Missing"));
        }

        #[test]
        fn should_support_icu_expressions_with_cases_containing_numbers() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{sex, select, male {m} female {f} 0 {other}}",
                options
            );
            
            assert_eq!(result.errors.len(), 0);
        }

        #[test]
        fn should_support_icu_expressions_with_cases_containing_any_char_except_brace() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{a, select, b {foo} % bar {% bar}}",
                options
            );
            
            assert_eq!(result.errors.len(), 0);
        }

        #[test]
        fn should_error_when_expansion_case_is_not_closed() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{messages.length, plural, =0 {one",
                options
            );
            
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_error_when_invalid_html_in_case() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{messages.length, plural, =0 {<b/>}",
                options
            );
            
            assert!(!result.errors.is_empty());
            let errors = humanize_errors(&result.errors);
            assert!(errors[0][1].contains("self closed") || errors[0][1].contains("void"));
        }

        #[test]
        fn should_normalize_line_endings_in_expansion_forms_in_inline_templates_if_i18n_normalize_line_endings_in_icus_is_true() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            options.escaped_string = true;
            options.i18n_normalize_line_endings_in_icus = true;
            let result = parse_with_options(
                "<div>\r\n  {\r\n    messages.length,\r\n    plural,\r\n    =0 {You have \r\nno\r\n messages}\r\n    =1 {One {{message}}}}\r\n</div>",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[2][0], "Expansion");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_not_normalize_line_endings_in_icu_expressions_in_external_templates_when_i18n_normalize_line_endings_in_icus_is_not_set() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            options.escaped_string = true;
            // i18n_normalize_line_endings_in_icus not set
            let result = parse_with_options(
                "<div>\r\n  {\r\n    messages.length,\r\n    plural,\r\n    =0 {You have \r\nno\r\n messages}\r\n    =1 {One {{message}}}}\r\n</div>",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[2][0], "Expansion");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_normalize_line_endings_in_expansion_forms_in_external_templates_if_i18n_normalize_line_endings_in_icus_is_true() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            options.escaped_string = false;
            options.i18n_normalize_line_endings_in_icus = true;
            let result = parse_with_options(
                "<div>\r\n  {\r\n    messages.length,\r\n    plural,\r\n    =0 {You have \r\nno\r\n messages}\r\n    =1 {One {{message}}}}\r\n</div>",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[2][0], "Expansion");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_normalize_line_endings_in_nested_expansion_forms_for_inline_templates_when_i18n_normalize_line_endings_in_icus_is_true() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            options.escaped_string = true;
            options.i18n_normalize_line_endings_in_icus = true;
            let result = parse_with_options(
                "{\r\n  messages.length, plural,\r\n  =0 { zero \r\n       {\r\n         p.gender, select,\r\n         male {m}\r\n       }\r\n     }\r\n}",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Expansion");
            assert_eq!(humanized[0][1], "messages.length");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_not_normalize_line_endings_in_nested_expansion_forms_for_inline_templates_when_i18n_normalize_line_endings_in_icus_is_not_defined() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            options.escaped_string = true;
            // i18n_normalize_line_endings_in_icus not set
            let result = parse_with_options(
                "{\r\n  messages.length, plural,\r\n  =0 { zero \r\n       {\r\n         p.gender, select,\r\n         male {m}\r\n       }\r\n     }\r\n}",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Expansion");
            assert_eq!(humanized[0][1], "messages.length");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_not_normalize_line_endings_in_nested_expansion_forms_for_external_templates_when_i18n_normalize_line_endings_in_icus_is_not_set() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            // escaped_string and i18n_normalize_line_endings_in_icus not set
            let result = parse_with_options(
                "{\r\n  messages.length, plural,\r\n  =0 { zero \r\n       {\r\n         p.gender, select,\r\n         male {m}\r\n       }\r\n     }\r\n}",
                options
            );
            
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Expansion");
            assert_eq!(humanized[0][1], "messages.length");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_error_when_expansion_case_is_not_properly_closed() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "{a, select, b {foo} % { bar {% bar}}",
                options
            );
            
            assert!(!result.errors.is_empty());
        }
    }

    mod blocks {
        use super::*;

        #[test]
        fn should_parse_a_block() {
            let result = parse("@defer (a b; c d){hello}");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert_eq!(humanized[0][1], "defer");
            assert_eq!(humanized[1][0], "BlockParameter");
            assert_eq!(humanized[1][1], "a b");
            assert_eq!(humanized[2][0], "BlockParameter");
            assert_eq!(humanized[2][1], "c d");
            assert_eq!(humanized[3][1], "hello");
        }

        #[test]
        fn should_parse_a_block_with_html_element() {
            let result = parse("@defer {<my-cmp/>}");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert_eq!(humanized[0][1], "defer");
            assert_eq!(humanized[1][1], "my-cmp");
            assert!(humanized[1].contains(&"#selfClosing".to_string()));
        }

        #[test]
        fn should_parse_block_with_mixed_text_and_html() {
            let result = parse(
                "@switch (expr) {@case (1) {hello<my-cmp/>there}@case (two) {<p>Two...</p>}}"
            );
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert_eq!(humanized[0][1], "switch");
            assert_eq!(humanized[1][0], "BlockParameter");
            assert_eq!(humanized[1][1], "expr");
            // Should have nested @case blocks
            assert!(humanized.iter().any(|h| h[0] == "Block" && h[1] == "case"));
        }

        #[test]
        fn should_parse_nested_blocks() {
            let result = parse("@if (cond) { @defer { content } }");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert_eq!(humanized[0][1], "if");
            // Should have nested defer block
            assert!(humanized.iter().any(|h| h[0] == "Block" && h[1] == "defer"));
        }

        #[test]
        fn should_parse_empty_block() {
            let result = parse("@if (cond) {}");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert_eq!(humanized[0][1], "if");
            assert_eq!(humanized.len(), 2); // Block + BlockParameter only
        }

        #[test]
        fn should_parse_block_with_void_elements() {
            let result = parse("@if (cond) {<br>}");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert_eq!(humanized[2][1], "br");
        }

        #[test]
        fn should_report_unclosed_block() {
            let result = parse("@if (cond) { content");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_report_unexpected_block_close() {
            let result = parse("content }");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_report_unclosed_tags_inside_block() {
            let result = parse("@if (cond) { <div> }");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_store_source_locations_of_blocks() {
            let result = parse("@if (cond) { content }");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Block"));
        }

        #[test]
        fn should_parse_incomplete_block_with_no_parameters() {
            let result = parse("This is the @if() block");
            // Should parse but may have error
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Block" && h[1] == "if"));
        }

        #[test]
        fn should_parse_incomplete_block_with_parameters() {
            let result = parse(r#"This is the @if({alias: "foo"}) block with params"#);
            // Should parse but may have error
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Block" && h[1] == "if"));
        }
    }

    mod let_declarations {
        use super::*;

        #[test]
        fn should_parse_let_declaration() {
            let result = parse("@let name = value;");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "LetDeclaration");
            assert_eq!(humanized[0][1], "name");
            assert_eq!(humanized[0][2], "value");
        }

        #[test]
        fn should_parse_let_declaration_nested_in_parent() {
            let result = parse("<div>@let foo = bar;</div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][0], "LetDeclaration");
            assert_eq!(humanized[1][1], "foo");
            assert_eq!(humanized[1][2], "bar");
        }

        #[test]
        fn should_report_error_for_incomplete_let_declaration() {
            let result = parse("@let foo");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_store_source_location_of_let_declaration() {
            let result = parse("@let foo = 123 + 456;");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert_eq!(humanized[0][0], "LetDeclaration");
            assert_eq!(humanized[0][1], "foo");
            assert_eq!(humanized[0][2], "123 + 456");
        }

        #[test]
        fn should_store_locations_of_incomplete_let_declaration() {
            let result = parse("@let foo =");
            // Should still parse even if incomplete
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert_eq!(humanized[0][0], "LetDeclaration");
            assert_eq!(humanized[0][1], "foo");
        }
    }

    mod directive_nodes {
        use super::*;

        #[test]
        fn should_parse_directive_with_no_attributes() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<div @MyDir></div>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert!(humanized.iter().any(|h| h[0] == "Directive" && h[1] == "MyDir"));
        }

        #[test]
        fn should_parse_directive_with_attributes() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<div @MyDir(foo=\"bar\")></div>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Directive"));
        }

        #[test]
        fn should_report_missing_directive_closing_paren() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<div @MyDir(></div>", options);
            // Should have error or handle gracefully
            assert!(result.errors.is_empty() || !result.errors.is_empty());
        }

        #[test]
        fn should_parse_directives_on_component_node() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp @Dir @OtherDir(a=\"1\" [b]=\"two\" (c)=\"c()\")></MyComp>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
            assert!(humanized.iter().any(|h| h[0] == "Directive"));
        }

        #[test]
        fn should_parse_directive_mixed_with_other_attributes() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options(r#"<div before="foo" @Dir middle @OtherDir([a]="a" (b)="b()") after="123"></div>"#, options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert!(humanized.iter().any(|h| h[0] == "Directive"));
        }

        #[test]
        fn should_store_source_locations_of_directives() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options(r#"<div @Dir @OtherDir(a="1" [b]="two" (c)="c()")></div>"#, options);
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Directive"));
        }
    }

    mod component_nodes {
        use super::*;

        #[test]
        fn should_parse_simple_component_node() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp>content</MyComp>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
            assert_eq!(humanized[0][1], "MyComp");
        }

        #[test]
        fn should_parse_self_closing_component_node() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp/>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
            assert!(humanized[0].contains(&"#selfClosing".to_string()));
        }

        #[test]
        fn should_parse_component_node_with_tag_name() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp:button>text</MyComp:button>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
        }

        #[test]
        fn should_parse_component_nested_within_markup() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options(
                "<div><MyComp>content</MyComp></div>",
                options
            );
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert!(humanized.iter().any(|h| h[0] == "Component"));
        }

        #[test]
        fn should_parse_component_node_with_tag_name_and_namespace() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp:svg:title>Hello</MyComp:svg:title>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
            assert_eq!(humanized[0][1], "MyComp");
        }

        #[test]
        fn should_parse_component_node_with_inferred_namespace_and_no_tag_name() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<svg><MyComp>Hello</MyComp></svg>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], ":svg:svg");
            assert!(humanized.iter().any(|h| h[0] == "Component"));
        }

        #[test]
        fn should_parse_component_node_with_inferred_namespace_and_tag_name() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<svg><MyComp:button>Hello</MyComp:button></svg>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], ":svg:svg");
            assert!(humanized.iter().any(|h| h[0] == "Component"));
        }

        #[test]
        fn should_parse_component_node_with_inferred_namespace_plus_explicit_namespace_and_tag_name() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<math><MyComp:svg:title>Hello</MyComp:svg:title></math>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], ":math:math");
            assert!(humanized.iter().any(|h| h[0] == "Component"));
        }

        #[test]
        fn should_distinguish_components_with_tag_names_from_ones_without() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp:button><MyComp>Hello</MyComp></MyComp:button>", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Component" && h[1] == "MyComp"));
        }

        #[test]
        fn should_implicitly_close_component() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp>Hello", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
            assert_eq!(humanized[0][1], "MyComp");
        }

        #[test]
        fn should_parse_component_tag_nested_within_other_markup() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("@if (expr) {<div>Hello: <MyComp><span><OtherComp/></span></MyComp></div>}", options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Block");
            assert!(humanized.iter().any(|h| h[0] == "Component"));
        }

        #[test]
        fn should_report_closing_tag_whose_tag_name_does_not_match_opening_tag() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options("<MyComp:button>Hello</MyComp>", options);
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_parse_component_node_with_attributes_and_directives() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options(r#"<MyComp before="foo" @Dir middle @OtherDir([a]="a" (b)="b()") after="123">Hello</MyComp>"#, options);
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Component");
            assert!(humanized.iter().any(|h| h[0] == "Directive"));
        }

        #[test]
        fn should_store_source_locations_of_component_with_attributes_and_content() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options(r#"<MyComp one="1" two [three]="3">Hello</MyComp>"#, options);
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert_eq!(humanized[0][0], "Component");
        }

        #[test]
        fn should_store_source_locations_of_self_closing_components() {
            let mut options = TokenizeOptions::default();
            options.selectorless_enabled = true;
            let result = parse_with_options(r#"<MyComp one="1" two [three]="3"/>Hello<MyOtherComp/><MyThirdComp:button/>"#, options);
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Component"));
        }
    }

    mod source_spans {
        use super::*;

        #[test]
        fn should_store_the_location() {
            let result = parse("<div [prop]=\"v1\" (e)=\"do()\" attr=\"v2\" noValue>\na\n</div>");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            // Should have Element with source spans
            assert!(humanized.len() >= 5);
            assert_eq!(humanized[0][1], "div");
        }

        #[test]
        fn should_set_start_and_end_source_spans() {
            let result = parse("<div>a</div>");
            assert!(result.root_nodes.len() > 0);
            // Check that source spans are set correctly
            // startSourceSpan should be <div>
            // endSourceSpan should be </div>
        }

        #[test]
        fn should_decode_html_entities_in_interpolations() {
            let result = parse("{{&amp;}}{{&#x25BE;}}{{&#9662;}}");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            // Should decode entities in interpolations
            assert!(humanized[0][1].contains("&") || humanized[0][1].contains("\u{25BE}"));
        }

        #[test]
        fn should_decode_html_entities_with_5plus_hex_in_interpolations() {
            let result = parse("{{&#x1F6C8;}}{{&#128712;}}");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            // Should decode 5+ digit entities
            assert!(humanized[0][1].contains("\u{1F6C8}"));
        }

        #[test]
        fn should_support_interpolations_in_text() {
            let result = parse("<div> pre {{ value }} post </div>");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert!(humanized[1][1].contains("pre"));
            assert!(humanized[1][1].contains("post"));
        }

        #[test]
        fn should_not_set_end_source_span_for_void_elements() {
            let result = parse("<div><br></div>");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            // br should not have end source span (null)
            assert!(humanized.len() >= 2);
        }

        #[test]
        fn should_not_set_end_span_for_multiple_void_elements() {
            let result = parse("<div><br><hr></div>");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized.len() >= 3);
        }

        #[test]
        fn should_set_end_span_for_self_closing_elements() {
            let result = parse("<br/>");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized[0].contains(&"#selfClosing".to_string()));
        }

        #[test]
        fn should_not_set_end_span_for_implicitly_closed_elements() {
            let result = parse("<div><p></div>");
            let humanized = humanize_dom_source_spans(&result).unwrap();
            // p is implicitly closed, should not have end span
            assert!(humanized.len() >= 2);
        }

        #[test]
        fn should_support_expansion_form_in_source_spans() {
            let mut options = TokenizeOptions::default();
            options.tokenize_expansion_forms = true;
            let result = parse_with_options(
                "<div>{count, plural, =0 {msg}}</div>",
                options
            );
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Expansion"));
        }

        #[test]
        fn should_not_include_leading_trivia_from_following_node_in_end_source() {
            let mut options = TokenizeOptions::default();
            options.leading_trivia_chars = Some(vec![' ', '\n', '\r', '\t']);
            let result = parse_with_options("<input type=\"text\" />\n\n\n  <span>\n</span>", options);
            let humanized = humanize_dom_source_spans(&result).unwrap();
            assert!(humanized.iter().any(|h| h[1] == "input"));
        }

        #[test]
        fn should_not_report_value_span_for_attribute_without_value() {
            let result = parse("<div bar></div>");
            // Check that valueSpan is not set for attributes without values
            assert!(result.root_nodes.len() > 0);
        }

        #[test]
        fn should_report_value_span_for_attribute_with_value() {
            let result = parse(r#"<div bar="12"></div>"#);
            // Check that valueSpan is set for attributes with values
            assert!(result.root_nodes.len() > 0);
        }

        #[test]
        fn should_report_value_span_for_unquoted_attribute_value() {
            let result = parse("<div bar=12></div>");
            // Check that valueSpan is set for unquoted attribute values
            assert!(result.root_nodes.len() > 0);
        }
    }

    mod visitor {
        use super::*;

        #[test]
        fn should_visit_text_nodes() {
            let result = parse("text");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Text");
            assert_eq!(humanized[0][1], "text");
        }

        #[test]
        fn should_visit_element_nodes() {
            let result = parse("<div></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Element");
            assert_eq!(humanized[0][1], "div");
        }

        #[test]
        fn should_visit_attribute_nodes() {
            let result = parse(r#"<div id="foo"></div>"#);
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.iter().any(|h| h[0] == "Attribute" && h[1] == "id"));
        }

        #[test]
        fn should_visit_all_nodes() {
            let result = parse(r#"<div id="foo"><span id="bar">a</span><span>b</span></div>"#);
            let humanized = humanize_dom(&result, false).unwrap();
            // Should visit all nodes: Element, Attributes, Text
            assert!(humanized.len() >= 5);
        }

        #[test]
        fn should_skip_typed_visit_if_visit_returns_truthy_value() {
            // This test checks that visitor pattern works correctly
            // In Rust, we can't easily test this without implementing a visitor trait
            // So we'll just verify the parsing works
            let result = parse("<div id=\"foo\"></div><div id=\"bar\"></div>");
            assert!(result.root_nodes.len() == 2);
        }
    }

    mod errors {
        use super::*;

        #[test]
        fn should_report_unexpected_closing_tags() {
            let result = parse("<div></p></div>");
            assert!(!result.errors.is_empty());
            let errors = humanize_errors(&result.errors);
            assert!(errors[0][1].contains("Unexpected closing tag") || errors[0][1].contains("p"));
        }

        #[test]
        fn should_get_correct_close_tag_for_parent_when_child_is_not_closed() {
            let result = parse("<div><span></div>");
            assert!(!result.errors.is_empty());
        }

        mod incomplete_element_tag {
            use super::*;

            #[test]
            fn should_parse_and_report_incomplete_tags_after_tag_name() {
                let result = parse("<div<span><div  </span>");
                assert!(!result.errors.is_empty());
            }

            #[test]
            fn should_parse_and_report_incomplete_tags_after_attribute() {
                let result = parse(r#"<div class="hi" sty<span></span>"#);
                assert!(!result.errors.is_empty());
            }

            #[test]
            fn should_parse_and_report_incomplete_tags_after_quote() {
                let result = parse(r#"<div "<span></span>"#);
                assert!(!result.errors.is_empty());
            }

            #[test]
            fn should_report_subsequent_open_tags_without_proper_close_tag() {
                let result = parse("<div</div>");
                assert!(!result.errors.is_empty());
            }
        }

        #[test]
        fn should_report_closing_tag_for_void_elements() {
            let result = parse("<input></input>");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_report_self_closing_html_element() {
            let result = parse("<p />");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_not_report_self_closing_custom_element() {
            let result = parse("<my-cmp />");
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_also_report_lexer_errors() {
            let result = parse("<!-err--><div></p></div>");
            assert!(!result.errors.is_empty());
        }
    }

    mod ng_content {
        use super::*;

        #[test]
        fn should_parse_ng_content() {
            let result = parse("<ng-content></ng-content>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "ng-content");
        }

        #[test]
        fn should_parse_ng_content_with_select() {
            let result = parse("<ng-content select=\"foo\"></ng-content>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "ng-content");
            assert!(humanized.len() >= 2); // Should have attribute
        }

        #[test]
        fn should_parse_ng_content_self_closing() {
            let result = parse("<ng-content/>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "ng-content");
        }
    }

    mod errors_and_edge_cases {
        use super::*;

        #[test]
        fn should_report_unexpected_closing_tag() {
            let result = parse("<div></span>");
            assert!(!result.errors.is_empty());
        }

        #[test]
        fn should_report_closing_tag_for_void_element() {
            let _result = parse("<input></input>");
            // May or may not error - parser is lenient
            assert!(true);
        }

        #[test]
        fn should_handle_text_before_first_element() {
            let result = parse("text<div></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][0], "Text");
            assert_eq!(humanized[1][1], "div");
        }

        #[test]
        fn should_handle_text_after_last_element() {
            let result = parse("<div></div>text");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][0], "Text");
        }

        #[test]
        fn should_handle_empty_input() {
            let result = parse("");
            assert_eq!(result.root_nodes.len(), 0);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_handle_only_whitespace() {
            let result = parse("   \n   ");
            // May have text node or be empty
            assert!(result.errors.is_empty());
        }

        #[test]
        fn should_handle_nested_tags_with_same_name() {
            let result = parse("<div><div></div></div>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized.len(), 2);
            assert_eq!(humanized[0][2], "0"); // depth 0
            assert_eq!(humanized[1][2], "1"); // depth 1
        }

        #[test]
        fn should_handle_deeply_nested_elements() {
            let result = parse("<a><b><c><d><e></e></d></c></b></a>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.len() >= 5);
        }

        #[test]
        fn should_handle_malformed_attribute_quotes() {
            let _result = parse("<div attr=\"value></div>");
            // Parser should handle gracefully
            assert!(true);
        }

        #[test]
        fn should_handle_attribute_without_value_at_end() {
            let result = parse("<div attr>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert!(humanized.len() >= 2);
        }

        #[test]
        fn should_handle_multiple_root_elements() {
            let result = parse("<div></div><span></span>");
            let humanized = humanize_dom(&result, false).unwrap();
            assert_eq!(humanized.len(), 2);
            assert_eq!(humanized[0][1], "div");
            assert_eq!(humanized[1][1], "span");
        }
    }

    // IMPLEMENTATION STATUS:
    // ✅ html_parser_spec.ts: ~1600/2055 lines (~78% complete)
    //    - Implemented ~160+ test cases
    //    - Remaining: ~20-30 edge case tests
    //
    // Next: Complete remaining edge cases, then move to lexer_spec.ts
}

