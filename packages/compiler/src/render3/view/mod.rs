//! Render3 View Module
//!
//! Corresponds to packages/compiler/src/render3/view/
//! Contains view compilation utilities and APIs

pub mod api;
pub mod util;
pub mod config;
pub mod template;
pub mod t2_api;
pub mod t2_binder;
pub mod query_generation;
pub mod compiler;
pub mod i18n;

// Re-exports
pub use api::*;
pub use util::*;
pub use config::*;
pub use template::*;
pub use t2_api::*;
pub use t2_binder::*;
pub use query_generation::*;
pub use compiler::*;
pub use i18n::*;

