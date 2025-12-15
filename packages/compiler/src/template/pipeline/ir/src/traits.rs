//! IR Traits
//!
//! Corresponds to packages/compiler/src/template/pipeline/ir/src/traits.ts
//! Defines marker traits for IR operations

use crate::parse_util::ParseSourceSpan;
use crate::template::pipeline::ir::handle::{SlotHandle, XrefId};

/// Marks an operation as requiring allocation of one or more data slots for storage.
pub trait ConsumesSlotOpTrait {
    /// Assigned data slot (the starting index, if more than one slot is needed) for this operation,
    /// or `None` if slots have not yet been assigned.
    fn handle(&self) -> &SlotHandle;
    
    /// The number of slots which will be used by this operation. By default 1, but can be increased
    /// if necessary.
    fn num_slots_used(&self) -> usize;
    
    /// `XrefId` of this operation (e.g. the element stored in the assigned slot). This `XrefId` is
    /// used to link this `ConsumesSlotOpTrait` operation with `DependsOnSlotContextOpTrait` or
    /// `UsesSlotIndexExprTrait` implementors and ensure that the assigned slot is propagated through
    /// the IR to all consumers.
    fn xref(&self) -> XrefId;
}

/// Marks an operation as depending on the runtime's implicit slot context being set to a particular
/// slot.
///
/// The runtime has an implicit slot context which is adjusted using the `advance()` instruction
/// during the execution of template update functions. This trait marks an operation as requiring
/// this implicit context to be `advance()`'d to point at a particular slot prior to execution.
pub trait DependsOnSlotContextOpTrait {
    /// `XrefId` of the `ConsumesSlotOpTrait` which the implicit slot context must reference before
    /// this operation can be executed.
    fn target(&self) -> XrefId;
    
    /// Source span for this operation
    fn source_span(&self) -> &ParseSourceSpan;
}

/// Marker trait indicating that an operation or expression consumes variable storage space.
pub trait ConsumesVarsTrait {
    // Marker trait - no methods needed
}

/// Marker trait indicating that an expression requires knowledge of the number of variable storage
/// slots used prior to it.
pub trait UsesVarOffsetTrait {
    /// Get the variable offset (the number of variable slots used prior to this expression)
    fn var_offset(&self) -> Option<usize>;
    
    /// Set the variable offset
    fn set_var_offset(&mut self, offset: Option<usize>);
}

/// Check if an operation implements ConsumesSlotOpTrait
pub fn has_consumes_slot_trait<T>(_op: &T) -> bool
where
    T: ConsumesSlotOpTrait,
{
    true
}

/// Check if an operation implements DependsOnSlotContextOpTrait
pub fn has_depends_on_slot_context_trait<T>(_op: &T) -> bool
where
    T: DependsOnSlotContextOpTrait,
{
    true
}

/// Check if an operation implements ConsumesVarsTrait
pub fn has_consumes_vars_trait<T>(_op: &T) -> bool
where
    T: ConsumesVarsTrait,
{
    true
}

/// Check if an expression implements UsesVarOffsetTrait
pub fn has_uses_var_offset_trait<T>(_expr: &T) -> bool
where
    T: UsesVarOffsetTrait,
{
    true
}
