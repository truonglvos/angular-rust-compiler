//! Generate projection definitions.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/generate_projection_def.ts
//!
//! Locate projection slots, populate the each component's `ngContentSelectors` literal field,
//! populate `project` arguments, and generate the required `projectionDef` instruction for the job's
//! root view.

use crate::core::SelectorFlags;
use crate::directive_matching::CssSelector;
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
            // ProjectionDef = (string | R3CssSelector[])[]
            let def: Vec<LiteralType> = selectors
                .iter()
                .map(|s| {
                    if s == "*" {
                        LiteralType::String(s.clone())
                    } else if let Ok(css_selectors) = CssSelector::parse(s) {
                        // R3CssSelector[]
                        let selectors_list: Vec<LiteralType> = css_selectors
                            .iter()
                            .map(|selector| {
                                let mut vec = Vec::new();
                                // Element name
                                vec.push(LiteralType::String(
                                    selector.element.clone().unwrap_or_default(),
                                ));

                                // Classes
                                for class_name in &selector.class_names {
                                    vec.push(LiteralType::Number(
                                        SelectorFlags::CLASS as u32 as f64,
                                    ));
                                    vec.push(LiteralType::String(class_name.clone()));
                                }

                                // Attributes
                                for i in (0..selector.attrs.len()).step_by(2) {
                                    vec.push(LiteralType::Number(
                                        SelectorFlags::ATTRIBUTE as u32 as f64,
                                    ));
                                    vec.push(LiteralType::String(selector.attrs[i].clone()));
                                    vec.push(LiteralType::String(selector.attrs[i + 1].clone()));
                                }

                                // Not Selectors
                                for not_selector in &selector.not_selectors {
                                    // Element
                                    if let Some(element) = &not_selector.element {
                                        if element != "*" {
                                            vec.push(LiteralType::Number(
                                                (SelectorFlags::NOT as u32
                                                    | SelectorFlags::ELEMENT as u32)
                                                    as f64,
                                            ));
                                            vec.push(LiteralType::String(element.clone()));
                                        }
                                    }

                                    // Classes
                                    for class_name in &not_selector.class_names {
                                        vec.push(LiteralType::Number(
                                            (SelectorFlags::NOT as u32
                                                | SelectorFlags::CLASS as u32)
                                                as f64,
                                        ));
                                        vec.push(LiteralType::String(class_name.clone()));
                                    }

                                    // Attributes
                                    for i in (0..not_selector.attrs.len()).step_by(2) {
                                        vec.push(LiteralType::Number(
                                            (SelectorFlags::NOT as u32
                                                | SelectorFlags::ATTRIBUTE as u32)
                                                as f64,
                                        ));
                                        vec.push(LiteralType::String(
                                            not_selector.attrs[i].clone(),
                                        ));
                                        vec.push(LiteralType::String(
                                            not_selector.attrs[i + 1].clone(),
                                        ));
                                    }
                                }

                                LiteralType::Array(vec)
                            })
                            .collect();

                        LiteralType::Array(selectors_list)
                    } else {
                        // Fallback if parse fails (shouldn't happen with valid selectors)
                        LiteralType::Array(vec![LiteralType::Array(vec![LiteralType::String(
                            s.clone(),
                        )])])
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
