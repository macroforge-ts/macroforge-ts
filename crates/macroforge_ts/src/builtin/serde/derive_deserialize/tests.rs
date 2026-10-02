use super::types::DeserializeField;
use super::validation::generate_field_validations;

use crate::ts_syn::ts_ident;

use super::super::{TypeCategory, Validator, ValidatorSpec};

#[test]
fn test_deserialize_field_has_validators() {
    let field = DeserializeField {
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
        nullable_serializable_type: None,
        deserialize_with: None,
        decimal_format: false,
        array_elem_serializable_type: None,
        set_elem_kind: None,
        set_elem_serializable_type: None,
        map_value_kind: None,
        map_value_serializable_type: None,
        record_value_serializable_type: None,
        wrapper_serializable_type: None,
        primitive_union_guard: None,
        array_elem_primitive_union_guard: None,
        union_string_validators: vec![],
    };
    assert!(field.has_validators());

    let field_no_validators = DeserializeField {
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
        generate_field_validations(&[spec], "n", "count", "Counter", true)
            .source()
            .to_string()
    };

    assert!(checks(Validator::Email).contains("test(n)"));
    assert!(checks(Validator::MaxLength(255)).contains("if (n.length > 255)"));
    // nonNegativeInt must enforce both integrality and non-negativity
    assert!(checks(Validator::NonNegativeInt).contains("if (!Number.isInteger(n) || n < 0)"));
}
