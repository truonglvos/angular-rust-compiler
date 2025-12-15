//! Render3 Partial Pipe Compilation
//!
//! Corresponds to packages/compiler/src/render3/partial/pipe.ts
//! Contains pipe declaration compilation for partial/linking mode

use crate::output::output_ast::{
    Expression, LiteralExpr, LiteralValue, ExternalExpr,
};
use crate::render3::r3_identifiers::Identifiers as R3;
use crate::render3::r3_pipe_compiler::R3PipeMetadata;
use crate::render3::util::R3CompiledExpression;
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

/// Compile a Pipe declaration defined by the `R3PipeMetadata`.
pub fn compile_declare_pipe_from_metadata(meta: &R3PipeMetadata) -> R3CompiledExpression {
    let definition_map = create_pipe_definition_map(meta);

    let declare_pipe_ref = R3::declare_pipe();
    let declare_pipe_expr = external_expr(declare_pipe_ref);
    
    let expression = Expression::InvokeFn(crate::output::output_ast::InvokeFunctionExpr {
        fn_: Box::new(declare_pipe_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: false,
    });

    // TODO: implement create_pipe_type
    let type_ = crate::output::output_ast::dynamic_type();

    R3CompiledExpression::new(expression, type_, vec![])
}

/// Gathers the declaration fields for a Pipe into a `DefinitionMap`.
pub fn create_pipe_definition_map(meta: &R3PipeMetadata) -> DefinitionMap {
    let mut definition_map = DefinitionMap::new();

    definition_map.set("minVersion", Some(literal(LiteralValue::String(MINIMUM_PARTIAL_LINKER_VERSION.to_string()))));
    definition_map.set("version", Some(literal(LiteralValue::String("0.0.0-PLACEHOLDER".to_string()))));
    
    // ngImport: import("@angular/core")
    let core_ref = R3::core();
    let ng_import_expr = external_expr(core_ref);
    definition_map.set("ngImport", Some(ng_import_expr));

    // e.g. `type: MyPipe`
    definition_map.set("type", Some(meta.type_.value.clone()));

    if !meta.is_standalone {
        definition_map.set("isStandalone", Some(literal(LiteralValue::Bool(false))));
    }

    // e.g. `name: "myPipe"`
    let name = meta.pipe_name.as_ref().unwrap_or(&meta.name);
    definition_map.set("name", Some(literal(LiteralValue::String(name.clone()))));

    if !meta.pure {
        // e.g. `pure: false`
        definition_map.set("pure", Some(literal(LiteralValue::Bool(false))));
    }

    definition_map
}
