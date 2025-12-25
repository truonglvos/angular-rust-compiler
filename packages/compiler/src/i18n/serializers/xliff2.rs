//! XLIFF2 Serializer Module
//!
//! Corresponds to packages/compiler/src/i18n/serializers/xliff2.ts
//! XLIFF 2.0 format serializer
#![allow(dead_code)]

use crate::i18n::digest::decimal_digest;
use crate::i18n::i18n_ast::{self as i18n, Message, Node, Visitor};
use crate::i18n::serializers::serializer::{PlaceholderMapper, Serializer};
use crate::i18n::serializers::xml_helper as xml;
use crate::i18n::translation_bundle::LoadResult;
use std::collections::HashMap;

const VERSION: &str = "2.0";
const XMLNS: &str = "urn:oasis:names:tc:xliff:document:2.0";
const DEFAULT_SOURCE_LANG: &str = "en";
const PLACEHOLDER_TAG: &str = "ph";
const PLACEHOLDER_SPANNING_TAG: &str = "pc";
const MARKER_TAG: &str = "mrk";
const XLIFF_TAG: &str = "xliff";
const SOURCE_TAG: &str = "source";
const TARGET_TAG: &str = "target";
const UNIT_TAG: &str = "unit";

/// XLIFF 2.0 serializer
/// See https://docs.oasis-open.org/xliff/xliff-core/v2.0/os/xliff-core-v2.0-os.html
pub struct Xliff2 {
    // Implementation fields
}

impl Xliff2 {
    pub fn new() -> Self {
        Xliff2 {}
    }
}

impl Default for Xliff2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for Xliff2 {
    fn write(&self, messages: &[Message], locale: Option<&str>) -> String {
        let mut visitor = Xliff2Visitor;
        let source_lang = locale.unwrap_or(DEFAULT_SOURCE_LANG);

        let mut xliff_attrs = HashMap::new();
        xliff_attrs.insert("version".to_string(), VERSION.to_string());
        xliff_attrs.insert("xmlns".to_string(), XMLNS.to_string());
        xliff_attrs.insert("srcLang".to_string(), source_lang.to_string());
        let mut xliff_tag = xml::Tag::new(XLIFF_TAG.to_string(), xliff_attrs, Vec::new());

        let mut file_attrs = HashMap::new();
        file_attrs.insert("id".to_string(), "f1".to_string());
        let mut file_tag = xml::Tag::new("file".to_string(), file_attrs, Vec::new());

        for message in messages {
            let mut unit_attrs = HashMap::new();
            unit_attrs.insert("id".to_string(), message.id.clone());
            let mut unit_tag = xml::Tag::new(UNIT_TAG.to_string(), unit_attrs, Vec::new());

            // Add notes for meaning and description
            if !message.meaning.is_empty() {
                let mut note_attrs = HashMap::new();
                note_attrs.insert("category".to_string(), "x-meaning".to_string());
                let mut note_tag = xml::Tag::new("note".to_string(), note_attrs, Vec::new());
                note_tag
                    .children
                    .push(Box::new(xml::Text::new(message.meaning.clone())));
                unit_tag.children.push(Box::new(note_tag));
            }

            if !message.description.is_empty() {
                let mut note_attrs = HashMap::new();
                note_attrs.insert("category".to_string(), "x-description".to_string());
                let mut note_tag = xml::Tag::new("note".to_string(), note_attrs, Vec::new());
                note_tag
                    .children
                    .push(Box::new(xml::Text::new(message.description.clone())));
                unit_tag.children.push(Box::new(note_tag));
            }

            // Add segment with source
            let source_nodes = visitor.serialize(&message.nodes);
            let mut segment_tag = xml::Tag::new("segment".to_string(), HashMap::new(), Vec::new());
            let mut source_tag = xml::Tag::new(SOURCE_TAG.to_string(), HashMap::new(), Vec::new());
            source_tag.children.extend(source_nodes);
            segment_tag.children.push(Box::new(source_tag));
            unit_tag.children.push(Box::new(segment_tag));

            file_tag.children.push(Box::new(unit_tag));
        }

        xliff_tag.children.push(Box::new(file_tag));

        let mut decl_attrs = HashMap::new();
        decl_attrs.insert("version".to_string(), "1.0".to_string());
        decl_attrs.insert("encoding".to_string(), "UTF-8".to_string());

        let mut nodes: Vec<Box<dyn xml::Node>> = Vec::new();
        nodes.push(Box::new(xml::Declaration::new(decl_attrs)));
        nodes.push(Box::new(xliff_tag));

        xml::serialize(&nodes)
    }

    fn load(&self, _content: &str, _url: &str) -> LoadResult {
        // TODO: Implement XLIFF 2.0 load
        LoadResult {
            locale: None,
            i18n_nodes_by_msg_id: HashMap::new(),
        }
    }

    fn digest(&self, message: &Message) -> String {
        decimal_digest(message)
    }

    fn create_name_mapper(&self, _message: &Message) -> Option<Box<dyn PlaceholderMapper>> {
        None
    }
}

/// Visitor that converts i18n AST nodes to XLIFF2 XML nodes
struct Xliff2Visitor;

impl Xliff2Visitor {
    fn serialize(&mut self, nodes: &[Node]) -> Vec<Box<dyn xml::Node>> {
        let mut result: Vec<Box<dyn xml::Node>> = Vec::new();
        for node in nodes {
            let xml_nodes = node.visit(self, None);
            if let Ok(nodes_vec) = xml_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                result.extend(*nodes_vec);
            }
        }
        result
    }
}

impl Visitor for Xliff2Visitor {
    fn visit_text(
        &mut self,
        text: &i18n::Text,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        Box::new(vec![
            Box::new(xml::Text::new(text.value.clone())) as Box<dyn xml::Node>
        ])
    }

    fn visit_container(
        &mut self,
        container: &i18n::Container,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let mut result: Vec<Box<dyn xml::Node>> = Vec::new();
        for child in &container.children {
            let child_nodes = child.visit(self, None);
            if let Ok(nodes_vec) = child_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                result.extend(*nodes_vec);
            }
        }
        Box::new(result)
    }

    fn visit_icu(
        &mut self,
        icu: &i18n::Icu,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        // ICU messages are converted to text representation
        let mut result: Vec<Box<dyn xml::Node>> = Vec::new();
        let mut case_texts = Vec::new();
        for (k, v) in &icu.cases {
            let case_nodes = v.visit(self, None);
            let case_text = if let Ok(nodes_vec) = case_nodes.downcast::<Vec<Box<dyn xml::Node>>>()
            {
                xml::serialize(&nodes_vec)
            } else {
                String::new()
            };
            case_texts.push(format!("{} {{{}}}", k, case_text));
        }
        let icu_text = format!(
            "{{{{{}}}, {}, {}}}",
            icu.expression,
            icu.type_,
            case_texts.join(" ")
        );
        result.push(Box::new(xml::Text::new(icu_text)));
        Box::new(result)
    }

    fn visit_tag_placeholder(
        &mut self,
        ph: &i18n::TagPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.start_name.clone());
        let mut ph_tag = xml::Tag::new(PLACEHOLDER_SPANNING_TAG.to_string(), attrs, Vec::new());

        for child in &ph.children {
            let child_nodes = child.visit(self, None);
            if let Ok(nodes_vec) = child_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                ph_tag.children.extend(*nodes_vec);
            }
        }

        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_placeholder(
        &mut self,
        ph: &i18n::Placeholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.name.clone());
        let ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_block_placeholder(
        &mut self,
        ph: &i18n::BlockPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.start_name.clone());
        let mut ph_tag = xml::Tag::new(PLACEHOLDER_SPANNING_TAG.to_string(), attrs, Vec::new());

        for child in &ph.children {
            let child_nodes = child.visit(self, None);
            if let Ok(nodes_vec) = child_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                ph_tag.children.extend(*nodes_vec);
            }
        }

        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_icu_placeholder(
        &mut self,
        ph: &i18n::IcuPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.name.clone());
        let ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }
}
