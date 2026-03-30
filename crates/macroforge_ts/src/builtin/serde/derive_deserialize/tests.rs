use super::field_processing::generate_object_variant_deser_block;
use super::types::*;
use super::validation::*;

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
        has_default: false,
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
        record_value_kind: None,
        record_value_serializable_type: None,
        wrapper_inner_kind: None,
        wrapper_serializable_type: None,
        optional_inner_kind: None,
        optional_serializable_type: None,
    };
    assert!(field.has_validators());

    let field_no_validators = DeserializeField {
        validators: vec![],
        ..field
    };
    assert!(!field_no_validators.has_validators());
}

#[test]
fn test_validation_condition_generation() {
    let condition = generate_validation_condition(&Validator::Email, "value");
    assert!(condition.contains("test(value)"));

    let condition = generate_validation_condition(&Validator::MaxLength(255), "str");
    assert_eq!(condition, "str.length > 255");
}

#[test]
fn test_object_variant_tag_extraction() {
    // Simulate what happens during union collection: extract tag value from
    // an inline object's field whose ts_type is a string literal.
    let tag_field = "__type";

    // Double-quoted literal
    let ts_type = "\"admin\"";
    let trimmed = ts_type.trim();
    assert!(trimmed.starts_with('"') && trimmed.ends_with('"'));
    let tag_value = &trimmed[1..trimmed.len() - 1];
    assert_eq!(tag_value, "admin");

    // Single-quoted literal
    let ts_type2 = "'viewer'";
    let trimmed2 = ts_type2.trim();
    assert!(trimmed2.starts_with('\'') && trimmed2.ends_with('\''));
    let tag_value2 = &trimmed2[1..trimmed2.len() - 1];
    assert_eq!(tag_value2, "viewer");

    // Non-literal type should not match
    let ts_type3 = "string";
    let trimmed3 = ts_type3.trim();
    assert!(
        !(trimmed3.starts_with('"') && trimmed3.ends_with('"')
            || trimmed3.starts_with('\'') && trimmed3.ends_with('\''))
    );

    let _ = tag_field;
}

#[test]
fn test_generate_object_variant_deser_block() {
    // Build an ObjectVariant with two fields and verify the generated code
    let variant = ObjectVariant {
        tag_value: Some("admin".to_string()),
        fields: vec![
            DeserializeField {
                json_key: "permissions".into(),
                field_name: "permissions".into(),
                field_ident: ts_ident!("permissions"),
                raw_cast_type: "string[]".into(),
                ts_type: "string[]".into(),
                type_cat: TypeCategory::Array("string".into()),
                optional: false,
                has_default: false,
                default_expr: None,
                flatten: false,
                validators: vec![],
                nullable_inner_kind: None,
                array_elem_kind: Some(SerdeValueKind::PrimitiveLike),
                nullable_serializable_type: None,
                deserialize_with: None,
                decimal_format: false,
                array_elem_serializable_type: None,
                set_elem_kind: None,
                set_elem_serializable_type: None,
                map_value_kind: None,
                map_value_serializable_type: None,
                record_value_kind: None,
                record_value_serializable_type: None,
                wrapper_inner_kind: None,
                wrapper_serializable_type: None,
                optional_inner_kind: None,
                optional_serializable_type: None,
            },
            DeserializeField {
                json_key: "level".into(),
                field_name: "level".into(),
                field_ident: ts_ident!("level"),
                raw_cast_type: "number".into(),
                ts_type: "number".into(),
                type_cat: TypeCategory::Primitive,
                optional: true,
                has_default: false,
                default_expr: None,
                flatten: false,
                validators: vec![],
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
                record_value_kind: None,
                record_value_serializable_type: None,
                wrapper_inner_kind: None,
                wrapper_serializable_type: None,
                optional_inner_kind: None,
                optional_serializable_type: None,
            },
        ],
        required_field_keys: vec!["permissions".into()],
        all_field_keys: vec!["permissions".into(), "level".into()],
        display_name: "{__type:\"admin\"}".into(),
    };

    let block = generate_object_variant_deser_block(&variant, "value", "__type", "UserRoles");
    let code = &block.source();

    // Should set the tag field
    assert!(
        code.contains("__inst[\"__type\"] = \"admin\""),
        "should set tag field"
    );
    // Should handle the required array field
    assert!(
        code.contains("permissions"),
        "should reference permissions field"
    );
    assert!(code.contains("string[]"), "should cast array to string[]");
    // Should handle the optional field with presence check
    assert!(
        code.contains("\"level\" in __obj"),
        "should check optional field"
    );
    // Should track for freeze and return
    assert!(
        code.contains("ctx.trackForFreeze"),
        "should track for freeze"
    );
    assert!(
        code.contains("return __inst as UserRoles"),
        "should return typed instance"
    );
}
