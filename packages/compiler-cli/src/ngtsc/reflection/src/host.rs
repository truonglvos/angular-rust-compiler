use oxc_ast::ast;
use std::fmt::Debug;

/// Metadata extracted from an instance of a decorator on another declaration.
#[derive(Debug, Clone)]
pub struct Decorator<'a> {
    /// Name by which the decorator was invoked in the user's code.
    pub name: String,

    /// Identifier which refers to the decorator in the user's code.
    pub identifier: Option<DecoratorIdentifier>,

    /// `Import` by which the decorator was brought into the module in which it was invoked.
    pub import: Option<Import<'a>>,

    /// Oxc AST reference to the decorator itself.
    pub node: &'a ast::Decorator<'a>,

    /// Arguments of the invocation of the decorator.
    pub args: Option<Vec<&'a ast::Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct DecoratorIdentifier {
    pub name: String,
    pub module_name: Option<String>,
}

/// The Oxc `Class` node.
// In TS: export type ClassDeclaration<T extends DeclarationNode = DeclarationNode> = T & { name: ts.Identifier };
// In Rust/Oxc: ast::Class already has `id: Option<BindingIdentifier>`.
// effectively we use `ast::Class` but in methods receiving it we might assume `id` is present if needed.
pub type ClassDeclaration<'a> = ast::Class<'a>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassMemberKind {
    Constructor,
    Getter,
    Setter,
    Property,
    Method,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassMemberAccessLevel {
    PublicWritable,
    PublicReadonly,
    Protected,
    Private,
    EcmaScriptPrivate,
}

#[derive(Debug, Clone)]
pub struct ClassMember<'a> {
    /// Reference to the class member node.
    pub node: Option<&'a ast::ClassElement<'a>>, // Simplified common type for class elements

    pub kind: ClassMemberKind,
    pub access_level: ClassMemberAccessLevel,

    pub type_node: Option<&'a ast::TSType<'a>>,

    pub name: String,
    pub name_node: Option<&'a ast::PropertyKey<'a>>, // Might need adjustment depending on Oxc

    pub value: Option<&'a ast::Expression<'a>>,

    // In TS this is ts.Declaration. In Oxc, method definitions are in ClassElement.
    // implementation: Option<&'a ast::Declaration<'a>>,
    pub is_static: bool,
    pub decorators: Option<Vec<Decorator<'a>>>,
}

#[derive(Debug, Clone)]
pub struct CtorParameter<'a> {
    pub name: Option<String>,
    pub name_node: &'a ast::BindingPattern<'a>, // equivalent to ts.BindingName

    pub type_value_reference: TypeValueReference<'a>,
    pub type_node: Option<&'a ast::TSType<'a>>,

    pub decorators: Option<Vec<Decorator<'a>>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition<'a> {
    pub node: &'a ast::Function<'a>,
    pub body: Option<&'a ast::FunctionBody<'a>>,
    pub parameters: Vec<Parameter<'a>>,
    pub type_parameters: Option<&'a ast::TSTypeParameterDeclaration<'a>>,
    pub signature_count: usize,
}

#[derive(Debug, Clone)]
pub struct Parameter<'a> {
    pub name: Option<String>,
    pub node: &'a ast::FormalParameter<'a>,
    pub initializer: Option<&'a ast::Expression<'a>>,
    pub type_node: Option<&'a ast::TSType<'a>>,
}

#[derive(Debug, Clone)]
pub struct Import<'a> {
    pub name: String,
    pub from: String,
    pub node: &'a ast::ImportDeclaration<'a>,
}

#[derive(Debug, Clone)]
pub struct Declaration<'a> {
    /// Absolute module path if imported.
    pub via_module: Option<String>,
    pub node: &'a ast::Declaration<'a>,
}

// NOTE: TypeValueReference and related types are complex to port 1:1 without TypeChecker.
// We will define them but usage might be limited in Oxc-only host.

#[derive(Debug, Clone)]
pub enum TypeValueReferenceKind {
    Local,
    Imported,
    Unavailable,
}

#[derive(Debug, Clone)]
pub enum TypeValueReference<'a> {
    Local(LocalTypeValueReference<'a>),
    Imported(ImportedTypeValueReference<'a>),
    Unavailable(UnavailableTypeValueReference<'a>),
}

#[derive(Debug, Clone)]
pub struct LocalTypeValueReference<'a> {
    pub kind: TypeValueReferenceKind,
    pub expression: &'a ast::Expression<'a>,
    pub default_import_statement: Option<&'a ast::ImportDeclaration<'a>>,
}

#[derive(Debug, Clone)]
pub struct ImportedTypeValueReference<'a> {
    pub kind: TypeValueReferenceKind,
    pub module_name: String,
    pub imported_name: String,
    pub nested_path: Option<Vec<String>>,
    pub value_declaration: Option<&'a ast::Declaration<'a>>,
}

#[derive(Debug, Clone)]
pub struct UnavailableTypeValueReference<'a> {
    pub kind: TypeValueReferenceKind,
    pub reason: UnavailableValue<'a>,
}

#[derive(Debug, Clone)]
pub enum ValueUnavailableKind {
    MissingType,
    NoValueDeclaration,
    TypeOnlyImport,
    UnknownReference,
    Namespace,
    Unsupported,
}

#[derive(Debug, Clone)]
pub enum UnavailableValue<'a> {
    Unsupported {
        type_node: &'a ast::TSType<'a>,
    },
    NoValueDeclaration {
        type_node: &'a ast::TSType<'a>,
        decl: Option<&'a ast::Declaration<'a>>,
    },
    TypeOnlyImport {
        type_node: &'a ast::TSType<'a>,
        node: &'a ast::ImportDeclaration<'a>,
    }, // simplified node type
    Namespace {
        type_node: &'a ast::TSType<'a>,
        import_clause: &'a ast::ImportDeclaration<'a>,
    },
    UnknownReference {
        type_node: &'a ast::TSType<'a>,
    },
    MissingType,
}

/// Abstracts reflection operations on the AST.
pub trait ReflectionHost<'a> {
    fn get_decorators_of_declaration(
        &self,
        declaration: &'a ast::Declaration<'a>,
    ) -> Vec<Decorator<'a>>;

    fn get_members_of_class(&self, clazz: &'a ClassDeclaration<'a>) -> Vec<ClassMember<'a>>;

    fn get_constructor_parameters(
        &self,
        clazz: &'a ClassDeclaration<'a>,
    ) -> Option<Vec<CtorParameter<'a>>>;

    // Using generic Node equivalent? In Oxc we might need specific types.
    // For now we assume Function node.
    fn get_definition_of_function(
        &self,
        fn_node: &'a ast::Function<'a>,
    ) -> Option<FunctionDefinition<'a>>;

    fn get_import_of_identifier(&self, id: &'a ast::IdentifierReference<'a>) -> Option<Import<'a>>;

    fn get_declaration_of_identifier(
        &self,
        id: &'a ast::IdentifierReference<'a>,
    ) -> Option<Declaration<'a>>;

    fn get_exports_of_module(
        &self,
        module: &'a ast::Program<'a>,
    ) -> Option<std::collections::HashMap<String, Declaration<'a>>>;

    fn is_class(&self, node: &'a ast::Declaration<'a>) -> bool;

    fn has_base_class(&self, clazz: &'a ClassDeclaration<'a>) -> bool;

    fn get_base_class_expression(
        &self,
        clazz: &'a ClassDeclaration<'a>,
    ) -> Option<&'a ast::Expression<'a>>;

    fn get_generic_arity_of_class(&self, clazz: &'a ClassDeclaration<'a>) -> Option<usize>;

    fn get_variable_value(
        &self,
        declaration: &'a ast::VariableDeclarator<'a>,
    ) -> Option<&'a ast::Expression<'a>>;

    fn is_statically_exported(&self, decl: &'a ast::Declaration<'a>) -> bool;
}
