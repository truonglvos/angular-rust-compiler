//! Render3 Partial Injector Compilation
//!
//! Corresponds to packages/compiler/src/render3/partial/injector.ts
//! Contains injector declaration compilation for partial/linking mode

use crate::output::output_ast::{
    Expression, ExternalExpr, InvokeFunctionExpr, LiteralArrayExpr, LiteralExpr, LiteralValue,
};
use crate::render3::r3_identifiers::Identifiers as R3;
use crate::render3::r3_injector_compiler::R3InjectorMetadata;
use crate::render3::util::R3CompiledExpression;
use crate::render3::view::util::DefinitionMap;

/// Minimum version for partial linker
const MINIMUM_PARTIAL_LINKER_VERSION: &str = "12.0.0";

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

/// Compile an Injector declaration.
pub fn compile_declare_injector_from_metadata(meta: &R3InjectorMetadata) -> R3CompiledExpression {
    let definition_map = create_injector_definition_map(meta);

    let declare_injector_ref = R3::declare_injector();
    let declare_injector_expr = external_expr(declare_injector_ref);

    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(declare_injector_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: false,
    });

    // TODO: implement create_injector_type
    let type_ = crate::output::output_ast::dynamic_type();

    R3CompiledExpression::new(expression, type_, vec![])
}

/// Gathers the declaration fields for an Injector into a `DefinitionMap`.
fn create_injector_definition_map(meta: &R3InjectorMetadata) -> DefinitionMap {
    let mut definition_map = DefinitionMap::new();

    definition_map.set(
        "minVersion",
        Some(literal(LiteralValue::String(
            MINIMUM_PARTIAL_LINKER_VERSION.to_string(),
        ))),
    );
    definition_map.set(
        "version",
        Some(literal(LiteralValue::String(
            "0.0.0-PLACEHOLDER".to_string(),
        ))),
    );

    // ngImport: import("@angular/core")
    let core_ref = R3::core();
    let ng_import_expr = external_expr(core_ref);
    definition_map.set("ngImport", Some(ng_import_expr));

    definition_map.set("type", Some(meta.type_.value.clone()));

    if let Some(ref providers) = meta.providers {
        definition_map.set("providers", Some(providers.clone()));
    }

    if !meta.imports.is_empty() {
        definition_map.set(
            "imports",
            Some(Expression::LiteralArray(LiteralArrayExpr {
                entries: meta.imports.clone(),
                type_: None,
                source_span: None,
            })),
        );
    }

    definition_map
}
