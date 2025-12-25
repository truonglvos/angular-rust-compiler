//! Resolve Sanitizers Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/resolve_sanitizers.ts
//! Resolves sanitization functions for ops that need them.

use crate::core::SecurityContext;
use crate::output::output_ast::{Expression, ExternalExpr, ExternalReference};
use crate::render3::r3_identifiers::Identifiers;
use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::host::DomPropertyOp;
use crate::template::pipeline::ir::ops::update::{AttributeOp, PropertyOp};
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, ComponentCompilationJob,
};

/// Map of security contexts to their sanitizer function.
fn get_sanitizer_fn(security_context: SecurityContext) -> Option<ExternalReference> {
    match security_context {
        SecurityContext::HTML => Some(Identifiers::sanitize_html()),
        SecurityContext::ResourceUrl => Some(Identifiers::sanitize_resource_url()),
        SecurityContext::SCRIPT => Some(Identifiers::sanitize_script()),
        SecurityContext::STYLE => Some(Identifiers::sanitize_style()),
        SecurityContext::URL => Some(Identifiers::sanitize_url()),
        SecurityContext::NONE => None,
        // Note: ATTRIBUTE_NO_BINDING is not yet in SecurityContext enum in Rust version
        // When added, it should map to Identifiers::validate_attribute()
    }
}

/// Map of security contexts to their trusted value function.
fn get_trusted_value_fn(security_context: SecurityContext) -> Option<ExternalReference> {
    match security_context {
        SecurityContext::HTML => Some(Identifiers::trust_constant_html()),
        SecurityContext::ResourceUrl => Some(Identifiers::trust_constant_resource_url()),
        _ => None,
    }
}

/// Convert ExternalReference to Expression
fn import_expr(external_ref: ExternalReference) -> Expression {
    Expression::External(ExternalExpr {
        value: external_ref,
        type_: None,
        source_span: None,
    })
}

/// Resolves sanitization functions for ops that need them.
pub fn resolve_sanitizers(job: &mut dyn CompilationJob) {
    let component_job = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        let job_ptr = job_ptr as *mut ComponentCompilationJob;
        &mut *job_ptr
    };

    // Process root unit
    process_unit(
        &mut component_job.root,
        job.kind() != CompilationJobKind::Host,
    );

    // Process all view units
    for (_, unit) in component_job.views.iter_mut() {
        process_unit(unit, job.kind() != CompilationJobKind::Host);
    }
}

fn process_unit(
    unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit,
    is_not_host: bool,
) {
    // For normal element bindings we create trusted values for security sensitive constant
    // attributes. However, for host bindings we skip this step (this matches what
    // TemplateDefinitionBuilder does).
    if is_not_host {
        for op in unit.create.iter_mut() {
            if op.kind() == OpKind::ExtractedAttribute {
                unsafe {
                    use crate::template::pipeline::ir::ops::create::ExtractedAttributeOp;
                    let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
                    let extracted_attr_ptr = op_ptr as *mut ExtractedAttributeOp;
                    let extracted_attr = &mut *extracted_attr_ptr;

                    let trusted_value_fn =
                        get_only_security_context(&extracted_attr.security_context)
                            .and_then(get_trusted_value_fn);

                    extracted_attr.trusted_value_fn = trusted_value_fn.map(import_expr);
                }
            }
        }
    }

    // Process update ops
    for op in unit.update.iter_mut() {
        match op.kind() {
            OpKind::Property | OpKind::Attribute | OpKind::DomProperty => {
                unsafe {
                    // Get sanitizer function based on op's security context
                    // op is &Box<dyn UpdateOp>, so we need to dereference it
                    let sanitizer_fn = get_sanitizer_for_op(op.as_ref() as &dyn ir::UpdateOp);

                    // Apply sanitizer to the appropriate op type
                    match op.kind() {
                        OpKind::Property => {
                            let op_ptr = op.as_mut() as *mut dyn ir::UpdateOp;
                            let prop_ptr = op_ptr as *mut PropertyOp;
                            let prop = &mut *prop_ptr;
                            prop.sanitizer = sanitizer_fn.map(import_expr);
                        }
                        OpKind::Attribute => {
                            let op_ptr = op.as_mut() as *mut dyn ir::UpdateOp;
                            let attr_ptr = op_ptr as *mut AttributeOp;
                            let attr = &mut *attr_ptr;
                            attr.sanitizer = sanitizer_fn.map(import_expr);
                        }
                        OpKind::DomProperty => {
                            let op_ptr = op.as_mut() as *mut dyn ir::UpdateOp;
                            let dom_prop_ptr = op_ptr as *mut DomPropertyOp;
                            let dom_prop = &mut *dom_prop_ptr;
                            dom_prop.sanitizer = sanitizer_fn.map(import_expr);
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => {}
        }
    }
}

/// Get sanitizer function for an op based on its security context
fn get_sanitizer_for_op(op: &dyn ir::UpdateOp) -> Option<ExternalReference> {
    // Extract security context from the op based on its type
    let security_context = match op.kind() {
        OpKind::Property => unsafe {
            let op_ptr = op as *const dyn ir::UpdateOp;
            let prop_ptr = op_ptr as *const PropertyOp;
            let prop = &*prop_ptr;
            prop.security_context.clone()
        },
        OpKind::Attribute => unsafe {
            let op_ptr = op as *const dyn ir::UpdateOp;
            let attr_ptr = op_ptr as *const AttributeOp;
            let attr = &*attr_ptr;
            attr.security_context.clone()
        },
        OpKind::DomProperty => unsafe {
            let op_ptr = op as *const dyn ir::UpdateOp;
            let dom_prop_ptr = op_ptr as *const DomPropertyOp;
            let dom_prop = &*dom_prop_ptr;
            dom_prop.security_context.clone()
        },
        _ => return None,
    };

    // Check for special case: URL and RESOURCE_URL together
    // When the host element isn't known, some URL attributes (such as "src" and "href") may
    // be part of multiple different security contexts. In this case we use special
    // sanitization function and select the actual sanitizer at runtime based on a tag name
    // that is provided while invoking sanitization function.
    if security_context.len() == 2
        && security_context.contains(&SecurityContext::URL)
        && security_context.contains(&SecurityContext::ResourceUrl)
    {
        return Some(Identifiers::sanitize_url_or_resource_url());
    }

    // Get single security context and map to sanitizer function
    get_only_security_context(&security_context).and_then(get_sanitizer_fn)
}

/// Asserts that there is only a single security context and returns it.
///
/// This function handles the case where multiple security contexts are present.
/// Currently, we only have a special case for URL/ResourceUrl combination.
/// For other ambiguous cases, we throw an error to ensure they are properly handled.
///
/// In the original TemplateDefinitionBuilder (TDB), it would just take the first context,
/// but we prefer to be explicit about these cases to ensure security is handled correctly.
fn get_only_security_context(security_context: &[SecurityContext]) -> Option<SecurityContext> {
    if security_context.len() > 1 {
        // Check if this is the known special case (URL + ResourceUrl)
        // This case is already handled in get_sanitizer_for_op before calling this function
        // So if we reach here with multiple contexts, it's an unexpected case

        // Log the contexts for debugging
        let contexts_str: Vec<String> = security_context
            .iter()
            .map(|ctx| format!("{:?}", ctx))
            .collect();

        panic!(
            "AssertionError: Ambiguous security context: {:?}. \
            This case should be handled explicitly. If this is a valid case, \
            please add a special case handler similar to URL/ResourceUrl.",
            contexts_str
        );
    }
    security_context.first().copied()
}
