//! IR Variables
//!
//! Corresponds to packages/compiler/src/template/pipeline/ir/src/variable.ts
//! Defines variable types used in IR

use crate::output::output_ast::Expression;
use crate::template::pipeline::ir::handle::XrefId;

/// Distinguishes between different kinds of `SemanticVariable`s.
pub use crate::template::pipeline::ir::enums::SemanticVariableKind;

/// Base trait for semantic variables
pub trait SemanticVariableTrait {
    /// Get the kind of this semantic variable
    fn kind(&self) -> SemanticVariableKind;
    /// Get the name assigned to this variable in generated code, or `None` if not yet assigned
    fn name(&self) -> Option<&str>;
}

/// A variable that represents the context of a particular view.
#[derive(Debug, Clone)]
pub struct ContextVariable {
    /// `XrefId` of the view that this variable represents.
    pub view: XrefId,
    /// Name assigned to this variable in generated code, or `None` if not yet assigned.
    pub name: Option<String>,
}

impl ContextVariable {
    pub fn new(view: XrefId) -> Self {
        ContextVariable { view, name: None }
    }
}

impl SemanticVariableTrait for ContextVariable {
    fn kind(&self) -> SemanticVariableKind {
        SemanticVariableKind::Context
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A variable that represents a specific identifier within a template.
#[derive(Debug, Clone)]
pub struct IdentifierVariable {
    /// The identifier whose value in the template is tracked in this variable.
    pub identifier: String,
    /// Whether the variable was declared locally within the same view or somewhere else.
    pub local: bool,
    /// Name assigned to this variable in generated code, or `None` if not yet assigned.
    pub name: Option<String>,
}

impl IdentifierVariable {
    pub fn new(identifier: String, local: bool) -> Self {
        IdentifierVariable {
            identifier,
            local,
            name: None,
        }
    }
}

impl SemanticVariableTrait for IdentifierVariable {
    fn kind(&self) -> SemanticVariableKind {
        SemanticVariableKind::Identifier
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A variable that represents a saved view context.
#[derive(Debug, Clone)]
pub struct SavedViewVariable {
    /// The view context saved in this variable.
    pub view: XrefId,
    /// Name assigned to this variable in generated code, or `None` if not yet assigned.
    pub name: Option<String>,
}

impl SavedViewVariable {
    pub fn new(view: XrefId) -> Self {
        SavedViewVariable { view, name: None }
    }
}

impl SemanticVariableTrait for SavedViewVariable {
    fn kind(&self) -> SemanticVariableKind {
        SemanticVariableKind::SavedView
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// A variable that will be inlined at every location it is used.
/// An alias is also allowed to depend on the value of a semantic variable.
#[derive(Debug, Clone)]
pub struct AliasVariable {
    /// The identifier for this alias variable.
    pub identifier: String,
    /// Expression representing the value of the alias.
    pub expression: Expression,
    /// Name assigned to this variable in generated code, or `None` if not yet assigned.
    pub name: Option<String>,
}

impl AliasVariable {
    pub fn new(identifier: String, expression: Expression) -> Self {
        AliasVariable {
            identifier,
            expression,
            name: None,
        }
    }
}

impl SemanticVariableTrait for AliasVariable {
    fn kind(&self) -> SemanticVariableKind {
        SemanticVariableKind::Alias
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

/// Union type for the different kinds of variables.
#[derive(Debug, Clone)]
pub enum SemanticVariable {
    Context(ContextVariable),
    Identifier(IdentifierVariable),
    SavedView(SavedViewVariable),
    Alias(AliasVariable),
}

impl SemanticVariable {
    /// Get the kind of this semantic variable
    pub fn kind(&self) -> SemanticVariableKind {
        match self {
            SemanticVariable::Context(_) => SemanticVariableKind::Context,
            SemanticVariable::Identifier(_) => SemanticVariableKind::Identifier,
            SemanticVariable::SavedView(_) => SemanticVariableKind::SavedView,
            SemanticVariable::Alias(_) => SemanticVariableKind::Alias,
        }
    }

    /// Get the name of this semantic variable, if assigned
    pub fn name(&self) -> Option<&str> {
        match self {
            SemanticVariable::Context(v) => v.name.as_deref(),
            SemanticVariable::Identifier(v) => v.name.as_deref(),
            SemanticVariable::SavedView(v) => v.name.as_deref(),
            SemanticVariable::Alias(v) => v.name.as_deref(),
        }
    }

    /// Set the name of this semantic variable
    pub fn set_name(&mut self, name: Option<String>) {
        match self {
            SemanticVariable::Context(v) => v.name = name,
            SemanticVariable::Identifier(v) => v.name = name,
            SemanticVariable::SavedView(v) => v.name = name,
            SemanticVariable::Alias(v) => v.name = name,
        }
    }
}

impl From<ContextVariable> for SemanticVariable {
    fn from(v: ContextVariable) -> Self {
        SemanticVariable::Context(v)
    }
}

impl From<IdentifierVariable> for SemanticVariable {
    fn from(v: IdentifierVariable) -> Self {
        SemanticVariable::Identifier(v)
    }
}

impl From<SavedViewVariable> for SemanticVariable {
    fn from(v: SavedViewVariable) -> Self {
        SemanticVariable::SavedView(v)
    }
}

impl From<AliasVariable> for SemanticVariable {
    fn from(v: AliasVariable) -> Self {
        SemanticVariable::Alias(v)
    }
}

/// Marker constant for context reference
///
/// When referenced in the template's context parameters, this indicates a reference to the entire
/// context object, rather than a specific parameter.
pub const CTX_REF: &str = "CTX_REF_MARKER";
