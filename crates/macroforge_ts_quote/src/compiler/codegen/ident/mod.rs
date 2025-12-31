mod with_optional;

use super::*;

impl Codegen {
    pub(in super::super) fn generate_ident(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::Ident(name) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                    #name.into(),
                    macroforge_ts::swc_core::common::DUMMY_SP,
                )
            }
        }
        IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr,
        } => {
            quote! { macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()) }
        }
        IrNode::IdentBlock { parts } => {
            // Build the identifier name from parts at runtime
            let part_strs: Vec<TokenStream> = parts
                .iter()
                .filter_map(|p| match p {
                    IrNode::Raw(text) => Some(quote! { __ident_str.push_str(#text); }),
                    IrNode::StrLit(text) => Some(quote! { __ident_str.push_str(#text); }),
                    IrNode::Ident(text) => Some(quote! { __ident_str.push_str(#text); }),
                    IrNode::Placeholder { expr, .. } => {
                        Some(quote! { __ident_str.push_str(&(#expr).to_string()); })
                    }
                    _ => None,
                })
                .collect();
            quote! {
                {
                    let mut __ident_str = String::new();
                    #(#part_strs)*
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        __ident_str.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                }
            }
        }
        _ => quote! {
            macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                "".into(),
                macroforge_ts::swc_core::common::DUMMY_SP,
            )
        },
    }
}

pub(in super::super) fn generate_ident_name(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::Ident(name) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::IdentName::new(
                    #name.into(),
                    macroforge_ts::swc_core::common::DUMMY_SP,
                )
            }
        }
        IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr,
        } => {
            quote! {
                {
                    let __ident = macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone());
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        __ident.sym,
                        __ident.span,
                    )
                }
            }
        }
        _ => quote! {
            macroforge_ts::swc_core::ecma::ast::IdentName::new(
                "".into(),
                macroforge_ts::swc_core::common::DUMMY_SP,
            )
        },
    }
}
}
