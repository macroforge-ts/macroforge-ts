use crate::template::{ControlNode, IdGen, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses a `{#while let ...}` block.
pub fn parse_while_let_loop(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (body, terminator) = parse_segments(iter, Some(&[Terminator::EndWhile]), ids, false)?;
    if !matches!(terminator, Some(Terminator::EndWhile)) {
        return Err(template_error(
            span,
            "Unclosed {#while let} block: Missing {/while}",
            Some("{#while let pattern = expr}...{/while}"),
        ));
    }
    Ok(ControlNode::WhileLet {
        pattern,
        expr,
        body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_while_let_test(
        pattern: proc_macro2::TokenStream,
        expr: proc_macro2::TokenStream,
        body: proc_macro2::TokenStream,
    ) -> syn::Result<ControlNode> {
        let mut iter = body.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        parse_while_let_loop(&mut iter, pattern, expr, proc_macro2::Span::call_site(), &mut ids)
    }

    #[test]
    fn test_basic_while_let() {
        let result = parse_while_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { @{x} {/while} },
        );
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), ControlNode::WhileLet { .. }));
    }

    #[test]
    fn test_while_let_empty_body() {
        let result = parse_while_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { {/while} },
        );
        assert!(result.is_ok());
        if let ControlNode::WhileLet { body, .. } = result.unwrap() {
            assert_eq!(body.len(), 0);
        } else {
            panic!("Expected WhileLet node");
        }
    }

    #[test]
    fn test_while_let_unclosed() {
        let result = parse_while_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { content },
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_while_let_preserves_pattern() {
        let pattern = quote! { Some((x, y)) };
        let result = parse_while_let_test(
            pattern.clone(),
            quote! { pair_opt },
            quote! { body {/while} },
        );
        assert!(result.is_ok());
        if let ControlNode::WhileLet { pattern: p, .. } = result.unwrap() {
            assert_eq!(p.to_string(), pattern.to_string());
        } else {
            panic!("Expected WhileLet node");
        }
    }

    #[test]
    fn test_while_let_preserves_expr() {
        let expr = quote! { iterator.next() };
        let result = parse_while_let_test(
            quote! { Some(item) },
            expr.clone(),
            quote! { body {/while} },
        );
        assert!(result.is_ok());
        if let ControlNode::WhileLet { expr: e, .. } = result.unwrap() {
            assert_eq!(e.to_string(), expr.to_string());
        } else {
            panic!("Expected WhileLet node");
        }
    }

    #[test]
    fn test_while_let_complex_pattern() {
        let result = parse_while_let_test(
            quote! { Ok(User { name, age }) },
            quote! { results.next() },
            quote! { @{name} is @{age} {/while} },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_while_let_with_static_content() {
        let result = parse_while_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { static text here {/while} },
        );
        assert!(result.is_ok());
        if let ControlNode::WhileLet { body, .. } = result.unwrap() {
            assert!(!body.is_empty());
        } else {
            panic!("Expected WhileLet node");
        }
    }

    #[test]
    fn test_while_let_with_interpolations() {
        let result = parse_while_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { value: @{x}, more: @{y} {/while} },
        );
        assert!(result.is_ok());
        if let ControlNode::WhileLet { body, .. } = result.unwrap() {
            assert!(!body.is_empty());
        } else {
            panic!("Expected WhileLet node");
        }
    }
}
