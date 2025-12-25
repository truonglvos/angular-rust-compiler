//! Render3 Module
//!
//! Corresponds to packages/compiler/src/render3/
//! Contains View Engine compilation logic

pub mod partial;
pub mod r3_ast;
pub mod r3_class_debug_info_compiler;
pub mod r3_class_metadata_compiler;
pub mod r3_control_flow;
pub mod r3_deferred_blocks;
pub mod r3_deferred_triggers;
pub mod r3_factory;
pub mod r3_hmr_compiler;
pub mod r3_identifiers;
pub mod r3_injector_compiler;
pub mod r3_jit;
pub mod r3_module_compiler;
pub mod r3_pipe_compiler;
pub mod r3_template_transform;
pub mod util;
pub mod view;

// Re-exports
pub use r3_class_debug_info_compiler::*;
pub use r3_class_metadata_compiler::*;
pub use r3_deferred_blocks::*;
pub use r3_factory::*;
pub use r3_hmr_compiler::*;
pub use r3_identifiers::Identifiers;
pub use r3_injector_compiler::*;
pub use r3_module_compiler::*;
pub use r3_pipe_compiler::*;
pub use r3_template_transform::*;
pub use util::*;
