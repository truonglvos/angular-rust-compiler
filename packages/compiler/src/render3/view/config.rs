//! Render3 View Config
//!
//! Corresponds to packages/compiler/src/render3/view/config.ts
//! Contains configuration for view compilation

use std::sync::atomic::{AtomicBool, Ordering};

/// Whether to produce instructions that will attach the source location to each DOM node.
static ENABLE_TEMPLATE_SOURCE_LOCATIONS: AtomicBool = AtomicBool::new(false);

/// Utility function to enable source locations. Intended to be used **only** inside unit tests.
pub fn set_enable_template_source_locations(value: bool) {
    ENABLE_TEMPLATE_SOURCE_LOCATIONS.store(value, Ordering::SeqCst);
}

/// Gets whether template source locations are enabled.
pub fn get_template_source_locations_enabled() -> bool {
    ENABLE_TEMPLATE_SOURCE_LOCATIONS.load(Ordering::SeqCst)
}
