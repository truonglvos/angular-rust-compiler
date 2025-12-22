// Template Indexing
//
// Indexes template elements and bindings.


/// Template element index.
#[derive(Debug, Clone)]
pub struct TemplateIndex {
    /// Elements in template.
    pub elements: Vec<IndexedElement>,
    /// Bindings in template.
    pub bindings: Vec<IndexedBinding>,
    /// References in template.
    pub references: Vec<IndexedReference>,
}

/// Indexed element.
#[derive(Debug, Clone)]
pub struct IndexedElement {
    pub tag_name: String,
    pub start: usize,
    pub end: usize,
}

/// Indexed binding.
#[derive(Debug, Clone)]
pub struct IndexedBinding {
    pub kind: BindingKind,
    pub name: String,
    pub expression: String,
    pub start: usize,
    pub end: usize,
}

/// Binding kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingKind {
    Property,
    Attribute,
    Event,
    TwoWay,
}

/// Indexed reference.
#[derive(Debug, Clone)]
pub struct IndexedReference {
    pub name: String,
    pub target: Option<String>,
    pub start: usize,
    pub end: usize,
}

/// Index a template.
pub fn index_template(_template: &str) -> TemplateIndex {
    TemplateIndex {
        elements: Vec::new(),
        bindings: Vec::new(),
        references: Vec::new(),
    }
}
