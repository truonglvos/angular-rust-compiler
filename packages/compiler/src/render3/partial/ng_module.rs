//! Render3 Partial NgModule Compilation
//!
//! Corresponds to packages/compiler/src/render3/partial/ng_module.ts
//! Contains NgModule declaration compilation for partial/linking mode

use crate::output::output_ast::{
    Expression, LiteralExpr, LiteralValue, LiteralArrayExpr, ExternalExpr, InvokeFunctionExpr,
};
use crate::render3::r3_identifiers::Identifiers as R3;
use crate::render3::r3_module_compiler::{R3NgModuleMetadata, R3NgModuleMetadataKind};
use crate::render3::util::{R3CompiledExpression, refs_to_array};
use crate::render3::view::util::DefinitionMap;

/// Minimum version for partial linker
const MINIMUM_PARTIAL_LINKER_VERSION: &str = "14.0.0";

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

/// Compile an NgModule declaration.
pub fn compile_declare_ng_module_from_metadata(meta: &R3NgModuleMetadata) -> R3CompiledExpression {
    let definition_map = create_ng_module_definition_map(meta);

    let declare_ng_module_ref = R3::declare_ng_module();
    let declare_ng_module_expr = external_expr(declare_ng_module_ref);
    
    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(declare_ng_module_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: false,
    });

    // TODO: implement create_ng_module_type
    let type_ = crate::output::output_ast::dynamic_type();

    R3CompiledExpression::new(expression, type_, vec![])
}

/// Gathers the declaration fields for an NgModule into a `DefinitionMap`.
fn create_ng_module_definition_map(meta: &R3NgModuleMetadata) -> DefinitionMap {
    let mut definition_map = DefinitionMap::new();

    if matches!(meta.kind(), R3NgModuleMetadataKind::Local) {
        panic!("Invalid path! Local compilation mode should not get into the partial compilation path");
    }

    definition_map.set("minVersion", Some(literal(LiteralValue::String(MINIMUM_PARTIAL_LINKER_VERSION.to_string()))));
    definition_map.set("version", Some(literal(LiteralValue::String("0.0.0-PLACEHOLDER".to_string()))));
    
    // ngImport: import("@angular/core")
    let core_ref = R3::core();
    let ng_import_expr = external_expr(core_ref);
    definition_map.set("ngImport", Some(ng_import_expr));
    
    definition_map.set("type", Some(meta.type_().value.clone()));

    // Get global metadata
    if let R3NgModuleMetadata::Global(global) = meta {
        if !global.bootstrap.is_empty() {
            definition_map.set("bootstrap", Some(refs_to_array(&global.bootstrap, global.contains_forward_decls)));
        }

        if !global.declarations.is_empty() {
            definition_map.set("declarations", Some(refs_to_array(&global.declarations, global.contains_forward_decls)));
        }

        if !global.imports.is_empty() {
            definition_map.set("imports", Some(refs_to_array(&global.imports, global.contains_forward_decls)));
        }

        if !global.exports.is_empty() {
            definition_map.set("exports", Some(refs_to_array(&global.exports, global.contains_forward_decls)));
        }

        if let Some(ref schemas) = global.common.schemas {
            if !schemas.is_empty() {
                let schema_exprs: Vec<Expression> = schemas.iter().map(|r| r.value.clone()).collect();
                definition_map.set("schemas", Some(Expression::LiteralArray(LiteralArrayExpr {
                    entries: schema_exprs,
                    type_: None,
                    source_span: None,
                })));
            }
        }

        if let Some(ref id) = global.common.id {
            definition_map.set("id", Some(id.clone()));
        }
    }

    definition_map
}
