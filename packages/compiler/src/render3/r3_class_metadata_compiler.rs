//! Render3 Class Metadata Compiler
//!
//! Corresponds to packages/compiler/src/render3/r3_class_metadata_compiler.ts
//! Contains class metadata compilation for TestBed APIs

use crate::output::output_ast::{
    Expression, ArrowFunctionExpr, ArrowFunctionBody, InvokeFunctionExpr, FnParam,
    LiteralExpr, LiteralValue, LiteralArrayExpr, DynamicImportExpr, ReadPropExpr,
    ReadVarExpr, ExternalExpr,
};
use crate::output::output_ast::dynamic_type;

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

/// Metadata of a class which captures the original Angular decorators
#[derive(Debug, Clone)]
pub struct R3ClassMetadata {
    /// The class type for which the metadata is captured
    pub type_: Expression,
    /// An expression representing the Angular decorators applied on the class
    pub decorators: Expression,
    /// An expression representing the Angular decorators applied to constructor parameters
    pub ctor_parameters: Option<Expression>,
    /// An expression representing the Angular decorators applied on properties
    pub prop_decorators: Option<Expression>,
}

/// Dependency information for deferred loading
#[derive(Debug, Clone)]
pub struct R3DeferPerComponentDependency {
    pub symbol_name: String,
    pub import_path: String,
    pub is_default_import: bool,
}

/// Compile class metadata
pub fn compile_class_metadata(metadata: &R3ClassMetadata) -> Expression {
    let fn_call = internal_compile_class_metadata(metadata);
    
    // Wrap in arrow function with devOnlyGuardedExpression
    let guarded = dev_only_guarded_expression(Expression::InvokeFn(fn_call));
    let arrow = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![],
        body: ArrowFunctionBody::Statements(vec![guarded.to_stmt()]),
        type_: None,
        source_span: None,
    });
    
    Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(arrow),
        args: vec![],
        type_: None,
        source_span: None,
        pure: false,
    })
}

/// Compiles only the `setClassMetadata` call without any additional wrappers
fn internal_compile_class_metadata(metadata: &R3ClassMetadata) -> InvokeFunctionExpr {
    let set_class_metadata_ref = R3::set_class_metadata();
    let set_class_metadata_expr = external_expr(set_class_metadata_ref);
    
    InvokeFunctionExpr {
        fn_: Box::new(set_class_metadata_expr),
        args: vec![
            metadata.type_.clone(),
            metadata.decorators.clone(),
            metadata.ctor_parameters.clone().unwrap_or_else(|| literal(LiteralValue::Null)),
            metadata.prop_decorators.clone().unwrap_or_else(|| literal(LiteralValue::Null)),
        ],
        type_: None,
        source_span: None,
        pure: false,
    }
}

/// Compile component class metadata with deferred dependencies
pub fn compile_component_class_metadata(
    metadata: &R3ClassMetadata,
    dependencies: Option<&[R3DeferPerComponentDependency]>,
) -> Expression {
    match dependencies {
        None | Some(&[]) => compile_class_metadata(metadata),
        Some(deps) => {
            let wrapper_params: Vec<FnParam> = deps
                .iter()
                .map(|dep| FnParam {
                    name: dep.symbol_name.clone(),
                    type_: Some(dynamic_type()),
                })
                .collect();
            let resolver = compile_component_metadata_async_resolver(deps);
            internal_compile_set_class_metadata_async(metadata, wrapper_params, Expression::ArrowFn(resolver))
        }
    }
}

/// Compile opaque async class metadata
pub fn compile_opaque_async_class_metadata(
    metadata: &R3ClassMetadata,
    defer_resolver: Expression,
    deferred_dependency_names: &[String],
) -> Expression {
    let wrapper_params: Vec<FnParam> = deferred_dependency_names
        .iter()
        .map(|name| FnParam {
            name: name.clone(),
            type_: Some(dynamic_type()),
        })
        .collect();
    internal_compile_set_class_metadata_async(metadata, wrapper_params, defer_resolver)
}

/// Internal logic to compile a `setClassMetadataAsync` call
fn internal_compile_set_class_metadata_async(
    metadata: &R3ClassMetadata,
    wrapper_params: Vec<FnParam>,
    dependency_resolver_fn: Expression,
) -> Expression {
    let set_class_metadata_call = internal_compile_class_metadata(metadata);
    
    let set_class_meta_wrapper = Expression::ArrowFn(ArrowFunctionExpr {
        params: wrapper_params,
        body: ArrowFunctionBody::Statements(vec![Expression::InvokeFn(set_class_metadata_call).to_stmt()]),
        type_: None,
        source_span: None,
    });
    
    let set_class_metadata_async_ref = R3::set_class_metadata_async();
    let set_class_metadata_async_expr = external_expr(set_class_metadata_async_ref);
    
    let set_class_meta_async = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(set_class_metadata_async_expr),
        args: vec![
            metadata.type_.clone(),
            dependency_resolver_fn,
            set_class_meta_wrapper,
        ],
        type_: None,
        source_span: None,
        pure: false,
    });
    
    let guarded = dev_only_guarded_expression(set_class_meta_async);
    let outer_arrow = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![],
        body: ArrowFunctionBody::Statements(vec![guarded.to_stmt()]),
        type_: None,
        source_span: None,
    });
    
    Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(outer_arrow),
        args: vec![],
        type_: None,
        source_span: None,
        pure: false,
    })
}

/// Compiles the function that loads the dependencies for the component
pub fn compile_component_metadata_async_resolver(
    dependencies: &[R3DeferPerComponentDependency],
) -> ArrowFunctionExpr {
    let dynamic_imports: Vec<Expression> = dependencies
        .iter()
        .map(|dep| {
            // e.g. `(m) => m.CmpA` or `(m) => m.default`
            let prop_name = if dep.is_default_import {
                "default".to_string()
            } else {
                dep.symbol_name.clone()
            };
            
            let inner_fn = Expression::ArrowFn(ArrowFunctionExpr {
                params: vec![FnParam {
                    name: "m".to_string(),
                    type_: Some(dynamic_type()),
                }],
                body: ArrowFunctionBody::Expression(Box::new(
                    Expression::ReadProp(ReadPropExpr {
                        receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                            name: "m".to_string(),
                            type_: None,
                            source_span: None,
                        })),
                        name: prop_name,
                        type_: None,
                        source_span: None,
                    })
                )),
                type_: None,
                source_span: None,
            });
            
            // e.g. `import('./cmp-a').then(...)`
            let dynamic_import = Expression::DynamicImport(DynamicImportExpr {
                url: dep.import_path.clone(),
                source_span: None,
            });
            
            Expression::InvokeFn(InvokeFunctionExpr {
                fn_: Box::new(Expression::ReadProp(ReadPropExpr {
                    receiver: Box::new(dynamic_import),
                    name: "then".to_string(),
                    type_: None,
                    source_span: None,
                })),
                args: vec![inner_fn],
                type_: None,
                source_span: None,
                pure: false,
            })
        })
        .collect();
    
    // e.g. `() => [ ... ]`
    ArrowFunctionExpr {
        params: vec![],
        body: ArrowFunctionBody::Expression(Box::new(Expression::LiteralArray(LiteralArrayExpr {
            entries: dynamic_imports,
            type_: None,
            source_span: None,
        }))),
        type_: None,
        source_span: None,
    }
}
