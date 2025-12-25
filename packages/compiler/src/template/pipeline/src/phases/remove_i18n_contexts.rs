//! Remove I18n Contexts Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/remove_i18n_contexts.ts
//!
//! Remove the i18n context ops after they are no longer needed, and null out references to them to
//! be safe.

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::I18nStartOp;
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, ComponentCompilationJob,
};

/// Remove the i18n context ops after they are no longer needed.
pub fn remove_i18n_contexts(job: &mut dyn CompilationJob) {
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
    let (indices_to_remove, i18n_start_indices) = {
        let mut indices_to_remove = Vec::new();
        let mut i18n_start_indices = Vec::new();

        // Collect I18nContext ops to remove and I18nStart ops to modify
        for (idx, op) in unit.create.iter().enumerate() {
            match op.kind() {
                OpKind::I18nContext => {
                    indices_to_remove.push(idx);
                }
                OpKind::I18nStart => {
                    i18n_start_indices.push(idx);
                }
                _ => {}
            }
        }

        (indices_to_remove, i18n_start_indices)
    };

    // Null out I18nStart.context
    for idx in &i18n_start_indices {
        unsafe {
            let op_ptr = unit.create.get_mut(*idx).unwrap().as_mut() as *mut dyn ir::CreateOp;
            let i18n_ptr = op_ptr as *mut I18nStartOp;
            let i18n = &mut *i18n_ptr;
            i18n.base.context = None;
        }
    }

    // Remove I18nContext ops in reverse order to maintain indices
    let mut sorted_indices = indices_to_remove;
    sorted_indices.sort();
    sorted_indices.reverse();
    for idx in sorted_indices {
        unit.create.remove_at(idx);
    }
}
