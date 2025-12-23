use crate::template::{ControlNode, MatchCase, PlaceholderUse, Segment};
use proc_macro2::{Span, TokenStream as TokenStream2};
use std::collections::{HashMap, HashSet};

use super::compile::compile_stmt_segments;

/// Checks if the trailing part of a static segment indicates the next placeholder is in a type position.
///
/// This is a heuristic for detecting type positions when placeholders are inside control blocks
/// and their IDs are not available in the top-level context_map.
pub(crate) fn is_type_position_suffix(s: &str) -> bool {
    let trimmed = s.trim_end();

    // Check for common type position patterns
    // Note: Order matters for some overlapping patterns

    // `as @{type}` - type assertion
    if trimmed.ends_with(" as") || trimmed.ends_with("\tas") || trimmed == "as" {
        return true;
    }

    // `: @{type}` - parameter type annotation or property type
    // `?: @{type}` - optional parameter type
    if trimmed.ends_with(':') || trimmed.ends_with("?:") {
        return true;
    }

    // `): @{type}` - return type (already covered by `:` check above, but explicit for clarity)
    // The `:` at the end catches this case

    // `<@{type}` - type argument
    if trimmed.ends_with('<') {
        return true;
    }

    // `| @{type}` - union type member
    if trimmed.ends_with('|') {
        return true;
    }

    // `& @{type}` - intersection type member
    if trimmed.ends_with('&') {
        return true;
    }

    // `, @{type}` - additional type parameter in `<T, @{type}>`
    // This is tricky because `,` can also be in expression contexts.
    // We check for this only if it looks like we're in a type parameter list
    // by looking for a preceding `<` without a closing `>`.
    // For now, skip this heuristic as it's too risky.

    // `extends @{type}` - generic constraint
    if trimmed.ends_with(" extends") || trimmed.ends_with("\textends") {
        return true;
    }

    // `implements @{type}` - interface implementation
    if trimmed.ends_with(" implements") || trimmed.ends_with("\timplements") {
        return true;
    }

    // `typeof @{type}` - type query (when used in type position)
    // This is ambiguous - `typeof` can be used in expression context too.
    // Skip for safety.

    // `keyof @{type}` - key of type operator
    if trimmed.ends_with(" keyof") || trimmed.ends_with("\tkeyof") {
        return true;
    }

    // `infer @{type}` - conditional type inference
    if trimmed.ends_with(" infer") || trimmed.ends_with("\tinfer") {
        return true;
    }

    // `is @{type}` - type predicate return type (e.g., `x is @{type}`)
    if trimmed.ends_with(" is") || trimmed.ends_with("\tis") {
        return true;
    }

    // `asserts @{type}` - assertion function return type
    if trimmed.ends_with(" asserts") || trimmed.ends_with("\tasserts") {
        return true;
    }

    // `satisfies @{type}` - satisfies operator
    if trimmed.ends_with(" satisfies") || trimmed.ends_with("\tsatisfies") {
        return true;
    }

    false
}

/// Recursively checks if any segment (including inside control blocks) has a type placeholder.
///
/// This is used to determine whether the type_placeholder_path should be used,
/// even when type placeholders are nested inside control blocks that get compiled to Rust code.
///
/// For top-level segments, we use the context_map. For segments inside control blocks,
/// we detect type positions heuristically (e.g., preceded by `as `, `:`, `<`, etc.).
pub(crate) fn has_type_placeholder_recursive(
    segments: &[Segment],
    context_map: &HashMap<usize, PlaceholderUse>,
) -> bool {
    let mut pending_type_suffix = false;
    for seg in segments {
        match seg {
            Segment::Static(s) => {
                pending_type_suffix = is_type_position_suffix(s);
            }
            _ => {
                if check_segment_for_type_placeholder(seg, context_map, pending_type_suffix) {
                    return true;
                }
                pending_type_suffix = false;
            }
        }
    }
    false
}

fn check_segment_for_type_placeholder(
    seg: &Segment,
    context_map: &HashMap<usize, PlaceholderUse>,
    pending_type_suffix: bool,
) -> bool {
    match seg {
        Segment::Interpolation { id, .. } => {
            // Check context_map first (for top-level placeholders)
            if matches!(context_map.get(id), Some(PlaceholderUse::Type)) {
                return true;
            }
            // For placeholders inside control blocks (not in context_map),
            // check if preceded by `as ` which indicates type position
            pending_type_suffix
        }
        Segment::BraceBlock { inner, .. } => has_type_placeholder_recursive(inner, context_map),
        Segment::Control { node, .. } => check_control_node_for_type_placeholder(node, context_map),
        _ => false,
    }
}

fn check_control_node_for_type_placeholder(
    node: &ControlNode,
    context_map: &HashMap<usize, PlaceholderUse>,
) -> bool {
    match node {
        ControlNode::If {
            then_branch,
            else_branch,
            ..
        } => {
            has_type_placeholder_recursive(then_branch, context_map)
                || else_branch
                    .as_ref()
                    .is_some_and(|b| has_type_placeholder_recursive(b, context_map))
        }
        ControlNode::IfLet {
            then_branch,
            else_branch,
            ..
        } => {
            has_type_placeholder_recursive(then_branch, context_map)
                || else_branch
                    .as_ref()
                    .is_some_and(|b| has_type_placeholder_recursive(b, context_map))
        }
        ControlNode::For { body, .. } => has_type_placeholder_recursive(body, context_map),
        ControlNode::While { body, .. } => has_type_placeholder_recursive(body, context_map),
        ControlNode::WhileLet { body, .. } => has_type_placeholder_recursive(body, context_map),
        ControlNode::Match { cases, .. } => cases
            .iter()
            .any(|MatchCase { body, .. }| has_type_placeholder_recursive(body, context_map)),
    }
}

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

    // Tests for is_type_position_suffix
    #[test]
    fn test_is_type_position_suffix_as() {
        use super::is_type_position_suffix;

        assert!(is_type_position_suffix(" as"));
        assert!(is_type_position_suffix("value as"));
        assert!(is_type_position_suffix("value as "));
        assert!(is_type_position_suffix("\tas"));
        assert!(!is_type_position_suffix("class"));
        assert!(!is_type_position_suffix("has"));
    }

    #[test]
    fn test_is_type_position_suffix_colon() {
        use super::is_type_position_suffix;

        // Parameter type: `param:` or `param: `
        assert!(is_type_position_suffix("param:"));
        assert!(is_type_position_suffix("param: "));
        assert!(is_type_position_suffix("(input: unknown, opts?:"));
        assert!(is_type_position_suffix("(input: unknown, opts?: "));

        // Return type: `):`
        assert!(is_type_position_suffix("):"));
        assert!(is_type_position_suffix("): "));
    }

    #[test]
    fn test_is_type_position_suffix_angle_bracket() {
        use super::is_type_position_suffix;

        // Type argument: `Array<`
        assert!(is_type_position_suffix("Array<"));
        assert!(is_type_position_suffix("Array< "));
        assert!(is_type_position_suffix("<"));
        // Note: `Map<string, ` ends with `, ` which is ambiguous (could be expression or type context)
        // so we intentionally don't match it to avoid false positives
        assert!(!is_type_position_suffix("Map<string, "));
    }

    #[test]
    fn test_is_type_position_suffix_union_intersection() {
        use super::is_type_position_suffix;

        // Union: `|`
        assert!(is_type_position_suffix("|"));
        assert!(is_type_position_suffix("string |"));
        assert!(is_type_position_suffix("string | "));

        // Intersection: `&`
        assert!(is_type_position_suffix("&"));
        assert!(is_type_position_suffix("A &"));
        assert!(is_type_position_suffix("A & "));
    }

    #[test]
    fn test_is_type_position_suffix_keywords() {
        use super::is_type_position_suffix;

        // extends
        assert!(is_type_position_suffix(" extends"));
        assert!(is_type_position_suffix("T extends"));
        assert!(is_type_position_suffix("T extends "));

        // implements
        assert!(is_type_position_suffix(" implements"));
        assert!(is_type_position_suffix("class Foo implements"));
        assert!(is_type_position_suffix("class Foo implements "));

        // keyof
        assert!(is_type_position_suffix(" keyof"));
        assert!(is_type_position_suffix("type K = keyof"));
        assert!(is_type_position_suffix("type K = keyof "));

        // infer
        assert!(is_type_position_suffix(" infer"));

        // is (type predicate)
        assert!(is_type_position_suffix(" is"));
        assert!(is_type_position_suffix("x is"));

        // asserts
        assert!(is_type_position_suffix(" asserts"));

        // satisfies
        assert!(is_type_position_suffix(" satisfies"));
    }

    #[test]
    fn test_is_type_position_suffix_false_positives() {
        use super::is_type_position_suffix;

        // Should NOT match these
        assert!(!is_type_position_suffix("const x = "));
        assert!(!is_type_position_suffix("return "));
        assert!(!is_type_position_suffix("if ("));
        assert!(!is_type_position_suffix("function foo("));
        assert!(!is_type_position_suffix("class "));
        assert!(!is_type_position_suffix("."));
        assert!(!is_type_position_suffix(","));  // Comma alone is ambiguous
        assert!(!is_type_position_suffix("="));
    }

    #[test]
    fn test_has_type_placeholder_recursive_with_colon() {
        // Test that type placeholders after `:` are detected
        let context_map = HashMap::new(); // Empty - simulating control block inner segments

        let segments = vec![
            Segment::Static("(input: unknown, opts?:".to_string()),
            Segment::Interpolation {
                id: 1,
                expr: quote! { OptsType },
            },
            Segment::Static("):".to_string()),
            Segment::Interpolation {
                id: 2,
                expr: quote! { ReturnType },
            },
            Segment::Static(" {}".to_string()),
        ];

        let result = has_type_placeholder_recursive(&segments, &context_map);
        assert!(result, "Should detect type placeholder after `?:`");
    }

    #[test]
    fn test_has_type_placeholder_recursive_in_control_block() {
        // Test that type placeholders inside control blocks are detected
        let context_map = HashMap::new();

        let control_body = vec![
            Segment::Static("export function fn(opts?:".to_string()),
            Segment::Interpolation {
                id: 5,
                expr: quote! { OptsType },
            },
            Segment::Static("):".to_string()),
            Segment::Interpolation {
                id: 6,
                expr: quote! { ReturnType },
            },
            Segment::Static(" {}".to_string()),
        ];

        let segments = vec![Segment::Control {
            id: 0,
            node: ControlNode::For {
                pat: quote! { item },
                iter: quote! { items },
                body: control_body,
            },
        }];

        let result = has_type_placeholder_recursive(&segments, &context_map);
        assert!(result, "Should detect type placeholder inside control block");
    }
}
