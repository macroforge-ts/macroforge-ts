#![cfg(feature = "oxc")]
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr, Ident};

/// A builder for constructing Oxc AST nodes.
/// Currently just a simple string-based generator for Oxc structs.
pub(crate) struct Builder {
    type_name: String,
    fields: Vec<(String, Expr)>,
}

impl Builder {
    pub fn new(ident: &str) -> Self {
        Self {
            type_name: ident.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &str, value: Expr) {
        self.fields.push((name.to_string(), value));
    }

    pub fn build(self) -> Expr {
        let type_name = Ident::new(&self.type_name, proc_macro2::Span::call_site());
        let fields = self.fields.iter().map(|(name, value)| {
            let name = Ident::new(name, proc_macro2::Span::call_site());
            quote! { #name: #value }
        });

        syn::parse_quote! {
            oxc_ast::ast::#type_name {
                #(#fields),*
            }
        }
    }
}
