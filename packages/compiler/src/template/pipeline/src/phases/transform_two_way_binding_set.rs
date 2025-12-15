//! Transform Two Way Binding Set Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/transform_two_way_binding_set.ts
//! Transforms a `TwoWayBindingSet` expression into an expression that either
//! sets a value through the `twoWayBindingSet` instruction or falls back to setting
//! the value directly.

use crate::output::output_ast::Expression;
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::expression::transform_expressions_in_op;
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationUnit};
use crate::template::pipeline::src::instruction::two_way_binding_set;

/// Transforms a `TwoWayBindingSet` expression into an expression that either
/// sets a value through the `twoWayBindingSet` instruction or falls back to setting
/// the value directly.
pub fn transform_two_way_binding_set(job: &mut dyn CompilationJob) {
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *job_ptr
    };
    
    // Process root unit
    process_unit(&mut component_job.root);
    
    // Process all view units
    for (_, unit) in component_job.views.iter_mut() {
        process_unit(unit);
    }
}

fn process_unit(unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit) {
    for op in unit.create_mut().iter_mut() {
        if op.kind() == OpKind::TwoWayListener {
            unsafe {
                use crate::template::pipeline::ir::ops::create::TwoWayListenerOp;
                let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                let two_way_listener_ptr = op_ptr as *mut TwoWayListenerOp;
                let two_way_listener = &mut *two_way_listener_ptr;
                
                for handler_op in two_way_listener.handler_ops.iter_mut() {
                    transform_expressions_in_op(
                        handler_op.as_mut(),
                        &mut |expr: Expression, _flags| {
                            // Check if this is a TwoWayBindingSetExpr
                            if let Expression::TwoWayBindingSet(two_way_expr) = &expr {
                                let target = two_way_expr.target.clone();
                                let value = two_way_expr.value.clone();
                                let source_span = two_way_expr.source_span.clone();
                                
                                // Transform based on target type
                                match &*target {
                                    Expression::ReadProp(_) | Expression::ReadKey(_) => {
                                        // For ReadPropExpr or ReadKeyExpr, create:
                                        // twoWayBindingSet(target, value) || (target = value)
                                        let two_way_set = two_way_binding_set(
                                            target.clone(),
                                            value.clone(),
                                        );
                                        let assign = target.set(value, source_span.clone());
                                        *two_way_set.or(assign, source_span)
                                    }
                                    Expression::ReadVariable(_) => {
                                        // For ReadVariableExpr (local template variable),
                                        // only emit the twoWayBindingSet since the fallback
                                        // would be attempting to write into a constant.
                                        *two_way_binding_set(
                                            target.clone(),
                                            value.clone(),
                                        )
                                    }
                                    _ => {
                                        panic!("Unsupported expression in two-way action binding.");
                                    }
                                }
                            } else {
                                expr
                            }
                        },
                        ir::VisitorContextFlag::IN_CHILD_OPERATION,
                    );
                }
            }
        }
    }
}

