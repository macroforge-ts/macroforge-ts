use super::super::*;

impl Codegen {
    /// Try to generate a node as an expression.
    pub(in super::super::super) fn try_generate_as_expr(&self, node: &IrNode) -> Option<TokenStream> {
    match node {
        IrNode::Ident(_)
        | IrNode::StrLit(_)
        | IrNode::NumLit(_)
        | IrNode::BoolLit(_)
        | IrNode::NullLit
        | IrNode::ThisExpr
        | IrNode::CallExpr { .. }
        | IrNode::MemberExpr { .. }
        | IrNode::ObjectLit { .. }
        | IrNode::ArrayLit { .. }
        | IrNode::BinExpr { .. }
        | IrNode::AssignExpr { .. }
        | IrNode::CondExpr { .. }
        | IrNode::ArrowExpr { .. }
        | IrNode::NewExpr { .. }
        | IrNode::TplLit { .. }
        | IrNode::Raw(_)
        | IrNode::Placeholder { .. }
        | IrNode::IdentBlock { .. }
        | IrNode::StringInterp { .. } => Some(self.generate_expr(node)),
        _ => None,
    }
}
}
