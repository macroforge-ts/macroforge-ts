use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::compile_stmt_segments;
use crate::template::ControlNode;

/// Compiles statement-level control nodes into Rust control flow.
pub fn compile_stmt_control(
    node: &ControlNode,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    match node {
        ControlNode::If {
            cond,
            then_branch,
            else_branch,
        } => {
            let then_code = compile_stmt_segments(
                then_branch,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            let else_code = if let Some(branch) = else_branch {
                compile_stmt_segments(branch, out_ident, comments_ident, pending_ident, pos_ident)?
            } else {
                TokenStream2::new()
            };
            Ok(quote! {
                if #cond {
                    #then_code
                } else {
                    #else_code
                }
            })
        }
        ControlNode::IfLet {
            pattern,
            expr,
            then_branch,
            else_branch,
        } => {
            let then_code = compile_stmt_segments(
                then_branch,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            let else_code = if let Some(branch) = else_branch {
                compile_stmt_segments(branch, out_ident, comments_ident, pending_ident, pos_ident)?
            } else {
                TokenStream2::new()
            };
            Ok(quote! {
                if let #pattern = #expr {
                    #then_code
                } else {
                    #else_code
                }
            })
        }
        ControlNode::For { pat, iter, body } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                for #pat in #iter {
                    #body_code
                }
            })
        }
        ControlNode::While { cond, body } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                while #cond {
                    #body_code
                }
            })
        }
        ControlNode::WhileLet {
            pattern,
            expr,
            body,
        } => {
            let body_code =
                compile_stmt_segments(body, out_ident, comments_ident, pending_ident, pos_ident)?;
            Ok(quote! {
                while let #pattern = #expr {
                    #body_code
                }
            })
        }
        ControlNode::Match { expr, cases } => {
            let mut arms = TokenStream2::new();
            for case in cases {
                let body_code = compile_stmt_segments(
                    &case.body,
                    out_ident,
                    comments_ident,
                    pending_ident,
                    pos_ident,
                )?;
                let pattern = &case.pattern;
                arms.extend(quote! {
                    #pattern => { #body_code }
                });
            }
            Ok(quote! {
                match #expr {
                    #arms
                }
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{MatchCase, Segment};
    use proc_macro2::Span;

    fn make_idents() -> (
        proc_macro2::Ident,
        proc_macro2::Ident,
        proc_macro2::Ident,
        proc_macro2::Ident,
    ) {
        (
            proc_macro2::Ident::new("__mf_out", Span::call_site()),
            proc_macro2::Ident::new("__mf_comments", Span::call_site()),
            proc_macro2::Ident::new("__mf_pending", Span::call_site()),
            proc_macro2::Ident::new("__mf_pos", Span::call_site()),
        )
    }

    #[test]
    fn test_compile_stmt_control_if_with_else() {
        let node = ControlNode::If {
            cond: quote! { x > 5 },
            then_branch: vec![Segment::Static("const a = 1;".to_string())],
            else_branch: Some(vec![Segment::Static("const b = 2;".to_string())]),
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if x > 5"));
        assert!(tokens_str.contains("else"));
    }

    #[test]
    fn test_compile_stmt_control_if_without_else() {
        let node = ControlNode::If {
            cond: quote! { x > 5 },
            then_branch: vec![Segment::Static("const a = 1;".to_string())],
            else_branch: None,
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if x > 5"));
    }

    #[test]
    fn test_compile_stmt_control_if_empty_branches() {
        let node = ControlNode::If {
            cond: quote! { true },
            then_branch: vec![],
            else_branch: Some(vec![]),
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_control_if_let_with_else() {
        let node = ControlNode::IfLet {
            pattern: quote! { Some(x) },
            expr: quote! { opt },
            then_branch: vec![Segment::Static("const a = x;".to_string())],
            else_branch: Some(vec![Segment::Static("const b = 0;".to_string())]),
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if let Some (x) = opt"));
    }

    #[test]
    fn test_compile_stmt_control_if_let_without_else() {
        let node = ControlNode::IfLet {
            pattern: quote! { Some(x) },
            expr: quote! { opt },
            then_branch: vec![Segment::Static("const a = x;".to_string())],
            else_branch: None,
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("if let Some (x) = opt"));
    }

    #[test]
    fn test_compile_stmt_control_for_loop() {
        let node = ControlNode::For {
            pat: quote! { item },
            iter: quote! { items },
            body: vec![Segment::Static("console.log(item);".to_string())],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("for item in items"));
    }

    #[test]
    fn test_compile_stmt_control_for_loop_empty_body() {
        let node = ControlNode::For {
            pat: quote! { _ },
            iter: quote! { vec![] },
            body: vec![],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_control_while_loop() {
        let node = ControlNode::While {
            cond: quote! { counter < 10 },
            body: vec![Segment::Static("counter++;".to_string())],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("while counter < 10"));
    }

    #[test]
    fn test_compile_stmt_control_while_loop_empty_body() {
        let node = ControlNode::While {
            cond: quote! { true },
            body: vec![],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_control_while_let_loop() {
        let node = ControlNode::WhileLet {
            pattern: quote! { Some(x) },
            expr: quote! { iter.next() },
            body: vec![Segment::Static("process(x);".to_string())],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("while let Some (x) = iter . next ()"));
    }

    #[test]
    fn test_compile_stmt_control_while_let_empty_body() {
        let node = ControlNode::WhileLet {
            pattern: quote! { Some(_) },
            expr: quote! { iter.next() },
            body: vec![],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_control_match_single_case() {
        let node = ControlNode::Match {
            expr: quote! { value },
            cases: vec![MatchCase {
                pattern: quote! { x },
                body: vec![Segment::Static("return x;".to_string())],
            }],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("match value"));
        assert!(tokens_str.contains("x =>"));
    }

    #[test]
    fn test_compile_stmt_control_match_multiple_cases() {
        let node = ControlNode::Match {
            expr: quote! { status },
            cases: vec![
                MatchCase {
                    pattern: quote! { Status::Active },
                    body: vec![Segment::Static("handleActive();".to_string())],
                },
                MatchCase {
                    pattern: quote! { Status::Inactive },
                    body: vec![Segment::Static("handleInactive();".to_string())],
                },
                MatchCase {
                    pattern: quote! { _ },
                    body: vec![Segment::Static("handleDefault();".to_string())],
                },
            ],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("match status"));
        assert!(tokens_str.contains("Status :: Active =>"));
        assert!(tokens_str.contains("Status :: Inactive =>"));
        assert!(tokens_str.contains("_ =>"));
    }

    #[test]
    fn test_compile_stmt_control_match_empty_cases() {
        let node = ControlNode::Match {
            expr: quote! { value },
            cases: vec![],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("match value"));
    }

    #[test]
    fn test_compile_stmt_control_match_case_with_empty_body() {
        let node = ControlNode::Match {
            expr: quote! { value },
            cases: vec![MatchCase {
                pattern: quote! { _ },
                body: vec![],
            }],
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_control(&node, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }
}
