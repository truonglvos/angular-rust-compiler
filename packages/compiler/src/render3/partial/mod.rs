//! Render3 Partial Compilation Module
//!
//! Corresponds to packages/compiler/src/render3/partial/
//! Contains partial/linking compilation APIs

pub mod api;
pub mod util;
pub mod directive;
pub mod component;
pub mod pipe;
pub mod factory;
pub mod injectable;
pub mod injector;
pub mod ng_module;
pub mod class_metadata;

// Re-exports
pub use api::*;
pub use util::*;
pub use directive::*;
pub use component::*;
pub use pipe::*;
pub use factory::*;
pub use injectable::*;
pub use injector::*;
pub use ng_module::*;
pub use class_metadata::*;

