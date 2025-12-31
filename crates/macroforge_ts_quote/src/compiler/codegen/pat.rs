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
        IrNode::Placeholder { kind, expr } => {
            match kind {
                PlaceholderKind::Ident => {
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()),
                                type_ann: None,
                            }
                        )
                    }
                }
                PlaceholderKind::Expr => {
                    // Expressions can be patterns too (like member expressions for assignment)
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::Pat::Expr(
                            Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()))
                        )
                    }
                }
                _ => {
                    // Fallback: treat as expression pattern
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::Pat::Expr(
                            Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()))
                        )
                    }
                }
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
        IrNode::ArrayPat { elems, type_ann, optional } => {
            let elems_code: Vec<TokenStream> = elems.iter().map(|opt_elem| {
                match opt_elem {
                    Some(elem) => {
                        let elem_code = self.generate_pat(elem);
                        quote! { Some(#elem_code) }
                    }
                    None => quote! { None },
                }
            }).collect();
            let type_ann_code = type_ann.as_ref().map(|t| {
                let tc = self.generate_type_ann(t);
                quote! { Some(Box::new(#tc)) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Array(
                    macroforge_ts::swc_core::ecma::ast::ArrayPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        elems: vec![#(#elems_code),*],
                        optional: #optional,
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        IrNode::ObjectPat { props, type_ann, optional } => {
            let props_code: Vec<TokenStream> = props.iter().map(|prop| {
                self.generate_object_pat_prop(prop)
            }).collect();
            let type_ann_code = type_ann.as_ref().map(|t| {
                let tc = self.generate_type_ann(t);
                quote! { Some(Box::new(#tc)) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Object(
                    macroforge_ts::swc_core::ecma::ast::ObjectPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        props: vec![#(#props_code),*],
                        optional: #optional,
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        IrNode::RestPat { arg, type_ann } => {
            let arg_code = self.generate_pat(arg);
            let type_ann_code = type_ann.as_ref().map(|t| {
                let tc = self.generate_type_ann(t);
                quote! { Some(Box::new(#tc)) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Rest(
                    macroforge_ts::swc_core::ecma::ast::RestPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        _ => {
            // Fallback: try to generate as expression and wrap in Pat::Expr
            let expr_code = self.generate_expr(node);
            quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Expr(Box::new(#expr_code))
            }
        }
    }
}

/// Generate code for an object pattern property.
pub(super) fn generate_object_pat_prop(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::ObjectPatProp { key, value } => {
            let key_code = self.generate_ident(key);
            match value {
                Some(val) => {
                    let val_code = self.generate_pat(val);
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::ObjectPatProp::KeyValue(
                            macroforge_ts::swc_core::ecma::ast::KeyValuePatProp {
                                key: macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                                        #key_code.sym.clone(),
                                        macroforge_ts::swc_core::common::DUMMY_SP,
                                    )
                                ),
                                value: Box::new(#val_code),
                            }
                        )
                    }
                }
                None => {
                    // Shorthand: { a } is the same as { a: a }
                    quote! {
                        macroforge_ts::swc_core::ecma::ast::ObjectPatProp::Assign(
                            macroforge_ts::swc_core::ecma::ast::AssignPatProp {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                key: #key_code,
                                value: None,
                            }
                        )
                    }
                }
            }
        }
        IrNode::RestPat { arg, type_ann } => {
            let arg_code = self.generate_pat(arg);
            let type_ann_code = type_ann.as_ref().map(|t| {
                let tc = self.generate_type_ann(t);
                quote! { Some(Box::new(#tc)) }
            }).unwrap_or(quote! { None });
            quote! {
                macroforge_ts::swc_core::ecma::ast::ObjectPatProp::Rest(
                    macroforge_ts::swc_core::ecma::ast::RestPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                        type_ann: #type_ann_code,
                    }
                )
            }
        }
        _ => {
            // Fallback: try to create an assign pattern
            let key_code = self.generate_ident(node);
            quote! {
                macroforge_ts::swc_core::ecma::ast::ObjectPatProp::Assign(
                    macroforge_ts::swc_core::ecma::ast::AssignPatProp {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        key: #key_code,
                        value: None,
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
