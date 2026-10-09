//! Errors and typo suggestions for validators that do not parse.

use super::span;
use crate::builtin::derive::endec::{Validator, extract_validators};
use crate::ts_syn::abi::DiagnosticCollector;

// ========================================================================
// Validator error tests
// ========================================================================

#[test]
fn test_unknown_validator_returns_error() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    let result = parse_validator_string("unknownValidator");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("unknown validator"));
    assert!(err.message.contains("unknownValidator"));
}

#[test]
fn test_unknown_validator_with_typo_suggests_correction() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    // "emai" is close to "email"
    let result = parse_validator_string("emai");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.help.is_some());
    assert!(err.help.as_ref().unwrap().contains("email"));
}

#[test]
fn test_unknown_validator_no_suggestion_for_unrelated() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    // "xyz" has no similar validators
    let result = parse_validator_string("xyz");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // For short strings with no matches, help may be None
    assert!(err.help.is_none() || !err.help.as_ref().unwrap().contains("email"));
}

#[test]
fn test_invalid_maxlength_args_returns_error() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    let result = parse_validator_string("maxLength(abc)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("maxLength"));
}

#[test]
fn test_invalid_between_args_returns_error() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
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
        "field 'test_field'",
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
        "field 'test_field'",
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
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    // "rul" is close to "url"
    let result = parse_validator_string("rul");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.help.is_some());
    assert!(err.help.as_ref().unwrap().contains("url"));
}

#[test]
fn test_typo_suggestion_maxlength() {
    use crate::builtin::derive::endec::validators::parse::parse_validator_string;
    // "maxLenth" is close to "maxLength"
    let result = parse_validator_string("maxLenth(10)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.help.is_some());
    assert!(err.help.as_ref().unwrap().contains("maxLength"));
}
