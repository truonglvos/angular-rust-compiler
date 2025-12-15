//! Remove Empty Bindings Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/remove_empty_bindings.ts
//! Binding with no content can be safely deleted.

use crate::output::output_ast::Expression;
use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::update::{AttributeOp, BindingOp, ClassPropOp, ClassMapOp, PropertyOp, StylePropOp, StyleMapOp};
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationJobKind, CompilationUnit};

/// Binding with no content can be safely deleted.
pub fn remove_empty_bindings(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();
    
    if matches!(job_kind, CompilationJobKind::Tmpl | CompilationJobKind::Both) {
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
    } else {
        // Handle HostBindingCompilationJob
        use crate::template::pipeline::src::compilation::HostBindingCompilationJob;
        let host_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let job_ptr = job_ptr as *mut HostBindingCompilationJob;
            &mut *job_ptr
        };
        
        process_unit_host(&mut host_job.root);
    }
}

fn process_unit(unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit) {
    let mut ops_to_remove: Vec<usize> = Vec::new();
    
    // Collect ops to remove
    for (index, op) in unit.update().iter().enumerate() {
        match op.kind() {
            OpKind::Attribute | OpKind::Binding | OpKind::ClassProp | OpKind::ClassMap 
            | OpKind::Property | OpKind::StyleProp | OpKind::StyleMap => {
                if is_empty_expression_op(op) {
                    ops_to_remove.push(index);
                }
            }
            _ => {}
        }
    }
    
    // Remove ops in reverse order to maintain indices
    for index in ops_to_remove.iter().rev() {
        unit.update_mut().remove_at(*index);
    }
}

fn process_unit_host(unit: &mut crate::template::pipeline::src::compilation::HostBindingCompilationUnit) {
    let mut ops_to_remove: Vec<usize> = Vec::new();
    
    // Collect ops to remove
    for (index, op) in unit.update().iter().enumerate() {
        match op.kind() {
            OpKind::Attribute | OpKind::Binding | OpKind::ClassProp | OpKind::ClassMap 
            | OpKind::Property | OpKind::StyleProp | OpKind::StyleMap => {
                if is_empty_expression_op(op) {
                    ops_to_remove.push(index);
                }
            }
            _ => {}
        }
    }
    
    // Remove ops in reverse order to maintain indices
    for index in ops_to_remove.iter().rev() {
        unit.update_mut().remove_at(*index);
    }
}

/// Check if an op has an empty expression
fn is_empty_expression_op(op: &Box<dyn ir::UpdateOp + Send + Sync>) -> bool {
    unsafe {
        match op.kind() {
            OpKind::Attribute => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let attr_op = &*(op_ptr as *const AttributeOp);
                match &attr_op.expression {
                    ir::ops::update::BindingExpression::Expression(expr) => is_empty_expression(expr),
                    ir::ops::update::BindingExpression::Interpolation(_) => false,
                }
            }
            OpKind::Binding => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let binding_op = &*(op_ptr as *const BindingOp);
                match &binding_op.expression {
                    ir::ops::update::BindingExpression::Expression(expr) => is_empty_expression(expr),
                    ir::ops::update::BindingExpression::Interpolation(_) => false,
                }
            }
            OpKind::ClassProp => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let class_prop_op = &*(op_ptr as *const ClassPropOp);
                is_empty_expression(&class_prop_op.expression)
            }
            OpKind::ClassMap => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let class_map_op = &*(op_ptr as *const ClassMapOp);
                match &class_map_op.expression {
                    ir::ops::update::BindingExpression::Expression(expr) => is_empty_expression(expr),
                    ir::ops::update::BindingExpression::Interpolation(_) => false,
                }
            }
            OpKind::Property => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let property_op = &*(op_ptr as *const PropertyOp);
                match &property_op.expression {
                    ir::ops::update::BindingExpression::Expression(expr) => is_empty_expression(expr),
                    ir::ops::update::BindingExpression::Interpolation(_) => false,
                }
            }
            OpKind::StyleProp => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let style_prop_op = &*(op_ptr as *const StylePropOp);
                match &style_prop_op.expression {
                    ir::ops::update::BindingExpression::Expression(expr) => is_empty_expression(expr),
                    ir::ops::update::BindingExpression::Interpolation(_) => false,
                }
            }
            OpKind::StyleMap => {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let style_map_op = &*(op_ptr as *const StyleMapOp);
                match &style_map_op.expression {
                    ir::ops::update::BindingExpression::Expression(expr) => is_empty_expression(expr),
                    ir::ops::update::BindingExpression::Interpolation(_) => false,
                }
            }
            _ => false,
        }
    }
}

/// Check if an expression is an EmptyExpr
fn is_empty_expression(expr: &Expression) -> bool {
    matches!(expr, Expression::Empty(_))
}
