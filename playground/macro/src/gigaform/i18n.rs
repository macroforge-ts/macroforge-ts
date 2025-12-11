//! i18n integration for Gigaform using Paraglide.

use crate::gigaform::parser::GigaformOptions;

/// Gets the error message for a validator, using i18n if configured.
pub fn get_validator_message(
    validator_name: &str,
    args: &[String],
    field_name: &str,
    options: &GigaformOptions,
) -> String {
    if let Some(prefix) = &options.i18n_prefix {
        // Generate i18n message key call
        let message_key = format!("{prefix}_{field_name}_{validator_name}");
        let params = get_validator_params(validator_name, args);
        if params.is_empty() {
            format!("m.{message_key}()")
        } else {
            format!("m.{message_key}({{ {params} }})")
        }
    } else {
        // Fallback to static English message
        get_static_validator_message(validator_name, args)
    }
}

/// Gets the error message for an async validator.
pub fn get_async_validator_message(
    fn_name: &str,
    field_name: &str,
    options: &GigaformOptions,
) -> String {
    if let Some(prefix) = &options.i18n_prefix {
        format!("m.{prefix}_{field_name}_{fn_name}()")
    } else {
        format!("Validation failed for {field_name}")
    }
}

/// Gets parameters to pass to i18n message function.
fn get_validator_params(validator_name: &str, args: &[String]) -> String {
    match validator_name {
        "minLength" | "minItems" | "greaterThan" | "greaterThanOrEqualTo" => {
            if let Some(n) = args.first() {
                format!("min: {n}")
            } else {
                String::new()
            }
        }
        "maxLength" | "maxItems" | "lessThan" | "lessThanOrEqualTo" => {
            if let Some(n) = args.first() {
                format!("max: {n}")
            } else {
                String::new()
            }
        }
        "length" => {
            if args.len() == 1 {
                format!("length: {}", args[0])
            } else if args.len() >= 2 {
                format!("min: {}, max: {}", args[0], args[1])
            } else {
                String::new()
            }
        }
        "between" => {
            if args.len() >= 2 {
                format!("min: {}, max: {}", args[0], args[1])
            } else {
                String::new()
            }
        }
        "pattern" => {
            if let Some(p) = args.first() {
                format!("pattern: \"{}\"", p.replace('\\', "\\\\").replace('"', "\\\""))
            } else {
                String::new()
            }
        }
        "startsWith" => {
            if let Some(p) = args.first() {
                format!("prefix: \"{p}\"")
            } else {
                String::new()
            }
        }
        "endsWith" => {
            if let Some(s) = args.first() {
                format!("suffix: \"{s}\"")
            } else {
                String::new()
            }
        }
        "includes" => {
            if let Some(s) = args.first() {
                format!("substring: \"{s}\"")
            } else {
                String::new()
            }
        }
        "multipleOf" => {
            if let Some(n) = args.first() {
                format!("step: {n}")
            } else {
                String::new()
            }
        }
        "greaterThanDate" | "lessThanDate" => {
            if let Some(d) = args.first() {
                format!("date: \"{d}\"")
            } else {
                String::new()
            }
        }
        "betweenDate" => {
            if args.len() >= 2 {
                format!("minDate: \"{}\", maxDate: \"{}\"", args[0], args[1])
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

/// Gets a static (non-i18n) error message for a validator.
fn get_static_validator_message(validator_name: &str, args: &[String]) -> String {
    match validator_name {
        // String validators
        "email" => "Must be a valid email address".to_string(),
        "url" => "Must be a valid URL".to_string(),
        "uuid" => "Must be a valid UUID".to_string(),
        "nonEmpty" => "Must not be empty".to_string(),
        "trimmed" => "Must not have leading or trailing whitespace".to_string(),
        "lowercase" => "Must be lowercase".to_string(),
        "uppercase" => "Must be uppercase".to_string(),
        "capitalized" => "Must start with an uppercase letter".to_string(),
        "uncapitalized" => "Must start with a lowercase letter".to_string(),
        "minLength" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must have at least {n} characters")
        }
        "maxLength" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must have at most {n} characters")
        }
        "length" => {
            if args.len() == 1 {
                format!("Must have exactly {} characters", args[0])
            } else if args.len() >= 2 {
                format!("Must have between {} and {} characters", args[0], args[1])
            } else {
                "Invalid length".to_string()
            }
        }
        "pattern" => {
            let p = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must match pattern {p}")
        }
        "startsWith" => {
            let p = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must start with '{p}'")
        }
        "endsWith" => {
            let s = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must end with '{s}'")
        }
        "includes" => {
            let s = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must contain '{s}'")
        }

        // Number validators
        "positive" => "Must be positive".to_string(),
        "negative" => "Must be negative".to_string(),
        "nonNegative" => "Must be zero or positive".to_string(),
        "nonPositive" => "Must be zero or negative".to_string(),
        "int" => "Must be an integer".to_string(),
        "finite" => "Must be a finite number".to_string(),
        "nonNaN" => "Must be a valid number".to_string(),
        "uint8" => "Must be between 0 and 255".to_string(),
        "greaterThan" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must be greater than {n}")
        }
        "greaterThanOrEqualTo" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must be at least {n}")
        }
        "lessThan" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must be less than {n}")
        }
        "lessThanOrEqualTo" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must be at most {n}")
        }
        "between" => {
            if args.len() >= 2 {
                format!("Must be between {} and {}", args[0], args[1])
            } else {
                "Must be within range".to_string()
            }
        }
        "multipleOf" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("1");
            format!("Must be a multiple of {n}")
        }

        // Array validators
        "minItems" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must have at least {n} items")
        }
        "maxItems" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must have at most {n} items")
        }
        "itemsCount" => {
            let n = args.first().map(|s| s.as_str()).unwrap_or("0");
            format!("Must have exactly {n} items")
        }

        // Date validators
        "validDate" => "Must be a valid date".to_string(),
        "greaterThanDate" => {
            let d = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must be after {d}")
        }
        "greaterThanOrEqualToDate" => {
            let d = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must be on or after {d}")
        }
        "lessThanDate" => {
            let d = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must be before {d}")
        }
        "lessThanOrEqualToDate" => {
            let d = args.first().map(|s| s.as_str()).unwrap_or("");
            format!("Must be on or before {d}")
        }
        "betweenDate" => {
            if args.len() >= 2 {
                format!("Must be between {} and {}", args[0], args[1])
            } else {
                "Must be within date range".to_string()
            }
        }

        // Custom
        "custom" => "Validation failed".to_string(),

        // Unknown
        _ => format!("Invalid value ({validator_name})")
    }
}
