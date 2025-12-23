//! Statement run flushing - the main orchestrator for template code generation.
//!
//! This module coordinates the different code paths for template processing:
//! - Type placeholder path: When type placeholders are present
//! - Class wrapped path: When module parsing fails (class body members)
//! - Standard path: Normal module statements and exports

use super::class_wrapped_path::generate_class_wrapped_code;
use super::standard_path::{generate_standard_code, StandardCodeContext};
use super::type_placeholder_path::generate_type_placeholder_code;
use crate::template::{
    build_template_and_bindings, collect_block_compilations, collect_ident_name_ids,
    generate_type_placeholder_fix, ident_name_fix_block, parse_ts_module_with_source,
    PlaceholderUse, Segment,
};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::HashMap;

/// Emits a run of statement segments as parsed SWC statements, attaching comments.
///
/// This is the main orchestrator that:
/// 1. Builds template and bindings from segments
/// 2. Collects ident name and type placeholder fixes
/// 3. Routes to the appropriate code generation path
pub fn flush_stmt_run(
    run: &[&Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<TokenStream2> {
    // Step 1: Build template and bindings
    let template_result = build_template_and_bindings(run.iter().copied(), context_map)?;
    if template_result.template.trim().is_empty() {
        return Ok(TokenStream2::new());
    }

    // Step 2: Collect fixes for ident names
    let ident_name_ids = collect_ident_name_ids(run.iter().copied(), context_map);
    let ident_name_fix = ident_name_fix_block(
        &proc_macro2::Ident::new("__mf_stmt", Span::call_site()),
        &ident_name_ids,
    );

    // Step 3: Generate type placeholder fixes
    let type_fix = generate_type_placeholder_fix(&template_result.type_placeholders);

    // Step 4: Collect block compilations
    let block_compilations =
        collect_block_compilations(run, context_map, comments_ident, pending_ident, pos_ident)?;

    // Step 5: Route to appropriate code path
    if !template_result.type_placeholders.is_empty() {
        // Type placeholder path: Use runtime parsing with full TypeScript support
        return Ok(generate_type_placeholder_code(
            &template_result,
            out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
        ));
    }

    // Try parsing as a module first
    let parse_result = parse_ts_module_with_source(&template_result.template);

    // Route based on parsing result
    match parse_result {
        Ok((module, cm)) => {
            // Standard path: Normal module processing
            let ctx = StandardCodeContext {
                cm: &cm,
                template_result: &template_result,
                ident_name_fix: &ident_name_fix,
                type_fix: &type_fix,
                block_compilations: &block_compilations,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            };
            generate_standard_code(&module, &ctx)
        }
        Err(_) => {
            // Class wrapped path: Try wrapping in a class for class body members
            let wrapped_source = format!("class __MfWrapper {{ {} }}", &template_result.template);
            let (module, cm) = parse_ts_module_with_source(&wrapped_source)?;
            generate_class_wrapped_code(&template_result, &cm, &module, out_ident)
        }
    }
}
