//! Shared utilities for comparison and equality derive macros

use crate::ts_syn::abi::DecoratorIR;

// ============================================================================
// Field Options for Comparison Macros
// ============================================================================

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

/// Get default value for a TypeScript type
pub fn get_type_default(ts_type: &str) -> String {
    match ts_type.trim() {
        "string" => r#""""#.to_string(),
        "number" => "0".to_string(),
        "boolean" => "false".to_string(),
        "bigint" => "0n".to_string(),
        t if t.ends_with("[]") => "[]".to_string(),
        t if t.starts_with("Array<") => "[]".to_string(),
        t if t.starts_with("Map<") => "new Map()".to_string(),
        t if t.starts_with("Set<") => "new Set()".to_string(),
        "Date" => "new Date()".to_string(),
        _ => "null as any".to_string(),
    }
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
        assert_eq!(get_type_default("User"), "null as any");
    }
}
