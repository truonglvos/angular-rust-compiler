//! Render3 Factory
//!
//! Corresponds to packages/compiler/src/render3/r3_factory.ts
//! Contains factory function generation for Angular

use crate::core::InjectFlags;
use crate::output::output_ast::{
    Expression, Statement, Type, ExternalReference, BinaryOperator, BinaryOperatorExpr,
    ReadVarExpr, InstantiateExpr, InvokeFunctionExpr, DeclareVarStmt, ReturnStatement,
    IfStmt, FunctionExpr, FnParam, ArrowFunctionExpr, ArrowFunctionBody, ExternalExpr,
    LiteralExpr, LiteralValue, LiteralArrayExpr, LiteralMapExpr, LiteralMapEntry, WriteVarExpr,
    ExpressionType, ExpressionStatement, TypeModifier, StmtModifier,
    null_expr, inferred_type, dynamic_type, none_type,
};
use super::r3_identifiers::Identifiers as R3;
use super::util::{R3CompiledExpression, R3Reference, type_with_parameters};

/// Target types for factory generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactoryTarget {
    Directive = 0,
    Component = 1,
    Injectable = 2,
    Pipe = 3,
    NgModule = 4,
}

/// Metadata required by the factory generator
#[derive(Debug, Clone)]
pub struct R3ConstructorFactoryMetadata {
    /// String name of the type being generated (used to name the factory function)
    pub name: String,
    /// An expression representing the interface type being constructed
    pub type_: R3Reference,
    /// Number of arguments for the type
    pub type_argument_count: usize,
    /// Dependencies for the constructor
    pub deps: Option<DepsOrInvalid>,
    /// Type of the target being created by the factory
    pub target: FactoryTarget,
}

/// Dependencies can be valid, invalid (unresolvable), or null (inherited)
#[derive(Debug, Clone)]
pub enum DepsOrInvalid {
    Valid(Vec<R3DependencyMetadata>),
    Invalid,
}

/// Factory delegate type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R3FactoryDelegateType {
    Class = 0,
    Function = 1,
}

/// Metadata for delegated factory
#[derive(Debug, Clone)]
pub struct R3DelegatedFnOrClassMetadata {
    pub base: R3ConstructorFactoryMetadata,
    pub delegate: Expression,
    pub delegate_type: R3FactoryDelegateType,
    pub delegate_deps: Vec<R3DependencyMetadata>,
}

/// Metadata for expression factory
#[derive(Debug, Clone)]
pub struct R3ExpressionFactoryMetadata {
    pub base: R3ConstructorFactoryMetadata,
    pub expression: Expression,
}

/// Union of all factory metadata types
#[derive(Debug, Clone)]
pub enum R3FactoryMetadata {
    Constructor(R3ConstructorFactoryMetadata),
    Delegated(R3DelegatedFnOrClassMetadata),
    Expression(R3ExpressionFactoryMetadata),
}

impl R3FactoryMetadata {
    pub fn base(&self) -> &R3ConstructorFactoryMetadata {
        match self {
            R3FactoryMetadata::Constructor(m) => m,
            R3FactoryMetadata::Delegated(m) => &m.base,
            R3FactoryMetadata::Expression(m) => &m.base,
        }
    }
}

/// Dependency metadata for DI
#[derive(Debug, Clone)]
pub struct R3DependencyMetadata {
    /// Token or value to be injected, or None if unresolvable
    pub token: Option<Expression>,
    /// Literal type of attribute name if @Attribute decorator is present
    pub attribute_name_type: Option<Expression>,
    /// Whether the dependency has an @Host qualifier
    pub host: bool,
    /// Whether the dependency has an @Optional qualifier
    pub optional: bool,
    /// Whether the dependency has an @Self qualifier
    pub self_: bool,
    /// Whether the dependency has an @SkipSelf qualifier
    pub skip_self: bool,
}

/// Construct a factory function expression for the given R3FactoryMetadata
pub fn compile_factory_function(meta: &R3FactoryMetadata) -> R3CompiledExpression {
    let base = meta.base();
    let t = Expression::ReadVar(ReadVarExpr {
        name: "__ngFactoryType__".to_string(),
        type_: None,
        source_span: None,
    });
    let mut base_factory_var: Option<ReadVarExpr> = None;

    // The type to instantiate via constructor invocation
    let type_for_ctor = if !matches!(meta, R3FactoryMetadata::Delegated(_)) {
        Expression::BinaryOp(BinaryOperatorExpr {
            operator: BinaryOperator::Or,
            lhs: Box::new(t.clone()),
            rhs: Box::new(base.type_.value.clone()),
            type_: None,
            source_span: None,
        })
    } else {
        t.clone()
    };

    let mut ctor_expr: Option<Expression> = None;
    if let Some(deps) = &base.deps {
        match deps {
            DepsOrInvalid::Valid(dep_list) => {
                let deps_exprs = inject_dependencies(dep_list, base.target);
                ctor_expr = Some(Expression::Instantiate(InstantiateExpr {
                    class_expr: Box::new(type_for_ctor.clone()),
                    args: deps_exprs,
                    type_: None,
                    source_span: None,
                }));
            }
            DepsOrInvalid::Invalid => {
                // deps is 'invalid', ctor_expr remains None
            }
        }
    } else {
        // No constructor, use base class factory
        let var_name = format!("ɵ{}_BaseFactory", base.name);
        base_factory_var = Some(ReadVarExpr {
            name: var_name.clone(),
            type_: None,
            source_span: None,
        });
        ctor_expr = Some(Expression::InvokeFn(InvokeFunctionExpr {
            fn_: Box::new(Expression::ReadVar(base_factory_var.clone().unwrap())),
            args: vec![type_for_ctor.clone()],
            type_: None,
            source_span: None,
            pure: false,
        }));
    }

    let mut body: Vec<Statement> = Vec::new();
    #[allow(unused_assignments)]
    let mut ret_expr: Option<Expression> = None;

    // Helper to create conditional factory
    fn make_conditional_factory(
        body: &mut Vec<Statement>,
        t: &Expression,
        ctor_expr: &Option<Expression>,
        non_ctor_expr: Expression,
    ) -> Expression {
        let r = ReadVarExpr {
            name: "__ngConditionalFactory__".to_string(),
            type_: None,
            source_span: None,
        };
        
        body.push(Statement::DeclareVar(DeclareVarStmt {
            name: r.name.clone(),
            value: Some(null_expr()),
            type_: Some(inferred_type()),
            modifiers: StmtModifier::None,
            source_span: None,
        }));

        let ctor_stmt = if let Some(ctor) = ctor_expr {
            Statement::Expression(ExpressionStatement {
                expr: Box::new(Expression::WriteVar(WriteVarExpr {
                    name: r.name.clone(),
                    value: Box::new(ctor.clone()),
                    type_: None,
                    source_span: None,
                })),
                source_span: None,
            })
        } else {
            Statement::Expression(ExpressionStatement {
                expr: Box::new(Expression::InvokeFn(InvokeFunctionExpr {
                    fn_: Box::new(Expression::External(ExternalExpr {
                        value: R3::invalid_factory(),
                        type_: None,
                        source_span: None,
                    })),
                    args: vec![],
                    type_: None,
                    source_span: None,
                    pure: false,
                })),
                source_span: None,
            })
        };

        let else_stmt = Statement::Expression(ExpressionStatement {
            expr: Box::new(Expression::WriteVar(WriteVarExpr {
                name: r.name.clone(),
                value: Box::new(non_ctor_expr),
                type_: None,
                source_span: None,
            })),
            source_span: None,
        });

        body.push(Statement::IfStmt(IfStmt {
            condition: Box::new(t.clone()),
            true_case: vec![ctor_stmt],
            false_case: vec![else_stmt],
            source_span: None,
        }));

        Expression::ReadVar(r)
    }

    match meta {
        R3FactoryMetadata::Delegated(delegated_meta) => {
            let delegate_args = inject_dependencies(&delegated_meta.delegate_deps, base.target);
            let factory_expr = if delegated_meta.delegate_type == R3FactoryDelegateType::Class {
                Expression::Instantiate(InstantiateExpr {
                    class_expr: Box::new(delegated_meta.delegate.clone()),
                    args: delegate_args,
                    type_: None,
                    source_span: None,
                })
            } else {
                Expression::InvokeFn(InvokeFunctionExpr {
                    fn_: Box::new(delegated_meta.delegate.clone()),
                    args: delegate_args,
                    type_: None,
                    source_span: None,
                    pure: false,
                })
            };
            ret_expr = Some(make_conditional_factory(&mut body, &t, &ctor_expr, factory_expr));
        }
        R3FactoryMetadata::Expression(expr_meta) => {
            ret_expr = Some(make_conditional_factory(
                &mut body,
                &t,
                &ctor_expr,
                expr_meta.expression.clone(),
            ));
        }
        R3FactoryMetadata::Constructor(_) => {
            ret_expr = ctor_expr;
        }
    }

    if ret_expr.is_none() {
        // Cannot form expression, render invalidFactory() call
        body.push(Statement::Expression(ExpressionStatement {
            expr: Box::new(Expression::InvokeFn(InvokeFunctionExpr {
                fn_: Box::new(Expression::External(ExternalExpr {
                    value: R3::invalid_factory(),
                    type_: None,
                    source_span: None,
                })),
                args: vec![],
                type_: None,
                source_span: None,
                pure: false,
            })),
            source_span: None,
        }));
    } else if let Some(ref base_var) = base_factory_var {
        // Uses base factory
        let get_inherited_factory_call = Expression::InvokeFn(InvokeFunctionExpr {
            fn_: Box::new(Expression::External(ExternalExpr {
                value: R3::get_inherited_factory(),
                type_: None,
                source_span: None,
            })),
            args: vec![base.type_.value.clone()],
            type_: None,
            source_span: None,
            pure: false,
        });

        // Memoize: baseFactory || (baseFactory = getInheritedFactory(...))
        let base_factory = Expression::BinaryOp(BinaryOperatorExpr {
            operator: BinaryOperator::Or,
            lhs: Box::new(Expression::ReadVar(base_var.clone())),
            rhs: Box::new(Expression::WriteVar(WriteVarExpr {
                name: base_var.name.clone(),
                value: Box::new(get_inherited_factory_call),
                type_: None,
                source_span: None,
            })),
            type_: None,
            source_span: None,
        });

        body.push(Statement::Return(ReturnStatement {
            value: Box::new(Expression::InvokeFn(InvokeFunctionExpr {
                fn_: Box::new(base_factory),
                args: vec![type_for_ctor.clone()],
                type_: None,
                source_span: None,
                pure: false,
            })),
            source_span: None,
        }));
    } else if let Some(ret) = ret_expr {
        // Straightforward factory
        body.push(Statement::Return(ReturnStatement {
            value: Box::new(ret),
            source_span: None,
        }));
    }

    let mut factory_fn: Expression = Expression::Fn(FunctionExpr {
        params: vec![FnParam {
            name: "__ngFactoryType__".to_string(),
            type_: Some(dynamic_type()),
        }],
        statements: body,
        type_: Some(inferred_type()),
        source_span: None,
        name: Some(format!("{}_Factory", base.name)),
    });

    if let Some(ref base_var) = base_factory_var {
        // Wrap with IIFE
        factory_fn = Expression::InvokeFn(InvokeFunctionExpr {
            fn_: Box::new(Expression::ArrowFn(ArrowFunctionExpr {
                params: vec![],
                body: ArrowFunctionBody::Statements(vec![
                    Statement::DeclareVar(DeclareVarStmt {
                        name: base_var.name.clone(),
                        value: None,
                        type_: None,
                        modifiers: StmtModifier::None,
                        source_span: None,
                    }),
                    Statement::Return(ReturnStatement {
                        value: Box::new(factory_fn),
                        source_span: None,
                    }),
                ]),
                type_: None,
                source_span: None,
            })),
            args: vec![],
            type_: None,
            source_span: None,
            pure: true,
        });
    }

    R3CompiledExpression::new(factory_fn, create_factory_type(meta), vec![])
}

/// Create the type for the factory function
pub fn create_factory_type(meta: &R3FactoryMetadata) -> Type {
    let base = meta.base();
    let ctor_deps_type = if let Some(DepsOrInvalid::Valid(deps)) = &base.deps {
        create_ctor_deps_type(deps)
    } else {
        none_type()
    };

    Type::Expression(ExpressionType {
        value: Box::new(Expression::External(ExternalExpr {
            value: R3::factory_declaration(),
            type_: None,
            source_span: None,
        })),
        modifiers: TypeModifier::None,
        type_params: Some(vec![
            type_with_parameters(base.type_.type_expr.clone(), base.type_argument_count),
            ctor_deps_type,
        ]),
    })
}

fn inject_dependencies(deps: &[R3DependencyMetadata], target: FactoryTarget) -> Vec<Expression> {
    deps.iter()
        .enumerate()
        .map(|(index, dep)| compile_inject_dependency(dep, target, index))
        .collect()
}

fn compile_inject_dependency(
    dep: &R3DependencyMetadata,
    target: FactoryTarget,
    index: usize,
) -> Expression {
    if dep.token.is_none() {
        // Invalid dependency
        return Expression::InvokeFn(InvokeFunctionExpr {
            fn_: Box::new(Expression::External(ExternalExpr {
                value: R3::invalid_factory_dep(),
                type_: None,
                source_span: None,
            })),
            args: vec![Expression::Literal(LiteralExpr {
                value: LiteralValue::Number(index as f64),
                type_: None,
                source_span: None,
            })],
            type_: None,
            source_span: None,
            pure: false,
        });
    }

    let token = dep.token.clone().unwrap();

    if dep.attribute_name_type.is_none() {
        // Build injection flags
        let mut flags = InjectFlags::Default as u32;
        if dep.self_ {
            flags |= InjectFlags::Self_ as u32;
        }
        if dep.skip_self {
            flags |= InjectFlags::SkipSelf as u32;
        }
        if dep.host {
            flags |= InjectFlags::Host as u32;
        }
        if dep.optional {
            flags |= InjectFlags::Optional as u32;
        }

        let mut inject_args = vec![token];
        if flags != InjectFlags::Default as u32 || dep.optional {
            inject_args.push(Expression::Literal(LiteralExpr {
                value: LiteralValue::Number(flags as f64),
                type_: None,
                source_span: None,
            }));
        }

        let inject_fn = get_inject_fn(target);
        Expression::InvokeFn(InvokeFunctionExpr {
            fn_: Box::new(Expression::External(ExternalExpr {
                value: inject_fn,
                type_: None,
                source_span: None,
            })),
            args: inject_args,
            type_: None,
            source_span: None,
            pure: false,
        })
    } else {
        // @Attribute() dependency
        Expression::InvokeFn(InvokeFunctionExpr {
            fn_: Box::new(Expression::External(ExternalExpr {
                value: R3::inject_attribute(),
                type_: None,
                source_span: None,
            })),
            args: vec![token],
            type_: None,
            source_span: None,
            pure: false,
        })
    }
}

fn create_ctor_deps_type(deps: &[R3DependencyMetadata]) -> Type {
    let mut has_types = false;
    let attribute_types: Vec<Expression> = deps
        .iter()
        .map(|dep| {
            if let Some(type_expr) = create_ctor_dep_type(dep) {
                has_types = true;
                type_expr
            } else {
                Expression::Literal(LiteralExpr {
                    value: LiteralValue::Null,
                    type_: None,
                    source_span: None,
                })
            }
        })
        .collect();

    if has_types {
        Type::Expression(ExpressionType {
            value: Box::new(Expression::LiteralArray(LiteralArrayExpr {
                entries: attribute_types,
                type_: None,
                source_span: None,
            })),
            modifiers: TypeModifier::None,
            type_params: None,
        })
    } else {
        none_type()
    }
}

fn create_ctor_dep_type(dep: &R3DependencyMetadata) -> Option<Expression> {
    let mut entries: Vec<LiteralMapEntry> = Vec::new();

    if let Some(ref attr_type) = dep.attribute_name_type {
        entries.push(LiteralMapEntry {
            key: "attribute".to_string(),
            value: Box::new(attr_type.clone()),
            quoted: false,
        });
    }
    if dep.optional {
        entries.push(LiteralMapEntry {
            key: "optional".to_string(),
            value: Box::new(Expression::Literal(LiteralExpr {
                value: LiteralValue::Bool(true),
                type_: None,
                source_span: None,
            })),
            quoted: false,
        });
    }
    if dep.host {
        entries.push(LiteralMapEntry {
            key: "host".to_string(),
            value: Box::new(Expression::Literal(LiteralExpr {
                value: LiteralValue::Bool(true),
                type_: None,
                source_span: None,
            })),
            quoted: false,
        });
    }
    if dep.self_ {
        entries.push(LiteralMapEntry {
            key: "self".to_string(),
            value: Box::new(Expression::Literal(LiteralExpr {
                value: LiteralValue::Bool(true),
                type_: None,
                source_span: None,
            })),
            quoted: false,
        });
    }
    if dep.skip_self {
        entries.push(LiteralMapEntry {
            key: "skipSelf".to_string(),
            value: Box::new(Expression::Literal(LiteralExpr {
                value: LiteralValue::Bool(true),
                type_: None,
                source_span: None,
            })),
            quoted: false,
        });
    }

    if !entries.is_empty() {
        Some(Expression::LiteralMap(LiteralMapExpr {
            entries,
            type_: None,
            source_span: None,
        }))
    } else {
        None
    }
}

/// Check if metadata is delegated factory
pub fn is_delegated_factory_metadata(meta: &R3FactoryMetadata) -> bool {
    matches!(meta, R3FactoryMetadata::Delegated(_))
}

/// Check if metadata is expression factory
pub fn is_expression_factory_metadata(meta: &R3FactoryMetadata) -> bool {
    matches!(meta, R3FactoryMetadata::Expression(_))
}

fn get_inject_fn(target: FactoryTarget) -> ExternalReference {
    match target {
        FactoryTarget::Component | FactoryTarget::Directive | FactoryTarget::Pipe => {
            R3::directive_inject()
        }
        FactoryTarget::NgModule | FactoryTarget::Injectable => R3::inject(),
    }
}
