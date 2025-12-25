//! Has Const Expression Collection Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/has_const_expression_collection.ts
//! `ir.ConstCollectedExpr` may be present in any IR expression. This means that expression needs to
//! be lifted into the component const array, and replaced with a reference to the const array at its
//! usage site. This phase walks the IR and performs this transformation.

use crate::output::output_ast::{Expression, ExpressionTrait};
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::expression::transform_expressions_in_op;
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, CompilationUnit, ComponentCompilationJob,
};

// Helper function to transform expressions in create ops
fn transform_expressions_in_create_op(
    op: &mut Box<dyn ir::CreateOp + Send + Sync>,
    transform: &mut dyn FnMut(Expression, ir::VisitorContextFlag) -> Expression,
    flags: ir::VisitorContextFlag,
) {
    // For now, use the same approach as transform_expressions_in_op
    // This is a simplified version - full implementation would need to handle all create op types
    use crate::template::pipeline::ir::enums::OpKind;
    use crate::template::pipeline::ir::expression::transform_expressions_in_expression;
    use crate::template::pipeline::ir::ops::create::RepeaterCreateOp;

    unsafe {
        let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;

        match op.kind() {
            OpKind::RepeaterCreate => {
                let repeater_ptr = op_ptr as *mut RepeaterCreateOp;
                let repeater = &mut *repeater_ptr;

                // Transform track expression if track_by_ops is None
                if repeater.track_by_ops.is_none() {
                    let track_expr = (*repeater.track).clone();
                    let transformed = transform(track_expr, flags);
                    repeater.track = Box::new(transform_expressions_in_expression(
                        transformed,
                        transform,
                        flags,
                    ));
                }
            }
            _ => {
                // Other create ops don't have expressions or are handled elsewhere
            }
        }
    }
}

/// `ir.ConstCollectedExpr` may be present in any IR expression. This means that expression needs to
/// be lifted into the component const array, and replaced with a reference to the const array at its
/// usage site. This phase walks the IR and performs this transformation.
pub fn collect_const_expressions(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();

    if matches!(
        job_kind,
        CompilationJobKind::Tmpl | CompilationJobKind::Both
    ) {
        let component_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let job_ptr = job_ptr as *mut ComponentCompilationJob;
            &mut *job_ptr
        };

        // Process root unit
        {
            let component_job_ptr = component_job as *mut ComponentCompilationJob;
            let root = &mut component_job.root;
            process_unit(root, component_job_ptr);
        }

        // Process all view units - collect keys first to avoid borrow checker issues
        let view_keys: Vec<_> = component_job.views.keys().cloned().collect();
        let component_job_ptr = component_job as *mut ComponentCompilationJob;
        for key in view_keys {
            if let Some(unit) = component_job.views.get_mut(&key) {
                process_unit(unit, component_job_ptr);
            }
        }
    }
}

fn process_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    component_job_ptr: *mut ComponentCompilationJob,
) {
    // Use unsafe to bypass borrow checker - we know it's safe because:
    // 1. We're only reading from unit while calling add_const
    // 2. add_const doesn't modify unit, only component_job.consts

    // Process create ops
    for op in unit.create_mut().iter_mut() {
        transform_expressions_in_create_op(
            op,
            &mut |expr, _flags| {
                if let Expression::ConstCollected(const_collected) = &expr {
                    // Replace with literal reference to const array
                    // SAFETY: component_job_ptr is valid and add_const doesn't modify unit
                    let const_index = unsafe {
                        (&mut *component_job_ptr).add_const((*const_collected.expr).clone(), None)
                    };
                    Expression::Literal(crate::output::output_ast::LiteralExpr {
                        value: crate::output::output_ast::LiteralValue::Number(
                            const_index.as_usize() as f64,
                        ),
                        type_: None,
                        source_span: expr.source_span().cloned(),
                    })
                } else {
                    expr
                }
            },
            ir::VisitorContextFlag::NONE,
        );
    }

    // Process update ops
    for op in unit.update_mut().iter_mut() {
        transform_expressions_in_op(
            op.as_mut(),
            &mut |expr, _flags| {
                if let Expression::ConstCollected(const_collected) = &expr {
                    // Replace with literal reference to const array
                    // SAFETY: component_job_ptr is valid and add_const doesn't modify unit
                    let const_index = unsafe {
                        (&mut *component_job_ptr).add_const((*const_collected.expr).clone(), None)
                    };
                    Expression::Literal(crate::output::output_ast::LiteralExpr {
                        value: crate::output::output_ast::LiteralValue::Number(
                            const_index.as_usize() as f64,
                        ),
                        type_: None,
                        source_span: expr.source_span().cloned(),
                    })
                } else {
                    expr
                }
            },
            ir::VisitorContextFlag::NONE,
        );
    }
}
