//! Remove Content Selectors Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/phase_remove_content_selectors.ts
//! Removes 'select' attributes from ng-content elements as they are handled as projection properties

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::update::BindingOp;
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, CompilationUnit, ComponentCompilationJob,
};
use crate::template::pipeline::src::util::elements::{create_op_xref_map, lookup_element};

/// Attributes of `ng-content` named 'select' are specifically removed, because they control which
/// content matches as a property of the `projection`, and are not a plain attribute.
pub fn remove_content_selectors(job: &mut dyn CompilationJob) {
    let job_kind = job.kind();

    // Only process ComponentCompilationJob (HostBindingCompilationJob doesn't have Projection ops)
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
    let elements = create_op_xref_map(unit);

    // Collect ops to remove - we need to iterate over update ops
    let mut ops_to_remove: Vec<usize> = Vec::new();

    for (index, op) in unit.update().iter().enumerate() {
        if op.kind() == OpKind::Binding {
            // Check if this is a 'select' attribute on a Projection op
            if is_select_attribute_binding(op, unit, &elements) {
                ops_to_remove.push(index);
            }
        }
    }

    // Remove ops in reverse order to maintain indices
    for index in ops_to_remove.iter().rev() {
        unit.update_mut().remove_at(*index);
    }
}

/// Check if a binding op is a 'select' attribute on a Projection
fn is_select_attribute_binding(
    op: &Box<dyn ir::UpdateOp + Send + Sync>,
    unit: &dyn crate::template::pipeline::src::compilation::CompilationUnit,
    elements: &std::collections::HashMap<ir::XrefId, usize>,
) -> bool {
    if op.kind() != OpKind::Binding {
        return false;
    }

    // Downcast to BindingOp to check name and target
    // This is safe because we've verified the kind
    unsafe {
        // Cast the box pointer to get the inner type pointer
        let binding_op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
        let binding_op = binding_op_ptr as *const BindingOp;
        let binding_op_ref = &*binding_op;

        // Check if name is 'select'
        if !is_select_attribute(&binding_op_ref.name) {
            return false;
        }

        // Check if target is a Projection op
        let target_xref = binding_op_ref.target;
        let target_op = lookup_element(unit, elements, target_xref);

        target_op.kind() == OpKind::Projection
    }
}

/// Helper to check if attribute name is 'select'
fn is_select_attribute(name: &str) -> bool {
    name.to_lowercase() == "select"
}
