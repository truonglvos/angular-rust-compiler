//! ICU Serializer
//!
//! Corresponds to packages/compiler/src/render3/view/i18n/icu_serializer.ts
//! Contains ICU node serialization logic

use crate::i18n::i18n_ast as i18n;

use super::util::format_i18n_placeholder_name;

/// ICU serializer visitor
pub struct IcuSerializerVisitor;

impl IcuSerializerVisitor {
    pub fn new() -> Self {
        IcuSerializerVisitor
    }

    fn format_ph(&self, value: &str) -> String {
        format!("{{{}}}", format_i18n_placeholder_name(value, false))
    }

    pub fn visit_text(&self, text: &i18n::Text) -> String {
        text.value.clone()
    }

    pub fn visit_container(&self, container: &i18n::Container) -> String {
        container.children
            .iter()
            .map(|child| self.visit_node(child))
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn visit_icu(&self, icu: &i18n::Icu) -> String {
        let str_cases: Vec<String> = icu.cases
            .iter()
            .map(|(k, v)| format!("{} {{{}}}", k, self.visit_node(v)))
            .collect();
        
        // Use expression_placeholder if available, otherwise use expression
        let placeholder = icu.expression_placeholder
            .as_ref()
            .unwrap_or(&icu.expression);
        
        format!(
            "{{{}, {}, {}}}",
            placeholder,
            icu.type_,
            str_cases.join(" ")
        )
    }

    pub fn visit_tag_placeholder(&self, ph: &i18n::TagPlaceholder) -> String {
        if ph.is_void {
            self.format_ph(&ph.start_name)
        } else {
            let children: String = ph.children
                .iter()
                .map(|child| self.visit_node(child))
                .collect();
            format!(
                "{}{}{}",
                self.format_ph(&ph.start_name),
                children,
                self.format_ph(&ph.close_name)
            )
        }
    }

    pub fn visit_placeholder(&self, ph: &i18n::Placeholder) -> String {
        self.format_ph(&ph.name)
    }

    pub fn visit_block_placeholder(&self, ph: &i18n::BlockPlaceholder) -> String {
        let children: String = ph.children
            .iter()
            .map(|child| self.visit_node(child))
            .collect();
        format!(
            "{}{}{}",
            self.format_ph(&ph.start_name),
            children,
            self.format_ph(&ph.close_name)
        )
    }

    pub fn visit_icu_placeholder(&self, ph: &i18n::IcuPlaceholder) -> String {
        self.format_ph(&ph.name)
    }

    pub fn visit_node(&self, node: &i18n::Node) -> String {
        match node {
            i18n::Node::Text(text) => self.visit_text(text),
            i18n::Node::Container(container) => self.visit_container(container),
            i18n::Node::Icu(icu) => self.visit_icu(icu),
            i18n::Node::TagPlaceholder(ph) => self.visit_tag_placeholder(ph),
            i18n::Node::Placeholder(ph) => self.visit_placeholder(ph),
            i18n::Node::BlockPlaceholder(ph) => self.visit_block_placeholder(ph),
            i18n::Node::IcuPlaceholder(ph) => self.visit_icu_placeholder(ph),
        }
    }
}

impl Default for IcuSerializerVisitor {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    static ref SERIALIZER: IcuSerializerVisitor = IcuSerializerVisitor::new();
}

/// Serialize an ICU node to a string
pub fn serialize_icu_node(icu: &i18n::Icu) -> String {
    SERIALIZER.visit_icu(icu)
}

