//! Compiler Main Module
//!
//! Corresponds to packages/compiler/src/compiler.ts
//! Main compiler exports and re-exports

// Configuration
pub use crate::config::CompilerConfig;

// Utilities
pub use crate::util::Version;
pub use crate::parse_util::{ParseSourceSpan, ParseLocation, ParseSourceFile, ParseError, ParseErrorLevel};

// Expression Parser
pub use crate::expression_parser::{
    Lexer,
    Parser as ExpressionParser,
    ast as expression_ast,
};

// Template Parser
pub use crate::render3::view::template::{
    parse_template,
    make_binding_parser,
    ParseTemplateOptions,
    ParsedTemplate,
};

// ML Parser (HTML AST)
pub use crate::ml_parser::ast as html_ast;

// Compiler Facade
pub use crate::compiler_facade_interface::*;
pub use crate::jit_compiler_facade::{
    CompilerFacadeImpl,
};

// Output AST
pub use crate::output::{
    output_ast,
    output_jit,
};

// Injection
pub use crate::injectable_compiler_2::{
    compile_injectable,
    R3InjectableMetadata,
};

// Shadow CSS
pub use crate::shadow_css::ShadowCss;

// Schema
pub use crate::schema::{
    ElementSchemaRegistry,
    DomElementSchemaRegistry,
};

// Render3 compilation (Core)
pub use crate::render3::view::compiler::{
    compile_component_from_metadata,
    compile_directive_from_metadata,
    ParsedHostBindings,
    verify_host_bindings,
    parse_host_bindings,
};
pub use crate::render3::r3_module_compiler::{
    compile_ng_module,
    R3NgModuleMetadata,
};
pub use crate::render3::r3_pipe_compiler::{
    compile_pipe_from_metadata,
    R3PipeMetadata,
};
pub use crate::render3::r3_injector_compiler::{
    compile_injector,
    R3InjectorMetadata,
};

// Constants
pub use crate::constant_pool::ConstantPool;
