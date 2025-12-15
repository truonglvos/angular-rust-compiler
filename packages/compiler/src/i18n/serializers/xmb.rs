//! XMB Serializer Module
//!
//! Corresponds to packages/compiler/src/i18n/serializers/xmb.ts
//! XMB (XML Message Bundle) format serializer
#![allow(dead_code)]

use crate::i18n::i18n_ast::{self as i18n, Message, Node, Visitor};
use crate::i18n::digest::decimal_digest;
use crate::i18n::serializers::serializer::{Serializer, PlaceholderMapper, SimplePlaceholderMapper};
use crate::i18n::serializers::xml_helper as xml;
use crate::i18n::translation_bundle::LoadResult;
use std::collections::HashMap;

/// Defines the `handler` value on the serialized XMB, indicating that Angular
/// generated the bundle. This is useful for analytics in Translation Console.
const XMB_HANDLER: &str = "angular";
const MESSAGES_TAG: &str = "messagebundle";
const MESSAGE_TAG: &str = "msg";
const PLACEHOLDER_TAG: &str = "ph";
const EXAMPLE_TAG: &str = "ex";
const SOURCE_TAG: &str = "source";

const DOCTYPE: &str = r#"<!ELEMENT messagebundle (msg)*>
<!ATTLIST messagebundle class CDATA #IMPLIED>

<!ELEMENT msg (#PCDATA|ph|source)*>
<!ATTLIST msg id CDATA #IMPLIED>
<!ATTLIST msg seq CDATA #IMPLIED>
<!ATTLIST msg name CDATA #IMPLIED>
<!ATTLIST msg desc CDATA #IMPLIED>
<!ATTLIST msg meaning CDATA #IMPLIED>
<!ATTLIST msg obsolete (obsolete) #IMPLIED>
<!ATTLIST msg xml:space (default|preserve) "default">
<!ATTLIST msg is_hidden CDATA #IMPLIED>

<!ELEMENT source (#PCDATA)>

<!ELEMENT ph (#PCDATA|ex)*>
<!ATTLIST ph name CDATA #REQUIRED>

<!ELEMENT ex (#PCDATA)>"#;

/// XMB (XML Message Bundle) serializer
pub struct Xmb {
    // Implementation fields
}

impl Xmb {
    pub fn new() -> Self {
        Xmb {}
    }
}

impl Default for Xmb {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for Xmb {
    fn write(&self, messages: &[Message], _locale: Option<&str>) -> String {
        let mut visitor = XmbVisitor;
        let mut root_attrs = HashMap::new();
        root_attrs.insert("handler".to_string(), XMB_HANDLER.to_string());
        let mut root = xml::Tag::new(MESSAGES_TAG.to_string(), root_attrs, Vec::new());

        for message in messages {
            let mut attrs = HashMap::new();
            attrs.insert("id".to_string(), message.id.clone());

            if !message.description.is_empty() {
                attrs.insert("desc".to_string(), message.description.clone());
            }

            if !message.meaning.is_empty() {
                attrs.insert("meaning".to_string(), message.meaning.clone());
            }

            let mut source_tags: Vec<Box<dyn xml::Node>> = Vec::new();
            for source in &message.sources {
                let source_attrs = HashMap::new();
                let source_text = if source.end_line != source.start_line {
                    format!("{}:{},{}", source.file_path, source.start_line, source.end_line)
                } else {
                    format!("{}:{}", source.file_path, source.start_line)
                };
                let mut source_tag = xml::Tag::new(SOURCE_TAG.to_string(), source_attrs, Vec::new());
                source_tag.children.push(Box::new(xml::Text::new(source_text)));
                source_tags.push(Box::new(source_tag));
            }

            let msg_nodes = visitor.serialize(&message.nodes);
            let mut msg_tag = xml::Tag::new(MESSAGE_TAG.to_string(), attrs, Vec::new());
            msg_tag.children.extend(source_tags);
            msg_tag.children.extend(msg_nodes);
            root.children.push(Box::new(msg_tag));
        }

        let mut decl_attrs = HashMap::new();
        decl_attrs.insert("version".to_string(), "1.0".to_string());
        decl_attrs.insert("encoding".to_string(), "UTF-8".to_string());

        let mut nodes: Vec<Box<dyn xml::Node>> = Vec::new();
        nodes.push(Box::new(xml::Declaration::new(decl_attrs)));
        nodes.push(Box::new(xml::Doctype::new(MESSAGES_TAG.to_string(), DOCTYPE.to_string())));
        nodes.push(Box::new(root));

        xml::serialize(&nodes)
    }

    fn load(&self, _content: &str, _url: &str) -> LoadResult {
        // XMB is write-only format, use XTB for loading
        panic!("Unsupported: XMB is a write-only format. Use XTB to load translations.");
    }

    fn digest(&self, message: &Message) -> String {
        decimal_digest(message)
    }

    fn create_name_mapper(&self, message: &Message) -> Option<Box<dyn PlaceholderMapper>> {
        Some(Box::new(SimplePlaceholderMapper::new(message, to_public_name)))
    }
}

/// Convert placeholder name to public XMB format
/// XMB placeholders can only contain A-Z, 0-9 and _
pub fn to_public_name(internal_name: &str) -> String {
    internal_name
        .to_uppercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

/// Visitor that converts i18n AST nodes to XMB XML nodes
struct XmbVisitor;

impl XmbVisitor {
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

impl Visitor for XmbVisitor {
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
                // Extract text from XML nodes by serializing them
                let serialized = xml::serialize(&nodes_vec);
                serialized
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
        attrs.insert("name".to_string(), ph.start_name.clone());
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
        attrs.insert("name".to_string(), ph.name.clone());
        let ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }

    fn visit_block_placeholder(&mut self, ph: &i18n::BlockPlaceholder, _context: Option<&mut dyn std::any::Any>) -> Box<dyn std::any::Any> {
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), ph.start_name.clone());
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
        attrs.insert("name".to_string(), ph.name.clone());
        let ph_tag = xml::Tag::new(PLACEHOLDER_TAG.to_string(), attrs, Vec::new());
        Box::new(vec![Box::new(ph_tag) as Box<dyn xml::Node>])
    }
}


