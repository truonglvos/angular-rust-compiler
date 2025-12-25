// TypeCheck API Module

pub mod api;
pub mod checker;
pub mod symbols;

// Re-exports
pub use api::{
    ControlFlowPrevention, PendingTypeCheckBlock, TcbLocation, TypeCheckBlockMetadata,
    TypeCheckContext, TypeCheckError, TypeCheckOp, TypeCheckingConfig,
};
pub use checker::{TemplateTypeChecker, TypeCheckResult};
pub use symbols::{
    DirectiveSymbolInfo, ElementSymbolInfo, ExpressionSymbolInfo, InputBinding, OutputBinding,
    PipeSymbolInfo, ReferenceSymbolInfo, TemplateSymbol, VariableKind, VariableSymbolInfo,
};
