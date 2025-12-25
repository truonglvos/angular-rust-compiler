// Annotations NgModule Source Module

pub mod handler;
pub mod module_with_providers;
pub mod symbol;

// Re-exports
pub use handler::{
    NgModuleAnalysis, NgModuleDecoratorHandler, NgModuleResolution, R3FactoryMetadata,
    R3InjectorMetadata, R3NgModuleMetadata,
};
pub use module_with_providers::{
    is_module_with_providers_type, is_resolved_module_with_providers,
    try_resolve_module_with_providers, ModuleWithProvidersError, MwpResolverConfig,
    ResolvedModuleWithProviders,
};
pub use symbol::{NgModuleSymbol, RemotelyScopedComponent};
