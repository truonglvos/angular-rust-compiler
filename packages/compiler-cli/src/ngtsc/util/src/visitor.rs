// Visitor Utilities
//
// Tree visitor utilities.

/// Visitor trait for AST traversal.
pub trait Visitor {
    type Node;
    type Result;
    
    fn visit(&mut self, node: &Self::Node) -> Self::Result;
}

/// Visit all children of a node.
pub fn visit_each<V, N, R>(visitor: &mut V, nodes: &[N]) -> Vec<R>
where
    V: Visitor<Node = N, Result = R>,
{
    nodes.iter().map(|n| visitor.visit(n)).collect()
}
