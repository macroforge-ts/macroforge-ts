use swc_core::ecma::ast::*;
use syn::parse_quote;

use super::ToCode;
use crate::ctxt::{Ctx, VarPos};

impl ToCode for swc_core::ecma::ast::Ident {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        if let Some(var_name) = self.sym.strip_prefix('$')
            && let Some(var) = cx.var(VarPos::Ident, var_name)
        {
            return var.get_expr();
        }

        let sym_value = self.sym.to_code(cx);
        let optional_value = self.optional.to_code(cx);
        parse_quote!(macroforge_ts::swc_core::ecma::ast::Ident {
            span: macroforge_ts::swc_core::common::DUMMY_SP,
            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
            sym: #sym_value,
            optional: #optional_value,
        })
    }
}

impl ToCode for swc_core::ecma::ast::IdentName {
    fn to_code(&self, cx: &Ctx) -> syn::Expr {
        if let Some(var_name) = self.sym.strip_prefix('$')
            && let Some(var) = cx.var(VarPos::Ident, var_name)
        {
            let var_expr = var.get_expr();
            return parse_quote!(#var_expr.into());
        }

        let span_value = self.span.to_code(cx);
        let sym_value = self.sym.to_code(cx);
        parse_quote!(macroforge_ts::swc_core::ecma::ast::IdentName {
            span: #span_value,
            sym: #sym_value,
        })
    }
}

impl_struct!(PrivateName, [span, name]);
