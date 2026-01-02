use super::error::{GenError, GenResult};
use super::*;

impl Codegen {
    pub(super) fn generate_ts_interface_body(&self, body: &[IrNode]) -> GenResult<TokenStream> {
        // Check if body has any control flow
        let has_control_flow = body.iter().any(|n| {
            matches!(
                n,
                IrNode::For { .. } | IrNode::If { .. } | IrNode::While { .. } | IrNode::Match { .. }
            )
        });

        if has_control_flow {
            // Generate code that builds the body dynamically
            let body_stmts: Vec<TokenStream> = body
                .iter()
                .map(|node| self.generate_interface_member_stmt(node))
                .collect::<GenResult<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect();

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::TsInterfaceBody {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    body: {
                        let mut __members: Vec<macroforge_ts::swc_core::ecma::ast::TsTypeElement> = Vec::new();
                        #(#body_stmts)*
                        __members
                    },
                }
            })
        } else {
            // Static body - generate directly
            let members: Vec<TokenStream> = body
                .iter()
                .map(|node| self.generate_interface_member(node))
                .collect::<GenResult<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect();

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::TsInterfaceBody {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    body: vec![#(#members),*],
                }
            })
        }
    }

    pub(super) fn generate_interface_member_stmt(&self, node: &IrNode) -> GenResult<Option<TokenStream>> {
        match node {
            IrNode::For {
                pattern,
                iterator,
                body,
                ..
            } => {
                let body_stmts: Vec<TokenStream> = body
                    .iter()
                    .map(|n| self.generate_interface_member_stmt(n))
                    .collect::<GenResult<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                Ok(Some(quote! {
                    for #pattern in #iterator {
                        #(#body_stmts)*
                    }
                }))
            }
            IrNode::If {
                condition,
                then_body,
                else_if_branches,
                else_body,
                ..
            } => {
                let then_stmts: Vec<TokenStream> = then_body
                    .iter()
                    .map(|n| self.generate_interface_member_stmt(n))
                    .collect::<GenResult<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect();

                let else_code = if else_if_branches.is_empty() && else_body.is_none() {
                    quote! {}
                } else {
                    let mut branches = TokenStream::new();
                    for (cond, body) in else_if_branches {
                        let branch_stmts: Vec<TokenStream> = body
                            .iter()
                            .map(|n| self.generate_interface_member_stmt(n))
                            .collect::<GenResult<Vec<_>>>()?
                            .into_iter()
                            .flatten()
                            .collect();
                        branches.extend(quote! { else if #cond { #(#branch_stmts)* } });
                    }
                    if let Some(body) = else_body {
                        let else_stmts: Vec<TokenStream> = body
                            .iter()
                            .map(|n| self.generate_interface_member_stmt(n))
                            .collect::<GenResult<Vec<_>>>()?
                            .into_iter()
                            .flatten()
                            .collect();
                        branches.extend(quote! { else { #(#else_stmts)* } });
                    }
                    branches
                };

                Ok(Some(quote! {
                    if #condition {
                        #(#then_stmts)*
                    }
                    #else_code
                }))
            }
            IrNode::PropSignature {
                readonly,
                name,
                optional,
                type_ann,
                ..
            } => {
                let member_code =
                    self.generate_prop_signature(*readonly, name, *optional, type_ann.as_deref())?;
                Ok(Some(quote! {
                    __members.push(#member_code);
                }))
            }
            IrNode::MethodSignature {
                name,
                optional,
                type_params,
                params,
                return_type,
                ..
            } => {
                let member_code = self.generate_method_signature(
                    name,
                    *optional,
                    type_params.as_deref(),
                    params,
                    return_type.as_deref(),
                )?;
                Ok(Some(quote! {
                    __members.push(#member_code);
                }))
            }
            IrNode::IndexSignature {
                readonly,
                params,
                type_ann,
                ..
            } => {
                let member_code = self.generate_index_signature(*readonly, params, type_ann)?;
                Ok(Some(quote! {
                    __members.push(#member_code);
                }))
            }
            IrNode::Let {
                pattern,
                mutable,
                type_hint,
                value,
                ..
            } => {
                let mutability = if *mutable {
                    quote! { mut }
                } else {
                    quote! {}
                };

                if let Some(ty) = type_hint {
                    Ok(Some(quote! { let #mutability #pattern: #ty = #value; }))
                } else {
                    Ok(Some(quote! { let #mutability #pattern = #value; }))
                }
            }
            IrNode::Raw { .. } => Ok(None), // Skip whitespace/raw text
            _ => Ok(None),
        }
    }

    pub(super) fn generate_interface_member(&self, node: &IrNode) -> GenResult<Option<TokenStream>> {
        match node {
            IrNode::PropSignature {
                readonly,
                name,
                optional,
                type_ann,
                ..
            } => Ok(Some(self.generate_prop_signature(*readonly, name, *optional, type_ann.as_deref())?)),
            IrNode::MethodSignature {
                name,
                optional,
                type_params,
                params,
                return_type,
                ..
            } => Ok(Some(self.generate_method_signature(
                name,
                *optional,
                type_params.as_deref(),
                params,
                return_type.as_deref(),
            )?)),
            IrNode::IndexSignature {
                readonly,
                params,
                type_ann,
                ..
            } => Ok(Some(self.generate_index_signature(*readonly, params, type_ann)?)),
            _ => Ok(None),
        }
    }

    /// Generate an index signature: [key: Type]: Type
    pub(super) fn generate_index_signature(
        &self,
        readonly: bool,
        params: &[IrNode],
        type_ann: &IrNode,
    ) -> GenResult<TokenStream> {
        let params_code = self.generate_fn_type_params(params)?;
        let type_ann_code = self.generate_type_ann(type_ann)?;

        Ok(quote! {
            macroforge_ts::swc_core::ecma::ast::TsTypeElement::TsIndexSignature(
                macroforge_ts::swc_core::ecma::ast::TsIndexSignature {
                    span: macroforge_ts::swc_core::common::DUMMY_SP,
                    params: #params_code,
                    type_ann: Some(Box::new(#type_ann_code)),
                    readonly: #readonly,
                    is_static: false,
                }
            )
        })
    }
}
