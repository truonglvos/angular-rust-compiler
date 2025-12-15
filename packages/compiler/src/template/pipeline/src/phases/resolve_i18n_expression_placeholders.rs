//! Resolve I18n Expression Placeholders Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/resolve_i18n_expression_placeholders.ts
//!
//! Resolve the i18n expression placeholders in i18n messages.

use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::enums::{OpKind, I18nExpressionFor, I18nParamResolutionTime, I18nParamValueFlags};
use crate::template::pipeline::ir::ops::create::{I18nStartOp, I18nContextOp, IcuPlaceholderOp};
use crate::template::pipeline::ir::ops::update::I18nExpressionOp;
use crate::template::pipeline::ir::ops::create::I18nParamValue;
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationJobKind};

/// Resolve the i18n expression placeholders in i18n messages.
pub fn resolve_i18n_expression_placeholders(job: &mut dyn CompilationJob) {
    if job.kind() != CompilationJobKind::Tmpl {
        return;
    }
    
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let component_job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *component_job_ptr
    };
    
    // Record all of the i18n context ops, and the sub-template index for each i18n op.
    let mut sub_template_indices: std::collections::HashMap<ir::XrefId, Option<usize>> = std::collections::HashMap::new();
    let mut i18n_contexts: std::collections::HashMap<ir::XrefId, I18nContextOp> = std::collections::HashMap::new();
    
    // Collect from root unit
    for op in component_job.root.create.iter() {
        match op.kind() {
            OpKind::I18nStart => {
                unsafe {
                    let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                    let i18n_ptr = op_ptr as *const I18nStartOp;
                    let i18n = &*i18n_ptr;
                    sub_template_indices.insert(i18n.base.xref, i18n.base.sub_template_index);
                }
            }
            OpKind::I18nContext => {
                unsafe {
                    let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                    let ctx_ptr = op_ptr as *const I18nContextOp;
                    let ctx = &*ctx_ptr;
                    i18n_contexts.insert(ctx.xref, ctx.clone());
                }
            }
            _ => {}
        }
    }
    
    // Collect from all view units
    for (_, unit) in component_job.views.iter() {
        for op in unit.create.iter() {
            match op.kind() {
                OpKind::I18nStart => {
                    unsafe {
                        let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                        let i18n_ptr = op_ptr as *const I18nStartOp;
                        let i18n = &*i18n_ptr;
                        sub_template_indices.insert(i18n.base.xref, i18n.base.sub_template_index);
                    }
                }
                OpKind::I18nContext => {
                    unsafe {
                        let op_ptr = op.as_ref() as *const dyn ir::CreateOp;
                        let ctx_ptr = op_ptr as *const I18nContextOp;
                        let ctx = &*ctx_ptr;
                        i18n_contexts.insert(ctx.xref, ctx.clone());
                    }
                }
                _ => {}
            }
        }
    }
    
    // Keep track of the next available expression index for each i18n message.
    let mut expression_indices: std::collections::HashMap<ir::XrefId, usize> = std::collections::HashMap::new();
    
    // Keep track of a reference index for each expression.
    // We use different references for normal i18n expression and attribute i18n expressions. This is
    // because child i18n blocks in templates don't get their own context, since they're rolled into
    // the translated message of the parent, but they may target a different slot.
    fn reference_index(op: &I18nExpressionOp) -> ir::XrefId {
        if op.usage == I18nExpressionFor::I18nText {
            op.i18n_owner
        } else {
            op.context
        }
    }
    
    // Process root unit - collect first
    let mut root_updates: Vec<(I18nExpressionOp, I18nParamValue, ir::XrefId)> = Vec::new();
    for op in component_job.root.update.iter() {
        if op.kind() == OpKind::I18nExpression {
            unsafe {
                let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                let expr_op_ptr = op_ptr as *const I18nExpressionOp;
                let expr_op = &*expr_op_ptr;
                
                let ref_idx = reference_index(expr_op);
                let index = expression_indices.get(&ref_idx).copied().unwrap_or(0);
                let sub_template_index = sub_template_indices.get(&expr_op.i18n_owner).copied().flatten();
                
                let value = I18nParamValue {
                    value: ir::ops::create::I18nParamValueValue::Number(index),
                    sub_template_index,
                    flags: I18nParamValueFlags::EXPRESSION_INDEX,
                };
                
                root_updates.push((expr_op.clone(), value, ref_idx));
                expression_indices.insert(ref_idx, index + 1);
            }
        }
    }
    
    // Apply updates to root unit
    for (expr_op, value, _) in root_updates {
        update_placeholder(
            &expr_op,
            value,
            &mut i18n_contexts,
            &mut component_job.root,
        );
    }
    
    // Process all view units - collect first
    let mut view_updates: Vec<(ir::XrefId, I18nExpressionOp, I18nParamValue, ir::XrefId)> = Vec::new();
    for (view_key, unit) in component_job.views.iter() {
        for op in unit.update.iter() {
            if op.kind() == OpKind::I18nExpression {
                unsafe {
                    let op_ptr = op.as_ref() as *const dyn ir::UpdateOp;
                    let expr_op_ptr = op_ptr as *const I18nExpressionOp;
                    let expr_op = &*expr_op_ptr;
                    
                    let ref_idx = reference_index(expr_op);
                    let index = expression_indices.get(&ref_idx).copied().unwrap_or(0);
                    let sub_template_index = sub_template_indices.get(&expr_op.i18n_owner).copied().flatten();
                    
                    let value = I18nParamValue {
                        value: ir::ops::create::I18nParamValueValue::Number(index),
                        sub_template_index,
                        flags: I18nParamValueFlags::EXPRESSION_INDEX,
                    };
                    
                    view_updates.push((*view_key, expr_op.clone(), value, ref_idx));
                    expression_indices.insert(ref_idx, index + 1);
                }
            }
        }
    }
    
    // Apply updates to view units
    for (view_key, expr_op, value, _) in view_updates {
        if let Some(unit) = component_job.views.get_mut(&view_key) {
            update_placeholder(
                &expr_op,
                value,
                &mut i18n_contexts,
                unit,
            );
        }
    }
}

fn update_placeholder(
    op: &I18nExpressionOp,
    value: I18nParamValue,
    i18n_contexts: &mut std::collections::HashMap<ir::XrefId, I18nContextOp>,
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
) {
    if let Some(ref i18n_placeholder) = op.i18n_placeholder {
        if let Some(i18n_context) = i18n_contexts.get_mut(&op.context) {
            let params = if op.resolution_time == I18nParamResolutionTime::Creation {
                &mut i18n_context.params
            } else {
                &mut i18n_context.postprocessing_params
            };
            
            let values = params.entry(i18n_placeholder.clone()).or_insert_with(Vec::new);
            values.push(value.clone());
        }
    }
    
    if let Some(icu_placeholder_xref) = op.icu_placeholder {
        // Find and update IcuPlaceholderOp in the unit
        for create_op in unit.create.iter_mut() {
            if create_op.kind() == OpKind::IcuPlaceholder && create_op.xref() == icu_placeholder_xref {
                unsafe {
                    let op_ptr = create_op.as_mut() as *mut dyn ir::CreateOp;
                    let icu_ptr = op_ptr as *mut IcuPlaceholderOp;
                    let icu = &mut *icu_ptr;
                    icu.expression_placeholders.push(value);
                }
                break;
            }
        }
    }
}

