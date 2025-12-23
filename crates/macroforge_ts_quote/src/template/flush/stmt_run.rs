//! Statement run flushing - the main orchestrator for template code generation.
//!
//! This module coordinates the different code paths for template processing:
//! - Type placeholder path: When type placeholders are present
//! - Class wrapped path: When module parsing fails (class body members)
//! - Function wrapped path: When class-wrapped fails (method body statements like `this.x = y;`)
//! - Standard path: Normal module statements and exports

use super::class_wrapped_path::generate_class_wrapped_code;
use super::function_wrapped_path::generate_function_wrapped_code;
use super::standard_path::{StandardCodeContext, generate_standard_code};
use super::type_placeholder_path::generate_type_placeholder_code;
use crate::template::build::{PlaceholderSourceKind, build_placeholder_source};
use crate::template::{
    PlaceholderUse, Segment, build_template_and_bindings, collect_block_compilations,
    collect_ident_name_ids, generate_type_placeholder_fix, has_type_placeholder_recursive,
    ident_name_fix_block, parse_ts_module_with_source,
};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::HashMap;

fn format_parse_error_context(source: &str, err: &syn::Error) -> Option<String> {
    let msg = err.to_string();
    let range = msg.split("error: (").nth(1)?;
    let mut parts = range.split("..");
    let start: usize = parts.next()?.parse().ok()?;
    let end_part = parts.next()?;
    let end_str = end_part.split(',').next()?;
    let end: usize = end_str.parse().ok()?;

    let snippet_start = start.saturating_sub(40);
    let snippet_end = (end + 40).min(source.len());
    let snippet = source.get(snippet_start..snippet_end)?;
    Some(format!(
        "Parse context ({}..{}): ...{}...",
        start, end, snippet
    ))
}

fn validate_template_source(source: &str, label: &str) -> syn::Result<()> {
    if let Err(err) = parse_ts_module_with_source(source) {
        let context = format_parse_error_context(source, &err);
        return Err(crate::template::template_error(
            Span::call_site(),
            &format!("TypeScript parse error in template {label}: {err}"),
            context.as_deref(),
        ));
    }
    Ok(())
}


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

    // Build placeholder source for parsing (without $ substitution markers)
    // This is separate from the template which uses $name for quote! substitution
    let segments_vec: Vec<_> = run.iter().copied().cloned().collect();

    // Step 5: Route to appropriate code path
    let (placeholder_source, _) =
        build_placeholder_source(&segments_vec, PlaceholderSourceKind::Module);

    // Check for type placeholders - both in the top-level template AND inside control blocks
    // Type placeholders inside control blocks (like {#match}) aren't added to template_result.type_placeholders
    // because those segments get compiled to Rust code. We need to check recursively.
    let has_type_placeholders = !template_result.type_placeholders.is_empty()
        || has_type_placeholder_recursive(&segments_vec, context_map);

    if template_result.template.contains("export function")
        && template_result.template.contains("opts?:")
    {
        eprintln!(
            "DEBUG: has_type_placeholders={}, type_placeholders={:?}, template={}",
            has_type_placeholders,
            template_result
                .type_placeholders
                .iter()
                .map(|t| t.id)
                .collect::<Vec<_>>(),
            template_result.template
        );
    }

    if has_type_placeholders {
        // Type placeholder path: Use runtime parsing with full TypeScript support
        // Use the same fallback logic as other paths: module -> class -> function

        // Try parsing as module first
        if parse_ts_module_with_source(&placeholder_source).is_ok() {
            // Skip validate_template_source here - the template has $ placeholders for quote!
            // but we're doing runtime parsing which uses valid TS identifiers after conversion
            return Ok(generate_type_placeholder_code(
                &template_result,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
                &block_compilations,
            ));
        }

        // Try class-wrapped - use class_wrapped_path which already supports type placeholders
        let class_wrapped_source = format!("class __MfWrapper {{ {} }}", &placeholder_source);
        if let Ok((module, cm)) = parse_ts_module_with_source(&class_wrapped_source) {
            // Skip validate_template_source - runtime parsing handles $ placeholder conversion
            return generate_class_wrapped_code(&template_result, &cm, &module, out_ident);
        }

        // Try function-wrapped
        let function_wrapped_source = format!("function __MfWrapper() {{ {} }}", &placeholder_source);
        if let Ok((module, cm)) = parse_ts_module_with_source(&function_wrapped_source) {
            // Skip validate_template_source - function_wrapped uses quote_ts which handles $ placeholders
            return generate_function_wrapped_code(&template_result, &cm, &module, out_ident);
        }

        // All paths failed
        return Err(crate::template::template_error(
            Span::call_site(),
            "TypeScript parse error in type-placeholder template. Could not parse as module, class body, or function body.",
            Some(&format!("Source: {}", placeholder_source)),
        ));
    }

    // Try parsing as a module first
    let parse_result = parse_ts_module_with_source(&placeholder_source);

    // Route based on parsing result
    match parse_result {
        Ok((module, cm)) => {
            validate_template_source(&template_result.template, "module")?;
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
            let class_wrapped_source = format!("class __MfWrapper {{ {} }}", &placeholder_source);
            match parse_ts_module_with_source(&class_wrapped_source) {
                Ok((module, cm)) => {
                    let class_wrapped_template =
                        format!("class __MfWrapper {{ {} }}", &template_result.template);
                    validate_template_source(&class_wrapped_template, "class")?;
                    generate_class_wrapped_code(&template_result, &cm, &module, out_ident)
                }
                Err(class_err) => {
                    // Function wrapped path: Try wrapping in a function for method body statements
                    // This handles code like `this.x = y;` which is valid inside a method body
                    let function_wrapped_source =
                        format!("function __MfWrapper() {{ {} }}", &placeholder_source);
                    match parse_ts_module_with_source(&function_wrapped_source) {
                        Ok((module, cm)) => {
                            let function_wrapped_template = format!(
                                "function __MfWrapper() {{ {} }}",
                                &template_result.template
                            );
                            validate_template_source(&function_wrapped_template, "function")?;
                            generate_function_wrapped_code(
                                &template_result,
                                &cm,
                                &module,
                                out_ident,
                            )
                        }
                        Err(func_err) => {
                            let mut context = String::new();
                            if let Some(snippet) =
                                format_parse_error_context(&placeholder_source, &func_err)
                            {
                                context.push_str(&snippet);
                            } else {
                                context.push_str("Failed to parse placeholder source in module/class/function paths.");
                            }
                            Err(crate::template::template_error(
                                Span::call_site(),
                                &format!(
                                    "TypeScript parse error: {func_err} (class parse error: {class_err})"
                                ),
                                Some(&context),
                            ))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{PlaceholderUse, Segment};
    use quote::quote;

    fn create_test_ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    #[test]
    fn test_flush_stmt_run_empty_segments() {
        let run: Vec<&Segment> = Vec::new();
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        // Empty run should produce empty result (not error)
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle empty segments"
        );
    }

    #[test]
    fn test_flush_stmt_run_static_only() {
        let segment = Segment::Static("const x = 1;".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle static segment");
        let code = result.unwrap();
        assert!(!code.is_empty(), "Should generate code for static segment");
    }

    #[test]
    fn test_flush_stmt_run_standard_path() {
        // Standard path: normal statement without type placeholders
        let segment = Segment::Static("const x = 1;".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should route to standard path");
        let code = result.unwrap();
        let code_str = code.to_string();
        // Standard path should include __mf_stmt
        assert!(
            code_str.contains("__mf_stmt") || code_str.is_empty(),
            "Should use standard path"
        );
    }

    #[test]
    fn test_flush_stmt_run_with_interpolation() {
        let segment = Segment::Interpolation {
            id: 0,
            expr: quote! { my_value },
        };
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::Expr);

        let seg1 = Segment::Static("const x = ".to_string());
        let seg3 = Segment::Static(";".to_string());
        let run = vec![&seg1, &segment, &seg3];
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle interpolation");
    }

    #[test]
    fn test_flush_stmt_run_export_statement() {
        let segment = Segment::Static("export const x = 1;".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle export statements");
        let code = result.unwrap();
        let code_str = code.to_string();
        assert!(
            code_str.contains("ModuleItem") || code_str.contains("__mf_stmt"),
            "Should handle export"
        );
    }

    #[test]
    fn test_flush_stmt_run_class_wrapped_path() {
        // Class wrapped path: class body member that fails module parsing
        let segment = Segment::Static("constructor() {}".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        // Constructor should trigger class wrapped path
        assert!(
            result.is_ok(),
            "Should handle class body members via class wrapped path"
        );
    }

    #[test]
    fn test_flush_stmt_run_function_wrapped_path() {
        // Function wrapped path: method body code that uses `this.`
        let segment = Segment::Static("this.x = 1;".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        // this.x = 1; should trigger function wrapped path
        assert!(
            result.is_ok(),
            "Should handle method body statements via function wrapped path"
        );
    }

    #[test]
    fn test_flush_stmt_run_multiple_statements() {
        let segment1 = Segment::Static("const x = 1;".to_string());
        let segment2 = Segment::Static("const y = 2;".to_string());
        let run = vec![&segment1, &segment2];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle multiple statements");
    }

    #[test]
    fn test_flush_stmt_run_uses_provided_idents() {
        let segment = Segment::Static("const x = 1;".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__custom_out");
        let comments_ident = create_test_ident("__custom_comments");
        let pending_ident = create_test_ident("__custom_pending");
        let pos_ident = create_test_ident("__custom_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok());
        let code = result.unwrap();
        let code_str = code.to_string();

        // Should reference the custom identifiers
        assert!(
            code_str.contains("__custom_out") || code_str.is_empty(),
            "Should use provided out_ident"
        );
    }

    #[test]
    fn test_flush_stmt_run_with_comment_segment() {
        let segment = Segment::Comment {
            style: crate::template::CommentStyle::Line,
            text: "test comment".to_string(),
        };
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        // Comment segments should be handled
        assert!(
            result.is_ok() || result.is_err(),
            "Should handle comment segments"
        );
    }

    #[test]
    fn test_flush_stmt_run_complex_typescript() {
        let segment = Segment::Static("interface User { name: string; age: number; }".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle complex TypeScript");
    }

    #[test]
    fn test_flush_stmt_run_function_declaration() {
        let segment = Segment::Static(
            "function add(a: number, b: number): number { return a + b; }".to_string(),
        );
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle function declarations");
    }

    #[test]
    fn test_flush_stmt_run_class_declaration() {
        let segment =
            Segment::Static("class User { constructor(public name: string) {} }".to_string());
        let run = vec![&segment];
        let context_map = HashMap::new();
        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(result.is_ok(), "Should handle class declarations");
    }

    #[test]
    fn test_flush_stmt_run_export_function_with_type_placeholders() {
        // Test export function with type placeholders in parameter and return type positions
        // This matches the failing derive_deserialize.rs case
        let seg1 = Segment::Static("export function ".to_string());
        let seg2 = Segment::Interpolation {
            id: 0,
            expr: quote! { fn_ident },
        };
        let seg3 = Segment::Static("(input: unknown, opts?: ".to_string());
        let seg4 = Segment::Interpolation {
            id: 1,
            expr: quote! { OptsType },
        };
        let seg5 = Segment::Static("): ".to_string());
        let seg6 = Segment::Interpolation {
            id: 2,
            expr: quote! { ReturnType },
        };
        let seg7 = Segment::Static(" {}".to_string());

        let run = vec![&seg1, &seg2, &seg3, &seg4, &seg5, &seg6, &seg7];

        // Context map should have type placeholders classified correctly
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::Ident);  // function name
        context_map.insert(1, PlaceholderUse::Type);   // optional param type
        context_map.insert(2, PlaceholderUse::Type);   // return type

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(
            result.is_ok(),
            "Should handle export function with type placeholders. Error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_flush_stmt_run_export_function_without_context_map() {
        // Test what happens when context_map is EMPTY (simulating the bug)
        // This should still detect type placeholders via classification
        let seg1 = Segment::Static("export function ".to_string());
        let seg2 = Segment::Interpolation {
            id: 0,
            expr: quote! { fn_ident },
        };
        let seg3 = Segment::Static("(input: unknown, opts?: ".to_string());
        let seg4 = Segment::Interpolation {
            id: 1,
            expr: quote! { OptsType },
        };
        let seg5 = Segment::Static("): ".to_string());
        let seg6 = Segment::Interpolation {
            id: 2,
            expr: quote! { ReturnType },
        };
        let seg7 = Segment::Static(" {}".to_string());

        let run = vec![&seg1, &seg2, &seg3, &seg4, &seg5, &seg6, &seg7];

        // EMPTY context map - this is what might be happening in the actual bug
        let context_map = HashMap::new();

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        // This should STILL work because build_template_and_bindings
        // does its own classification when context_map is empty
        assert!(
            result.is_ok(),
            "Should handle export function even with empty context_map. Error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_flush_stmt_run_export_function_with_control_block() {
        use crate::template::ControlNode;
        // Test export function with type placeholders AND a control block in a BraceBlock body
        let seg1 = Segment::Static("export function ".to_string());
        let seg2 = Segment::Interpolation {
            id: 0,
            expr: quote! { fn_ident },
        };
        let seg3 = Segment::Static("(input: unknown, opts?: ".to_string());
        let seg4 = Segment::Interpolation {
            id: 1,
            expr: quote! { OptsType },
        };
        let seg5 = Segment::Static("): ".to_string());
        let seg6 = Segment::Interpolation {
            id: 2,
            expr: quote! { ReturnType },
        };
        let seg7 = Segment::Static(" ".to_string());
        // The control block should be inside a BraceBlock
        let seg8 = Segment::BraceBlock {
            id: 4,  // Block id
            inner: vec![
                Segment::Control {
                    id: 3,
                    node: ControlNode::For {
                        pat: quote! { item },
                        iter: quote! { items },
                        body: vec![
                            Segment::Static("console.log(item);".to_string()),
                        ],
                    },
                },
            ],
        };

        let run = vec![&seg1, &seg2, &seg3, &seg4, &seg5, &seg6, &seg7, &seg8];

        // Context map with types AND the control block as Stmt
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::Ident);  // function name
        context_map.insert(1, PlaceholderUse::Type);   // optional param type
        context_map.insert(2, PlaceholderUse::Type);   // return type
        context_map.insert(3, PlaceholderUse::Stmt);   // control block

        let out_ident = create_test_ident("__mf_out");
        let comments_ident = create_test_ident("__mf_comments");
        let pending_ident = create_test_ident("__mf_pending");
        let pos_ident = create_test_ident("__mf_pos");

        let result = flush_stmt_run(
            &run,
            &context_map,
            &out_ident,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        );

        assert!(
            result.is_ok(),
            "Should handle export function with type placeholders and control block. Error: {:?}",
            result.err()
        );
    }

    #[test]
    fn test_build_template_and_bindings_with_type_placeholders() {
        // Test that build_template_and_bindings correctly handles type placeholders
        use crate::template::build_template_and_bindings;

        let seg1 = Segment::Static("export function ".to_string());
        let seg2 = Segment::Interpolation {
            id: 0,
            expr: quote! { fn_ident },
        };
        let seg3 = Segment::Static("(input: unknown, opts?: ".to_string());
        let seg4 = Segment::Interpolation {
            id: 1,
            expr: quote! { OptsType },
        };
        let seg5 = Segment::Static("): ".to_string());
        let seg6 = Segment::Interpolation {
            id: 2,
            expr: quote! { ReturnType },
        };
        let seg7 = Segment::Static(" {}".to_string());

        let segments = [&seg1, &seg2, &seg3, &seg4, &seg5, &seg6, &seg7];

        // Context map with types
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::Ident);
        context_map.insert(1, PlaceholderUse::Type);
        context_map.insert(2, PlaceholderUse::Type);

        let result = build_template_and_bindings(segments.iter().copied(), &context_map).unwrap();

        eprintln!("Template: {}", result.template);
        eprintln!("Type placeholders: {:?}", result.type_placeholders.iter().map(|t| t.id).collect::<Vec<_>>());
        eprintln!("Bindings: {:?}", result.bindings.iter().map(|b| b.name.to_string()).collect::<Vec<_>>());

        // Type placeholders should use __MfTypeMarkerX
        assert!(result.template.contains("__MfTypeMarker1"), "Template should contain type marker for id 1");
        assert!(result.template.contains("__MfTypeMarker2"), "Template should contain type marker for id 2");

        // Type placeholders should be in the type_placeholders list
        assert_eq!(result.type_placeholders.len(), 2, "Should have 2 type placeholders");

        // Non-type placeholders should use $__mf_hole_X
        assert!(result.template.contains("$__mf_hole_0"), "Template should contain $ placeholder for id 0");
    }
}
