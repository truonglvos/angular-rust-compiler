//! Render3 Partial Factory Compilation
//!
//! Corresponds to packages/compiler/src/render3/partial/factory.ts
//! Contains factory declaration compilation for partial/linking mode

use crate::output::output_ast::{
    Expression, ExternalExpr, InvokeFunctionExpr, LiteralExpr, LiteralValue, ReadPropExpr,
};
use crate::render3::r3_factory::{FactoryTarget, R3FactoryMetadata};
use crate::render3::r3_identifiers::Identifiers as R3;
use crate::render3::util::R3CompiledExpression;
use crate::render3::view::util::DefinitionMap;

use super::util::{compile_dependencies, DepsValue};

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

/// Compile a factory declaration.
pub fn compile_declare_factory_function(meta: &R3FactoryMetadata) -> R3CompiledExpression {
    let base = meta.base();
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

    definition_map.set("type", Some(base.type_.value.clone()));

    // deps
    let deps_value = match &base.deps {
        Some(crate::render3::r3_factory::DepsOrInvalid::Valid(deps)) => {
            DepsValue::Valid(deps.clone())
        }
        Some(crate::render3::r3_factory::DepsOrInvalid::Invalid) => DepsValue::Invalid,
        None => DepsValue::None,
    };
    definition_map.set("deps", Some(compile_dependencies(&deps_value)));

    // target - convert enum to string name
    let target_name = match base.target {
        FactoryTarget::Directive => "Directive",
        FactoryTarget::Component => "Component",
        FactoryTarget::Injectable => "Injectable",
        FactoryTarget::Pipe => "Pipe",
        FactoryTarget::NgModule => "NgModule",
    };

    // FactoryTarget.X - access via property
    // In TypeScript: o.importExpr(R3.FactoryTarget).prop(FactoryTarget[meta.target])
    // We need to import FactoryTarget and access the property
    let factory_target_ref = R3::factory_target();
    let factory_target_expr = external_expr(factory_target_ref);
    let target_expr = Expression::ReadProp(ReadPropExpr {
        receiver: Box::new(factory_target_expr),
        name: target_name.to_string(),
        type_: None,
        source_span: None,
    });
    definition_map.set("target", Some(target_expr));

    let declare_factory_ref = R3::declare_factory();
    let declare_factory_expr = external_expr(declare_factory_ref);

    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(declare_factory_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: false,
    });

    // TODO: implement create_factory_type
    let type_ = crate::output::output_ast::dynamic_type();

    R3CompiledExpression::new(expression, type_, vec![])
}
