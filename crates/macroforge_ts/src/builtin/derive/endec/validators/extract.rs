//! Reading validators out of `@endec(...)` decorator arguments.

use super::ValidatorSpec;
use super::parse::{parse_validator_object, parse_validator_string};
use super::suggest::KNOWN_VALIDATORS;
use crate::builtin::derive::common::parse_string_literal;
use crate::ts_syn::abi::{DiagnosticCollector, SpanIR};

// Validator parsing functions
// ============================================================================

/// Known options that are NOT validators (to avoid false positives)
pub(super) const KNOWN_OPTIONS: &[&str] = &[
    "skip",
    "skipEncoding",
    "skipDecoding",
    "flatten",
    "default",
    "rename",
    "validate",
    "message",
    "encodeWith",
    "decodeWith",
    "format",
];

/// Extract validators from decorator arguments with diagnostic collection
/// Supports:
/// - Explicit array: validate: ["email", "maxLength(255)"]
/// - Object array: validate: [{ validate: "email", message: "..." }]
/// - Shorthand: @endec(email) or @endec(minLength(2), maxLength(50))
///
/// `subject` prefixes diagnostics, e.g. `field 'email'` or `type 'Meters'`.
pub fn extract_validators(
    args: &str,
    decorator_span: SpanIR,
    subject: &str,
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
                return parse_validator_array(value_start, decorator_span, subject, diagnostics);
            } else {
                diagnostics.error(
                    decorator_span,
                    format!(
                        "{}: validate must be an array, e.g., validate: [\"email\"]",
                        subject
                    ),
                );
                return validators;
            }
        }
    }

    // Parse shorthand validators: @endec(email) or @endec(minLength(2), maxLength(50))
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
                            format!("{}: {}", subject, err.message),
                            help,
                        );
                    } else {
                        diagnostics.error(decorator_span, format!("{}: {}", subject, err.message));
                    }
                }
            }
        }
    }

    validators
}

/// Split decorator arguments by commas, respecting nested parentheses and strings
pub(super) fn split_decorator_args(input: &str) -> Vec<String> {
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
pub(super) fn parse_validator_array(
    input: &str,
    decorator_span: SpanIR,
    subject: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let mut validators = Vec::new();

    // Find matching ] bracket
    let Some(content) = extract_bracket_content(input, '[', ']') else {
        diagnostics.error(
            decorator_span,
            format!("{}: malformed validator array", subject),
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
                            format!("{}: {}", subject, err.message),
                            help,
                        );
                    } else {
                        diagnostics.error(decorator_span, format!("{}: {}", subject, err.message));
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
                                format!("{}: {}", subject, err.message),
                                help,
                            );
                        } else {
                            diagnostics
                                .error(decorator_span, format!("{}: {}", subject, err.message));
                        }
                    }
                }
            }
        }
    }
    validators
}

/// Extract content between matching brackets
pub(super) fn extract_bracket_content(input: &str, open: char, close: char) -> Option<String> {
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
pub(super) fn split_array_items(input: &str) -> Vec<String> {
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
