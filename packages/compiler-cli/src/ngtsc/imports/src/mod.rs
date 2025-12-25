// Imports Source Module

pub mod alias;
pub mod core;
pub mod default;
pub mod deferred_symbol_tracker;
pub mod emitter;
pub mod find_export;
pub mod imported_symbols_tracker;
pub mod local_compilation_extra_imports_tracker;
pub mod reexport;
pub mod references;
pub mod resolver;

// Re-exports
pub use alias::{
    AliasStrategy, AliasingHost, PrivateExportAliasingHost, UnifiedModulesAliasingHost,
};
pub use core::{
    validate_and_rewrite_core_symbol, ImportRewriter, NoopImportRewriter, R3SymbolsImportRewriter,
};
pub use default::{
    attach_default_import_declaration, get_default_import_declaration, DefaultImportTracker,
};
pub use deferred_symbol_tracker::{DeferredSymbolTracker, SymbolState};
pub use emitter::{
    AbsoluteModuleStrategy, EmittedReference, FailedEmitResult, ImportFlags, ImportedFile,
    LocalIdentifierStrategy, LogicalProjectStrategy, ReferenceEmitKind, ReferenceEmitResult,
    ReferenceEmitStrategy, ReferenceEmitter, RelativePathStrategy,
};
pub use find_export::{find_exported_name_of_node, ExportInfo, ExportMap};
pub use imported_symbols_tracker::ImportedSymbolsTracker;
pub use local_compilation_extra_imports_tracker::{
    remove_quotations, LocalCompilationExtraImportsTracker,
};
pub use reexport::Reexport;
pub use references::{AmbientImport, OwningModule, Reference};
pub use resolver::ModuleResolver;
