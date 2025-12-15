
//! Pipes that accept more than 4 arguments are variadic, and are handled with a different runtime instruction.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/pipe_variadic.ts

use crate::output::output_ast as o;
use crate::template::pipeline::ir;
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationUnit};

pub fn create_variadic_pipes(job: &mut dyn CompilationJob) {
     if let Some(component_job) = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        if job.kind() == crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
            Some(&mut *(job_ptr as *mut ComponentCompilationJob))
        } else {
            None
        }
    } {
        for unit in component_job.views.values_mut() {
            for op in unit.update_mut().iter_mut() {
                ir::transform_expressions_in_op(op.as_mut(), &mut transform_pipe, ir::VisitorContextFlag::NONE);
            }
        }
    }
}

fn transform_pipe(expr: o::Expression, _flags: ir::VisitorContextFlag) -> o::Expression {
    if !ir::is_ir_expression(&expr) {
        return expr;
    }
    
    // Check if it's a PipeBinding expression
    if let Some(pipe_expr) = ir::as_ir_expression(&expr).and_then(|e| {
         if let ir::IRExpression::PipeBinding(pb) = e { Some(pb) } else { None }
    }) {
        if pipe_expr.args.len() <= 4 {
            return expr;
        }
        
        let args_literal = o::literal_arr(pipe_expr.args.clone());
        let num_args = pipe_expr.args.len();
        
        // Return PipeBindingVariadic
        // ir::IRExpression::PipeBindingVariadic(...)
        // Need to construct IRExpression wrapping it.
        // Assuming helper exists or direct construction.
        
        let variadic = ir::expression::PipeBindingVariadicExpr::new(
            pipe_expr.target,
            pipe_expr.target_slot.clone(),
            pipe_expr.name.clone(),
            args_literal,
            num_args,
        );
        
        o::Expression::PipeBindingVariadic(variadic)
    } else {
        expr
    }
}
