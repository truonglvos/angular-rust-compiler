//! Render3 T2 API
//!
//! Corresponds to packages/compiler/src/render3/view/t2_api.ts
//! Contains type definitions for t2 binder/analysis APIs

use std::collections::HashSet;

use crate::expression_parser::ast::AST;
use crate::render3::r3_ast::{
    Content, DeferredBlock, DeferredBlockError,
    DeferredBlockLoading, DeferredBlockPlaceholder, DeferredTrigger, Element,
    ForLoopBlock, ForLoopBlockEmpty, IfBlockBranch, LetDeclaration, R3Node,
    Reference, SwitchBlockCase, Template, Variable,
    Component, Directive, HostElement,
};

/// Node that has a `Scope` associated with it.
#[derive(Debug, Clone)]
pub enum ScopedNode {
    Template(Template),
    SwitchBlockCase(SwitchBlockCase),
    IfBlockBranch(IfBlockBranch),
    ForLoopBlock(ForLoopBlock),
    ForLoopBlockEmpty(ForLoopBlockEmpty),
    DeferredBlock(DeferredBlock),
    DeferredBlockError(DeferredBlockError),
    DeferredBlockLoading(DeferredBlockLoading),
    DeferredBlockPlaceholder(DeferredBlockPlaceholder),
    Content(Content),
    HostElement(HostElement),
}

/// Possible values that a reference can be resolved to.
#[derive(Debug, Clone)]
pub enum ReferenceTarget<DirectiveT> {
    DirectiveOnNode {
        directive: DirectiveT,
        node: DirectiveOwner,
    },
    Element(Element),
    Template(Template),
}

/// Entity that is local to the template and defined within the template.
#[derive(Debug, Clone)]
pub enum TemplateEntity {
    Reference(Reference),
    Variable(Variable),
    LetDeclaration(LetDeclaration),
}

impl PartialEq for TemplateEntity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TemplateEntity::Reference(a), TemplateEntity::Reference(b)) => a.name == b.name && a.value == b.value,
            (TemplateEntity::Variable(a), TemplateEntity::Variable(b)) => a.name == b.name && a.value == b.value,
            (TemplateEntity::LetDeclaration(a), TemplateEntity::LetDeclaration(b)) => a.name == b.name,
            _ => false,
        }
    }
}

impl Eq for TemplateEntity {}

impl std::hash::Hash for TemplateEntity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            TemplateEntity::Reference(r) => {
                state.write_u8(0);
                r.name.hash(state);
                r.value.hash(state);
            }
            TemplateEntity::Variable(v) => {
                state.write_u8(1);
                v.name.hash(state);
                v.value.hash(state);
            }
            TemplateEntity::LetDeclaration(l) => {
                state.write_u8(2);
                l.name.hash(state);
            }
        }
    }
}

/// Nodes that can have directives applied to them.
#[derive(Debug, Clone)]
pub enum DirectiveOwner {
    Element(Element),
    Template(Template),
    Component(Component),
    Directive(Directive),
    HostElement(HostElement),
}

/// A logical target for analysis, which could contain a template or other types of bindings.
#[derive(Debug, Clone)]
pub struct Target<DirectiveT> {
    pub template: Option<Vec<R3Node>>,
    pub host: Option<HostBinding<DirectiveT>>,
}

#[derive(Debug, Clone)]
pub struct HostBinding<DirectiveT> {
    pub node: HostElement,
    pub directives: Vec<DirectiveT>,
}

/// A data structure which can indicate whether a given property name is present or not.
pub trait InputOutputPropertySet {
    fn has_binding_property_name(&self, property_name: &str) -> bool;
}

/// Animation trigger names data structure.
#[derive(Debug, Clone, Default)]
pub struct LegacyAnimationTriggerNames {
    pub includes_dynamic_animations: bool,
    pub static_trigger_names: Vec<String>,
}

/// Metadata regarding a directive that's needed to match it against template elements.
pub trait DirectiveMeta {
    /// Name of the directive class (used for debugging).
    fn name(&self) -> &str;
    /// The selector for the directive or `None` if there isn't one.
    fn selector(&self) -> Option<&str>;
    /// Whether the directive is a component.
    fn is_component(&self) -> bool;
    /// Set of inputs which this directive claims.
    fn inputs(&self) -> &dyn InputOutputPropertySet;
    /// Set of outputs which this directive claims.
    fn outputs(&self) -> &dyn InputOutputPropertySet;
    /// Name under which the directive is exported, if any.
    fn export_as(&self) -> Option<&[String]>;
    /// Whether the directive is a structural directive.
    fn is_structural(&self) -> bool;
    /// If the directive is a component, includes the selectors of its `ng-content` elements.
    fn ng_content_selectors(&self) -> Option<&[String]>;
    /// Whether the template of the component preserves whitespaces.
    fn preserve_whitespaces(&self) -> bool;
    /// Animation trigger names.
    fn animation_trigger_names(&self) -> Option<&LegacyAnimationTriggerNames>;
}

/// Interface to the binding API.
pub trait TargetBinder<D: DirectiveMeta> {
    fn bind(&self, target: Target<D>) -> Box<dyn BoundTarget<D>>;
}

/// Result of performing the binding operation against a `Target`.
pub trait BoundTarget<DirectiveT: DirectiveMeta> {
    /// Get the original `Target` that was bound.
    fn target(&self) -> &Target<DirectiveT>;

    /// For a given template node, get the set of directives which matched the node.
    fn get_directives_of_node(&self, node: &DirectiveOwner) -> Option<Vec<DirectiveT>>;

    /// For a given `Reference`, get the reference's target.
    fn get_reference_target(&self, reference: &Reference) -> Option<ReferenceTarget<DirectiveT>>;

    /// For a given binding, get the entity to which the binding is being made.
    fn get_consumer_of_binding(&self, binding: &dyn std::any::Any) -> Option<ConsumerOfBinding<DirectiveT>>;

    /// If the given `AST` expression refers to a `Reference` or `Variable`, return that.
    fn get_expression_target(&self, expr: &AST) -> Option<TemplateEntity>;

    /// Get the `ScopedNode` which created a symbol.
    fn get_definition_node_of_symbol(&self, symbol: &TemplateEntity) -> Option<ScopedNode>;

    /// Get the nesting level of a particular `ScopedNode`.
    fn get_nesting_level(&self, node: &ScopedNode) -> usize;

    /// Get all `Reference`s and `Variables` visible within the given `ScopedNode`.
    fn get_entities_in_scope(&self, node: Option<&ScopedNode>) -> HashSet<TemplateEntity>;

    /// Get a list of all the directives used by the target.
    fn get_used_directives(&self) -> Vec<DirectiveT>;

    /// Get a list of eagerly used directives from the target.
    fn get_eagerly_used_directives(&self) -> Vec<DirectiveT>;

    /// Get a list of all the pipes used by the target.
    fn get_used_pipes(&self) -> Vec<String>;

    /// Get a list of eagerly used pipes from the target.
    fn get_eagerly_used_pipes(&self) -> Vec<String>;

    /// Get a list of all `@defer` blocks used by the target.
    fn get_defer_blocks(&self) -> Vec<DeferredBlock>;

    /// Gets the element that a specific deferred block trigger is targeting.
    fn get_deferred_trigger_target(&self, block: &DeferredBlock, trigger: &DeferredTrigger) -> Option<Element>;

    /// Whether a given node is located in a `@defer` block.
    fn is_deferred(&self, node: &Element) -> bool;

    /// Checks whether a referenced directive exists.
    fn referenced_directive_exists(&self, name: &str) -> bool;
}

/// Consumer of binding - either a directive or element/template.
#[derive(Debug, Clone)]
pub enum ConsumerOfBinding<DirectiveT> {
    Directive(DirectiveT),
    Element(Element),
    Template(Template),
}

