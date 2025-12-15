//! Shared Operations
//!
//! Corresponds to packages/compiler/src/template/pipeline/ir/src/ops/shared.ts
//! Defines shared operation types used by both CreateOp and UpdateOp

use crate::parse_util::ParseSourceSpan;
use crate::template::pipeline::ir::enums::{OpKind, VariableFlags};
use crate::template::pipeline::ir::handle::XrefId;
use crate::template::pipeline::ir::operations::{CreateOp, Op, UpdateOp};

/// List end operation - special marker for linked list boundaries
#[derive(Debug, Clone)]
pub struct ListEndOp<OpT> {
    pub kind: OpKind,
    pub next: Option<Box<OpT>>,
    pub prev: Option<Box<OpT>>,
    pub debug_list_id: Option<usize>,
}

impl<OpT: std::fmt::Debug + 'static> Op for ListEndOp<OpT> {
    fn kind(&self) -> OpKind {
        OpKind::ListEnd
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn source_span(&self) -> Option<&ParseSourceSpan> {
        None
    }
}

/// Implement CreateOp for ListEndOp when OpT is Box<dyn CreateOp + Send + Sync>
impl CreateOp for ListEndOp<Box<dyn CreateOp + Send + Sync>> {
    fn xref(&self) -> XrefId {
        // ListEndOp doesn't have an xref - return dummy value
        XrefId::new(0)
    }
}

/// Implement UpdateOp for ListEndOp when OpT is Box<dyn UpdateOp + Send + Sync>
impl UpdateOp for ListEndOp<Box<dyn UpdateOp + Send + Sync>> {
    fn xref(&self) -> XrefId {
        // ListEndOp doesn't have an xref - return dummy value
        XrefId::new(0)
    }
}

// Safe to implement Send + Sync for ListEndOp
unsafe impl<OpT: Send> Send for ListEndOp<OpT> {}
unsafe impl<OpT: Sync> Sync for ListEndOp<OpT> {}

/// Statement operation - wraps an output AST statement
#[derive(Debug, Clone)]
pub struct StatementOp<OpT> {
    pub kind: OpKind,
    pub statement: Box<crate::output::output_ast::Statement>,
    pub next: Option<Box<OpT>>,
    pub prev: Option<Box<OpT>>,
    pub debug_list_id: Option<usize>,
}

impl<OpT: std::fmt::Debug + 'static> Op for StatementOp<OpT> {
    fn kind(&self) -> OpKind {
        OpKind::Statement
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn source_span(&self) -> Option<&ParseSourceSpan> {
        // TODO: Extract source span from statement when available
        None
    }
}

/// Implement CreateOp for StatementOp when OpT is Box<dyn CreateOp + Send + Sync>
impl CreateOp for StatementOp<Box<dyn CreateOp + Send + Sync>> {
    fn xref(&self) -> XrefId {
        // StatementOp doesn't have an xref - return dummy value
        XrefId::new(0)
    }
}

/// Implement UpdateOp for StatementOp when OpT is Box<dyn UpdateOp + Send + Sync>
impl UpdateOp for StatementOp<Box<dyn UpdateOp + Send + Sync>> {
    fn xref(&self) -> XrefId {
        // StatementOp doesn't have an xref - return dummy value
        XrefId::new(0)
    }
}

// Safe to implement Send + Sync for StatementOp
unsafe impl<OpT: Send> Send for StatementOp<OpT> {}
unsafe impl<OpT: Sync> Sync for StatementOp<OpT> {}

/// Variable operation - declares and initializes a semantic variable
/// That is valid either in create or update IR.
#[derive(Debug, Clone)]
pub struct VariableOp<OpT> {
    pub kind: OpKind,
    /// `XrefId` which identifies this specific variable, and is used to reference this variable from
    /// other parts of the IR.
    pub xref: XrefId,
    /// The `SemanticVariable` which describes the meaning behind this variable.
    pub variable: crate::template::pipeline::ir::SemanticVariable,
    /// Expression representing the value of the variable.
    pub initializer: Box<crate::output::output_ast::Expression>,
    /// Flags controlling variable behavior (e.g., AlwaysInline).
    pub flags: VariableFlags,
    pub next: Option<Box<OpT>>,
    pub prev: Option<Box<OpT>>,
    pub debug_list_id: Option<usize>,
}

impl<OpT: std::fmt::Debug + 'static> Op for VariableOp<OpT> {
    fn kind(&self) -> OpKind {
        OpKind::Variable
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn source_span(&self) -> Option<&ParseSourceSpan> {
        None
    }
}

/// Implement CreateOp for VariableOp when OpT is Box<dyn CreateOp + Send + Sync>
impl CreateOp for VariableOp<Box<dyn CreateOp + Send + Sync>> {
    fn xref(&self) -> XrefId {
        self.xref
    }
}

/// Implement UpdateOp for VariableOp when OpT is Box<dyn UpdateOp + Send + Sync>
impl UpdateOp for VariableOp<Box<dyn UpdateOp + Send + Sync>> {
    fn xref(&self) -> XrefId {
        self.xref
    }
}

// Safe to implement Send + Sync for VariableOp
unsafe impl<OpT: Send> Send for VariableOp<OpT> {}
unsafe impl<OpT: Sync> Sync for VariableOp<OpT> {}

/// Create a StatementOp
pub fn create_statement_op<OpT>(
    statement: Box<crate::output::output_ast::Statement>,
) -> StatementOp<OpT> {
    StatementOp {
        kind: OpKind::Statement,
        statement,
        next: None,
        prev: None,
        debug_list_id: None,
    }
}

/// Create a VariableOp.
pub fn create_variable_op<OpT>(
    xref: XrefId,
    variable: crate::template::pipeline::ir::SemanticVariable,
    initializer: Box<crate::output::output_ast::Expression>,
    flags: VariableFlags,
) -> VariableOp<OpT> {
    VariableOp {
        kind: OpKind::Variable,
        xref,
        variable,
        initializer,
        flags,
        next: None,
        prev: None,
        debug_list_id: None,
    }
}
