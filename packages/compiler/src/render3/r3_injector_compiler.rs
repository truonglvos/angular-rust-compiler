//! Render3 Injector Compiler
//!
//! Corresponds to packages/compiler/src/render3/r3_injector_compiler.ts
//! Contains injector definition compilation

use crate::output::output_ast::{
    Expression, Type, ExpressionType, LiteralArrayExpr, ExternalExpr, InvokeFunctionExpr,
    TypeModifier,
};

use super::r3_identifiers::Identifiers as R3;
use super::util::{R3CompiledExpression, R3Reference, type_with_parameters};
use super::view::util::DefinitionMap;

/// Helper to create external expression from ExternalReference
fn external_expr(reference: crate::output::output_ast::ExternalReference) -> Expression {
    Expression::External(ExternalExpr {
        value: reference,
        type_: None,
        source_span: None,
    })
}

/// Metadata for injector compilation
#[derive(Debug, Clone)]
pub struct R3InjectorMetadata {
    pub name: String,
    pub type_: R3Reference,
    pub providers: Option<Expression>,
    pub imports: Vec<Expression>,
}

/// Compile an injector definition
pub fn compile_injector(meta: &R3InjectorMetadata) -> R3CompiledExpression {
    let mut definition_map = DefinitionMap::new();

    if let Some(ref providers) = meta.providers {
        definition_map.set("providers", Some(providers.clone()));
    }

    if !meta.imports.is_empty() {
        definition_map.set("imports", Some(Expression::LiteralArray(LiteralArrayExpr {
            entries: meta.imports.clone(),
            type_: None,
            source_span: None,
        })));
    }

    let define_injector_ref = R3::define_injector();
    let define_injector_expr = external_expr(define_injector_ref);
    
    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(define_injector_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: true,
    });

    let type_ = create_injector_type(meta);

    R3CompiledExpression::new(expression, type_, vec![])
}

/// Creates the type for an injector
pub fn create_injector_type(meta: &R3InjectorMetadata) -> Type {
    let injector_declaration_ref = R3::injector_declaration();
    let injector_declaration_expr = external_expr(injector_declaration_ref);
    
    Type::Expression(ExpressionType {
        value: Box::new(injector_declaration_expr),
        modifiers: TypeModifier::None,
        type_params: Some(vec![type_with_parameters(meta.type_.type_expr.clone(), 0)]),
    })
}
