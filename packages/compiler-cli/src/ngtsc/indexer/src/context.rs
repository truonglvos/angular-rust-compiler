// Indexing Context
//
// Context for indexing Angular declarations.

use super::api::*;
use std::collections::HashMap;

/// Indexing context.
#[derive(Debug, Default)]
pub struct IndexingContext {
    components: HashMap<String, Vec<IndexedComponent>>,
    directives: HashMap<String, Vec<IndexedDirective>>,
    pipes: HashMap<String, Vec<IndexedPipe>>,
}

impl IndexingContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_component(&mut self, file: &str, component: IndexedComponent) {
        self.components
            .entry(file.to_string())
            .or_default()
            .push(component);
    }

    pub fn add_directive(&mut self, file: &str, directive: IndexedDirective) {
        self.directives
            .entry(file.to_string())
            .or_default()
            .push(directive);
    }

    pub fn add_pipe(&mut self, file: &str, pipe: IndexedPipe) {
        self.pipes.entry(file.to_string()).or_default().push(pipe);
    }

    pub fn get_components(&self, file: &str) -> Option<&Vec<IndexedComponent>> {
        self.components.get(file)
    }

    pub fn all_components(&self) -> impl Iterator<Item = &IndexedComponent> {
        self.components.values().flat_map(|v| v.iter())
    }
}

/// Indexer.
pub struct Indexer {
    context: IndexingContext,
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            context: IndexingContext::new(),
        }
    }

    pub fn context(&self) -> &IndexingContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut IndexingContext {
        &mut self.context
    }
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new()
    }
}
