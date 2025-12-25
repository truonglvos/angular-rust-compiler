//! IR Handles
//!
//! Corresponds to packages/compiler/src/template/pipeline/ir/src/handle.ts
//! Defines handles and IDs used in the IR

/// Branded type for a cross-reference ID. During ingest, `XrefId`s are generated to link together
/// different IR operations which need to reference each other.
///
/// Note: In TypeScript, this is defined as `export type XrefId = number & {__brand: 'XrefId'};`
/// In Rust, we use a newtype struct to achieve type safety.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct XrefId(pub usize);

impl XrefId {
    pub fn new(id: usize) -> Self {
        XrefId(id)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// Branded type for a constant index.
/// This is used to index into the consts array.
/// Note: This type is not explicitly defined in the TypeScript handle.ts, but is used throughout
/// the codebase as a branded type for constant indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConstIndex(pub usize);

impl ConstIndex {
    pub fn new(index: usize) -> Self {
        ConstIndex(index)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// Slot handle for operations that consume slots.
///
/// In TypeScript, this is defined as a class with a nullable `slot` field:
/// ```typescript
/// export class SlotHandle {
///   slot: number | null = null;
/// }
/// ```
///
/// In Rust, we use a struct with an `Option<usize>` to represent the nullable slot value.
/// The `handle` field in operations is typically initialized with `SlotHandle::new()` which creates
/// a handle with `slot: None`, matching the TypeScript default value of `null`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotHandle {
    /// The slot number, or `None` if slots have not yet been assigned.
    /// This corresponds to the `slot: number | null` field in TypeScript.
    pub slot: Option<usize>,
}

impl SlotHandle {
    /// Create a new SlotHandle with no slot assigned (slot = None/null).
    /// This matches the TypeScript default: `slot: number | null = null`
    pub fn new() -> Self {
        SlotHandle { slot: None }
    }

    /// Create a SlotHandle with a specific slot number.
    pub fn with_slot(slot: usize) -> Self {
        SlotHandle { slot: Some(slot) }
    }

    /// Get the slot number if assigned, or None if not yet assigned.
    pub fn get_slot(&self) -> Option<usize> {
        self.slot
    }

    /// Set the slot number.
    pub fn set_slot(&mut self, slot: usize) {
        self.slot = Some(slot);
    }

    /// Clear the slot assignment (set to None/null).
    pub fn clear_slot(&mut self) {
        self.slot = None;
    }

    /// Check if a slot has been assigned.
    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }
}

impl Default for SlotHandle {
    fn default() -> Self {
        SlotHandle::new()
    }
}

// Implement PartialEq and Eq manually to match TypeScript behavior
// (TypeScript classes use reference equality by default, but here we use value equality)
impl PartialEq<usize> for SlotHandle {
    fn eq(&self, other: &usize) -> bool {
        self.slot == Some(*other)
    }
}

impl PartialEq<SlotHandle> for usize {
    fn eq(&self, other: &SlotHandle) -> bool {
        Some(*self) == other.slot
    }
}
