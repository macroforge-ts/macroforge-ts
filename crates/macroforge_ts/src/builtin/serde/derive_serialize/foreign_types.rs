use super::super::{TypeCategory, get_foreign_types, rewrite_expression_namespaces};

/// Tries to generate a composite serialize expression for types where a foreign
/// type is nested inside array, set, or nullable wrappers (e.g., `DateTime.Utc[]`,
/// `DateTime.Utc | null`).
///
/// Similar to `try_composite_foreign_deserialize` in the deserialize module.
pub(crate) fn try_composite_foreign_serialize(ts_type: &str) -> Option<String> {
    let foreign_types = get_foreign_types();

    // Split by | and classify parts
    let parts: Vec<&str> = ts_type
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let has_null = parts.contains(&"null");
    let has_undefined = parts.contains(&"undefined");
    let non_null: Vec<&str> = parts
        .iter()
        .filter(|s| **s != "null" && **s != "undefined")
        .copied()
        .collect();

    // Only handle single non-null type for now
    if non_null.len() != 1 {
        return None;
    }

    let core = non_null[0];

    // Check for array/set types
    let (container, elem_type) = if let Some(inner) = core.strip_suffix("[]") {
        ("array", inner.trim())
    } else if let Some(rest) = core.strip_prefix("Array<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("array", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Set<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("set", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Map<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::super::find_top_level_comma(inner) {
                ("map", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Record<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::super::find_top_level_comma(inner) {
                ("record", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else {
        ("none", core)
    };

    // Try matching the element type against foreign types
    let ft_match = TypeCategory::match_foreign_type(elem_type, &foreign_types);
    let ser_expr = ft_match.config.and_then(|ft| ft.serialize_expr.clone())?;
    let rewritten = rewrite_expression_namespaces(&ser_expr);

    match (has_null, has_undefined, container) {
        (_, _, "array") if has_null || has_undefined => Some(format!(
            "(arr) => arr == null ? null : arr.map(item => ({rewritten})(item))"
        )),
        (_, _, "array") => Some(format!("(arr) => arr.map(item => ({rewritten})(item))")),
        (_, _, "set") if has_null || has_undefined => Some(format!(
            "(s) => s == null ? null : Array.from(s).map(item => ({rewritten})(item))"
        )),
        (_, _, "set") => Some(format!(
            "(s) => Array.from(s).map(item => ({rewritten})(item))"
        )),
        (_, _, "map") if has_null || has_undefined => Some(format!(
            "(m) => m == null ? null : Object.fromEntries(Array.from(m.entries()).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (_, _, "map") => Some(format!(
            "(m) => Object.fromEntries(Array.from(m.entries()).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (_, _, "record") if has_null || has_undefined => Some(format!(
            "(obj) => obj == null ? null : Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (_, _, "record") => Some(format!(
            "(obj) => Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (true, _, "none") => Some(format!("(raw) => raw === null ? null : ({rewritten})(raw)")),
        (_, true, "none") => Some(format!(
            "(raw) => raw === undefined ? undefined : ({rewritten})(raw)"
        )),
        _ => None, // Direct match should have been caught at field level
    }
}
