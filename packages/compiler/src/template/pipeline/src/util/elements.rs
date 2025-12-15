//! Element Utilities
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/util/elements.ts

use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::handle::XrefId;
use crate::template::pipeline::ir::ops::create::RepeaterCreateOp;
use crate::template::pipeline::ir::operations::CreateOp;
use crate::template::pipeline::src::compilation::CompilationUnit;
use std::collections::HashMap;

/// Check if an operation kind implements ConsumesSlotOpTrait.
///
/// This is based on the OpKinds that implement ConsumesSlotOpTrait:
/// - ElementStart, Element, ContainerStart, Container
/// - Template
/// - Text
/// - I18nStart, I18nOp, I18nAttributes
/// - RepeaterCreate, ConditionalCreate, ConditionalBranchCreate
/// - Projection
/// - DeferOp
/// - DeclareLet
/// - PipeOp
/// Check if an operation kind implements ConsumesSlotOpTrait.
///
/// This is based on the OpKinds that implement ConsumesSlotOpTrait:
/// - ElementStart, Element, ContainerStart, Container
/// - Template
/// - Text
/// - I18nStart, I18nOp, I18nAttributes
/// - RepeaterCreate, ConditionalCreate, ConditionalBranchCreate
/// - Projection
/// - DeferOp
/// - DeclareLet
/// - PipeOp
pub(crate) fn op_kind_has_consumes_slot_trait(kind: OpKind) -> bool {
    matches!(
        kind,
        OpKind::ElementStart
            | OpKind::Element
            | OpKind::ContainerStart
            | OpKind::Container
            | OpKind::Template
            | OpKind::Text
            | OpKind::I18nStart
            | OpKind::I18n
            | OpKind::I18nAttributes
            | OpKind::RepeaterCreate
            | OpKind::ConditionalCreate
            | OpKind::ConditionalBranchCreate
            | OpKind::Projection
            | OpKind::Defer
            | OpKind::DeclareLet
            | OpKind::Pipe
    )
}

/// Gets a map of all elements in the given view by their xref id.
///
/// This function iterates through all create operations in the unit and collects
/// those that implement `ConsumesSlotOpTrait` into a map keyed by their `XrefId`.
///
/// Special case: For `@for` loops with `@empty` blocks, the empty view's XrefId
/// is also added to the map, pointing to the same `RepeaterCreateOp`.
///
/// Note: This implementation stores indices into the OpList instead of the operations themselves.
/// The operations can be retrieved using the `lookup_element` function which takes the unit
/// as a parameter. This approach avoids ownership issues while still providing efficient lookup.
pub fn create_op_xref_map(
    unit: &dyn CompilationUnit,
) -> HashMap<XrefId, usize> {
    let mut map = HashMap::new();

    for (index, op) in unit.create().iter().enumerate() {
        let kind = op.kind();
        if !op_kind_has_consumes_slot_trait(kind) {
            continue;
        }

        let xref = op.xref();
        map.insert(xref, index);

        // Special handling for RepeaterCreate with empty_view
        if kind == OpKind::RepeaterCreate {
            // We need to downcast to access empty_view, but can't do it directly
            // Instead, we'll handle this in lookup_element by checking the kind
            // and downcasting there. For now, we note that this op might have
            // an empty_view that also needs to be in the map.
            // This is handled by a second pass after we know all the indices.
        }
    }

    // Second pass: handle RepeaterCreate with empty_view
    // We need to check each RepeaterCreate op to see if it has an empty_view
    // and add it to the map if present
    // Since we can't easily downcast Box<dyn CreateOp> in safe Rust, we use
    // unsafe code to access the concrete RepeaterCreateOp type
    for (index, op) in unit.create().iter().enumerate() {
        if op.kind() == OpKind::RepeaterCreate {
            // We know this is a RepeaterCreateOp, so we can safely access it
            // This is safe because:
            // 1. We've verified op.kind() == OpKind::RepeaterCreate
            // 2. We're only reading the empty_view field
            // 3. The operation is owned by unit.create()
            unsafe {
                // Cast to the concrete type pointer
                // This is safe because we know it's a RepeaterCreateOp
                let op_ptr = op as *const Box<dyn CreateOp + Send + Sync>;
                let repeater_ptr = op_ptr as *const Box<RepeaterCreateOp>;
                
                if !repeater_ptr.is_null() {
                    let repeater = &**repeater_ptr;
                    if let Some(empty_view) = repeater.empty_view {
                        // Use the same index for empty_view (points to the same RepeaterCreateOp)
                        map.insert(empty_view, index);
                    }
                }
            }
        }
    }

    map
}

/// Looks up an element in the unit by xref ID using the index map.
///
/// This is a helper function that provides the same functionality as the TypeScript version.
/// It first looks up the index in the map, then retrieves the operation from the unit.
/// 
/// Note: The returned reference is borrowed from `unit`, so it cannot outlive the unit.
pub fn lookup_element<'a>(
    unit: &'a dyn CompilationUnit,
    index_map: &HashMap<XrefId, usize>,
    xref: XrefId,
) -> &'a Box<dyn CreateOp + Send + Sync> {
    let index = index_map
        .get(&xref)
        .copied()
        .expect("All attributes should have an element-like target.");
    
    unit.create()
        .get(index)
        .expect("Operation index out of bounds")
}
