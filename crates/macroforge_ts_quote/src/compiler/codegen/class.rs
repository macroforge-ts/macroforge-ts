use super::error::{GenError, GenResult};
use super::*;

impl Codegen {
    pub(super) fn generate_class_members(&self, members: &[IrNode]) -> GenResult<TokenStream> {
    // Check if any member contains control flow (for loop, if, etc.)
    let has_control_flow = members.iter().any(|n| {
        matches!(
            n,
            IrNode::For { .. }
                | IrNode::If { .. }
                | IrNode::While { .. }
                | IrNode::Match { .. }
                | IrNode::Let { .. }
                | IrNode::Do { .. }
        )
    });

    if has_control_flow {
        // Generate code that dynamically builds the vec
        let member_pushes = self.generate_class_member_pushes(members)?;
        Ok(quote! {
            {
                let mut __class_members: Vec<macroforge_ts::swc_core::ecma::ast::ClassMember> = Vec::new();
                #member_pushes
                __class_members
            }
        })
    } else {
        // No control flow - generate literal vec
        let members_code: Vec<TokenStream> = members
            .iter()
            .map(|m| self.generate_class_member(m))
            .collect::<GenResult<Vec<_>>>()?;
        Ok(quote! { vec![#(#members_code),*] })
    }
}

pub(super) fn generate_class_member_pushes(&self, members: &[IrNode]) -> GenResult<TokenStream> {
    let pushes: Vec<TokenStream> = members
        .iter()
        .map(|m| self.generate_class_member_push(m))
        .collect::<GenResult<Vec<_>>>()?;
    Ok(quote! { #(#pushes)* })
}

pub(super) fn generate_class_member_push(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::For {
            pattern,
            iterator,
            body,
            ..
        } => {
            let body_pushes = self.generate_class_member_pushes(body)?;
            Ok(quote! {
                for #pattern in #iterator { #body_pushes }
            })
        }
        IrNode::If {
            condition,
            then_body,
            else_if_branches,
            else_body,
            ..
        } => {
            let then_pushes = self.generate_class_member_pushes(then_body)?;

            let else_code = if else_if_branches.is_empty() && else_body.is_none() {
                quote! {}
            } else {
                let mut branches = TokenStream::new();
                for (cond, body) in else_if_branches {
                    let b = self.generate_class_member_pushes(body)?;
                    branches.extend(quote! { else if #cond { #b } });
                }
                if let Some(eb) = else_body {
                    let eb_pushes = self.generate_class_member_pushes(eb)?;
                    branches.extend(quote! { else { #eb_pushes } });
                }
                branches
            };

            Ok(quote! {
                if #condition { #then_pushes } #else_code
            })
        }
        IrNode::While { condition, body, .. } => {
            let body_pushes = self.generate_class_member_pushes(body)?;
            Ok(quote! {
                while #condition { #body_pushes }
            })
        }
        IrNode::Match { expr, arms, .. } => {
            let arm_tokens: Vec<TokenStream> = arms
                .iter()
                .map(
                    |MatchArm {
                         pattern,
                         guard,
                         body,
                         ..
                     }| {
                        let b = self.generate_class_member_pushes(body)?;
                        if let Some(g) = guard {
                            Ok(quote! { #pattern if #g => { #b } })
                        } else {
                            Ok(quote! { #pattern => { #b } })
                        }
                    },
                )
                .collect::<GenResult<Vec<_>>>()?;
            Ok(quote! {
                match #expr { #(#arm_tokens)* }
            })
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
                Ok(quote! { let #mutability #pattern: #ty = #value; })
            } else {
                Ok(quote! { let #mutability #pattern = #value; })
            }
        }
        IrNode::Do { code, .. } => Ok(quote! { #code; }),
        _ => {
            // Regular class member - push to __class_members
            let member_code = self.generate_class_member(node)?;
            Ok(quote! { __class_members.push(#member_code); })
        }
    }
}

pub(super) fn generate_class_member(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::Constructor {
            accessibility,
            params,
            body,
            ..
        } => {
            let params_code = self.generate_constructor_params(params)?;
            let body_code = match body {
                Some(b) => self.generate_block_stmt_opt(b)?,
                None => quote! { None },
            };
            let accessibility_code = match accessibility {
                Some(Accessibility::Public) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Public) }
                }
                Some(Accessibility::Private) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Private) }
                }
                Some(Accessibility::Protected) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Protected) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::ClassMember::Constructor(
                    macroforge_ts::swc_core::ecma::ast::Constructor {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                        key: macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                            macroforge_ts::swc_core::ecma::ast::IdentName::new(
                                "constructor".into(),
                                macroforge_ts::swc_core::common::DUMMY_SP,
                            )
                        ),
                        params: #params_code,
                        body: #body_code,
                        accessibility: #accessibility_code,
                        is_optional: false,
                    }
                )
            })
        }
        IrNode::Method {
            static_,
            accessibility,
            readonly: _,
            async_,
            generator,
            kind,
            name,
            optional,
            type_params,
            params,
            return_type,
            body,
            ..
        } => {
            let name_code = self.generate_prop_name(name)?;
            let params_code = self.generate_params(params)?;
            let body_code = match body {
                Some(b) => self.generate_block_stmt_opt(b)?,
                None => quote! { None },
            };
            let return_type_code = match return_type {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            let type_params_code = match type_params {
                Some(tp) => {
                    let tpc = self.generate_type_params(tp)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };
            let accessibility_code = match accessibility {
                Some(Accessibility::Public) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Public) }
                }
                Some(Accessibility::Private) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Private) }
                }
                Some(Accessibility::Protected) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Protected) }
                }
                None => quote! { None },
            };
            let kind_code = match kind {
                MethodKind::Method => {
                    quote! { macroforge_ts::swc_core::ecma::ast::MethodKind::Method }
                }
                MethodKind::Getter => {
                    quote! { macroforge_ts::swc_core::ecma::ast::MethodKind::Getter }
                }
                MethodKind::Setter => {
                    quote! { macroforge_ts::swc_core::ecma::ast::MethodKind::Setter }
                }
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::ClassMember::Method(
                    macroforge_ts::swc_core::ecma::ast::ClassMethod {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        key: #name_code,
                        function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                            params: #params_code,
                            decorators: vec![],
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                            body: #body_code,
                            is_generator: #generator,
                            is_async: #async_,
                            type_params: #type_params_code,
                            return_type: #return_type_code,
                        }),
                        kind: #kind_code,
                        is_static: #static_,
                        accessibility: #accessibility_code,
                        is_abstract: false,
                        is_optional: #optional,
                        is_override: false,
                    }
                )
            })
        }
        IrNode::ClassProp {
            static_,
            accessibility,
            readonly,
            declare,
            optional,
            definite,
            name,
            type_ann,
            value,
            ..
        } => {
            let name_code = self.generate_prop_name(name)?;
            let type_ann_code = match type_ann {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            let value_code = match value {
                Some(v) => {
                    let vc = self.generate_expr(v)?;
                    quote! { Some(Box::new(#vc)) }
                }
                None => quote! { None },
            };
            let accessibility_code = match accessibility {
                Some(Accessibility::Public) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Public) }
                }
                Some(Accessibility::Private) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Private) }
                }
                Some(Accessibility::Protected) => {
                    quote! { Some(macroforge_ts::swc_core::ecma::ast::Accessibility::Protected) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::ClassMember::ClassProp(
                    macroforge_ts::swc_core::ecma::ast::ClassProp {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        key: #name_code,
                        value: #value_code,
                        type_ann: #type_ann_code,
                        is_static: #static_,
                        decorators: vec![],
                        accessibility: #accessibility_code,
                        is_abstract: false,
                        is_optional: #optional,
                        is_override: false,
                        readonly: #readonly,
                        declare: #declare,
                        definite: #definite,
                    }
                )
            })
        }
        _ => Err(GenError::unexpected_node(
            "class member",
            node,
            &["Constructor", "Method", "ClassProp"],
        )),
    }
}

    pub(super) fn generate_constructor_params(&self, params: &[IrNode]) -> GenResult<TokenStream> {
        let params_code: Vec<TokenStream> = params
            .iter()
            .map(|p| {
                let param_code = self.generate_param(p)?;
                Ok(quote! {
                    macroforge_ts::swc_core::ecma::ast::ParamOrTsParamProp::Param(#param_code)
                })
            })
            .collect::<GenResult<Vec<_>>>()?;
        Ok(quote! { vec![#(#params_code),*] })
    }
}
