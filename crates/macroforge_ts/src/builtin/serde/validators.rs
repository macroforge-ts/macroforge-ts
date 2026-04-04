//! Validator types and parsing for field validation during deserialization.

use super::helpers::{extract_named_string, parse_string_literal};
use crate::ts_syn::abi::{DiagnosticCollector, SpanIR};

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
    pub help: Option<String>,
}

impl ValidatorParseError {
    /// Create error for an unknown validator name
    pub fn unknown_validator(name: &str) -> Self {
        let similar = find_similar_validator(name);
        Self {
            message: format!("unknown validator '{}'", name),
            help: similar.map(|s| format!("did you mean '{}'?", s)),
        }
    }

    /// Create error for invalid arguments
    pub fn invalid_args(name: &str, reason: &str) -> Self {
        Self {
            message: format!("invalid arguments for '{}': {}", name, reason),
            help: None,
        }
    }
}

/// List of all known validator names for typo detection
const KNOWN_VALIDATORS: &[&str] = &[
    "email",
    "url",
    "uuid",
    "maxLength",
    "minLength",
    "length",
    "pattern",
    "nonEmpty",
    "trimmed",
    "lowercase",
    "uppercase",
    "capitalized",
    "uncapitalized",
    "startsWith",
    "endsWith",
    "includes",
    "greaterThan",
    "greaterThanOrEqualTo",
    "lessThan",
    "lessThanOrEqualTo",
    "between",
    "int",
    "nonNaN",
    "finite",
    "positive",
    "nonNegative",
    "negative",
    "nonPositive",
    "multipleOf",
    "uint8",
    "maxItems",
    "minItems",
    "itemsCount",
    "validDate",
    "greaterThanDate",
    "greaterThanOrEqualToDate",
    "lessThanDate",
    "lessThanOrEqualToDate",
    "betweenDate",
    "positiveBigInt",
    "nonNegativeBigInt",
    "negativeBigInt",
    "nonPositiveBigInt",
    "greaterThanBigInt",
    "greaterThanOrEqualToBigInt",
    "lessThanBigInt",
    "lessThanOrEqualToBigInt",
    "betweenBigInt",
    "custom",
];

/// Find a similar validator name for typo suggestions using Levenshtein distance
fn find_similar_validator(name: &str) -> Option<&'static str> {
    let name_lower = name.to_lowercase();
    KNOWN_VALIDATORS
        .iter()
        .filter_map(|v| {
            let dist = levenshtein_distance(&v.to_lowercase(), &name_lower);
            if dist <= 2 { Some((*v, dist)) } else { None }
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

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    let mut prev_row: Vec<usize> = (0..=len_b).collect();
    let mut curr_row: Vec<usize> = vec![0; len_b + 1];

    for i in 1..=len_a {
        curr_row[0] = i;
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr_row[j] = (prev_row[j] + 1)
                .min(curr_row[j - 1] + 1)
                .min(prev_row[j - 1] + cost);
        }
        std::mem::swap(&mut prev_row, &mut curr_row);
    }
    prev_row[len_b]
}

// ============================================================================
// Validator parsing functions
// ============================================================================

/// Known options that are NOT validators (to avoid false positives)
const KNOWN_OPTIONS: &[&str] = &[
    "skip",
    "skipSerializing",
    "skipDeserializing",
    "flatten",
    "default",
    "rename",
    "validate",
    "message",
    "serializeWith",
    "deserializeWith",
    "format",
];

/// Extract validators from decorator arguments with diagnostic collection
/// Supports:
/// - Explicit array: validate: ["email", "maxLength(255)"]
/// - Object array: validate: [{ validate: "email", message: "..." }]
/// - Shorthand: @serde(email) or @serde(minLength(2), maxLength(50))
pub fn extract_validators(
    args: &str,
    decorator_span: SpanIR,
    field_name: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    // First, check for explicit validate: [...] format
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
                    format!(
                        "field '{}': validate must be an array, e.g., validate: [\"email\"]",
                        field_name
                    ),
                );
                return validators;
            }
        }
    }

    // Parse shorthand validators: @serde(email) or @serde(minLength(2), maxLength(50))
    // Strip outer braces if args are an object literal: { key: value, ... }
    let args_inner = args.trim();
    let args_inner = if args_inner.starts_with('{') && args_inner.ends_with('}') {
        &args_inner[1..args_inner.len() - 1]
    } else {
        args_inner
    };
    for item in split_decorator_args(args_inner) {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        // Skip known options (case-insensitive comparison)
        let base_name = item.split('(').next().unwrap_or(item);
        let base_name = base_name.split(':').next().unwrap_or(base_name).trim();

        if KNOWN_OPTIONS
            .iter()
            .any(|o| o.eq_ignore_ascii_case(base_name))
        {
            continue;
        }

        // Check if this looks like a validator (either a known name or has function syntax)
        let is_likely_validator = KNOWN_VALIDATORS
            .iter()
            .any(|v| v.eq_ignore_ascii_case(base_name))
            || item.contains('('); // Function-like syntax suggests validator

        if is_likely_validator {
            match parse_validator_string(item) {
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

    validators
}

/// Split decorator arguments by commas, respecting nested parentheses and strings
fn split_decorator_args(input: &str) -> Vec<String> {
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
            '(' | '[' | '{' => {
                depth += 1;
                current.push(c);
            }
            ')' | ']' | '}' => {
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
        diagnostics.error(
            decorator_span,
            format!("field '{}': malformed validator array", field_name),
        );
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
pub(super) fn parse_validator_string(s: &str) -> Result<Validator, ValidatorParseError> {
    let trimmed = s.trim();

    // Check for function-call style: name(args)
    if let Some(paren_idx) = trimmed.find('(') {
        let name = &trimmed[..paren_idx];
        let Some(args_end) = trimmed.rfind(')') else {
            return Err(ValidatorParseError::invalid_args(
                name,
                "missing closing parenthesis",
            ));
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
                1 => parts[0].trim().parse().map(Validator::Length).map_err(|_| {
                    ValidatorParseError::invalid_args(name, "expected a positive integer")
                }),
                2 => {
                    let min = parts[0].trim().parse().map_err(|_| {
                        ValidatorParseError::invalid_args(name, "expected two positive integers")
                    })?;
                    let max = parts[1].trim().parse().map_err(|_| {
                        ValidatorParseError::invalid_args(name, "expected two positive integers")
                    })?;
                    Ok(Validator::LengthRange(min, max))
                }
                _ => Err(ValidatorParseError::invalid_args(
                    name,
                    "expected 1 or 2 arguments",
                )),
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
                Err(ValidatorParseError::invalid_args(
                    name,
                    "expected two numbers separated by comma",
                ))
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
                let min = parse_validator_string_arg(parts[0].trim()).ok_or_else(|| {
                    ValidatorParseError::invalid_args(name, "expected two date strings")
                })?;
                let max = parse_validator_string_arg(parts[1].trim()).ok_or_else(|| {
                    ValidatorParseError::invalid_args(name, "expected two date strings")
                })?;
                Ok(Validator::BetweenDate(min, max))
            } else {
                Err(ValidatorParseError::invalid_args(
                    name,
                    "expected two date strings separated by comma",
                ))
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
                Err(ValidatorParseError::invalid_args(
                    name,
                    "expected two bigint values separated by comma",
                ))
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
