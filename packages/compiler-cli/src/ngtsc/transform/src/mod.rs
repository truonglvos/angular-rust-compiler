// Transform module - Core compilation infrastructure
//
// This module provides the transform pipeline for Angular compilation,
// including decorator handling, trait management, and code generation.

pub mod alias;
pub mod api;
pub mod compilation;
pub mod declaration;
pub mod trait_;
pub mod transform;

// Re-export commonly used types
pub use alias::{AliasTransformConfig, ExportAlias};
pub use api::{
    AnalysisOutput, CompilationMode, CompileResult, ConstantPool, DecoratorHandler, DetectResult,
    HandlerPrecedence, ResolveResult,
};
pub use compilation::{ClassRecord, TraitCompiler};
pub use declaration::{DtsTransformRegistry, IvyDeclarationDtsTransform, IvyDeclarationField};
pub use trait_::{Trait, TraitFactory, TraitState};
pub use transform::{IvyCompilationVisitor, IvyTransformConfig, IvyTransformationVisitor};
