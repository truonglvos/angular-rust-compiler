// Scope Source Module

pub mod api;
pub mod component_scope;
pub mod dependency;
pub mod local;
pub mod standalone;
pub mod typecheck;
pub mod util;

// Re-exports
pub use api::{
    CompilationScope, DirectiveExport, DirectiveInScope, ExportScope, PipeExport, PipeInScope,
    RegisterResult,
};
pub use component_scope::ComponentScopeReader;
pub use dependency::{DependencyScopeReader, ExternalDirectiveMetadata, ExternalPipeMetadata};
pub use local::LocalModuleScopeRegistry;
pub use standalone::{RemoteScope, StandaloneComponentScopeReader, StandaloneImport};
pub use typecheck::{
    TypeCheckDirective, TypeCheckInput, TypeCheckOutput, TypeCheckPipe, TypeCheckScope,
    TypeCheckScopeRegistry,
};
pub use util::{parse_selector, selector_matches_element, ReferenceKind, SelectorPart};
