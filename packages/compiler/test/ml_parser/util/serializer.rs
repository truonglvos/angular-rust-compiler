use angular_compiler::ml_parser::ast::*;
use angular_compiler::ml_parser::html_tags::get_html_tag_definition;
/**
 * AST Serializer Utility
 *
 * Mirrors angular/packages/compiler/test/ml_parser/util/util.ts
 * Serializes HTML AST nodes back to HTML strings
 */
use std::sync::Arc;

pub struct SerializerVisitor;

impl SerializerVisitor {
    pub fn new() -> Self {
        SerializerVisitor
    }

    pub fn visit_element(&self, element: &Element) -> String {
        let attrs = self.visit_all_attributes(&element.attrs);

        let tag_def = get_html_tag_definition(&element.name);
        if tag_def.is_void {
            return format!("<{}{}/>", element.name, attrs);
        }

        let children = self.visit_all(&element.children);
        format!("<{}{}>{}</{}>", element.name, attrs, children, element.name)
    }

    pub fn visit_attribute(&self, attribute: &Attribute) -> String {
        format!("{}=\"{}\"", attribute.name, attribute.value)
    }

    pub fn visit_text(&self, text: &Text) -> String {
        text.value.to_string()
    }

    pub fn visit_comment(&self, comment: &Comment) -> String {
        let value = comment
            .value
            .as_ref()
            .map(|v| v.clone())
            .unwrap_or_default();
        format!("<!--{}-->", value)
    }

    pub fn visit_expansion(&self, expansion: &Expansion) -> String {
        let cases = self.visit_all_expansion_cases(&expansion.cases);
        format!(
            "{{{}, {},{}}}",
            expansion.switch_value, expansion.expansion_type, cases
        )
    }

    pub fn visit_expansion_case(&self, case: &ExpansionCase) -> String {
        let expression = self.visit_all(&case.expression);
        format!(" {} {{{}}}", case.value, expression)
    }

    pub fn visit_block(&self, block: &Block) -> String {
        let params = if block.parameters.is_empty() {
            " ".to_string()
        } else {
            let params_str = self.visit_all_block_params(&block.parameters);
            format!(" ({}) ", params_str)
        };
        let children = self.visit_all(&block.children);
        format!("@{}{}{{{}}}", block.name, params, children)
    }

    pub fn visit_block_parameter(&self, parameter: &BlockParameter) -> String {
        parameter.expression.to_string()
    }

    pub fn visit_let_declaration(&self, decl: &LetDeclaration) -> String {
        format!("@let {} = {};", decl.name, decl.value)
    }

    pub fn visit_component(&self, component: &Component) -> String {
        let attrs = self.visit_all_attributes(&component.attrs);
        let children = self.visit_all(&component.children);
        format!(
            "<{}{}>{}</{}>",
            component.component_name, attrs, children, component.component_name
        )
    }

    pub fn visit_directive(&self, directive: &Directive) -> String {
        let attrs = self.visit_all_attributes(&directive.attrs);
        format!("@{}{}", directive.name, attrs)
    }

    fn visit_all(&self, nodes: &[Node]) -> String {
        self.visit_all_with_separator(nodes, "", "")
    }

    fn visit_all_with_separator(&self, nodes: &[Node], separator: &str, prefix: &str) -> String {
        if nodes.is_empty() {
            return String::new();
        }

        let results: Vec<String> = nodes
            .iter()
            .map(|node| match node {
                Node::Element(e) => self.visit_element(e),
                Node::Attribute(a) => self.visit_attribute(a),
                Node::Text(t) => self.visit_text(t),
                Node::Comment(c) => self.visit_comment(c),
                Node::Expansion(e) => self.visit_expansion(e),
                Node::ExpansionCase(c) => self.visit_expansion_case(c),
                Node::Block(b) => self.visit_block(b),
                Node::BlockParameter(p) => self.visit_block_parameter(p),
                Node::LetDeclaration(d) => self.visit_let_declaration(d),
                Node::Component(c) => self.visit_component(c),
                Node::Directive(d) => self.visit_directive(d),
            })
            .collect();

        format!("{}{}", prefix, results.join(separator))
    }

    fn visit_all_expansion_cases(&self, cases: &[ExpansionCase]) -> String {
        cases
            .iter()
            .map(|case| self.visit_expansion_case(case))
            .collect::<Vec<_>>()
            .join("")
    }

    fn visit_all_block_params(&self, params: &[BlockParameter]) -> String {
        params
            .iter()
            .map(|p| self.visit_block_parameter(p))
            .collect::<Vec<_>>()
            .join(";")
    }

    fn visit_all_attributes(&self, attrs: &[Attribute]) -> String {
        if attrs.is_empty() {
            return String::new();
        }
        let attrs_str: Vec<String> = attrs.iter().map(|a| self.visit_attribute(a)).collect();
        format!(" {}", attrs_str.join(" "))
    }
}

/// Serialize HTML AST nodes back to HTML strings
pub fn serialize_nodes(nodes: &[Node]) -> Vec<String> {
    let visitor = SerializerVisitor::new();
    nodes
        .iter()
        .map(|node| match node {
            Node::Element(e) => visitor.visit_element(e),
            Node::Attribute(a) => visitor.visit_attribute(a),
            Node::Text(t) => visitor.visit_text(t),
            Node::Comment(c) => visitor.visit_comment(c),
            Node::Expansion(e) => visitor.visit_expansion(e),
            Node::ExpansionCase(c) => visitor.visit_expansion_case(c),
            Node::Block(b) => visitor.visit_block(b),
            Node::BlockParameter(p) => visitor.visit_block_parameter(p),
            Node::LetDeclaration(d) => visitor.visit_let_declaration(d),
            Node::Component(c) => visitor.visit_component(c),
            Node::Directive(d) => visitor.visit_directive(d),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_simple_text() {
        use angular_compiler::parse_util::{ParseLocation, ParseSourceFile, ParseSourceSpan};

        let file = ParseSourceFile::new(String::new(), "test.html".to_string());
        let location = ParseLocation::new(Arc::new(file), 0, 0, 0);
        let span = ParseSourceSpan::new(location.clone(), location);

        let text = Text {
            value: "hello".to_string().into(),
            source_span: span,
            tokens: vec![],
            i18n: None,
        };

        let visitor = SerializerVisitor::new();
        assert_eq!(visitor.visit_text(&text), "hello");
    }
}
