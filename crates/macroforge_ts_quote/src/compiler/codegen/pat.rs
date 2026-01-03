use super::error::{GenError, GenResult};
use super::*;

impl Codegen {
    pub(super) fn generate_pat(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::BindingIdent {
            name,
            type_ann,
            optional: _,
            ..
        } => {
            let name_code = self.generate_ident(name)?;
            let type_ann_code = match type_ann.as_ref() {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: #name_code,
                        type_ann: #type_ann_code,
                    }
                )
            })
        }
        IrNode::Ident { value: name, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                    macroforge_ts::swc_core::ecma::ast::BindingIdent {
                        id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                            #name.into(),
                            macroforge_ts::swc_core::common::DUMMY_SP,
                        ),
                        type_ann: None,
                    }
                )
            })
        }
        IrNode::Placeholder { kind, expr, .. } => {
            match kind {
                PlaceholderKind::Ident => {
                    Ok(quote! {
                        macroforge_ts::swc_core::ecma::ast::Pat::Ident(
                            macroforge_ts::swc_core::ecma::ast::BindingIdent {
                                id: macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()),
                                type_ann: None,
                            }
                        )
                    })
                }
                PlaceholderKind::Expr => {
                    // Expressions can be patterns too (like member expressions for assignment)
                    Ok(quote! {
                        macroforge_ts::swc_core::ecma::ast::Pat::Expr(
                            Box::new(macroforge_ts::ts_syn::ToTsExpr::to_ts_expr((#expr).clone()))
                        )
                    })
                }
                _ => {
                    Err(GenError::invalid_placeholder(
                        "pattern",
                        &format!("{:?}", kind),
                        &["Ident", "Expr"],
                    ))
                }
            }
        }
        IrNode::IdentBlock { parts, .. } => {
            // Build identifier string from parts at runtime
            let part_exprs: Vec<TokenStream> = parts
                    .iter()
                    .filter_map(|p| match p {
                        IrNode::StrLit { value: text, .. } => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Ident { value: text, .. } => Some(quote! { __ident.push_str(#text); }),
                        IrNode::Placeholder { expr, .. } => {
                            Some(quote! {
                                __ident.push_str(&macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone()).sym.to_string());
                            })
                        }
                        _ => None,
                    })
                    .collect();

            Ok(quote! {
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
            })
        }
        IrNode::ArrayPat { elems, type_ann, optional, .. } => {
            let elems_code: Vec<TokenStream> = elems.iter().map(|opt_elem| {
                match opt_elem {
                    Some(elem) => {
                        let elem_code = self.generate_pat(elem)?;
                        Ok(quote! { Some(#elem_code) })
                    }
                    None => Ok(quote! { None }),
                }
            }).collect::<GenResult<Vec<_>>>()?;
            let type_ann_code = match type_ann.as_ref() {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Array(
                    macroforge_ts::swc_core::ecma::ast::ArrayPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        elems: vec![#(#elems_code),*],
                        optional: #optional,
                        type_ann: #type_ann_code,
                    }
                )
            })
        }
        IrNode::ObjectPat { props, type_ann, optional, .. } => {
            let props_code: Vec<TokenStream> = props.iter().map(|prop| {
                self.generate_object_pat_prop(prop)
            }).collect::<GenResult<Vec<_>>>()?;
            let type_ann_code = match type_ann.as_ref() {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Object(
                    macroforge_ts::swc_core::ecma::ast::ObjectPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        props: vec![#(#props_code),*],
                        optional: #optional,
                        type_ann: #type_ann_code,
                    }
                )
            })
        }
        IrNode::RestPat { arg, type_ann, .. } => {
            let arg_code = self.generate_pat(arg)?;
            let type_ann_code = match type_ann.as_ref() {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Rest(
                    macroforge_ts::swc_core::ecma::ast::RestPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                        type_ann: #type_ann_code,
                    }
                )
            })
        }
        IrNode::AssignPat { left, right, .. } => {
            let left_code = self.generate_pat(left)?;
            let right_code = self.generate_expr(right)?;
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::Pat::Assign(
                    macroforge_ts::swc_core::ecma::ast::AssignPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        left: Box::new(#left_code),
                        right: Box::new(#right_code),
                    }
                )
            })
        }
        _ => {
            Err(GenError::unexpected_node(
                "pattern",
                node,
                &["BindingIdent", "Ident", "Placeholder", "IdentBlock", "ArrayPat", "ObjectPat", "RestPat", "AssignPat"],
            ))
        }
    }
}

/// Generate code for an object pattern property.
pub(super) fn generate_object_pat_prop(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::ObjectPatProp { key, value, .. } => {
            let key_code = self.generate_ident(key)?;
            match value {
                Some(val) => {
                    let val_code = self.generate_pat(val)?;
                    Ok(quote! {
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
                    })
                }
                None => {
                    // Shorthand: { a } is the same as { a: a }
                    Ok(quote! {
                        macroforge_ts::swc_core::ecma::ast::ObjectPatProp::Assign(
                            macroforge_ts::swc_core::ecma::ast::AssignPatProp {
                                span: macroforge_ts::swc_core::common::DUMMY_SP,
                                key: #key_code,
                                value: None,
                            }
                        )
                    })
                }
            }
        }
        IrNode::RestPat { arg, type_ann, .. } => {
            let arg_code = self.generate_pat(arg)?;
            let type_ann_code = match type_ann.as_ref() {
                Some(t) => {
                    let tc = self.generate_type_ann(t)?;
                    quote! { Some(Box::new(#tc)) }
                }
                None => quote! { None },
            };
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::ObjectPatProp::Rest(
                    macroforge_ts::swc_core::ecma::ast::RestPat {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        arg: Box::new(#arg_code),
                        type_ann: #type_ann_code,
                    }
                )
            })
        }
        _ => {
            Err(GenError::unexpected_node(
                "object pattern property",
                node,
                &["ObjectPatProp", "RestPat"],
            ))
        }
    }
}

pub(super) fn generate_pats(&self, nodes: &[IrNode]) -> GenResult<TokenStream> {
    let pats_code: Vec<TokenStream> = nodes.iter().map(|p| self.generate_pat(p)).collect::<GenResult<Vec<_>>>()?;
    Ok(quote! { vec![#(#pats_code),*] })
}
}
