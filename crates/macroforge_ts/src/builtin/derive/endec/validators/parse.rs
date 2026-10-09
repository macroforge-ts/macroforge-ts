//! Parsing one validator: its name, arguments and custom message.

use super::{CustomValidator, Validator, ValidatorParseError, ValidatorSpec};
use crate::builtin::derive::common::parse_string_literal;
use crate::builtin::derive::endec::helpers::extract_named_value;

use super::extract::extract_bracket_content;

/// Parse object form: { validate: "email", message: "Invalid email" }
pub(super) fn parse_validator_object(input: &str) -> Result<ValidatorSpec, ValidatorParseError> {
    let content = extract_bracket_content(input, '{', '}')
        .ok_or_else(|| ValidatorParseError::invalid_args("object", "malformed validator object"))?;

    let validator_str = extract_named_value(&content, "validate")
        .ok_or_else(|| ValidatorParseError::invalid_args("object", "missing 'validate' field"))?;
    let validator = parse_validator_string(&validator_str)?;
    let custom_message = extract_named_value(&content, "message");

    Ok(ValidatorSpec {
        validator,
        custom_message,
    })
}

/// Parse a validator string like "email", "maxLength(255)", "custom(myValidator)"
pub(in crate::builtin::derive::endec) fn parse_validator_string(
    s: &str,
) -> Result<Validator, ValidatorParseError> {
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
        "nonnegativeint" => Ok(Validator::NonNegativeInt),
        _ => Err(ValidatorParseError::unknown_validator(trimmed)),
    }
}

/// Parse validators with arguments
pub(super) fn parse_validator_with_args(
    name: &str,
    args: &str,
) -> Result<Validator, ValidatorParseError> {
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
        "multipleof" => match args.trim().parse::<f64>() {
            Ok(divisor) if divisor.is_finite() && divisor != 0.0 => {
                Ok(Validator::MultipleOf(divisor))
            }
            Ok(_) => Err(ValidatorParseError::invalid_args(
                name,
                "the divisor must be a finite number other than zero",
            )),
            Err(_) => Err(ValidatorParseError::invalid_args(name, "expected a number")),
        },
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
        "custom" => parse_custom_validator(args).map(Validator::Custom),
        _ => Err(ValidatorParseError::unknown_validator(name)),
    }
}

/// `custom(myValidator)`, with the name quoted or not, or
/// `custom({ function: "isEven", source: "./validators" })`.
pub(super) fn parse_custom_validator(args: &str) -> Result<CustomValidator, ValidatorParseError> {
    let trimmed = args.trim();
    if !trimmed.starts_with('{') {
        let function = parse_validator_string_arg(trimmed).ok_or_else(|| {
            ValidatorParseError::invalid_args("custom", "expected a function name")
        })?;
        return Ok(CustomValidator {
            function,
            source: None,
        });
    }
    let function = extract_named_value(trimmed, "function").ok_or_else(|| {
        ValidatorParseError::invalid_args(
            "custom",
            "the object form needs `function`, the name of the validator to call",
        )
    })?;
    let source = extract_named_value(trimmed, "source");
    let is_identifier = function
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && function
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    if source.is_some() && !is_identifier {
        return Err(ValidatorParseError::invalid_args(
            "custom",
            "with `source`, `function` must be the plain name the module exports",
        ));
    }
    Ok(CustomValidator { function, source })
}

/// Parse a string argument (handles both quoted and unquoted)
pub(super) fn parse_validator_string_arg(input: &str) -> Option<String> {
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
