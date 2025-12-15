//! Render3 Partial Injectable Compilation
//!
//! Corresponds to packages/compiler/src/render3/partial/injectable.ts
//! Contains injectable declaration compilation for partial/linking mode

use crate::output::output_ast::{
    Expression, LiteralExpr, LiteralValue, LiteralArrayExpr, ExternalExpr, InvokeFunctionExpr,
};
use crate::render3::r3_identifiers::Identifiers as R3;
use crate::render3::util::{R3CompiledExpression, convert_from_maybe_forward_ref_expression};
use crate::render3::view::util::DefinitionMap;

use super::util::compile_dependency;

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

/// Metadata for injectable compilation
#[derive(Debug, Clone)]
pub struct R3InjectableMetadata {
    pub name: String,
    pub type_: crate::render3::util::R3Reference,
    pub provided_in: Option<crate::render3::util::MaybeForwardRefExpression>,
    pub use_class: Option<crate::render3::util::MaybeForwardRefExpression>,
    pub use_factory: Option<Expression>,
    pub use_existing: Option<crate::render3::util::MaybeForwardRefExpression>,
    pub use_value: Option<crate::render3::util::MaybeForwardRefExpression>,
    pub deps: Option<Vec<crate::render3::r3_factory::R3DependencyMetadata>>,
}

/// Compile an Injectable declaration defined by the `R3InjectableMetadata`.
pub fn compile_declare_injectable_from_metadata(
    meta: &R3InjectableMetadata,
) -> R3CompiledExpression {
    let definition_map = create_injectable_definition_map(meta);

    let declare_injectable_ref = R3::declare_injectable();
    let declare_injectable_expr = external_expr(declare_injectable_ref);
    
    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(declare_injectable_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: false,
    });

    // TODO: implement create_injectable_type
    let type_ = crate::output::output_ast::dynamic_type();

    R3CompiledExpression::new(expression, type_, vec![])
}

/// Gathers the declaration fields for an Injectable into a `DefinitionMap`.
pub fn create_injectable_definition_map(meta: &R3InjectableMetadata) -> DefinitionMap {
    let mut definition_map = DefinitionMap::new();

    definition_map.set("minVersion", Some(literal(LiteralValue::String(MINIMUM_PARTIAL_LINKER_VERSION.to_string()))));
    definition_map.set("version", Some(literal(LiteralValue::String("0.0.0-PLACEHOLDER".to_string()))));
    
    // ngImport: import("@angular/core")
    let core_ref = R3::core();
    let ng_import_expr = external_expr(core_ref);
    definition_map.set("ngImport", Some(ng_import_expr));
    
    definition_map.set("type", Some(meta.type_.value.clone()));

    // Only generate providedIn property if it has a non-null value
    if let Some(ref provided_in) = meta.provided_in {
        let converted = convert_from_maybe_forward_ref_expression(provided_in);
        // Check if it's not null literal
        if !matches!(&converted, Expression::Literal(lit) if matches!(lit.value, LiteralValue::Null)) {
            definition_map.set("providedIn", Some(converted));
        }
    }

    if let Some(ref use_class) = meta.use_class {
        definition_map.set("useClass", Some(convert_from_maybe_forward_ref_expression(use_class)));
    }

    if let Some(ref use_existing) = meta.use_existing {
        definition_map.set("useExisting", Some(convert_from_maybe_forward_ref_expression(use_existing)));
    }

    if let Some(ref use_value) = meta.use_value {
        definition_map.set("useValue", Some(convert_from_maybe_forward_ref_expression(use_value)));
    }

    if let Some(ref use_factory) = meta.use_factory {
        definition_map.set("useFactory", Some(use_factory.clone()));
    }

    if let Some(ref deps) = meta.deps {
        let deps_exprs: Vec<Expression> = deps.iter().map(compile_dependency).collect();
        definition_map.set("deps", Some(Expression::LiteralArray(LiteralArrayExpr {
            entries: deps_exprs,
            type_: None,
            source_span: None,
        })));
    }

    definition_map
}
