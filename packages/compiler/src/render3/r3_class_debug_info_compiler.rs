//! Render3 Class Debug Info Compiler
//!
//! Corresponds to packages/compiler/src/render3/r3_class_debug_info_compiler.ts
//! Contains class debug info compilation for runtime errors

use crate::output::output_ast::{
    Expression, ArrowFunctionExpr, ArrowFunctionBody, InvokeFunctionExpr,
    LiteralExpr, LiteralValue, LiteralMapExpr, LiteralMapEntry, ExternalExpr,
};

use super::r3_identifiers::Identifiers as R3;
use super::util::dev_only_guarded_expression;

/// Helper to create literal expression
fn literal(value: LiteralValue) -> Expression {
    Expression::Literal(LiteralExpr {
        value,
        type_: None,
        source_span: None,
    })
}

/// Helper to create external expression from ExternalReference
fn external_expr(reference: crate::output::output_ast::ExternalReference) -> Expression {
    Expression::External(ExternalExpr {
        value: reference,
        type_: None,
        source_span: None,
    })
}

/// Info needed for runtime errors related to a class
#[derive(Debug, Clone)]
pub struct R3ClassDebugInfo {
    /// The class identifier
    pub type_: Expression,
    /// A string literal containing the original class name
    pub class_name: Expression,
    /// A string literal containing the relative path of the file
    pub file_path: Option<Expression>,
    /// A number literal containing the line number
    pub line_number: Expression,
    /// Whether to check for orphan rendering
    pub forbid_orphan_rendering: bool,
}

/// Generate an ngDevMode guarded call to setClassDebugInfo
pub fn compile_class_debug_info(debug_info: &R3ClassDebugInfo) -> Expression {
    let mut debug_info_entries: Vec<LiteralMapEntry> = vec![];

    // className is always included
    debug_info_entries.push(LiteralMapEntry {
        key: "className".to_string(),
        value: Box::new(debug_info.class_name.clone()),
        quoted: false,
    });

    // Include file path and line number only if file path is available
    if let Some(ref file_path) = debug_info.file_path {
        debug_info_entries.push(LiteralMapEntry {
            key: "filePath".to_string(),
            value: Box::new(file_path.clone()),
            quoted: false,
        });
        debug_info_entries.push(LiteralMapEntry {
            key: "lineNumber".to_string(),
            value: Box::new(debug_info.line_number.clone()),
            quoted: false,
        });
    }

    // Include forbidOrphanRendering only if it's true (to reduce generated code)
    if debug_info.forbid_orphan_rendering {
        debug_info_entries.push(LiteralMapEntry {
            key: "forbidOrphanRendering".to_string(),
            value: Box::new(literal(LiteralValue::Bool(true))),
            quoted: false,
        });
    }

    let set_class_debug_info_ref = R3::set_class_debug_info();
    let set_class_debug_info_expr = external_expr(set_class_debug_info_ref);
    
    let fn_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(set_class_debug_info_expr),
        args: vec![
            debug_info.type_.clone(),
            Expression::LiteralMap(LiteralMapExpr {
                entries: debug_info_entries,
                type_: None,
                source_span: None,
            }),
        ],
        type_: None,
        source_span: None,
        pure: false,
    });

    let guarded = dev_only_guarded_expression(fn_call);
    let iife = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![],
        body: ArrowFunctionBody::Statements(vec![guarded.to_stmt()]),
        type_: None,
        source_span: None,
    });

    Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(iife),
        args: vec![],
        type_: None,
        source_span: None,
        pure: false,
    })
}
