use super::super::error::GenResult;
use super::super::*;

impl Codegen {
    /// Try to generate a node as an expression.
    /// Returns Ok(None) if the node is not an expression type.
    /// Returns Ok(Some(code)) if generation succeeds.
    /// Returns Err(e) if the node is an expression type but generation fails.
    pub(in super::super::super) fn try_generate_as_expr(&self, node: &IrNode) -> GenResult<Option<TokenStream>> {
    match node {
        IrNode::Ident { .. }
        | IrNode::StrLit { .. }
        | IrNode::NumLit { .. }
        | IrNode::BoolLit { .. }
        | IrNode::NullLit { .. }
        | IrNode::ThisExpr { .. }
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
        | IrNode::Raw { .. }
        | IrNode::Placeholder { .. }
        | IrNode::IdentBlock { .. }
        | IrNode::StringInterp { .. }
        // Phase 3: TypeScript expression types
        | IrNode::TsAsExpr { .. }
        | IrNode::TsSatisfiesExpr { .. }
        | IrNode::TsNonNullExpr { .. }
        | IrNode::TsInstantiation { .. }
        | IrNode::AwaitExpr { .. }
        | IrNode::YieldExpr { .. }
        // Phase 4: Literal/operator expressions
        | IrNode::PrivateName { .. }
        | IrNode::BigIntLit { .. }
        | IrNode::UpdateExpr { .. }
        | IrNode::OptChainExpr { .. }
        // Phase 5: Complex expressions
        | IrNode::FnExpr { .. }
        | IrNode::ClassExpr { .. }
        | IrNode::ParenExpr { .. }
        | IrNode::SeqExpr { .. }
        | IrNode::TaggedTpl { .. } => Ok(Some(self.generate_expr(node)?)),
        _ => Ok(None),
    }
}
}
