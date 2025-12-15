//! XLIFF Serializer Module
//!
//! Corresponds to packages/compiler/src/i18n/serializers/xliff.ts
//! XLIFF 1.2 format serializer
#![allow(dead_code)]

use crate::i18n::i18n_ast::{self as i18n, Message, Node, Visitor};
use crate::i18n::digest::digest;
use crate::i18n::serializers::serializer::{Serializer, PlaceholderMapper};
use crate::i18n::serializers::xml_helper as xml;
use crate::i18n::translation_bundle::LoadResult;
use std::collections::HashMap;

const VERSION: &str = "1.2";
const XMLNS: &str = "urn:oasis:names:tc:xliff:document:1.2";
const DEFAULT_SOURCE_LANG: &str = "en";
const PLACEHOLDER_TAG: &str = "x";
const MARKER_TAG: &str = "mrk";
const FILE_TAG: &str = "file";
const SOURCE_TAG: &str = "source";
const SEGMENT_SOURCE_TAG: &str = "seg-source";
const ALT_TRANS_TAG: &str = "alt-trans";
const TARGET_TAG: &str = "target";
const UNIT_TAG: &str = "trans-unit";
const CONTEXT_GROUP_TAG: &str = "context-group";
const CONTEXT_TAG: &str = "context";

/// XLIFF 1.2 serializer
/// See https://docs.oasis-open.org/xliff/v1.2/os/xliff-core.html
/// See https://docs.oasis-open.org/xliff/v1.2/xliff-profile-html/xliff-profile-html-1.2.html
pub struct Xliff {
    // Implementation fields
}

impl Xliff {
    pub fn new() -> Self {
        Xliff {}
    }
}

impl Default for Xliff {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for Xliff {
    fn write(&self, messages: &[Message], locale: Option<&str>) -> String {
        let mut visitor = XliffVisitor;
        let source_lang = locale.unwrap_or(DEFAULT_SOURCE_LANG);
        
        let mut xliff_attrs = HashMap::new();
        xliff_attrs.insert("version".to_string(), VERSION.to_string());
        xliff_attrs.insert("xmlns".to_string(), XMLNS.to_string());
        let mut xliff_tag = xml::Tag::new("xliff".to_string(), xliff_attrs, Vec::new());

        let mut file_attrs = HashMap::new();
        file_attrs.insert("source-language".to_string(), source_lang.to_string());
        file_attrs.insert("datatype".to_string(), "html".to_string());
        let mut file_tag = xml::Tag::new(FILE_TAG.to_string(), file_attrs, Vec::new());

        let mut body_tag = xml::Tag::new("body".to_string(), HashMap::new(), Vec::new());

        for message in messages {
            let mut unit_attrs = HashMap::new();
            unit_attrs.insert("id".to_string(), message.id.clone());
            let mut unit_tag = xml::Tag::new(UNIT_TAG.to_string(), unit_attrs, Vec::new());

            // Add context-group with context for meaning and description
            if !message.meaning.is_empty() || !message.description.is_empty() {
                let mut context_group = xml::Tag::new(CONTEXT_GROUP_TAG.to_string(), HashMap::new(), Vec::new());
                
                if !message.meaning.is_empty() {
                    let mut context_attrs = HashMap::new();
                    context_attrs.insert("context-type".to_string(), "x-meaning".to_string());
                    let mut context_tag = xml::Tag::new(CONTEXT_TAG.to_string(), context_attrs, Vec::new());
                    context_tag.children.push(Box::new(xml::Text::new(message.meaning.clone())));
                    context_group.children.push(Box::new(context_tag));
                }

                if !message.description.is_empty() {
                    let mut context_attrs = HashMap::new();
                    context_attrs.insert("context-type".to_string(), "x-description".to_string());
                    let mut context_tag = xml::Tag::new(CONTEXT_TAG.to_string(), context_attrs, Vec::new());
                    context_tag.children.push(Box::new(xml::Text::new(message.description.clone())));
                    context_group.children.push(Box::new(context_tag));
                }

                unit_tag.children.push(Box::new(context_group));
            }

            // Add source
            let source_nodes = visitor.serialize(&message.nodes);
            let mut source_tag = xml::Tag::new(SOURCE_TAG.to_string(), HashMap::new(), Vec::new());
            source_tag.children.extend(source_nodes);
            unit_tag.children.push(Box::new(source_tag));

            body_tag.children.push(Box::new(unit_tag));
        }

        file_tag.children.push(Box::new(body_tag));
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
        // TODO: Implement XLIFF 1.2 load
        LoadResult {
            locale: None,
            i18n_nodes_by_msg_id: HashMap::new(),
        }
    }

    fn digest(&self, message: &Message) -> String {
        digest(message)
    }

    fn create_name_mapper(&self, _message: &Message) -> Option<Box<dyn PlaceholderMapper>> {
        None
    }
}

/// Visitor that converts i18n AST nodes to XLIFF XML nodes
struct XliffVisitor;

impl XliffVisitor {
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

impl Visitor for XliffVisitor {
    fn visit_text(&mut self, text: &i18n::Text, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        Box::new(vec![Box::new(xml::Text::new(text.value.clone())) as Box<dyn xml::Node>])
    }

    fn visit_container(&mut self, container: &i18n::Container, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        let mut result: Vec<Box<dyn xml::Node>> = Vec::new();
        for child in &container.children {
            let child_nodes = child.visit(self, None);
            if let Ok(nodes_vec) = child_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                result.extend(*nodes_vec);
            }
        }
        Box::new(result)
    }

    fn visit_icu(&mut self, icu: &i18n::Icu, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // ICU messages are converted to text representation
        let mut result: Vec<Box<dyn xml::Node>> = Vec::new();
        let mut case_texts = Vec::new();
        for (k, v) in &icu.cases {
            let case_nodes = v.visit(self, None);
            let case_text = if let Ok(nodes_vec) = case_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                xml::serialize(&nodes_vec)
            } else {
                String::new()
            };
            case_texts.push(format!("{} {{{}}}", k, case_text));
        }
        let icu_text = format!("{{{{{}}}, {}, {}}}", 
            icu.expression,
            icu.type_,
            case_texts.join(" ")
        );
        result.push(Box::new(xml::Text::new(icu_text)));
        Box::new(result)
    }

    fn visit_tag_placeholder(&mut self, ph: &i18n::TagPlaceholder, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.start_name.clone());
        let mut ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        
        for child in &ph.children {
            let child_nodes = child.visit(self, None);
            if let Ok(nodes_vec) = child_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                ph_tag.children.extend(*nodes_vec);
            }
        }
        
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_placeholder(&mut self, ph: &i18n::Placeholder, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.name.clone());
        let ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_block_placeholder(&mut self, ph: &i18n::BlockPlaceholder, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.start_name.clone());
        let mut ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        
        for child in &ph.children {
            let child_nodes = child.visit(self, None);
            if let Ok(nodes_vec) = child_nodes.downcast::<Vec<Box<dyn xml::Node>>>() {
                ph_tag.children.extend(*nodes_vec);
            }
        }
        
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_icu_placeholder(&mut self, ph: &i18n::IcuPlaceholder, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("id".to_string(), ph.name.clone());
        let ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }
}

