use super::error::{GenError, GenResult};
use super::*;

impl Codegen {
    pub(super) fn generate_props(&self, props: &[IrNode]) -> GenResult<TokenStream> {
        let mut props_code: Vec<TokenStream> = Vec::new();
        for p in props {
            if let Some(code) = self.generate_prop(p)? {
                props_code.push(code);
            }
        }
        Ok(quote! { vec![#(#props_code),*] })
    }

pub(super) fn generate_prop(&self, node: &IrNode) -> GenResult<Option<TokenStream>> {
    match node {
        IrNode::KeyValueProp { key, value, .. } => {
            let key_code = self.generate_prop_name(key)?;
            let value_code = self.generate_expr(value)?;
            Ok(Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::KeyValue(
                        macroforge_ts::swc_core::ecma::ast::KeyValueProp {
                            key: #key_code,
                            value: Box::new(#value_code),
                        }
                    )
                ))
            }))
        }
        IrNode::ShorthandProp { key, .. } => {
            let key_code = self.generate_ident(key)?;
            Ok(Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::Shorthand(#key_code)
                ))
            }))
        }
        IrNode::SpreadElement { expr, .. } => {
            let expr_code = self.generate_expr(expr)?;
            Ok(Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Spread(
                    macroforge_ts::swc_core::ecma::ast::SpreadElement {
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            }))
        }
        // Method property: `name() { }`
        IrNode::MethodProp { async_, generator, name, type_params, params, return_type, body, .. } => {
            let key_code = self.generate_prop_name(name)?;
            let params_code = self.generate_params(params)?;
            let body_code = self.generate_block_stmt(body)?;
            let type_params_code = match type_params {
                Some(tp) => {
                    let tpc = self.generate_type_params(tp)?;
                    quote! { Some(Box::new(#tpc)) }
                }
                None => quote! { None },
            };
            let return_type_code = match return_type {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            Ok(Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::Method(
                        macroforge_ts::swc_core::ecma::ast::MethodProp {
                            key: #key_code,
                            function: Box::new(macroforge_ts::swc_core::ecma::ast::Function {
                                params: #params_code,
                                decorators: vec![],
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                ctxt: macroforge_ts::swc_core::common::SyntaxContext::empty(),
                                body: Some(#body_code),
                                is_generator: #generator,
                                is_async: #async_,
                                type_params: #type_params_code,
                                return_type: #return_type_code,
                            }),
                        }
                    )
                ))
            }))
        }
        // Getter property: `get name() { }`
        IrNode::GetterProp { name, type_ann, body, .. } => {
            let key_code = self.generate_prop_name(name)?;
            let body_code = self.generate_block_stmt(body)?;
            let type_ann_code = match type_ann {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            Ok(Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::Getter(
                        macroforge_ts::swc_core::ecma::ast::GetterProp {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            key: #key_code,
                            type_ann: #type_ann_code,
                            body: Some(#body_code),
                        }
                    )
                ))
            }))
        }
        // Setter property: `set name(param) { }`
        IrNode::SetterProp { name, param, body, .. } => {
            let key_code = self.generate_prop_name(name)?;
            let param_code = self.generate_param(param)?;
            let body_code = self.generate_block_stmt(body)?;
            Ok(Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::Setter(
                        macroforge_ts::swc_core::ecma::ast::SetterProp {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            key: #key_code,
                            this_param: None,
                            param: #param_code,
                            body: Some(#body_code),
                        }
                    )
                ))
            }))
        }
        // For control flow in object literals - handled at higher level
        IrNode::If { .. } | IrNode::For { .. } => Ok(None),
        _ => Ok(None),
    }
}

pub(super) fn generate_prop_name(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::Ident { value: name, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                )
            })
        }
        IrNode::StrLit { value, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Str(
                    macroforge_ts::swc_core::ecma::ast::Str {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        value: #value.into(),
                        raw: None,
                    }
                )
            })
        }
        IrNode::ComputedPropName { expr, .. } => {
            let expr_code = self.generate_expr(expr)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Computed(
                    macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            })
        }
        // Handle all placeholder kinds in property key position
        // Ident kind uses ToTsIdent directly
        IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr,
            ..
        } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Ident({
                    let __ident = macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone());
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        __ident.sym,
                        __ident.span,
                    )
                })
            })
        }
        // Expr kind in property position: use computed property with the expression
        IrNode::Placeholder {
            kind: PlaceholderKind::Expr,
            expr,
            ..
        } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Computed(
                    macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone())),
                    }
                )
            })
        }
        // Type placeholder shouldn't appear in property key position - error
        IrNode::Placeholder {
            kind: PlaceholderKind::Type,
            ..
        } => {
            Err(GenError::invalid_placeholder(
                "property name",
                "Type",
                &["Ident", "Expr"],
            ))
        }
        // Stmt placeholder also shouldn't be in property key position - error
        IrNode::Placeholder {
            kind: PlaceholderKind::Stmt,
            ..
        } => {
            Err(GenError::invalid_placeholder(
                "property name",
                "Stmt",
                &["Ident", "Expr"],
            ))
        }
        _ => Err(GenError::unexpected_node(
            "property name",
            node,
            &["Ident", "StrLit", "ComputedPropName", "Placeholder(Ident)", "Placeholder(Expr)"],
        )),
    }
}
}
