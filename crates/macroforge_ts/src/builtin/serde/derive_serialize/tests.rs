use super::*;

#[test]
fn test_serialize_field_struct() {
    let field = SerializeField {
        json_key: "name".into(),
        json_key_ident: ts_ident!("name"),
        field_name: "name".into(),
        field_ident: ts_ident!("name"),
        type_cat: TypeCategory::Primitive,
        optional: false,
        flatten: false,
        optional_inner_kind: None,
        nullable_inner_kind: None,
        array_elem_kind: None,
        set_elem_kind: None,
        map_value_kind: None,
        record_value_kind: None,
        wrapper_inner_kind: None,
        optional_serializable_type: None,
        nullable_serializable_type: None,
        array_elem_serializable_type: None,
        set_elem_serializable_type: None,
        map_value_serializable_type: None,
        record_value_serializable_type: None,
        wrapper_serializable_type: None,
        serialize_with: None,
        decimal_format: false,
    };
    assert_eq!(field.json_key, "name");
    assert!(!field.optional);
}
