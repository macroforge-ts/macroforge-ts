use convert_case::{Case, Casing};

use crate::builtin::serde::{TypeCategory, get_foreign_types, split_top_level_union};

/// Check if a TypeScript type is a primitive type
pub fn is_primitive_type(ts_type: &str) -> bool {
    matches!(
        ts_type.trim(),
        "string" | "number" | "boolean" | "bigint" | "null" | "undefined"
    )
}

/// Check if a TypeScript type is numeric
pub fn is_numeric_type(ts_type: &str) -> bool {
    matches!(ts_type.trim(), "number" | "bigint")
}

/// Check if a TypeScript type is nullable (contains `| null` or `| undefined`)
/// Like Rust's Option<T>, these types default to null.
pub fn is_nullable_type(ts_type: &str) -> bool {
    let normalized = ts_type.replace(' ', "");
    normalized.contains("|null") || normalized.contains("|undefined")
}

/// Check if a type name contains generic parameters (e.g., "RecordLink<Service>")
/// This is used to detect generic type instantiations that need special handling.
pub fn is_generic_type(type_name: &str) -> bool {
    type_name.contains('<') && type_name.contains('>')
}

/// Extracts base type and type arguments from a generic type.
/// "RecordLink<Service>" -> Some(("RecordLink", "Service"))
/// "Map<string, number>" -> Some(("Map", "string, number"))
/// "User" -> None
pub fn parse_generic_type(type_name: &str) -> Option<(&str, &str)> {
    let open = type_name.find('<')?;
    let close = type_name.rfind('>')?;
    if open < close {
        let base = &type_name[..open];
        let args = &type_name[open + 1..close];
        Some((base.trim(), args.trim()))
    } else {
        None
    }
}

/// Returns whether a type can have a default value generated for it.
///
/// Always returns `true` because all types are assumed to implement Default:
/// primitives and collections have built-in defaults, and custom types are
/// assumed to provide a `{typeName}DefaultValue()` standalone function
/// (following Rust's `derive(Default)` philosophy). This function exists
/// as a named predicate for readability.
pub fn has_known_default(_ts_type: &str) -> bool {
    true
}

/// Get default value for a TypeScript type
pub fn get_type_default(ts_type: &str) -> String {
    let t = ts_type.trim();

    // Check for foreign type default first
    let foreign_types = get_foreign_types();
    let ft_match = TypeCategory::match_foreign_type(t, &foreign_types);
    // Note: Warnings from near-matches are handled by serialize/deserialize macros
    // which have access to diagnostics
    if let Some(ft) = ft_match.config
        && let Some(ref default_expr) = ft.default_expr
    {
        // Wrap the expression in an IIFE if it's a function
        // Foreign type defaults are expected to be functions: () => DateTime.now()
        // Rewrite namespace references to use generated aliases
        let rewritten = crate::builtin::serde::rewrite_expression_namespaces(default_expr);
        return format!("({})()", rewritten);
    }

    // Nullable first (like Rust's Option::default() -> None)
    if is_nullable_type(t) {
        return "null".to_string();
    }

    // Object literal types: { [key: string]: number }, { foo: string }, etc.
    // Must be checked before union splitting since braces can contain pipes.
    if t.starts_with('{') {
        return "{}".to_string();
    }

    // Handle union types (e.g., string | Account, "Estimate" | "Invoice")
    // Nullable unions (T | null, T | undefined) are already handled above.
    if let Some(parts) = split_top_level_union(t) {
        // 1. If any member is a primitive, use that primitive's default
        for part in &parts {
            if is_primitive_type(part) {
                return get_type_default(part);
            }
        }
        // 2. If any member is a literal, use the first literal
        for part in &parts {
            let p = part.trim();
            if (p.starts_with('"') && p.ends_with('"'))
                || (p.starts_with('\'') && p.ends_with('\''))
                || (p.starts_with('`') && p.ends_with('`'))
                || p.parse::<f64>().is_ok()
                || matches!(p, "true" | "false")
            {
                return get_type_default(p);
            }
        }
        // 3. Union of only custom types — default via first member
        return get_type_default(parts[0]);
    }

    match t {
        "string" => r#""""#.to_string(),
        "number" => "0".to_string(),
        "boolean" => "false".to_string(),
        "bigint" => "0n".to_string(),
        t if t.ends_with("[]") => "[]".to_string(),
        t if t.starts_with("Array<") => "[]".to_string(),
        t if t.starts_with("Map<") => "new Map()".to_string(),
        t if t.starts_with("Set<") => "new Set()".to_string(),
        "Date" => "new Date()".to_string(),
        // Generic type instantiations like RecordLink<Service>
        t if is_generic_type(t) => {
            if let Some((base, args)) = parse_generic_type(t) {
                format!("{}DefaultValue<{}>()", base.to_case(Case::Camel), args)
            } else {
                // Fallback: shouldn't happen if is_generic_type returned true
                format_default_call(t)
            }
        }
        // String literal types: "active", 'pending', `template`
        t if (t.starts_with('"') && t.ends_with('"'))
            || (t.starts_with('\'') && t.ends_with('\''))
            || (t.starts_with('`') && t.ends_with('`')) =>
        {
            t.to_string()
        }
        // Number literal types: 42, 3.14
        t if t.parse::<f64>().is_ok() => t.to_string(),
        // Boolean literal types
        "true" | "false" => t.to_string(),
        // Unknown types: assume they implement Default trait
        type_name => format_default_call(type_name),
    }
}

fn format_default_call(type_name: &str) -> String {
    format!("{}DefaultValue()", type_name.to_case(Case::Camel))
}
