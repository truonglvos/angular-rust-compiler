//! Message Bundle Module
//!
//! Corresponds to packages/compiler/src/i18n/message_bundle.ts
//! A container for messages extracted from templates

use crate::i18n::extractor_merger::extract_messages;
use crate::i18n::i18n_ast::{Message, Node, Visitor};
use crate::i18n::serializers::serializer::{PlaceholderMapper, Serializer};
use crate::ml_parser::ast::Node as HtmlNode;
use crate::ml_parser::html_parser::HtmlParser;
use crate::ml_parser::lexer::TokenizeOptions;
use crate::parse_util::ParseError;
use std::collections::HashMap;

/// A container for message extracted from the templates
pub struct MessageBundle {
    messages: Vec<Message>,
    html_parser: HtmlParser,
    implicit_tags: Vec<String>,
    implicit_attrs: HashMap<String, Vec<String>>,
    locale: Option<String>,
    preserve_whitespace: bool,
}

impl MessageBundle {
    pub fn new(
        html_parser: HtmlParser,
        implicit_tags: Vec<String>,
        implicit_attrs: HashMap<String, Vec<String>>,
        locale: Option<String>,
        preserve_whitespace: bool,
    ) -> Self {
        MessageBundle {
            messages: Vec::new(),
            html_parser,
            implicit_tags,
            implicit_attrs,
            locale,
            preserve_whitespace,
        }
    }

    pub fn update_from_template(&mut self, source: &str, url: &str) -> Vec<ParseError> {
        // Parse HTML with tokenizeExpansionForms: true
        let mut tokenize_options = TokenizeOptions::default();
        tokenize_options.tokenize_expansion_forms = true;

        let html_parser_result = self.html_parser.parse(source, url, Some(tokenize_options));

        if !html_parser_result.errors.is_empty() {
            return html_parser_result.errors;
        }

        // Note: WhitespaceVisitor is not yet implemented, so we skip whitespace trimming
        // for now. This can be added later when WhitespaceVisitor is implemented.
        let root_nodes: Vec<HtmlNode> = if self.preserve_whitespace {
            html_parser_result.root_nodes
        } else {
            // TODO: Apply WhitespaceVisitor when implemented
            // For now, just use the nodes as-is
            html_parser_result.root_nodes
        };

        // Extract i18n messages
        let i18n_parser_result = extract_messages(
            &root_nodes,
            &self.implicit_tags,
            &self.implicit_attrs,
            self.preserve_whitespace,
        );

        if !i18n_parser_result.errors.is_empty() {
            return i18n_parser_result.errors;
        }

        // Push messages to self.messages
        self.messages.extend(i18n_parser_result.messages);
        vec![]
    }

    pub fn get_messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn write(
        &self,
        serializer: &dyn Serializer,
        filter_sources: Option<fn(&str) -> String>,
    ) -> String {
        let mut messages: HashMap<String, Message> = HashMap::new();
        let mapper_visitor = MapPlaceholderNames;

        // Deduplicate messages based on their ID
        for message in &self.messages {
            let id = serializer.digest(message);
            if !messages.contains_key(&id) {
                messages.insert(id.clone(), message.clone());
            } else {
                // Merge sources
                if let Some(msg) = messages.get_mut(&id) {
                    msg.sources.extend(message.sources.clone());
                }
            }
        }

        // Transform placeholder names using the serializer mapping
        let msg_list: Vec<Message> = messages
            .into_iter()
            .map(|(id, src)| {
                let mapper = serializer.create_name_mapper(&src);
                let nodes = if let Some(ref mapper) = mapper {
                    mapper_visitor.convert(&src.nodes, mapper.as_ref())
                } else {
                    src.nodes.clone()
                };

                let mut transformed_message = Message::new(
                    nodes,
                    HashMap::new(),
                    HashMap::new(),
                    src.meaning.clone(),
                    src.description.clone(),
                    id.clone(),
                );
                transformed_message.sources = src.sources;
                transformed_message.id = id;

                // Apply filter_sources if provided
                if let Some(filter) = filter_sources {
                    for source in &mut transformed_message.sources {
                        source.file_path = filter(&source.file_path);
                    }
                }

                transformed_message
            })
            .collect();

        serializer.write(&msg_list, self.locale.as_deref())
    }
}

/// Transform an i18n AST by renaming the placeholder nodes with the given mapper
struct MapPlaceholderNames;

impl MapPlaceholderNames {
    fn convert(&self, nodes: &[Node], mapper: &dyn PlaceholderMapper) -> Vec<Node> {
        let mut visitor = MapPlaceholderNamesVisitor { mapper };
        nodes
            .iter()
            .map(|n| {
                let result = n.visit(&mut visitor, None);
                *result.downcast::<Node>().unwrap()
            })
            .collect()
    }
}

struct MapPlaceholderNamesVisitor<'a> {
    mapper: &'a dyn PlaceholderMapper,
}

impl Visitor for MapPlaceholderNamesVisitor<'_> {
    fn visit_text(
        &mut self,
        text: &crate::i18n::i18n_ast::Text,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        Box::new(Node::Text(text.clone()))
    }

    fn visit_container(
        &mut self,
        container: &crate::i18n::i18n_ast::Container,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let children: Vec<Node> = container
            .children
            .iter()
            .map(|n| {
                let result = n.visit(self, None);
                *result.downcast::<Node>().unwrap()
            })
            .collect();
        Box::new(Node::Container(crate::i18n::i18n_ast::Container {
            children,
            source_span: container.source_span.clone(),
        }))
    }

    fn visit_icu(
        &mut self,
        icu: &crate::i18n::i18n_ast::Icu,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let cases: HashMap<String, crate::i18n::i18n_ast::Node> = icu
            .cases
            .iter()
            .map(|(k, v)| {
                let result = v.visit(self, None);
                (k.clone(), *result.downcast::<Node>().unwrap())
            })
            .collect();
        Box::new(Node::Icu(crate::i18n::i18n_ast::Icu {
            expression: icu.expression.clone(),
            type_: icu.type_.clone(),
            cases,
            source_span: icu.source_span.clone(),
            expression_placeholder: icu.expression_placeholder.clone(),
        }))
    }

    fn visit_tag_placeholder(
        &mut self,
        ph: &crate::i18n::i18n_ast::TagPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let start_name = self
            .mapper
            .to_public_name(&ph.start_name)
            .unwrap_or_else(|| ph.start_name.clone());
        let close_name = self
            .mapper
            .to_public_name(&ph.close_name)
            .unwrap_or_else(|| ph.close_name.clone());
        let children: Vec<Node> = ph
            .children
            .iter()
            .map(|n| {
                let result = n.visit(self, None);
                *result.downcast::<Node>().unwrap()
            })
            .collect();
        Box::new(Node::TagPlaceholder(
            crate::i18n::i18n_ast::TagPlaceholder {
                tag: ph.tag.clone(),
                attrs: ph.attrs.clone(),
                start_name,
                close_name,
                children,
                is_void: ph.is_void,
                source_span: ph.source_span.clone(),
                start_source_span: ph.start_source_span.clone(),
                end_source_span: ph.end_source_span.clone(),
            },
        ))
    }

    fn visit_placeholder(
        &mut self,
        ph: &crate::i18n::i18n_ast::Placeholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let name = self
            .mapper
            .to_public_name(&ph.name)
            .unwrap_or_else(|| ph.name.clone());
        Box::new(Node::Placeholder(crate::i18n::i18n_ast::Placeholder {
            value: ph.value.clone(),
            name,
            source_span: ph.source_span.clone(),
        }))
    }

    fn visit_icu_placeholder(
        &mut self,
        ph: &crate::i18n::i18n_ast::IcuPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let name = self
            .mapper
            .to_public_name(&ph.name)
            .unwrap_or_else(|| ph.name.clone());
        Box::new(Node::IcuPlaceholder(
            crate::i18n::i18n_ast::IcuPlaceholder {
                value: ph.value.clone(),
                name,
                source_span: ph.source_span.clone(),
                previous_message: ph.previous_message.clone(),
            },
        ))
    }

    fn visit_block_placeholder(
        &mut self,
        ph: &crate::i18n::i18n_ast::BlockPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let start_name = self
            .mapper
            .to_public_name(&ph.start_name)
            .unwrap_or_else(|| ph.start_name.clone());
        let close_name = self
            .mapper
            .to_public_name(&ph.close_name)
            .unwrap_or_else(|| ph.close_name.clone());
        let children: Vec<Node> = ph
            .children
            .iter()
            .map(|n| {
                let result = n.visit(self, None);
                *result.downcast::<Node>().unwrap()
            })
            .collect();
        Box::new(Node::BlockPlaceholder(
            crate::i18n::i18n_ast::BlockPlaceholder {
                name: ph.name.clone(),
                parameters: ph.parameters.clone(),
                start_name,
                close_name,
                children,
                source_span: ph.source_span.clone(),
                start_source_span: ph.start_source_span.clone(),
                end_source_span: ph.end_source_span.clone(),
            },
        ))
    }
}
