use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use crate::template::IdentPart;

/// Builds an SWC identifier from an ident block's parts.
pub fn compile_ident_block(parts: &[IdentPart]) -> TokenStream2 {
    let mut stmts = TokenStream2::new();
    let mut part_index = 0usize;

    for part in parts {
        match part {
            IdentPart::Static(s) => {
                stmts.extend(quote! {
                    __mf_name.push_str(#s);
                });
            }
            IdentPart::Interpolation { expr, .. } => {
                let var =
                    proc_macro2::Ident::new(&format!("__mf_part_{part_index}"), Span::call_site());
                part_index += 1;
                stmts.extend(quote! {
                    let #var: swc_core::ecma::ast::Ident = (#expr).clone().into();
                    __mf_name.push_str(&#var.sym.to_string());
                });
            }
        }
    }

    quote! {
        {
            let mut __mf_name = String::new();
            #stmts
            swc_core::ecma::ast::Ident::new(
                __mf_name.into(),
                swc_core::common::DUMMY_SP,
                swc_core::common::SyntaxContext::empty()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_compile_ident_block_empty() {
        let parts: Vec<IdentPart> = vec![];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("let mut __mf_name = String :: new ()"));
        assert!(result_str.contains("swc_core :: ecma :: ast :: Ident :: new"));
    }

    #[test]
    fn test_compile_ident_block_static_only() {
        let parts = vec![IdentPart::Static("myIdent".to_string())];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("__mf_name . push_str (\"myIdent\")"));
    }

    #[test]
    fn test_compile_ident_block_multiple_static() {
        let parts = vec![
            IdentPart::Static("get".to_string()),
            IdentPart::Static("User".to_string()),
            IdentPart::Static("Name".to_string()),
        ];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("__mf_name . push_str (\"get\")"));
        assert!(result_str.contains("__mf_name . push_str (\"User\")"));
        assert!(result_str.contains("__mf_name . push_str (\"Name\")"));
    }

    #[test]
    fn test_compile_ident_block_interpolation_only() {
        let parts = vec![IdentPart::Interpolation {
            expr: quote! { field_name },
        }];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("let __mf_part_0"));
        assert!(result_str.contains("swc_core :: ecma :: ast :: Ident"));
        assert!(result_str.contains("(field_name) . clone () . into ()"));
        assert!(result_str.contains("__mf_name . push_str (& __mf_part_0 . sym . to_string ())"));
    }

    #[test]
    fn test_compile_ident_block_mixed_parts() {
        let parts = vec![
            IdentPart::Static("get".to_string()),
            IdentPart::Interpolation {
                expr: quote! { field_name },
            },
            IdentPart::Static("Value".to_string()),
        ];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("__mf_name . push_str (\"get\")"));
        assert!(result_str.contains("let __mf_part_0"));
        assert!(result_str.contains("__mf_name . push_str (\"Value\")"));
    }

    #[test]
    fn test_compile_ident_block_multiple_interpolations() {
        let parts = vec![
            IdentPart::Interpolation {
                expr: quote! { prefix },
            },
            IdentPart::Static("_".to_string()),
            IdentPart::Interpolation {
                expr: quote! { suffix },
            },
        ];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("let __mf_part_0"));
        assert!(result_str.contains("let __mf_part_1"));
        assert!(result_str.contains("__mf_name . push_str (\"_\")"));
    }

    #[test]
    fn test_compile_ident_block_complex_expr() {
        let parts = vec![
            IdentPart::Static("handle".to_string()),
            IdentPart::Interpolation {
                expr: quote! { event_type.to_string().to_uppercase() },
            },
        ];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("__mf_name . push_str (\"handle\")"));
        assert!(result_str.contains("event_type . to_string () . to_uppercase ()"));
    }

    #[test]
    fn test_compile_ident_block_returns_swc_ident() {
        let parts = vec![IdentPart::Static("test".to_string())];
        let result = compile_ident_block(&parts);

        let result_str = result.to_string();
        assert!(result_str.contains("swc_core :: ecma :: ast :: Ident :: new"));
        assert!(result_str.contains("swc_core :: common :: DUMMY_SP"));
        assert!(result_str.contains("swc_core :: common :: SyntaxContext :: empty ()"));
    }
}
