use super::error::{GenError, GenResult};
use super::*;

impl Codegen {
    pub(super) fn generate_param(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::Param { decorators: _, pat, .. } => {
                let pat_code = self.generate_pat(pat)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::Param {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        decorators: vec![],
                        pat: #pat_code,
                    }
                })
            }
            IrNode::BindingIdent {
                name,
                type_ann,
                optional: _,
                ..
            } => {
                let name_code = self.generate_ident(name)?;
                // Type annotation is legitimately optional for parameters
                let type_ann_code = type_ann
                    .as_ref()
                    .map(|t| -> GenResult<TokenStream> {
                        let tc = self.generate_type_ann(t)?;
                        Ok(quote! { Some(Box::new(#tc)) })
                    })
                    .transpose()?
                    .unwrap_or(quote! { None });

                Ok(quote! {
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
                })
            }
            // Ident nodes are valid - treat them as simple parameter identifiers
            IrNode::Ident { .. } | IrNode::Placeholder { .. } => {
                let ident_code = self.generate_ident(node)?;
                Ok(quote! {
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
                })
            }
            _ => Err(GenError::unexpected_node(
                "parameter",
                node,
                &["Param", "BindingIdent", "Ident", "Placeholder"],
            )),
        }
    }

    /// Generate params as `Vec<TsFnParam>` for interface method signatures.
    pub(super) fn generate_ts_fn_params(&self, params: &[IrNode]) -> GenResult<TokenStream> {
        let params_code: Vec<TokenStream> = params
            .iter()
            .map(|p| self.generate_ts_fn_param(p))
            .collect::<GenResult<Vec<_>>>()?;
        Ok(quote! { vec![#(#params_code),*] })
    }

    /// Generate a single `TsFnParam` for interface method signatures.
    pub(super) fn generate_ts_fn_param(&self, node: &IrNode) -> GenResult<TokenStream> {
        match node {
            IrNode::BindingIdent {
                name,
                type_ann,
                optional,
                ..
            } => {
                let name_code = self.generate_ident_with_optional(name, *optional)?;
                // Type annotation is legitimately optional for parameters
                let type_ann_code = type_ann
                    .as_ref()
                    .map(|t| -> GenResult<TokenStream> {
                        let tc = self.generate_type_ann(t)?;
                        Ok(quote! { Some(Box::new(#tc)) })
                    })
                    .transpose()?
                    .unwrap_or(quote! { None });

                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: #name_code,
                            type_ann: #type_ann_code,
                        }
                    )
                })
            }
            IrNode::Param { pat, .. } => {
                // Recurse to the pattern
                self.generate_ts_fn_param(pat)
            }
            IrNode::RestPat { arg, type_ann, .. } => {
                let arg_code = self.generate_pat(arg)?;
                // Type annotation is legitimately optional for rest parameters
                let type_ann_code = type_ann
                    .as_ref()
                    .map(|t| -> GenResult<TokenStream> {
                        let tc = self.generate_type_ann(t)?;
                        Ok(quote! { Some(Box::new(#tc)) })
                    })
                    .transpose()?
                    .unwrap_or(quote! { None });

                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsFnParam::Rest(
                        macroforge_ts::swc_core::ecma::ast::RestPat {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                            arg: Box::new(#arg_code),
                            type_ann: #type_ann_code,
                        }
                    )
                })
            }
            // Ident nodes are valid - treat them as simple parameter identifiers
            IrNode::Ident { .. } | IrNode::Placeholder { .. } => {
                let ident_code = self.generate_ident(node)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::TsFnParam::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: #ident_code,
                            type_ann: None,
                        }
                    )
                })
            }
            _ => Err(GenError::unexpected_node(
                "TypeScript function parameter",
                node,
                &["BindingIdent", "Param", "RestPat", "Ident", "Placeholder"],
            )),
        }
    }

    pub(super) fn generate_params(&self, params: &[IrNode]) -> GenResult<TokenStream> {
        let params_code: Vec<TokenStream> = params
            .iter()
            .map(|p| self.generate_param(p))
            .collect::<GenResult<Vec<_>>>()?;
        Ok(quote! { vec![#(#params_code),*] })
    }
}
