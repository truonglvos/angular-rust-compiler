//! Attribute Utilities
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/util/attributes.ts

const ARIA_PREFIX: &str = "aria-";

/// Returns whether `name` is an ARIA attribute name.
///
/// This is a heuristic based on whether name begins with and is longer than `aria-`.
pub fn is_aria_attribute(name: &str) -> bool {
    name.starts_with(ARIA_PREFIX) && name.len() > ARIA_PREFIX.len()
}
