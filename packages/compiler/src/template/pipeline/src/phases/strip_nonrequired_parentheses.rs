//! Strip Non-required Parentheses Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/strip_nonrequired_parentheses.ts
//! In most cases we can drop user added parentheses from expressions. However, in some cases
//! parentheses are needed for the expression to be considered valid JavaScript or for Typescript to
//! generate the correct output.

use crate::output::output_ast::BinaryOperator;
use crate::output::output_ast::{BinaryOperatorExpr, Expression, ParenthesizedExpr};
use crate::template::pipeline::ir;
use crate::template::pipeline::src::compilation::{CompilationUnit, ComponentCompilationJob};
use std::collections::HashSet;

/// In most cases we can drop user added parentheses from expressions. However, in some cases
/// parentheses are needed for the expression to be considered valid JavaScript or for Typescript to
/// generate the correct output. This phases strips all parentheses except in the following
/// situations where they are required:
///
/// 1. Unary operators in the base of an exponentiation expression. For example, `-2 ** 3` is not
///    valid JavaScript, but `(-2) ** 3` is.
///
/// 2. When mixing nullish coalescing (`??`) and logical and/or operators (`&&`, `||`), we need
///    parentheses. For example, `a ?? b && c` is not valid JavaScript, but `a ?? (b && c)` is.
///    Note: Because of the outcome of https://github.com/microsoft/TypeScript/issues/62307
///    We need (for now) to keep parentheses around the `??` operator when it is used with and/or operators.
///    For example, `a ?? b && c` is not valid JavaScript, but `(a ?? b) && c` is.
///
/// 3. Ternary expression used as an operand for nullish coalescing. Typescript generates incorrect
///    code if the parentheses are missing. For example when `(a ? b : c) ?? d` is translated to
///    typescript AST, the parentheses node is removed, and then the remaining AST is printed, it
///    incorrectly prints `a ? b : c ?? d`. This is different from how it handles the same situation
///    with `||` and `&&` where it prints the parentheses even if they are not present in the AST.
pub fn strip_nonrequired_parentheses(job: &mut ComponentCompilationJob) {
    // Check which parentheses are required.
    let mut required_parens: HashSet<*const ParenthesizedExpr> = HashSet::new();

    // Process root view
    {
        let unit = &mut job.root;
        collect_required_parens_in_unit(unit, &mut required_parens);
    }

    // Process all other views
    for (_, unit) in job.views.iter_mut() {
        collect_required_parens_in_unit(unit, &mut required_parens);
    }

    // Remove any non-required parentheses.
    {
        let unit = &mut job.root;
        strip_nonrequired_parens_in_unit(unit, &required_parens);
    }

    for (_, unit) in job.views.iter_mut() {
        strip_nonrequired_parens_in_unit(unit, &required_parens);
    }
}

fn collect_required_parens_in_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    required_parens: &mut HashSet<*const ParenthesizedExpr>,
) {
    use crate::template::pipeline::ir::expression::visit_expressions_in_op;

    // Process create ops
    for op in unit.create_mut().iter_mut() {
        visit_expressions_in_op(op.as_mut(), &mut |expr: &Expression, _flags| {
            check_nested_expressions(expr, required_parens);
        });
    }

    // Process update ops
    for op in unit.update_mut().iter_mut() {
        visit_expressions_in_op(op.as_mut(), &mut |expr: &Expression, _flags| {
            check_nested_expressions(expr, required_parens);
        });
    }

    // Process nested operations (listeners, trackByOps)
    for op in unit.create_mut().iter_mut() {
        match op.kind() {
            ir::OpKind::Listener => {
                use crate::template::pipeline::ir::ops::create::ListenerOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let listener_ptr = op_ptr as *mut ListenerOp;
                    let listener = &mut *listener_ptr;

                    for handler_op in listener.handler_ops.iter_mut() {
                        visit_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: &Expression, _flags| {
                                check_nested_expressions(expr, required_parens);
                            },
                        );
                    }
                }
            }
            ir::OpKind::TwoWayListener => {
                use crate::template::pipeline::ir::ops::create::TwoWayListenerOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let two_way_ptr = op_ptr as *mut TwoWayListenerOp;
                    let two_way = &mut *two_way_ptr;

                    for handler_op in two_way.handler_ops.iter_mut() {
                        visit_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: &Expression, _flags| {
                                check_nested_expressions(expr, required_parens);
                            },
                        );
                    }
                }
            }
            ir::OpKind::Animation => {
                use crate::template::pipeline::ir::ops::create::AnimationOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let anim_ptr = op_ptr as *mut AnimationOp;
                    let anim = &mut *anim_ptr;

                    for handler_op in anim.handler_ops.iter_mut() {
                        visit_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: &Expression, _flags| {
                                check_nested_expressions(expr, required_parens);
                            },
                        );
                    }
                }
            }
            ir::OpKind::AnimationListener => {
                use crate::template::pipeline::ir::ops::create::AnimationListenerOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let anim_listener_ptr = op_ptr as *mut AnimationListenerOp;
                    let anim_listener = &mut *anim_listener_ptr;

                    for handler_op in anim_listener.handler_ops.iter_mut() {
                        visit_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: &Expression, _flags| {
                                check_nested_expressions(expr, required_parens);
                            },
                        );
                    }
                }
            }
            ir::OpKind::RepeaterCreate => {
                use crate::template::pipeline::ir::ops::create::RepeaterCreateOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let repeater_ptr = op_ptr as *mut RepeaterCreateOp;
                    let repeater = &mut *repeater_ptr;

                    if let Some(ref mut track_by_ops) = repeater.track_by_ops {
                        for track_by_op in track_by_ops.iter_mut() {
                            visit_expressions_in_op(
                                track_by_op.as_mut(),
                                &mut |expr: &Expression, _flags| {
                                    check_nested_expressions(expr, required_parens);
                                },
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn strip_nonrequired_parens_in_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    required_parens: &HashSet<*const ParenthesizedExpr>,
) {
    use crate::template::pipeline::ir::expression::transform_expressions_in_op;

    // Process create ops
    for op in unit.create_mut().iter_mut() {
        transform_expressions_in_op(
            &mut **op,
            &mut |expr: Expression, _flags| {
                if let Expression::Parens(ref parens_expr) = &expr {
                    let ptr = parens_expr as *const ParenthesizedExpr;
                    if !required_parens.contains(&ptr) {
                        return *parens_expr.expr.clone();
                    }
                }
                expr
            },
            ir::VisitorContextFlag::NONE,
        );
    }

    // Process update ops
    for op in unit.update_mut().iter_mut() {
        transform_expressions_in_op(
            &mut **op,
            &mut |expr: Expression, _flags| {
                if let Expression::Parens(ref parens_expr) = &expr {
                    let ptr = parens_expr as *const ParenthesizedExpr;
                    if !required_parens.contains(&ptr) {
                        return *parens_expr.expr.clone();
                    }
                }
                expr
            },
            ir::VisitorContextFlag::NONE,
        );
    }

    // Process nested operations (listeners, trackByOps)
    for op in unit.create_mut().iter_mut() {
        match op.kind() {
            ir::OpKind::Listener => {
                use crate::template::pipeline::ir::ops::create::ListenerOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let listener_ptr = op_ptr as *mut ListenerOp;
                    let listener = &mut *listener_ptr;

                    for handler_op in listener.handler_ops.iter_mut() {
                        transform_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: Expression, _flags| {
                                if let Expression::Parens(ref parens_expr) = &expr {
                                    let ptr = parens_expr as *const ParenthesizedExpr;
                                    if !required_parens.contains(&ptr) {
                                        return *parens_expr.expr.clone();
                                    }
                                }
                                expr
                            },
                            ir::VisitorContextFlag::NONE,
                        );
                    }
                }
            }
            ir::OpKind::TwoWayListener => {
                use crate::template::pipeline::ir::ops::create::TwoWayListenerOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let two_way_ptr = op_ptr as *mut TwoWayListenerOp;
                    let two_way = &mut *two_way_ptr;

                    for handler_op in two_way.handler_ops.iter_mut() {
                        transform_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: Expression, _flags| {
                                if let Expression::Parens(ref parens_expr) = &expr {
                                    let ptr = parens_expr as *const ParenthesizedExpr;
                                    if !required_parens.contains(&ptr) {
                                        return *parens_expr.expr.clone();
                                    }
                                }
                                expr
                            },
                            ir::VisitorContextFlag::NONE,
                        );
                    }
                }
            }
            ir::OpKind::Animation => {
                use crate::template::pipeline::ir::ops::create::AnimationOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let anim_ptr = op_ptr as *mut AnimationOp;
                    let anim = &mut *anim_ptr;

                    for handler_op in anim.handler_ops.iter_mut() {
                        transform_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: Expression, _flags| {
                                if let Expression::Parens(ref parens_expr) = &expr {
                                    let ptr = parens_expr as *const ParenthesizedExpr;
                                    if !required_parens.contains(&ptr) {
                                        return *parens_expr.expr.clone();
                                    }
                                }
                                expr
                            },
                            ir::VisitorContextFlag::NONE,
                        );
                    }
                }
            }
            ir::OpKind::AnimationListener => {
                use crate::template::pipeline::ir::ops::create::AnimationListenerOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let anim_listener_ptr = op_ptr as *mut AnimationListenerOp;
                    let anim_listener = &mut *anim_listener_ptr;

                    for handler_op in anim_listener.handler_ops.iter_mut() {
                        transform_expressions_in_op(
                            handler_op.as_mut(),
                            &mut |expr: Expression, _flags| {
                                if let Expression::Parens(ref parens_expr) = &expr {
                                    let ptr = parens_expr as *const ParenthesizedExpr;
                                    if !required_parens.contains(&ptr) {
                                        return *parens_expr.expr.clone();
                                    }
                                }
                                expr
                            },
                            ir::VisitorContextFlag::NONE,
                        );
                    }
                }
            }
            ir::OpKind::RepeaterCreate => {
                use crate::template::pipeline::ir::ops::create::RepeaterCreateOp;
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let repeater_ptr = op_ptr as *mut RepeaterCreateOp;
                    let repeater = &mut *repeater_ptr;

                    if let Some(ref mut track_by_ops) = repeater.track_by_ops {
                        for track_by_op in track_by_ops.iter_mut() {
                            transform_expressions_in_op(
                                track_by_op.as_mut(),
                                &mut |expr: Expression, _flags| {
                                    if let Expression::Parens(ref parens_expr) = &expr {
                                        let ptr = parens_expr as *const ParenthesizedExpr;
                                        if !required_parens.contains(&ptr) {
                                            return *parens_expr.expr.clone();
                                        }
                                    }
                                    expr
                                },
                                ir::VisitorContextFlag::NONE,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn check_nested_expressions(
    expr: &Expression,
    required_parens: &mut HashSet<*const ParenthesizedExpr>,
) {
    if let Expression::BinaryOp(ref bin_op) = expr {
        match bin_op.operator {
            BinaryOperator::Exponentiation => {
                check_exponentiation_parens(bin_op, required_parens);
            }
            BinaryOperator::NullishCoalesce => {
                check_nullish_coalescing_parens(bin_op, required_parens);
            }
            BinaryOperator::And | BinaryOperator::Or => {
                check_and_or_parens(bin_op, required_parens);
            }
            _ => {}
        }
        // Recursively check nested
        check_nested_expressions(&*bin_op.lhs, required_parens);
        check_nested_expressions(&*bin_op.rhs, required_parens);
    } else if let Expression::Parens(ref parens_expr) = expr {
        check_nested_expressions(&*parens_expr.expr, required_parens);
    }
}

fn check_exponentiation_parens(
    expr: &BinaryOperatorExpr,
    required_parens: &mut HashSet<*const ParenthesizedExpr>,
) {
    if let Expression::Parens(ref parens_expr) = *expr.lhs {
        if let Expression::Unary(_) = *parens_expr.expr {
            required_parens.insert(parens_expr as *const ParenthesizedExpr);
        }
    }
}

fn check_nullish_coalescing_parens(
    expr: &BinaryOperatorExpr,
    required_parens: &mut HashSet<*const ParenthesizedExpr>,
) {
    if let Expression::Parens(ref parens_expr) = *expr.lhs {
        if is_logical_and_or(&*parens_expr.expr)
            || matches!(*parens_expr.expr, Expression::Conditional(_))
        {
            required_parens.insert(parens_expr as *const ParenthesizedExpr);
        }
    }
    if let Expression::Parens(ref parens_expr) = *expr.rhs {
        if is_logical_and_or(&*parens_expr.expr)
            || matches!(*parens_expr.expr, Expression::Conditional(_))
        {
            required_parens.insert(parens_expr as *const ParenthesizedExpr);
        }
    }
}

fn check_and_or_parens(
    expr: &BinaryOperatorExpr,
    required_parens: &mut HashSet<*const ParenthesizedExpr>,
) {
    if let Expression::Parens(ref parens_expr) = *expr.lhs {
        if let Expression::BinaryOp(ref inner_bin_op) = *parens_expr.expr {
            if inner_bin_op.operator == BinaryOperator::NullishCoalesce {
                required_parens.insert(parens_expr as *const ParenthesizedExpr);
            }
        }
    }
}

fn is_logical_and_or(expr: &Expression) -> bool {
    if let Expression::BinaryOp(ref bin_op) = expr {
        matches!(bin_op.operator, BinaryOperator::And | BinaryOperator::Or)
    } else {
        false
    }
}
