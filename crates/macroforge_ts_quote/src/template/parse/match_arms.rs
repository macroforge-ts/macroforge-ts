use crate::template::{ControlNode, IdGen, MatchCase, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses the arms for a `{#match ...}` block.
pub fn parse_match_arms(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    expr: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let mut cases = Vec::new();
    let mut pending_pattern: Option<TokenStream2> = None;

    loop {
        let (body, terminator) = parse_segments(
            iter,
            Some(&[Terminator::Case(TokenStream2::new()), Terminator::EndMatch]),
            ids,
            false,
        )?;

        match terminator {
            Some(Terminator::Case(pattern)) => {
                if let Some(prev) = pending_pattern.take() {
                    cases.push(MatchCase {
                        pattern: prev,
                        body,
                    });
                } else if !body.is_empty() {
                    return Err(template_error(
                        span,
                        "Unexpected content before {:case}",
                        None,
                    ));
                }
                pending_pattern = Some(pattern);
            }
            Some(Terminator::EndMatch) => {
                if let Some(prev) = pending_pattern.take() {
                    cases.push(MatchCase {
                        pattern: prev,
                        body,
                    });
                } else if !body.is_empty() {
                    return Err(template_error(
                        span,
                        "Unexpected content before {/match}",
                        None,
                    ));
                }
                break;
            }
            None => {
                return Err(template_error(
                    span,
                    "Unclosed {#match} block: Missing {/match}",
                    Some("{#match expr}{:case pattern}...{/match}"),
                ));
            }
            Some(other) => {
                return Err(template_error(
                    span,
                    &format!("Unexpected terminator in {{#match}}: {other:?}"),
                    None,
                ));
            }
        }
    }

    Ok(ControlNode::Match { expr, cases })
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_match_test(
        expr: proc_macro2::TokenStream,
        body: proc_macro2::TokenStream,
    ) -> syn::Result<ControlNode> {
        let mut iter = body.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        parse_match_arms(&mut iter, expr, proc_macro2::Span::call_site(), &mut ids)
    }

    #[test]
    fn test_single_case() {
        let result = parse_match_test(
            quote! { value },
            quote! { {:case Some(x)} @{x} {/match} },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 1);
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_multiple_cases() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Some(x)} has @{x}
                {:case None} none
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 2);
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_many_cases() {
        let result = parse_match_test(
            quote! { status },
            quote! {
                {:case Status::Active} active
                {:case Status::Pending} pending
                {:case Status::Inactive} inactive
                {:case _} unknown
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 4);
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_empty_case_body() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Some(x)}
                {:case None}
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 2);
            assert_eq!(cases[0].body.len(), 0);
            assert_eq!(cases[1].body.len(), 0);
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_case_with_interpolation() {
        let result = parse_match_test(
            quote! { result },
            quote! {
                {:case Ok(val)} success: @{val}
                {:case Err(e)} error: @{e}
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 2);
            assert!(!cases[0].body.is_empty());
            assert!(!cases[1].body.is_empty());
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_unclosed_match() {
        let result = parse_match_test(
            quote! { value },
            quote! { {:case Some(x)} body },
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_content_before_first_case() {
        let result = parse_match_test(
            quote! { value },
            quote! { unexpected content {:case Some(x)} body {/match} },
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unexpected content"));
    }

    #[test]
    fn test_preserves_expr() {
        let expr = quote! { complex_expr.method(arg) };
        let result = parse_match_test(
            expr.clone(),
            quote! { {:case x} x {/match} },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { expr: e, .. } = result.unwrap() {
            assert_eq!(e.to_string(), expr.to_string());
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_preserves_patterns() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Some((x, y))} tuple
                {:case None} none
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 2);
            assert_eq!(cases[0].pattern.to_string(), "Some ((x , y))");
            assert_eq!(cases[1].pattern.to_string(), "None");
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_wildcard_pattern() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Some(x)} some
                {:case _} default
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 2);
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_or_pattern() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Some(1) | Some(2)} one_or_two
                {:case _} other
                {/match}
            },
        );
        assert!(result.is_ok());
        if let ControlNode::Match { cases, .. } = result.unwrap() {
            assert_eq!(cases.len(), 2);
        } else {
            panic!("Expected Match node");
        }
    }

    #[test]
    fn test_complex_nested_pattern() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Ok(User { name, settings: Settings { theme } })} @{name} uses @{theme}
                {:case Err(_)} error
                {/match}
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_case_with_guard() {
        let result = parse_match_test(
            quote! { value },
            quote! {
                {:case Some(x) if x > 0} positive
                {:case _} other
                {/match}
            },
        );
        assert!(result.is_ok());
    }
}
