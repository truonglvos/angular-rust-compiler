/**
 * HTML Whitespaces Tests
 *
 * Test suite for whitespace removal
 * Mirrors angular/packages/compiler/test/ml_parser/html_whitespaces_spec.ts (196 lines)
 * COMPLETE IMPLEMENTATION - All 14 test cases from TypeScript
 */

#[path = "util/mod.rs"]
mod utils;

#[cfg(test)]
mod tests {
    use super::utils::humanize_dom;
    use angular_compiler::ml_parser::html_parser::HtmlParser;
    use angular_compiler::ml_parser::html_whitespaces::{
        remove_whitespaces, PRESERVE_WS_ATTR_NAME,
    };
    use angular_compiler::ml_parser::lexer::TokenizeOptions;

    fn parse_and_remove_ws(template: &str, options: Option<TokenizeOptions>) -> Vec<Vec<String>> {
        let parser = HtmlParser::new();
        let parse_result = parser.parse(template, "TestComp", options);
        let result = remove_whitespaces(parse_result, true);
        humanize_dom(&result, false).expect("Should parse without errors")
    }

    #[test]
    fn should_remove_blank_text_nodes() {
        assert_eq!(parse_and_remove_ws(" ", None), Vec::<Vec<String>>::new());
        assert_eq!(parse_and_remove_ws("\n", None), Vec::<Vec<String>>::new());
        assert_eq!(parse_and_remove_ws("\t", None), Vec::<Vec<String>>::new());
        assert_eq!(
            parse_and_remove_ws("    \t    \n ", None),
            Vec::<Vec<String>>::new()
        );
    }

    #[test]
    fn should_remove_whitespaces_between_elements() {
        let result = parse_and_remove_ws("<br>  <br>\t<br>\n<br>", None);
        assert_eq!(
            result,
            vec![
                vec!["Element".to_string(), "br".to_string(), "0".to_string()],
                vec!["Element".to_string(), "br".to_string(), "0".to_string()],
                vec!["Element".to_string(), "br".to_string(), "0".to_string()],
                vec!["Element".to_string(), "br".to_string(), "0".to_string()],
            ]
        );
    }

    #[test]
    fn should_remove_whitespaces_from_child_text_nodes() {
        let result = parse_and_remove_ws("<div><span> </span></div>", None);
        assert_eq!(
            result,
            vec![
                vec!["Element".to_string(), "div".to_string(), "0".to_string()],
                vec!["Element".to_string(), "span".to_string(), "1".to_string()],
            ]
        );
    }

    #[test]
    fn should_remove_whitespaces_from_beginning_and_end_of_template() {
        let result = parse_and_remove_ws(" <br>\t", None);
        assert_eq!(
            result,
            vec![vec![
                "Element".to_string(),
                "br".to_string(),
                "0".to_string()
            ],]
        );
    }

    #[test]
    fn should_convert_ngsp_to_space_and_preserve_it() {
        let template = format!("<div><span>foo</span>&ngsp;<span>bar</span></div>");
        let result = parse_and_remove_ws(&template, None);

        // Expected structure:
        // [Element, 'div', 0],
        // [Element, 'span', 1],
        // [Text, 'foo', 2, ['foo']],
        // [Text, ' ', 1, [''], [NGSP_UNICODE, '&ngsp;'], ['']],
        // [Element, 'span', 1],
        // [Text, 'bar', 2, ['bar']],
        assert_eq!(result.len(), 6);
        assert_eq!(result[0][1], "div");
        assert_eq!(result[1][1], "span");
        assert_eq!(result[2][1], "foo");
        assert_eq!(result[3][1], " "); // Space from &ngsp;
        assert_eq!(result[4][1], "span");
        assert_eq!(result[5][1], "bar");
    }

    #[test]
    fn should_replace_multiple_whitespaces_with_one_space() {
        let result1 = parse_and_remove_ws("\n\n\nfoo\t\t\t", None);
        assert_eq!(
            result1,
            vec![vec![
                "Text".to_string(),
                " foo ".to_string(),
                "0".to_string()
            ],]
        );

        let result2 = parse_and_remove_ws("   \n foo  \t ", None);
        assert_eq!(
            result2,
            vec![vec![
                "Text".to_string(),
                " foo ".to_string(),
                "0".to_string()
            ],]
        );
    }

    #[test]
    fn should_remove_whitespace_inside_blocks() {
        let markup = "@if (cond) {<br>  <br>\t<br>\n<br>}";
        let result = parse_and_remove_ws(markup, None);

        assert_eq!(
            result,
            vec![
                vec!["Block".to_string(), "if".to_string(), "0".to_string()],
                vec!["BlockParameter".to_string(), "cond".to_string()],
                vec!["Element".to_string(), "br".to_string(), "1".to_string()],
                vec!["Element".to_string(), "br".to_string(), "1".to_string()],
                vec!["Element".to_string(), "br".to_string(), "1".to_string()],
                vec!["Element".to_string(), "br".to_string(), "1".to_string()],
            ]
        );
    }

    #[test]
    fn should_not_replace_nbsp() {
        let result = parse_and_remove_ws("&nbsp;", None);
        assert_eq!(result[0][1], "\u{00A0}");
    }

    #[test]
    fn should_not_replace_sequences_of_nbsp() {
        let result = parse_and_remove_ws("&nbsp;&nbsp;foo&nbsp;&nbsp;", None);
        // Should have text node with nbsp characters preserved
        assert_eq!(result.len(), 1);
        assert_eq!(result[0][0], "Text");
        assert_eq!(result[0][1], "\u{00A0}\u{00A0}foo\u{00A0}\u{00A0}");
    }

    #[test]
    fn should_not_replace_single_tab_and_newline_with_spaces() {
        let result1 = parse_and_remove_ws("\nfoo", None);
        assert_eq!(
            result1,
            vec![vec![
                "Text".to_string(),
                "\nfoo".to_string(),
                "0".to_string()
            ],]
        );

        let result2 = parse_and_remove_ws("\tfoo", None);
        assert_eq!(
            result2,
            vec![vec![
                "Text".to_string(),
                "\tfoo".to_string(),
                "0".to_string()
            ],]
        );
    }

    #[test]
    fn should_preserve_single_whitespaces_between_interpolations() {
        let result1 = parse_and_remove_ws("{{fooExp}} {{barExp}}", None);
        assert_eq!(result1[0][1], "{{fooExp}} {{barExp}}");

        let result2 = parse_and_remove_ws("{{fooExp}}\t{{barExp}}", None);
        assert_eq!(result2[0][1], "{{fooExp}}\t{{barExp}}");

        let result3 = parse_and_remove_ws("{{fooExp}}\n{{barExp}}", None);
        assert_eq!(result3[0][1], "{{fooExp}}\n{{barExp}}");
    }

    #[test]
    fn should_preserve_whitespaces_around_interpolations() {
        let result = parse_and_remove_ws(" {{exp}} ", None);
        assert_eq!(result[0][1], " {{exp}} ");
    }

    #[test]
    fn should_preserve_whitespaces_around_icu_expansions() {
        let mut options = TokenizeOptions::default();
        options.tokenize_expansion_forms = true;
        let result = parse_and_remove_ws("<span> {a, b, =4 {c}} </span>", Some(options));

        // Expected:
        // [Element, 'span', 0],
        // [Text, ' ', 1],
        // [Expansion, 'a', 'b', 1],
        // [ExpansionCase, '=4', 2],
        // [Text, ' ', 1],
        assert_eq!(result.len(), 5);
        assert_eq!(result[0][1], "span");
        assert_eq!(result[1][1], " ");
        assert_eq!(result[2][0], "Expansion");
        assert_eq!(result[4][1], " ");
    }

    #[test]
    fn should_preserve_whitespaces_inside_pre_elements() {
        let result = parse_and_remove_ws(
            "<pre><strong>foo</strong>\n<strong>bar</strong></pre>",
            None,
        );

        // Expected:
        // [Element, 'pre', 0],
        // [Element, 'strong', 1],
        // [Text, 'foo', 2],
        // [Text, '\n', 1],
        // [Element, 'strong', 1],
        // [Text, 'bar', 2],
        assert_eq!(result.len(), 6);
        assert_eq!(result[0][1], "pre");
        assert_eq!(result[3][1], "\n"); // Preserved newline
    }

    #[test]
    fn should_skip_whitespace_trimming_in_textarea() {
        let result = parse_and_remove_ws("<textarea>foo\n\n  bar</textarea>", None);

        assert_eq!(
            result,
            vec![
                vec![
                    "Element".to_string(),
                    "textarea".to_string(),
                    "0".to_string()
                ],
                vec![
                    "Text".to_string(),
                    "foo\n\n  bar".to_string(),
                    "1".to_string()
                ],
            ]
        );
    }

    #[test]
    fn should_preserve_whitespaces_inside_elements_with_preserve_attr() {
        let template = format!("<div {}><img> <img></div>", PRESERVE_WS_ATTR_NAME);
        let result = parse_and_remove_ws(&template, None);

        // Expected:
        // [Element, 'div', 0],
        // [Element, 'img', 1],
        // [Text, ' ', 1],
        // [Element, 'img', 1],
        assert_eq!(result.len(), 4);
        assert_eq!(result[0][1], "div");
        assert_eq!(result[1][1], "img");
        assert_eq!(result[2][1], " "); // Preserved space
        assert_eq!(result[3][1], "img");
    }
}
