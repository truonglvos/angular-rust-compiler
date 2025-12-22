// Docs Entities
//
// Represents extracted documentation entries.

use std::collections::HashMap;

/// Type of documentation entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    Class,
    Interface,
    Constant,
    Function,
    Enum,
    TypeAlias,
    Decorator,
    Directive,
    Component,
    Pipe,
    NgModule,
    Injectable,
}

/// Documentation entry for a class/interface/function/etc.
#[derive(Debug, Clone)]
pub struct DocEntry {
    /// Entry name.
    pub name: String,
    /// Entry type.
    pub entry_type: EntryType,
    /// Description from JSDoc.
    pub description: String,
    /// JSDoc tags.
    pub jsdoc_tags: Vec<JsDocTag>,
    /// Source file.
    pub source_file: String,
    /// Line number.
    pub line: usize,
    /// Is deprecated.
    pub deprecated: Option<String>,
    /// Additional metadata.
    pub metadata: HashMap<String, String>,
}

impl DocEntry {
    pub fn new(name: impl Into<String>, entry_type: EntryType) -> Self {
        Self {
            name: name.into(),
            entry_type,
            description: String::new(),
            jsdoc_tags: Vec::new(),
            source_file: String::new(),
            line: 0,
            deprecated: None,
            metadata: HashMap::new(),
        }
    }
}

/// JSDoc tag.
#[derive(Debug, Clone)]
pub struct JsDocTag {
    /// Tag name (e.g., "param", "returns", "deprecated").
    pub name: String,
    /// Tag value/text.
    pub text: String,
}

/// Class member entry.
#[derive(Debug, Clone)]
pub struct MemberEntry {
    /// Member name.
    pub name: String,
    /// Member type.
    pub member_type: MemberType,
    /// Type annotation.
    pub type_annotation: String,
    /// Description.
    pub description: String,
    /// Is inherited.
    pub inherited: bool,
    /// Visibility.
    pub visibility: Visibility,
}

/// Member type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemberType {
    Property,
    Method,
    Getter,
    Setter,
    Input,
    Output,
}

/// Member visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    #[default]
    Public,
    Protected,
    Private,
}

/// Class documentation entry.
#[derive(Debug, Clone)]
pub struct ClassEntry {
    /// Base doc entry.
    pub base: DocEntry,
    /// Members.
    pub members: Vec<MemberEntry>,
    /// Constructor parameters.
    pub constructor_params: Vec<ParameterEntry>,
    /// Extended class.
    pub extends: Option<String>,
    /// Implemented interfaces.
    pub implements: Vec<String>,
    /// Type parameters.
    pub type_params: Vec<TypeParameterEntry>,
}

/// Parameter entry.
#[derive(Debug, Clone)]
pub struct ParameterEntry {
    /// Parameter name.
    pub name: String,
    /// Type annotation.
    pub type_annotation: String,
    /// Is optional.
    pub optional: bool,
    /// Default value.
    pub default_value: Option<String>,
    /// Description.
    pub description: String,
}

/// Type parameter entry.
#[derive(Debug, Clone)]
pub struct TypeParameterEntry {
    /// Name.
    pub name: String,
    /// Constraint.
    pub constraint: Option<String>,
    /// Default.
    pub default: Option<String>,
}

/// Function documentation entry.
#[derive(Debug, Clone)]
pub struct FunctionEntry {
    /// Base doc entry.
    pub base: DocEntry,
    /// Parameters.
    pub params: Vec<ParameterEntry>,
    /// Return type.
    pub return_type: String,
    /// Type parameters.
    pub type_params: Vec<TypeParameterEntry>,
}

/// Enum documentation entry.
#[derive(Debug, Clone)]
pub struct EnumEntry {
    /// Base doc entry.
    pub base: DocEntry,
    /// Enum members.
    pub members: Vec<EnumMemberEntry>,
}

/// Enum member entry.
#[derive(Debug, Clone)]
pub struct EnumMemberEntry {
    /// Member name.
    pub name: String,
    /// Value.
    pub value: String,
    /// Description.
    pub description: String,
}
