use crate::template::{PlaceholderUse, Segment};
use std::collections::HashMap;
use swc_core::ecma::visit::VisitWith;

use super::build::{build_placeholder_source, PlaceholderSourceKind};
use super::parse::{parse_ts_expr, parse_ts_module};
use super::placeholder::PlaceholderFinder;

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
    if let Ok(module) = parse_ts_module(&source) {
        let mut finder = PlaceholderFinder::new(map);
        module.visit_with(&mut finder);
        return Ok(finder.into_map());
    }

    // If module parsing fails, try wrapping in a class (for class body members like static methods)
    let wrapped_source = format!("class __MfWrapper {{ {} }}", source);
    if let Ok(module) = parse_ts_module(&wrapped_source) {
        let mut finder = PlaceholderFinder::new(map);
        module.visit_with(&mut finder);
        return Ok(finder.into_map());
    }

    // If both fail, try parsing as an expression
    if let Ok(expr) = parse_ts_expr(&source) {
        let mut finder = PlaceholderFinder::new(map);
        expr.visit_with(&mut finder);
        return Ok(finder.into_map());
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
