//! Render3 Partial Declaration API
//!
//! Corresponds to packages/compiler/src/render3/partial/api.ts
//! Contains API definitions for partial/linking compilation

use std::collections::HashMap;

use crate::compiler_facade_interface::FactoryTarget;
use crate::core::{ChangeDetectionStrategy, ViewEncapsulation};
use crate::output::output_ast::Expression;

/// Base interface for partial declarations
#[derive(Debug, Clone)]
pub struct R3PartialDeclaration {
    /// The minimum version of the compiler that can process this partial declaration.
    pub min_version: String,
    /// Version number of the Angular compiler.
    pub version: String,
    /// A reference to the `@angular/core` ES module.
    pub ng_import: Expression,
    /// Reference to the decorated class.
    pub type_: Expression,
}

/// Legacy input partial mapping
#[derive(Debug, Clone)]
pub enum LegacyInputPartialMapping {
    Simple(String),
    Complex {
        binding_property_name: String,
        class_property_name: String,
        transform_function: Option<Expression>,
    },
}

/// Input field metadata for partial declarations
#[derive(Debug, Clone)]
pub enum InputPartialMetadata {
    Legacy(LegacyInputPartialMapping),
    Full {
        class_property_name: String,
        public_name: String,
        is_signal: bool,
        is_required: bool,
        transform_function: Option<Expression>,
    },
}

/// Host bindings for partial declarations
#[derive(Debug, Clone, Default)]
pub struct R3DeclareHostMetadata {
    /// A mapping of attribute names to their value expression.
    pub attributes: Option<HashMap<String, Expression>>,
    /// A mapping of event names to their unparsed event handler expression.
    pub listeners: HashMap<String, String>,
    /// A mapping of bound properties to their unparsed binding expression.
    pub properties: Option<HashMap<String, String>>,
    /// The value of the class attribute.
    pub class_attribute: Option<String>,
    /// The value of the style attribute.
    pub style_attribute: Option<String>,
}

/// Query metadata for partial declarations
#[derive(Debug, Clone)]
pub struct R3DeclareQueryMetadata {
    /// Name of the property on the class to update with query results.
    pub property_name: String,
    /// Whether to read only the first matching result. Defaults to false.
    pub first: bool,
    /// The predicate for the query.
    pub predicate: QueryPredicate,
    /// Whether to include only direct children or all descendants. Defaults to false.
    pub descendants: bool,
    /// True to only fire changes if there are underlying changes to the query.
    pub emit_distinct_changes_only: bool,
    /// An expression representing a type to read from each matched node.
    pub read: Option<Expression>,
    /// Whether or not this query should collect only static results.
    pub static_: bool,
    /// Whether the query is signal-based.
    pub is_signal: bool,
}

/// Query predicate
#[derive(Debug, Clone)]
pub enum QueryPredicate {
    Expression(Expression),
    Selectors(Vec<String>),
}

/// Describes the shape of the object that the `ɵɵngDeclareDirective()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareDirectiveMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// Unparsed selector of the directive.
    pub selector: Option<String>,
    /// A mapping of inputs
    pub inputs: Option<HashMap<String, InputPartialMetadata>>,
    /// A mapping of outputs
    pub outputs: Option<HashMap<String, String>>,
    /// Information about host bindings
    pub host: Option<R3DeclareHostMetadata>,
    /// Information about content queries
    pub queries: Option<Vec<R3DeclareQueryMetadata>>,
    /// Information about view queries
    pub view_queries: Option<Vec<R3DeclareQueryMetadata>>,
    /// The list of providers
    pub providers: Option<Expression>,
    /// The names by which the directive is exported
    pub export_as: Option<Vec<String>>,
    /// Whether the directive has an inheritance clause
    pub uses_inheritance: bool,
    /// Whether the directive implements the `ngOnChanges` hook
    pub uses_on_changes: bool,
    /// Whether the directive is standalone
    pub is_standalone: bool,
    /// Whether the directive is signal-based
    pub is_signal: bool,
    /// Additional directives applied to the directive host
    pub host_directives: Option<Vec<R3DeclareHostDirectiveMetadata>>,
}

/// Describes the shape of the object that the `ɵɵngDeclareComponent()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareComponentMetadata {
    /// Base directive metadata
    pub directive: R3DeclareDirectiveMetadata,
    /// The component's unparsed template string
    pub template: Expression,
    /// Whether the template was inline
    pub is_inline: bool,
    /// CSS from inline styles and included styleUrls
    pub styles: Option<Vec<String>>,
    /// List of components which matched in the template
    pub components: Option<Vec<R3DeclareDirectiveDependencyMetadata>>,
    /// List of directives which matched in the template
    pub directives: Option<Vec<R3DeclareDirectiveDependencyMetadata>>,
    /// List of dependencies which matched in the template
    pub dependencies: Option<Vec<R3DeclareTemplateDependencyMetadata>>,
    /// List of defer block dependency functions
    pub defer_block_dependencies: Option<Vec<Expression>>,
    /// A map of pipe names to expressions
    pub pipes: Option<HashMap<String, Expression>>,
    /// The list of view providers
    pub view_providers: Option<Expression>,
    /// A collection of animation triggers
    pub animations: Option<Expression>,
    /// Strategy used for detecting changes
    pub change_detection: Option<ChangeDetectionStrategy>,
    /// An encapsulation policy for the component's styling
    pub encapsulation: Option<ViewEncapsulation>,
    /// Whether whitespace should be preserved
    pub preserve_whitespaces: bool,
}

/// Template dependency metadata types
#[derive(Debug, Clone)]
pub enum R3DeclareTemplateDependencyMetadata {
    Directive(R3DeclareDirectiveDependencyMetadata),
    Pipe(R3DeclarePipeDependencyMetadata),
    NgModule(R3DeclareNgModuleDependencyMetadata),
}

/// Directive dependency metadata
#[derive(Debug, Clone)]
pub struct R3DeclareDirectiveDependencyMetadata {
    /// Kind of dependency
    pub kind: DirectiveDependencyKind,
    /// Selector of the directive
    pub selector: String,
    /// Reference to the directive class
    pub type_: Expression,
    /// Property names of the directive's inputs
    pub inputs: Option<Vec<String>>,
    /// Event names of the directive's outputs
    pub outputs: Option<Vec<String>>,
    /// Names by which this directive exports itself
    pub export_as: Option<Vec<String>>,
}

/// Kind of directive dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectiveDependencyKind {
    Directive,
    Component,
}

/// Pipe dependency metadata
#[derive(Debug, Clone)]
pub struct R3DeclarePipeDependencyMetadata {
    /// Pipe name
    pub name: String,
    /// Reference to the pipe class
    pub type_: Expression,
}

/// NgModule dependency metadata
#[derive(Debug, Clone)]
pub struct R3DeclareNgModuleDependencyMetadata {
    /// Reference to the NgModule class
    pub type_: Expression,
}

/// Describes the shape of the object that the `ɵɵngDeclareNgModule()` accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareNgModuleMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// An array of expressions representing the bootstrap components
    pub bootstrap: Option<Vec<Expression>>,
    /// An array of expressions representing the directives and pipes declared
    pub declarations: Option<Vec<Expression>>,
    /// An array of expressions representing the imports
    pub imports: Option<Vec<Expression>>,
    /// An array of expressions representing the exports
    pub exports: Option<Vec<Expression>>,
    /// The set of schemas
    pub schemas: Option<Vec<Expression>>,
    /// Unique ID or expression representing the unique ID
    pub id: Option<Expression>,
}

/// Describes the shape of the object that the `ɵɵngDeclareInjector()` accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareInjectorMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// The list of providers
    pub providers: Option<Expression>,
    /// The list of imports
    pub imports: Option<Vec<Expression>>,
}

/// Describes the shape of the object that the `ɵɵngDeclarePipe()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclarePipeMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// The name to use in templates
    pub name: String,
    /// Whether this pipe is "pure"
    pub pure: bool,
    /// Whether the pipe is standalone
    pub is_standalone: bool,
}

/// Describes the shape of the object that the `ɵɵngDeclareFactory()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareFactoryMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// A collection of dependencies
    pub deps: FactoryDeps,
    /// Type of the target
    pub target: FactoryTarget,
}

/// Factory dependencies
#[derive(Debug, Clone)]
pub enum FactoryDeps {
    Valid(Vec<R3DeclareDependencyMetadata>),
    Invalid,
    None,
}

/// Describes the shape of the object that the `ɵɵngDeclareInjectable()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareInjectableMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// Specifies that the declared injectable belongs to a particular injector
    pub provided_in: Option<Expression>,
    /// An expression that evaluates to a class to use when creating an instance
    pub use_class: Option<Expression>,
    /// An expression that evaluates to a factory function
    pub use_factory: Option<Expression>,
    /// An expression that evaluates to a token of another injectable
    pub use_existing: Option<Expression>,
    /// An expression that evaluates to the value
    pub use_value: Option<Expression>,
    /// An array of dependencies
    pub deps: Option<Vec<R3DeclareDependencyMetadata>>,
}

/// Metadata indicating how a dependency should be injected into a factory.
#[derive(Debug, Clone)]
pub struct R3DeclareDependencyMetadata {
    /// An expression representing the token or value to be injected
    pub token: Option<Expression>,
    /// Whether the dependency is injecting an attribute value
    pub attribute: bool,
    /// Whether the dependency has an @Host qualifier
    pub host: bool,
    /// Whether the dependency has an @Optional qualifier
    pub optional: bool,
    /// Whether the dependency has an @Self qualifier
    pub self_: bool,
    /// Whether the dependency has an @SkipSelf qualifier
    pub skip_self: bool,
}

impl Default for R3DeclareDependencyMetadata {
    fn default() -> Self {
        R3DeclareDependencyMetadata {
            token: None,
            attribute: false,
            host: false,
            optional: false,
            self_: false,
            skip_self: false,
        }
    }
}

/// Describes the shape of the object that the `ɵɵngDeclareClassMetadata()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareClassMetadata {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// The Angular decorators of the class
    pub decorators: Expression,
    /// Constructor parameters
    pub ctor_parameters: Option<Expression>,
    /// Angular decorators applied to the class properties
    pub prop_decorators: Option<Expression>,
}

/// Describes the shape of the object that the `ɵɵngDeclareClassMetadataAsync()` function accepts.
#[derive(Debug, Clone)]
pub struct R3DeclareClassMetadataAsync {
    /// Base partial declaration
    pub base: R3PartialDeclaration,
    /// Function that loads the deferred dependencies
    pub resolve_deferred_deps: Expression,
    /// Function that returns the class metadata
    pub resolve_metadata: Expression,
}

/// Host directive metadata for partial declarations
#[derive(Debug, Clone)]
pub struct R3DeclareHostDirectiveMetadata {
    /// Reference to the directive
    pub directive: Expression,
    /// Inputs to expose
    pub inputs: Option<Vec<String>>,
    /// Outputs to expose
    pub outputs: Option<Vec<String>>,
}
