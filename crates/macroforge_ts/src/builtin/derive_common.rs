//! # Shared Utilities for Derive Macros
//!
//! This module provides common functionality used by multiple derive macros,
//! including:
//!
//! - **Field options parsing**: `CompareFieldOptions`, `DefaultFieldOptions`
//! - **Type utilities**: Type checking and default value generation
//! - **Decorator parsing**: Flag extraction and named argument parsing
//!
//! ## Field Options
//!
//! Many macros support field-level customization through decorators:
//!
//! ```typescript
//! /** @derive(PartialEq, Hash, Default) */
//! class User {
//!     /** @partialEq({ skip: true }) @hash({ skip: true }) */
//!     cachedValue: number;
//!
//!     /** @default("guest") */
//!     name: string;
//! }
//! ```
//!
//! ## Type Defaults (Rust-like Philosophy)
//!
//! Like Rust's `Default` trait, this module assumes all types implement
//! default values:
//!
//! | Type | Default Value |
//! |------|---------------|
//! | `string` | `""` |
//! | `number` | `0` |
//! | `boolean` | `false` |
//! | `bigint` | `0n` |
//! | `T[]` | `[]` |
//! | `Map<K,V>` | `new Map()` |
//! | `Set<T>` | `new Set()` |
//! | `Date` | `new Date()` |
//! | `T \| null` | `null` |
//! | `CustomType` | `CustomType.defaultValue()` |

use convert_case::{Case, Casing};

use crate::builtin::serde::{TypeCategory, get_foreign_types};
use crate::ts_syn::abi::DecoratorIR;

/// Options parsed from field-level decorators for comparison macros
/// Supports @partialEq(skip), @hash(skip), @ord(skip)
#[derive(Default, Clone)]
pub struct CompareFieldOptions {
    pub skip: bool,
}

impl CompareFieldOptions {
    /// Parse field options from decorators for a specific attribute name
    pub fn from_decorators(decorators: &[DecoratorIR], attr_name: &str) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case(attr_name) {
                continue;
            }
            let args = decorator.args_src.trim();
            if has_flag(args, "skip") {
                opts.skip = true;
            }
        }
        opts
    }
}

// ============================================================================
// Field Options for Default Macro
// ============================================================================

/// Options parsed from @default decorator on fields
#[derive(Default, Clone)]
pub struct DefaultFieldOptions {
    /// The default value expression (e.g., "0", "\"\"", "[]")
    pub value: Option<String>,
    /// Whether this field has a @default decorator
    pub has_default: bool,
}

impl DefaultFieldOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("default") {
                continue;
            }
            opts.has_default = true;
            let args = decorator.args_src.trim();

            // Check for @default("value") or @default({ value: "..." })
            if let Some(value) = extract_default_value(args) {
                opts.value = Some(value);
            } else if !args.is_empty() {
                // Treat the args directly as the value if not empty
                // This handles @default(0), @default([]), @default(false), etc.
                opts.value = Some(args.to_string());
            }
        }
        opts
    }
}

/// Extract default value from decorator arguments
fn extract_default_value(args: &str) -> Option<String> {
    // Try named form: { value: "..." }
    if let Some(value) = extract_named_string(args, "value") {
        return Some(value);
    }

    // Try direct string literal: "..."
    if let Some(value) = parse_string_literal(args) {
        return Some(format!("\"{}\"", value));
    }

    None
}

// ============================================================================
// Type Utilities
// ============================================================================

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

/// Check if a type has a known default value.
/// All types are assumed to implement Default - primitives/collections have built-in defaults,
/// unknown types are assumed to have a defaultValue() static method (Rust's derive(Default) philosophy).
pub fn has_known_default(_ts_type: &str) -> bool {
    // All types are assumed to implement Default
    // - Primitives/collections have built-in defaults
    // - Unknown types are assumed to have defaultValue() method
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
    if let Some(ft) = ft_match.config {
        if let Some(ref default_expr) = ft.default_expr {
            // Wrap the expression in an IIFE if it's a function
            // Foreign type defaults are expected to be functions: () => DateTime.now()
            return format!("({})()", default_expr);
        }
    }

    // Nullable first (like Rust's Option::default() -> None)
    if is_nullable_type(t) {
        return "null".to_string();
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

// ============================================================================
// Helper functions (shared with other modules)
// ============================================================================

pub fn has_flag(args: &str, flag: &str) -> bool {
    if flag_explicit_false(args, flag) {
        return false;
    }

    args.split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|token| token.eq_ignore_ascii_case(flag))
}

fn flag_explicit_false(args: &str, flag: &str) -> bool {
    let lower = args.to_ascii_lowercase();
    let condensed: String = lower.chars().filter(|c| !c.is_whitespace()).collect();
    condensed.contains(&format!("{flag}:false")) || condensed.contains(&format!("{flag}=false"))
}

pub fn extract_named_string(args: &str, name: &str) -> Option<String> {
    let lower = args.to_ascii_lowercase();
    let idx = lower.find(name)?;
    let remainder = &args[idx + name.len()..];
    let remainder = remainder.trim_start();

    if remainder.starts_with(':') || remainder.starts_with('=') {
        let value = remainder[1..].trim_start();
        return parse_string_literal(value);
    }

    if remainder.starts_with('(')
        && let Some(close) = remainder.rfind(')')
    {
        let inner = remainder[1..close].trim();
        return parse_string_literal(inner);
    }

    None
}

fn parse_string_literal(input: &str) -> Option<String> {
    let trimmed = input.trim();
    let mut chars = trimmed.chars();
    let quote = chars.next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let mut escaped = false;
    let mut buf = String::new();
    for c in chars {
        if escaped {
            buf.push(c);
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if c == quote {
            return Some(buf);
        }
        buf.push(c);
    }
    None
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ts_syn::abi::SpanIR;

    fn span() -> SpanIR {
        SpanIR::new(0, 0)
    }

    fn make_decorator(name: &str, args: &str) -> DecoratorIR {
        DecoratorIR {
            name: name.into(),
            args_src: args.into(),
            span: span(),
            node: None,
        }
    }

    #[test]
    fn test_compare_field_skip() {
        let decorator = make_decorator("partialEq", "skip");
        let opts = CompareFieldOptions::from_decorators(&[decorator], "partialEq");
        assert!(opts.skip);
    }

    #[test]
    fn test_compare_field_no_skip() {
        let decorator = make_decorator("partialEq", "");
        let opts = CompareFieldOptions::from_decorators(&[decorator], "partialEq");
        assert!(!opts.skip);
    }

    #[test]
    fn test_compare_field_skip_false() {
        let decorator = make_decorator("hash", "skip: false");
        let opts = CompareFieldOptions::from_decorators(&[decorator], "hash");
        assert!(!opts.skip);
    }

    #[test]
    fn test_default_field_with_string_value() {
        let decorator = make_decorator("default", r#""hello""#);
        let opts = DefaultFieldOptions::from_decorators(&[decorator]);
        assert!(opts.has_default);
        assert_eq!(opts.value.as_deref(), Some(r#""hello""#));
    }

    #[test]
    fn test_default_field_with_number_value() {
        let decorator = make_decorator("default", "42");
        let opts = DefaultFieldOptions::from_decorators(&[decorator]);
        assert!(opts.has_default);
        assert_eq!(opts.value.as_deref(), Some("42"));
    }

    #[test]
    fn test_default_field_with_array_value() {
        let decorator = make_decorator("default", "[]");
        let opts = DefaultFieldOptions::from_decorators(&[decorator]);
        assert!(opts.has_default);
        assert_eq!(opts.value.as_deref(), Some("[]"));
    }

    #[test]
    fn test_default_field_with_named_value() {
        let decorator = make_decorator("default", r#"{ value: "test" }"#);
        let opts = DefaultFieldOptions::from_decorators(&[decorator]);
        assert!(opts.has_default);
        assert_eq!(opts.value.as_deref(), Some("test"));
    }

    #[test]
    fn test_is_primitive_type() {
        assert!(is_primitive_type("string"));
        assert!(is_primitive_type("number"));
        assert!(is_primitive_type("boolean"));
        assert!(is_primitive_type("bigint"));
        assert!(!is_primitive_type("Date"));
        assert!(!is_primitive_type("User"));
        assert!(!is_primitive_type("string[]"));
    }

    #[test]
    fn test_is_numeric_type() {
        assert!(is_numeric_type("number"));
        assert!(is_numeric_type("bigint"));
        assert!(!is_numeric_type("string"));
        assert!(!is_numeric_type("boolean"));
    }

    #[test]
    fn test_get_type_default() {
        assert_eq!(get_type_default("string"), r#""""#);
        assert_eq!(get_type_default("number"), "0");
        assert_eq!(get_type_default("boolean"), "false");
        assert_eq!(get_type_default("bigint"), "0n");
        assert_eq!(get_type_default("string[]"), "[]");
        assert_eq!(get_type_default("Array<number>"), "[]");
        assert_eq!(get_type_default("Map<string, number>"), "new Map()");
        assert_eq!(get_type_default("Set<string>"), "new Set()");
        assert_eq!(get_type_default("Date"), "new Date()");
        // Unknown types call their defaultValue() method (Prefix style)
        assert_eq!(get_type_default("User"), "userDefaultValue()");
        // Generic type instantiations use typeDefaultValue<Args>() syntax
        assert_eq!(
            get_type_default("RecordLink<Service>"),
            "recordLinkDefaultValue<Service>()"
        );
        assert_eq!(
            get_type_default("Result<User, Error>"),
            "resultDefaultValue<User, Error>()"
        );
    }

    #[test]
    fn test_is_generic_type() {
        // Generic types
        assert!(is_generic_type("RecordLink<Service>"));
        assert!(is_generic_type("Map<string, number>"));
        assert!(is_generic_type("Array<User>"));
        assert!(is_generic_type("Result<T, E>"));

        // Non-generic types
        assert!(!is_generic_type("User"));
        assert!(!is_generic_type("string"));
        assert!(!is_generic_type("number[]")); // Array syntax, not generic
    }

    #[test]
    fn test_parse_generic_type() {
        // Simple generic
        assert_eq!(
            parse_generic_type("RecordLink<Service>"),
            Some(("RecordLink", "Service"))
        );

        // Multiple type parameters
        assert_eq!(
            parse_generic_type("Map<string, number>"),
            Some(("Map", "string, number"))
        );

        // Nested generics
        assert_eq!(
            parse_generic_type("Result<Array<User>, Error>"),
            Some(("Result", "Array<User>, Error"))
        );

        // Non-generic types return None
        assert_eq!(parse_generic_type("User"), None);
        assert_eq!(parse_generic_type("string"), None);

        // Malformed (no closing bracket)
        assert_eq!(parse_generic_type("Array<User"), None);
    }
}
