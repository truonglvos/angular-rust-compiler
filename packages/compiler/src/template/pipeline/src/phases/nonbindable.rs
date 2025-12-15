//! Nonbindable Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/nonbindable.ts
//! When a container is marked with `ngNonBindable`, the non-bindable characteristic also applies to
//! all descendants of that container. Therefore, we must emit `disableBindings` and `enableBindings`
//! instructions for every such container.

use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::{ElementStartOp, ContainerStartOp, create_disable_bindings_op, create_enable_bindings_op};
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationUnit};
use crate::template::pipeline::src::util::elements::create_op_xref_map;
use std::collections::HashMap;

/// Looks up an element in the given map by xref ID.
fn lookup_element(
    elements: &HashMap<ir::XrefId, usize>,
    unit: &dyn CompilationUnit,
    xref: ir::XrefId,
) -> bool {
    let index = elements.get(&xref)
        .copied()
        .expect("All attributes should have an element-like target.");
    
    let op = unit.create()
        .get(index)
        .expect("Operation index out of bounds");
    
    // Check if the element/container is non-bindable
    match op.kind() {
        OpKind::ElementStart => {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let elem_start_ptr = op_ptr as *const ElementStartOp;
                let elem_start = &*elem_start_ptr;
                elem_start.base.base.non_bindable
            }
        }
        OpKind::Element => {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                // ElementOp has the same structure as ElementStartOp
                let elem_ptr = op_ptr as *const ElementStartOp;
                let elem = &*elem_ptr;
                elem.base.base.non_bindable
            }
        }
        OpKind::ContainerStart => {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                let container_start_ptr = op_ptr as *const ContainerStartOp;
                let container_start = &*container_start_ptr;
                container_start.base.non_bindable
            }
        }
        OpKind::Container => {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                // ContainerOp has the same structure as ContainerStartOp
                let container_ptr = op_ptr as *const ContainerStartOp;
                let container = &*container_ptr;
                container.base.non_bindable
            }
        }
        _ => false,
    }
}

/// When a container is marked with `ngNonBindable`, the non-bindable characteristic also applies to
/// all descendants of that container. Therefore, we must emit `disableBindings` and `enableBindings`
/// instructions for every such container.
pub fn disable_bindings(job: &mut dyn CompilationJob) {
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *job_ptr
    };
    
    // Process root unit
    {
        let root_elements = create_op_xref_map(&component_job.root as &dyn CompilationUnit);
        process_unit(&mut component_job.root, &root_elements);
    }
    
    // Process all view units
    for (_, unit) in component_job.views.iter_mut() {
        let elements = create_op_xref_map(unit as &dyn CompilationUnit);
        process_unit(unit, &elements);
    }
}

fn process_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    elements: &HashMap<ir::XrefId, usize>,
) {
    let mut insertions: Vec<(usize, ir::XrefId, bool)> = Vec::new(); // (index, xref, is_disable)
    
    // First pass: collect insertions
    for (index, op) in unit.create().iter().enumerate() {
        match op.kind() {
            OpKind::ElementStart | OpKind::ContainerStart => {
                let is_non_bindable = unsafe {
                    match op.kind() {
                        OpKind::ElementStart => {
                            let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                            let elem_start_ptr = op_ptr as *const ElementStartOp;
                            let elem_start = &*elem_start_ptr;
                            elem_start.base.base.non_bindable
                        }
                        OpKind::ContainerStart => {
                            let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                            let container_start_ptr = op_ptr as *const ContainerStartOp;
                            let container_start = &*container_start_ptr;
                            container_start.base.non_bindable
                        }
                        _ => false,
                    }
                };
                
                if is_non_bindable {
                    // Insert DisableBindingsOp after this start op
                    insertions.push((index + 1, op.xref(), true));
                }
            }
            OpKind::ElementEnd | OpKind::ContainerEnd => {
                let xref = op.xref();
                if lookup_element(elements, unit as &dyn CompilationUnit, xref) {
                    // Insert EnableBindingsOp before this end op
                    insertions.push((index, xref, false));
                }
            }
            _ => {}
        }
    }
    
    // Second pass: insert ops in reverse order (to maintain indices)
    for (index, xref, is_disable) in insertions.iter().rev() {
        let op = if *is_disable {
            create_disable_bindings_op(*xref)
        } else {
            create_enable_bindings_op(*xref)
        };
        unit.create_mut().insert_at(*index, op);
    }
}
