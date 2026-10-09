use crate::macros::ts_template;
use crate::ts_syn::TsStream;

use super::super::{Validator, ValidatorSpec};
use crate::builtin::derive::common::{bigint_literal, js_string, regex_literal, rendered};

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

const EMAIL_PATTERN: &str = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";
const UUID_PATTERN: &str = r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$";

/// Statements that push an error to `errors` for each validator `value_var`
/// fails, reporting it under `field_name` of `context_name`.
pub(super) fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    field_name: &str,
    context_name: &str,
    missing: Missing,
) -> TsStream {
    let field = js_string(field_name);
    let checks: Vec<TsStream> = validators
        .iter()
        .map(|spec| {
            let condition = failure_condition(&spec.validator, value_var);
            let default_message =
                get_default_validator_message(&spec.validator, field_name, context_name);
            let message = js_string(spec.custom_message.as_ref().unwrap_or(&default_message));
            ts_template! {
                if (@{condition}) { errors.push({ field: @{&field}, message: @{message} }); }
            }
        })
        .collect();
    if checks.is_empty() {
        return TsStream::merge_all(checks);
    }
    let checks = TsStream::merge_all(checks);

    // Validators check the declared type's value. Where the type allows a
    // missing value (`T | null`, `T | undefined`, an optional field) it passes
    // unchecked; where it requires one, a missing value fails as missing rather
    // than reaching a check that assumes a value. Either guard narrows the
    // value's type for the checks inside it.
    match missing {
        Missing::Excluded => checks,
        Missing::Allowed => ts_template! {
            if (@{value_var} != null) {
                {$typescript checks}
            }
        },
        Missing::Required => {
            let message = js_string(&format!("{context_name}.{field_name} is required"));
            ts_template! {
                if (@{value_var} == null) { errors.push({ field: @{field}, message: @{message} }); } else {
                    {$typescript checks}
                }
            }
        }
    }
}

/// The expression true when `value` fails `validator`.
fn failure_condition(validator: &Validator, value: &str) -> String {
    let check = match validator {
        // String validators
        Validator::Email => {
            let pattern = regex_literal(EMAIL_PATTERN, "");
            ts_template! { !@{pattern}.test(@{value}) }
        }
        Validator::Url => ts_template! { !URL.canParse(@{value}) },
        Validator::Uuid => {
            let pattern = regex_literal(UUID_PATTERN, "i");
            ts_template! { !@{pattern}.test(@{value}) }
        }
        Validator::MaxLength(n) => ts_template! { @{value}.length > @{n} },
        Validator::MinLength(n) => ts_template! { @{value}.length < @{n} },
        Validator::Length(n) => ts_template! { @{value}.length !== @{n} },
        Validator::LengthRange(min, max) => {
            ts_template! { @{value}.length < @{min} || @{value}.length > @{max} }
        }
        Validator::Pattern(pattern) => {
            let pattern = regex_literal(pattern, "");
            ts_template! { !@{pattern}.test(@{value}) }
        }
        Validator::NonEmpty => ts_template! { @{value}.length === 0 },
        Validator::Trimmed => ts_template! { @{value} !== @{value}.trim() },
        Validator::Lowercase => ts_template! { @{value} !== @{value}.toLowerCase() },
        Validator::Uppercase => ts_template! { @{value} !== @{value}.toUpperCase() },
        Validator::Capitalized => {
            ts_template! { @{value}.length > 0 && @{value}[0] !== @{value}[0].toUpperCase() }
        }
        Validator::Uncapitalized => {
            ts_template! { @{value}.length > 0 && @{value}[0] !== @{value}[0].toLowerCase() }
        }
        Validator::StartsWith(prefix) => {
            let prefix = js_string(prefix);
            ts_template! { !@{value}.startsWith(@{prefix}) }
        }
        Validator::EndsWith(suffix) => {
            let suffix = js_string(suffix);
            ts_template! { !@{value}.endsWith(@{suffix}) }
        }
        Validator::Includes(text) => {
            let text = js_string(text);
            ts_template! { !@{value}.includes(@{text}) }
        }

        // Number validators
        Validator::GreaterThan(n) => ts_template! { @{value} <= @{n} },
        Validator::GreaterThanOrEqualTo(n) => ts_template! { @{value} < @{n} },
        Validator::LessThan(n) => ts_template! { @{value} >= @{n} },
        Validator::LessThanOrEqualTo(n) => ts_template! { @{value} > @{n} },
        Validator::Between(min, max) => ts_template! { @{value} < @{min} || @{value} > @{max} },
        Validator::Int => ts_template! { !Number.isInteger(@{value}) },
        Validator::NonNaN => ts_template! { Number.isNaN(@{value}) },
        Validator::Finite => ts_template! { !Number.isFinite(@{value}) },
        Validator::Positive => ts_template! { @{value} <= 0 },
        Validator::NonNegative => ts_template! { @{value} < 0 },
        Validator::Negative => ts_template! { @{value} >= 0 },
        Validator::NonPositive => ts_template! { @{value} > 0 },
        Validator::MultipleOf(n) => {
            crate::host::import_registry::with_registry_mut(|registry| {
                registry.request_import(
                    "__mf_isMultipleOf",
                    Some("isMultipleOf"),
                    crate::package::ENDEC,
                    false,
                );
            });
            let divisor = n.abs();
            ts_template! { !__mf_isMultipleOf(@{value}, @{divisor}) }
        }
        Validator::Uint8 => {
            ts_template! { !Number.isInteger(@{value}) || @{value} < 0 || @{value} > 255 }
        }
        Validator::NonNegativeInt => ts_template! { !Number.isInteger(@{value}) || @{value} < 0 },

        // Array validators
        Validator::MaxItems(n) => ts_template! { @{value}.length > @{n} },
        Validator::MinItems(n) => ts_template! { @{value}.length < @{n} },
        Validator::ItemsCount(n) => ts_template! { @{value}.length !== @{n} },

        // Date validators. `validDate` only makes sense on a string value;
        // parse it before checking (`new Date` also accepts a `Date`).
        Validator::ValidDate => ts_template! { isNaN(new Date(@{value}).getTime()) },
        Validator::GreaterThanDate(date) => {
            let date = js_string(date);
            ts_template! { @{value}.getTime() <= new Date(@{date}).getTime() }
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            let date = js_string(date);
            ts_template! { @{value}.getTime() < new Date(@{date}).getTime() }
        }
        Validator::LessThanDate(date) => {
            let date = js_string(date);
            ts_template! { @{value}.getTime() >= new Date(@{date}).getTime() }
        }
        Validator::LessThanOrEqualToDate(date) => {
            let date = js_string(date);
            ts_template! { @{value}.getTime() > new Date(@{date}).getTime() }
        }
        Validator::BetweenDate(min, max) => {
            let (min, max) = (js_string(min), js_string(max));
            ts_template! {
                @{value}.getTime() < new Date(@{min}).getTime() || @{value}.getTime() > new Date(@{max}).getTime()
            }
        }

        // BigInt validators
        Validator::GreaterThanBigInt(n) => {
            let n = bigint_literal(n);
            ts_template! { @{value} <= @{n} }
        }
        Validator::GreaterThanOrEqualToBigInt(n) => {
            let n = bigint_literal(n);
            ts_template! { @{value} < @{n} }
        }
        Validator::LessThanBigInt(n) => {
            let n = bigint_literal(n);
            ts_template! { @{value} >= @{n} }
        }
        Validator::LessThanOrEqualToBigInt(n) => {
            let n = bigint_literal(n);
            ts_template! { @{value} > @{n} }
        }
        Validator::BetweenBigInt(min, max) => {
            let (min, max) = (bigint_literal(min), bigint_literal(max));
            ts_template! { @{value} < @{min} || @{value} > @{max} }
        }
        Validator::PositiveBigInt => ts_template! { @{value} <= 0n },
        Validator::NonNegativeBigInt => ts_template! { @{value} < 0n },
        Validator::NegativeBigInt => ts_template! { @{value} >= 0n },
        Validator::NonPositiveBigInt => ts_template! { @{value} > 0n },

        // Custom validator, imported into the expanded file when it names
        // a source
        Validator::Custom(custom) => {
            let callee = custom.callee();
            if let Some(source) = &custom.source {
                crate::host::import_registry::with_registry_mut(|registry| {
                    registry.request_import(&callee, Some(&custom.function), source, false);
                });
            }
            ts_template! { !@{callee}(@{value}) }
        }
    };
    rendered(check)
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
