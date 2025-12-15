//! Store Let Optimization Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/store_let_optimization.ts
//! Removes any `storeLet` calls that aren't referenced outside of the current view.

use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::expression::{transform_expressions_in_op, transform_expressions_in_expression};
use crate::template::pipeline::ir::ops::create::DeclareLetOp;
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationJobKind, CompilationUnit};
use crate::output::output_ast::Expression;

/// Removes any `storeLet` calls that aren't referenced outside of the current view.
pub fn optimize_store_let(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();
    
    if matches!(job_kind, CompilationJobKind::Tmpl | CompilationJobKind::Both) {
        let component_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let job_ptr = job_ptr as *mut ComponentCompilationJob;
            &mut *job_ptr
        };
        
        // Find all @let declarations that are used externally (via ContextLetReferenceExpr)
        let mut let_used_externally: std::collections::HashSet<ir::XrefId> = std::collections::HashSet::new();
        let mut declare_let_ops: std::collections::HashMap<ir::XrefId, usize> = std::collections::HashMap::new();
        
        // First pass: collect DeclareLetOp and ContextLetReferenceExpr
        // We'll collect DeclareLetOp first, then collect ContextLetReferenceExpr during optimization
        {
            // Process root unit
            collect_declare_let_ops(&component_job.root, &mut declare_let_ops);
            
            // Process all view units
            for (_, unit) in component_job.views.iter() {
                collect_declare_let_ops(unit, &mut declare_let_ops);
            }
        }
        
        // Collect ContextLetReferenceExpr references (use transform with identity to visit)
        {
            // Process root unit
            collect_context_let_references(&mut component_job.root, &mut let_used_externally);
            
            // Process all view units
            for (_, unit) in component_job.views.iter_mut() {
                collect_context_let_references(unit, &mut let_used_externally);
            }
        }
        
        // Second pass: optimize StoreLetExpr and remove unused DeclareLetOp
        {
            // Process root unit
            optimize_unit(&mut component_job.root, &let_used_externally, &mut declare_let_ops);
            
            // Process all view units
            for (_, unit) in component_job.views.iter_mut() {
                optimize_unit(unit, &let_used_externally, &mut declare_let_ops);
            }
        }
    }
}

fn collect_declare_let_ops(
    unit: &crate::template::pipeline::src::compilation::ViewCompilationUnit,
    declare_let_ops: &mut std::collections::HashMap<ir::XrefId, usize>,
) {
    // Collect DeclareLetOp indices
    for (index, op) in unit.create().iter().enumerate() {
        if op.kind() == OpKind::DeclareLet {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let declare_let_ptr = op_ptr as *const DeclareLetOp;
                let declare_let = &*declare_let_ptr;
                declare_let_ops.insert(declare_let.xref, index);
            }
        }
    }
}

fn collect_context_let_references(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    let_used_externally: &mut std::collections::HashSet<ir::XrefId>,
) {
    // Visit expressions in create ops using transform with identity
    for op in unit.create_mut().iter_mut() {
        unsafe {
            let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
            let op_ptr = op_ptr as *mut dyn ir::Op;
            transform_expressions_in_op(
                &mut *op_ptr,
                &mut |expr, _flags| {
                    if let Expression::ContextLetReference(ref ctx_let_ref) = expr {
                        let_used_externally.insert(ctx_let_ref.target);
                    }
                    expr // Identity transform - don't modify
                },
                ir::VisitorContextFlag::NONE,
            );
        }
    }
    
    // Visit expressions in update ops using transform with identity
    for op in unit.update_mut().iter_mut() {
        transform_expressions_in_op(
            op.as_mut(),
            &mut |expr, _flags| {
                if let Expression::ContextLetReference(ref ctx_let_ref) = expr {
                    let_used_externally.insert(ctx_let_ref.target);
                }
                expr // Identity transform - don't modify
            },
            ir::VisitorContextFlag::NONE,
        );
    }
}

fn optimize_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    let_used_externally: &std::collections::HashSet<ir::XrefId>,
    declare_let_ops: &mut std::collections::HashMap<ir::XrefId, usize>,
) {
    // Transform StoreLetExpr in update ops
    for op in unit.update_mut().iter_mut() {
        transform_expressions_in_op(
            op.as_mut(),
            &mut |expr, _flags| {
                if let Expression::StoreLet(ref store_let) = expr {
                    if !let_used_externally.contains(&store_let.target) {
                        // If @let isn't used in other views, we don't have to store its value
                        // Furthermore, if the @let isn't using pipes, we can also drop its declareLet op
                        // The DeclareLetOp will be removed later if needed
                        // Return the value expression instead of StoreLetExpr
                        return (*store_let.value).clone();
                    }
                }
                expr
            },
            ir::VisitorContextFlag::NONE,
        );
    }
    
    // Remove DeclareLetOp that are no longer needed
    // Collect indices to remove (in reverse order)
    let mut indices_to_remove: Vec<usize> = Vec::new();
    for (target_xref, index) in declare_let_ops.iter() {
        if !let_used_externally.contains(target_xref) {
            // Check if there's a StoreLetExpr for this target that doesn't have pipes
            // We need to check all StoreLetExpr in the unit
            let mut has_store_let_without_pipe = false;
            for op in unit.update_mut().iter_mut() {
                transform_expressions_in_op(
                    op.as_mut(),
                    &mut |expr, _flags| {
                        if let Expression::StoreLet(ref store_let) = expr {
                            if store_let.target == *target_xref && !has_pipe(store_let) {
                                has_store_let_without_pipe = true;
                            }
                        }
                        expr // Identity transform - don't modify
                    },
                    ir::VisitorContextFlag::NONE,
                );
            }
            
            if has_store_let_without_pipe {
                indices_to_remove.push(*index);
            }
        }
    }
    
    // Remove DeclareLetOps in reverse order to maintain indices
    indices_to_remove.sort_by(|a, b| b.cmp(a));
    for index in indices_to_remove {
        unit.create_mut().remove_at(index);
    }
}

/// Determines if a `storeLet` expression contains a pipe.
fn has_pipe(store_let: &crate::template::pipeline::ir::expression::StoreLetExpr) -> bool {
    let mut result = false;
    
    transform_expressions_in_expression(
        (*store_let.value).clone(),
        &mut |expr, _flags| {
            match expr {
                Expression::PipeBinding(_) | Expression::PipeBindingVariadic(_) => {
                    result = true;
                }
                _ => {}
            }
            expr
        },
        ir::VisitorContextFlag::NONE,
    );
    
    result
}


