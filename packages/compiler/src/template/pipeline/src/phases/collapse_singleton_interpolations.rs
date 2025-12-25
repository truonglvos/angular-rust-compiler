//! Collapse Singleton Interpolations Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/collapse_singleton_interpolations.ts
//! Collapses singleton interpolations into plain expressions for eligible ops

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::update::{
    AttributeOp, BindingExpression, ClassMapOp, StyleMapOp, StylePropOp,
};
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationUnit, ComponentCompilationJob,
};

/// Attribute or style interpolations of the form `[attr.foo]="{{foo}}""` should be "collapsed"
/// into a plain instruction, instead of an interpolated one.
///
/// (We cannot do this for singleton property interpolations,
/// because they need to stringify their expressions)
///
/// The reification step is also capable of performing this transformation, but doing it early in the
/// pipeline allows other phases to accurately know what instruction will be emitted.
pub fn collapse_singleton_interpolations(job: &mut dyn CompilationJob) {
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
    for op in unit.update_mut().iter_mut() {
        let eligible = matches!(
            op.kind(),
            OpKind::Attribute | OpKind::StyleProp | OpKind::StyleMap | OpKind::ClassMap
        );

        if !eligible {
            continue;
        }

        // Downcast to appropriate op type and check/collapse interpolation
        unsafe {
            let op_ptr = op.as_mut() as *mut dyn ir::UpdateOp;

            match op.kind() {
                OpKind::Attribute => {
                    let attr_op_ptr = op_ptr as *mut AttributeOp;
                    let attr_op = &mut *attr_op_ptr;

                    if let BindingExpression::Interpolation(ref interp) = attr_op.expression {
                        if interp.strings.len() == 2
                            && interp.strings.iter().all(|s| s.is_empty())
                            && interp.expressions.len() == 1
                        {
                            // Collapse: replace interpolation with first expression
                            attr_op.expression =
                                BindingExpression::Expression(interp.expressions[0].clone());
                        }
                    }
                }
                OpKind::StyleProp => {
                    let style_prop_op_ptr = op_ptr as *mut StylePropOp;
                    let style_prop_op = &mut *style_prop_op_ptr;

                    if let BindingExpression::Interpolation(ref interp) = style_prop_op.expression {
                        if interp.strings.len() == 2
                            && interp.strings.iter().all(|s| s.is_empty())
                            && interp.expressions.len() == 1
                        {
                            // Collapse: replace interpolation with first expression
                            style_prop_op.expression =
                                BindingExpression::Expression(interp.expressions[0].clone());
                        }
                    }
                }
                OpKind::StyleMap => {
                    let style_map_op_ptr = op_ptr as *mut StyleMapOp;
                    let style_map_op = &mut *style_map_op_ptr;

                    if let BindingExpression::Interpolation(ref interp) = style_map_op.expression {
                        if interp.strings.len() == 2
                            && interp.strings.iter().all(|s| s.is_empty())
                            && interp.expressions.len() == 1
                        {
                            // Collapse: replace interpolation with first expression
                            style_map_op.expression =
                                BindingExpression::Expression(interp.expressions[0].clone());
                        }
                    }
                }
                OpKind::ClassMap => {
                    let class_map_op_ptr = op_ptr as *mut ClassMapOp;
                    let class_map_op = &mut *class_map_op_ptr;

                    if let BindingExpression::Interpolation(ref interp) = class_map_op.expression {
                        if interp.strings.len() == 2
                            && interp.strings.iter().all(|s| s.is_empty())
                            && interp.expressions.len() == 1
                        {
                            // Collapse: replace interpolation with first expression
                            class_map_op.expression =
                                BindingExpression::Expression(interp.expressions[0].clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
