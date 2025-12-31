use super::*;

impl Codegen {
    pub(super) fn generate_binary_op(&self, op: &BinaryOp) -> TokenStream {
    match op {
        BinaryOp::Add => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Add },
        BinaryOp::Sub => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Sub },
        BinaryOp::Mul => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Mul },
        BinaryOp::Div => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Div },
        BinaryOp::Mod => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Mod },
        BinaryOp::Exp => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Exp },
        BinaryOp::EqEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::EqEq },
        BinaryOp::NotEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::NotEq },
        BinaryOp::EqEqEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::EqEqEq },
        BinaryOp::NotEqEq => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::NotEqEq },
        BinaryOp::Lt => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Lt },
        BinaryOp::Le => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LtEq },
        BinaryOp::Gt => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::Gt },
        BinaryOp::Ge => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::GtEq },
        BinaryOp::And => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LogicalAnd },
        BinaryOp::Or => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LogicalOr },
        BinaryOp::NullishCoalesce => {
            quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::NullishCoalescing }
        }
        BinaryOp::BitAnd => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::BitAnd },
        BinaryOp::BitOr => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::BitOr },
        BinaryOp::BitXor => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::BitXor },
        BinaryOp::Shl => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::LShift },
        BinaryOp::Shr => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::RShift },
        BinaryOp::UShr => {
            quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::ZeroFillRShift }
        }
        BinaryOp::In => quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::In },
        BinaryOp::InstanceOf => {
            quote! { macroforge_ts::swc_core::ecma::ast::BinaryOp::InstanceOf }
        }
    }
}

pub(super) fn generate_assign_op(&self, op: &AssignOp) -> TokenStream {
    match op {
        AssignOp::Assign => quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::Assign },
        AssignOp::AddAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::AddAssign }
        }
        AssignOp::SubAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::SubAssign }
        }
        AssignOp::MulAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::MulAssign }
        }
        AssignOp::DivAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::DivAssign }
        }
        AssignOp::ModAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::ModAssign }
        }
        AssignOp::ExpAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::ExpAssign }
        }
        AssignOp::ShlAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::LShiftAssign }
        }
        AssignOp::ShrAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::RShiftAssign }
        }
        AssignOp::UShrAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::ZeroFillRShiftAssign }
        }
        AssignOp::BitAndAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::BitAndAssign }
        }
        AssignOp::BitOrAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::BitOrAssign }
        }
        AssignOp::BitXorAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::BitXorAssign }
        }
        AssignOp::AndAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::AndAssign }
        }
        AssignOp::OrAssign => quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::OrAssign },
        AssignOp::NullishAssign => {
            quote! { macroforge_ts::swc_core::ecma::ast::AssignOp::NullishAssign }
        }
    }
}
}
