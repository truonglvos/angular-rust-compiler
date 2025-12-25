//! Render3 Pipe Compiler
//!
//! Corresponds to packages/compiler/src/render3/r3_pipe_compiler.ts
//! Contains pipe definition compilation

use super::r3_factory::R3DependencyMetadata;
use super::r3_identifiers::Identifiers as R3;
use super::util::{type_with_parameters, R3CompiledExpression, R3Reference};
use crate::output::output_ast::{
    Expression, ExpressionType, ExternalExpr, InvokeFunctionExpr, LiteralExpr, LiteralMapEntry,
    LiteralMapExpr, LiteralValue, Type, TypeModifier,
};

/// Metadata for pipe compilation
#[derive(Debug, Clone)]
pub struct R3PipeMetadata {
    /// Name of the pipe type
    pub name: String,
    /// An expression representing a reference to the pipe itself
    pub type_: R3Reference,
    /// Number of generic type parameters of the type itself
    pub type_argument_count: usize,
    /// Name of the pipe
    pub pipe_name: Option<String>,
    /// Dependencies of the pipe's constructor
    pub deps: Option<Vec<R3DependencyMetadata>>,
    /// Whether the pipe is marked as pure
    pub pure: bool,
    /// Whether the pipe is standalone
    pub is_standalone: bool,
}

/// Compile a pipe from metadata
pub fn compile_pipe_from_metadata(metadata: &R3PipeMetadata) -> R3CompiledExpression {
    let mut definition_map_values: Vec<LiteralMapEntry> = Vec::new();

    // e.g. `name: 'myPipe'`
    let pipe_name = metadata
        .pipe_name
        .clone()
        .unwrap_or_else(|| metadata.name.clone());
    definition_map_values.push(LiteralMapEntry {
        key: "name".to_string(),
        value: Box::new(Expression::Literal(LiteralExpr {
            value: LiteralValue::String(pipe_name.clone()),
            type_: None,
            source_span: None,
        })),
        quoted: false,
    });

    // e.g. `type: MyPipe`
    definition_map_values.push(LiteralMapEntry {
        key: "type".to_string(),
        value: Box::new(metadata.type_.value.clone()),
        quoted: false,
    });

    // e.g. `pure: true`
    definition_map_values.push(LiteralMapEntry {
        key: "pure".to_string(),
        value: Box::new(Expression::Literal(LiteralExpr {
            value: LiteralValue::Bool(metadata.pure),
            type_: None,
            source_span: None,
        })),
        quoted: false,
    });

    if !metadata.is_standalone {
        definition_map_values.push(LiteralMapEntry {
            key: "standalone".to_string(),
            value: Box::new(Expression::Literal(LiteralExpr {
                value: LiteralValue::Bool(false),
                type_: None,
                source_span: None,
            })),
            quoted: false,
        });
    }

    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(Expression::External(ExternalExpr {
            value: R3::define_pipe(),
            type_: None,
            source_span: None,
        })),
        args: vec![Expression::LiteralMap(LiteralMapExpr {
            entries: definition_map_values,
            type_: None,
            source_span: None,
        })],
        type_: None,
        source_span: None,
        pure: true,
    });

    let type_ = create_pipe_type(metadata);

    R3CompiledExpression::new(expression, type_, vec![])
}

/// Create the type for a pipe
pub fn create_pipe_type(metadata: &R3PipeMetadata) -> Type {
    let pipe_name = metadata
        .pipe_name
        .clone()
        .unwrap_or_else(|| metadata.name.clone());

    Type::Expression(ExpressionType {
        value: Box::new(Expression::External(ExternalExpr {
            value: R3::pipe_declaration(),
            type_: None,
            source_span: None,
        })),
        modifiers: TypeModifier::None,
        type_params: Some(vec![
            type_with_parameters(
                metadata.type_.type_expr.clone(),
                metadata.type_argument_count,
            ),
            Type::Expression(ExpressionType {
                value: Box::new(Expression::Literal(LiteralExpr {
                    value: LiteralValue::String(pipe_name),
                    type_: None,
                    source_span: None,
                })),
                modifiers: TypeModifier::None,
                type_params: None,
            }),
            Type::Expression(ExpressionType {
                value: Box::new(Expression::Literal(LiteralExpr {
                    value: LiteralValue::Bool(metadata.is_standalone),
                    type_: None,
                    source_span: None,
                })),
                modifiers: TypeModifier::None,
                type_params: None,
            }),
        ]),
    })
}
