//! Render3 Utilities
//!
//! Corresponds to packages/compiler/src/render3/util.ts
//! Contains utility functions for render3 compilation

use super::r3_identifiers::Identifiers;
use crate::output::abstract_emitter::escape_identifier;
use crate::output::output_ast::{
    ArrowFunctionBody, ArrowFunctionExpr, BinaryOperator, BinaryOperatorExpr, BuiltinType,
    BuiltinTypeName, Expression, ExpressionType, ExternalExpr, ExternalReference,
    InvokeFunctionExpr, LiteralArrayExpr, LiteralExpr, LiteralValue, Statement, Type, TypeModifier,
    TypeofExpr, WrappedNodeExpr,
};

/// Creates an expression type with the given number of type parameters
pub fn type_with_parameters(type_expr: Expression, num_params: usize) -> Type {
    if num_params == 0 {
        return Type::Expression(ExpressionType {
            value: Box::new(type_expr),
            modifiers: TypeModifier::None,
            type_params: None,
        });
    }
    let params: Vec<Type> = (0..num_params)
        .map(|_| {
            Type::Builtin(BuiltinType {
                name: BuiltinTypeName::Dynamic,
                modifiers: TypeModifier::None,
            })
        })
        .collect();
    Type::Expression(ExpressionType {
        value: Box::new(type_expr),
        modifiers: TypeModifier::None,
        type_params: Some(params),
    })
}

/// Reference containing both value and type expressions
#[derive(Debug, Clone)]
pub struct R3Reference {
    pub value: Expression,
    pub type_expr: Expression,
}

impl R3Reference {
    pub fn new(value: Expression, type_expr: Expression) -> Self {
        R3Reference { value, type_expr }
    }
}

/// Result of compilation of a render3 code unit (component, directive, pipe, etc.)
#[derive(Debug, Clone)]
pub struct R3CompiledExpression {
    pub expression: Expression,
    pub type_: Type,
    pub statements: Vec<Statement>,
}

impl R3CompiledExpression {
    pub fn new(expression: Expression, type_: Type, statements: Vec<Statement>) -> Self {
        R3CompiledExpression {
            expression,
            type_,
            statements,
        }
    }
}

const LEGACY_ANIMATE_SYMBOL_PREFIX: &str = "@";

/// Prepares a synthetic property name for animations
pub fn prepare_synthetic_property_name(name: &str) -> String {
    format!("{}{}", LEGACY_ANIMATE_SYMBOL_PREFIX, name)
}

/// Prepares a synthetic listener name for animations
pub fn prepare_synthetic_listener_name(name: &str, phase: &str) -> String {
    format!("{}{}.{}", LEGACY_ANIMATE_SYMBOL_PREFIX, name, phase)
}

/// Gets a safe property access string
pub fn get_safe_property_access_string(accessor: &str, name: &str) -> String {
    let escaped_name = escape_identifier(name, false, false);
    if escaped_name != name {
        format!("{}[{}]", accessor, escaped_name)
    } else {
        format!("{}.{}", accessor, name)
    }
}

/// Prepares a synthetic listener function name for animations
pub fn prepare_synthetic_listener_function_name(name: &str, phase: &str) -> String {
    format!("animation_{}_{}", name, phase)
}

/// Wraps an expression in a JIT-only guard
pub fn jit_only_guarded_expression(expr: Expression) -> Expression {
    guarded_expression("ngJitMode", expr)
}

/// Wraps an expression in a dev-only guard
pub fn dev_only_guarded_expression(expr: Expression) -> Expression {
    guarded_expression("ngDevMode", expr)
}

/// Wraps an expression in a guard check
pub fn guarded_expression(guard: &str, expr: Expression) -> Expression {
    let guard_expr = Expression::External(ExternalExpr {
        value: ExternalReference {
            name: Some(guard.to_string()),
            module_name: None,
            runtime: None,
        },
        type_: None,
        source_span: None,
    });

    let guard_not_defined = Expression::BinaryOp(BinaryOperatorExpr {
        operator: BinaryOperator::Identical,
        lhs: Box::new(Expression::TypeOf(TypeofExpr {
            expr: Box::new(guard_expr.clone()),
            type_: None,
            source_span: None,
        })),
        rhs: Box::new(Expression::Literal(LiteralExpr {
            value: LiteralValue::String("undefined".to_string()),
            type_: None,
            source_span: None,
        })),
        type_: None,
        source_span: None,
    });

    let guard_undefined_or_true = Expression::BinaryOp(BinaryOperatorExpr {
        operator: BinaryOperator::Or,
        lhs: Box::new(guard_not_defined),
        rhs: Box::new(guard_expr),
        type_: None,
        source_span: None,
    });

    Expression::BinaryOp(BinaryOperatorExpr {
        operator: BinaryOperator::And,
        lhs: Box::new(guard_undefined_or_true),
        rhs: Box::new(expr),
        type_: None,
        source_span: None,
    })
}

/// Wraps a value in an R3Reference
pub fn wrap_reference<T: 'static>(value: T) -> R3Reference {
    let wrapped = Expression::WrappedNode(WrappedNodeExpr {
        node: Box::new(value),
        type_: None,
        source_span: None,
    });
    R3Reference {
        value: wrapped.clone(),
        type_expr: wrapped,
    }
}

/// Converts references to an array expression
pub fn refs_to_array(refs: &[R3Reference], should_forward_declare: bool) -> Expression {
    let values: Vec<Expression> = refs.iter().map(|r| r.value.clone()).collect();
    let literal_arr = Expression::LiteralArray(LiteralArrayExpr {
        entries: values,
        type_: None,
        source_span: None,
    });

    if should_forward_declare {
        Expression::ArrowFn(ArrowFunctionExpr {
            params: vec![],
            body: ArrowFunctionBody::Expression(Box::new(literal_arr)),
            type_: None,
            source_span: None,
        })
    } else {
        literal_arr
    }
}

/// Specifies how a forward ref has been handled in a MaybeForwardRefExpression
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForwardRefHandling {
    /// The expression was not wrapped in a `forwardRef()` call in the first place.
    None,
    /// The expression is still wrapped in a `forwardRef()` call.
    Wrapped,
    /// The expression was wrapped in a `forwardRef()` call but has since been unwrapped.
    Unwrapped,
}

/// Describes an expression that may have been wrapped in a `forwardRef()` guard.
#[derive(Debug, Clone)]
pub struct MaybeForwardRefExpression {
    /// The unwrapped expression.
    pub expression: Expression,
    /// Specified whether the `expression` contains a reference to something that has not yet been
    /// defined, and whether the expression is still wrapped in a `forwardRef()` call.
    pub forward_ref: ForwardRefHandling,
}

impl MaybeForwardRefExpression {
    pub fn new(expression: Expression, forward_ref: ForwardRefHandling) -> Self {
        MaybeForwardRefExpression {
            expression,
            forward_ref,
        }
    }
}

/// Creates a MaybeForwardRefExpression
pub fn create_may_be_forward_ref_expression(
    expression: Expression,
    forward_ref: ForwardRefHandling,
) -> MaybeForwardRefExpression {
    MaybeForwardRefExpression::new(expression, forward_ref)
}

/// Convert a `MaybeForwardRefExpression` to an `Expression`, possibly wrapping its expression in a
/// `forwardRef()` call.
pub fn convert_from_maybe_forward_ref_expression(
    maybe_forward_ref: &MaybeForwardRefExpression,
) -> Expression {
    match maybe_forward_ref.forward_ref {
        ForwardRefHandling::None | ForwardRefHandling::Wrapped => {
            maybe_forward_ref.expression.clone()
        }
        ForwardRefHandling::Unwrapped => generate_forward_ref(maybe_forward_ref.expression.clone()),
    }
}

/// Generate an expression that has the given `expr` wrapped in the following form:
/// ```ts
/// forwardRef(() => expr)
/// ```
pub fn generate_forward_ref(expr: Expression) -> Expression {
    let forward_ref_import = Expression::External(ExternalExpr {
        value: Identifiers::forward_ref(),
        type_: None,
        source_span: None,
    });

    let arrow_fn = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![],
        body: ArrowFunctionBody::Expression(Box::new(expr)),
        type_: None,
        source_span: None,
    });

    Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(forward_ref_import),
        args: vec![arrow_fn],
        type_: None,
        source_span: None,
        pure: false,
    })
}
