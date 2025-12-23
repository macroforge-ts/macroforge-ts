use crate::template::{ControlNode, IdGen, Segment, Terminator, template_error};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::iter::Peekable;

use super::{parse_if_chain, parse_segments};

/// Parses an `{#if let ...}` chain into a control node.
pub fn parse_if_let_chain(
    iter: &mut Peekable<proc_macro2::token_stream::IntoIter>,
    pattern: TokenStream2,
    expr: TokenStream2,
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
                    "Unclosed {#if let} block: Missing {/if}",
                    Some("{#if let pattern = expr}...{:else}...{/if}"),
                ));
            }
            Ok(ControlNode::IfLet {
                pattern,
                expr,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::ElseIf(next_cond)) => {
            let else_branch = vec![Segment::Control {
                id: ids.next(),
                node: parse_if_chain(iter, next_cond, span, ids)?,
            }];
            Ok(ControlNode::IfLet {
                pattern,
                expr,
                then_branch,
                else_branch: Some(else_branch),
            })
        }
        Some(Terminator::EndIf) => Ok(ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch: None,
        }),
        _ => Err(template_error(
            span,
            "Unclosed {#if let} block: Missing {/if}",
            Some("{#if let pattern = expr}...{/if}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    fn parse_if_let_test(
        pattern: proc_macro2::TokenStream,
        expr: proc_macro2::TokenStream,
        body: proc_macro2::TokenStream,
    ) -> syn::Result<ControlNode> {
        let mut iter = body.into_iter().peekable();
        let mut ids = IdGen { next: 0 };
        parse_if_let_chain(&mut iter, pattern, expr, proc_macro2::Span::call_site(), &mut ids)
    }

    #[test]
    fn test_basic_if_let() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { @{x} {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_none());
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_else() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { some: @{x} {:else} none {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { then_branch, else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
            assert!(!then_branch.is_empty());
            assert!(!else_branch.unwrap().is_empty());
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_else_if() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { some {:else if other_cond} alt {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
            let else_seg = &else_branch.unwrap()[0];
            // Should contain a regular If node, not IfLet
            assert!(matches!(else_seg, crate::template::Segment::Control { .. }));
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_empty_then() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { then_branch, .. } = result.unwrap() {
            assert_eq!(then_branch.len(), 0);
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_empty_else() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { content {:else} {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { else_branch, .. } = result.unwrap() {
            assert!(else_branch.is_some());
            assert_eq!(else_branch.unwrap().len(), 0);
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_unclosed() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { content },
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_if_let_unclosed_after_else() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { then {:else} else_content },
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Unclosed"));
    }

    #[test]
    fn test_if_let_preserves_pattern() {
        let pattern = quote! { Ok(User { name, age }) };
        let result = parse_if_let_test(
            pattern.clone(),
            quote! { result },
            quote! { ok {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { pattern: p, .. } = result.unwrap() {
            assert_eq!(p.to_string(), pattern.to_string());
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_preserves_expr() {
        let expr = quote! { complex_function(arg1, arg2) };
        let result = parse_if_let_test(
            quote! { Some(x) },
            expr.clone(),
            quote! { content {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { expr: e, .. } = result.unwrap() {
            assert_eq!(e.to_string(), expr.to_string());
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_complex_pattern() {
        let result = parse_if_let_test(
            quote! { Some((x, y, z)) },
            quote! { triple_opt },
            quote! { @{x}, @{y}, @{z} {/if} },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_let_with_static_content() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { static text here {/if} },
        );
        assert!(result.is_ok());
        if let ControlNode::IfLet { then_branch, .. } = result.unwrap() {
            assert!(!then_branch.is_empty());
        } else {
            panic!("Expected IfLet node");
        }
    }

    #[test]
    fn test_if_let_nested_pattern() {
        let result = parse_if_let_test(
            quote! { Some(Ok(value)) },
            quote! { nested_result },
            quote! { success: @{value} {/if} },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_if_let_else_if_else() {
        let result = parse_if_let_test(
            quote! { Some(x) },
            quote! { opt },
            quote! { some {:else if backup.is_some()} backup {:else} fallback {/if} },
        );
        assert!(result.is_ok());
    }
}
