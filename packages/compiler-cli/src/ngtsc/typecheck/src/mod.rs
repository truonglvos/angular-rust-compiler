// TypeCheck Source Module

pub mod checker;
pub mod context;
pub mod diagnostics;
pub mod type_check_block;

// Re-exports
pub use checker::TemplateTypeCheckerImpl;
pub use context::{TypeCheckEnvironment, TypeCheckingContext};
pub use diagnostics::{
    create_missing_pipe_diagnostic, create_missing_required_input_diagnostic,
    create_type_mismatch_diagnostic, create_unknown_element_diagnostic,
    create_unknown_property_diagnostic, TemplateDiagnosticCode,
};
pub use type_check_block::{OutOfBandDiagnosticRecorder, TypeCheckBlockGenerator};
