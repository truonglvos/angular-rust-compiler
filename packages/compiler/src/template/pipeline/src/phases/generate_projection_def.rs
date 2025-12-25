//! Generate projection definitions.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/generate_projection_def.ts
//!
//! Locate projection slots, populate the each component's `ngContentSelectors` literal field,
//! populate `project` arguments, and generate the required `projectionDef` instruction for the job's
//! root view.

use crate::output::output_ast::Expression;
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::ops::create::{create_projection_def_op, ProjectionOp};
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob};
use crate::template::pipeline::src::conversion::{literal_or_array_literal, LiteralType};

pub fn generate_projection_defs(job: &mut dyn CompilationJob) {
    if job.kind() != crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
        return;
    }

    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let component_job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *component_job_ptr
    };

    // TODO: Why does TemplateDefinitionBuilder force a shared constant?
    let share = component_job.compatibility() == ir::CompatibilityMode::TemplateDefinitionBuilder;

    // Collect all selectors from this component, and its nested views. Also, assign each projection a
    // unique ascending projection slot index.
    let mut selectors = Vec::new();
    let mut projection_slot_index = 0;

    // Process root view
    for op in component_job.root.create.iter_mut() {
        if op.kind() == ir::OpKind::Projection {
            unsafe {
                let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                let projection_ptr = op_ptr as *mut ProjectionOp;
                let projection = &mut *projection_ptr;

                selectors.push(projection.selector.clone());
                projection.projection_slot_index = projection_slot_index;
                projection_slot_index += 1;
            }
        }
    }

    // Process embedded views
    for view in component_job.views.values_mut() {
        for op in view.create.iter_mut() {
            if op.kind() == ir::OpKind::Projection {
                unsafe {
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let projection_ptr = op_ptr as *mut ProjectionOp;
                    let projection = &mut *projection_ptr;

                    selectors.push(projection.selector.clone());
                    projection.projection_slot_index = projection_slot_index;
                    projection_slot_index += 1;
                }
            }
        }
    }

    if !selectors.is_empty() {
        // Create the projectionDef array. If we only found a single wildcard selector, then we use the
        // default behavior with no arguments instead.
        let def_expr: Option<Expression> = if selectors.len() > 1 || selectors[0] != "*" {
            // Parse selectors to R3 selector format
            let def: Vec<LiteralType> = selectors
                .iter()
                .map(|s| {
                    if s == "*" {
                        LiteralType::String(s.clone())
                    } else {
                        // Parse selector to R3 selector format
                        // R3 selector is a nested array: [[element, attr1, attr2, ...], ...]
                        // For simple selectors, we'll create a basic structure
                        // TODO: Implement full selector parsing (parseSelectorToR3Selector)
                        LiteralType::Array(vec![
                            LiteralType::String("".to_string()), // namespace
                            LiteralType::String(s.clone()),      // element/selector
                            LiteralType::String("".to_string()), // attrs (empty for now)
                        ])
                    }
                })
                .collect();

            let expr = component_job
                .pool
                .get_const_literal(literal_or_array_literal(LiteralType::Array(def)), share);
            Some(expr)
        } else {
            None
        };

        // Create the ngContentSelectors constant
        let selectors_literal: Vec<LiteralType> = selectors
            .iter()
            .map(|s| LiteralType::String(s.clone()))
            .collect();

        component_job.content_selectors = Some(component_job.pool.get_const_literal(
            literal_or_array_literal(LiteralType::Array(selectors_literal)),
            share,
        ));

        // The projection def instruction goes at the beginning of the root view, before any
        // `projection` instructions.
        let def_op = create_projection_def_op(def_expr);
        component_job.root.create.prepend(vec![def_op]);
    }
}
