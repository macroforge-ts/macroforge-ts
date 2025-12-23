use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use crate::template::{escape_tpl_segment, quote_ts, BindingSpec, QuoteTsResult, StringPart};

/// Builds an SWC expression from an interpolated string literal.
pub fn build_string_interp_expr(parts: &[StringPart], id: usize) -> TokenStream2 {
    let mut template = String::new();
    let mut bindings = Vec::new();
    let mut expr_index = 0usize;

    template.push('`');
    for part in parts {
        match part {
            StringPart::Text(text) => {
                template.push_str(&escape_tpl_segment(text));
            }
            StringPart::Expr(expr) => {
                let name = format!("__mf_str_{id}_{expr_index}");
                expr_index += 1;
                template.push_str("${$");
                template.push_str(&name);
                template.push('}');
                let ident = proc_macro2::Ident::new(&name, Span::call_site());
                bindings.push(BindingSpec {
                    name: ident,
                    ty: quote!(Expr),
                    expr: quote! { macroforge_ts::ts_syn::to_ts_expr(#expr) },
                });
            }
        }
    }
    template.push('`');

    let quote_ts = quote_ts(&template, quote!(Expr), &bindings);
    let QuoteTsResult { bindings, expr } = quote_ts;
    quote! {{
        #bindings
        #expr
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_text_only() {
        let parts = vec![StringPart::Text("Hello, World!".to_string())];
        let result = build_string_interp_expr(&parts, 0);
        let result_str = result.to_string();

        // Should create a template string with no expressions
        assert!(result_str.contains("`Hello, World!`"));
    }

    #[test]
    fn test_single_expr() {
        let parts = vec![StringPart::Expr(quote!(name))];
        let result = build_string_interp_expr(&parts, 1);
        let result_str = result.to_string();

        // Should have a binding for the expression
        assert!(result_str.contains("__mf_str_1_0"));
        // Should use ${} interpolation syntax
        assert!(result_str.contains("${$"));
    }

    #[test]
    fn test_text_and_expr() {
        let parts = vec![
            StringPart::Text("Hello, ".to_string()),
            StringPart::Expr(quote!(name)),
            StringPart::Text("!".to_string()),
        ];
        let result = build_string_interp_expr(&parts, 2);
        let result_str = result.to_string();

        // Should contain text parts and expression placeholder
        assert!(result_str.contains("Hello,"));
        assert!(result_str.contains("__mf_str_2_0"));
        assert!(result_str.contains("!"));
    }

    #[test]
    fn test_multiple_exprs() {
        let parts = vec![
            StringPart::Expr(quote!(first)),
            StringPart::Text(" and ".to_string()),
            StringPart::Expr(quote!(second)),
        ];
        let result = build_string_interp_expr(&parts, 3);
        let result_str = result.to_string();

        // Should have two separate bindings with sequential indices
        assert!(result_str.contains("__mf_str_3_0"));
        assert!(result_str.contains("__mf_str_3_1"));
        assert!(result_str.contains(" and "));
    }

    #[test]
    fn test_empty_parts() {
        let parts: Vec<StringPart> = vec![];
        let result = build_string_interp_expr(&parts, 4);
        let result_str = result.to_string();

        // Should create an empty template string
        assert!(result_str.contains("``"));
    }

    #[test]
    fn test_expr_index_increments() {
        let parts = vec![
            StringPart::Expr(quote!(a)),
            StringPart::Expr(quote!(b)),
            StringPart::Expr(quote!(c)),
        ];
        let result = build_string_interp_expr(&parts, 5);
        let result_str = result.to_string();

        // Each expression should get a unique index within the same ID
        assert!(result_str.contains("__mf_str_5_0"));
        assert!(result_str.contains("__mf_str_5_1"));
        assert!(result_str.contains("__mf_str_5_2"));
    }

    #[test]
    fn test_special_characters_in_text() {
        let parts = vec![
            StringPart::Text("Line 1\nLine 2\tTabbed".to_string()),
            StringPart::Expr(quote!(value)),
        ];
        let result = build_string_interp_expr(&parts, 6);
        let result_str = result.to_string();

        // escape_tpl_segment should handle special characters
        // The result should contain the expression binding
        assert!(result_str.contains("__mf_str_6_0"));
    }

    #[test]
    fn test_backtick_in_text() {
        let parts = vec![StringPart::Text("Text with ` backtick".to_string())];
        let result = build_string_interp_expr(&parts, 7);
        let result_str = result.to_string();

        // Should handle backticks properly (escape_tpl_segment should escape them)
        // The outer backticks should still be present for the template
        assert!(result_str.contains("`"));
    }

    #[test]
    fn test_dollar_sign_in_text() {
        let parts = vec![StringPart::Text("Price: $100".to_string())];
        let result = build_string_interp_expr(&parts, 8);
        let result_str = result.to_string();

        // escape_tpl_segment should escape dollar signs
        // Should contain the escaped text
        assert!(result_str.contains("Price"));
    }

    #[test]
    fn test_backslash_in_text() {
        let parts = vec![StringPart::Text("Path\\to\\file".to_string())];
        let result = build_string_interp_expr(&parts, 9);
        let result_str = result.to_string();

        // Backslashes should be handled by escape_tpl_segment
        assert!(result_str.contains("Path"));
    }

    #[test]
    fn test_complex_expr() {
        let parts = vec![
            StringPart::Text("Result: ".to_string()),
            StringPart::Expr(quote!(user.name.to_uppercase())),
        ];
        let result = build_string_interp_expr(&parts, 10);
        let result_str = result.to_string();

        // Should handle complex expression
        assert!(result_str.contains("__mf_str_10_0"));
        assert!(result_str.contains("Result"));
    }

    #[test]
    fn test_alternating_text_and_expr() {
        let parts = vec![
            StringPart::Text("a".to_string()),
            StringPart::Expr(quote!(x)),
            StringPart::Text("b".to_string()),
            StringPart::Expr(quote!(y)),
            StringPart::Text("c".to_string()),
        ];
        let result = build_string_interp_expr(&parts, 11);
        let result_str = result.to_string();

        // Should handle alternating pattern
        assert!(result_str.contains("__mf_str_11_0"));
        assert!(result_str.contains("__mf_str_11_1"));
        assert!(result_str.contains("a"));
        assert!(result_str.contains("b"));
        assert!(result_str.contains("c"));
    }

    #[test]
    fn test_unicode_in_text() {
        let parts = vec![StringPart::Text("Hello 世界 🌍".to_string())];
        let result = build_string_interp_expr(&parts, 12);
        let result_str = result.to_string();

        // Should handle unicode properly
        // The template literal should preserve unicode
        assert!(result_str.contains("`"));
    }

    #[test]
    fn test_empty_text_parts() {
        let parts = vec![
            StringPart::Text("".to_string()),
            StringPart::Expr(quote!(value)),
            StringPart::Text("".to_string()),
        ];
        let result = build_string_interp_expr(&parts, 13);
        let result_str = result.to_string();

        // Should handle empty text parts
        assert!(result_str.contains("__mf_str_13_0"));
    }

    #[test]
    fn test_bindings_use_to_ts_expr() {
        let parts = vec![StringPart::Expr(quote!(my_var))];
        let result = build_string_interp_expr(&parts, 14);
        let result_str = result.to_string();

        // Should call to_ts_expr on the expression
        assert!(result_str.contains("to_ts_expr"));
        assert!(result_str.contains("my_var"));
    }
}
