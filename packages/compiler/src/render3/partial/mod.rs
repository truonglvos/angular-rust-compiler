//! Render3 Partial Compilation Module
//!
//! Corresponds to packages/compiler/src/render3/partial/
//! Contains partial/linking compilation APIs

pub mod api;
pub mod class_metadata;
pub mod component;
pub mod directive;
pub mod factory;
pub mod injectable;
pub mod injector;
pub mod ng_module;
pub mod pipe;
pub mod util;

// Re-exports
pub use api::*;
pub use class_metadata::*;
pub use component::*;
pub use directive::*;
pub use factory::*;
pub use injectable::*;
pub use injector::*;
pub use ng_module::*;
pub use pipe::*;
pub use util::*;
