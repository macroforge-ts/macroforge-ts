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
    pub validators: Vec<ValidatorSpec>,
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

            // Extract validators
            let validators = extract_validators(args);
            opts.validators.extend(validators);
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
// Validator types for field validation
// ============================================================================

/// A single validator with optional custom message
#[derive(Debug, Clone)]
pub struct ValidatorSpec {
    pub validator: Validator,
    pub custom_message: Option<String>,
}

/// All supported validators for field validation during deserialization
#[derive(Debug, Clone, PartialEq)]
pub enum Validator {
    // String validators
    Email,
    Url,
    Uuid,
    MaxLength(usize),
    MinLength(usize),
    Length(usize),
    LengthRange(usize, usize),
    Pattern(String),
    NonEmpty,
    Trimmed,
    Lowercase,
    Uppercase,
    Capitalized,
    Uncapitalized,
    StartsWith(String),
    EndsWith(String),
    Includes(String),

    // Number validators
    GreaterThan(f64),
    GreaterThanOrEqualTo(f64),
    LessThan(f64),
    LessThanOrEqualTo(f64),
    Between(f64, f64),
    Int,
    NonNaN,
    Finite,
    Positive,
    NonNegative,
    Negative,
    NonPositive,
    MultipleOf(f64),
    Uint8,

    // Array validators
    MaxItems(usize),
    MinItems(usize),
    ItemsCount(usize),

    // Date validators
    ValidDate,
    GreaterThanDate(String),
    GreaterThanOrEqualToDate(String),
    LessThanDate(String),
    LessThanOrEqualToDate(String),
    BetweenDate(String, String),

    // BigInt validators
    GreaterThanBigInt(String),
    GreaterThanOrEqualToBigInt(String),
    LessThanBigInt(String),
    LessThanOrEqualToBigInt(String),
    BetweenBigInt(String, String),
    PositiveBigInt,
    NonNegativeBigInt,
    NegativeBigInt,
    NonPositiveBigInt,

    // Custom validator
    Custom(String),
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
// Validator parsing functions
// ============================================================================

/// Extract validators from decorator arguments
/// Supports: validate: ["email", "maxLength(255)"] or validate: [{ validate: "email", message: "..." }]
pub fn extract_validators(args: &str) -> Vec<ValidatorSpec> {
    let lower = args.to_ascii_lowercase();
    if let Some(idx) = lower.find("validate") {
        let remainder = &args[idx + 8..].trim_start();
        if remainder.starts_with(':') || remainder.starts_with('=') {
            let value_start = &remainder[1..].trim_start();
            if value_start.starts_with('[') {
                return parse_validator_array(value_start);
            }
        }
    }
    Vec::new()
}

/// Parse array content: ["email", "maxLength(255)", { validate: "...", message: "..." }]
fn parse_validator_array(input: &str) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    // Find matching ] bracket
    if let Some(content) = extract_bracket_content(input, '[', ']') {
        // Split by commas (respecting nested structures)
        for item in split_array_items(&content) {
            let item = item.trim();
            if item.starts_with('{') {
                // Object form: { validate: "...", message: "..." }
                if let Some(spec) = parse_validator_object(item) {
                    validators.push(spec);
                }
            } else if item.starts_with('"') || item.starts_with('\'') {
                // String form: "email" or "maxLength(255)"
                if let Some(s) = parse_string_literal(item)
                    && let Some(v) = parse_validator_string(&s)
                {
                    validators.push(ValidatorSpec {
                        validator: v,
                        custom_message: None,
                    });
                }
            }
        }
    }
    validators
}

/// Extract content between matching brackets
fn extract_bracket_content(input: &str, open: char, close: char) -> Option<String> {
    let mut depth = 0;
    let mut start = None;

    for (i, c) in input.char_indices() {
        if c == open {
            if depth == 0 {
                start = Some(i + 1);
            }
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0
                && let Some(s) = start
            {
                return Some(input[s..i].to_string());
            }
        }
    }
    None
}

/// Split array items by commas, respecting nested brackets and strings
fn split_array_items(input: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut string_char = '"';

    for c in input.chars() {
        if in_string {
            current.push(c);
            if c == string_char {
                in_string = false;
            }
            continue;
        }

        match c {
            '"' | '\'' => {
                in_string = true;
                string_char = c;
                current.push(c);
            }
            '[' | '{' | '(' => {
                depth += 1;
                current.push(c);
            }
            ']' | '}' | ')' => {
                depth -= 1;
                current.push(c);
            }
            ',' if depth == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() {
                    items.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        items.push(trimmed);
    }

    items
}

/// Parse object form: { validate: "email", message: "Invalid email" }
fn parse_validator_object(input: &str) -> Option<ValidatorSpec> {
    let content = extract_bracket_content(input, '{', '}')?;

    let validator_str = extract_named_string(&content, "validate")?;
    let validator = parse_validator_string(&validator_str)?;
    let custom_message = extract_named_string(&content, "message");

    Some(ValidatorSpec {
        validator,
        custom_message,
    })
}

/// Parse a validator string like "email", "maxLength(255)", "custom(myValidator)"
fn parse_validator_string(s: &str) -> Option<Validator> {
    let trimmed = s.trim();

    // Check for function-call style: name(args)
    if let Some(paren_idx) = trimmed.find('(') {
        let name = &trimmed[..paren_idx];
        let args_end = trimmed.rfind(')')?;
        let args = &trimmed[paren_idx + 1..args_end];
        return parse_validator_with_args(name, args);
    }

    // Simple validators without args
    match trimmed.to_lowercase().as_str() {
        "email" => Some(Validator::Email),
        "url" => Some(Validator::Url),
        "uuid" => Some(Validator::Uuid),
        "nonempty" | "nonemptystring" => Some(Validator::NonEmpty),
        "trimmed" => Some(Validator::Trimmed),
        "lowercase" | "lowercased" => Some(Validator::Lowercase),
        "uppercase" | "uppercased" => Some(Validator::Uppercase),
        "capitalized" => Some(Validator::Capitalized),
        "uncapitalized" => Some(Validator::Uncapitalized),
        "int" => Some(Validator::Int),
        "nonnan" => Some(Validator::NonNaN),
        "finite" => Some(Validator::Finite),
        "positive" => Some(Validator::Positive),
        "nonnegative" => Some(Validator::NonNegative),
        "negative" => Some(Validator::Negative),
        "nonpositive" => Some(Validator::NonPositive),
        "uint8" => Some(Validator::Uint8),
        "validdate" | "validdatefromself" => Some(Validator::ValidDate),
        "positivebigint" | "positivebigintfromself" => Some(Validator::PositiveBigInt),
        "nonnegativebigint" | "nonnegativebigintfromself" => Some(Validator::NonNegativeBigInt),
        "negativebigint" | "negativebigintfromself" => Some(Validator::NegativeBigInt),
        "nonpositivebigint" | "nonpositivebigintfromself" => Some(Validator::NonPositiveBigInt),
        "nonnegativeint" => Some(Validator::Int), // Int + NonNegative combined
        _ => None,
    }
}

/// Parse validators with arguments
fn parse_validator_with_args(name: &str, args: &str) -> Option<Validator> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "maxlength" => args.trim().parse().ok().map(Validator::MaxLength),
        "minlength" => args.trim().parse().ok().map(Validator::MinLength),
        "length" => {
            let parts: Vec<&str> = args.split(',').collect();
            match parts.len() {
                1 => parts[0].trim().parse().ok().map(Validator::Length),
                2 => {
                    let min = parts[0].trim().parse().ok()?;
                    let max = parts[1].trim().parse().ok()?;
                    Some(Validator::LengthRange(min, max))
                }
                _ => None,
            }
        }
        "pattern" => parse_validator_string_arg(args).map(Validator::Pattern),
        "startswith" => parse_validator_string_arg(args).map(Validator::StartsWith),
        "endswith" => parse_validator_string_arg(args).map(Validator::EndsWith),
        "includes" => parse_validator_string_arg(args).map(Validator::Includes),
        "greaterthan" => args.trim().parse().ok().map(Validator::GreaterThan),
        "greaterthanorequalto" => args
            .trim()
            .parse()
            .ok()
            .map(Validator::GreaterThanOrEqualTo),
        "lessthan" => args.trim().parse().ok().map(Validator::LessThan),
        "lessthanorequalto" => args.trim().parse().ok().map(Validator::LessThanOrEqualTo),
        "between" => {
            let parts: Vec<&str> = args.split(',').collect();
            if parts.len() == 2 {
                let min = parts[0].trim().parse().ok()?;
                let max = parts[1].trim().parse().ok()?;
                Some(Validator::Between(min, max))
            } else {
                None
            }
        }
        "multipleof" => args.trim().parse().ok().map(Validator::MultipleOf),
        "maxitems" => args.trim().parse().ok().map(Validator::MaxItems),
        "minitems" => args.trim().parse().ok().map(Validator::MinItems),
        "itemscount" => args.trim().parse().ok().map(Validator::ItemsCount),
        "greaterthandate" => parse_validator_string_arg(args).map(Validator::GreaterThanDate),
        "greaterthanorequaltodate" => {
            parse_validator_string_arg(args).map(Validator::GreaterThanOrEqualToDate)
        }
        "lessthandate" => parse_validator_string_arg(args).map(Validator::LessThanDate),
        "lessthanorequaltodate" => {
            parse_validator_string_arg(args).map(Validator::LessThanOrEqualToDate)
        }
        "betweendate" => {
            let parts: Vec<&str> = args.splitn(2, ',').collect();
            if parts.len() == 2 {
                let min = parse_validator_string_arg(parts[0].trim())?;
                let max = parse_validator_string_arg(parts[1].trim())?;
                Some(Validator::BetweenDate(min, max))
            } else {
                None
            }
        }
        "greaterthanbigint" => Some(Validator::GreaterThanBigInt(args.trim().to_string())),
        "greaterthanorequaltobigint" => Some(Validator::GreaterThanOrEqualToBigInt(
            args.trim().to_string(),
        )),
        "lessthanbigint" => Some(Validator::LessThanBigInt(args.trim().to_string())),
        "lessthanorequaltobigint" => {
            Some(Validator::LessThanOrEqualToBigInt(args.trim().to_string()))
        }
        "betweenbigint" => {
            let parts: Vec<&str> = args.splitn(2, ',').collect();
            if parts.len() == 2 {
                Some(Validator::BetweenBigInt(
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                ))
            } else {
                None
            }
        }
        "custom" => {
            // custom(myValidator) - extract function name (can be quoted or unquoted)
            let fn_name =
                parse_validator_string_arg(args).unwrap_or_else(|| args.trim().to_string());
            Some(Validator::Custom(fn_name))
        }
        _ => None,
    }
}

/// Parse a string argument (handles both quoted and unquoted)
fn parse_validator_string_arg(input: &str) -> Option<String> {
    let trimmed = input.trim();
    // Try to parse as quoted string first
    if let Some(s) = parse_string_literal(trimmed) {
        return Some(s);
    }
    // Otherwise return as-is if not empty
    if !trimmed.is_empty() {
        return Some(trimmed.to_string());
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

    // ========================================================================
    // Validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_simple_validators() {
        assert!(matches!(
            parse_validator_string("email"),
            Some(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("url"),
            Some(Validator::Url)
        ));
        assert!(matches!(
            parse_validator_string("uuid"),
            Some(Validator::Uuid)
        ));
        assert!(matches!(
            parse_validator_string("nonEmpty"),
            Some(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("trimmed"),
            Some(Validator::Trimmed)
        ));
        assert!(matches!(
            parse_validator_string("lowercase"),
            Some(Validator::Lowercase)
        ));
        assert!(matches!(
            parse_validator_string("uppercase"),
            Some(Validator::Uppercase)
        ));
        assert!(matches!(
            parse_validator_string("int"),
            Some(Validator::Int)
        ));
        assert!(matches!(
            parse_validator_string("positive"),
            Some(Validator::Positive)
        ));
        assert!(matches!(
            parse_validator_string("validDate"),
            Some(Validator::ValidDate)
        ));
    }

    #[test]
    fn test_parse_validators_with_args() {
        assert!(matches!(
            parse_validator_string("maxLength(255)"),
            Some(Validator::MaxLength(255))
        ));
        assert!(matches!(
            parse_validator_string("minLength(1)"),
            Some(Validator::MinLength(1))
        ));
        assert!(matches!(
            parse_validator_string("length(36)"),
            Some(Validator::Length(36))
        ));
        assert!(matches!(
            parse_validator_string("between(0, 100)"),
            Some(Validator::Between(min, max)) if min == 0.0 && max == 100.0
        ));
        assert!(matches!(
            parse_validator_string("greaterThan(5)"),
            Some(Validator::GreaterThan(n)) if n == 5.0
        ));
    }

    #[test]
    fn test_parse_validators_with_string_args() {
        assert!(matches!(
            parse_validator_string(r#"startsWith("https://")"#),
            Some(Validator::StartsWith(s)) if s == "https://"
        ));
        assert!(matches!(
            parse_validator_string(r#"endsWith(".com")"#),
            Some(Validator::EndsWith(s)) if s == ".com"
        ));
        assert!(matches!(
            parse_validator_string(r#"includes("@")"#),
            Some(Validator::Includes(s)) if s == "@"
        ));
    }

    #[test]
    fn test_parse_custom_validator() {
        assert!(matches!(
            parse_validator_string("custom(myValidator)"),
            Some(Validator::Custom(fn_name)) if fn_name == "myValidator"
        ));
    }

    #[test]
    fn test_extract_validators_from_args() {
        let validators = extract_validators(r#"{ validate: ["email", "maxLength(255)"] }"#);
        assert_eq!(validators.len(), 2);
        assert!(matches!(validators[0].validator, Validator::Email));
        assert!(matches!(validators[1].validator, Validator::MaxLength(255)));
    }

    #[test]
    fn test_extract_validators_with_message() {
        let validators = extract_validators(
            r#"{ validate: [{ validate: "email", message: "Invalid email!" }] }"#,
        );
        assert_eq!(validators.len(), 1);
        assert!(matches!(validators[0].validator, Validator::Email));
        assert_eq!(
            validators[0].custom_message.as_deref(),
            Some("Invalid email!")
        );
    }

    #[test]
    fn test_extract_validators_mixed() {
        let validators = extract_validators(
            r#"{ validate: ["nonEmpty", { validate: "email", message: "Bad email" }] }"#,
        );
        assert_eq!(validators.len(), 2);
        assert!(matches!(validators[0].validator, Validator::NonEmpty));
        assert!(validators[0].custom_message.is_none());
        assert!(matches!(validators[1].validator, Validator::Email));
        assert_eq!(validators[1].custom_message.as_deref(), Some("Bad email"));
    }

    #[test]
    fn test_field_with_validators() {
        let decorator = make_decorator(r#"{ validate: ["email", "maxLength(255)"] }"#);
        let opts = SerdeFieldOptions::from_decorators(&[decorator]);
        assert_eq!(opts.validators.len(), 2);
        assert!(matches!(opts.validators[0].validator, Validator::Email));
        assert!(matches!(
            opts.validators[1].validator,
            Validator::MaxLength(255)
        ));
    }
}
