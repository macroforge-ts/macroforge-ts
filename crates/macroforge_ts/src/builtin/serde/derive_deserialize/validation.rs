use crate::ts_syn::TsStream;

use super::super::{Validator, ValidatorSpec};

/// Generates a JavaScript boolean expression that evaluates to `true` when validation fails.
///
/// This function produces the *failure condition* - the expression should be used in
/// `if (condition) { errors.push(...) }` to detect invalid values.
///
/// # Arguments
///
/// * `validator` - The validator type to generate a condition for
/// * `value_var` - The variable name containing the value to validate
///
/// # Returns
///
/// A string containing a JavaScript boolean expression. The expression evaluates to
/// `true` when the value is **invalid** (fails validation).
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::serde::Validator;
/// use macroforge_ts::builtin::serde::derive_deserialize::generate_validation_condition;
///
/// let condition = generate_validation_condition(&Validator::Email, "email");
/// assert!(condition.contains("test(email)"));
///
/// let condition = generate_validation_condition(&Validator::MaxLength(100), "name");
/// assert_eq!(condition, "name.length > 100");
/// ```
pub fn generate_validation_condition(validator: &Validator, value_var: &str) -> String {
    match validator {
        // String validators
        Validator::Email => {
            format!(r#"!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test({value_var})"#)
        }
        Validator::Url => {
            format!(
                r#"(() => {{ try {{ new URL({value_var}); return false; }} catch {{ return true; }} }})()"#
            )
        }
        Validator::Uuid => {
            format!(
                r#"!/^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[1-5][0-9a-f]{{3}}-[89ab][0-9a-f]{{3}}-[0-9a-f]{{12}}$/i.test({value_var})"#
            )
        }
        Validator::MaxLength(n) => format!("{value_var}.length > {n}"),
        Validator::MinLength(n) => format!("{value_var}.length < {n}"),
        Validator::Length(n) => format!("{value_var}.length !== {n}"),
        Validator::LengthRange(min, max) => {
            format!("{value_var}.length < {min} || {value_var}.length > {max}")
        }
        Validator::Pattern(regex) => {
            // For regex literals (/.../), only escape the delimiter `/`
            let escaped = regex.replace('/', "\\/");
            format!("!/{escaped}/.test({value_var})")
        }
        Validator::NonEmpty => format!("{value_var} != null && {value_var}.length === 0"),
        Validator::Trimmed => format!("{value_var} !== {value_var}.trim()"),
        Validator::Lowercase => format!("{value_var} !== {value_var}.toLowerCase()"),
        Validator::Uppercase => format!("{value_var} !== {value_var}.toUpperCase()"),
        Validator::Capitalized => {
            format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toUpperCase()")
        }
        Validator::Uncapitalized => {
            format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toLowerCase()")
        }
        Validator::StartsWith(prefix) => format!(r#"!{value_var}.startsWith("{prefix}")"#),
        Validator::EndsWith(suffix) => format!(r#"!{value_var}.endsWith("{suffix}")"#),
        Validator::Includes(substr) => format!(r#"!{value_var}.includes("{substr}")"#),

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
        Validator::MultipleOf(n) => format!("{value_var} % {n} !== 0"),
        Validator::Uint8 => {
            format!("!Number.isInteger({value_var}) || {value_var} < 0 || {value_var} > 255")
        }

        // Array validators
        Validator::MaxItems(n) => format!("{value_var}.length > {n}"),
        Validator::MinItems(n) => format!("{value_var}.length < {n}"),
        Validator::ItemsCount(n) => format!("{value_var}.length !== {n}"),

        // Date validators (null-safe: JSON.stringify converts Invalid Date to null)
        Validator::ValidDate => format!("{value_var} == null || isNaN({value_var}.getTime())"),
        Validator::GreaterThanDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() <= new Date("{date}").getTime()"#
            )
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() < new Date("{date}").getTime()"#
            )
        }
        Validator::LessThanDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() >= new Date("{date}").getTime()"#
            )
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() > new Date("{date}").getTime()"#
            )
        }
        Validator::BetweenDate(min, max) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() < new Date("{min}").getTime() || {value_var}.getTime() > new Date("{max}").getTime()"#
            )
        }

        // BigInt validators
        Validator::GreaterThanBigInt(n) => format!("{value_var} <= BigInt({n})"),
        Validator::GreaterThanOrEqualToBigInt(n) => format!("{value_var} < BigInt({n})"),
        Validator::LessThanBigInt(n) => format!("{value_var} >= BigInt({n})"),
        Validator::LessThanOrEqualToBigInt(n) => format!("{value_var} > BigInt({n})"),
        Validator::BetweenBigInt(min, max) => {
            format!("{value_var} < BigInt({min}) || {value_var} > BigInt({max})")
        }
        Validator::PositiveBigInt => format!("{value_var} <= 0n"),
        Validator::NonNegativeBigInt => format!("{value_var} < 0n"),
        Validator::NegativeBigInt => format!("{value_var} >= 0n"),
        Validator::NonPositiveBigInt => format!("{value_var} > 0n"),

        // Custom validator - handled specially
        Validator::Custom(_) => String::new(),
    }
}

/// Generate TypeScript validation code for a field
///
/// This function generates inline validation code that pushes errors to an `errors` array
/// when validation fails. It's used in the deserialization process to validate field values.
///
/// # Arguments
/// * `validators` - List of validators to apply
/// * `value_var` - Variable name containing the value to validate (e.g., "__val", "__raw")
/// * `field_name` - JSON field name for error reporting
/// * `context_name` - Context name for error messages (e.g., class name)
///
/// # Returns
/// A TypeScript expression that runs the validation statements.
pub(super) fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    field_name: &str,
    context_name: &str,
) -> TsStream {
    let mut code = String::new();

    for spec in validators {
        let condition = match &spec.validator {
            // String validators
            Validator::Email => format!("!/^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$/.test({value_var})"),
            Validator::Url => format!("!/^https?:\\/\\/.+/.test({value_var})"),
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
                // For regex literals (/.../), only escape the delimiter `/`
                // Backslashes like \d, \s, \+ are valid regex syntax and must NOT be double-escaped
                let escaped = pattern.replace('/', "\\/");
                format!("!/{escaped}/.test({value_var})")
            }
            Validator::NonEmpty => {
                format!("{value_var} !== null && {value_var}.trim().length === 0")
            }
            Validator::Trimmed => format!("{value_var} !== {value_var}.trim()"),
            Validator::Lowercase => format!("{value_var} !== {value_var}.toLowerCase()"),
            Validator::Uppercase => format!("{value_var} !== {value_var}.toUpperCase()"),
            Validator::Capitalized => format!(
                "{value_var}.length === 0 || {value_var}[0] !== {value_var}[0].toUpperCase() || {value_var}.slice(1) !== {value_var}.slice(1).toLowerCase()"
            ),
            Validator::Uncapitalized => {
                format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toLowerCase()")
            }
            Validator::StartsWith(prefix) => format!("!{value_var}.startsWith(\"{prefix}\")"),
            Validator::EndsWith(suffix) => format!("!{value_var}.endsWith(\"{suffix}\")"),
            Validator::Includes(text) => format!("!{value_var}.includes(\"{text}\")"),

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
            Validator::MultipleOf(n) => format!("{value_var} % {n} !== 0"),
            Validator::Uint8 => {
                format!("!Number.isInteger({value_var}) || {value_var} < 0 || {value_var} > 255")
            }

            // Array validators
            Validator::MaxItems(n) => format!("{value_var}.length > {n}"),
            Validator::MinItems(n) => format!("{value_var}.length < {n}"),
            Validator::ItemsCount(n) => format!("{value_var}.length !== {n}"),

            // Date validators
            Validator::ValidDate => format!("isNaN({value_var}.getTime())"),
            Validator::GreaterThanDate(date) => {
                format!("{value_var}.getTime() <= new Date(\"{date}\").getTime()")
            }
            Validator::GreaterThanOrEqualToDate(date) => {
                format!("{value_var}.getTime() < new Date(\"{date}\").getTime()")
            }
            Validator::LessThanDate(date) => {
                format!("{value_var}.getTime() >= new Date(\"{date}\").getTime()")
            }
            Validator::LessThanOrEqualToDate(date) => {
                format!("{value_var}.getTime() > new Date(\"{date}\").getTime()")
            }
            Validator::BetweenDate(min, max) => format!(
                "{value_var}.getTime() < new Date(\"{min}\").getTime() || {value_var}.getTime() > new Date(\"{max}\").getTime()"
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

            // Custom validator
            Validator::Custom(fn_name) => format!("!{fn_name}({value_var})"),
        };

        let default_message =
            get_default_validator_message(&spec.validator, field_name, context_name);
        let message = spec.custom_message.as_ref().unwrap_or(&default_message);
        let escaped_message = message.replace('\\', "\\\\").replace('"', "\\\"");

        code.push_str(&format!(
            "                                if ({condition}) {{ errors.push({{ field: \"{field_name}\", message: \"{escaped_message}\" }}); }}\n"
        ));
    }

    TsStream::from_string(code)
}

/// Generate default error message for a validator
pub(super) fn get_default_validator_message(
    validator: &Validator,
    field_name: &str,
    context_name: &str,
) -> String {
    match validator {
        Validator::Email => format!("{context_name}.{field_name} must be a valid email"),
        Validator::Url => format!("{context_name}.{field_name} must be a valid URL"),
        Validator::Uuid => format!("{context_name}.{field_name} must be a valid UUID"),
        Validator::MaxLength(n) => {
            format!("{context_name}.{field_name} must have at most {n} characters")
        }
        Validator::MinLength(n) => {
            format!("{context_name}.{field_name} must have at least {n} characters")
        }
        Validator::Length(n) => {
            format!("{context_name}.{field_name} must have exactly {n} characters")
        }
        Validator::LengthRange(min, max) => {
            format!("{context_name}.{field_name} must have between {min} and {max} characters")
        }
        Validator::Pattern(pattern) => {
            format!("{context_name}.{field_name} must match pattern {pattern}")
        }
        Validator::NonEmpty => format!("{context_name}.{field_name} must not be empty"),
        Validator::Trimmed => format!("{context_name}.{field_name} must be trimmed"),
        Validator::Lowercase => format!("{context_name}.{field_name} must be lowercase"),
        Validator::Uppercase => format!("{context_name}.{field_name} must be uppercase"),
        Validator::Capitalized => format!("{context_name}.{field_name} must be capitalized"),
        Validator::Uncapitalized => format!("{context_name}.{field_name} must be uncapitalized"),
        Validator::StartsWith(prefix) => {
            format!("{context_name}.{field_name} must start with '{prefix}'")
        }
        Validator::EndsWith(suffix) => {
            format!("{context_name}.{field_name} must end with '{suffix}'")
        }
        Validator::Includes(text) => format!("{context_name}.{field_name} must include '{text}'"),
        Validator::GreaterThan(n) => {
            format!("{context_name}.{field_name} must be greater than {n}")
        }
        Validator::GreaterThanOrEqualTo(n) => {
            format!("{context_name}.{field_name} must be greater than or equal to {n}")
        }
        Validator::LessThan(n) => format!("{context_name}.{field_name} must be less than {n}"),
        Validator::LessThanOrEqualTo(n) => {
            format!("{context_name}.{field_name} must be less than or equal to {n}")
        }
        Validator::Between(min, max) => {
            format!("{context_name}.{field_name} must be between {min} and {max}")
        }
        Validator::Int => format!("{context_name}.{field_name} must be an integer"),
        Validator::NonNaN => format!("{context_name}.{field_name} must not be NaN"),
        Validator::Finite => format!("{context_name}.{field_name} must be finite"),
        Validator::Positive => format!("{context_name}.{field_name} must be positive"),
        Validator::NonNegative => format!("{context_name}.{field_name} must be non-negative"),
        Validator::Negative => format!("{context_name}.{field_name} must be negative"),
        Validator::NonPositive => format!("{context_name}.{field_name} must be non-positive"),
        Validator::MultipleOf(n) => {
            format!("{context_name}.{field_name} must be a multiple of {n}")
        }
        Validator::Uint8 => format!("{context_name}.{field_name} must be a uint8"),
        Validator::MaxItems(n) => {
            format!("{context_name}.{field_name} must have at most {n} items")
        }
        Validator::MinItems(n) => {
            format!("{context_name}.{field_name} must have at least {n} items")
        }
        Validator::ItemsCount(n) => {
            format!("{context_name}.{field_name} must have exactly {n} items")
        }
        Validator::ValidDate => format!("{context_name}.{field_name} must be a valid date"),
        Validator::GreaterThanDate(date) => {
            format!("{context_name}.{field_name} must be after {date}")
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!("{context_name}.{field_name} must be on or after {date}")
        }
        Validator::LessThanDate(date) => {
            format!("{context_name}.{field_name} must be before {date}")
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!("{context_name}.{field_name} must be on or before {date}")
        }
        Validator::BetweenDate(min, max) => {
            format!("{context_name}.{field_name} must be between {min} and {max}")
        }
        Validator::GreaterThanBigInt(n) => {
            format!("{context_name}.{field_name} must be greater than {n}")
        }
        Validator::GreaterThanOrEqualToBigInt(n) => {
            format!("{context_name}.{field_name} must be greater than or equal to {n}")
        }
        Validator::LessThanBigInt(n) => {
            format!("{context_name}.{field_name} must be less than {n}")
        }
        Validator::LessThanOrEqualToBigInt(n) => {
            format!("{context_name}.{field_name} must be less than or equal to {n}")
        }
        Validator::BetweenBigInt(min, max) => {
            format!("{context_name}.{field_name} must be between {min} and {max}")
        }
        Validator::PositiveBigInt => format!("{context_name}.{field_name} must be positive"),
        Validator::NonNegativeBigInt => format!("{context_name}.{field_name} must be non-negative"),
        Validator::NegativeBigInt => format!("{context_name}.{field_name} must be negative"),
        Validator::NonPositiveBigInt => format!("{context_name}.{field_name} must be non-positive"),
        Validator::Custom(fn_name) => {
            format!("{context_name}.{field_name} failed custom validation ({fn_name})")
        }
    }
}
