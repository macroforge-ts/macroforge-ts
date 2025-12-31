use super::*;

impl Codegen {
    pub(super) fn generate_param(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::Param { decorators: _, pat } => {
            let pat_code = self.generate_pat(pat);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Param {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    decorators: vec![],
                    pat: #pat_code,
                }
            }
        }
        IrNode::BindingIdent {
            name,
            type_ann,
            optional,
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
                macroforge_ts::swc_core::ecma::ast::Param {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    decorators: vec![],
                    pat: macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: #name_code,
                            type_ann: #type_ann_code,
                        }
                    ),
                }
            }
        }
        // For other node types, assume they're identifiers
        _ => {
            let ident_code = self.generate_ident(node);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Param {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    decorators: vec![],
                    pat: macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: #ident_code,
                            type_ann: None,
                        }
                    ),
                }
            }
        }
    }
}
/// Generate params as `Vec<TsFnParam>` for interface method signatures.
    pub(super) fn generate_ts_fn_params(&self, params: &[IrNode]) -> TokenStream {
    let params_code: Vec<TokenStream> = params
        .iter()
        .map(|p| self.generate_ts_fn_param(p))
        .collect();
    quote! { vec![#(#params_code),*] }
}

/// Generate a single `TsFnParam` for interface method signatures.
    pub(super) fn generate_ts_fn_param(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::BindingIdent {
            name,
            type_ann,
            optional,
        } => {
            let name_code = self.generate_ident_with_optional(name, *optional);
            let type_ann_code = type_ann
                .as_ref()
                .map(|t| {
                    let tc = self.generate_type_ann(t);
                    quote! { Some(Box::new(#tc)) }
                })
                .unwrap_or(quote! { None });

            quote! {
                macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: #name_code,
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        IrNode::Param { pat, .. } => {
            // Recurse to the pattern
            self.generate_ts_fn_param(pat)
        }
        IrNode::RestPat { arg, type_ann } => {
            let arg_code = self.generate_pat(arg);
            let type_ann_code = type_ann
                .as_ref()
                .map(|t| {
                    let tc = self.generate_type_ann(t);
                    quote! { Some(Box::new(#tc)) }
                })
                .unwrap_or(quote! { None });

            quote! {
                macroforge_ts::swc_core::ecma::ast::TsFnParam::Rest(
                    macroforge_ts::swc_core::ecma::ast::RestPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        // Fallback: treat as identifier
        _ => {
            let ident_code = self.generate_ident(node);
            quote! {
                macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: #ident_code,
                        type_ann: None,
                    }
                )
            }
        }
    }
}
pub(super) fn generate_params(&self, params: &[IrNode]) -> TokenStream {
    let params_code: Vec<TokenStream> = params.iter().map(|p| self.generate_param(p)).collect();
    quote! { vec![#(#params_code),*] }
}
}
