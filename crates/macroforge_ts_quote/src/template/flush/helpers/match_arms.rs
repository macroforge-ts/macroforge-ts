//! Match arm generation for visitor substitution.

use crate::template::{BindingSpec, TypePlaceholder};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// Generates ident match arms for Ident-typed bindings.
///
/// Template uses `$__mf_hole_N` format, which parses as an ident with `sym="$__mf_hole_N"`.
/// Only generates arms for bindings with `Ident` type.
pub fn generate_ident_arms(bindings: &[BindingSpec]) -> Vec<TokenStream2> {
    bindings
        .iter()
        .filter(|binding| binding.ty.to_string() == "Ident")
        .map(|binding| {
            let name = &binding.name;
            let placeholder = format!("${}", name);
            let field_name = format_ident!("binding_{}", name);
            quote! {
                #placeholder => {
                    ident.sym = self.#field_name.sym.clone();
                }
            }
        })
        .collect()
}

/// Generates expr match arms for Expr-typed bindings.
///
/// When a placeholder appears as an expression (e.g., function name), replaces the entire expr.
pub fn generate_expr_arms(bindings: &[BindingSpec]) -> Vec<TokenStream2> {
    bindings
        .iter()
        .filter(|binding| binding.ty.to_string() == "Expr")
        .map(|binding| {
            let name = &binding.name;
            let placeholder = format!("${}", name);
            let field_name = format_ident!("binding_{}", name);
            quote! {
                #placeholder => Some(self.#field_name.clone()),
            }
        })
        .collect()
}

/// Generates type match arms for type placeholders.
///
/// Uses types from `use swc_core::ecma::ast::*` and common imports.
pub fn generate_type_arms(
    type_placeholders: &[TypePlaceholder],
    swc_core_path: &TokenStream2,
) -> Vec<TokenStream2> {
    type_placeholders
        .iter()
        .map(|tp| {
            let marker = format!("__MfTypeMarker{}", tp.id);
            let field_name = format_ident!("__mf_type_{}", tp.id);
            quote! {
                #marker => {
                    *ty = TsType::TsTypeRef(TsTypeRef {
                        span: #swc_core_path::common::DUMMY_SP,
                        type_name: TsEntityName::Ident(Ident::new(
                            self.#field_name.clone().into(),
                            #swc_core_path::common::DUMMY_SP,
                            Default::default(),
                        )),
                        type_params: None,
                    });
                }
            }
        })
        .collect()
}
