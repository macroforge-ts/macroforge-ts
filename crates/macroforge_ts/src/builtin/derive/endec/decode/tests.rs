use super::types::DecodeField;
use super::validation::{Missing, generate_field_validations};

use crate::ts_syn::ts_ident;

use super::super::{TypeCategory, Validator, ValidatorSpec};

#[test]
fn test_decode_field_has_validators() {
    let field = DecodeField {
        json_key: "email".into(),
        field_name: "email".into(),
        field_ident: ts_ident!("email"),
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
        array_elem_kind: None,
        nullable_encodable_type: None,
        decode_with: None,
        decimal_format: false,
        array_elem_encodable_type: None,
        set_elem_kind: None,
        set_elem_encodable_type: None,
        map_value_kind: None,
        map_value_encodable_type: None,
        record_value_encodable_type: None,
        wrapper_encodable_type: None,
        union_guard: None,
        nullable_union_guard: None,
        array_elem_union_guard: None,
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
