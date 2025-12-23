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
