use super::*;
use crate::ts_syn::abi::{DiagnosticCollector, SpanIR};

fn span() -> SpanIR {
    SpanIR::new(0, 0)
}

fn make_decorator(args: &str) -> crate::ts_syn::abi::DecoratorIR {
    crate::ts_syn::abi::DecoratorIR {
        name: "serde".into(),
        args_src: args.into(),
        span: span(),
        node: None,
    }
}

#[test]
fn test_field_skip() {
    let decorator = make_decorator("skip");
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.skip);
    assert!(!opts.should_serialize());
    assert!(!opts.should_deserialize());
}

#[test]
fn test_field_skip_serializing() {
    let decorator = make_decorator("skipSerializing");
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.skip_serializing);
    assert!(!opts.should_serialize());
    assert!(opts.should_deserialize());
}

#[test]
fn test_field_rename() {
    let decorator = make_decorator(r#"{ rename: "user_id" }"#);
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.rename.as_deref(), Some("user_id"));
}

#[test]
fn test_field_default_flag() {
    let decorator = make_decorator("default");
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.default);
    assert!(opts.default_expr.is_none());
}

#[test]
fn test_field_default_expr() {
    let decorator = make_decorator(r#"{ default: "new Date()" }"#);
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.default);
    assert_eq!(opts.default_expr.as_deref(), Some("new Date()"));
}

#[test]
fn test_field_flatten() {
    let decorator = make_decorator("flatten");
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.flatten);
}

#[test]
fn test_container_rename_all() {
    let decorator = make_decorator(r#"{ renameAll: "camelCase" }"#);
    let opts = SerdeContainerOptions::from_decorators(&[decorator]);
    assert_eq!(opts.rename_all, RenameAll::CamelCase);
}

#[test]
fn test_container_deny_unknown_fields() {
    let decorator = make_decorator("denyUnknownFields");
    let opts = SerdeContainerOptions::from_decorators(&[decorator]);
    assert!(opts.deny_unknown_fields);
}

#[test]
fn test_container_tag_internally_tagged() {
    let decorator = make_decorator(r#"{ tag: "type" }"#);
    let opts = SerdeContainerOptions::from_decorators(&[decorator]);
    assert_eq!(
        opts.tagging,
        TaggingMode::InternallyTagged {
            tag: "type".to_string()
        }
    );
    assert_eq!(opts.tag_field(), Some("type"));
    assert_eq!(opts.tag_field_or_default(), "type");
    assert_eq!(opts.content_field(), None);
}

#[test]
fn test_container_tag_default() {
    let opts = SerdeContainerOptions::default();
    assert_eq!(
        opts.tagging,
        TaggingMode::InternallyTagged {
            tag: "__type".to_string()
        }
    );
    assert_eq!(opts.tag_field(), Some("__type"));
    assert_eq!(opts.tag_field_or_default(), "__type");
}

#[test]
fn test_container_externally_tagged() {
    let decorator = make_decorator(r#"{ externallyTagged: true }"#);
    let opts = SerdeContainerOptions::from_decorators(&[decorator]);
    assert_eq!(opts.tagging, TaggingMode::ExternallyTagged);
    assert_eq!(opts.tag_field(), None);
    assert_eq!(opts.tag_field_or_default(), "__type");
}

#[test]
fn test_container_adjacently_tagged() {
    let decorator = make_decorator(r#"{ tag: "t", content: "c" }"#);
    let opts = SerdeContainerOptions::from_decorators(&[decorator]);
    assert_eq!(
        opts.tagging,
        TaggingMode::AdjacentlyTagged {
            tag: "t".to_string(),
            content: "c".to_string()
        }
    );
    assert_eq!(opts.tag_field(), Some("t"));
    assert_eq!(opts.tag_field_or_default(), "t");
    assert_eq!(opts.content_field(), Some("c"));
}

#[test]
fn test_container_untagged() {
    let decorator = make_decorator(r#"{ untagged: true }"#);
    let opts = SerdeContainerOptions::from_decorators(&[decorator]);
    assert_eq!(opts.tagging, TaggingMode::Untagged);
    assert_eq!(opts.tag_field(), None);
    assert_eq!(opts.tag_field_or_default(), "__type");
    assert_eq!(opts.content_field(), None);
}

#[test]
fn test_type_category_primitives() {
    assert_eq!(
        TypeCategory::from_ts_type("string"),
        TypeCategory::Primitive
    );
    assert_eq!(
        TypeCategory::from_ts_type("number"),
        TypeCategory::Primitive
    );
    assert_eq!(
        TypeCategory::from_ts_type("boolean"),
        TypeCategory::Primitive
    );
}

#[test]
fn test_type_category_date() {
    assert_eq!(TypeCategory::from_ts_type("Date"), TypeCategory::Date);
}

#[test]
fn test_type_category_array() {
    assert_eq!(
        TypeCategory::from_ts_type("string[]"),
        TypeCategory::Array("string".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("Array<number>"),
        TypeCategory::Array("number".into())
    );
}

#[test]
fn test_type_category_map() {
    assert_eq!(
        TypeCategory::from_ts_type("Map<string, number>"),
        TypeCategory::Map("string".into(), "number".into())
    );
}

#[test]
fn test_type_category_set() {
    assert_eq!(
        TypeCategory::from_ts_type("Set<string>"),
        TypeCategory::Set("string".into())
    );
}

#[test]
fn test_type_category_optional() {
    assert_eq!(
        TypeCategory::from_ts_type("string | undefined"),
        TypeCategory::Optional("string".into())
    );
}

#[test]
fn test_type_category_nullable() {
    assert_eq!(
        TypeCategory::from_ts_type("string | null"),
        TypeCategory::Nullable("string".into())
    );
}

#[test]
fn test_type_category_serializable() {
    assert_eq!(
        TypeCategory::from_ts_type("User"),
        TypeCategory::Serializable("User".into())
    );
}

#[test]
fn test_type_category_serializable_strips_generics() {
    // Generic type parameters must be stripped from the Serializable variant
    // so they don't leak into generated function names.
    assert_eq!(
        TypeCategory::from_ts_type("RecordLink<Employee>"),
        TypeCategory::Serializable("RecordLink".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("RecordLink<Account>"),
        TypeCategory::Serializable("RecordLink".into())
    );
}

#[test]
fn test_convert_case_with_angle_brackets() {
    use convert_case::{Case, Casing};
    // Generics must be stripped before camelCase conversion since `<>` chars
    // are not recognized as word boundaries by convert_case.
    let base = "RecordLink";
    let fn_name = format!("{}SerializeWithContext", base.to_case(Case::Camel));
    assert_eq!(fn_name, "recordLinkSerializeWithContext");
}

#[test]
fn test_rename_all_camel_case() {
    assert_eq!(RenameAll::CamelCase.apply("user_name"), "userName");
    assert_eq!(RenameAll::CamelCase.apply("created_at"), "createdAt");
}

#[test]
fn test_rename_all_snake_case() {
    assert_eq!(RenameAll::SnakeCase.apply("userName"), "user_name");
    assert_eq!(RenameAll::SnakeCase.apply("createdAt"), "created_at");
}

#[test]
fn test_rename_all_pascal_case() {
    assert_eq!(RenameAll::PascalCase.apply("user_name"), "UserName");
}

#[test]
fn test_rename_all_kebab_case() {
    assert_eq!(RenameAll::KebabCase.apply("userName"), "user-name");
}

#[test]
fn test_rename_all_screaming_snake_case() {
    assert_eq!(RenameAll::ScreamingSnakeCase.apply("userName"), "USER_NAME");
}

// ========================================================================
// Validator parsing tests
// ========================================================================

#[test]
fn test_parse_simple_validators() {
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
    assert!(matches!(
        parse_validator_string("custom(myValidator)"),
        Ok(Validator::Custom(fn_name)) if fn_name == "myValidator"
    ));
}

#[test]
fn test_extract_validators_from_args() {
    let mut diagnostics = DiagnosticCollector::new();
    let validators = extract_validators(
        r#"{ validate: ["email", "maxLength(255)"] }"#,
        span(),
        "test_field",
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
        "test_field",
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
        "test_field",
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
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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
    use super::validators::parse_validator_string;
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

// ========================================================================
// Validator error tests
// ========================================================================

#[test]
fn test_unknown_validator_returns_error() {
    use super::validators::parse_validator_string;
    let result = parse_validator_string("unknownValidator");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("unknown validator"));
    assert!(err.message.contains("unknownValidator"));
}

#[test]
fn test_unknown_validator_with_typo_suggests_correction() {
    use super::validators::parse_validator_string;
    // "emai" is close to "email"
    let result = parse_validator_string("emai");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.help.is_some());
    assert!(err.help.as_ref().unwrap().contains("email"));
}

#[test]
fn test_unknown_validator_no_suggestion_for_unrelated() {
    use super::validators::parse_validator_string;
    // "xyz" has no similar validators
    let result = parse_validator_string("xyz");
    assert!(result.is_err());
    let err = result.unwrap_err();
    // For short strings with no matches, help may be None
    assert!(err.help.is_none() || !err.help.as_ref().unwrap().contains("email"));
}

#[test]
fn test_invalid_maxlength_args_returns_error() {
    use super::validators::parse_validator_string;
    let result = parse_validator_string("maxLength(abc)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("maxLength"));
}

#[test]
fn test_invalid_between_args_returns_error() {
    use super::validators::parse_validator_string;
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
        "test_field",
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
        "test_field",
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
    use super::validators::parse_validator_string;
    // "rul" is close to "url"
    let result = parse_validator_string("rul");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.help.is_some());
    assert!(err.help.as_ref().unwrap().contains("url"));
}

#[test]
fn test_typo_suggestion_maxlength() {
    use super::validators::parse_validator_string;
    // "maxLenth" is close to "maxLength"
    let result = parse_validator_string("maxLenth(10)");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.help.is_some());
    assert!(err.help.as_ref().unwrap().contains("maxLength"));
}

// ========================================================================
// Custom serializer/deserializer tests (serializeWith/deserializeWith)
// ========================================================================

#[test]
fn test_field_serialize_with() {
    let decorator = make_decorator(r#"{ serializeWith: "mySerializer" }"#);
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.serialize_with.as_deref(), Some("mySerializer"));
    assert!(opts.deserialize_with.is_none());
}

#[test]
fn test_field_deserialize_with() {
    let decorator = make_decorator(r#"{ deserializeWith: "myDeserializer" }"#);
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert!(opts.serialize_with.is_none());
    assert_eq!(opts.deserialize_with.as_deref(), Some("myDeserializer"));
}

#[test]
fn test_field_serialize_and_deserialize_with() {
    let decorator = make_decorator(r#"{ serializeWith: "toJson", deserializeWith: "fromJson" }"#);
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.serialize_with.as_deref(), Some("toJson"));
    assert_eq!(opts.deserialize_with.as_deref(), Some("fromJson"));
}

#[test]
fn test_field_serialize_with_combined_with_other_options() {
    let decorator = make_decorator(
        r#"{ serializeWith: "customSerialize", rename: "custom_field", skip: false }"#,
    );
    let result = SerdeFieldOptions::from_decorators(&[decorator], "test_field");
    let opts = result.options;
    assert_eq!(opts.serialize_with.as_deref(), Some("customSerialize"));
    assert_eq!(opts.rename.as_deref(), Some("custom_field"));
    assert!(!opts.skip);
}

// ========================================================================
// String literal type classification tests
// ========================================================================

#[test]
fn test_type_category_string_literal_double_quotes() {
    // String literal types like "Zoned" should be treated as primitives
    assert_eq!(
        TypeCategory::from_ts_type(r#""Zoned""#),
        TypeCategory::Primitive
    );
    assert_eq!(
        TypeCategory::from_ts_type(r#""some_value""#),
        TypeCategory::Primitive
    );
}

#[test]
fn test_type_category_string_literal_single_quotes() {
    // Single-quoted string literals should also be primitive
    assert_eq!(TypeCategory::from_ts_type("'foo'"), TypeCategory::Primitive);
    assert_eq!(
        TypeCategory::from_ts_type("'bar_baz'"),
        TypeCategory::Primitive
    );
}

#[test]
fn test_type_category_non_literal_type_names() {
    // Regular type names should still be Serializable
    assert_eq!(
        TypeCategory::from_ts_type("Zoned"),
        TypeCategory::Serializable("Zoned".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("User"),
        TypeCategory::Serializable("User".into())
    );
}

// ========================================================================
// TypeScript utility type classification tests
// ========================================================================

#[test]
fn test_type_category_record() {
    // Record<K, V> should be properly parsed as a Record variant
    assert_eq!(
        TypeCategory::from_ts_type("Record<string, unknown>"),
        TypeCategory::Record("string".into(), "unknown".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("Record<string, number>"),
        TypeCategory::Record("string".into(), "number".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("Record<string, User>"),
        TypeCategory::Record("string".into(), "User".into())
    );
}

#[test]
fn test_split_top_level_union_tracks_braces() {
    // Pipes inside braces should not split
    assert_eq!(split_top_level_union("{ a: string | number }"), None);
    assert_eq!(
        split_top_level_union("{ status: \"active\" | \"inactive\" }"),
        None
    );
    // Pipes outside braces should still split
    assert_eq!(
        split_top_level_union("{ a: string } | { b: number }"),
        Some(vec!["{ a: string }", "{ b: number }"])
    );
    // Mixed: pipe inside braces ignored, pipe outside splits
    assert_eq!(
        split_top_level_union("{ a: string | number } | null"),
        Some(vec!["{ a: string | number }", "null"])
    );
}

#[test]
fn test_type_category_wrapper_utility_types() {
    // Wrapper utility types that preserve structure should extract the inner type
    assert_eq!(
        TypeCategory::from_ts_type("Partial<User>"),
        TypeCategory::Wrapper("User".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("Required<Config>"),
        TypeCategory::Wrapper("Config".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("Readonly<Data>"),
        TypeCategory::Wrapper("Data".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("NonNullable<User>"),
        TypeCategory::Wrapper("User".into())
    );
    // Pick and Omit extract the first type argument
    assert_eq!(
        TypeCategory::from_ts_type("Pick<User, 'name' | 'email'>"),
        TypeCategory::Wrapper("User".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("Omit<User, 'password'>"),
        TypeCategory::Wrapper("User".into())
    );
}

#[test]
fn test_type_category_non_serializable_utility_types() {
    // Utility types operating on functions/unions/async should be Unknown
    assert_eq!(
        TypeCategory::from_ts_type("Promise<string>"),
        TypeCategory::Unknown
    );
    assert_eq!(
        TypeCategory::from_ts_type("ReturnType<typeof fn>"),
        TypeCategory::Unknown
    );
    assert_eq!(
        TypeCategory::from_ts_type("Awaited<Promise<User>>"),
        TypeCategory::Unknown
    );
}
