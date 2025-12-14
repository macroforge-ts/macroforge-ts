//! Shared utilities for Serialize and Deserialize derive macros

pub mod derive_deserialize;
pub mod derive_serialize;

use crate::ts_syn::abi::{DecoratorIR, DiagnosticCollector, SpanIR};

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

/// Result of parsing field options, containing both options and any diagnostics
#[derive(Debug, Clone, Default)]
pub struct SerdeFieldParseResult {
    pub options: SerdeFieldOptions,
    pub diagnostics: DiagnosticCollector,
}

impl SerdeFieldOptions {
    /// Parse field options from decorators, collecting diagnostics for invalid configurations
    pub fn from_decorators(decorators: &[DecoratorIR], field_name: &str) -> SerdeFieldParseResult {
        let mut opts = Self::default();
        let mut diagnostics = DiagnosticCollector::new();

        for decorator in decorators {
            if !decorator.name.eq_ignore_ascii_case("serde") {
                continue;
            }
            let args = decorator.args_src.trim();
            let decorator_span = decorator.span;

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

            // Extract validators with diagnostic collection
            let validators = extract_validators(args, decorator_span, field_name, &mut diagnostics);
            opts.validators.extend(validators);
        }

        SerdeFieldParseResult { options: opts, diagnostics }
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
// Validator parsing errors
// ============================================================================

/// Error information from parsing a validator string
#[derive(Debug, Clone)]
pub struct ValidatorParseError {
    pub message: String,
    pub validator_text: String,
    pub help: Option<String>,
}

impl ValidatorParseError {
    /// Create error for an unknown validator name
    pub fn unknown_validator(name: &str) -> Self {
        let similar = find_similar_validator(name);
        Self {
            message: format!("unknown validator '{}'", name),
            validator_text: name.to_string(),
            help: similar.map(|s| format!("did you mean '{}'?", s)),
        }
    }

    /// Create error for invalid arguments
    pub fn invalid_args(name: &str, reason: &str) -> Self {
        Self {
            message: format!("invalid arguments for '{}': {}", name, reason),
            validator_text: name.to_string(),
            help: None,
        }
    }

    /// Create error for missing required arguments
    pub fn missing_args(name: &str) -> Self {
        Self {
            message: format!("'{}' requires arguments", name),
            validator_text: name.to_string(),
            help: Some(format!("use '{}(value)' syntax", name)),
        }
    }
}

/// List of all known validator names for typo detection
const KNOWN_VALIDATORS: &[&str] = &[
    "email", "url", "uuid", "maxLength", "minLength", "length",
    "pattern", "nonEmpty", "trimmed", "lowercase", "uppercase",
    "capitalized", "uncapitalized", "startsWith", "endsWith", "includes",
    "greaterThan", "greaterThanOrEqualTo", "lessThan", "lessThanOrEqualTo",
    "between", "int", "nonNaN", "finite", "positive", "nonNegative",
    "negative", "nonPositive", "multipleOf", "uint8",
    "maxItems", "minItems", "itemsCount",
    "validDate", "greaterThanDate", "greaterThanOrEqualToDate",
    "lessThanDate", "lessThanOrEqualToDate", "betweenDate",
    "positiveBigInt", "nonNegativeBigInt", "negativeBigInt", "nonPositiveBigInt",
    "greaterThanBigInt", "greaterThanOrEqualToBigInt", "lessThanBigInt",
    "lessThanOrEqualToBigInt", "betweenBigInt",
    "custom",
];

/// Find a similar validator name for typo suggestions using Levenshtein distance
fn find_similar_validator(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    KNOWN_VALIDATORS.iter()
        .filter_map(|v| {
            let dist = levenshtein_distance(&v.to_lowercase(), &name_lower);
            if dist <= 2 {
                Some((*v, dist))
            } else {
                None
            }
        })
        .min_by_key(|(_, dist)| *dist)
        .map(|(v, _)| v)
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();

    if len_a == 0 { return len_b; }
    if len_b == 0 { return len_a; }

    let mut prev_row: Vec<usize> = (0..=len_b).collect();
    let mut curr_row: Vec<usize> = vec![0; len_b + 1];

    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[len_b]
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

/// Extract validators from decorator arguments with diagnostic collection
/// Supports: validate: ["email", "maxLength(255)"] or validate: [{ validate: "email", message: "..." }]
pub fn extract_validators(
    args: &str,
    decorator_span: SpanIR,
    field_name: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let lower = args.to_ascii_lowercase();
    if let Some(idx) = lower.find("validate") {
        let remainder = &args[idx + 8..].trim_start();
        if remainder.starts_with(':') || remainder.starts_with('=') {
            let value_start = &remainder[1..].trim_start();
            if value_start.starts_with('[') {
                return parse_validator_array(value_start, decorator_span, field_name, diagnostics);
            } else {
                diagnostics.error(
                    decorator_span,
                    format!("field '{}': validate must be an array, e.g., validate: [\"email\"]", field_name),
                );
            }
        }
    }
    Vec::new()
}

/// Parse array content: ["email", "maxLength(255)", { validate: "...", message: "..." }]
fn parse_validator_array(
    input: &str,
    decorator_span: SpanIR,
    field_name: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    // Find matching ] bracket
    let Some(content) = extract_bracket_content(input, '[', ']') else {
        diagnostics.error(decorator_span, format!("field '{}': malformed validator array", field_name));
        return validators;
    };

    // Split by commas (respecting nested structures)
    for item in split_array_items(&content) {
        let item = item.trim();
        if item.starts_with('{') {
            // Object form: { validate: "...", message: "..." }
            match parse_validator_object(item) {
                Ok(spec) => validators.push(spec),
                Err(err) => {
                    if let Some(help) = err.help {
                        diagnostics.error_with_help(
                            decorator_span,
                            format!("field '{}': {}", field_name, err.message),
                            help,
                        );
                    } else {
                        diagnostics.error(
                            decorator_span,
                            format!("field '{}': {}", field_name, err.message),
                        );
                    }
                }
            }
        } else if item.starts_with('"') || item.starts_with('\'') {
            // String form: "email" or "maxLength(255)"
            if let Some(s) = parse_string_literal(item) {
                match parse_validator_string(&s) {
                    Ok(v) => validators.push(ValidatorSpec {
                        validator: v,
                        custom_message: None,
                    }),
                    Err(err) => {
                        if let Some(help) = err.help {
                            diagnostics.error_with_help(
                                decorator_span,
                                format!("field '{}': {}", field_name, err.message),
                                help,
                            );
                        } else {
                            diagnostics.error(
                                decorator_span,
                                format!("field '{}': {}", field_name, err.message),
                            );
                        }
                    }
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
fn parse_validator_object(input: &str) -> Result<ValidatorSpec, ValidatorParseError> {
    let content = extract_bracket_content(input, '{', '}')
        .ok_or_else(|| ValidatorParseError::invalid_args("object", "malformed validator object"))?;

    let validator_str = extract_named_string(&content, "validate")
        .ok_or_else(|| ValidatorParseError::invalid_args("object", "missing 'validate' field"))?;
    let validator = parse_validator_string(&validator_str)?;
    let custom_message = extract_named_string(&content, "message");

    Ok(ValidatorSpec {
        validator,
        custom_message,
    })
}

/// Parse a validator string like "email", "maxLength(255)", "custom(myValidator)"
fn parse_validator_string(s: &str) -> Result<Validator, ValidatorParseError> {
    let trimmed = s.trim();

    // Check for function-call style: name(args)
    if let Some(paren_idx) = trimmed.find('(') {
        let name = &trimmed[..paren_idx];
        let Some(args_end) = trimmed.rfind(')') else {
            return Err(ValidatorParseError::invalid_args(name, "missing closing parenthesis"));
        };
        let args = &trimmed[paren_idx + 1..args_end];
        return parse_validator_with_args(name, args);
    }

    // Simple validators without args
    match trimmed.to_lowercase().as_str() {
        "email" => Ok(Validator::Email),
        "url" => Ok(Validator::Url),
        "uuid" => Ok(Validator::Uuid),
        "nonempty" | "nonemptystring" => Ok(Validator::NonEmpty),
        "trimmed" => Ok(Validator::Trimmed),
        "lowercase" | "lowercased" => Ok(Validator::Lowercase),
        "uppercase" | "uppercased" => Ok(Validator::Uppercase),
        "capitalized" => Ok(Validator::Capitalized),
        "uncapitalized" => Ok(Validator::Uncapitalized),
        "int" => Ok(Validator::Int),
        "nonnan" => Ok(Validator::NonNaN),
        "finite" => Ok(Validator::Finite),
        "positive" => Ok(Validator::Positive),
        "nonnegative" => Ok(Validator::NonNegative),
        "negative" => Ok(Validator::Negative),
        "nonpositive" => Ok(Validator::NonPositive),
        "uint8" => Ok(Validator::Uint8),
        "validdate" | "validdatefromself" => Ok(Validator::ValidDate),
        "positivebigint" | "positivebigintfromself" => Ok(Validator::PositiveBigInt),
        "nonnegativebigint" | "nonnegativebigintfromself" => Ok(Validator::NonNegativeBigInt),
        "negativebigint" | "negativebigintfromself" => Ok(Validator::NegativeBigInt),
        "nonpositivebigint" | "nonpositivebigintfromself" => Ok(Validator::NonPositiveBigInt),
        "nonnegativeint" => Ok(Validator::Int), // Int + NonNegative combined
        _ => Err(ValidatorParseError::unknown_validator(trimmed)),
    }
}

/// Parse validators with arguments
fn parse_validator_with_args(name: &str, args: &str) -> Result<Validator, ValidatorParseError> {
    let name_lower = name.to_lowercase();
    match name_lower.as_str() {
        "maxlength" => args
            .trim()
            .parse()
            .map(Validator::MaxLength)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "minlength" => args
            .trim()
            .parse()
            .map(Validator::MinLength)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "length" => {
            let parts: Vec<&str> = args.split(',').collect();
            match parts.len() {
                1 => parts[0]
                    .trim()
                    .parse()
                    .map(Validator::Length)
                    .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
                2 => {
                    let min = parts[0]
                        .trim()
                        .parse()
                        .map_err(|_| ValidatorParseError::invalid_args(name, "expected two positive integers"))?;
                    let max = parts[1]
                        .trim()
                        .parse()
                        .map_err(|_| ValidatorParseError::invalid_args(name, "expected two positive integers"))?;
                    Ok(Validator::LengthRange(min, max))
                }
                _ => Err(ValidatorParseError::invalid_args(name, "expected 1 or 2 arguments")),
            }
        }
        "pattern" => parse_validator_string_arg(args)
            .map(Validator::Pattern)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string pattern")),
        "startswith" => parse_validator_string_arg(args)
            .map(Validator::StartsWith)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string")),
        "endswith" => parse_validator_string_arg(args)
            .map(Validator::EndsWith)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string")),
        "includes" => parse_validator_string_arg(args)
            .map(Validator::Includes)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a string")),
        "greaterthan" => args
            .trim()
            .parse()
            .map(Validator::GreaterThan)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "greaterthanorequalto" => args
            .trim()
            .parse()
            .map(Validator::GreaterThanOrEqualTo)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "lessthan" => args
            .trim()
            .parse()
            .map(Validator::LessThan)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "lessthanorequalto" => args
            .trim()
            .parse()
            .map(Validator::LessThanOrEqualTo)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "between" => {
            let parts: Vec<&str> = args.split(',').collect();
            if parts.len() == 2 {
                let min = parts[0]
                    .trim()
                    .parse()
                    .map_err(|_| ValidatorParseError::invalid_args(name, "expected two numbers"))?;
                let max = parts[1]
                    .trim()
                    .parse()
                    .map_err(|_| ValidatorParseError::invalid_args(name, "expected two numbers"))?;
                Ok(Validator::Between(min, max))
            } else {
                Err(ValidatorParseError::invalid_args(name, "expected two numbers separated by comma"))
            }
        }
        "multipleof" => args
            .trim()
            .parse()
            .map(Validator::MultipleOf)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a number")),
        "maxitems" => args
            .trim()
            .parse()
            .map(Validator::MaxItems)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "minitems" => args
            .trim()
            .parse()
            .map(Validator::MinItems)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "itemscount" => args
            .trim()
            .parse()
            .map(Validator::ItemsCount)
            .map_err(|_| ValidatorParseError::invalid_args(name, "expected a positive integer")),
        "greaterthandate" => parse_validator_string_arg(args)
            .map(Validator::GreaterThanDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "greaterthanorequaltodate" => parse_validator_string_arg(args)
            .map(Validator::GreaterThanOrEqualToDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "lessthandate" => parse_validator_string_arg(args)
            .map(Validator::LessThanDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "lessthanorequaltodate" => parse_validator_string_arg(args)
            .map(Validator::LessThanOrEqualToDate)
            .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected a date string")),
        "betweendate" => {
            let parts: Vec<&str> = args.splitn(2, ',').collect();
            if parts.len() == 2 {
                let min = parse_validator_string_arg(parts[0].trim())
                    .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected two date strings"))?;
                let max = parse_validator_string_arg(parts[1].trim())
                    .ok_or_else(|| ValidatorParseError::invalid_args(name, "expected two date strings"))?;
                Ok(Validator::BetweenDate(min, max))
            } else {
                Err(ValidatorParseError::invalid_args(name, "expected two date strings separated by comma"))
            }
        }
        "greaterthanbigint" => Ok(Validator::GreaterThanBigInt(args.trim().to_string())),
        "greaterthanorequaltobigint" => Ok(Validator::GreaterThanOrEqualToBigInt(
            args.trim().to_string(),
        )),
        "lessthanbigint" => Ok(Validator::LessThanBigInt(args.trim().to_string())),
        "lessthanorequaltobigint" => {
            Ok(Validator::LessThanOrEqualToBigInt(args.trim().to_string()))
        }
        "betweenbigint" => {
            let parts: Vec<&str> = args.splitn(2, ',').collect();
            if parts.len() == 2 {
                Ok(Validator::BetweenBigInt(
                    parts[0].trim().to_string(),
                    parts[1].trim().to_string(),
                ))
            } else {
                Err(ValidatorParseError::invalid_args(name, "expected two bigint values separated by comma"))
            }
        }
        "custom" => {
            // custom(myValidator) - extract function name (can be quoted or unquoted)
            let fn_name =
                parse_validator_string_arg(args).unwrap_or_else(|| args.trim().to_string());
            Ok(Validator::Custom(fn_name))
        }
        _ => Err(ValidatorParseError::unknown_validator(name)),
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
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.skip);
        assert!(!opts.should_serialize());
        assert!(!opts.should_deserialize());
    }

    #[test]
    fn test_field_skip_serializing() {
        let decorator = make_decorator("skip_serializing");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.skip_serializing);
        assert!(!opts.should_serialize());
        assert!(opts.should_deserialize());
    }

    #[test]
    fn test_field_rename() {
        let decorator = make_decorator(r#"{ rename: "user_id" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.rename.as_deref(), Some("user_id"));
    }

    #[test]
    fn test_field_default_flag() {
        let decorator = make_decorator("default");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.default);
        assert!(opts.default_expr.is_none());
    }

    #[test]
    fn test_field_default_expr() {
        let decorator = make_decorator(r#"{ default: "new Date()" }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert!(opts.default);
        assert_eq!(opts.default_expr.as_deref(), Some("new Date()"));
    }

    #[test]
    fn test_field_flatten() {
        let decorator = make_decorator("flatten");
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
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
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("url"),
            Ok(Validator::Url)
        ));
        assert!(matches!(
            parse_validator_string("uuid"),
            Ok(Validator::Uuid)
        ));
        assert!(matches!(
            parse_validator_string("nonEmpty"),
            Ok(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("trimmed"),
            Ok(Validator::Trimmed)
        ));
        assert!(matches!(
            parse_validator_string("lowercase"),
            Ok(Validator::Lowercase)
        ));
        assert!(matches!(
            parse_validator_string("uppercase"),
            Ok(Validator::Uppercase)
        ));
        assert!(matches!(
            parse_validator_string("int"),
            Ok(Validator::Int)
        ));
        assert!(matches!(
            parse_validator_string("positive"),
            Ok(Validator::Positive)
        ));
        assert!(matches!(
            parse_validator_string("validDate"),
            Ok(Validator::ValidDate)
        ));
    }

    #[test]
    fn test_parse_validators_with_args() {
        assert!(matches!(
            parse_validator_string("maxLength(255)"),
            Ok(Validator::MaxLength(255))
        ));
        assert!(matches!(
            parse_validator_string("minLength(1)"),
            Ok(Validator::MinLength(1))
        ));
        assert!(matches!(
            parse_validator_string("length(36)"),
            Ok(Validator::Length(36))
        ));
        assert!(matches!(
            parse_validator_string("between(0, 100)"),
            Ok(Validator::Between(min, max)) if min == 0.0 && max == 100.0
        ));
        assert!(matches!(
            parse_validator_string("greaterThan(5)"),
            Ok(Validator::GreaterThan(n)) if n == 5.0
        ));
    }

    #[test]
    fn test_parse_validators_with_string_args() {
        assert!(matches!(
            parse_validator_string(r#"startsWith("https://")"#),
            Ok(Validator::StartsWith(s)) if s == "https://"
        ));
        assert!(matches!(
            parse_validator_string(r#"endsWith(".com")"#),
            Ok(Validator::EndsWith(s)) if s == ".com"
        ));
        assert!(matches!(
            parse_validator_string(r#"includes("@")"#),
            Ok(Validator::Includes(s)) if s == "@"
        ));
    }

    #[test]
    fn test_parse_custom_validator() {
        assert!(matches!(
            parse_validator_string("custom(myValidator)"),
            Ok(Validator::Custom(fn_name)) if fn_name == "myValidator"
        ));
    }

    #[test]
    fn test_extract_validators_from_args() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["email", "maxLength(255)"] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        assert_eq!(validators.len(), 2);
        assert!(matches!(validators[0].validator, Validator::Email));
        assert!(matches!(validators[1].validator, Validator::MaxLength(255)));
        assert!(!diagnostics.has_errors());
    }

    #[test]
    fn test_extract_validators_with_message() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: [{ validate: "email", message: "Invalid email!" }] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        assert_eq!(validators.len(), 1);
        assert!(matches!(validators[0].validator, Validator::Email));
        assert_eq!(
            validators[0].custom_message.as_deref(),
            Some("Invalid email!")
        );
        assert!(!diagnostics.has_errors());
    }

    #[test]
    fn test_extract_validators_mixed() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["nonEmpty", { validate: "email", message: "Bad email" }] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        assert_eq!(validators.len(), 2);
        assert!(matches!(validators[0].validator, Validator::NonEmpty));
        assert!(validators[0].custom_message.is_none());
        assert!(matches!(validators[1].validator, Validator::Email));
        assert_eq!(validators[1].custom_message.as_deref(), Some("Bad email"));
        assert!(!diagnostics.has_errors());
    }

    #[test]
    fn test_field_with_validators() {
        let decorator = make_decorator(r#"{ validate: ["email", "maxLength(255)"] }"#);
        let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
        let opts = result.options;
        assert_eq!(opts.validators.len(), 2);
        assert!(matches!(opts.validators[0].validator, Validator::Email));
        assert!(matches!(
            opts.validators[1].validator,
            Validator::MaxLength(255)
        ));
    }

    // ========================================================================
    // Date validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_date_validators() {
        assert!(matches!(
            parse_validator_string("validDate"),
            Ok(Validator::ValidDate)
        ));
        assert!(matches!(
            parse_validator_string(r#"greaterThanDate("2020-01-01")"#),
            Ok(Validator::GreaterThanDate(d)) if d == "2020-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"greaterThanOrEqualToDate("2020-01-01")"#),
            Ok(Validator::GreaterThanOrEqualToDate(d)) if d == "2020-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"lessThanDate("2030-01-01")"#),
            Ok(Validator::LessThanDate(d)) if d == "2030-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"lessThanOrEqualToDate("2030-01-01")"#),
            Ok(Validator::LessThanOrEqualToDate(d)) if d == "2030-01-01"
        ));
        assert!(matches!(
            parse_validator_string(r#"betweenDate("2020-01-01", "2030-12-31")"#),
            Ok(Validator::BetweenDate(min, max)) if min == "2020-01-01" && max == "2030-12-31"
        ));
    }

    // ========================================================================
    // BigInt validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_bigint_validators() {
        assert!(matches!(
            parse_validator_string("positiveBigInt"),
            Ok(Validator::PositiveBigInt)
        ));
        assert!(matches!(
            parse_validator_string("nonNegativeBigInt"),
            Ok(Validator::NonNegativeBigInt)
        ));
        assert!(matches!(
            parse_validator_string("negativeBigInt"),
            Ok(Validator::NegativeBigInt)
        ));
        assert!(matches!(
            parse_validator_string("nonPositiveBigInt"),
            Ok(Validator::NonPositiveBigInt)
        ));
        assert!(matches!(
            parse_validator_string("greaterThanBigInt(100)"),
            Ok(Validator::GreaterThanBigInt(n)) if n == "100"
        ));
        assert!(matches!(
            parse_validator_string("greaterThanOrEqualToBigInt(0)"),
            Ok(Validator::GreaterThanOrEqualToBigInt(n)) if n == "0"
        ));
        assert!(matches!(
            parse_validator_string("lessThanBigInt(1000)"),
            Ok(Validator::LessThanBigInt(n)) if n == "1000"
        ));
        assert!(matches!(
            parse_validator_string("lessThanOrEqualToBigInt(999)"),
            Ok(Validator::LessThanOrEqualToBigInt(n)) if n == "999"
        ));
        assert!(matches!(
            parse_validator_string("betweenBigInt(0, 100)"),
            Ok(Validator::BetweenBigInt(min, max)) if min == "0" && max == "100"
        ));
    }

    // ========================================================================
    // Array validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_array_validators() {
        assert!(matches!(
            parse_validator_string("maxItems(10)"),
            Ok(Validator::MaxItems(10))
        ));
        assert!(matches!(
            parse_validator_string("minItems(1)"),
            Ok(Validator::MinItems(1))
        ));
        assert!(matches!(
            parse_validator_string("itemsCount(5)"),
            Ok(Validator::ItemsCount(5))
        ));
    }

    // ========================================================================
    // Additional number validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_additional_number_validators() {
        assert!(matches!(
            parse_validator_string("nonNaN"),
            Ok(Validator::NonNaN)
        ));
        assert!(matches!(
            parse_validator_string("finite"),
            Ok(Validator::Finite)
        ));
        assert!(matches!(
            parse_validator_string("uint8"),
            Ok(Validator::Uint8)
        ));
        assert!(matches!(
            parse_validator_string("multipleOf(5)"),
            Ok(Validator::MultipleOf(n)) if n == 5.0
        ));
        assert!(matches!(
            parse_validator_string("negative"),
            Ok(Validator::Negative)
        ));
        assert!(matches!(
            parse_validator_string("nonNegative"),
            Ok(Validator::NonNegative)
        ));
        assert!(matches!(
            parse_validator_string("nonPositive"),
            Ok(Validator::NonPositive)
        ));
    }

    // ========================================================================
    // Additional string validator parsing tests
    // ========================================================================

    #[test]
    fn test_parse_additional_string_validators() {
        assert!(matches!(
            parse_validator_string("capitalized"),
            Ok(Validator::Capitalized)
        ));
        assert!(matches!(
            parse_validator_string("uncapitalized"),
            Ok(Validator::Uncapitalized)
        ));
        assert!(matches!(
            parse_validator_string("length(5, 10)"),
            Ok(Validator::LengthRange(5, 10))
        ));
    }

    // ========================================================================
    // Case sensitivity tests
    // ========================================================================

    #[test]
    fn test_parse_validators_case_insensitive() {
        // Validators should be case-insensitive
        assert!(matches!(
            parse_validator_string("EMAIL"),
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("Email"),
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("NONEMPTY"),
            Ok(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("NonEmpty"),
            Ok(Validator::NonEmpty)
        ));
        assert!(matches!(
            parse_validator_string("MAXLENGTH(10)"),
            Ok(Validator::MaxLength(10))
        ));
    }

    // ========================================================================
    // Pattern validator with special characters
    // ========================================================================

    #[test]
    fn test_parse_pattern_with_special_chars() {
        // Test pattern with various regex special chars
        assert!(matches!(
            parse_validator_string(r#"pattern("^[A-Z]{3}$")"#),
            Ok(Validator::Pattern(p)) if p == "^[A-Z]{3}$"
        ));
        assert!(matches!(
            parse_validator_string(r#"pattern("\\d+")"#),
            Ok(Validator::Pattern(p)) if p == "\\d+"
        ));
        assert!(matches!(
            parse_validator_string(r#"pattern("^test\\.json$")"#),
            Ok(Validator::Pattern(p)) if p == "^test\\.json$"
        ));
    }

    // ========================================================================
    // Edge case: validators with whitespace
    // ========================================================================

    #[test]
    fn test_parse_validators_with_whitespace() {
        assert!(matches!(
            parse_validator_string("  email  "),
            Ok(Validator::Email)
        ));
        assert!(matches!(
            parse_validator_string("between( 1 , 100 )"),
            Ok(Validator::Between(min, max)) if min == 1.0 && max == 100.0
        ));
        assert!(matches!(
            parse_validator_string("maxLength( 50 )"),
            Ok(Validator::MaxLength(50))
        ));
    }

    // ========================================================================
    // Validator error tests
    // ========================================================================

    #[test]
    fn test_unknown_validator_returns_error() {
        let result = parse_validator_string("unknownValidator");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("unknown validator"));
        assert!(err.message.contains("unknownValidator"));
    }

    #[test]
    fn test_unknown_validator_with_typo_suggests_correction() {
        // "emai" is close to "email"
        let result = parse_validator_string("emai");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.help.is_some());
        assert!(err.help.as_ref().unwrap().contains("email"));
    }

    #[test]
    fn test_unknown_validator_no_suggestion_for_unrelated() {
        // "xyz" has no similar validators
        let result = parse_validator_string("xyz");
        assert!(result.is_err());
        let err = result.unwrap_err();
        // For short strings with no matches, help may be None
        assert!(err.help.is_none() || !err.help.as_ref().unwrap().contains("email"));
    }

    #[test]
    fn test_invalid_maxlength_args_returns_error() {
        let result = parse_validator_string("maxLength(abc)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("maxLength"));
    }

    #[test]
    fn test_invalid_between_args_returns_error() {
        // between requires two numbers
        let result = parse_validator_string("between(abc, def)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("between"));
    }

    #[test]
    fn test_extract_validators_collects_errors() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["unknownValidator", "email"] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        // Should still extract the valid "email" validator
        assert_eq!(validators.len(), 1);
        assert!(matches!(validators[0].validator, Validator::Email));
        // Should have recorded an error for the unknown validator
        assert!(diagnostics.has_errors());
        assert_eq!(diagnostics.len(), 1);
    }

    #[test]
    fn test_extract_validators_multiple_errors() {
        let mut diagnostics = DiagnosticCollector::new();
        let validators = extract_validators(
            r#"{ validate: ["unknown1", "unknown2", "email"] }"#,
            span(),
            "test_field",
            &mut diagnostics,
        );
        // Should still extract the valid "email" validator
        assert_eq!(validators.len(), 1);
        // Should have recorded two errors
        assert!(diagnostics.has_errors());
        assert_eq!(diagnostics.len(), 2);
    }

    #[test]
    fn test_typo_suggestion_url_vs_uuid() {
        // "rul" is close to "url"
        let result = parse_validator_string("rul");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.help.is_some());
        assert!(err.help.as_ref().unwrap().contains("url"));
    }

    #[test]
    fn test_typo_suggestion_maxlength() {
        // "maxLenth" is close to "maxLength"
        let result = parse_validator_string("maxLenth(10)");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.help.is_some());
        assert!(err.help.as_ref().unwrap().contains("maxLength"));
    }
}
