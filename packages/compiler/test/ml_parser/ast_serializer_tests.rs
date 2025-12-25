/**
 * AST Serializer Tests
 *
 * Tests for HTML AST serialization
 * Mirrors angular/packages/compiler/test/ml_parser/ast_serializer_spec.ts (61 lines)
 * COMPLETE IMPLEMENTATION - All 6 test cases from TypeScript
 */

#[path = "util/mod.rs"]
mod utils;

#[cfg(test)]
mod tests {
    use super::utils::serialize_nodes;
    use angular_compiler::ml_parser::html_parser::HtmlParser;
    use angular_compiler::ml_parser::lexer::TokenizeOptions;

    #[test]
    fn should_support_element() {
        let html = "<p></p>";
        let parser = HtmlParser::new();
        let ast = parser.parse(html, "url", None);

        assert_eq!(serialize_nodes(&ast.root_nodes), vec![html]);
    }

    #[test]
    fn should_support_attributes() {
        let html = "<p k=\"value\"></p>";
        let parser = HtmlParser::new();
        let ast = parser.parse(html, "url", None);

        assert_eq!(serialize_nodes(&ast.root_nodes), vec![html]);
    }

    #[test]
    fn should_support_text() {
        let html = "some text";
        let parser = HtmlParser::new();
        let ast = parser.parse(html, "url", None);

        assert_eq!(serialize_nodes(&ast.root_nodes), vec![html]);
    }

    #[test]
    fn should_support_expansion() {
        let html = "{number, plural, =0 {none} =1 {one} other {many}}";
        let parser = HtmlParser::new();
        let mut options = TokenizeOptions::default();
        options.tokenize_expansion_forms = true;
        let ast = parser.parse(html, "url", Some(options));

        assert_eq!(serialize_nodes(&ast.root_nodes), vec![html]);
    }

    #[test]
    fn should_support_comment() {
        let html = "<!--comment-->";
        let parser = HtmlParser::new();
        let mut options = TokenizeOptions::default();
        options.tokenize_expansion_forms = true;
        let ast = parser.parse(html, "url", Some(options));

        assert_eq!(serialize_nodes(&ast.root_nodes), vec![html]);
    }

    #[test]
    fn should_support_nesting() {
        let html = r#"<div i18n="meaning|desc">
        <span>{{ interpolation }}</span>
        <!--comment-->
        <p expansion="true">
          {number, plural, =0 {{sex, select, other {<b>?</b>}}}}
        </p>
      </div>"#;
        let parser = HtmlParser::new();
        let mut options = TokenizeOptions::default();
        options.tokenize_expansion_forms = true;
        let ast = parser.parse(html, "url", Some(options));

        assert_eq!(serialize_nodes(&ast.root_nodes), vec![html]);
    }
}
