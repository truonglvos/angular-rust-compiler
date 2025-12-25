// Semantic Graph Utilities

/// Check if two sets of references are semantically equal.
pub fn references_equal(
    a: &[super::api::SemanticReference],
    b: &[super::api::SemanticReference],
) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_set: std::collections::HashSet<_> = a.iter().collect();
    let b_set: std::collections::HashSet<_> = b.iter().collect();

    a_set == b_set
}

/// Check if a symbol's dependencies have changed.
pub fn has_dependency_changes(
    current_deps: &std::collections::HashSet<String>,
    prior_deps: &std::collections::HashSet<String>,
) -> bool {
    current_deps != prior_deps
}
