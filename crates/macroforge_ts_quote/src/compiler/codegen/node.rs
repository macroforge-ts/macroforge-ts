use super::*;

impl Codegen {
    /// Check if a node is a fragment that should be grouped with adjacent fragments.
    pub(super) fn is_fragment_node(&self, node: &IrNode) -> bool {
        matches!(
            node,
            IrNode::Ident { .. }
                | IrNode::StrLit { .. }
                | IrNode::IdentBlock { .. }
                | IrNode::StringInterp { .. }
                | IrNode::Placeholder { .. }
        )
    }

}
