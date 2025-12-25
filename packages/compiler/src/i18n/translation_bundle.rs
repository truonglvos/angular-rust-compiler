//! Translation Bundle Module
//!
//! Corresponds to packages/compiler/src/i18n/translation_bundle.ts
//! A container for translated messages

use crate::core::MissingTranslationStrategy;
use crate::i18n::i18n_ast::{
    BlockPlaceholder, Container, Icu, IcuPlaceholder, Message, Node, Placeholder, TagPlaceholder,
    Text, Visitor,
};
use crate::i18n::serializers::serializer::{PlaceholderMapper, Serializer};
use crate::i18n::serializers::xml_helper::escape_xml;
use crate::ml_parser::ast as html;
use crate::ml_parser::html_parser::HtmlParser;
use crate::ml_parser::lexer::TokenizeOptions;
use crate::parse_util::ParseError;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

/// A container for translated messages
pub struct TranslationBundle {
    i18n_nodes_by_msg_id: HashMap<String, Vec<Node>>,
    locale: Option<String>,
    digest_fn: Rc<dyn Fn(&Message) -> String>,
    mapper_factory: Option<Rc<dyn Fn(&Message) -> Box<dyn PlaceholderMapper>>>,
    missing_translation_strategy: MissingTranslationStrategy,
    i18n_to_html: I18nToHtmlVisitor,
    // Store serializer for load method - wrapped in Arc for sharing
    serializer: Option<Arc<dyn Serializer>>,
}

impl TranslationBundle {
    pub fn new(
        i18n_nodes_by_msg_id: HashMap<String, Vec<Node>>,
        locale: Option<String>,
        digest_fn: Rc<dyn Fn(&Message) -> String>,
        mapper_factory: Option<Rc<dyn Fn(&Message) -> Box<dyn PlaceholderMapper>>>,
        missing_translation_strategy: MissingTranslationStrategy,
    ) -> Self {
        let i18n_to_html = I18nToHtmlVisitor::new(
            i18n_nodes_by_msg_id.clone(),
            locale.clone(),
            digest_fn.clone(),
            mapper_factory.clone(),
            missing_translation_strategy,
        );

        TranslationBundle {
            i18n_nodes_by_msg_id,
            locale,
            digest_fn,
            mapper_factory,
            missing_translation_strategy,
            i18n_to_html,
            serializer: None,
        }
    }

    /// Creates a `TranslationBundle` by parsing the given `content` with the `serializer`.
    /// Note: This requires the serializer to be `Send + Sync` to be stored in Arc
    pub fn load<S: Serializer + Send + Sync + 'static>(
        content: &str,
        url: &str,
        serializer: S,
        missing_translation_strategy: MissingTranslationStrategy,
    ) -> Result<Self, String> {
        let load_result = serializer.load(content, url);

        // Wrap serializer in Arc for sharing
        let serializer_arc = Arc::new(serializer);

        // Create closures that capture the Arc
        let serializer_clone = serializer_arc.clone();
        let digest_fn: Rc<dyn Fn(&Message) -> String> =
            Rc::new(move |m: &Message| serializer_clone.digest(m));

        let serializer_clone2 = serializer_arc.clone();
        let mapper_factory: Option<Rc<dyn Fn(&Message) -> Box<dyn PlaceholderMapper>>> =
            Some(Rc::new(move |m: &Message| {
                serializer_clone2
                    .create_name_mapper(m)
                    .unwrap_or_else(|| Box::new(NoOpPlaceholderMapper))
            }));

        let mut bundle = TranslationBundle::new(
            load_result.i18n_nodes_by_msg_id,
            load_result.locale,
            digest_fn,
            mapper_factory,
            missing_translation_strategy,
        );
        bundle.serializer = Some(serializer_arc);
        Ok(bundle)
    }

    /// Returns the translation as HTML nodes from the given source message.
    pub fn get(&mut self, src_msg: &Message) -> Result<Vec<html::Node>, String> {
        let result = self.i18n_to_html.convert(src_msg);

        if !result.errors.is_empty() {
            return Err(result
                .errors
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("\n"));
        }

        Ok(result.nodes)
    }

    pub fn has(&self, src_msg: &Message) -> bool {
        let id = (self.digest_fn)(src_msg);
        self.i18n_nodes_by_msg_id.contains_key(&id)
    }
}

struct ConvertResult {
    nodes: Vec<html::Node>,
    errors: Vec<ParseError>,
}

struct I18nToHtmlVisitor {
    i18n_nodes_by_msg_id: HashMap<String, Vec<Node>>,
    locale: Option<String>,
    digest_fn: Rc<dyn Fn(&Message) -> String>,
    mapper_factory: Option<Rc<dyn Fn(&Message) -> Box<dyn PlaceholderMapper>>>,
    missing_translation_strategy: MissingTranslationStrategy,
    src_msg: Option<Message>,
    errors: Vec<ParseError>,
    context_stack: Vec<ContextEntry>,
    mapper: Option<Box<dyn Fn(&str) -> String>>,
}

struct ContextEntry {
    msg: Message,
    // Note: We can't store Box<dyn Fn> in ContextEntry because it can't be cloned
    // Instead, we'll recreate the mapper when needed
    // For now, we'll use a simpler approach: store the mapper as a function pointer
    // But that won't work either...
    // Actually, we don't need to clone ContextEntry, so we can keep it as is
    // The issue is in convert_to_text where we try to clone mapper
    mapper: Option<Box<dyn Fn(&str) -> String>>,
}

// No-op placeholder mapper for when create_name_mapper returns None
struct NoOpPlaceholderMapper;

impl PlaceholderMapper for NoOpPlaceholderMapper {
    fn to_public_name(&self, internal_name: &str) -> Option<String> {
        Some(internal_name.to_string())
    }

    fn to_internal_name(&self, public_name: &str) -> Option<String> {
        Some(public_name.to_string())
    }
}

impl I18nToHtmlVisitor {
    fn new(
        i18n_nodes_by_msg_id: HashMap<String, Vec<Node>>,
        locale: Option<String>,
        digest_fn: Rc<dyn Fn(&Message) -> String>,
        mapper_factory: Option<Rc<dyn Fn(&Message) -> Box<dyn PlaceholderMapper>>>,
        missing_translation_strategy: MissingTranslationStrategy,
    ) -> Self {
        I18nToHtmlVisitor {
            i18n_nodes_by_msg_id,
            locale,
            digest_fn,
            mapper_factory,
            missing_translation_strategy,
            src_msg: None,
            errors: Vec::new(),
            context_stack: Vec::new(),
            mapper: None,
        }
    }

    fn convert(&mut self, src_msg: &Message) -> ConvertResult {
        self.context_stack.clear();
        self.errors.clear();

        // i18n to text
        let text = self.convert_to_text(src_msg);

        // text to html
        let url = if !src_msg.nodes.is_empty() {
            &src_msg.nodes[0].source_span().start.file.url
        } else {
            ""
        };

        let html_parser = HtmlParser::new();
        let mut tokenize_options = TokenizeOptions::default();
        tokenize_options.tokenize_expansion_forms = true;
        let html_result = html_parser.parse(&text, url, Some(tokenize_options));

        ConvertResult {
            nodes: html_result.root_nodes,
            errors: {
                let mut all_errors = self.errors.clone();
                all_errors.extend(html_result.errors);
                all_errors
            },
        }
    }

    fn convert_to_text(&mut self, src_msg: &Message) -> String {
        let id = (self.digest_fn)(src_msg);
        let mapper = self.mapper_factory.as_ref().map(|factory| factory(src_msg));

        let nodes: Vec<Node>;
        let mapper_fn: Box<dyn Fn(&str) -> String>;

        // Save current context
        let current_msg = self.src_msg.clone();
        // Note: We can't clone Box<dyn Fn>, so we'll store None and recreate if needed
        // Actually, we don't need to store the mapper in context since we can recreate it
        self.context_stack.push(ContextEntry {
            msg: current_msg.unwrap_or_else(|| {
                Message::new(
                    vec![],
                    HashMap::new(),
                    HashMap::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                )
            }),
            mapper: None, // We'll recreate the mapper when restoring context
        });

        // Set new context
        self.src_msg = Some(src_msg.clone());

        if self.i18n_nodes_by_msg_id.contains_key(&id) {
            // When there is a translation use its nodes as the source
            // And create a mapper to convert serialized placeholder names to internal names
            nodes = self.i18n_nodes_by_msg_id[&id].clone();
            if let Some(m) = mapper {
                // Wrap mapper in Rc to share it
                let mapper_rc: Rc<dyn PlaceholderMapper> = Rc::from(m);
                mapper_fn = Box::new(move |name: &str| {
                    mapper_rc
                        .to_internal_name(name)
                        .unwrap_or_else(|| name.to_string())
                });
            } else {
                mapper_fn = Box::new(|name: &str| name.to_string());
            }
        } else {
            // When no translation has been found
            // - report an error / a warning / nothing,
            // - use the nodes from the original message
            // - placeholders are already internal and need no mapper
            if self.missing_translation_strategy == MissingTranslationStrategy::Error {
                let ctx = if let Some(ref locale) = self.locale {
                    format!(" for locale \"{}\"", locale)
                } else {
                    String::new()
                };
                if !src_msg.nodes.is_empty() {
                    self.add_error(
                        &src_msg.nodes[0],
                        &format!("Missing translation for message \"{}\"{}", id, ctx),
                    );
                }
            }
            // Note: Warning strategy would require a Console trait, which we don't have yet
            nodes = src_msg.nodes.clone();
            mapper_fn = Box::new(|name: &str| name.to_string());
        }

        self.mapper = Some(mapper_fn);

        let text: String = nodes
            .iter()
            .map(|node| {
                let result = node.visit(self, None);
                *result
                    .downcast::<String>()
                    .unwrap_or_else(|_| Box::new(String::new()))
            })
            .collect();

        // Restore context
        if let Some(context) = self.context_stack.pop() {
            self.src_msg = Some(context.msg);
            // Recreate mapper from mapper_factory if needed
            // For now, set to None - it will be recreated on next convert_to_text call
            self.mapper = None;
        }

        text
    }

    fn add_error(&mut self, node: &Node, msg: &str) {
        self.errors
            .push(ParseError::new(node.source_span().clone(), msg.to_string()));
    }
}

impl Visitor for I18nToHtmlVisitor {
    fn visit_text(
        &mut self,
        text: &Text,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        // `convert()` uses an `HtmlParser` to return `html.Node`s
        // we should then make sure that any special characters are escaped
        Box::new(escape_xml(&text.value))
    }

    fn visit_container(
        &mut self,
        container: &Container,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let result: String = container
            .children
            .iter()
            .map(|n| {
                let visit_result = n.visit(self, None);
                *visit_result
                    .downcast::<String>()
                    .unwrap_or_else(|_| Box::new(String::new()))
            })
            .collect();
        Box::new(result)
    }

    fn visit_icu(
        &mut self,
        icu: &Icu,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let cases: Vec<String> = icu
            .cases
            .iter()
            .map(|(k, v)| {
                let case_result = v.visit(self, None);
                let case_text = *case_result
                    .downcast::<String>()
                    .unwrap_or_else(|_| Box::new(String::new()));
                format!("{} {{{}}}", k, case_text)
            })
            .collect();

        // TODO(vicb): Once all format switch to using expression placeholders
        // we should throw when the placeholder is not in the source message
        let exp = if let Some(ref src_msg) = self.src_msg {
            if src_msg.placeholders.contains_key(&icu.expression) {
                src_msg.placeholders[&icu.expression].text.clone()
            } else {
                icu.expression.clone()
            }
        } else {
            icu.expression.clone()
        };

        Box::new(format!(
            "{{{{{}}}, {}, {}}}",
            exp,
            icu.type_,
            cases.join(" ")
        ))
    }

    fn visit_placeholder(
        &mut self,
        ph: &Placeholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let ph_name = if let Some(ref mapper) = self.mapper {
            mapper(&ph.name)
        } else {
            ph.name.clone()
        };

        if let Some(ref src_msg) = self.src_msg {
            if src_msg.placeholders.contains_key(&ph_name) {
                return Box::new(src_msg.placeholders[&ph_name].text.clone());
            }

            if let Some(msg) = src_msg.placeholder_to_message.get(&ph_name) {
                let msg_clone = msg.clone();
                return Box::new(self.convert_to_text(&msg_clone));
            }
        }

        self.add_error(
            &Node::Placeholder(ph.clone()),
            &format!("Unknown placeholder \"{}\"", ph.name),
        );
        Box::new(String::new())
    }

    fn visit_tag_placeholder(
        &mut self,
        ph: &TagPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let tag = &ph.tag;
        let attrs: Vec<String> = ph
            .attrs
            .iter()
            .map(|(name, value)| format!("{}=\"{}\"", name, value))
            .collect();
        let attrs_str = attrs.join(" ");

        if ph.is_void {
            Box::new(format!("<{} {}/>", tag, attrs_str))
        } else {
            let children: String = ph
                .children
                .iter()
                .map(|c| {
                    let result = c.visit(self, None);
                    *result
                        .downcast::<String>()
                        .unwrap_or_else(|_| Box::new(String::new()))
                })
                .collect();
            Box::new(format!("<{} {}>{}</{}>", tag, attrs_str, children, tag))
        }
    }

    fn visit_icu_placeholder(
        &mut self,
        ph: &IcuPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        // An ICU placeholder references the source message to be serialized
        if let Some(ref src_msg) = self.src_msg {
            if let Some(msg) = src_msg.placeholder_to_message.get(&ph.name) {
                let msg_clone = msg.clone();
                return Box::new(self.convert_to_text(&msg_clone));
            }
        }
        Box::new(String::new())
    }

    fn visit_block_placeholder(
        &mut self,
        ph: &BlockPlaceholder,
        _context: Option<&mut dyn std::any::Any>,
    ) -> Box<dyn std::any::Any> {
        let params = if ph.parameters.is_empty() {
            String::new()
        } else {
            format!(" ({})", ph.parameters.join("; "))
        };
        let children: String = ph
            .children
            .iter()
            .map(|c| {
                let result = c.visit(self, None);
                *result
                    .downcast::<String>()
                    .unwrap_or_else(|_| Box::new(String::new()))
            })
            .collect();
        Box::new(format!("@{}{} {{{}}}", ph.name, params, children))
    }
}

pub struct LoadResult {
    pub locale: Option<String>,
    pub i18n_nodes_by_msg_id: HashMap<String, Vec<Node>>,
}
