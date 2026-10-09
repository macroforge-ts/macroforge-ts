//! Field options, type classification and type defaults.

use super::make_decorator;
use crate::builtin::derive::common::{
    CompareFieldOptions, DefaultFieldOptions, get_type_default, has_known_default, is_generic_type,
    is_numeric_type, is_primitive_type, parse_generic_type, tuple_element,
};

#[test]
fn test_compare_field_skip() {
    let decorator = make_decorator("partialEq", "skip");
    let opts = CompareFieldOptions::from_decorators(&[decorator], "partialEq");
    assert!(opts.skip);
}

#[test]
fn test_compare_field_no_skip() {
    let decorator = make_decorator("partialEq", "");
    let opts = CompareFieldOptions::from_decorators(&[decorator], "partialEq");
    assert!(!opts.skip);
}

#[test]
fn test_compare_field_skip_false() {
    let decorator = make_decorator("hash", "skip: false");
    let opts = CompareFieldOptions::from_decorators(&[decorator], "hash");
    assert!(!opts.skip);
}

#[test]
fn test_default_field_with_string_value() {
    let decorator = make_decorator("default", r#""hello""#);
    let opts = DefaultFieldOptions::from_decorators(&[decorator]);
    assert!(opts.has_default);
    assert_eq!(opts.value.as_deref(), Some(r#""hello""#));
}

#[test]
fn test_default_field_with_number_value() {
    let decorator = make_decorator("default", "42");
    let opts = DefaultFieldOptions::from_decorators(&[decorator]);
    assert!(opts.has_default);
    assert_eq!(opts.value.as_deref(), Some("42"));
}

#[test]
fn test_default_field_with_array_value() {
    let decorator = make_decorator("default", "[]");
    let opts = DefaultFieldOptions::from_decorators(&[decorator]);
    assert!(opts.has_default);
    assert_eq!(opts.value.as_deref(), Some("[]"));
}

#[test]
fn test_default_field_with_named_value() {
    let decorator = make_decorator("default", r#"{ value: "test" }"#);
    let opts = DefaultFieldOptions::from_decorators(&[decorator]);
    assert!(opts.has_default);
    assert_eq!(opts.value.as_deref(), Some("test"));
}

#[test]
fn test_is_primitive_type() {
    assert!(is_primitive_type("string"));
    assert!(is_primitive_type("number"));
    assert!(is_primitive_type("boolean"));
    assert!(is_primitive_type("bigint"));
    assert!(!is_primitive_type("Date"));
    assert!(!is_primitive_type("User"));
    assert!(!is_primitive_type("string[]"));
}

#[test]
fn test_is_numeric_type() {
    assert!(is_numeric_type("number"));
    assert!(is_numeric_type("bigint"));
    assert!(!is_numeric_type("string"));
    assert!(!is_numeric_type("boolean"));
}

#[test]
fn test_get_type_default() {
    assert_eq!(get_type_default("string"), r#""""#);
    assert_eq!(get_type_default("number"), "0");
    assert_eq!(get_type_default("boolean"), "false");
    assert_eq!(get_type_default("bigint"), "0n");
    assert_eq!(get_type_default("string[]"), "[]");
    assert_eq!(get_type_default("Array<number>"), "[]");
    assert_eq!(get_type_default("Map<string, number>"), "new Map()");
    assert_eq!(get_type_default("Set<string>"), "new Set()");
    assert_eq!(get_type_default("Date"), "new Date()");
    // Unknown types call their defaultValue() method (Prefix style)
    assert_eq!(get_type_default("User"), "userDefaultValue()");
    // Generic type instantiations without a TypeRegistry cannot be resolved,
    // so they fall back to `undefined`. With a registry, `RecordLink<T>`
    // expands to its body (`string | T`) and the default flows through the
    // primitive-or-encodable detector to `"place:holder"`. See
    // `get_type_default_with_registry` for the resolved path.
    assert_eq!(get_type_default("RecordLink<Service>"), "undefined");
    assert_eq!(get_type_default("Result<User, Error>"), "undefined");
    // Object literal types default to {}
    assert_eq!(get_type_default("{ [key: string]: number }"), "{}");
    assert_eq!(get_type_default("{ foo: string; bar: number }"), "{}");
    assert_eq!(get_type_default("{ [K in keyof T]: V }"), "{}");
}

#[test]
fn test_get_type_default_object_literal_before_union_split() {
    // Object types containing pipes should not be split as unions
    assert_eq!(get_type_default("{ a: string | number }"), "{}");
    assert_eq!(
        get_type_default("{ status: \"active\" | \"inactive\" }"),
        "{}"
    );
}

#[test]
fn test_get_type_default_union_with_primitive() {
    // A union with a primitive member defaults to that primitive's default.
    assert_eq!(get_type_default("string | Account"), r#""""#);
    assert_eq!(get_type_default("string | Employee"), r#""""#);
    assert_eq!(get_type_default("string | Appointment"), r#""""#);
    assert_eq!(get_type_default("string | Site"), r#""""#);
    assert_eq!(get_type_default("number | Custom"), "0");
    assert_eq!(get_type_default("boolean | Foo"), "false");
    assert_eq!(get_type_default("bigint | Bar"), "0n");
    // Primitive need not be first in the union.
    assert_eq!(get_type_default("Account | string"), r#""""#);
}

#[test]
fn test_get_type_default_union_with_literal() {
    // Literal unions -> first literal value
    assert_eq!(
        get_type_default(r#""Estimate" | "Invoice""#),
        r#""Estimate""#
    );
    assert_eq!(
        get_type_default(r#""active" | "pending" | "completed""#),
        r#""active""#
    );
}

#[test]
fn test_get_type_default_union_custom_types() {
    // Union of only custom types -> default of the first member
    assert_eq!(
        get_type_default("Account | Employee"),
        "accountDefaultValue()"
    );
}

#[test]
fn test_get_type_default_nullable_union() {
    // Nullable unions are still handled by is_nullable_type
    assert_eq!(get_type_default("string | null"), "null");
    assert_eq!(get_type_default("Account | undefined"), "null");
    assert_eq!(get_type_default("string | Account | null"), "null");
}

#[test]
fn test_is_generic_type() {
    // Generic types
    assert!(is_generic_type("RecordLink<Service>"));
    assert!(is_generic_type("Map<string, number>"));
    assert!(is_generic_type("Array<User>"));
    assert!(is_generic_type("Result<T, E>"));

    // Non-generic types
    assert!(!is_generic_type("User"));
    assert!(!is_generic_type("string"));
    assert!(!is_generic_type("number[]")); // Array syntax, not generic
}

#[test]
fn test_parse_generic_type() {
    // Simple generic
    assert_eq!(
        parse_generic_type("RecordLink<Service>"),
        Some(("RecordLink", "Service"))
    );

    // Multiple type parameters
    assert_eq!(
        parse_generic_type("Map<string, number>"),
        Some(("Map", "string, number"))
    );

    // Nested generics
    assert_eq!(
        parse_generic_type("Result<Array<User>, Error>"),
        Some(("Result", "Array<User>, Error"))
    );

    // Non-generic types return None
    assert_eq!(parse_generic_type("User"), None);
    assert_eq!(parse_generic_type("string"), None);

    // Malformed (no closing bracket)
    assert_eq!(parse_generic_type("Array<User"), None);
}

#[test]
fn test_tuple_element_strips_labels_optional_and_rest() {
    let plain = tuple_element(" number ");
    assert_eq!((plain.ts_type, plain.rest), ("number", false));
    let labeled = tuple_element("name: string");
    assert_eq!((labeled.ts_type, labeled.rest), ("string", false));
    let optional = tuple_element("count?: number");
    assert_eq!((optional.ts_type, optional.rest), ("number", false));
    assert!(optional.optional);
    assert!(!plain.optional);
    let bare_optional = tuple_element("boolean?");
    assert!(bare_optional.optional);
    assert_eq!(
        (bare_optional.ts_type, bare_optional.rest),
        ("boolean", false)
    );
    let rest = tuple_element("...tags: string[]");
    assert_eq!((rest.ts_type, rest.rest), ("string[]", true));
    let object = tuple_element("{ a: number }");
    assert_eq!((object.ts_type, object.rest), ("{ a: number }", false));
}

#[test]
fn function_and_constructor_types_have_no_known_default() {
    for ts_type in [
        "() => string",
        "((a: number) => void)",
        "new () => Date",
        "<T>(value: T) => T",
    ] {
        assert!(!has_known_default(ts_type), "{ts_type}");
    }
    for ts_type in [
        "string",
        "(() => void) | null",
        "Callback",
        "Function",
        "Array<() => void>",
    ] {
        assert!(has_known_default(ts_type), "{ts_type}");
    }
}
