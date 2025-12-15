//! Render3 View i18n Module
//!
//! Corresponds to packages/compiler/src/render3/view/i18n/
//! Contains internationalization utilities for view compilation

pub mod util;
pub mod icu_serializer;
pub mod localize_utils;
pub mod meta;
pub mod get_msg_utils;

// Re-exports
pub use util::*;
pub use icu_serializer::*;
pub use localize_utils::*;
pub use meta::*;
pub use get_msg_utils::*;

