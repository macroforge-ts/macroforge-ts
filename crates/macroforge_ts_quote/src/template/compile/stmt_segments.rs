use proc_macro2::TokenStream as TokenStream2;

use super::segment_dispatch::{compile_segment, should_flush_run};
use crate::template::{classify_placeholders_module, flush_stmt_run, Segment};

/// Compiles statement-level segments into Rust code that builds SWC statements.
pub fn compile_stmt_segments(
    segments: &[Segment],
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    let context_map = classify_placeholders_module(segments)?;
    let mut output = TokenStream2::new();
    let mut run: Vec<&Segment> = Vec::new();

    for segment in segments {
        // Check if we need to flush the run buffer
        if should_flush_run(segment, &context_map) && !run.is_empty() {
            output.extend(flush_stmt_run(
                &run,
                &context_map,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?);
            run.clear();
        }

        // Compile the segment if it requires statement-level handling
        if let Some(compiled) = compile_segment(
            segment,
            &context_map,
            out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
        )? {
            output.extend(compiled);
        } else {
            // Add to run buffer for batch processing
            run.push(segment);
        }
    }

    // Flush any remaining segments in the run buffer
    if !run.is_empty() {
        output.extend(flush_stmt_run(
            &run,
            &context_map,
            out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
        )?);
    }

    Ok(output)
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

    #[test]
    fn test_compile_stmt_segments_empty() {
        let segments: Vec<Segment> = vec![];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_compile_stmt_segments_simple_static() {
        let segments = vec![Segment::Static("const x = 1;".to_string())];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_segments_multiple_static() {
        let segments = vec![
            Segment::Static("const x = 1;".to_string()),
            Segment::Static("const y = 2;".to_string()),
        ];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_segments_with_comment() {
        let segments = vec![
            Segment::Comment {
                style: CommentStyle::Line,
                text: "This is a comment".to_string(),
            },
            Segment::Static("const x = 1;".to_string()),
        ];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("__mf_pending"));
    }

    #[test]
    fn test_compile_stmt_segments_with_let_binding() {
        let segments = vec![Segment::Let {
            tokens: quote! { x = 5 },
        }];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("let x = 5"));
    }

    #[test]
    fn test_compile_stmt_segments_with_do_expr() {
        let segments = vec![Segment::Do {
            expr: quote! { println!("test") },
        }];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("println !"));
    }

    #[test]
    fn test_compile_stmt_segments_with_typescript_injection() {
        let segments = vec![Segment::Typescript {
            id: 0,
            expr: quote! { my_stream },
        }];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());

        let tokens = result.unwrap();
        let tokens_str = tokens.to_string();
        assert!(tokens_str.contains("my_stream"));
    }

    #[test]
    fn test_compile_stmt_segments_flushes_run_before_control() {
        let segments = vec![
            Segment::Static("const x = 1;".to_string()),
            Segment::Control {
                id: 0,
                node: ControlNode::If {
                    cond: quote! { true },
                    then_branch: vec![Segment::Static("const y = 2;".to_string())],
                    else_branch: None,
                },
            },
        ];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compile_stmt_segments_flushes_run_at_end() {
        let segments = vec![
            Segment::Static("const x = 1;".to_string()),
            Segment::Static("const y = 2;".to_string()),
        ];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_compile_stmt_segments_mixed_content() {
        let segments = vec![
            Segment::Comment {
                style: CommentStyle::Line,
                text: "Start".to_string(),
            },
            Segment::Static("const x = 1;".to_string()),
            Segment::Let {
                tokens: quote! { y = 2 },
            },
            Segment::Static("const z = 3;".to_string()),
        ];
        let (out, comments, pending, pos) = make_idents();

        let result = compile_stmt_segments(&segments, &out, &comments, &pending, &pos);
        assert!(result.is_ok());
    }
}
