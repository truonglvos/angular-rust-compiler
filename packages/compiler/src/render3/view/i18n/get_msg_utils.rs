//! goog.getMsg Utilities
//!
//! Corresponds to packages/compiler/src/render3/view/i18n/get_msg_utils.ts
//! Contains utilities for generating goog.getMsg() statements

use std::collections::HashMap;

use crate::i18n::i18n_ast as i18n;
use crate::output::output_ast::{
    Expression, Statement, ReadVarExpr, DeclareVarStmt, ExpressionStatement,
    LiteralExpr, LiteralValue, InvokeFunctionExpr, LiteralMapExpr, LiteralMapEntry,
    StmtModifier,
};

use super::icu_serializer::serialize_icu_node;
use super::meta::i18n_meta_to_jsdoc;
use super::util::{format_i18n_placeholder_name, format_i18n_placeholder_names_in_map};

/// Closure uses `goog.getMsg(message)` to lookup translations
const GOOG_GET_MSG: &str = "goog.getMsg";

/// Generates a `goog.getMsg()` statement and reassignment.
///
/// The template:
/// ```html
/// <div i18n>Sent from {{ sender }} to <span class="receiver">{{ receiver }}</span></div>
/// ```
///
/// Generates:
/// ```ts
/// const MSG_FOO = goog.getMsg(
///   'Sent from {$interpolation} to {$startTagSpan}{$interpolation_1}{$closeTagSpan}.',
///   { 'interpolation': '\uFFFD0\uFFFD', ... },
///   { original_code: { 'interpolation': '{{ sender }}', ... } },
/// );
/// const I18N_0 = MSG_FOO;
/// ```
/// Helper to create a literal string expression
fn literal_string(s: String) -> Expression {
    Expression::Literal(LiteralExpr {
        value: LiteralValue::String(s),
        type_: None,
        source_span: None,
    })
}

pub fn create_google_get_msg_statements(
    variable: &ReadVarExpr,
    message: &i18n::Message,
    closure_var: &ReadVarExpr,
    placeholder_values: &HashMap<String, Expression>,
) -> Vec<Statement> {
    let message_string = serialize_i18n_message_for_get_msg(message);
    let mut args: Vec<Expression> = vec![literal_string(message_string)];

    if !placeholder_values.is_empty() {
        // Message template parameters containing the magic strings
        let formatted_params = format_i18n_placeholder_names_in_map(placeholder_values, true);
        let params_entries: Vec<LiteralMapEntry> = formatted_params
            .into_iter()
            .map(|(key, value)| LiteralMapEntry {
                key,
                value: Box::new(value),
                quoted: true,
            })
            .collect();
        args.push(Expression::LiteralMap(LiteralMapExpr {
            entries: params_entries,
            type_: None,
            source_span: None,
        }));

        // Message options object with original source code for placeholders
        let original_code_entries: Vec<LiteralMapEntry> = placeholder_values
            .keys()
            .map(|param| {
                let formatted_name = format_i18n_placeholder_name(param, true);
                let value = if let Some(placeholder) = message.placeholders.get(param) {
                    literal_string(placeholder.source_span.to_string())
                } else if let Some(msg) = message.placeholder_to_message.get(param) {
                    let source: String = msg.nodes
                        .iter()
                        .map(|node| node.source_span().to_string())
                        .collect::<Vec<_>>()
                        .join("");
                    literal_string(source)
                } else {
                    literal_string(String::new())
                };
                LiteralMapEntry {
                    key: formatted_name,
                    value: Box::new(value),
                    quoted: true,
                }
            })
            .collect();

        let original_code_map = Expression::LiteralMap(LiteralMapExpr {
            entries: original_code_entries,
            type_: None,
            source_span: None,
        });
        let options_entries = vec![LiteralMapEntry {
            key: "original_code".to_string(),
            value: Box::new(original_code_map),
            quoted: false,
        }];
        args.push(Expression::LiteralMap(LiteralMapExpr {
            entries: options_entries,
            type_: None,
            source_span: None,
        }));
    }

    // const MSG_... = goog.getMsg(..);
    // In TypeScript: o.variable(GOOG_GET_MSG).callFn(args)
    let goog_get_msg_var = Expression::ReadVar(ReadVarExpr {
        name: GOOG_GET_MSG.to_string(),
        type_: None,
        source_span: None,
    });
    let goog_get_msg_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(goog_get_msg_var),
        args,
        type_: None,
        source_span: None,
        pure: false,
    });
    
    // Get the closure variable name
    let closure_var_name = closure_var.name.clone();
    
    let goog_get_msg_stmt = Statement::DeclareVar(DeclareVarStmt {
        name: closure_var_name,
        value: Some(Box::new(goog_get_msg_call)),
        type_: None,
        modifiers: StmtModifier::Final,
        source_span: None,
    });
    
    // TODO: Add JSDoc comment support via addLeadingComment
    // googGetMsgStmt.addLeadingComment(i18nMetaToJSDoc(message));
    let _jsdoc = i18n_meta_to_jsdoc(&super::meta::I18nMeta {
        id: if message.id.is_empty() { None } else { Some(message.id.clone()) },
        custom_id: if message.custom_id.is_empty() { None } else { Some(message.custom_id.clone()) },
        legacy_ids: if message.legacy_ids.is_empty() { None } else { Some(message.legacy_ids.clone()) },
        description: if message.description.is_empty() { None } else { Some(message.description.clone()) },
        meaning: if message.meaning.is_empty() { None } else { Some(message.meaning.clone()) },
    });

    // I18N_X = MSG_...;
    // In TypeScript: variable.set(closureVar)
    // Create a WriteVarExpr to assign closureVar to variable
    let write_expr = Expression::WriteVar(crate::output::output_ast::WriteVarExpr {
        name: variable.name.clone(),
        value: Box::new(Expression::ReadVar(closure_var.clone())),
        type_: None,
        source_span: None,
    });
    let i18n_assignment_stmt = Statement::Expression(ExpressionStatement {
        expr: Box::new(write_expr),
        source_span: None,
    });

    vec![goog_get_msg_stmt, i18n_assignment_stmt]
}

/// Visitor for serializing i18n messages for goog.getMsg
pub struct GetMsgSerializerVisitor;

impl GetMsgSerializerVisitor {
    pub fn new() -> Self {
        GetMsgSerializerVisitor
    }

    fn format_ph(&self, value: &str) -> String {
        format!("{{${}}}", format_i18n_placeholder_name(value, true))
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
        serialize_icu_node(icu)
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

impl Default for GetMsgSerializerVisitor {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    static ref SERIALIZER_VISITOR: GetMsgSerializerVisitor = GetMsgSerializerVisitor::new();
}

/// Serialize an i18n message for goog.getMsg
pub fn serialize_i18n_message_for_get_msg(message: &i18n::Message) -> String {
    message.nodes
        .iter()
        .map(|node| SERIALIZER_VISITOR.visit_node(node))
        .collect::<Vec<_>>()
        .join("")
}

