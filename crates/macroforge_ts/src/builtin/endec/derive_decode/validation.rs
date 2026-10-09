use crate::ts_syn::TsStream;

use super::super::{Validator, ValidatorSpec};
use crate::builtin::derive_common::js_string;

/// `pattern` as the body of a `/.../` regex literal: each unescaped `/` is
/// escaped, and everything already escaped is kept as written.
fn regex_literal_body(pattern: &str) -> String {
    let mut body = String::with_capacity(pattern.len());
    let mut escaped = false;
    for c in pattern.chars() {
        if c == '/' && !escaped {
            body.push('\\');
        }
        escaped = c == '\\' && !escaped;
        body.push(c);
    }
    body
}

/// What a missing (`null` or `undefined`) value means where validators run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Missing {
    /// The type admits a missing value, which passes unchecked.
    Allowed,
    /// The type requires a value, so a missing one fails as missing.
    Required,
    /// An earlier check, such as a `typeof` test, already proved a value.
    Excluded,
}

/// Generate TypeScript validation code for a field
///
/// This function generates inline validation code that pushes errors to an `errors` array
/// when validation fails. It's used in the decoding process to validate field values.
///
/// # Arguments
/// * `validators` - List of validators to apply
/// * `value_var` - Variable name containing the value to validate (e.g., "__val", "__raw")
/// * `field_name` - JSON field name for error reporting
/// * `context_name` - Context name for error messages (e.g., class name)
/// * `missing` - What a missing value means at this point
///
/// # Returns
/// A TypeScript expression that runs the validation statements.
pub(super) fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    field_name: &str,
    context_name: &str,
    missing: Missing,
) -> TsStream {
    let mut code = String::new();

    for spec in validators {
        let condition = match &spec.validator {
            // String validators
            Validator::Email => format!("!/^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$/.test({value_var})"),
            Validator::Url => format!("!URL.canParse({value_var})"),
            Validator::Uuid => format!(
                "!/^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{12}}$/i.test({value_var})"
            ),
            Validator::MaxLength(n) => format!("{value_var}.length > {n}"),
            Validator::MinLength(n) => format!("{value_var}.length < {n}"),
            Validator::Length(n) => format!("{value_var}.length !== {n}"),
            Validator::LengthRange(min, max) => {
                format!("{value_var}.length < {min} || {value_var}.length > {max}")
            }
            Validator::Pattern(pattern) => {
                format!("!/{}/.test({value_var})", regex_literal_body(pattern))
            }
            Validator::NonEmpty => format!("{value_var}.length === 0"),
            Validator::Trimmed => format!("{value_var} !== {value_var}.trim()"),
            Validator::Lowercase => format!("{value_var} !== {value_var}.toLowerCase()"),
            Validator::Uppercase => format!("{value_var} !== {value_var}.toUpperCase()"),
            Validator::Capitalized => {
                format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toUpperCase()")
            }
            Validator::Uncapitalized => {
                format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toLowerCase()")
            }
            Validator::StartsWith(prefix) => {
                format!("!{value_var}.startsWith({})", js_string(prefix))
            }
            Validator::EndsWith(suffix) => format!("!{value_var}.endsWith({})", js_string(suffix)),
            Validator::Includes(text) => format!("!{value_var}.includes({})", js_string(text)),

            // Number validators
            Validator::GreaterThan(n) => format!("{value_var} <= {n}"),
            Validator::GreaterThanOrEqualTo(n) => format!("{value_var} < {n}"),
            Validator::LessThan(n) => format!("{value_var} >= {n}"),
            Validator::LessThanOrEqualTo(n) => format!("{value_var} > {n}"),
            Validator::Between(min, max) => format!("{value_var} < {min} || {value_var} > {max}"),
            Validator::Int => format!("!Number.isInteger({value_var})"),
            Validator::NonNaN => format!("Number.isNaN({value_var})"),
            Validator::Finite => format!("!Number.isFinite({value_var})"),
            Validator::Positive => format!("{value_var} <= 0"),
            Validator::NonNegative => format!("{value_var} < 0"),
            Validator::Negative => format!("{value_var} >= 0"),
            Validator::NonPositive => format!("{value_var} > 0"),
            Validator::MultipleOf(n) => {
                crate::host::import_registry::with_registry_mut(|registry| {
                    registry.request_import(
                        "__mf_isMultipleOf",
                        Some("isMultipleOf"),
                        crate::package::ENDEC,
                        false,
                    );
                });
                format!("!__mf_isMultipleOf({value_var}, {})", n.abs())
            }
            Validator::Uint8 => {
                format!("!Number.isInteger({value_var}) || {value_var} < 0 || {value_var} > 255")
            }
            Validator::NonNegativeInt => {
                format!("!Number.isInteger({value_var}) || {value_var} < 0")
            }

            // Array validators
            Validator::MaxItems(n) => format!("{value_var}.length > {n}"),
            Validator::MinItems(n) => format!("{value_var}.length < {n}"),
            Validator::ItemsCount(n) => format!("{value_var}.length !== {n}"),

            // Date validators. `validDate` only makes sense on a string value;
            // parse it before checking (`new Date` also accepts a `Date`).
            Validator::ValidDate => format!("isNaN(new Date({value_var}).getTime())"),
            Validator::GreaterThanDate(date) => {
                format!(
                    "{value_var}.getTime() <= new Date({}).getTime()",
                    js_string(date)
                )
            }
            Validator::GreaterThanOrEqualToDate(date) => {
                format!(
                    "{value_var}.getTime() < new Date({}).getTime()",
                    js_string(date)
                )
            }
            Validator::LessThanDate(date) => {
                format!(
                    "{value_var}.getTime() >= new Date({}).getTime()",
                    js_string(date)
                )
            }
            Validator::LessThanOrEqualToDate(date) => {
                format!(
                    "{value_var}.getTime() > new Date({}).getTime()",
                    js_string(date)
                )
            }
            Validator::BetweenDate(min, max) => format!(
                "{value_var}.getTime() < new Date({}).getTime() || {value_var}.getTime() > new Date({}).getTime()",
                js_string(min),
                js_string(max)
            ),

            // BigInt validators
            Validator::GreaterThanBigInt(n) => format!("{value_var} <= {n}n"),
            Validator::GreaterThanOrEqualToBigInt(n) => format!("{value_var} < {n}n"),
            Validator::LessThanBigInt(n) => format!("{value_var} >= {n}n"),
            Validator::LessThanOrEqualToBigInt(n) => format!("{value_var} > {n}n"),
            Validator::BetweenBigInt(min, max) => {
                format!("{value_var} < {min}n || {value_var} > {max}n")
            }
            Validator::PositiveBigInt => format!("{value_var} <= 0n"),
            Validator::NonNegativeBigInt => format!("{value_var} < 0n"),
            Validator::NegativeBigInt => format!("{value_var} >= 0n"),
            Validator::NonPositiveBigInt => format!("{value_var} > 0n"),

            // Custom validator, imported into the expanded file when it names
            // a source
            Validator::Custom(custom) => {
                if let Some(source) = &custom.source {
                    crate::host::import_registry::with_registry_mut(|registry| {
                        registry.request_import(
                            &custom.callee(),
                            Some(&custom.function),
                            source,
                            false,
                        );
                    });
                }
                format!("!{}({value_var})", custom.callee())
            }
        };

        let default_message =
            get_default_validator_message(&spec.validator, field_name, context_name);
        let message = spec.custom_message.as_ref().unwrap_or(&default_message);

        code.push_str(&format!(
            "if ({condition}) {{ errors.push({{ field: {}, message: {} }}); }}\n",
            js_string(field_name),
            js_string(message)
        ));
    }

    // Validators check the declared type's value. Where the type allows a
    // missing value (`T | null`, `T | undefined`, an optional field) it passes
    // unchecked; where it requires one, a missing value fails as missing rather
    // than reaching a check that assumes a value. Either guard narrows the
    // value's type for the checks inside it.
    if code.is_empty() {
        return TsStream::from_string(code);
    }
    match missing {
        Missing::Excluded => TsStream::from_string(code),
        Missing::Allowed => {
            TsStream::from_string(format!("if ({value_var} != null) {{\n{code}}}\n"))
        }
        Missing::Required => {
            let message = js_string(&format!("{context_name}.{field_name} is required"));
            let field = js_string(field_name);
            TsStream::from_string(format!(
                "if ({value_var} == null) {{ errors.push({{ field: {field}, message: {message} }}); }} else {{\n{code}}}\n"
            ))
        }
    }
}

/// Generate default error message for a validator
pub(super) fn get_default_validator_message(
    validator: &Validator,
    field_name: &str,
    context_name: &str,
) -> String {
    // `_root` validates the value itself, as for a primitive alias.
    let subject = if field_name == "_root" {
        context_name.to_string()
    } else {
        format!("{context_name}.{field_name}")
    };
    match validator {
        Validator::Email => format!("{subject} must be a valid email"),
        Validator::Url => format!("{subject} must be a valid URL"),
        Validator::Uuid => format!("{subject} must be a valid UUID"),
        Validator::MaxLength(n) => {
            format!("{subject} must have at most {n} characters")
        }
        Validator::MinLength(n) => {
            format!("{subject} must have at least {n} characters")
        }
        Validator::Length(n) => {
            format!("{subject} must have exactly {n} characters")
        }
        Validator::LengthRange(min, max) => {
            format!("{subject} must have between {min} and {max} characters")
        }
        Validator::Pattern(pattern) => {
            format!("{subject} must match pattern {pattern}")
        }
        Validator::NonEmpty => format!("{subject} must not be empty"),
        Validator::Trimmed => format!("{subject} must be trimmed"),
        Validator::Lowercase => format!("{subject} must be lowercase"),
        Validator::Uppercase => format!("{subject} must be uppercase"),
        Validator::Capitalized => format!("{subject} must be capitalized"),
        Validator::Uncapitalized => format!("{subject} must be uncapitalized"),
        Validator::StartsWith(prefix) => {
            format!("{subject} must start with '{prefix}'")
        }
        Validator::EndsWith(suffix) => {
            format!("{subject} must end with '{suffix}'")
        }
        Validator::Includes(text) => format!("{subject} must include '{text}'"),
        Validator::GreaterThan(n) => {
            format!("{subject} must be greater than {n}")
        }
        Validator::GreaterThanOrEqualTo(n) => {
            format!("{subject} must be greater than or equal to {n}")
        }
        Validator::LessThan(n) => format!("{subject} must be less than {n}"),
        Validator::LessThanOrEqualTo(n) => {
            format!("{subject} must be less than or equal to {n}")
        }
        Validator::Between(min, max) => {
            format!("{subject} must be between {min} and {max}")
        }
        Validator::Int => format!("{subject} must be an integer"),
        Validator::NonNaN => format!("{subject} must not be NaN"),
        Validator::Finite => format!("{subject} must be finite"),
        Validator::Positive => format!("{subject} must be positive"),
        Validator::NonNegative => format!("{subject} must be non-negative"),
        Validator::Negative => format!("{subject} must be negative"),
        Validator::NonPositive => format!("{subject} must be non-positive"),
        Validator::MultipleOf(n) => {
            format!("{subject} must be a multiple of {n}")
        }
        Validator::Uint8 => format!("{subject} must be a uint8"),
        Validator::NonNegativeInt => {
            format!("{subject} must be a non-negative integer")
        }
        Validator::MaxItems(n) => {
            format!("{subject} must have at most {n} items")
        }
        Validator::MinItems(n) => {
            format!("{subject} must have at least {n} items")
        }
        Validator::ItemsCount(n) => {
            format!("{subject} must have exactly {n} items")
        }
        Validator::ValidDate => format!("{subject} must be a valid date"),
        Validator::GreaterThanDate(date) => {
            format!("{subject} must be after {date}")
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!("{subject} must be on or after {date}")
        }
        Validator::LessThanDate(date) => {
            format!("{subject} must be before {date}")
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!("{subject} must be on or before {date}")
        }
        Validator::BetweenDate(min, max) => {
            format!("{subject} must be between {min} and {max}")
        }
        Validator::GreaterThanBigInt(n) => {
            format!("{subject} must be greater than {n}")
        }
        Validator::GreaterThanOrEqualToBigInt(n) => {
            format!("{subject} must be greater than or equal to {n}")
        }
        Validator::LessThanBigInt(n) => {
            format!("{subject} must be less than {n}")
        }
        Validator::LessThanOrEqualToBigInt(n) => {
            format!("{subject} must be less than or equal to {n}")
        }
        Validator::BetweenBigInt(min, max) => {
            format!("{subject} must be between {min} and {max}")
        }
        Validator::PositiveBigInt => format!("{subject} must be positive"),
        Validator::NonNegativeBigInt => format!("{subject} must be non-negative"),
        Validator::NegativeBigInt => format!("{subject} must be negative"),
        Validator::NonPositiveBigInt => format!("{subject} must be non-positive"),
        Validator::Custom(custom) => {
            format!("{subject} failed custom validation ({})", custom.function)
        }
    }
}
