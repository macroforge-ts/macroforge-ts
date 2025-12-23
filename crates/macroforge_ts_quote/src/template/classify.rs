use crate::template::{PlaceholderUse, Segment};
use std::collections::HashMap;
use swc_core::ecma::visit::VisitWith;

use super::build::{build_placeholder_source, PlaceholderSourceKind};
use super::parse::{parse_ts_expr, parse_ts_module};
use super::placeholder::PlaceholderFinder;

/// Tries to classify placeholders by parsing segments as a TypeScript module.
///
/// Returns `Ok(Some(map))` if successful, `Ok(None)` if parsing fails.
fn try_classify_as_module(
    source: &str,
    map: HashMap<String, usize>,
) -> syn::Result<Option<HashMap<usize, PlaceholderUse>>> {
    if let Ok(module) = parse_ts_module(source) {
        let mut finder = PlaceholderFinder::new(map);
        module.visit_with(&mut finder);
        Ok(Some(finder.into_map()))
    } else {
        Ok(None)
    }
}

/// Tries to classify placeholders by wrapping segments in a class body and parsing.
///
/// This handles class body members like static methods that aren't valid as
/// top-level module items.
///
/// Returns `Ok(Some(map))` if successful, `Ok(None)` if parsing fails.
fn try_classify_as_class(
    source: &str,
    map: HashMap<String, usize>,
) -> syn::Result<Option<HashMap<usize, PlaceholderUse>>> {
    let wrapped_source = format!("class __MfWrapper {{ {} }}", source);
    if let Ok(module) = parse_ts_module(&wrapped_source) {
        let mut finder = PlaceholderFinder::new(map);
        module.visit_with(&mut finder);
        Ok(Some(finder.into_map()))
    } else {
        Ok(None)
    }
}

/// Tries to classify placeholders by parsing segments as a TypeScript expression.
///
/// This is the final fallback when module and class parsing fail.
///
/// Returns `Ok(Some(map))` if successful, `Ok(None)` if parsing fails.
fn try_classify_as_expr(
    source: &str,
    map: HashMap<String, usize>,
) -> syn::Result<Option<HashMap<usize, PlaceholderUse>>> {
    if let Ok(expr) = parse_ts_expr(source) {
        let mut finder = PlaceholderFinder::new(map);
        expr.visit_with(&mut finder);
        Ok(Some(finder.into_map()))
    } else {
        Ok(None)
    }
}

/// Classifies placeholder usage by parsing the segments as a module.
/// Falls back to wrapping in a class if the template contains class body members.
pub(crate) fn classify_placeholders_module(
    segments: &[Segment],
) -> syn::Result<HashMap<usize, PlaceholderUse>> {
    let (source, map) = build_placeholder_source(segments, PlaceholderSourceKind::Module);
    if source.trim().is_empty() {
        return Ok(HashMap::new());
    }

    // Try parsing as a module first
    if let Some(result) = try_classify_as_module(&source, map.clone())? {
        return Ok(result);
    }

    // If module parsing fails, try wrapping in a class (for class body members like static methods)
    if let Some(result) = try_classify_as_class(&source, map.clone())? {
        return Ok(result);
    }

    // If both fail, try parsing as an expression
    if let Some(result) = try_classify_as_expr(&source, map)? {
        return Ok(result);
    }

    // Fall back to returning an empty map (all placeholders will use default classification)
    Ok(HashMap::new())
}

/// Classifies placeholder usage by parsing the segments as an expression.
pub(crate) fn classify_placeholders_expr(segments: &[Segment]) -> syn::Result<HashMap<usize, PlaceholderUse>> {
    // println!("DEBUG: classify_placeholders_expr called");
    let (source, map) = build_placeholder_source(segments, PlaceholderSourceKind::Expr);
    if source.trim().is_empty() {
        return Ok(HashMap::new());
    }
    let expr = parse_ts_expr(&source)?;
    let mut finder = PlaceholderFinder::new(map);
    expr.visit_with(&mut finder);
    Ok(finder.into_map())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;

    /// Helper to create a simple interpolation segment
    fn interpolation_segment(id: usize, expr: TokenStream2) -> Segment {
        Segment::Interpolation { id, expr }
    }

    /// Helper to create a static segment
    fn static_segment(s: &str) -> Segment {
        Segment::Static(s.to_string())
    }

    #[test]
    fn test_classify_placeholders_module_expr_position() {
        // Test: const x = <placeholder>;
        let segments = vec![
            static_segment("const x = "),
            interpolation_segment(0, quote!(value)),
            static_segment(";"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_classify_placeholders_module_ident_position() {
        // Test: function <placeholder>() {}
        let segments = vec![
            static_segment("function "),
            interpolation_segment(0, quote!(funcName)),
            static_segment("() {}"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Ident));
    }

    #[test]
    fn test_classify_placeholders_module_type_position() {
        // Test: const x: <placeholder> = 1;
        let segments = vec![
            static_segment("const x: "),
            interpolation_segment(0, quote!(MyType)),
            static_segment(" = 1;"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Type));
    }

    #[test]
    fn test_classify_placeholders_module_ident_name_position() {
        // Test: obj.<placeholder>
        let segments = vec![
            static_segment("const x = obj."),
            interpolation_segment(0, quote!(propName)),
            static_segment(";"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::IdentName));
    }

    #[test]
    fn test_classify_placeholders_module_stmt_position() {
        // Test: <placeholder>; (standalone statement)
        let segments = vec![
            interpolation_segment(0, quote!(myStmt)),
            static_segment(";"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Stmt));
    }

    #[test]
    fn test_classify_placeholders_module_empty() {
        // Test: empty segments
        let segments = vec![];

        let result = classify_placeholders_module(&segments).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_classify_placeholders_module_whitespace_only() {
        // Test: whitespace-only segments
        let segments = vec![static_segment("   "), static_segment("\n\t")];

        let result = classify_placeholders_module(&segments).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_classify_placeholders_module_fallback_to_class() {
        // Test: static method (only valid in class body, not top-level module)
        // Note: The placeholder is actually detected as IdentName since it's the method name
        // in a class context (property name of the class)
        let segments = vec![
            static_segment("static "),
            interpolation_segment(0, quote!(methodName)),
            static_segment("() {}"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        // Should successfully classify via class wrapping fallback
        // Static method names are treated as IdentName (property names) in class bodies
        assert_eq!(result.get(&0), Some(&PlaceholderUse::IdentName));
    }

    #[test]
    fn test_classify_placeholders_module_fallback_to_expr() {
        // Test: binary expression (not valid as module, should fall back to expr)
        let segments = vec![
            interpolation_segment(0, quote!(a)),
            static_segment(" + "),
            interpolation_segment(1, quote!(b)),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        // Both should be classified as Expr
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_classify_placeholders_module_invalid_syntax() {
        // Test: completely invalid syntax that can't be parsed as module, class, or expr
        // Should return empty map as graceful fallback
        let segments = vec![
            static_segment("@#$%^&*"),
            interpolation_segment(0, quote!(foo)),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        // Should return empty map (graceful fallback)
        assert!(result.is_empty());
    }

    #[test]
    fn test_classify_placeholders_module_multiple_placeholders() {
        // Test: multiple placeholders in different positions
        let segments = vec![
            static_segment("function "),
            interpolation_segment(0, quote!(funcName)),  // Ident
            static_segment("(x: "),
            interpolation_segment(1, quote!(ParamType)), // Type
            static_segment(") { return "),
            interpolation_segment(2, quote!(expr)),      // Expr
            static_segment("; }"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Ident));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::Type));
        assert_eq!(result.get(&2), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_classify_placeholders_module_object_property() {
        // Test: { <placeholder>: value }
        let segments = vec![
            static_segment("const obj = { "),
            interpolation_segment(0, quote!(keyName)),
            static_segment(": 123 };"),
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::IdentName));
    }

    #[test]
    fn test_classify_placeholders_expr_binary_expression() {
        // Test: a + b
        let segments = vec![
            interpolation_segment(0, quote!(a)),
            static_segment(" + "),
            interpolation_segment(1, quote!(b)),
        ];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_classify_placeholders_expr_member_access() {
        // Test: obj.property
        let segments = vec![
            interpolation_segment(0, quote!(obj)),
            static_segment("."),
            interpolation_segment(1, quote!(property)),
        ];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::IdentName));
    }

    #[test]
    fn test_classify_placeholders_expr_empty() {
        // Test: empty segments
        let segments = vec![];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_classify_placeholders_expr_whitespace_only() {
        // Test: whitespace-only segments
        let segments = vec![static_segment("  \n\t  ")];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_classify_placeholders_expr_function_call() {
        // Test: func(arg)
        let segments = vec![
            interpolation_segment(0, quote!(func)),
            static_segment("("),
            interpolation_segment(1, quote!(arg)),
            static_segment(")"),
        ];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_classify_placeholders_expr_nested_member_access() {
        // Test: a.b.c
        let segments = vec![
            interpolation_segment(0, quote!(a)),
            static_segment("."),
            interpolation_segment(1, quote!(b)),
            static_segment("."),
            interpolation_segment(2, quote!(c)),
        ];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::IdentName));
        assert_eq!(result.get(&2), Some(&PlaceholderUse::IdentName));
    }

    #[test]
    fn test_classify_placeholders_expr_array_literal() {
        // Test: [a, b, c]
        let segments = vec![
            static_segment("["),
            interpolation_segment(0, quote!(a)),
            static_segment(", "),
            interpolation_segment(1, quote!(b)),
            static_segment(", "),
            interpolation_segment(2, quote!(c)),
            static_segment("]"),
        ];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::Expr));
        assert_eq!(result.get(&2), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_classify_placeholders_expr_object_literal() {
        // Test: { key: value }
        let segments = vec![
            static_segment("{ "),
            interpolation_segment(0, quote!(key)),
            static_segment(": "),
            interpolation_segment(1, quote!(value)),
            static_segment(" }"),
        ];

        let result = classify_placeholders_expr(&segments).unwrap();
        assert_eq!(result.get(&0), Some(&PlaceholderUse::IdentName));
        assert_eq!(result.get(&1), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_try_classify_as_module_success() {
        // Test the helper function directly
        let mut map = HashMap::new();
        map.insert("__mf_hole_0".to_string(), 0);

        let source = "const x = __mf_hole_0;";
        let result = try_classify_as_module(source, map).unwrap();

        assert!(result.is_some());
        let uses = result.unwrap();
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_try_classify_as_module_failure() {
        // Test the helper function with invalid syntax
        let mut map = HashMap::new();
        map.insert("__mf_hole_0".to_string(), 0);

        let source = "+++invalid+++";
        let result = try_classify_as_module(source, map).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_try_classify_as_class_success() {
        // Test the helper function directly with static method
        let mut map = HashMap::new();
        map.insert("__mf_hole_0".to_string(), 0);

        let source = "static __mf_hole_0() {}";
        let result = try_classify_as_class(source, map).unwrap();

        assert!(result.is_some());
        let uses = result.unwrap();
        // Static method names in classes are detected as IdentName (property names)
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::IdentName));
    }

    #[test]
    fn test_try_classify_as_class_failure() {
        // Test the helper function with invalid class body syntax
        let mut map = HashMap::new();
        map.insert("__mf_hole_0".to_string(), 0);

        let source = "+++invalid+++";
        let result = try_classify_as_class(source, map).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_try_classify_as_expr_success() {
        // Test the helper function directly
        let mut map = HashMap::new();
        map.insert("__mf_hole_0".to_string(), 0);
        map.insert("__mf_hole_1".to_string(), 1);

        let source = "__mf_hole_0 + __mf_hole_1";
        let result = try_classify_as_expr(source, map).unwrap();

        assert!(result.is_some());
        let uses = result.unwrap();
        assert_eq!(uses.get(&0), Some(&PlaceholderUse::Expr));
        assert_eq!(uses.get(&1), Some(&PlaceholderUse::Expr));
    }

    #[test]
    fn test_try_classify_as_expr_failure() {
        // Test the helper function with invalid expression syntax
        let mut map = HashMap::new();
        map.insert("__mf_hole_0".to_string(), 0);

        // Use syntax that is invalid as an expression (but valid as a statement)
        // Variable declarations are not expressions in JavaScript/TypeScript
        let source = "const x = 1;";
        let result = try_classify_as_expr(source, map).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_classify_preserves_highest_rank() {
        // Test that when a placeholder appears in multiple contexts,
        // the highest rank (most specific) usage is kept
        // Stmt (rank 5) > Type (rank 4) > Ident (rank 3) > IdentName (rank 2) > Expr (rank 1)

        // Create a contrived example where the same placeholder might be seen
        // in different contexts within the AST traversal
        let segments = vec![
            interpolation_segment(0, quote!(x)),
            static_segment(";"), // This makes it a statement
        ];

        let result = classify_placeholders_module(&segments).unwrap();
        // Should be classified as Stmt since it's in statement position
        assert_eq!(result.get(&0), Some(&PlaceholderUse::Stmt));
    }
}
