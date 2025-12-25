// Annotations Common Source Module

pub mod api;
pub mod debug_info;
pub mod di;
pub mod diagnostics;
pub mod evaluation;
pub mod factory;
pub mod injectable_registry;
pub mod input_transforms;
pub mod jit_declaration_registry;
pub mod metadata;
pub mod references_registry;
pub mod schema;
pub mod util;

// Re-exports
pub use api::{NoopResourceLoader, ResourceLoader, ResourceLoaderContext, ResourceType};
pub use debug_info::{extract_class_debug_info, R3ClassDebugInfo};
pub use di::{
    get_constructor_dependencies, get_valid_constructor_dependencies,
    unwrap_constructor_dependencies, ConstructorDepError, ConstructorDeps, CtorParameter,
    ParameterDecorator, R3DependencyMetadata, R3ResolvedDependencyType, UnavailableValueKind,
};
pub use diagnostics::{
    create_value_has_wrong_type_error, get_provider_diagnostics,
    get_undecorated_class_with_angular_features_diagnostic, make_diagnostic_chain,
    make_duplicate_declaration_error, Diagnostic, ErrorCode, FatalDiagnosticError, RelatedInfo,
};
pub use evaluation::{
    resolve_encapsulation_enum_value_locally, resolve_enum_value, EnumValue, ResolvedValue,
    ViewEncapsulation,
};
pub use factory::{
    compile_declare_factory, compile_ng_factory_def_field, CompileResult, FactoryTarget,
    R3FactoryMetadata,
};
pub use injectable_registry::{InjectableClassRegistry, InjectableMeta};
pub use input_transforms::{compile_input_transform_fields, InputMapping, InputTransform};
pub use jit_declaration_registry::JitDeclarationRegistry;
pub use metadata::{
    ctor_parameter_to_metadata, decorator_to_metadata, extract_class_metadata,
    CtorParameterMetadata, DecoratorMetadata, PropDecoratorMetadata, R3ClassMetadata,
};
pub use references_registry::{
    CollectingReferencesRegistry, NoopReferencesRegistry, ReferencesRegistry,
};
pub use schema::{
    extract_schemas, has_custom_elements_schema, has_no_errors_schema, SchemaError, SchemaMetadata,
};
pub use util::{
    expand_forward_ref, find_angular_decorator, get_angular_decorators, is_angular_core,
    is_angular_decorator, to_r3_reference, unwrap_expression, wrap_type_reference, Decorator,
    Import, R3Reference, CORE_MODULE,
};
