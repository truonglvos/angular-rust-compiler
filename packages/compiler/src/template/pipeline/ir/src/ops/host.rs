//! Host Operations
//!
//! Corresponds to packages/compiler/src/template/pipeline/ir/src/ops/host.ts
//! Defines host binding operations

use crate::core::SecurityContext;
use crate::parse_util::ParseSourceSpan;
use crate::template::pipeline::ir::enums::{BindingKind, OpKind};
use crate::template::pipeline::ir::handle::XrefId;
use crate::template::pipeline::ir::operations::{Op, UpdateOp};
use crate::template::pipeline::ir::ops::update::BindingExpression;
use crate::template::pipeline::ir::traits::ConsumesVarsTrait;

/// Logical operation representing a binding to a native DOM property.
#[derive(Debug, Clone)]
pub struct DomPropertyOp {
    /// Name of the property
    pub name: String,
    /// Expression which is bound to the property
    pub expression: BindingExpression,
    /// Binding kind
    pub binding_kind: BindingKind,
    /// i18n context XrefId
    pub i18n_context: Option<XrefId>,
    /// The security context of the binding
    pub security_context: Vec<SecurityContext>,
    /// The sanitizer for this property
    pub sanitizer: Option<crate::output::output_ast::Expression>,
    /// Source span
    pub source_span: ParseSourceSpan,
}

impl DomPropertyOp {
    pub fn new(
        name: String,
        expression: BindingExpression,
        binding_kind: BindingKind,
        i18n_context: Option<XrefId>,
        security_context: Vec<SecurityContext>,
        source_span: ParseSourceSpan,
    ) -> Self {
        DomPropertyOp {
            name,
            expression,
            binding_kind,
            i18n_context,
            security_context,
            sanitizer: None,
            source_span,
        }
    }
}

impl Op for DomPropertyOp {
    fn kind(&self) -> OpKind {
        OpKind::DomProperty
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn source_span(&self) -> Option<&ParseSourceSpan> {
        Some(&self.source_span)
    }
}

impl UpdateOp for DomPropertyOp {
    fn xref(&self) -> XrefId {
        // DomPropertyOp doesn't have a meaningful xref - return dummy value
        XrefId::new(0)
    }
}

impl ConsumesVarsTrait for DomPropertyOp {}

unsafe impl Send for DomPropertyOp {}
unsafe impl Sync for DomPropertyOp {}

/// Create a DomPropertyOp
pub fn create_dom_property_op(
    name: String,
    expression: BindingExpression,
    binding_kind: BindingKind,
    i18n_context: Option<XrefId>,
    security_context: Vec<SecurityContext>,
    source_span: ParseSourceSpan,
) -> Box<dyn UpdateOp + Send + Sync> {
    Box::new(DomPropertyOp::new(
        name,
        expression,
        binding_kind,
        i18n_context,
        security_context,
        source_span,
    ))
}
