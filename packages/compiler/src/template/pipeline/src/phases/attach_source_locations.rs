//! Attach Source Locations Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/attach_source_locations.ts
//! Locates all elements defined in a creation block and outputs an op that exposes their definition locations

use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::{ElementStartOp, ElementOp, ElementSourceLocation, create_source_location_op};
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationUnit};

/// Locates all of the elements defined in a creation block and outputs an op
/// that will expose their definition location in the DOM.
pub fn attach_source_locations(job: &mut dyn CompilationJob) {
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *job_ptr
    };
    
    if !component_job.enable_debug_locations || component_job.relative_template_path.is_none() {
        return;
    }
    
    let template_path = component_job.relative_template_path.as_ref().unwrap().clone();
    
    // Process root unit
    process_unit(&mut component_job.root, &template_path);
    
    // Process all view units
    for (_, unit) in component_job.views.iter_mut() {
        process_unit(unit, &template_path);
    }
}

fn process_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    template_path: &str,
) {
    let mut locations: Vec<ElementSourceLocation> = Vec::new();
    
    for op in unit.create().iter() {
        if op.kind() == OpKind::ElementStart {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let elem_start_ptr = op_ptr as *const ElementStartOp;
                let elem_start = &*elem_start_ptr;
                
                let start = &elem_start.base.base.start_source_span.start;
                locations.push(ElementSourceLocation {
                    target_slot: elem_start.base.base.handle.clone(),
                    offset: start.offset,
                    line: start.line,
                    column: start.col,
                });
            }
        } else if op.kind() == OpKind::Element {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let elem_ptr = op_ptr as *const ElementOp;
                let elem = &*elem_ptr;
                
                let start = &elem.base.base.start_source_span.start;
                locations.push(ElementSourceLocation {
                    target_slot: elem.base.base.handle.clone(),
                    offset: start.offset,
                    line: start.line,
                    column: start.col,
                });
            }
        }
    }
    
    if !locations.is_empty() {
        let source_location_op = create_source_location_op(template_path.to_string(), locations);
        unit.create_mut().push(source_location_op);
    }
}

