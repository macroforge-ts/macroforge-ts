use super::*;

impl Codegen {
    pub(super) fn generate_props(&self, props: &[IrNode]) -> TokenStream {
    let props_code: Vec<TokenStream> = props.iter().filter_map(|p| self.generate_prop(p)).collect();
    quote! { vec![#(#props_code),*] }
}

pub(super) fn generate_prop(&self, node: &IrNode) -> Option<TokenStream> {
    match node {
        IrNode::KeyValueProp { key, value } => {
            let key_code = self.generate_prop_name(key);
            let value_code = self.generate_expr(value);
            Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::KeyValue(
                        macroforge_ts::swc_core::ecma::ast::KeyValueProp {
                            key: #key_code,
                            value: Box::new(#value_code),
                        }
                    )
                ))
            })
        }
        IrNode::ShorthandProp { key } => {
            let key_code = self.generate_ident(key);
            Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Prop(Box::new(
                    macroforge_ts::swc_core::ecma::ast::Prop::Shorthand(#key_code)
                ))
            })
        }
        IrNode::SpreadElement { expr } => {
            let expr_code = self.generate_expr(expr);
            Some(quote! {
                macroforge_ts::swc_core::ecma::ast::PropOrSpread::Spread(
                    macroforge_ts::swc_core::ecma::ast::SpreadElement {
                        dot3_token: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            })
        }
        // For control flow in object literals
        IrNode::If { .. } | IrNode::For { .. } => None, // Handled at higher level
        _ => None,
    }
}

pub(super) fn generate_prop_name(&self, node: &IrNode) -> TokenStream {
    match node {
        IrNode::Ident(name) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        #name.into(),
                        macroforge_ts::swc_core::common::DUMMY_SP,
                    )
                )
            }
        }
        IrNode::StrLit(value) => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Str(
                    macroforge_ts::swc_core::ecma::ast::Str {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        value: #value.into(),
                        raw: None,
                    }
                )
            }
        }
        IrNode::ComputedPropName { expr } => {
            let expr_code = self.generate_expr(expr);
            quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Computed(
                    macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                        span: macroforge_ts::swc_core::common::DUMMY_SP,
                        expr: Box::new(#expr_code),
                    }
                )
            }
        }
        IrNode::Placeholder {
            kind: PlaceholderKind::Ident,
            expr,
        } => {
            quote! {
                macroforge_ts::swc_core::ecma::ast::PropName::Ident({
                    let __ident = macroforge_ts::ts_syn::ToTsIdent::to_ts_ident((#expr).clone());
                    macroforge_ts::swc_core::ecma::ast::IdentName::new(
                        __ident.sym,
                        __ident.span,
                    )
                })
            }
        }
        _ => quote! {
            macroforge_ts::swc_core::ecma::ast::PropName::Ident(
                macroforge_ts::swc_core::ecma::ast::IdentName::new(
                    "".into(),
                    macroforge_ts::swc_core::common::DUMMY_SP,
                )
            )
        },
    }
}
}
