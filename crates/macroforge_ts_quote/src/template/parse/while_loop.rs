use crate::template::{ControlNode, IdGen, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses a `{#while ...}` block.
pub fn parse_while_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (body, terminator) = parse_segments(iter, Some(&[Terminator::EndWhile]), ids, false)?;
    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            span,
            "Unclosed {#while} block: Missing {/while}",
            Some("{#while condition}...{/while}"),
        ));
    }
    Ok(ControlNode::While { cond, body })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_while_test(tokens: proc_macro2::TokenStream) -> syn::Result<ControlNode> {
        let mut iter = tokens.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        parse_while_loop(&mut iter, quote! { true }, proc_macro2::Span::call_site(), &mut ids)
    }

    #[test]
    fn test_basic_while() {
        let tokens = quote! { static content {/while} };
        let result = parse_while_test(tokens);
        assert!(result.is_ok());
        let node = result.unwrap();
        assert!(matches!(node, ControlNode::While { .. }));
    }

    #[test]
    fn test_while_empty_body() {
        let tokens = quote! { {/while} };
        let result = parse_while_test(tokens);
        assert!(result.is_ok());
        if let ControlNode::While { body, .. } = result.unwrap() {
            assert_eq!(body.len(), 0);
        } else {
            panic!("Expected While node");
        }
    }

    #[test]
    fn test_while_with_interpolation() {
        let tokens = quote! { @{value} {/while} };
        let result = parse_while_test(tokens);
        assert!(result.is_ok());
        if let ControlNode::While { body, .. } = result.unwrap() {
            assert!(!body.is_empty());
        } else {
            panic!("Expected While node");
        }
    }

    #[test]
    fn test_while_unclosed() {
        let tokens = quote! { static content };
        let result = parse_while_test(tokens);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_while_multiple_lines() {
        let tokens = quote! {
            line1
            line2
            line3
            {/while}
        };
        let result = parse_while_test(tokens);
        assert!(result.is_ok());
        if let ControlNode::While { body, .. } = result.unwrap() {
            assert!(!body.is_empty());
        } else {
            panic!("Expected While node");
        }
    }

    #[test]
    fn test_while_with_complex_condition() {
        let mut iter = quote! { content {/while} }.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        let result = parse_while_loop(
            &mut iter,
            quote! { counter < 10 && !done },
            proc_macro2::Span::call_site(),
            &mut ids
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_preserves_condition() {
        let condition = quote! { x > 0 };
        let mut iter = quote! { body {/while} }.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        let result = parse_while_loop(
            &mut iter,
            condition.clone(),
            proc_macro2::Span::call_site(),
            &mut ids
        );
        assert!(result.is_ok());
        if let ControlNode::While { cond, .. } = result.unwrap() {
            assert_eq!(cond.to_string(), condition.to_string());
        } else {
            panic!("Expected While node");
        }
    }
}
