use proc_macro2::TokenStream as TokenStream2;
use std::collections::HashMap;

use crate::template::{PlaceholderUse, Segment};

/// Determines if the run buffer should be flushed before processing this segment.
///
/// Returns `true` if the segment requires statement-level handling and the run
/// buffer should be flushed first.
pub(super) fn should_flush_run(
    segment: &Segment,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> bool {
    match segment {
        Segment::Control { id, .. } => {
            matches!(context_map.get(id), Some(PlaceholderUse::Stmt))
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
/// - Statement-level control flow
/// - TypeScript injections
/// - Comments
/// - Let bindings
/// - Do expressions
///
/// Returns `None` for segments that should be added to the run buffer instead.
pub(super) fn compile_segment(
    segment: &Segment,
    context_map: &HashMap<usize, PlaceholderUse>,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<Option<TokenStream2>> {
    use quote::quote;

    use super::{compile_stmt_control, compile_ts_injection};
    use crate::template::{build_comment_expr, template_error};

    match segment {
        Segment::Control { id, node } => {
            let is_stmt = matches!(context_map.get(id), Some(PlaceholderUse::Stmt));
            if is_stmt {
                let tokens = compile_stmt_control(
                    node,
                    out_ident,
                    comments_ident,
                    pending_ident,
                    pos_ident,
                )?;
                Ok(Some(tokens))
            } else {
                Ok(None)
            }
        }
        Segment::Typescript { id, expr } => {
            if matches!(context_map.get(id), Some(PlaceholderUse::Stmt) | None) {
                let tokens = compile_ts_injection(
                    expr,
                    out_ident,
                    comments_ident,
                    pending_ident,
                );
                Ok(Some(tokens))
            } else {
                Err(template_error(
                    proc_macro2::Span::call_site(),
                    "{$typescript} is only valid at statement boundaries",
                    None,
                ))
            }
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
