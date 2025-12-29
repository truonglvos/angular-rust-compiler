//! Merge sequential NextContextExpr operations.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/next_context_merging.ts
//!
//! Merges logically sequential `NextContextExpr` operations.
//! `NextContextExpr` can be referenced repeatedly, "popping" the runtime's context stack each time.
//! When two such expressions appear back-to-back, it's possible to merge them together into a single
//! `NextContextExpr` that steps multiple contexts. This merging is possible if all conditions are met:
//!
//!   * The result of the `NextContextExpr` that's folded into the subsequent one is not stored (that
//!     is, the call is purely side-effectful).
//!   * No operations in between them uses the implicit context.

use crate::output::output_ast::{Expression, Statement};
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::expression::{
    as_ir_expression, is_ir_expression, transform_expressions_in_op, VisitorContextFlag,
};
use crate::template::pipeline::ir::ops::create::{
    AnimationListenerOp, AnimationOp, ListenerOp, TwoWayListenerOp,
};
use crate::template::pipeline::ir::ops::shared::StatementOp;
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationUnit, ComponentCompilationJob,
};

pub fn merge_next_context_expressions(job: &mut dyn CompilationJob) {
    if let Some(component_job) = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        if job.kind() == crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
            Some(&mut *(job_ptr as *mut ComponentCompilationJob))
        } else {
            None
        }
    } {
        // Process handler ops in create operations
        for op in component_job.root.create_mut().iter_mut() {
            match op.kind() {
                ir::OpKind::Listener => unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let listener_ptr = op_ptr as *mut ListenerOp;
                    merge_next_contexts_in_ops(&mut (*listener_ptr).handler_ops);
                },
                ir::OpKind::TwoWayListener => unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let two_way_listener_ptr = op_ptr as *mut TwoWayListenerOp;
                    merge_next_contexts_in_ops(&mut (*two_way_listener_ptr).handler_ops);
                },
                ir::OpKind::AnimationListener => unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let anim_listener_ptr = op_ptr as *mut AnimationListenerOp;
                    merge_next_contexts_in_ops(&mut (*anim_listener_ptr).handler_ops);
                },
                ir::OpKind::Animation => unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let anim_ptr = op_ptr as *mut AnimationOp;
                    merge_next_contexts_in_ops(&mut (*anim_ptr).handler_ops);
                },
                _ => {}
            }
        }

        for view in component_job.views.values_mut() {
            for op in view.create_mut().iter_mut() {
                match op.kind() {
                    ir::OpKind::Listener => unsafe {
                        let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                        let listener_ptr = op_ptr as *mut ListenerOp;
                        merge_next_contexts_in_ops(&mut (*listener_ptr).handler_ops);
                    },
                    ir::OpKind::TwoWayListener => unsafe {
                        let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                        let two_way_listener_ptr = op_ptr as *mut TwoWayListenerOp;
                        merge_next_contexts_in_ops(&mut (*two_way_listener_ptr).handler_ops);
                    },
                    ir::OpKind::AnimationListener => unsafe {
                        let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                        let anim_listener_ptr = op_ptr as *mut AnimationListenerOp;
                        merge_next_contexts_in_ops(&mut (*anim_listener_ptr).handler_ops);
                    },
                    ir::OpKind::Animation => unsafe {
                        let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                        let anim_ptr = op_ptr as *mut AnimationOp;
                        merge_next_contexts_in_ops(&mut (*anim_ptr).handler_ops);
                    },
                    _ => {}
                }
            }
        }

        // Process update operations in root
        merge_next_contexts_in_ops(component_job.root.update_mut());

        // Process update operations in embedded views
        for view in component_job.views.values_mut() {
            merge_next_contexts_in_ops(view.update_mut());
        }
    }
}

fn merge_next_contexts_in_ops(ops: &mut ir::OpList<Box<dyn ir::UpdateOp + Send + Sync>>) {
    let mut indices_to_remove = Vec::new();
    let mut candidate_info = Vec::new();

    // First pass: collect candidate operations (StatementOp with NextContextExpr)
    for (idx, op) in ops.iter().enumerate() {
        if op.kind() != ir::OpKind::Statement {
            continue;
        }

        unsafe {
            let op_ptr = op.as_ref() as *const dyn ir::Op;
            let stmt_op_ptr = op_ptr as *const StatementOp<Box<dyn ir::Op + Send + Sync>>;
            let stmt_op = &*stmt_op_ptr;

            if let Statement::Expression(ref expr_stmt) = *stmt_op.statement {
                if let Some(ir_expr) = as_ir_expression(&expr_stmt.expr) {
                    if let ir::IRExpression::NextContext(ref next_ctx) = ir_expr {
                        candidate_info.push((idx, next_ctx.steps));
                    }
                }
            }
        }
    }

    // Second pass: try to merge each candidate with subsequent operations
    for (op_idx, merge_steps) in candidate_info {
        if indices_to_remove.contains(&op_idx) {
            continue; // Already merged
        }

        let mut found_merge_target: Option<usize> = None;
        let mut can_merge = true;

        // Look for merge target in subsequent operations
        for candidate_idx in (op_idx + 1)..ops.len() {
            if !can_merge {
                break;
            }

            if indices_to_remove.contains(&candidate_idx) {
                continue;
            }

            // Use mutable reference to check for blocking expressions and merge
            let candidate_op_mut = ops.get_mut(candidate_idx).unwrap();

            let mut has_blocking = false;
            let mut has_next_context = false;

            // First pass: check for blocking expressions and NextContextExpr
            transform_expressions_in_op(
                candidate_op_mut.as_mut(),
                &mut |expr: Expression, flags| {
                    if flags.contains(VisitorContextFlag::IN_CHILD_OPERATION) {
                        has_blocking = true;
                        return expr;
                    }

                    if is_ir_expression(&expr) {
                        if let Some(ir_expr) = as_ir_expression(&expr) {
                            match ir_expr {
                                ir::IRExpression::GetCurrentView(_)
                                | ir::IRExpression::Reference(_)
                                | ir::IRExpression::ContextLetReference(_) => {
                                    has_blocking = true;
                                }
                                ir::IRExpression::NextContext(_) => {
                                    has_next_context = true;
                                }
                                _ => {}
                            }
                        }
                    }

                    expr
                },
                VisitorContextFlag::NONE,
            );

            if has_blocking {
                can_merge = false;
                break;
            }

            // If we found NextContextExpr, merge into it (second pass)
            if has_next_context {
                // Handle normal expressions (StatementOp)
                transform_expressions_in_op(
                    candidate_op_mut.as_mut(),
                    &mut |mut expr: Expression, flags| {
                        if flags.contains(VisitorContextFlag::IN_CHILD_OPERATION) {
                            return expr;
                        }

                        if let Expression::NextContext(ref mut next_ctx) = expr {
                            next_ctx.steps += merge_steps;
                            found_merge_target = Some(candidate_idx);
                        }

                        expr
                    },
                    VisitorContextFlag::NONE,
                );

                // Handle VariableOp initializers explicitly if transform_expressions didn't catch it
                // (transform_expressions DOES catch it, but let's be sure we are modifying the right thing)
                // Actually, VariableOp implements UpdateOp/CreateOp, so transform_expressions_in_op SHOULD work over it.
                // The issue might be that candidate_op_mut IS a VariableOp.

                if found_merge_target.is_some() {
                    break;
                }
            }
        }

        // If we found a merge target, mark the source operation for removal
        if found_merge_target.is_some() {
            indices_to_remove.push(op_idx);
        }
    }

    // Remove merged operations (in reverse order to maintain indices)
    indices_to_remove.sort();
    indices_to_remove.reverse();
    for idx in indices_to_remove {
        ops.remove_at(idx);
    }
}
