//! Namespace Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/namespace.ts
//! Change namespaces between HTML, SVG and MathML, depending on the next element.

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::{Namespace, OpKind};
use crate::template::pipeline::ir::ops::create::{create_namespace_op, ElementStartOp};
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, CompilationUnit, ComponentCompilationJob,
};

/// Change namespaces between HTML, SVG and MathML, depending on the next element.
pub fn emit_namespace_changes(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();

    if matches!(
        job_kind,
        CompilationJobKind::Tmpl | CompilationJobKind::Both
    ) {
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
}

fn process_unit(unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit) {
    let mut active_namespace = Namespace::HTML;
    let mut insertions: Vec<(usize, Namespace)> = Vec::new();

    // First pass: collect namespace changes
    for (index, op) in unit.create().iter().enumerate() {
        if op.kind() == OpKind::ElementStart {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let elem_start_ptr = op_ptr as *const ElementStartOp;
                let elem_start = &*elem_start_ptr;

                if elem_start.base.namespace != active_namespace {
                    // Insert NamespaceOp before this ElementStart
                    insertions.push((index, elem_start.base.namespace));
                    active_namespace = elem_start.base.namespace;
                }
            }
        }
    }

    // Second pass: insert namespace ops in reverse order (to maintain indices)
    for (index, namespace) in insertions.iter().rev() {
        let namespace_op = create_namespace_op(*namespace);
        unit.create_mut().insert_at(*index, namespace_op);
    }
}
