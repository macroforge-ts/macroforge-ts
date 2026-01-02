use super::error::GenResult;
use super::*;

impl Codegen {
    pub(super) fn generate_assign_target(&self, node: &IrNode) -> GenResult<TokenStream> {
    match node {
        IrNode::Ident { value: name, .. } => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::AssignTarget::Simple(
                    macroforge_ts::swc_core::ecma::ast::SimpleAssignTarget::Ident(
                        macroforge_ts::swc_core::ecma::ast::BindingIdent {
                            id: macroforge_ts::swc_core::ecma::ast::Ident::new_no_ctxt(
                                #name.into(),
                                macroforge_ts::swc_core::common::DUMMY_SP,
                            ),
                            type_ann: None,
                        }
                    )
                )
            })
        }
        IrNode::MemberExpr {
            obj,
            prop,
            computed,
            ..
        } => {
            let obj_code = self.generate_expr(obj)?;
            let prop_code = if *computed {
                let p = self.generate_expr(prop)?;
                quote! {
                    macroforge_ts::swc_core::ecma::ast::MemberProp::Computed(
                        macroforge_ts::swc_core::ecma::ast::ComputedPropName {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            expr: Box::new(#p),
                        }
                    )
                }
            } else {
                let ident_code = self.generate_ident_name(prop)?;
                quote! {
                    macroforge_ts::swc_core::ecma::ast::MemberProp::Ident(#ident_code)
                }
            };

            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::AssignTarget::Simple(
                    macroforge_ts::swc_core::ecma::ast::SimpleAssignTarget::Member(
                        macroforge_ts::swc_core::ecma::ast::MemberExpr {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                            obj: Box::new(#obj_code),
                            prop: #prop_code,
                        }
                    )
                )
            })
        }
        _ => {
            Ok(quote! {
                macroforge_ts::swc_core::ecma::ast::AssignTarget::Simple(
                    macroforge_ts::swc_core::ecma::ast::SimpleAssignTarget::Invalid(
                        macroforge_ts::swc_core::ecma::ast::Invalid {
                            span: macroforge_ts::swc_core::common::DUMMY_SP,
                        }
                    )
                )
            })
        }
    }
}
}
