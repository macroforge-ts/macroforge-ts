use crate::template::{ControlNode, IdGen, Segment, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::parse_segments;

/// Parses an `{#if ...}` chain into a control node.
pub fn parse_if_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    cond: TokenStream2,
    span: Span,
    ids: &mut IdGen,
) -> syn::Result<ControlNode> {
    let (then_branch, terminator) = parse_segments(
        iter,
        Some(&[
            Terminator::Else,
            Terminator::ElseIf(TokenStream2::new()),
            Terminator::EndIf,
        ]),
        ids,
        false,
    )?;

    match terminator {
        Some(Terminator::Else) => {
            let (else_branch, terminator) =
                parse_segments(iter, Some(&[Terminator::EndIf]), ids, false)?;
            if !matches!(terminator, Some(Terminator::EndIf)) {
                return Err(template_error(
                    span,
                    "Unclosed {#if} block: Missing {/if}",
                    Some("{#if condition}...{:else}...{/if}"),
                ));
            }
            Ok(ControlNode::If {
                cond,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::ElseIf(next_cond)) => {
            let else_branch = vec![Segment::Control {
                id: ids.next(),
                node: parse_if_chain(iter, next_cond, span, ids)?,
            }];
            Ok(ControlNode::If {
                cond,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::EndIf) => Ok(ControlNode::If {
            cond,
            then_branch,
            else_branch: None,
        }),
        _ => Err(template_error(
            span,
            "Unclosed {#if} block: Missing {/if}",
            Some("{#if condition}...{/if}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_if_test(cond: proc_macro2::TokenStream, body: proc_macro2::TokenStream) -> syn::Result<ControlNode> {
        let mut iter = body.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        parse_if_chain(&mut iter, cond, proc_macro2::Span::call_site(), &mut ids)
    }

    #[test]
    fn test_basic_if() {
        let result = parse_if_test(quote! { x > 0 }, quote! { yes {/if} });
        assert!(result.is_ok());
        if let ControlNode::If { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_none());
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_else() {
        let result = parse_if_test(quote! { x > 0 }, quote! { yes {:else} no {/if} });
        assert!(result.is_ok());
        if let ControlNode::If { then_branch, else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
            assert!(!then_branch.is_empty());
            assert!(!else_branch.unwrap().is_empty());
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_else_if() {
        let result = parse_if_test(
            quote! { x > 0 },
            quote! { positive {:else if x < 0} negative {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::If { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
            let else_seg = &else_branch.unwrap()[0];
            assert!(matches!(else_seg, crate::template::Segment::Control { .. }));
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_else_if_else() {
        let result = parse_if_test(
            quote! { x > 0 },
            quote! { positive {:else if x < 0} negative {:else} zero {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::If { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_multiple_else_if() {
        let result = parse_if_test(
            quote! { x > 10 },
            quote! { large {:else if x > 5} medium {:else if x > 0} small {:else} zero {/if} },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_empty_then() {
        let result = parse_if_test(quote! { x }, quote! { {/if} });
        assert!(result.is_ok());
        if let ControlNode::If { then_branch, .. } = result.unwrap() {
            assert_eq!(then_branch.len(), 0);
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_empty_else() {
        let result = parse_if_test(quote! { x }, quote! { content {:else} {/if} });
        assert!(result.is_ok());
        if let ControlNode::If { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
            assert_eq!(else_branch.unwrap().len(), 0);
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_unclosed() {
        let result = parse_if_test(quote! { x }, quote! { content });
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_if_unclosed_after_else() {
        let result = parse_if_test(quote! { x }, quote! { then {:else} else_content });
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_if_preserves_condition() {
        let condition = quote! { user.is_admin() && user.active };
        let result = parse_if_test(condition.clone(), quote! { admin {/if} });
        assert!(result.is_ok());
        if let ControlNode::If { cond, .. } = result.unwrap() {
            assert_eq!(cond.to_string(), condition.to_string());
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_with_interpolations() {
        let result = parse_if_test(quote! { enabled }, quote! { Value: @{x} {/if} });
        assert!(result.is_ok());
        if let ControlNode::If { then_branch, .. } = result.unwrap() {
            assert!(!then_branch.is_empty());
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_if_nested_control_flow() {
        // Test nested if inside outer if
        let tokens: proc_macro2::TokenStream = "{#if inner} nested {/if} {/if}".parse().unwrap();
        let mut iter = tokens.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        let result = parse_if_chain(&mut iter, quote! { outer }, proc_macro2::Span::call_site(), &mut ids);
        assert!(result.is_ok());
        if let ControlNode::If { then_branch, .. } = result.unwrap() {
            assert!(!then_branch.is_empty());
        } else {
            panic!("Expected If node");
        }
    }
}
