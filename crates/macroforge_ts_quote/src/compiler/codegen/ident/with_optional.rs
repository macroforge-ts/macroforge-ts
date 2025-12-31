use super::super::*;

impl Codegen {
    /// Generate an identifier with the optional flag set (for optional parameters)
    pub(in super::super::super) fn generate_ident_with_optional(&self, node: &IrNode, optional: bool) -> TokenStream {
    match node {
        IrNode::Ident(name) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::Ident {
                    sym: #name.into(),
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    ctxt: Default::default(),
                    optional: #optional,
                }
            }
        }
        IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr,
        } => {
            if optional {
                quote! {
                    {
                        let mut __ident = macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone());
                        __ident.optional = true;
                        __ident
                    }
                }
            } else {
                quote! { macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()) }
            }
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
                    macroforge_ts::swc_core::ecma::ast::Ident {
                        sym: __ident_str.into(),
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: Default::default(),
                        optional: #optional,
                    }
                }
            }
        }
        // Fallback for other node types
        _ => self.generate_ident(node),
    }
}
}
