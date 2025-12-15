//! Compiles semantic operations across all views and generates output Statements.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/reify.ts

use crate::output::output_ast as o;
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::operations::{CreateOp, UpdateOp};
use crate::template::pipeline::src::compilation::{
    CompilationJob, ComponentCompilationJob, CompilationUnit,
};
use crate::template::pipeline::src::instruction as ng;

pub fn reify(job: &mut dyn CompilationJob) {
    if let Some(component_job) = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        if job.kind() == crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
             Some(&mut *(job_ptr as *mut ComponentCompilationJob))
        } else {
            None
        }
    } {
        reify_unit(&mut component_job.root);
        for unit in component_job.views.values_mut() {
            reify_unit(unit);
        }
    }
}

fn reify_unit(unit: &mut dyn CompilationUnit) {
    reify_create_operations(unit);
    reify_update_operations(unit);
}

fn reify_create_operations(unit: &mut dyn CompilationUnit) {
    for op in unit.create_mut().iter_mut() {
        ir::transform_expressions_in_op(op.as_mut(), &mut reify_ir_expression, ir::VisitorContextFlag::NONE);
        
        let new_op: Option<Box<dyn CreateOp + Send + Sync>> = match op.kind() {
            ir::OpKind::Text => {
                if let Some(text_op) = op.as_any().downcast_ref::<ir::ops::create::TextOp>() {
                   if let Some(slot) = text_op.handle.slot {
                        let stmt = ng::text(slot as i32, text_op.initial_value.clone(), text_op.source_span.clone());
                        Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
                   } else { None }
                } else { None }
            }
            ir::OpKind::Element | ir::OpKind::ElementStart => {
                if let Some(el_op) = op.as_any().downcast_ref::<ir::ops::create::ElementStartOp>() {
                   if let Some(slot) = el_op.base.base.handle.slot {
                       let const_index = el_op.base.base.attributes.map(|idx| idx.as_usize() as i32);
                       let local_ref_index = el_op.base.base.local_refs_index.map(|idx| idx.as_usize() as i32);
                       let tag = el_op.base.tag.clone().unwrap_or_default();
                       let stmt = ng::element_start(slot as i32, tag, const_index, local_ref_index, el_op.base.base.start_source_span.clone());
                       Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
                   } else { None }
                } else if let Some(el_op) = op.as_any().downcast_ref::<ir::ops::create::ElementOp>() {
                   if let Some(slot) = el_op.base.base.handle.slot {
                       let const_index = el_op.base.base.attributes.map(|idx| idx.as_usize() as i32);
                       let local_ref_index = el_op.base.base.local_refs_index.map(|idx| idx.as_usize() as i32);
                       let tag = el_op.base.tag.clone().unwrap_or_default();
                       let stmt = ng::element(slot as i32, tag, const_index, local_ref_index, el_op.base.base.start_source_span.clone());
                       Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
                   } else { None }
                } else {
                    None
                }
            }
            ir::OpKind::ElementEnd => {
                 let stmt = ng::element_end(op.source_span().cloned());
                 Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
            }
            ir::OpKind::Pipe => {
                 if let Some(pipe_op) = op.as_any().downcast_ref::<ir::ops::create::CreatePipeOp>() {
                     if let Some(slot) = pipe_op.handle.slot {
                         let stmt = ng::pipe(slot as i32, pipe_op.name.clone());
                          Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
                     } else { None }
                 } else { None }
            }
             ir::OpKind::DisableBindings => {
                 let stmt = ng::disable_bindings();
                 Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
             },
             ir::OpKind::EnableBindings => {
                 let stmt = ng::enable_bindings();
                 Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn CreateOp + Send + Sync>>(Box::new(stmt))))
             },
            _ => None
        };
        
        if let Some(new) = new_op {
             *op = new;
        }
    }
}

fn reify_update_operations(unit: &mut dyn CompilationUnit) {
     for op in unit.update_mut().iter_mut() {
        ir::transform_expressions_in_op(op.as_mut(), &mut reify_ir_expression, ir::VisitorContextFlag::NONE);
        
        let new_op: Option<Box<dyn UpdateOp + Send + Sync>> = match op.kind() {
            ir::OpKind::Advance => {
                 if let Some(adv) = op.as_any().downcast_ref::<ir::ops::update::AdvanceOp>() {
                     let stmt = ng::advance(adv.delta as i32, adv.source_span.clone());
                     Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn UpdateOp + Send + Sync>>(Box::new(stmt))))
                 } else { None }
            }
            ir::OpKind::Property => {
                 if let Some(prop) = op.as_any().downcast_ref::<ir::ops::update::PropertyOp>() {
                     if let ir::ops::update::BindingExpression::Expression(expr) = &prop.expression {
                         let stmt = ng::property(prop.name.clone(), expr.clone(), prop.sanitizer.clone(), prop.source_span.clone());
                         Some(Box::new(ir::ops::shared::create_statement_op::<Box<dyn UpdateOp + Send + Sync>>(Box::new(stmt))))
                     } else { None }
                 } else { None }
            }
             _ => None
        };

        if let Some(new) = new_op {
            *op = new;
        }
    }
}

fn reify_ir_expression(expr: o::Expression, _flags: ir::VisitorContextFlag) -> o::Expression {
    if !ir::is_ir_expression(&expr) {
        return expr.clone();
    }
    
    match &expr {
        o::Expression::ReadVar(_read_var) => {
            // If it's a semantic variable, resolve it?
            // Currently `output_ast` handles variable references.
            // If `ir::is_ir_expression` checks specifically for IR-only variants (which we don't really have in `output_ast` except via wrapper or if we added them),
            // then we handle them. 
            // `output_ast` seems clean of IR ops. References to IR ops (XrefId) might be in specialized expression types if they exist.
            // But strict `output_ast` `Expression` implies they are already reified or standard AST.
            // If `o::Expression` has `LexicalRead` or similar IR constructs, convert.
            // Assuming `output_ast::Expression` is pure output AST now.
            expr
        }
        _ => expr
    }
}
