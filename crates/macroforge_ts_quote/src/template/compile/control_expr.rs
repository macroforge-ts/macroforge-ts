use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;

use super::compile_expr_segments;
use crate::template::{template_error, ControlNode};

/// Compiles control nodes in expression context into Rust expressions.
pub fn compile_control_expr(node: &ControlNode, span: Span) -> syn::Result<TokenStream2> {
    match node {
        ControlNode::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let then_expr = compile_expr_segments(then_branch)?;
            let else_expr = if let Some(branch) = else_branch {
                compile_expr_segments(branch)?
            } else {
                return Err(template_error(
                    span,
                    "Expression-level {#if} requires an {:else} branch",
                    None,
                ));
            };
            Ok(quote! {
                if #cond {
                    #then_expr
                } else {
                    #else_expr
                }
            })
        }
        ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch,
        } => {
            let then_expr = compile_expr_segments(then_branch)?;
            let else_expr = if let Some(branch) = else_branch {
                compile_expr_segments(branch)?
            } else {
                return Err(template_error(
                    span,
                    "Expression-level {#if let} requires an {:else} branch",
                    None,
                ));
            };
            Ok(quote! {
                if let #pattern = #expr {
                    #then_expr
                } else {
                    #else_expr
                }
            })
        }
        ControlNode::Match { expr, cases } => {
            let mut arms = TokenStream2::new();
            for case in cases {
                let pattern = &case.pattern;
                let body_expr = compile_expr_segments(&case.body)?;
                arms.extend(quote! {
                    #pattern => #body_expr,
                });
            }
            Ok(quote! {
                match #expr {
                    #arms
                }
            })
        }
        ControlNode::For { .. } | ControlNode::While { .. } | ControlNode::WhileLet { .. } => {
            Err(template_error(
                span,
                "Loop constructs are not allowed in expression context",
                None,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{MatchCase, Segment};
    use quote::quote;

    #[test]
    fn test_if_with_else_branch() {
        // Use valid TypeScript expressions (not just identifiers)
        let node = ControlNode::If {
            cond: quote! { x > 5 },
            then_branch: vec![Segment::Static("1".to_string())],
            else_branch: Some(vec![Segment::Static("0".to_string())]),
        };

        let result = compile_control_expr(&node, Span::call_site());
        if let Err(e) = &result {
            eprintln!("Error in test_if_with_else_branch: {}", e);
        }
        assert!(result.is_ok(), "Expected Ok result");

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if x > 5"));
        assert!(tokens_str.contains("else"));
    }

    #[test]
    fn test_if_without_else_branch_fails() {
        let node = ControlNode::If {
            cond: quote! { x > 5 },
            then_branch: vec![Segment::Static("then".to_string())],
            else_branch: None,
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("requires an {:else} branch"));
    }

    #[test]
    fn test_if_let_with_else_branch() {
        let node = ControlNode::IfLet {
            pattern: quote! { Some(x) },
            expr: quote! { opt },
            then_branch: vec![Segment::Static("some".to_string())],
            else_branch: Some(vec![Segment::Static("none".to_string())]),
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if let Some (x) = opt"));
    }

    #[test]
    fn test_if_let_without_else_branch_fails() {
        let node = ControlNode::IfLet {
            pattern: quote! { Some(x) },
            expr: quote! { opt },
            then_branch: vec![Segment::Static("some".to_string())],
            else_branch: None,
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("requires an {:else} branch"));
    }

    #[test]
    fn test_match_single_case() {
        let node = ControlNode::Match {
            expr: quote! { value },
            cases: vec![MatchCase {
                pattern: quote! { x },
                body: vec![Segment::Static("matched".to_string())],
            }],
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("match value"));
    }

    #[test]
    fn test_match_multiple_cases() {
        let node = ControlNode::Match {
            expr: quote! { value },
            cases: vec![
                MatchCase {
                    pattern: quote! { 1 },
                    body: vec![Segment::Static("one".to_string())],
                },
                MatchCase {
                    pattern: quote! { 2 },
                    body: vec![Segment::Static("two".to_string())],
                },
                MatchCase {
                    pattern: quote! { _ },
                    body: vec![Segment::Static("other".to_string())],
                },
            ],
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("match value"));
        assert!(tokens_str.contains("1 =>"));
        assert!(tokens_str.contains("2 =>"));
        assert!(tokens_str.contains("_ =>"));
    }

    #[test]
    fn test_match_empty_cases() {
        let node = ControlNode::Match {
            expr: quote! { value },
            cases: vec![],
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("match value"));
    }

    #[test]
    fn test_for_loop_fails() {
        let node = ControlNode::For {
            pat: quote! { item },
            iter: quote! { items },
            body: vec![Segment::Static("body".to_string())],
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("Loop constructs are not allowed"));
    }

    #[test]
    fn test_while_loop_fails() {
        let node = ControlNode::While {
            cond: quote! { true },
            body: vec![Segment::Static("body".to_string())],
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("Loop constructs are not allowed"));
    }

    #[test]
    fn test_while_let_loop_fails() {
        let node = ControlNode::WhileLet {
            pattern: quote! { Some(x) },
            expr: quote! { iter.next() },
            body: vec![Segment::Static("body".to_string())],
        };

        let result = compile_control_expr(&node, Span::call_site());
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.to_string().contains("Loop constructs are not allowed"));
    }
}
