use super::*;

impl Codegen {
    /// Check if a node is a fragment that should be grouped with adjacent fragments.
    pub(super) fn is_fragment_node(&self, node: &IrNode) -> bool {
        matches!(
            node,
            IrNode::Raw { .. }
                | IrNode::Ident { .. }
                | IrNode::StrLit { .. }
                | IrNode::IdentBlock { .. }
                | IrNode::StringInterp { .. }
                | IrNode::Placeholder { .. }
        )
    }

// =========================================================================
// Expression Generation
// =========================================================================

pub(super) fn is_control_flow_node(&self, node: &IrNode) -> bool {
    matches!(
        node,
        IrNode::For { .. }
            | IrNode::If { .. }
            | IrNode::While { .. }
            | IrNode::Match { .. }
            | IrNode::Let { .. }
            | IrNode::Do { .. }
    )
}
}
