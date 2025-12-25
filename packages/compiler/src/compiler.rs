//! Compiler Main Module
//!
//! Corresponds to packages/compiler/src/compiler.ts
//! Main compiler exports and re-exports

// Configuration
pub use crate::config::CompilerConfig;

// Utilities
pub use crate::parse_util::{
    ParseError, ParseErrorLevel, ParseLocation, ParseSourceFile, ParseSourceSpan,
};
pub use crate::util::Version;

// Expression Parser
pub use crate::expression_parser::{ast as expression_ast, Lexer, Parser as ExpressionParser};

// Template Parser
pub use crate::render3::view::template::{
    make_binding_parser, parse_template, ParseTemplateOptions, ParsedTemplate,
};

// ML Parser (HTML AST)
pub use crate::ml_parser::ast as html_ast;

// Compiler Facade
pub use crate::compiler_facade_interface::*;
pub use crate::jit_compiler_facade::CompilerFacadeImpl;

// Output AST
pub use crate::output::{output_ast, output_jit};

// Injection
pub use crate::injectable_compiler_2::{compile_injectable, R3InjectableMetadata};

// Shadow CSS
pub use crate::shadow_css::ShadowCss;

// Schema
pub use crate::schema::{DomElementSchemaRegistry, ElementSchemaRegistry};

// Render3 compilation (Core)
pub use crate::render3::r3_injector_compiler::{compile_injector, R3InjectorMetadata};
pub use crate::render3::r3_module_compiler::{compile_ng_module, R3NgModuleMetadata};
pub use crate::render3::r3_pipe_compiler::{compile_pipe_from_metadata, R3PipeMetadata};
pub use crate::render3::view::compiler::{
    compile_component_from_metadata, compile_directive_from_metadata, parse_host_bindings,
    verify_host_bindings, ParsedHostBindings,
};

// Constants
pub use crate::constant_pool::ConstantPool;
