// Annotations Directive Source Module

pub mod handler;
pub mod initializer_function_access;
pub mod initializer_functions;
pub mod input_function;
pub mod input_output_parse_options;
pub mod model_function;
pub mod output_function;
pub mod query_functions;
pub mod symbol;

// Re-exports
pub use handler::{DirectiveDecoratorHandler, DirectiveHandlerData};
pub use initializer_function_access::{
    validate_access_of_initializer_api_member, AccessLevel, AccessLevelError, InitializerApiConfig,
};
pub use initializer_functions::{
    try_parse_initializer_api, InitializerApiFunction, InitializerFunctionMetadata,
    InitializerFunctionName, OwningModule,
};
pub use input_function::{
    input_initializer_config, try_parse_signal_input_mapping, SignalInputMapping,
};
pub use input_output_parse_options::{
    parse_and_validate_input_and_output_options, parse_options_from_pairs, InputOutputOptions,
    OptionsParseError,
};
pub use model_function::{try_parse_model_function, ModelFunctionMetadata};
pub use output_function::{
    output_from_observable_config, output_initializer_config, output_initializer_configs,
    try_parse_initializer_based_output, OutputMapping,
};
pub use query_functions::{query_initializer_apis, try_parse_signal_query, QueryFunctionMetadata};
pub use symbol::{
    DirectiveSymbol, DirectiveTypeCheckMeta, InputMappingMeta, InputOrOutput,
    SemanticTypeParameter, TemplateGuardMeta, TemplateGuardType,
};
