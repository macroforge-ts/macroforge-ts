//! Parsing validators out of `@endec(...)`.

use super::{make_decorator, span};
use crate::builtin::derive::endec::{EndecFieldOptions, Validator, extract_validators};
use crate::ts_syn::abi::DiagnosticCollector;

// ========================================================================
// Validator parsing tests
// ========================================================================

#[test]
fn test_parse_simple_validators() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    assert!(matches!(
        parse_validator_string("email"),
        Ok(Validator::Email)
    ));
    assert!(matches!(parse_validator_string("url"), Ok(Validator::Url)));
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
    assert!(matches!(parse_validator_string("int"), Ok(Validator::Int)));
    assert!(matches!(
        parse_validator_string("nonNegativeInt"),
        Ok(Validator::NonNegativeInt)
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    assert!(matches!(
        parse_validator_string("custom(myValidator)"),
        Ok(Validator::Custom(custom)) if custom.function == "myValidator" && custom.source.is_none()
    ));
}

#[test]
fn test_parse_custom_validator_with_a_source() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;

    let Ok(Validator::Custom(custom)) =
        parse_validator_string(r#"custom({ function: "isEven", source: "./validators" })"#)
    else {
        panic!("the object form parses");
    };
    assert_eq!(custom.function, "isEven");
    assert_eq!(custom.source.as_deref(), Some("./validators"));
    assert_eq!(custom.callee(), "__mf_isEven__validators");

    assert!(
        parse_validator_string(r#"custom({ source: "./validators" })"#).is_err(),
        "the object form needs a function"
    );
    assert!(
        parse_validator_string(r#"custom({ function: "v.isEven", source: "./validators" })"#)
            .is_err(),
        "an imported function is named by its export"
    );
}

#[test]
fn test_extract_validators_from_args() {
    let mut diagnostics = DiagnosticCollector::new();
    let validators = extract_validators(
        r#"{ validate: ["email", "maxLength(255)"] }"#,
        span(),
        "field 'test_field'",
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
        "field 'test_field'",
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
        "field 'test_field'",
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
    let result = EndecFieldOptions::from_decorators(&[decorator], "test_field");
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
