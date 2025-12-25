//! Render3 Module Compiler
//!
//! Corresponds to packages/compiler/src/render3/r3_module_compiler.ts
//! Contains NgModule compilation logic

use crate::output::output_ast::{
    BuiltinType, BuiltinTypeName, Expression, ExpressionType, ExternalExpr, FunctionExpr,
    InvokeFunctionExpr, LiteralArrayExpr, Statement, Type, TypeModifier, TypeofExpr,
};

use super::r3_identifiers::Identifiers as R3;
use super::util::{jit_only_guarded_expression, refs_to_array, R3CompiledExpression, R3Reference};
use super::view::util::DefinitionMap;

/// Helper to create external expression from ExternalReference
fn external_expr(reference: crate::output::output_ast::ExternalReference) -> Expression {
    Expression::External(ExternalExpr {
        value: reference,
        type_: None,
        source_span: None,
    })
}

/// How the selector scope of an NgModule should be emitted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R3SelectorScopeMode {
    /// Emit the declarations inline into the module definition.
    Inline,
    /// Emit using side effectful function call, guarded by ngJitMode flag.
    SideEffect,
    /// Don't generate selector scopes at all.
    Omit,
}

/// The type of NgModule metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R3NgModuleMetadataKind {
    Global,
    Local,
}

/// Common metadata for NgModule
#[derive(Debug, Clone)]
pub struct R3NgModuleMetadataCommon {
    pub kind: R3NgModuleMetadataKind,
    pub type_: R3Reference,
    pub selector_scope_mode: R3SelectorScopeMode,
    pub schemas: Option<Vec<R3Reference>>,
    pub id: Option<Expression>,
}

/// Metadata for full/partial compilation mode
#[derive(Debug, Clone)]
pub struct R3NgModuleMetadataGlobal {
    pub common: R3NgModuleMetadataCommon,
    pub bootstrap: Vec<R3Reference>,
    pub declarations: Vec<R3Reference>,
    pub public_declaration_types: Option<Vec<Expression>>,
    pub imports: Vec<R3Reference>,
    pub include_import_types: bool,
    pub exports: Vec<R3Reference>,
    pub contains_forward_decls: bool,
}

/// Metadata for local compilation mode
#[derive(Debug, Clone)]
pub struct R3NgModuleMetadataLocal {
    pub common: R3NgModuleMetadataCommon,
    pub bootstrap_expression: Option<Expression>,
    pub declarations_expression: Option<Expression>,
    pub imports_expression: Option<Expression>,
    pub exports_expression: Option<Expression>,
}

/// Combined NgModule metadata
#[derive(Debug, Clone)]
pub enum R3NgModuleMetadata {
    Global(R3NgModuleMetadataGlobal),
    Local(R3NgModuleMetadataLocal),
}

impl R3NgModuleMetadata {
    pub fn kind(&self) -> R3NgModuleMetadataKind {
        match self {
            R3NgModuleMetadata::Global(g) => g.common.kind,
            R3NgModuleMetadata::Local(l) => l.common.kind,
        }
    }

    pub fn type_(&self) -> &R3Reference {
        match self {
            R3NgModuleMetadata::Global(g) => &g.common.type_,
            R3NgModuleMetadata::Local(l) => &l.common.type_,
        }
    }

    pub fn selector_scope_mode(&self) -> R3SelectorScopeMode {
        match self {
            R3NgModuleMetadata::Global(g) => g.common.selector_scope_mode,
            R3NgModuleMetadata::Local(l) => l.common.selector_scope_mode,
        }
    }

    pub fn schemas(&self) -> &Option<Vec<R3Reference>> {
        match self {
            R3NgModuleMetadata::Global(g) => &g.common.schemas,
            R3NgModuleMetadata::Local(l) => &l.common.schemas,
        }
    }

    pub fn id(&self) -> &Option<Expression> {
        match self {
            R3NgModuleMetadata::Global(g) => &g.common.id,
            R3NgModuleMetadata::Local(l) => &l.common.id,
        }
    }
}

/// Construct an R3NgModuleDef for the given R3NgModuleMetadata
pub fn compile_ng_module(meta: &R3NgModuleMetadata) -> R3CompiledExpression {
    let mut statements: Vec<Statement> = vec![];
    let mut definition_map = DefinitionMap::new();

    // type
    definition_map.set("type", Some(meta.type_().value.clone()));

    // bootstrap (Global mode only)
    if let R3NgModuleMetadata::Global(global) = meta {
        if !global.bootstrap.is_empty() {
            definition_map.set(
                "bootstrap",
                Some(refs_to_array(
                    &global.bootstrap,
                    global.contains_forward_decls,
                )),
            );
        }
    }

    // Handle selector scope mode
    match meta.selector_scope_mode() {
        R3SelectorScopeMode::Inline => {
            if let R3NgModuleMetadata::Global(global) = meta {
                if !global.declarations.is_empty() {
                    definition_map.set(
                        "declarations",
                        Some(refs_to_array(
                            &global.declarations,
                            global.contains_forward_decls,
                        )),
                    );
                }
                if !global.imports.is_empty() {
                    definition_map.set(
                        "imports",
                        Some(refs_to_array(
                            &global.imports,
                            global.contains_forward_decls,
                        )),
                    );
                }
                if !global.exports.is_empty() {
                    definition_map.set(
                        "exports",
                        Some(refs_to_array(
                            &global.exports,
                            global.contains_forward_decls,
                        )),
                    );
                }
            }
        }
        R3SelectorScopeMode::SideEffect => {
            if let Some(set_ng_module_scope_call) = generate_set_ng_module_scope_call(meta) {
                statements.push(set_ng_module_scope_call);
            }
        }
        R3SelectorScopeMode::Omit => {
            // Skip selector scope
        }
    }

    // schemas
    if let Some(schemas) = meta.schemas() {
        if !schemas.is_empty() {
            let schema_exprs: Vec<Expression> = schemas.iter().map(|r| r.value.clone()).collect();
            definition_map.set(
                "schemas",
                Some(Expression::LiteralArray(LiteralArrayExpr {
                    entries: schema_exprs,
                    type_: None,
                    source_span: None,
                })),
            );
        }
    }

    // id
    if let Some(id) = meta.id() {
        definition_map.set("id", Some(id.clone()));

        // Generate side-effectful call to register NgModule by id
        let register_ng_module_type_ref = R3::register_ng_module_type();
        let register_ng_module_type_expr = external_expr(register_ng_module_type_ref);

        statements.push(
            Expression::InvokeFn(InvokeFunctionExpr {
                fn_: Box::new(register_ng_module_type_expr),
                args: vec![meta.type_().value.clone(), id.clone()],
                type_: None,
                source_span: None,
                pure: false,
            })
            .to_stmt(),
        );
    }

    let define_ng_module_ref = R3::define_ng_module();
    let define_ng_module_expr = external_expr(define_ng_module_ref);

    let expression = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(define_ng_module_expr),
        args: vec![Expression::LiteralMap(definition_map.to_literal_map())],
        type_: None,
        source_span: None,
        pure: true,
    });

    let type_ = create_ng_module_type(meta);

    R3CompiledExpression::new(expression, type_, statements)
}

/// Creates the type for an NgModule
pub fn create_ng_module_type(meta: &R3NgModuleMetadata) -> Type {
    if matches!(meta.kind(), R3NgModuleMetadataKind::Local) {
        return expression_type(meta.type_().value.clone());
    }

    if let R3NgModuleMetadata::Global(global) = meta {
        let module_type = &global.common.type_;
        let declarations = &global.declarations;
        let exports = &global.exports;
        let imports = &global.imports;
        let include_import_types = global.include_import_types;
        let public_declaration_types = &global.public_declaration_types;

        let type_params: Vec<Type> = vec![
            expression_type(module_type.type_expr.clone()),
            if let Some(pub_types) = public_declaration_types {
                tuple_of_types(pub_types)
            } else {
                tuple_type_of(declarations)
            },
            if include_import_types {
                tuple_type_of(imports)
            } else {
                none_type()
            },
            tuple_type_of(exports),
        ];

        let ng_module_declaration_ref = R3::ng_module_declaration();
        let ng_module_declaration_expr = external_expr(ng_module_declaration_ref);

        return Type::Expression(ExpressionType {
            value: Box::new(ng_module_declaration_expr),
            modifiers: crate::output::output_ast::TypeModifier::None,
            type_params: Some(type_params),
        });
    }

    expression_type(meta.type_().value.clone())
}

/// Generate set_ng_module_scope call
fn generate_set_ng_module_scope_call(meta: &R3NgModuleMetadata) -> Option<Statement> {
    let mut scope_map = DefinitionMap::new();

    match meta {
        R3NgModuleMetadata::Global(global) => {
            if !global.declarations.is_empty() {
                scope_map.set(
                    "declarations",
                    Some(refs_to_array(
                        &global.declarations,
                        global.contains_forward_decls,
                    )),
                );
            }
            if !global.imports.is_empty() {
                scope_map.set(
                    "imports",
                    Some(refs_to_array(
                        &global.imports,
                        global.contains_forward_decls,
                    )),
                );
            }
            if !global.exports.is_empty() {
                scope_map.set(
                    "exports",
                    Some(refs_to_array(
                        &global.exports,
                        global.contains_forward_decls,
                    )),
                );
            }
        }
        R3NgModuleMetadata::Local(local) => {
            if let Some(ref decl_expr) = local.declarations_expression {
                scope_map.set("declarations", Some(decl_expr.clone()));
            }
            if let Some(ref imports_expr) = local.imports_expression {
                scope_map.set("imports", Some(imports_expr.clone()));
            }
            if let Some(ref exports_expr) = local.exports_expression {
                scope_map.set("exports", Some(exports_expr.clone()));
            }
            if let Some(ref bootstrap_expr) = local.bootstrap_expression {
                scope_map.set("bootstrap", Some(bootstrap_expr.clone()));
            }
        }
    }

    if scope_map.values.is_empty() {
        return None;
    }

    // setNgModuleScope(...)
    let set_ng_module_scope_ref = R3::set_ng_module_scope();
    let set_ng_module_scope_expr = external_expr(set_ng_module_scope_ref);

    let fn_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(set_ng_module_scope_expr),
        args: vec![
            meta.type_().value.clone(),
            Expression::LiteralMap(scope_map.to_literal_map()),
        ],
        type_: None,
        source_span: None,
        pure: false,
    });

    // (ngJitMode guard) && setNgModuleScope(...)
    let guarded_call = jit_only_guarded_expression(fn_call);

    // function() { (ngJitMode guard) && setNgModuleScope(...); }
    let iife = Expression::Fn(FunctionExpr {
        params: vec![],
        statements: vec![guarded_call.to_stmt()],
        type_: None,
        source_span: None,
        name: None,
    });

    // (function() { ... })()
    let iife_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(iife),
        args: vec![],
        type_: None,
        source_span: None,
        pure: false,
    });

    Some(iife_call.to_stmt())
}

fn none_type() -> Type {
    Type::Builtin(BuiltinType {
        name: BuiltinTypeName::None,
        modifiers: TypeModifier::None,
    })
}

fn expression_type(expr: Expression) -> Type {
    Type::Expression(ExpressionType {
        value: Box::new(expr),
        modifiers: TypeModifier::None,
        type_params: None,
    })
}

fn tuple_type_of(refs: &[R3Reference]) -> Type {
    if refs.is_empty() {
        return none_type();
    }
    let types: Vec<Expression> = refs
        .iter()
        .map(|r| {
            Expression::TypeOf(TypeofExpr {
                expr: Box::new(r.type_expr.clone()),
                type_: None,
                source_span: None,
            })
        })
        .collect();
    expression_type(Expression::LiteralArray(LiteralArrayExpr {
        entries: types,
        type_: None,
        source_span: None,
    }))
}

fn tuple_of_types(types: &[Expression]) -> Type {
    if types.is_empty() {
        return none_type();
    }
    let typeof_types: Vec<Expression> = types
        .iter()
        .map(|t| {
            Expression::TypeOf(TypeofExpr {
                expr: Box::new(t.clone()),
                type_: None,
                source_span: None,
            })
        })
        .collect();
    expression_type(Expression::LiteralArray(LiteralArrayExpr {
        entries: typeof_types,
        type_: None,
        source_span: None,
    }))
}
