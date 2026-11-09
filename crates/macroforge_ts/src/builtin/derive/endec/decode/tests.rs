use super::types::DecodeField;
use super::validation::{Missing, generate_field_validations};

use crate::builtin::derive::common::Member;

use super::super::{TypeCategory, Validator, ValidatorSpec};

#[test]
fn test_decode_field_has_validators() {
    let field = DecodeField {
        json_key: "email".into(),
        field_name: "email".into(),
        member: Member::of("email"),
        raw_cast_type: "string".into(),
        ts_type: "string".into(),
        type_cat: TypeCategory::Primitive,
        optional: false,
        default_expr: None,
        flatten: false,
        validators: vec![ValidatorSpec {
            validator: Validator::Email,
            custom_message: None,
        }],
        nullable_inner_kind: None,
        nullable_encodable_type: None,
        nullable_nested: None,
        present_inner: None,
        decode_with: None,
        decimal_format: false,
        never_read: false,
        flattened_keys: None,
        composite: None,
        wrapped: None,
        by_parameter: None,
        union_guard: None,
        union_string_validators: vec![],
    };
    assert!(field.has_validators());

    let field_no_validators = DecodeField {
        validators: vec![],
        ..field
    };
    assert!(!field_no_validators.has_validators());
}

#[test]
fn test_field_validations_check_each_validator() {
    let checks = |validator: Validator| {
        let spec = ValidatorSpec {
            validator,
            custom_message: None,
        };
        generate_field_validations(&[spec], "n", "count", "Counter", Missing::Excluded)
            .source()
            .to_string()
    };

    assert!(checks(Validator::Email).contains("test(n)"));
    assert!(checks(Validator::MaxLength(255)).contains("if (n.length > 255)"));
    // nonNegativeInt must enforce both integrality and non-negativity
    assert!(checks(Validator::NonNegativeInt).contains("if (!Number.isInteger(n) || n < 0)"));
}

#[test]
fn test_field_validations_guard_a_missing_value_as_the_type_says() {
    let guarded = |missing: Missing| {
        let spec = ValidatorSpec {
            validator: Validator::NonEmpty,
            custom_message: None,
        };
        generate_field_validations(&[spec], "n", "name", "User", missing)
            .source()
            .to_string()
    };

    assert!(guarded(Missing::Allowed).starts_with("if (n != null) {"));
    assert!(guarded(Missing::Required).starts_with(
        "if (n == null) { errors.push({ field: \"name\", message: \"User.name is required\" }); } else {"
    ));
    let excluded = guarded(Missing::Excluded);
    assert!(!excluded.contains("null"), "{excluded}");
    assert!(excluded.starts_with("if (n.length === 0)"), "{excluded}");
}
