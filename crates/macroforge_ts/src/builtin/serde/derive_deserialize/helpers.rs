use crate::swc_ecma_ast::Expr;
use crate::ts_syn::{TsSynError, parse_ts_expr};

use convert_case::{Case, Casing};

use super::super::{TypeCategory, get_foreign_types, rewrite_expression_namespaces};
use super::types::SerdeValueKind;
use crate::host::ForeignTypeConfig;
use crate::ts_syn::abi::ir::type_alias::{TypeBody, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry};

/// Determines whether a TypeScript type can accept a raw `string` value,
/// using the type registry and foreign type configs to resolve types.
///
/// When true, the generated `Deserialize` wrapper must NOT call `JSON.parse`
/// on string inputs -- the string IS the value, not a JSON-encoded payload.
pub(super) fn type_accepts_string(
    type_name: &str,
    registry: Option<&TypeRegistry>,
    foreign_types: &[ForeignTypeConfig],
) -> bool {
    // Primitive string keyword
    if type_name == "string" {
        return true;
    }

    // Check foreign types -- their hasShape expr tells us what values they accept.
    // e.g. Utc has `hasShape: (v) => typeof v === "string"` -> accepts strings.
    for ft in foreign_types {
        if ft.get_type_name() == type_name
            || ft.name == type_name
            || ft.aliases.iter().any(|a| a.name == type_name)
        {
            if let Some(has_shape) = &ft.has_shape_expr
                && has_shape.contains("typeof")
                && has_shape.contains("\"string\"")
            {
                return true;
            }
            // No hasShape or it doesn't check for string -> not a string type
            return false;
        }
    }

    // Check the type registry
    let registry = match registry {
        Some(r) => r,
        None => {
            return false;
        }
    };

    let entry = match registry.get(type_name) {
        Some(e) => e,
        None => {
            // Try qualified lookup for ambiguous names
            match registry.get_all(type_name).next() {
                Some(e) => e,
                None => return false,
            }
        }
    };

    match &entry.definition {
        TypeDefinitionIR::TypeAlias(alias) => match &alias.body {
            // Union: check if any member is string, a string literal, or a foreign string type
            TypeBody::Union(members) => members.iter().any(|m| match &m.kind {
                TypeMemberKind::TypeRef(t) => type_accepts_string(t, Some(registry), foreign_types),
                TypeMemberKind::Literal(lit) => lit.starts_with('"') || lit.starts_with('\''),
                TypeMemberKind::Object { .. } => false,
            }),
            // Simple alias: recurse
            TypeBody::Alias(target) => type_accepts_string(target, Some(registry), foreign_types),
            _ => false,
        },
        // Enums with string members accept strings
        TypeDefinitionIR::Enum(e) => e.variants.iter().any(|v| v.value.is_string()),
        // Classes and interfaces are always objects
        TypeDefinitionIR::Class(_) | TypeDefinitionIR::Interface(_) => false,
    }
}

pub(super) fn parse_default_expr(expr_src: &str) -> Result<Expr, TsSynError> {
    let expr = parse_ts_expr(expr_src)?;
    if matches!(*expr, Expr::Ident(_)) {
        let literal_src = format!("{expr_src:?}");
        return parse_ts_expr(&literal_src).map(|expr| *expr);
    }
    Ok(*expr)
}

pub(super) fn is_ts_primitive_keyword(s: &str) -> bool {
    matches!(
        s.trim(),
        "string" | "number" | "boolean" | "bigint" | "null" | "undefined"
    )
}

pub(super) fn is_ts_literal(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    if matches!(s, "true" | "false") {
        return true;
    }

    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        return true;
    }

    // Very small heuristic: numeric / bigint literals
    if let Some(digits) = s.strip_suffix('n') {
        return !digits.is_empty()
            && digits
                .chars()
                .all(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '+');
    }

    s.chars()
        .all(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '+' || c == '.')
}

pub(super) fn is_union_of_primitive_like(s: &str) -> bool {
    if !s.contains('|') {
        return false;
    }
    s.split('|').all(|part| {
        let part = part.trim();
        is_ts_primitive_keyword(part) || is_ts_literal(part)
    })
}

pub(super) fn classify_serde_value_kind(ts_type: &str) -> SerdeValueKind {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive => SerdeValueKind::PrimitiveLike,
        TypeCategory::Date => SerdeValueKind::Date,
        TypeCategory::Nullable(inner) => match classify_serde_value_kind(&inner) {
            SerdeValueKind::Date => SerdeValueKind::NullableDate,
            SerdeValueKind::PrimitiveLike => SerdeValueKind::PrimitiveLike,
            _ => SerdeValueKind::Other,
        },
        TypeCategory::Optional(inner) => classify_serde_value_kind(&inner),
        _ => {
            if is_union_of_primitive_like(ts_type) {
                SerdeValueKind::PrimitiveLike
            } else {
                SerdeValueKind::Other
            }
        }
    }
}

/// If the given type string is a Serializable type, return its name.
/// Returns None for primitives, Date, and other non-serializable types.
pub(super) fn get_serializable_type_name(ts_type: &str) -> Option<String> {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Serializable(name) => Some(name),
        _ => None,
    }
}

/// Tries to generate a composite deserialize expression for types where a foreign
/// type is nested inside nullable or array wrappers (e.g., `Utc[] | null`, `Utc | null`).
///
/// Splits the type by `|`, strips `null`/`undefined`, detects `[]` arrays,
/// and matches the element type against configured foreign types.
pub(super) fn try_composite_foreign_deserialize(ts_type: &str) -> Option<String> {
    let foreign_types = get_foreign_types();

    // Split by | and classify parts
    let parts: Vec<&str> = ts_type
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let is_nullable = parts.iter().any(|s| *s == "null" || *s == "undefined");
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

    // Check for container types
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
    let deser_expr = ft_match.config.and_then(|ft| ft.deserialize_expr.clone())?;
    let rewritten = rewrite_expression_namespaces(&deser_expr);

    match (is_nullable, container) {
        (true, "array") => Some(format!(
            "(raw) => raw === null ? null : (raw as any[]).map(item => ({rewritten})(item))"
        )),
        (false, "array") => Some(format!(
            "(raw) => (raw as any[]).map(item => ({rewritten})(item))"
        )),
        (true, "set") => Some(format!(
            "(raw) => raw === null ? null : new Set((raw as any[]).map(item => ({rewritten})(item)))"
        )),
        (false, "set") => Some(format!(
            "(raw) => new Set((raw as any[]).map(item => ({rewritten})(item)))"
        )),
        (true, "map") => Some(format!(
            "(raw) => raw === null ? null : new Map(Object.entries(raw as Record<string, unknown>).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (false, "map") => Some(format!(
            "(raw) => new Map(Object.entries(raw as Record<string, unknown>).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (true, "record") => Some(format!(
            "(raw) => raw === null ? null : Object.fromEntries(Object.entries(raw as Record<string, unknown>).map(([k, v]) => [k, ({rewritten})(v)])) as any"
        )),
        (false, "record") => Some(format!(
            "(raw) => Object.fromEntries(Object.entries(raw as Record<string, unknown>).map(([k, v]) => [k, ({rewritten})(v)])) as any"
        )),
        (true, "none") => Some(format!("(raw) => raw === null ? null : ({rewritten})(raw)")),
        (false, "none") => None, // Direct match should have been caught already
        _ => None,
    }
}

/// Extracts the base type name from a potentially generic type.
/// For example: "User<T>" -> "User", "Map<string, number>" -> "Map"
pub(super) fn extract_base_type(ts_type: &str) -> String {
    if let Some(idx) = ts_type.find('<') {
        ts_type[..idx].to_string()
    } else {
        ts_type.to_string()
    }
}

/// For inline object types in tagged unions, extract the tag value from the type literal.
/// e.g. `{ variant: 'GlobalAdmin' }` with tag_field "variant" -> Some("GlobalAdmin")
/// Returns None if the type is not an inline object or doesn't contain the tag field.
#[allow(dead_code)]
pub(super) fn extract_inline_tag_value(ts_type: &str, tag_field: &str) -> Option<String> {
    let trimmed = ts_type.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return None;
    }
    // Look for: variant: 'Value' or variant: "Value"
    let pattern = format!("{}:", tag_field);
    let pos = trimmed.find(&pattern)?;
    let after_colon = trimmed[pos + pattern.len()..].trim();
    // Extract the quoted value
    let quote = after_colon.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    let end = after_colon[1..].find(quote)?;
    Some(after_colon[1..1 + end].to_string())
}

/// Check if a type string is an inline object literal (starts with `{`).
#[allow(dead_code)]
pub(super) fn is_inline_object_type(ts_type: &str) -> bool {
    ts_type.trim().starts_with('{')
}

/// Generates the DeserializeWithContext function name for a nested deserializable type.
/// For example: "User" -> "userDeserializeWithContext"
pub(super) fn nested_deserialize_fn_name(type_name: &str) -> String {
    // Strip generic parameters before camelCase conversion (see nested_serialize_fn_name)
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}DeserializeWithContext", base.to_case(Case::Camel))
}

/// Generates the public Deserialize function name (result-returning) for a nested deserializable type.
/// For example: "User" -> "userDeserialize"
pub(super) fn nested_deserialize_result_fn_name(type_name: &str) -> String {
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}Deserialize", base.to_case(Case::Camel))
}

/// Generates the HasShape function name for a type.
/// For example: "DailyRecurrenceRule" -> "dailyRecurrenceRuleHasShape"
pub(super) fn nested_has_shape_fn_name(type_name: &str) -> String {
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}HasShape", base.to_case(Case::Camel))
}

/// Get JavaScript typeof string for a TypeScript primitive type
#[allow(dead_code)]
pub(super) fn get_js_typeof(ts_type: &str) -> &'static str {
    match ts_type.trim() {
        "string" => "string",
        "number" => "number",
        "boolean" => "boolean",
        "bigint" => "bigint",
        _ => "object",
    }
}
