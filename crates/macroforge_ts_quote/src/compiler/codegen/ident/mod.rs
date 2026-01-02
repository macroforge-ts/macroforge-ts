mod with_optional;

use super::error::{GenError, GenResult};
use super::*;

impl Codegen {
    pub(in super::super) fn generate_ident(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Ident { value: name, .. } => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                })
            }
            // Handle Raw nodes as identifiers (e.g., "*" in `export * as ns`)
            IrNode::Raw { value: name, .. } => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                })
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
                ..
            } => {
                Ok(quote! { macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()) })
            }
            IrNode::Placeholder { kind, .. } => {
                let kind_str = format!("{:?}", kind);
                Err(GenError::invalid_placeholder("identifier", &kind_str, &["Ident"]))
            }
            IrNode::IdentBlock { parts, .. } => {
                // Build the identifier name from parts at runtime
                let part_strs: Vec<TokenStream> = parts
                    .iter()
                    .filter_map(|p| match p {
                        IrNode::Raw { value: text, .. } => Some(quote! { __ident_str.push_str(#text); }),
                        IrNode::StrLit { value: text, .. } => Some(quote! { __ident_str.push_str(#text); }),
                        IrNode::Ident { value: text, .. } => Some(quote! { __ident_str.push_str(#text); }),
                        IrNode::Placeholder { expr, .. } => {
                            Some(quote! { __ident_str.push_str(&(#expr).to_string()); })
                        }
                        _ => None,
                    })
                    .collect();
                Ok(quote! {
                    {
                        let mut __ident_str = String::new();
                        #(#part_strs)*
                        macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            __ident_str.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        )
                    }
                })
            }
            _ => Err(GenError::unexpected_node(
                "identifier",
                node,
                &["Ident", "Raw", "Placeholder(Ident)", "IdentBlock"],
            )),
        }
    }

    pub(in super::super) fn generate_ident_name(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Ident { value: name, .. } => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                })
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
                ..
            } => {
                Ok(quote! {
                    {
                        let __ident = macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone());
                        macroforge_ts::swc_core::ecma::ast::IdentName::new(
                            __ident.sym,
                            __ident.span,
                        )
                    }
                })
            }
            IrNode::Placeholder { kind, .. } => {
                let kind_str = format!("{:?}", kind);
                Err(GenError::invalid_placeholder("identifier name", &kind_str, &["Ident"]))
            }
            _ => Err(GenError::unexpected_node(
                "identifier name",
                node,
                &["Ident", "Placeholder(Ident)"],
            )),
        }
    }
}
