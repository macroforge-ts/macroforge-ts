//! Field options, type classification and type defaults.

use super::make_decorator;
use crate::builtin::derive::common::{
    CompareFieldOptions, Declared, DefaultFieldOptions, get_type_default, has_known_default,
    is_generic_type, is_nullable_type, is_numeric_type, is_primitive_type, parse_generic_type,
    primitive_keyword_test, tuple_element, undefaultable,
};
use crate::ts_syn::abi::ir::TypeRegistry;

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
fn a_grouped_type_defaults_as_the_type_in_it() {
    assert_eq!(get_type_default("(number | null)"), "null");
    assert_eq!(get_type_default("((string | undefined))"), "undefined");
    assert_eq!(get_type_default("((number | null) | null)"), "null");
    assert_eq!(get_type_default("(number)"), "0");
    assert_eq!(get_type_default("(string | number)[]"), "[]");
    assert!(is_nullable_type("(number | null)"));
    assert!(is_nullable_type("(string | null) | number"));
    assert!(!is_nullable_type("(number | null)[]"));
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
    assert_eq!(get_type_default("ReadonlyMap<string, number>"), "new Map()");
    assert_eq!(get_type_default("ReadonlySet<string>"), "new Set()");
    assert_eq!(get_type_default("ReadonlyArray<string>"), "[]");
    assert_eq!(get_type_default("readonly string[]"), "[]");
    assert_eq!(get_type_default("Date"), "new Date()");
    // Unknown types call their defaultValue() method (Prefix style)
    assert_eq!(get_type_default("User"), "userDefaultValue()");
    // A generic type defaults through its own function, as any named type
    // does, given the same arguments. An alias the registry holds is written
    // out first: see `get_type_default_with_registry`.
    assert_eq!(
        get_type_default("RecordLink<Service>"),
        "recordLinkDefaultValue<Service>()"
    );
    assert_eq!(
        get_type_default("Result<User, Error>"),
        "resultDefaultValue<User, Error>()"
    );
    // A promise is settled on its value's default.
    assert_eq!(
        get_type_default("Promise<string>"),
        r#"Promise.resolve("")"#
    );
    assert_eq!(
        get_type_default("Promise<void>"),
        "Promise.resolve(undefined)"
    );
    // A boxed primitive takes the primitive's.
    assert_eq!(get_type_default("String"), r#""""#);
    assert_eq!(get_type_default("Object"), "({})");
    // An object type written in place defaults to the members it has to
    // have, and one that names none to an empty object.
    assert_eq!(get_type_default("{ [key: string]: number }"), "({})");
    assert_eq!(
        get_type_default("{ foo: string; bar: number; later?: Date }"),
        r#"({ foo: "", bar: 0 })"#
    );
    assert_eq!(get_type_default("{ [K in keyof T]: V }"), "({})");
    // A `Required` or a `NonNullable` defaults as what it wraps.
    assert_eq!(get_type_default("Required<User>"), "userDefaultValue()");
    assert_eq!(get_type_default("NonNullable<string | null>"), r#""""#);
    // A selection of a type the registry does not hold names no field.
    assert_eq!(get_type_default("Pick<User, 'id'>"), "({})");
}

#[test]
fn a_type_no_default_can_be_written_for_is_told_with_why() {
    let registry = TypeRegistry::default();
    let declared = Declared {
        registry: &registry,
        file_path: "",
        file_imports: &[],
    };
    for (ts_type, why) in [
        ("keyof Config", "a `keyof` type"),
        ("typeof seed", "a `typeof` type"),
        ("Config['mode']", "an indexed access type"),
        ("this", "the type of whatever value holds it"),
        ("import('./other').Other", "an inline `import()`"),
        ("A & B", "an intersection"),
        ("string & { readonly __brand: 'Id' }", "an intersection"),
        ("Exclude<'a' | 'b', 'a'>", "a utility type"),
        ("ReturnType<() => string>", "a utility type"),
        ("{ [K in 'a' | 'b']: number }", "a mapped type"),
        ("T extends string ? A : B", "a conditional type"),
        ("unique symbol", "a symbol only its own declaration makes"),
        ("never", "a type with no value"),
        ("URL", "a built-in class with no value to default to"),
        ("[string, Promise<this>]", "which holds `this`"),
        ("{ at: URL }", "which holds `URL`"),
        (
            "Record<'a' | 'b', keyof Config>",
            "which holds `keyof Config`",
        ),
        ("Readonly<A & B>", "which holds `A & B`"),
        ("A & B | C", "which holds `A & B`"),
    ] {
        let reason = undefaultable(ts_type, declared);
        assert!(
            reason.as_deref().is_some_and(|reason| reason.contains(why)),
            "{ts_type}: {reason:?}"
        );
    }
    for ts_type in [
        "string",
        "User",
        "Box<string>",
        "Promise<string>",
        "A & B | null",
        "string | (A & B)",
        "[string, URL?]",
        "{ at?: URL }",
        "URL[]",
        "Map<string, URL>",
        "Record<string, URL>",
        "symbol",
        "Error",
        "WeakMap<object, string>",
    ] {
        assert_eq!(undefaultable(ts_type, declared), None, "{ts_type}");
    }
}

#[test]
fn test_get_type_default_object_literal_before_union_split() {
    // Object types containing pipes should not be split as unions
    assert_eq!(get_type_default("{ a: string | number }"), r#"({ a: "" })"#);
    assert_eq!(
        get_type_default("{ status: \"active\" | \"inactive\" }"),
        r#"({ status: "active" })"#
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
    // A nullable union defaults to the absent value its type has.
    assert_eq!(get_type_default("string | null"), "null");
    assert_eq!(get_type_default("Account | undefined"), "undefined");
    assert_eq!(get_type_default("string | null | undefined"), "null");
    assert_eq!(get_type_default("string | Account | null"), "null");
    // A collection of nullable elements is not itself nullable.
    assert_eq!(get_type_default("(string | null)[]"), "[]");
    assert_eq!(get_type_default("Array<Account | null>"), "[]");
    assert_eq!(
        get_type_default("Map<string, number | undefined>"),
        "new Map()"
    );
}

#[test]
fn test_get_type_default_tuple() {
    assert_eq!(get_type_default("[string, number]"), r#"["", 0]"#);
    assert_eq!(
        get_type_default("[label: string, at: Date]"),
        r#"["", new Date()]"#
    );
    assert_eq!(
        get_type_default("[Map<string, number>, [boolean, string | null]]"),
        "[new Map(), [false, null]]"
    );
    assert_eq!(get_type_default("[]"), "[]");
    // Only the required prefix has to be there.
    assert_eq!(
        get_type_default("[string, number?, ...boolean[]]"),
        r#"[""]"#
    );
}

#[test]
fn test_primitive_keyword_test_uses_identity_for_null_and_undefined() {
    assert_eq!(
        primitive_keyword_test("string", "value"),
        r#"typeof value === "string""#
    );
    assert_eq!(primitive_keyword_test("null", "item"), "item === null");
    assert_eq!(
        primitive_keyword_test("undefined", "item"),
        "item === undefined"
    );
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
