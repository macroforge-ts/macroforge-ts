//! How a field's type is classified for encoding and decoding.

use crate::builtin::derive::endec::{TypeCategory, WrapperFields};
use crate::ts_syn::abi::ir::split_top_level_union;

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
fn test_type_category_reads_every_array_spelling_alike() {
    for spelling in [
        "(User | null)[]",
        "Array<User | null>",
        "ReadonlyArray<User | null>",
        "readonly (User | null)[]",
    ] {
        assert_eq!(
            TypeCategory::from_ts_type(spelling),
            TypeCategory::Array("User | null".into()),
            "{spelling}"
        );
    }
    // A union that merely ends with an array member is not an array.
    assert_eq!(
        TypeCategory::from_ts_type("string | number[]"),
        TypeCategory::Unknown
    );
}

#[test]
fn test_type_category_readonly_map_and_set() {
    assert_eq!(
        TypeCategory::from_ts_type("ReadonlyMap<string, number>"),
        TypeCategory::Map("string".into(), "number".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("ReadonlySet<string>"),
        TypeCategory::Set("string".into())
    );
}

#[test]
fn test_wrapper_fields_tell_which_fields_a_value_has() {
    assert_eq!(WrapperFields::of("User"), None);
    assert_eq!(WrapperFields::of("Array<User>"), None);
    for whole in ["Required<User>", "Readonly<User>", "NonNullable<User>"] {
        assert_eq!(WrapperFields::of(whole), Some(Ok(WrapperFields::Whole)));
    }
    assert_eq!(
        WrapperFields::of("Partial<User>"),
        Some(Ok(WrapperFields::Any))
    );
    assert_eq!(
        WrapperFields::of("Pick<User, 'id' | \"name\">"),
        Some(Ok(WrapperFields::Only(vec!["id".into(), "name".into()])))
    );
    assert_eq!(
        WrapperFields::of("Omit<Box<string, number>, 'held'>"),
        Some(Ok(WrapperFields::Except(vec!["held".into()])))
    );
    // Keys behind a name say nothing here about which fields remain.
    assert!(matches!(
        WrapperFields::of("Pick<User, UserKeys>"),
        Some(Err(message)) if message.contains("string literals")
    ));
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
fn test_type_category_encodable() {
    assert_eq!(
        TypeCategory::from_ts_type("User"),
        TypeCategory::Encodable("User".into())
    );
}

#[test]
fn test_type_category_encodable_strips_generics() {
    // Generic type parameters must be stripped from the Encodable variant
    // so they don't leak into generated function names.
    assert_eq!(
        TypeCategory::from_ts_type("RecordLink<Employee>"),
        TypeCategory::Encodable("RecordLink".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("RecordLink<Account>"),
        TypeCategory::Encodable("RecordLink".into())
    );
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
    // Regular type names should still be Encodable
    assert_eq!(
        TypeCategory::from_ts_type("Zoned"),
        TypeCategory::Encodable("Zoned".into())
    );
    assert_eq!(
        TypeCategory::from_ts_type("User"),
        TypeCategory::Encodable("User".into())
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
fn test_type_category_non_encodable_utility_types() {
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
