//! Render3 Partial Class Metadata Compilation
//!
//! Corresponds to packages/compiler/src/render3/partial/class_metadata.ts
//! Contains class metadata declaration compilation for partial/linking mode

use crate::output::output_ast::{
    ArrowFunctionBody, ArrowFunctionExpr, Expression, ExternalExpr, FnParam, InvokeFunctionExpr,
    LiteralExpr, LiteralValue,
};
use crate::render3::r3_class_metadata_compiler::{
    compile_component_metadata_async_resolver, R3ClassMetadata, R3DeferPerComponentDependency,
};
use crate::render3::r3_identifiers::Identifiers as R3;
use crate::render3::view::util::DefinitionMap;

/// Minimum version for partial linker
const MINIMUM_PARTIAL_LINKER_VERSION: &str = "12.0.0";

/// Minimum version at which deferred blocks are supported in the linker.
const MINIMUM_PARTIAL_LINKER_DEFER_SUPPORT_VERSION: &str = "18.0.0";

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

/// Compile class metadata declaration.
pub fn compile_declare_class_metadata(metadata: &R3ClassMetadata) -> Expression {
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

    definition_map.set("type", Some(metadata.type_.clone()));
    definition_map.set("decorators", Some(metadata.decorators.clone()));

    if let Some(ref ctor_params) = metadata.ctor_parameters {
        definition_map.set("ctorParameters", Some(ctor_params.clone()));
    }

    if let Some(ref prop_decorators) = metadata.prop_decorators {
        definition_map.set("propDecorators", Some(prop_decorators.clone()));
    }

    let declare_meta_ref = R3::declare_class_metadata();
    let declare_meta_expr = external_expr(declare_meta_ref);

    Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(declare_meta_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: false,
    })
}

/// Compile component class metadata with deferred dependencies.
pub fn compile_component_declare_class_metadata(
    metadata: &R3ClassMetadata,
    dependencies: Option<&[R3DeferPerComponentDependency]>,
) -> Expression {
    match dependencies {
        None | Some(&[]) => compile_declare_class_metadata(metadata),
        Some(deps) => {
            let mut definition_map = DefinitionMap::new();

            // Build callback return definition map
            let mut callback_return_map = DefinitionMap::new();
            callback_return_map.set("decorators", Some(metadata.decorators.clone()));
            callback_return_map.set(
                "ctorParameters",
                Some(
                    metadata
                        .ctor_parameters
                        .clone()
                        .unwrap_or_else(|| literal(LiteralValue::Null)),
                ),
            );
            callback_return_map.set(
                "propDecorators",
                Some(
                    metadata
                        .prop_decorators
                        .clone()
                        .unwrap_or_else(|| literal(LiteralValue::Null)),
                ),
            );

            definition_map.set(
                "minVersion",
                Some(literal(LiteralValue::String(
                    MINIMUM_PARTIAL_LINKER_DEFER_SUPPORT_VERSION.to_string(),
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

            definition_map.set("type", Some(metadata.type_.clone()));

            let async_resolver = compile_component_metadata_async_resolver(deps);
            definition_map.set(
                "resolveDeferredDeps",
                Some(Expression::ArrowFn(async_resolver)),
            );

            // resolveMetadata callback
            let params: Vec<FnParam> = deps
                .iter()
                .map(|dep| FnParam {
                    name: dep.symbol_name.clone(),
                    type_: Some(crate::output::output_ast::dynamic_type()),
                })
                .collect();
            let resolve_metadata = Expression::ArrowFn(ArrowFunctionExpr {
                params,
                body: ArrowFunctionBody::Expression(Box::new(Expression::LiteralMap(
                    callback_return_map.to_literal_map(),
                ))),
                type_: None,
                source_span: None,
            });
            definition_map.set("resolveMetadata", Some(resolve_metadata));

            let declare_meta_async_ref = R3::declare_class_metadata_async();
            let declare_meta_async_expr = external_expr(declare_meta_async_ref);

            Expression::InvokeFn(InvokeFunctionExpr {
                fn_: Box::new(declare_meta_async_expr),
                args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
                type_: None,
                source_span: None,
                pure: false,
            })
        }
    }
}
