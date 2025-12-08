//! Shared utilities for Serialize and Deserialize derive macros

pub mod derive_deserialize;
pub mod derive_serialize;

use crate::ts_syn::abi::DecoratorIR;

/// Naming convention for JSON field renaming
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum RenameAll {
    #[default]
    None,
    CamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    PascalCase,
}

impl RenameAll {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().replace(['-', '_'], "").as_str() {
            "camelcase" => Some(Self::CamelCase),
            "snakecase" => Some(Self::SnakeCase),
            "screamingsnakecase" => Some(Self::ScreamingSnakeCase),
            "kebabcase" => Some(Self::KebabCase),
            "pascalcase" => Some(Self::PascalCase),
            _ => None,
        }
    }

    pub fn apply(&self, name: &str) -> String {
        match self {
            Self::None => name.to_string(),
            Self::CamelCase => to_camel_case(name),
            Self::SnakeCase => to_snake_case(name),
            Self::ScreamingSnakeCase => to_snake_case(name).to_uppercase(),
            Self::KebabCase => to_snake_case(name).replace('_', "-"),
            Self::PascalCase => to_pascal_case(name),
        }
    }
}

/// Container-level serde options (on the class/interface itself)
#[derive(Debug, Clone, Default)]
pub struct SerdeContainerOptions {
    pub rename_all: RenameAll,
    pub deny_unknown_fields: bool,
}

impl SerdeContainerOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();

            if let Some(rename_all) = extract_named_string(args, "rename_all")
                && let Some(convention) = RenameAll::from_str(&rename_all)
            {
                opts.rename_all = convention;
            }

            if has_flag(args, "deny_unknown_fields") {
                opts.deny_unknown_fields = true;
            }
        }
        opts
    }
}

/// Field-level serde options
#[derive(Debug, Clone, Default)]
pub struct SerdeFieldOptions {
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
    pub rename: Option<String>,
    pub default: bool,
    pub default_expr: Option<String>,
    pub flatten: bool,
}

impl SerdeFieldOptions {
    pub fn from_decorators(decorators: &[DecoratorIR]) -> Self {
        let mut opts = Self::default();
        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();

            if has_flag(args, "skip") {
                opts.skip = true;
            }
            if has_flag(args, "skip_serializing") {
                opts.skip_serializing = true;
            }
            if has_flag(args, "skip_deserializing") {
                opts.skip_deserializing = true;
            }
            if has_flag(args, "flatten") {
                opts.flatten = true;
            }

            // Check for default (both boolean flag and expression)
            if let Some(default_expr) = extract_named_string(args, "default") {
                opts.default = true;
                opts.default_expr = Some(default_expr);
            } else if has_flag(args, "default") {
                opts.default = true;
            }

            if let Some(rename) = extract_named_string(args, "rename") {
                opts.rename = Some(rename);
            }
        }
        opts
    }

    pub fn should_serialize(&self) -> bool {
        !self.skip && !self.skip_serializing
    }

    pub fn should_deserialize(&self) -> bool {
        !self.skip && !self.skip_deserializing
    }
}

/// Determines the serialization strategy for a TypeScript type
#[derive(Debug, Clone, PartialEq)]
pub enum TypeCategory {
    Primitive,
    Array(String),
    Optional(String),
    Nullable(String),
    Date,
    Map(String, String),
    Set(String),
    Serializable(String),
    Unknown,
}

impl TypeCategory {
    pub fn from_ts_type(ts_type: &str) -> Self {
        let trimmed = ts_type.trim();

        // Handle primitives
        match trimmed {
            "string" | "number" | "boolean" | "null" | "undefined" | "bigint" => {
                return Self::Primitive;
            }
            "Date" => return Self::Date,
            _ => {}
        }

        // Handle Array<T> or T[]
        if trimmed.starts_with("Array<") && trimmed.ends_with('>') {
            let inner = &trimmed[6..trimmed.len() - 1];
            return Self::Array(inner.to_string());
        }
        if let Some(inner) = trimmed.strip_suffix("[]") {
            return Self::Array(inner.to_string());
        }

        // Handle Map<K, V>
        if trimmed.starts_with("Map<") && trimmed.ends_with('>') {
            let inner = &trimmed[4..trimmed.len() - 1];
            if let Some(comma_pos) = find_top_level_comma(inner) {
                let key = inner[..comma_pos].trim().to_string();
                let value = inner[comma_pos + 1..].trim().to_string();
                return Self::Map(key, value);
            }
        }

        // Handle Set<T>
        if trimmed.starts_with("Set<") && trimmed.ends_with('>') {
            let inner = &trimmed[4..trimmed.len() - 1];
            return Self::Set(inner.to_string());
        }

        // Handle union types (T | undefined, T | null)
        if trimmed.contains('|') {
            let parts: Vec<&str> = trimmed.split('|').map(|s| s.trim()).collect();
            if parts.contains(&"undefined") {
                let non_undefined: Vec<&str> = parts
                    .iter()
                    .filter(|p| *p != &"undefined")
                    .copied()
                    .collect();
                return Self::Optional(non_undefined.join(" | "));
            }
            if parts.contains(&"null") {
                let non_null: Vec<&str> = parts.iter().filter(|p| *p != &"null").copied().collect();
                return Self::Nullable(non_null.join(" | "));
            }
        }

        // Check if it looks like a class/interface name (starts with uppercase)
        if let Some(first_char) = trimmed.chars().next()
            && first_char.is_uppercase()
            && !matches!(
                trimmed,
                "String" | "Number" | "Boolean" | "Object" | "Function" | "Symbol"
            )
        {
            return Self::Serializable(trimmed.to_string());
        }

        Self::Unknown
    }
}

// ============================================================================
// Helper functions (adapted from derive_debug.rs)
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

/// Find the position of a comma at the top level (not inside <> brackets)
fn find_top_level_comma(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => return Some(i),
            _ => {}
        }
    }
    None
}

// ============================================================================
// Case conversion utilities
// ============================================================================

pub fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    let mut first = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else if first {
            result.push(c.to_ascii_lowercase());
            first = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;

    for (i, c) in s.char_indices() {
        if c.is_uppercase() {
            if i > 0 && !prev_was_upper {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_upper = true;
        } else if c == '-' {
            result.push('_');
            prev_was_upper = false;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }
    result
}

pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
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

    fn make_decorator(args: &str) -> DecoratorIR {
        DecoratorIR {
            name: "serde".into(),
            args_src: args.into(),
            span: span(),
            node: None,
        }
    }

    #[test]
    fn test_field_skip() {
        let decorator = make_decorator("skip");
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert!(opts.skip);
        assert!(!opts.should_serialize());
        assert!(!opts.should_deserialize());
    }

    #[test]
    fn test_field_skip_serializing() {
        let decorator = make_decorator("skip_serializing");
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert!(opts.skip_serializing);
        assert!(!opts.should_serialize());
        assert!(opts.should_deserialize());
    }

    #[test]
    fn test_field_rename() {
        let decorator = make_decorator(r#"{ rename: "user_id" }"#);
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert_eq!(opts.rename.as_deref(), Some("user_id"));
    }

    #[test]
    fn test_field_default_flag() {
        let decorator = make_decorator("default");
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert!(opts.default);
        assert!(opts.default_expr.is_none());
    }

    #[test]
    fn test_field_default_expr() {
        let decorator = make_decorator(r#"{ default: "new Date()" }"#);
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert!(opts.default);
        assert_eq!(opts.default_expr.as_deref(), Some("new Date()"));
    }

    #[test]
    fn test_field_flatten() {
        let decorator = make_decorator("flatten");
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert!(opts.flatten);
    }

    #[test]
    fn test_container_rename_all() {
        let decorator = make_decorator(r#"{ rename_all: "camelCase" }"#);
        let opts = SerdeContainerOptions::from_decorators(&[decorator]);
        assert_eq!(opts.rename_all, RenameAll::CamelCase);
    }

    #[test]
    fn test_container_deny_unknown_fields() {
        let decorator = make_decorator("deny_unknown_fields");
        let opts = SerdeContainerOptions::from_decorators(&[decorator]);
        assert!(opts.deny_unknown_fields);
    }

    #[test]
    fn test_type_category_primitives() {
        assert_eq!(
            TypeCategory::from_ts_type("string"),
            TypeCategory::Primitive
        );
        assert_eq!(
            TypeCategory::from_ts_type("number"),
            TypeCategory::Primitive
        );
        assert_eq!(
            TypeCategory::from_ts_type("boolean"),
            TypeCategory::Primitive
        );
    }

    #[test]
    fn test_type_category_date() {
        assert_eq!(TypeCategory::from_ts_type("Date"), TypeCategory::Date);
    }

    #[test]
    fn test_type_category_array() {
        assert_eq!(
            TypeCategory::from_ts_type("string[]"),
            TypeCategory::Array("string".into())
        );
        assert_eq!(
            TypeCategory::from_ts_type("Array<number>"),
            TypeCategory::Array("number".into())
        );
    }

    #[test]
    fn test_type_category_map() {
        assert_eq!(
            TypeCategory::from_ts_type("Map<string, number>"),
            TypeCategory::Map("string".into(), "number".into())
        );
    }

    #[test]
    fn test_type_category_set() {
        assert_eq!(
            TypeCategory::from_ts_type("Set<string>"),
            TypeCategory::Set("string".into())
        );
    }

    #[test]
    fn test_type_category_optional() {
        assert_eq!(
            TypeCategory::from_ts_type("string | undefined"),
            TypeCategory::Optional("string".into())
        );
    }

    #[test]
    fn test_type_category_nullable() {
        assert_eq!(
            TypeCategory::from_ts_type("string | null"),
            TypeCategory::Nullable("string".into())
        );
    }

    #[test]
    fn test_type_category_serializable() {
        assert_eq!(
            TypeCategory::from_ts_type("User"),
            TypeCategory::Serializable("User".into())
        );
    }

    #[test]
    fn test_rename_all_camel_case() {
        assert_eq!(RenameAll::CamelCase.apply("user_name"), "userName");
        assert_eq!(RenameAll::CamelCase.apply("created_at"), "createdAt");
    }

    #[test]
    fn test_rename_all_snake_case() {
        assert_eq!(RenameAll::SnakeCase.apply("userName"), "user_name");
        assert_eq!(RenameAll::SnakeCase.apply("createdAt"), "created_at");
    }

    #[test]
    fn test_rename_all_pascal_case() {
        assert_eq!(RenameAll::PascalCase.apply("user_name"), "UserName");
    }

    #[test]
    fn test_rename_all_kebab_case() {
        assert_eq!(RenameAll::KebabCase.apply("userName"), "user-name");
    }

    #[test]
    fn test_rename_all_screaming_snake_case() {
        assert_eq!(RenameAll::ScreamingSnakeCase.apply("userName"), "USER_NAME");
    }
}
