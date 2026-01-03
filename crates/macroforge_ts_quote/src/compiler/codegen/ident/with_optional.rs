use super::super::error::{GenError, GenResult};
use super::super::*;

impl Codegen {
    /// Generate an identifier with the optional flag set (for optional parameters)
    pub(in super::super::super) fn generate_ident_with_optional(&self, node: &IrNode, optional: bool) -> GenResult<TokenStream> {
        match node {
            IrNode::Ident { value: name, .. } => {
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Ident {
                        sym: #name.into(),
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: Default::default(),
                        optional: #optional,
                    }
                })
            }
            IrNode::Placeholder {
                kind: PlaceholderKind::Ident,
                expr,
                ..
            } => {
                if optional {
                    Ok(quote! {
                        {
                            let mut __ident = macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone());
                            __ident.optional = true;
                            __ident
                        }
                    })
                } else {
                    Ok(quote! { macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()) })
                }
            }
            IrNode::Placeholder { kind, .. } => {
                let kind_str = format!("{:?}", kind);
                Err(GenError::invalid_placeholder("identifier (with optional)", &kind_str, &["Ident"]))
            }
            IrNode::IdentBlock { parts, .. } => {
                // Build the identifier name from parts at runtime
                let part_strs: Vec<TokenStream> = parts
                    .iter()
                    .filter_map(|p| match p {
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
                        macroforge_ts::swc_core::ecma::ast::Ident {
                            sym: __ident_str.into(),
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: Default::default(),
                            optional: #optional,
                        }
                    }
                })
            }
            // Fallback for other node types - propagate the error from generate_ident
            _ => self.generate_ident(node),
        }
    }
}
