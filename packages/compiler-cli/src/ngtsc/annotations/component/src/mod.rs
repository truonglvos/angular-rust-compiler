// Annotations Component Source Module

pub mod handler;
pub mod metadata;
pub mod resources;
pub mod symbol;

// Re-exports
pub use handler::ComponentDecoratorHandler;
pub use metadata::{
    ChangeDetectionStrategy, ComponentHostBindings, ComponentInput, ComponentOutput,
    ComponentTemplateInfo, DeferTrigger, DeferredBlock, R3ComponentMetadata, ViewEncapsulation,
};
pub use resources::{
    extract_template, parse_template_declaration, ExtractTemplateOptions, ParsedComponentTemplate,
    ParsedTemplateWithSource, ResourceTypeForDiagnostics, SourceMapping, StyleUrlMeta,
    TemplateDeclaration,
};
pub use symbol::{ComponentSymbol, SemanticReference};
