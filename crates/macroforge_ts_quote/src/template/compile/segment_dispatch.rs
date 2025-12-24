use proc_macro2::TokenStream as TokenStream2;

use crate::template::Segment;

/// Determines if the run buffer should be flushed before processing this segment.
///
/// Returns `true` if the segment requires statement-level handling and the run
/// buffer should be flushed first.
pub(super) fn should_flush_run(segment: &Segment) -> bool {
    match segment {
        Segment::Control { .. } => {
            // Control segments should NOT cause flush (return false)
            false
        }
        Segment::Typescript { .. } | Segment::Comment { .. } | Segment::Let { .. } | Segment::Do { .. } => {
            true
        }
        _ => false,
    }
}

/// Compiles a single segment into Rust code for statement-level processing.
///
/// This handles segments that require special statement-level treatment:
/// - TypeScript injections
/// - Comments
/// - Let bindings
/// - Do expressions
///
/// Returns `None` for segments that should be added to the run buffer instead.
pub(super) fn compile_segment(
    segment: &Segment,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    _pos_ident: &proc_macro2::Ident,
) -> syn::Result<Option<TokenStream2>> {
    use quote::quote;

    use super::compile_ts_injection;
    use crate::template::build_comment_expr;

    match segment {
        Segment::Control { .. } => {
            // Control segments should return Ok(None) (not is_stmt context)
            Ok(None)
        }
        Segment::Typescript { expr, .. } => {
            // Since check was `Some(Stmt) | None` and we always have None, it always matches - remove the check
            let tokens = compile_ts_injection(
                expr,
                out_ident,
                comments_ident,
                pending_ident,
            );
            Ok(Some(tokens))
        }
        Segment::Comment { style, text, .. } => {
            let comment = build_comment_expr(style, text);
            Ok(Some(quote! {
                #pending_ident.push(#comment);
            }))
        }
        Segment::Let { tokens, .. } => {
            Ok(Some(quote! { let #tokens; }))
        }
        Segment::Do { expr, .. } => {
            Ok(Some(quote! { #expr; }))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{CommentStyle, ControlNode};
    use proc_macro2::Span;
    use quote::quote;

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

    // Tests for should_flush_run

    #[test]
    fn test_should_flush_run_static_segment() {
        let segment = Segment::Static("test".to_string());

        assert!(!should_flush_run(&segment));
    }

    #[test]
    fn test_should_flush_run_comment_segment() {
        let segment = Segment::Comment {
            style: CommentStyle::Line,
            text: "comment".to_string(),
        };

        assert!(should_flush_run(&segment));
    }

    #[test]
    fn test_should_flush_run_typescript_segment() {
        let segment = Segment::Typescript {
            expr: quote! { stream },
        };

        assert!(should_flush_run(&segment));
    }

    #[test]
    fn test_should_flush_run_let_segment() {
        let segment = Segment::Let {
            tokens: quote! { x = 5 },
        };

        assert!(should_flush_run(&segment));
    }

    #[test]
    fn test_should_flush_run_do_segment() {
        let segment = Segment::Do {
            expr: quote! { println!("test") },
        };

        assert!(should_flush_run(&segment));
    }

    #[test]
    fn test_should_flush_run_control_segment() {
        let segment = Segment::Control {
            node: ControlNode::If {
                cond: quote! { true },
                then_branch: vec![],
                else_branch: None,
            },
        };

        assert!(!should_flush_run(&segment));
    }

    #[test]
    fn test_should_flush_run_interpolation_segment() {
        let segment = Segment::Interpolation {
            id: 0,
            expr: quote! { x },
        };

        assert!(!should_flush_run(&segment));
    }

    // Tests for compile_segment

    #[test]
    fn test_compile_segment_static_returns_none() {
        let segment = Segment::Static("test".to_string());
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_compile_segment_comment() {
        let segment = Segment::Comment {
            style: CommentStyle::Line,
            text: "test comment".to_string(),
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert!(tokens.is_some());

        let tokens_str = tokens.unwrap().to_string();
        assert!(tokens_str.contains("__mf_pending"));
        assert!(tokens_str.contains("push"));
    }

    #[test]
    fn test_compile_segment_let_binding() {
        let segment = Segment::Let {
            tokens: quote! { x = 42 },
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert!(tokens.is_some());

        let tokens_str = tokens.unwrap().to_string();
        assert!(tokens_str.contains("let x = 42"));
    }

    #[test]
    fn test_compile_segment_do_expr() {
        let segment = Segment::Do {
            expr: quote! { dbg!(value) },
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert!(tokens.is_some());

        let tokens_str = tokens.unwrap().to_string();
        assert!(tokens_str.contains("dbg ! (value)"));
    }

    #[test]
    fn test_compile_segment_typescript() {
        let segment = Segment::Typescript {
            expr: quote! { my_stream },
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert!(tokens.is_some());
    }

    #[test]
    fn test_compile_segment_control_returns_none() {
        let segment = Segment::Control {
            node: ControlNode::If {
                cond: quote! { true },
                then_branch: vec![],
                else_branch: None,
            },
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_compile_segment_interpolation_returns_none() {
        let segment = Segment::Interpolation {
            id: 0,
            expr: quote! { x },
        };
        let (out, comments, pending, pos) = make_idents();

        let result = compile_segment(&segment, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
