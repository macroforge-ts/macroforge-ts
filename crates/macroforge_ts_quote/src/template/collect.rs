use crate::template::{PlaceholderUse, Segment};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::{HashMap, HashSet};

use super::compile::compile_stmt_segments;

/// Checks if a segment list contains statement-level control flow or typescript injection.
fn has_stmt_level_control(
    segments: &[Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
) -> bool {
    for s in segments {
        match s {
            Segment::Control { id, .. } => {
                if matches!(context_map.get(id), Some(PlaceholderUse::Stmt)) {
                    return true;
                }
            }
            // {$typescript} requires statement-level compilation
            Segment::Typescript { .. } => {
                return true;
            }
            Segment::BraceBlock { inner, .. } => {
                if has_stmt_level_control(inner, context_map) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

/// Recursively traverses segments to find and compile BraceBlocks that need block substitution.
fn traverse_segments_recursively(
    seg: &Segment,
    context_map: &HashMap<usize, PlaceholderUse>,
    out_ident: &proc_macro2::Ident,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
    compilations: &mut Vec<(usize, TokenStream2)>,
) -> syn::Result<()> {
    if let Segment::BraceBlock { id, inner, .. } = seg {
        // Check if this block has statement-level control flow or typescript injection
        if has_stmt_level_control(inner, context_map) {
            // Compile the inner segments
            let compiled = compile_stmt_segments(
                inner,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
            )?;
            compilations.push((*id, compiled));
        }

        // Recurse into inner segments to find nested BraceBlocks
        for inner_seg in inner {
            traverse_segments_recursively(
                inner_seg,
                context_map,
                out_ident,
                comments_ident,
                pending_ident,
                pos_ident,
                compilations,
            )?;
        }
    }
    Ok(())
}

/// Collects BraceBlocks that need block substitution and compiles their inner segments.
pub(crate) fn collect_block_compilations(
    run: &[&Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
    comments_ident: &proc_macro2::Ident,
    pending_ident: &proc_macro2::Ident,
    pos_ident: &proc_macro2::Ident,
) -> syn::Result<Vec<(usize, TokenStream2)>> {
    let mut compilations = Vec::new();
    let out_ident = proc_macro2::Ident::new("__mf_block_stmts", Span::call_site());

    for seg in run {
        traverse_segments_recursively(
            seg,
            context_map,
            &out_ident,
            comments_ident,
            pending_ident,
            pos_ident,
            &mut compilations,
        )?;
    }

    Ok(compilations)
}

/// Collects placeholder IDs used in `IdentName` positions.
pub(crate) fn collect_ident_name_ids<'a>(
    segments: impl IntoIterator<Item = &'a Segment>,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> Vec<usize> {
    let mut ids = HashSet::new();

    fn collect_from_segment(
        seg: &Segment,
        context_map: &HashMap<usize, PlaceholderUse>,
        ids: &mut HashSet<usize>,
    ) {
        let id = match seg {
            Segment::Interpolation { id, .. }
            | Segment::StringInterp { id, .. }
            | Segment::TemplateInterp { id, .. }
            | Segment::IdentBlock { id, .. }
            | Segment::Control { id, .. }
            | Segment::Typescript { id, .. }
            | Segment::ObjectPropLoop { id, .. } => Some(*id),
            Segment::BraceBlock { inner, .. } => {
                for inner_seg in inner {
                    collect_from_segment(inner_seg, context_map, ids);
                }
                None
            }
            _ => None,
        };
        if let Some(id) = id
            && matches!(context_map.get(&id), Some(PlaceholderUse::IdentName))
        {
            ids.insert(id);
        }
    }

    for seg in segments {
        collect_from_segment(seg, context_map, &mut ids);
    }

    let mut ids: Vec<_> = ids.into_iter().collect();
    ids.sort_unstable();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{ControlNode, IdentPart};
    use proc_macro2::Span;
    use quote::quote;

    // Helper function to create test identifiers
    fn test_ident(name: &str) -> proc_macro2::Ident {
        proc_macro2::Ident::new(name, Span::call_site())
    }

    #[test]
    fn test_collect_block_compilations_no_blocks() {
        // Test with segments that contain no BraceBlocks
        let context_map = HashMap::new();
        let segments = [Segment::Static("const x = ".to_string()),
            Segment::Interpolation {
                id: 0,
                expr: quote! { value },
            },
            Segment::Static(";".to_string())];
        let seg_refs: Vec<&Segment> = segments.iter().collect();

        let comments_ident = test_ident("comments");
        let pending_ident = test_ident("pending");
        let pos_ident = test_ident("pos");

        let result = collect_block_compilations(
            &seg_refs,
            &context_map,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        )
        .expect("Should succeed");

        assert_eq!(result.len(), 0, "Should have no compilations");
    }

    #[test]
    fn test_collect_block_compilations_single_block_with_control() {
        // Test with a single BraceBlock containing statement-level control flow
        // The block contains valid TypeScript that can be parsed as a module
        let mut context_map = HashMap::new();
        context_map.insert(1, PlaceholderUse::Stmt);

        let inner_segments = vec![
            Segment::Control {
                id: 1,
                node: ControlNode::For {
                    pat: quote! { item },
                    iter: quote! { items },
                    body: vec![
                        Segment::Static("console.log(item);".to_string()),
                    ],
                },
            },
        ];

        let segments = [Segment::Static("function test() ".to_string()),
            Segment::BraceBlock {
                id: 0,
                inner: inner_segments,
            }];
        let seg_refs: Vec<&Segment> = segments.iter().collect();

        let comments_ident = test_ident("comments");
        let pending_ident = test_ident("pending");
        let pos_ident = test_ident("pos");

        let result = collect_block_compilations(
            &seg_refs,
            &context_map,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        )
        .expect("Should succeed");

        assert_eq!(result.len(), 1, "Should have one compilation");
        assert_eq!(result[0].0, 0, "Should compile block with id 0");
    }

    #[test]
    fn test_collect_block_compilations_nested_blocks() {
        // Test with nested BraceBlocks where only the inner block has control flow
        let mut context_map = HashMap::new();
        context_map.insert(2, PlaceholderUse::Stmt);

        let innermost_segments = vec![Segment::Control {
            id: 2,
            node: ControlNode::For {
                pat: quote! { item },
                iter: quote! { items },
                body: vec![Segment::Static("process(item)".to_string())],
            },
        }];

        let inner_segments = vec![
            Segment::Static("if (check) ".to_string()),
            Segment::BraceBlock {
                id: 1,
                inner: innermost_segments,
            },
        ];

        let segments = [Segment::Static("function outer() ".to_string()),
            Segment::BraceBlock {
                id: 0,
                inner: inner_segments,
            }];
        let seg_refs: Vec<&Segment> = segments.iter().collect();

        let comments_ident = test_ident("comments");
        let pending_ident = test_ident("pending");
        let pos_ident = test_ident("pos");

        let result = collect_block_compilations(
            &seg_refs,
            &context_map,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        )
        .expect("Should succeed");

        // Both blocks contain statement-level control (inner has it, outer contains inner)
        // So both should be compiled
        assert_eq!(result.len(), 2, "Should have two compilations");
        // Find the compilations for blocks 0 and 1
        let mut ids: Vec<usize> = result.iter().map(|(id, _)| *id).collect();
        ids.sort();
        assert_eq!(ids, vec![0, 1], "Should compile both blocks");
    }

    #[test]
    fn test_collect_block_compilations_block_without_control() {
        // Test with BraceBlock that doesn't contain statement-level control
        let mut context_map = HashMap::new();
        context_map.insert(1, PlaceholderUse::Expr); // Expression-level, not Stmt

        let inner_segments = vec![
            Segment::Static("return ".to_string()),
            Segment::Interpolation {
                id: 1,
                expr: quote! { value },
            },
            Segment::Static(";".to_string()),
        ];

        let segments = [Segment::Static("function test() ".to_string()),
            Segment::BraceBlock {
                id: 0,
                inner: inner_segments,
            }];
        let seg_refs: Vec<&Segment> = segments.iter().collect();

        let comments_ident = test_ident("comments");
        let pending_ident = test_ident("pending");
        let pos_ident = test_ident("pos");

        let result = collect_block_compilations(
            &seg_refs,
            &context_map,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        )
        .expect("Should succeed");

        assert_eq!(result.len(), 0, "Should have no compilations");
    }

    #[test]
    fn test_collect_block_compilations_block_with_typescript_injection() {
        // Test with BraceBlock containing typescript injection
        let context_map = HashMap::new();

        let inner_segments = vec![
            Segment::Static("const x = 1;".to_string()),
            Segment::Typescript {
                id: 1,
                expr: quote! { ts_stream },
            },
        ];

        let segments = [Segment::Static("function test() ".to_string()),
            Segment::BraceBlock {
                id: 0,
                inner: inner_segments,
            }];
        let seg_refs: Vec<&Segment> = segments.iter().collect();

        let comments_ident = test_ident("comments");
        let pending_ident = test_ident("pending");
        let pos_ident = test_ident("pos");

        let result = collect_block_compilations(
            &seg_refs,
            &context_map,
            &comments_ident,
            &pending_ident,
            &pos_ident,
        )
        .expect("Should succeed");

        assert_eq!(result.len(), 1, "Should have one compilation");
        assert_eq!(result[0].0, 0, "Should compile block with typescript injection");
    }

    #[test]
    fn test_collect_ident_name_ids_empty() {
        // Test with empty segments
        let context_map = HashMap::new();
        let segments: Vec<Segment> = vec![];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result.len(), 0, "Should return empty vector");
    }

    #[test]
    fn test_collect_ident_name_ids_single_ident_name() {
        // Test with a single IdentName placeholder
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::IdentName);

        let segments = vec![
            Segment::Static("const ".to_string()),
            Segment::Interpolation {
                id: 0,
                expr: quote! { var_name },
            },
            Segment::Static(" = 42;".to_string()),
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result.len(), 1, "Should find one IdentName");
        assert_eq!(result[0], 0, "Should contain id 0");
    }

    #[test]
    fn test_collect_ident_name_ids_multiple() {
        // Test with multiple IdentName placeholders
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::IdentName);
        context_map.insert(2, PlaceholderUse::IdentName);
        context_map.insert(4, PlaceholderUse::IdentName);

        let segments = vec![
            Segment::Interpolation {
                id: 0,
                expr: quote! { name1 },
            },
            Segment::Static(", ".to_string()),
            Segment::Interpolation {
                id: 2,
                expr: quote! { name2 },
            },
            Segment::Static(", ".to_string()),
            Segment::Interpolation {
                id: 4,
                expr: quote! { name3 },
            },
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result.len(), 3, "Should find three IdentNames");
        assert_eq!(result, vec![0, 2, 4], "Should return sorted ids");
    }

    #[test]
    fn test_collect_ident_name_ids_not_ident_name() {
        // Test with placeholders that are not IdentName
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::Expr);
        context_map.insert(1, PlaceholderUse::Stmt);
        context_map.insert(2, PlaceholderUse::Type);

        let segments = vec![
            Segment::Interpolation {
                id: 0,
                expr: quote! { expr1 },
            },
            Segment::Control {
                id: 1,
                node: ControlNode::If {
                    cond: quote! { true },
                    then_branch: vec![],
                    else_branch: None,
                },
            },
            Segment::Interpolation {
                id: 2,
                expr: quote! { type1 },
            },
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result.len(), 0, "Should find no IdentNames");
    }

    #[test]
    fn test_collect_ident_name_ids_nested_in_brace_block() {
        // Test with IdentName placeholders nested inside BraceBlocks
        let mut context_map = HashMap::new();
        context_map.insert(1, PlaceholderUse::IdentName);
        context_map.insert(3, PlaceholderUse::IdentName);

        let inner_segments = vec![
            Segment::Static("const ".to_string()),
            Segment::Interpolation {
                id: 1,
                expr: quote! { inner_name },
            },
            Segment::Static(" = ".to_string()),
            Segment::Interpolation {
                id: 3,
                expr: quote! { value },
            },
        ];

        let segments = vec![
            Segment::Static("function test() ".to_string()),
            Segment::BraceBlock {
                id: 0,
                inner: inner_segments,
            },
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result.len(), 2, "Should find IdentNames inside BraceBlock");
        assert_eq!(result, vec![1, 3], "Should return sorted ids");
    }

    #[test]
    fn test_collect_ident_name_ids_mixed_segment_types() {
        // Test with various segment types including IdentBlock, StringInterp, TemplateInterp
        let mut context_map = HashMap::new();
        context_map.insert(0, PlaceholderUse::IdentName);
        context_map.insert(1, PlaceholderUse::IdentName);
        context_map.insert(2, PlaceholderUse::IdentName);
        context_map.insert(3, PlaceholderUse::Expr); // Not IdentName

        let segments = vec![
            Segment::IdentBlock {
                id: 0,
                parts: vec![IdentPart::Static("test".to_string())],
            },
            Segment::StringInterp {
                id: 1,
                parts: vec![],
            },
            Segment::TemplateInterp {
                id: 2,
                parts: vec![],
            },
            Segment::ObjectPropLoop {
                id: 3,
                pat: quote! { (k, v) },
                iter: quote! { items },
                key_expr: quote! { k },
                value_expr: quote! { v },
            },
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result.len(), 3, "Should find three IdentNames");
        assert_eq!(result, vec![0, 1, 2], "Should return sorted ids");
    }

    #[test]
    fn test_collect_ident_name_ids_sorting() {
        // Test that IDs are returned sorted
        let mut context_map = HashMap::new();
        context_map.insert(5, PlaceholderUse::IdentName);
        context_map.insert(2, PlaceholderUse::IdentName);
        context_map.insert(8, PlaceholderUse::IdentName);
        context_map.insert(1, PlaceholderUse::IdentName);

        let segments = vec![
            Segment::Interpolation {
                id: 5,
                expr: quote! { name5 },
            },
            Segment::Interpolation {
                id: 2,
                expr: quote! { name2 },
            },
            Segment::Interpolation {
                id: 8,
                expr: quote! { name8 },
            },
            Segment::Interpolation {
                id: 1,
                expr: quote! { name1 },
            },
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        assert_eq!(result, vec![1, 2, 5, 8], "Should return IDs in sorted order");
    }

    #[test]
    fn test_collect_ident_name_ids_duplicates() {
        // Test that duplicate IDs are handled correctly (should appear once)
        let mut context_map = HashMap::new();
        context_map.insert(1, PlaceholderUse::IdentName);

        // Create nested structure where the same ID might be encountered multiple times
        let inner_segments = vec![Segment::Interpolation {
            id: 1,
            expr: quote! { name },
        }];

        let segments = vec![
            Segment::Interpolation {
                id: 1,
                expr: quote! { name },
            },
            Segment::BraceBlock {
                id: 0,
                inner: inner_segments,
            },
        ];

        let result = collect_ident_name_ids(&segments, &context_map);

        // HashSet should deduplicate, so we should only see id 1 once
        assert_eq!(result.len(), 1, "Should deduplicate IDs");
        assert_eq!(result[0], 1, "Should contain id 1");
    }
}
