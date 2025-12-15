//! Render3 JIT
//!
//! Corresponds to packages/compiler/src/render3/r3_jit.ts
//! Contains JIT reflection utilities

use std::collections::HashMap;

use crate::output::output_ast::ExternalReference;

/// Trait for resolving external references at runtime
pub trait ExternalReferenceResolver {
    fn resolve_external_reference(&self, reference: &ExternalReference) -> Option<Box<dyn std::any::Any>>;
}

/// Implementation of `ExternalReferenceResolver` which resolves references to @angular/core
/// symbols at runtime, according to a consumer-provided mapping.
///
/// Only supports `resolve_external_reference`, all other methods throw.
pub struct R3JitReflector {
    context: HashMap<String, Box<dyn std::any::Any>>,
}

impl R3JitReflector {
    pub fn new(context: HashMap<String, Box<dyn std::any::Any>>) -> Self {
        R3JitReflector { context }
    }
}

impl ExternalReferenceResolver for R3JitReflector {
    fn resolve_external_reference(&self, reference: &ExternalReference) -> Option<Box<dyn std::any::Any>> {
        // This reflector only handles @angular/core imports
        if reference.module_name.as_ref().map_or(true, |m| m != "@angular/core") {
            panic!(
                "Cannot resolve external reference to {:?}, only references to @angular/core are supported.",
                reference.module_name
            );
        }

        let name = reference.name.as_ref()?;
        
        if !self.context.contains_key(name) {
            panic!("No value provided for @angular/core symbol '{}'.", name);
        }

        // Note: In Rust, we can't easily clone Box<dyn Any>, so this is a simplified implementation
        // In practice, this would need more careful handling based on the actual use case
        None
    }
}

