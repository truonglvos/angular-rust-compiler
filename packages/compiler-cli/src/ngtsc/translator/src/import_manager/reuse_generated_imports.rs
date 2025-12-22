use std::collections::HashMap;
use crate::ngtsc::translator::src::api::import_generator::ImportRequest;

// We use generic TFile and TExpression.
// Direct reuse cache stores Expression.
// Namespace import reuse cache stores Expression (Identifier).

pub struct ReuseGeneratedImportsTracker<TExpression> {
    pub direct_reuse_cache: HashMap<String, TExpression>,
    pub namespace_import_reuse_cache: HashMap<String, TExpression>, // Map<ModuleName, Identifier>
}

impl<TExpression> ReuseGeneratedImportsTracker<TExpression> {
    pub fn new() -> Self {
        Self {
            direct_reuse_cache: HashMap::new(),
            namespace_import_reuse_cache: HashMap::new(),
        }
    }
}

pub fn attempt_to_reuse_generated_imports<TFile: crate::ngtsc::translator::src::import_manager::check_unique_identifier_name::IdentifierScope, TExpression: Clone>(
    tracker: &ReuseGeneratedImportsTracker<TExpression>,
    request: &ImportRequest<TFile>,
) -> Option<TExpression> { // Returns TExpression or None
    let request_hash = hash_import_request(request);

    if let Some(existing) = tracker.direct_reuse_cache.get(&request_hash) {
        return Some(existing.clone());
    }

    let potential_namespace_import = tracker.namespace_import_reuse_cache.get(&request.export_module_specifier);
    if potential_namespace_import.is_none() {
        return None;
    }
    let potential_namespace_import = potential_namespace_import.unwrap();

    if request.export_symbol_name.is_none() {
        return Some(potential_namespace_import.clone());
    }

    // If it's a named import but we have a namespace import available, we need to return `Namespace.Symbol`.
    // But `attemptToReuseGeneratedImports` in TS returns `Identifier | [Identifier, Identifier]`.
    // It seems `TExpression` in Rust should support representing property access or the caller handles it.
    // In TS: return [potentialNamespaceImport, ts.factory.createIdentifier(request.exportSymbolName)];
    // But here we need to return TExpression.
    
    // The structure suggests TExpression might be flexible or we need a specific return type.
    // TS signature: `ts.Identifier | [ts.Identifier, ts.Identifier] | null`.
    // The caller `addImport` handles this.
    
    // Since we are generic over TExpression, we can't easily construct PropertyAccess here without AstFactory.
    // But TS implementation does `[ns, id]`.
    // Maybe we should return `Result<TExpression, (TExpression, String)>`? 
    // Or just `Option<ReuseResult<TExpression>>`.
    
    None // Placeholder: Reuse namespace for named imports requires property access construction support or return type change.
    // For now I'll stick to direct reuse or namespace reuse if symbol is null.
}

pub fn capture_generated_import<TFile: crate::ngtsc::translator::src::import_manager::check_unique_identifier_name::IdentifierScope, TExpression: Clone>(
    request: &ImportRequest<TFile>,
    tracker: &mut ReuseGeneratedImportsTracker<TExpression>,
    reference_node: TExpression, // Identifier or [Identifier, Identifier] in TS. Here TExpression.
) {
    tracker.direct_reuse_cache.insert(hash_import_request(request), reference_node.clone());

    // If it's a namespace import (exportSymbolName is null), capture it in namespace cache too.
    // Checks "&& !Array.isArray(referenceNode)". In Rust assume TExpression is opaque.
    if request.export_symbol_name.is_none() {
        tracker.namespace_import_reuse_cache.insert(request.export_module_specifier.clone(), reference_node);
    }
}

fn hash_import_request<TFile: crate::ngtsc::translator::src::import_manager::check_unique_identifier_name::IdentifierScope>(req: &ImportRequest<TFile>) -> String {
    format!(
        "{}:{}:{}{}",
        req.requested_file.file_name(),
        req.export_module_specifier,
        req.export_symbol_name.as_deref().unwrap_or(""),
        req.unsafe_alias_override.as_deref().map(|s| format!(":{}", s)).unwrap_or_default()
    )
}
