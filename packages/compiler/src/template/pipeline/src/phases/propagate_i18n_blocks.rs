//! Propagate I18n Blocks Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/propagate_i18n_blocks.ts
//!
//! Propagate i18n blocks down through child templates that act as placeholders in the root i18n
//! message. Specifically, perform an in-order traversal of all the views, and add i18nStart/i18nEnd
//! op pairs into descending views. Also, assign an increasing sub-template index to each
//! descending view.

use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::{I18nStartOp, ConditionalCreateOp, ConditionalBranchCreateOp, TemplateOp, RepeaterCreateOp, ProjectionOp, create_i18n_start_op, create_i18n_end_op};
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, ViewCompilationUnit};

/// Propagate i18n blocks down through child templates.
pub fn propagate_i18n_blocks(job: &mut dyn CompilationJob) {
    if job.kind() != crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
        return;
    }
    
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let component_job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *component_job_ptr
    };
    
    let component_job_ptr = component_job as *mut ComponentCompilationJob;
    let root_ptr = &mut component_job.root as *mut ViewCompilationUnit;
    propagate_i18n_blocks_to_templates(unsafe { &mut *root_ptr }, 0, component_job_ptr);
}

/// Propagates i18n ops in the given view through to any child views recursively.
fn propagate_i18n_blocks_to_templates(
    unit: &mut ViewCompilationUnit,
    sub_template_index: usize,
    job_ptr: *mut ComponentCompilationJob,
) -> usize {
    let mut current_sub_template_index = sub_template_index;
    let mut i18n_block: Option<usize> = None;
    
    // Collect indices first to avoid borrow conflicts
    let ops_indices: Vec<usize> = (0..unit.create.len()).collect();
    
    for idx in ops_indices {
        let op = unit.create.get(idx).unwrap();
        match op.kind() {
            OpKind::I18nStart => {
                if current_sub_template_index == 0 {
                    i18n_block = Some(idx);
                } else {
                    // Modify sub_template_index in place
                    unsafe {
                        let op_mut_ptr = unit.create.get_mut(idx).unwrap().as_mut() as *mut dyn ir::CreateOp;
                        let i18n_mut_ptr = op_mut_ptr as *mut I18nStartOp;
                        let i18n_mut = &mut *i18n_mut_ptr;
                        i18n_mut.base.sub_template_index = Some(current_sub_template_index);
                    }
                    i18n_block = Some(idx);
                }
            }
            OpKind::I18nEnd => {
                if let Some(i18n_idx) = i18n_block {
                    unsafe {
                        let op_ptr = unit.create.get(i18n_idx).unwrap().as_ref() as *const dyn ir::CreateOp;
                        let i18n_ptr = op_ptr as *const I18nStartOp;
                        let i18n = &*i18n_ptr;
                        
                        if i18n.base.sub_template_index.is_none() {
                            current_sub_template_index = 0;
                        }
                    }
                }
                i18n_block = None;
            }
            OpKind::ConditionalCreate | OpKind::ConditionalBranchCreate | OpKind::Template => {
                unsafe {
                    let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                    let view_xref = op.xref();
                    let job = &mut *job_ptr;
                    
                    if let Some(view) = job.views.get_mut(&view_xref) {
                        let i18n_placeholder = match op.kind() {
                            OpKind::ConditionalCreate => {
                                let cond_ptr = op_ptr as *const ConditionalCreateOp;
                                let cond = &*cond_ptr;
                                cond.i18n_placeholder.clone()
                            }
                            OpKind::ConditionalBranchCreate => {
                                let branch_ptr = op_ptr as *const ConditionalBranchCreateOp;
                                let branch = &*branch_ptr;
                                branch.i18n_placeholder.clone()
                            }
                            OpKind::Template => {
                                let template_ptr = op_ptr as *const TemplateOp;
                                let template = &*template_ptr;
                                template.i18n_placeholder.clone()
                            }
                            _ => None,
                        };
                        
                        let parent_i18n = if let Some(i18n_idx) = i18n_block {
                            let op_ptr = unit.create.get(i18n_idx).unwrap().as_ref() as *const dyn ir::CreateOp;
                            let i18n_ptr = op_ptr as *const I18nStartOp;
                            Some(&*i18n_ptr)
                        } else {
                            None
                        };
                        
                        current_sub_template_index = propagate_i18n_blocks_for_view(
                            view,
                            parent_i18n,
                            i18n_placeholder.as_ref().map(|p| p as &dyn std::any::Any),
                            current_sub_template_index,
                            job_ptr,
                        );
                    }
                }
            }
            OpKind::RepeaterCreate => {
                unsafe {
                    let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                    let repeater_ptr = op_ptr as *const RepeaterCreateOp;
                    let repeater = &*repeater_ptr;
                    let job = &mut *job_ptr;
                    
                    let view_xref = repeater.base.base.xref;
                    if let Some(for_view) = job.views.get_mut(&view_xref) {
                        let parent_i18n = if let Some(i18n_idx) = i18n_block {
                            let op_ptr = unit.create.get(i18n_idx).unwrap().as_ref() as *const dyn ir::CreateOp;
                            let i18n_ptr = op_ptr as *const I18nStartOp;
                            Some(&*i18n_ptr)
                        } else {
                            None
                        };
                        
                        current_sub_template_index = propagate_i18n_blocks_for_view(
                            for_view,
                            parent_i18n,
                            repeater.i18n_placeholder.as_ref().map(|p| p as &dyn std::any::Any),
                            current_sub_template_index,
                            job_ptr,
                        );
                    }
                    
                    // Then if there's an @empty template, propagate the i18n blocks for it as well.
                    if let Some(empty_view_xref) = repeater.empty_view {
                        if let Some(empty_view) = job.views.get_mut(&empty_view_xref) {
                            let parent_i18n = if let Some(i18n_idx) = i18n_block {
                                let op_ptr = unit.create.get(i18n_idx).unwrap().as_ref() as *const dyn ir::CreateOp;
                                let i18n_ptr = op_ptr as *const I18nStartOp;
                                Some(&*i18n_ptr)
                            } else {
                                None
                            };
                            
                            current_sub_template_index = propagate_i18n_blocks_for_view(
                                empty_view,
                                parent_i18n,
                                repeater.empty_i18n_placeholder.as_ref().map(|p| p as &dyn std::any::Any),
                                current_sub_template_index,
                                job_ptr,
                            );
                        }
                    }
                }
            }
            OpKind::Projection => {
                unsafe {
                    let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                    let projection_ptr = op_ptr as *const ProjectionOp;
                    let projection = &*projection_ptr;
                    let job = &mut *job_ptr;
                    
                    if let Some(fallback_view_xref) = projection.fallback_view {
                        if let Some(fallback_view) = job.views.get_mut(&fallback_view_xref) {
                            let parent_i18n = if let Some(i18n_idx) = i18n_block {
                                let op_ptr = unit.create.get(i18n_idx).unwrap().as_ref() as *const dyn ir::CreateOp;
                                let i18n_ptr = op_ptr as *const I18nStartOp;
                                Some(&*i18n_ptr)
                            } else {
                                None
                            };
                            
                            current_sub_template_index = propagate_i18n_blocks_for_view(
                                fallback_view,
                                parent_i18n,
                                projection.fallback_view_i18n_placeholder.as_ref().map(|p| p as &dyn std::any::Any),
                                current_sub_template_index,
                                job_ptr,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    current_sub_template_index
}

/// Propagate i18n blocks for a view.
fn propagate_i18n_blocks_for_view(
    view: &mut ViewCompilationUnit,
    i18n_block: Option<&I18nStartOp>,
    i18n_placeholder: Option<&dyn std::any::Any>,
    sub_template_index: usize,
    job_ptr: *mut ComponentCompilationJob,
) -> usize {
    // We found an <ng-template> inside an i18n block; increment the sub-template counter and
    // wrap the template's view in a child i18n block.
    if i18n_placeholder.is_some() {
        if i18n_block.is_none() {
            panic!("Expected template with i18n placeholder to be in an i18n block.");
        }
        let new_sub_template_index = sub_template_index + 1;
        wrap_template_with_i18n(view, i18n_block.unwrap(), job_ptr);
        return propagate_i18n_blocks_to_templates(view, new_sub_template_index, job_ptr);
    }

    // Continue traversing inside the template's view.
    propagate_i18n_blocks_to_templates(view, sub_template_index, job_ptr)
}

/// Wraps a template view with i18n start and end ops.
fn wrap_template_with_i18n(
    unit: &mut ViewCompilationUnit,
    parent_i18n: &I18nStartOp,
    job_ptr: *mut ComponentCompilationJob,
) {
    // Only add i18n ops if they have not already been propagated to this template.
    if unit.create.is_empty() || unit.create.get(0).unwrap().kind() != OpKind::I18nStart {
        unsafe {
            let job = &mut *job_ptr;
            let id = job.allocate_xref_id();
            // Nested ng-template i18n start/end ops should not receive source spans.
            let i18n_start_op = create_i18n_start_op(
                id,
                parent_i18n.base.message.clone(),
                Some(parent_i18n.base.root),
                None, // source_span
            );
            let i18n_end_op = create_i18n_end_op(id, None);
            
            unit.create.insert_at(0, i18n_start_op);
            unit.create.push(i18n_end_op);
        }
    }
}
