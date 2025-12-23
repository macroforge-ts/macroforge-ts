use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use crate::template::IdentPart;

/// Builds an SWC identifier from an ident block's parts.
pub fn compile_ident_block(parts: &[IdentPart]) -> TokenStream2 {
    let mut stmts = TokenStream2::new();
    let mut part_index = 0usize;

    for part in parts {
        match part {
            IdentPart::Static(s) => {
                stmts.extend(quote! {
                    __mf_name.push_str(#s);
                });
            }
            IdentPart::Interpolation { expr, .. } => {
                let var =
                    proc_macro2::Ident::new(&format!("__mf_part_{part_index}"), Span::call_site());
                part_index += 1;
                stmts.extend(quote! {
                    let #var: swc_core::ecma::ast::Ident = (#expr).clone().into();
                    __mf_name.push_str(&#var.sym.to_string());
                });
            }
        }
    }

    quote! {
        {
            let mut __mf_name = String::new();
            #stmts
            swc_core::ecma::ast::Ident::new(
                __mf_name.into(),
                swc_core::common::DUMMY_SP,
                swc_core::common::SyntaxContext::empty()
            )
        }
    }
}
