//! Binding initialization code generation.

use crate::template::{BindingSpec, TypePlaceholder};
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

/// Generates binding initialization code for template substitution.
///
/// Uses type-specific conversion:
/// - For Expr bindings: uses `to_ts_expr()` which handles Box<Expr>, Ident, String, etc.
/// - For Ident bindings: uses `.into()` which handles String -> Ident via SWC's From impls
///
/// This prevents type mismatch errors when the same placeholder appears in multiple contexts,
/// and correctly handles Box<Expr> which doesn't have a From impl for Expr.
pub fn generate_binding_initializations(
    bindings: &[BindingSpec],
    type_placeholders: &[TypePlaceholder],
) -> TokenStream2 {
    let mut output = TokenStream2::new();

    for binding in bindings {
        let name = &binding.name;
        let ty = &binding.ty;
        let expr = &binding.expr;
        let ty_str = ty.to_string();
        if ty_str == "Expr" {
            output.extend(quote! {
                let #name: #ty = macroforge_ts_syn::to_ts_expr(#expr);
            });
        } else {
            output.extend(quote! {
                let #name: #ty = (#expr).into();
            });
        }
    }

    // Generate type placeholder initializations
    // Use ToTsTypeName trait to convert to string, which handles Ident.sym correctly
    // and avoids including SyntaxContext markers like #0
    for tp in type_placeholders {
        let field_name = format_ident!("__mf_type_{}", tp.id);
        let expr = &tp.expr;
        output.extend(quote! {
            let #field_name: String = macroforge_ts_syn::ToTsTypeName::to_ts_type_name(&#expr);
        });
    }

    output
}

/// Generates visitor struct fields and initialization expressions.
pub fn generate_visitor_components(
    bindings: &[BindingSpec],
    type_placeholders: &[TypePlaceholder],
) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let mut fields = Vec::new();
    let mut inits = Vec::new();

    // Ident substitution fields
    for binding in bindings {
        let name = &binding.name;
        let field_name = format_ident!("binding_{}", name);
        let ty = &binding.ty;
        fields.push(quote! { #field_name: #ty });
        inits.push(quote! { #field_name: #name.clone() });
    }

    // Type substitution fields
    for tp in type_placeholders {
        let field_name = format_ident!("__mf_type_{}", tp.id);
        fields.push(quote! { #field_name: String });
        inits.push(quote! { #field_name: #field_name.clone() });
    }

    (fields, inits)
}
