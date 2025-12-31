use super::*;

impl Codegen {
    pub(super) fn generate_pat(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::BindingIdent {
            name,
            type_ann,
            optional: _,
        } => {
            let name_code = self.generate_ident(name);
            let type_ann_code = type_ann
                .as_ref()
                .map(|t| {
                    let tc = self.generate_type_ann(t);
                    quote! { Some(Box::new(#tc)) }
                })
                .unwrap_or(quote! { None });

            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: #name_code,
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        IrNode::Ident(name) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        ),
                        type_ann: None,
                    }
                )
            }
        }
        IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr,
        } => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()),
                        type_ann: None,
                    }
                )
            }
        }
        IrNode::IdentBlock { parts } => {
            // Build identifier string from parts at runtime
            let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .filter_map(|p| match p {
                        IrNode::Raw(text) => Some(quote! { __ident.push_str(#text); }),
                        IrNode::StrLit(text) => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Ident(text) => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Placeholder { expr, .. } => {
                            Some(quote! {
                                __ident.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                            })
                        }
                        _ => None,
                    })
                    .collect();

            quote! {
                {
                    let mut __ident = String::new();
                    #(#part_exprs)*
                    macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                __ident.into(),
                                macroforge_ts::swc_core::common::DUMMY_SP,
                            ),
                            type_ann: None,
                        }
                    )
                }
            }
        }
        _ => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Invalid(
                    macroforge_ts::swc_core::ecma::ast::Invalid {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                    }
                )
            }
        }
    }
}

pub(super) fn generate_pats(&self, nodes: &[IrNode]) -> TokenStream {
    let pats_code: Vec<TokenStream> = nodes.iter().map(|p| self.generate_pat(p)).collect();
    quote! { vec![#(#pats_code),*] }
}
}
