use crate::template::{BindingSpec, QuoteTsResult};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

/// Builds a `swc_core::quote!` invocation with placeholder bindings.
/// NOTE: This does NOT support TypeScript type annotations.
pub fn quote_ts(template: &str, output_type: TokenStream2, bindings: &[BindingSpec]) -> QuoteTsResult {
    let mut final_template = template.to_string();
    if output_type.to_string().contains("Expr") {
        let trimmed = final_template.trim();
        if trimmed.starts_with('{') && trimmed.ends_with('}') {
            final_template = format!("({})", trimmed);
        }
    }

    let lit = syn::LitStr::new(&final_template, Span::call_site());
    let mut binding_inits = TokenStream2::new();
    binding_inits.extend(quote! { use swc_core::ecma::ast::*; });

    // We use type-specific conversion to handle various input types:
    // - For Expr bindings: use `to_ts_expr()` which handles Box<Expr>, Ident, String, etc.
    // - For Ident bindings: use `.into()` which handles String -> Ident via SWC's From impls
    // This prevents type mismatch errors when the same placeholder appears in multiple contexts,
    // and correctly handles Box<Expr> which doesn't have a From impl for Expr.
    let mut quote_bindings = Vec::new();
    for binding in bindings {
        let name = &binding.name;
        let ty = &binding.ty;
        let expr = &binding.expr;
        let ty_str = ty.to_string();
        if ty_str == "Expr" {
            // Use to_ts_expr() which handles Box<Expr>, Ident, String, etc.
            binding_inits.extend(quote! { let #name: #ty = macroforge_ts_syn::to_ts_expr(#expr); });
        } else {
            // Use .into() for Ident and other types
            binding_inits.extend(quote! { let #name: #ty = (#expr).into(); });
        }
        quote_bindings.push(quote! { #name: #ty = #name.clone() });
    }

    let quote_call = if quote_bindings.is_empty() {
        quote! { swc_core::quote!(#lit as #output_type) }
    } else {
        quote! { swc_core::quote!(#lit as #output_type, #(#quote_bindings),*) }
    };

    QuoteTsResult {
        bindings: binding_inits,
        expr: quote_call,
    }
}
