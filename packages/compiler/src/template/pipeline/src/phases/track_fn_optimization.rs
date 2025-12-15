//! Track Function Optimization Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/track_fn_optimization.ts
//! `track` functions in `for` repeaters can sometimes be "optimized," i.e. transformed into inline
//! expressions, in lieu of an external function call.

use crate::output::output_ast::{Expression, Statement, ReturnStatement};
use crate::render3::r3_identifiers::Identifiers;
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::expression::{TrackContextExpr, transform_expressions_in_expression, VisitorContextFlag};
use crate::template::pipeline::ir::ops::create::RepeaterCreateOp;
use crate::template::pipeline::ir::ops::shared::create_statement_op;
use crate::template::pipeline::ir::operations::OpList;
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationUnit};
use crate::output::output_ast::ExpressionTrait;

/// `track` functions in `for` repeaters can sometimes be "optimized," i.e. transformed into inline
/// expressions, in lieu of an external function call.
pub fn optimize_track_fns(job: &mut dyn CompilationJob) {
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *job_ptr
    };
    
    let root_xref = component_job.root.xref();
    
    // Process root unit
    process_unit(&mut component_job.root, root_xref);
    
    // Process all view units
    for (_, unit) in component_job.views.iter_mut() {
        process_unit(unit, root_xref);
    }
}

fn process_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    root_xref: ir::XrefId,
) {
    // Get unit xref before borrowing create_mut
    let unit_xref = unit.xref();
    
    for op in unit.create_mut().iter_mut() {
        if op.kind() != OpKind::RepeaterCreate {
            continue;
        }
        
        unsafe {
            let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
            let repeater_ptr = op_ptr as *mut RepeaterCreateOp;
            let repeater = &mut *repeater_ptr;
            
            // Check if track is ReadVarExpr with '$index' or '$item'
            if let Expression::ReadVar(read_var) = &*repeater.track {
                if read_var.name == "$index" {
                    // Top-level access of `$index` uses the built in `repeaterTrackByIndex`.
                    repeater.track_by_fn = Some(Box::new(Expression::External(
                        crate::output::output_ast::ExternalExpr {
                            value: Identifiers::repeater_track_by_index(),
                            type_: None,
                            source_span: None,
                        }
                    )));
                    continue;
                } else if read_var.name == "$item" {
                    // Top-level access of the item uses the built in `repeaterTrackByIdentity`.
                    repeater.track_by_fn = Some(Box::new(Expression::External(
                        crate::output::output_ast::ExternalExpr {
                            value: Identifiers::repeater_track_by_identity(),
                            type_: None,
                            source_span: None,
                        }
                    )));
                    continue;
                }
            }
            
            // Check if track is a function call pattern: fn($index, item) or fn($index)
            if is_track_by_function_call(root_xref, &*repeater.track) {
                // Mark the function as using the component instance
                repeater.uses_component_instance = true;
                
                if let Expression::InvokeFn(invoke_fn) = &*repeater.track {
                    if let Expression::ReadProp(read_prop) = &*invoke_fn.fn_ {
                        if let Expression::Context(context_expr) = &*read_prop.receiver {
                            // Top-level method calls in the form of `fn($index, item)` can be passed in directly.
                            if context_expr.view == unit_xref {
                                repeater.track_by_fn = Some(Box::new(Expression::ReadProp(read_prop.clone())));
                            } else {
                                // This is a plain method call, but not in the component's root view.
                                // We need to get the component instance, and then call the method on it.
                                let component_instance = Expression::External(
                                    crate::output::output_ast::ExternalExpr {
                                        value: Identifiers::component_instance(),
                                        type_: None,
                                        source_span: None,
                                    }
                                );
                                let component_instance_call = component_instance.call_fn(vec![], None, None);
                                let track_by_fn = component_instance_call.prop(
                                    read_prop.name.clone(),
                                    repeater.track.as_ref().source_span().cloned(),
                                );
                                repeater.track_by_fn = Some(track_by_fn);
                                
                                // Because the context is not available (without a special function), we don't want to
                                // try to resolve it later. Let's get rid of it by overwriting the original track
                                // expression (which won't be used anyway).
                                repeater.track = repeater.track_by_fn.clone().unwrap();
                            }
                        }
                    }
                }
                continue;
            }
            
            // The track function could not be optimized.
            // Replace context reads with a special IR expression, since context reads in a track
            // function are emitted specially.
            let mut track_expr = (*repeater.track).clone();
            let mut has_context_expr = false;
            track_expr = transform_expressions_in_expression(
                track_expr,
                &mut |expr: Expression, _flags| {
                    // Check for pipes (not allowed in this context)
                    if matches!(expr, Expression::PipeBinding(_) | Expression::PipeBindingVariadic(_)) {
                        panic!("Illegal State: Pipes are not allowed in this context");
                    }
                    
                    // Replace ContextExpr with TrackContextExpr
                    if let Expression::Context(context_expr) = expr {
                        has_context_expr = true;
                        Expression::TrackContext(TrackContextExpr::new(context_expr.view))
                    } else {
                        expr
                    }
                },
                VisitorContextFlag::NONE,
            );
            
            // Set flag if we found ContextExpr
            if has_context_expr {
                repeater.uses_component_instance = true;
            }
            
            repeater.track = Box::new(track_expr);
            
            // Also create an OpList for the tracking expression since it may need
            // additional ops when generating the final code (e.g. temporary variables).
            let mut track_op_list: OpList<Box<dyn ir::UpdateOp + Send + Sync>> = OpList::new();
            let return_stmt = Statement::Return(ReturnStatement {
                value: repeater.track.clone(),
                source_span: repeater.track.as_ref().source_span().cloned(),
            });
            let stmt_op = create_statement_op::<Box<dyn ir::UpdateOp + Send + Sync>>(Box::new(return_stmt));
            track_op_list.push(Box::new(stmt_op) as Box<dyn ir::UpdateOp + Send + Sync>);
            repeater.track_by_ops = Some(track_op_list);
        }
    }
}

/// Check if the expression is a track-by function call pattern:
/// `fn($index, item)` or `fn($index)` where fn is called on ContextExpr from root view
fn is_track_by_function_call(
    root_view: ir::XrefId,
    expr: &Expression,
) -> bool {
    // Must be InvokeFunctionExpr with 1 or 2 args
    let invoke_fn = match expr {
        Expression::InvokeFn(invoke) => invoke,
        _ => return false,
    };
    
    if invoke_fn.args.is_empty() || invoke_fn.args.len() > 2 {
        return false;
    }
    
    // Receiver must be ReadPropExpr with ContextExpr receiver
    let read_prop = match &*invoke_fn.fn_ {
        Expression::ReadProp(read_prop) => read_prop,
        _ => return false,
    };
    
    let context_expr = match &*read_prop.receiver {
        Expression::Context(ctx) => ctx,
        _ => return false,
    };
    
    // Context must be from root view
    if context_expr.view != root_view {
        return false;
    }
    
    // First argument must be ReadVarExpr with name '$index'
    let arg0 = match invoke_fn.args.get(0) {
        Some(Expression::ReadVar(read_var)) => read_var,
        _ => return false,
    };
    
    if arg0.name != "$index" {
        return false;
    }
    
    // If there's a second argument, it must be ReadVarExpr with name '$item'
    if invoke_fn.args.len() == 2 {
        let arg1 = match invoke_fn.args.get(1) {
            Some(Expression::ReadVar(read_var)) => read_var,
            _ => return false,
        };
        
        if arg1.name != "$item" {
            return false;
        }
    }
    
    true
}

