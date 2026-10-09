//! How the value traits classify a field's type.

use crate::builtin::derive::common::{Builtin, ValueType, classify_value_type, field_value_type};

fn kind(ts_type: &str) -> ValueType<'_> {
    classify_value_type(ts_type, None).kind
}

#[test]
fn literal_types_compare_as_their_primitive() {
    assert_eq!(kind("'draft'"), ValueType::Primitive("string"));
    assert_eq!(kind("42"), ValueType::Primitive("number"));
    assert_eq!(kind("-1.5"), ValueType::Primitive("number"));
    assert_eq!(kind("10n"), ValueType::Primitive("bigint"));
    assert_eq!(kind("true"), ValueType::Primitive("boolean"));
}

#[test]
fn nullable_members_split_off_a_single_type() {
    let date = classify_value_type("Date | null", None);
    assert!(date.nullable);
    assert_eq!(date.kind, ValueType::Builtin(Builtin::Date));
    let optional = field_value_type("Date", true);
    assert!(classify_value_type(&optional, None).nullable);
}

#[test]
fn unions_of_several_types_stay_structural() {
    let union = classify_value_type("string | number[] | null", None);
    assert!(!union.nullable);
    assert_eq!(union.kind, ValueType::Structural);
    assert_eq!(kind("{ id: string }"), ValueType::Structural);
    assert_eq!(kind("[string, number]"), ValueType::Structural);
    assert_eq!(kind("A & B"), ValueType::Structural);
    assert_eq!(kind("unknown"), ValueType::Structural);
}

#[test]
fn collections_carry_their_element_types() {
    assert_eq!(
        kind("Map<string, Array<Date>>"),
        ValueType::Map {
            key: "string",
            value: "Array<Date>"
        }
    );
    assert_eq!(
        kind("Map<Record<string, number>, Date>"),
        ValueType::Map {
            key: "Record<string, number>",
            value: "Date"
        }
    );
    assert_eq!(kind("ReadonlySet<Tag>"), ValueType::Set("Tag"));
    assert_eq!(kind("readonly Date[]"), ValueType::Array("Date"));
}

#[test]
fn functions_and_symbols_compare_by_identity() {
    assert_eq!(kind("() => void"), ValueType::Opaque);
    assert_eq!(kind("(value: string) => number"), ValueType::Opaque);
    assert_eq!(kind("symbol"), ValueType::Opaque);
}

#[test]
fn named_types_drop_their_arguments() {
    assert_eq!(kind("Page<User>"), ValueType::Named("Page"));
    assert_eq!(kind("User"), ValueType::Named("User"));
    assert_eq!(
        kind("Float64Array"),
        ValueType::Builtin(Builtin::TypedArray)
    );
}
