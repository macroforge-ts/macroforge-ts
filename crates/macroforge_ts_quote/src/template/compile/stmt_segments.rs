use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::{compile_stmt_control, compile_ts_injection};
use crate::template::{
    build_comment_expr, classify_placeholders_module, flush_stmt_run, template_error,
    PlaceholderUse, Segment,
};

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
        match segment {
            Segment::Control { id, node } => {
                let is_stmt = matches!(context_map.get(id), Some(PlaceholderUse::Stmt));
                if is_stmt {
                    if !run.is_empty() {
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
                    output.extend(compile_stmt_control(
                        node,
                        out_ident,
                        comments_ident,
                        pending_ident,
                        pos_ident,
                    )?);
                } else {
                    run.push(segment);
                }
            }
            Segment::Typescript { id, expr } => {
                if !run.is_empty() {
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
                if matches!(context_map.get(id), Some(PlaceholderUse::Stmt) | None) {
                    output.extend(compile_ts_injection(
                        expr,
                        out_ident,
                        comments_ident,
                        pending_ident,
                    ));
                } else {
                    return Err(template_error(
                        proc_macro2::Span::call_site(),
                        "{$typescript} is only valid at statement boundaries",
                        None,
                    ));
                }
            }
            Segment::Comment { style, text, .. } => {
                if !run.is_empty() {
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
                let comment = build_comment_expr(style, text);
                output.extend(quote! {
                    #pending_ident.push(#comment);
                });
            }
            Segment::Let { tokens, .. } => {
                if !run.is_empty() {
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
                output.extend(quote! { let #tokens; });
            }
            Segment::Do { expr, .. } => {
                if !run.is_empty() {
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
                output.extend(quote! { #expr; });
            }
            // BraceBlock is handled like other segments - added to the run and processed
            // by build_template_and_bindings, which inlines the content with { }
            _ => run.push(segment),
        }
    }

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
