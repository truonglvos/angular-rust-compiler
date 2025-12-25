//! Remove Unused I18n Attributes Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/remove_unused_i18n_attrs.ts
//!
//! i18nAttributes ops will be generated for each i18n attribute. However, not all i18n attributes
//! will contain dynamic content, and so some of these i18nAttributes ops may be unnecessary.

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::I18nAttributesOp;
use crate::template::pipeline::ir::ops::update::I18nExpressionOp;
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, ComponentCompilationJob,
};

/// Remove unused i18n attributes ops.
pub fn remove_unused_i18n_attributes_ops(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();

    if matches!(
        job_kind,
        CompilationJobKind::Tmpl | CompilationJobKind::Both
    ) {
        let component_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let component_job_ptr = job_ptr as *mut ComponentCompilationJob;
            &mut *component_job_ptr
        };

        // Process root unit
        process_unit(&mut component_job.root);

        // Process all view units
        for (_, unit) in component_job.views.iter_mut() {
            process_unit(unit);
        }
    }
}

fn process_unit(unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit) {
    let mut owners_with_i18n_expressions = std::collections::HashSet::new();

    // Collect owners with i18n expressions
    for op in unit.update.iter() {
        if op.kind() == OpKind::I18nExpression {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let expr_op_ptr = op_ptr as *const I18nExpressionOp;
                let expr_op = &*expr_op_ptr;
                owners_with_i18n_expressions.insert(expr_op.i18n_owner);
            }
        }
    }

    // Remove I18nAttributes ops that don't have i18n expressions
    let mut indices_to_remove = Vec::new();
    for (idx, op) in unit.create.iter().enumerate() {
        if op.kind() == OpKind::I18nAttributes {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let attrs_op_ptr = op_ptr as *const I18nAttributesOp;
                let attrs_op = &*attrs_op_ptr;

                if !owners_with_i18n_expressions.contains(&attrs_op.xref) {
                    indices_to_remove.push(idx);
                }
            }
        }
    }

    // Remove in reverse order to maintain indices
    indices_to_remove.sort();
    indices_to_remove.reverse();
    for idx in indices_to_remove {
        unit.create.remove_at(idx);
    }
}
