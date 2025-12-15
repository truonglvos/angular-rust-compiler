//! Wrap I18n ICUs Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/wrap_icus.ts
//!
//! Wraps ICUs that do not already belong to an i18n block in a new i18n block.

use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::{IcuStartOp, create_i18n_start_op, create_i18n_end_op};
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationJobKind};

/// Wraps ICUs that do not already belong to an i18n block in a new i18n block.
pub fn wrap_i18n_icus(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();
    
    if matches!(job_kind, CompilationJobKind::Tmpl | CompilationJobKind::Both) {
        let component_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let component_job_ptr = job_ptr as *mut ComponentCompilationJob;
            &mut *component_job_ptr
        };
        
        // Process root unit
        process_unit(&mut component_job.root, job);
        
        // Process all view units
        for (_, unit) in component_job.views.iter_mut() {
            process_unit(unit, job);
        }
    }
}

fn process_unit(unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit, job: &mut dyn CompilationJob) {
    let mut current_i18n_op: Option<usize> = None;
    let mut added_i18n_id: Option<ir::XrefId> = None;
    let mut ops_to_insert: Vec<(usize, Box<dyn ir::CreateOp + Send + Sync>)> = Vec::new();
    
    for (idx, op) in unit.create.iter().enumerate() {
        match op.kind() {
            OpKind::I18nStart => {
                current_i18n_op = Some(idx);
            }
            OpKind::I18nEnd => {
                current_i18n_op = None;
            }
            OpKind::IcuStart => {
                if current_i18n_op.is_none() {
                    unsafe {
                        let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                        let icu_ptr = op_ptr as *const IcuStartOp;
                        let icu = &*icu_ptr;
                        
                        added_i18n_id = Some(job.allocate_xref_id());
                        // ICU i18n start/end ops should not receive source spans.
                        let i18n_start_op = create_i18n_start_op(
                            added_i18n_id.unwrap(),
                            icu.message.clone(),
                            None, // root
                            None, // source_span
                        );
                        ops_to_insert.push((idx, i18n_start_op));
                    }
                }
            }
            OpKind::IcuEnd => {
                if let Some(i18n_id) = added_i18n_id {
                    let i18n_end_op = create_i18n_end_op(i18n_id, None);
                    ops_to_insert.push((idx + 1, i18n_end_op));
                    added_i18n_id = None;
                }
            }
            _ => {}
        }
    }
    
    // Insert ops in reverse order to maintain indices
    ops_to_insert.sort_by(|a, b| b.0.cmp(&a.0));
    for (idx, op) in ops_to_insert {
        unit.create.insert_at(idx, op);
    }
}

