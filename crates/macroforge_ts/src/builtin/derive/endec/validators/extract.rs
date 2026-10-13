//! Reading validators out of `@endec(...)` decorator arguments.

use super::parse::{parse_validator_object, parse_validator_string};
use super::suggest::KNOWN_VALIDATORS;
use super::{ValidatorParseError, ValidatorSpec};
use crate::builtin::derive::common::{
    OptionValue, decorator_options, find_option, split_top_level_items, unwrapped,
    whole_string_literal,
};
use crate::builtin::derive::endec::options::endec_option;
use crate::ts_syn::abi::{DiagnosticCollector, SpanIR};

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
    // The explicit form, `validate: [...]`, holds every validator there is.
    if let Some(listed) = find_option(args, "validate") {
        return match listed.value {
            OptionValue::Assigned(list) if list.starts_with('[') => {
                parse_validator_array(list, decorator_span, subject, diagnostics)
            }
            OptionValue::Bare | OptionValue::Assigned(_) | OptionValue::Called(_) => {
                diagnostics.error(
                    decorator_span,
                    format!(
                        "{}: validate must be an array, e.g., validate: [\"email\"]",
                        subject
                    ),
                );
                Vec::new()
            }
        };
    }

    // The shorthand: @endec(email) or @endec(minLength(2), maxLength(50)).
    let mut validators = Vec::new();
    for option in decorator_options(args) {
        if endec_option(option.name).is_some() {
            continue;
        }
        let names_validator = KNOWN_VALIDATORS
            .iter()
            .any(|known| known.eq_ignore_ascii_case(option.name));
        // A call reads as a validator, whose arguments are then checked.
        if names_validator || matches!(option.value, OptionValue::Called(_)) {
            match parse_validator_string(option.text) {
                Ok(validator) => validators.push(ValidatorSpec {
                    validator,
                    custom_message: None,
                }),
                Err(err) => report(diagnostics, decorator_span, subject, err),
            }
        } else {
            // What is neither an option nor a validator would otherwise
            // check nothing, as a misspelled validator does.
            let written = if option.name.is_empty() {
                option.text
            } else {
                option.name
            };
            let unknown = ValidatorParseError::unknown_validator(written);
            let message = format!("{subject}: unknown option or validator '{written}'");
            match unknown.help {
                Some(help) => diagnostics.error_with_help(decorator_span, message, help),
                None => diagnostics.error(decorator_span, message),
            }
        }
    }

    validators
}

fn report(
    diagnostics: &mut DiagnosticCollector,
    span: SpanIR,
    subject: &str,
    err: ValidatorParseError,
) {
    let message = format!("{subject}: {}", err.message);
    match err.help {
        Some(help) => diagnostics.error_with_help(span, message, help),
        None => diagnostics.error(span, message),
    }
}

/// The validators of an array: `["email", "maxLength(255)"]`, each a string
/// or `{ validate: "...", message: "..." }`.
pub(super) fn parse_validator_array(
    input: &str,
    decorator_span: SpanIR,
    subject: &str,
    diagnostics: &mut DiagnosticCollector,
) -> Vec<ValidatorSpec> {
    let Some(content) = unwrapped(input, '[', ']') else {
        diagnostics.error(
            decorator_span,
            format!("{}: malformed validator array", subject),
        );
        return Vec::new();
    };

    let mut validators = Vec::new();
    for item in split_top_level_items(content) {
        let parsed = if item.starts_with('{') {
            parse_validator_object(item)
        } else {
            match whole_string_literal(item) {
                Some(written) => parse_validator_string(&written).map(|validator| ValidatorSpec {
                    validator,
                    custom_message: None,
                }),
                // Anything else would check nothing.
                None => Err(ValidatorParseError::not_a_validator(item)),
            }
        };
        match parsed {
            Ok(spec) => validators.push(spec),
            Err(err) => report(diagnostics, decorator_span, subject, err),
        }
    }
    validators
}
