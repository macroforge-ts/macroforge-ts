use super::*;
use crate::ts_syn::ResolvedTypeRef;
use crate::ts_syn::abi::SpanIR;

fn span() -> SpanIR {
    SpanIR::new(0, 0)
}

fn make_decorator(name: &str, args: &str) -> crate::ts_syn::abi::DecoratorIR {
    crate::ts_syn::abi::DecoratorIR {
        name: name.into(),
        args_src: args.into(),
        span: span(),
        #[cfg(feature = "swc")]
        node: None,
    }
}

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
    // Generic type instantiations use typeDefaultValue<Args>() syntax
    assert_eq!(
        get_type_default("RecordLink<Service>"),
        "recordLinkDefaultValue<Service>()"
    );
    assert_eq!(
        get_type_default("Result<User, Error>"),
        "resultDefaultValue<User, Error>()"
    );
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
    // string | CustomType -> default of the primitive member
    assert_eq!(get_type_default("string | Account"), r#""""#);
    assert_eq!(get_type_default("string | Employee"), r#""""#);
    assert_eq!(get_type_default("string | Appointment"), r#""""#);
    assert_eq!(get_type_default("string | Site"), r#""""#);
    assert_eq!(get_type_default("number | Custom"), "0");
    assert_eq!(get_type_default("boolean | Foo"), "false");
    assert_eq!(get_type_default("bigint | Bar"), "0n");
    // Primitive not first in union
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

// ========================================================================
// Type Registry Helper Tests
// ========================================================================

use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
use crate::ts_syn::abi::{ClassIR, InterfaceIR};

fn zero_span() -> SpanIR {
    SpanIR::new(0, 0)
}

fn make_registry_with_derives() -> TypeRegistry {
    let mut registry = TypeRegistry::new();

    // User class with @derive(Clone, Hash, PartialEq, Debug, Default)
    let user_entry = TypeRegistryEntry {
        name: "User".to_string(),
        file_path: "/project/src/user.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Class(ClassIR {
            name: "User".to_string(),
            span: zero_span(),
            body_span: zero_span(),
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![make_decorator(
                "derive",
                "Clone, Hash, PartialEq, Debug, Default",
            )],
            #[cfg(feature = "swc")]
            decorators_ast: vec![],
            fields: vec![],
            methods: vec![],
            #[cfg(feature = "swc")]
            members: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(user_entry, "/project");

    // Order interface with @derive(Clone) only
    let order_entry = TypeRegistryEntry {
        name: "Order".to_string(),
        file_path: "/project/src/order.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Interface(InterfaceIR {
            name: "Order".to_string(),
            span: zero_span(),
            body_span: zero_span(),
            type_params: vec![],
            heritage: vec![],
            decorators: vec![make_decorator("derive", "Clone")],
            fields: vec![],
            methods: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(order_entry, "/project");

    // Product class with no derives
    let product_entry = TypeRegistryEntry {
        name: "Product".to_string(),
        file_path: "/project/src/product.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Class(ClassIR {
            name: "Product".to_string(),
            span: zero_span(),
            body_span: zero_span(),
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            #[cfg(feature = "swc")]
            decorators_ast: vec![],
            fields: vec![],
            methods: vec![],
            #[cfg(feature = "swc")]
            members: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(product_entry, "/project");

    registry
}

#[test]
fn test_type_has_derive() {
    let registry = make_registry_with_derives();

    // User has Clone, Hash, PartialEq, Debug, Default
    assert!(type_has_derive(&registry, "User", "Clone"));
    assert!(type_has_derive(&registry, "User", "Hash"));
    assert!(type_has_derive(&registry, "User", "PartialEq"));
    assert!(type_has_derive(&registry, "User", "Debug"));
    assert!(type_has_derive(&registry, "User", "Default"));

    // User does NOT have Ord
    assert!(!type_has_derive(&registry, "User", "Ord"));

    // Order only has Clone
    assert!(type_has_derive(&registry, "Order", "Clone"));
    assert!(!type_has_derive(&registry, "Order", "Hash"));

    // Product has no derives
    assert!(!type_has_derive(&registry, "Product", "Clone"));

    // Unknown type returns false
    assert!(!type_has_derive(&registry, "Unknown", "Clone"));
}

#[test]
fn test_type_has_derive_ambiguous_name() {
    // Simulate a type that exists in both its own file and a barrel file
    let mut registry = TypeRegistry::new();

    let phone_entry = TypeRegistryEntry {
        name: "PhoneNumber".to_string(),
        file_path: "/project/src/types/phone-number.svelte.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Interface(InterfaceIR {
            name: "PhoneNumber".to_string(),
            span: zero_span(),
            body_span: zero_span(),
            type_params: vec![],
            heritage: vec![],
            decorators: vec![make_decorator(
                "derive",
                "Default, Serialize, Deserialize, Gigaform",
            )],
            fields: vec![],
            methods: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(phone_entry, "/project");

    // Same type in barrel file
    let barrel_entry = TypeRegistryEntry {
        name: "PhoneNumber".to_string(),
        file_path: "/project/src/types/all-types.svelte.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Interface(InterfaceIR {
            name: "PhoneNumber".to_string(),
            span: zero_span(),
            body_span: zero_span(),
            type_params: vec![],
            heritage: vec![],
            decorators: vec![make_decorator(
                "derive",
                "Default, Serialize, Deserialize, Gigaform",
            )],
            fields: vec![],
            methods: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(barrel_entry, "/project");

    // Must be marked ambiguous
    assert!(
        registry
            .ambiguous_names
            .contains(&"PhoneNumber".to_string())
    );

    // type_has_derive should still return true
    assert!(type_has_derive(&registry, "PhoneNumber", "Gigaform"));
    assert!(type_has_derive(&registry, "PhoneNumber", "Default"));
    assert!(type_has_derive(&registry, "PhoneNumber", "Serialize"));
    assert!(type_has_derive(&registry, "PhoneNumber", "Deserialize"));
}

#[test]
fn test_type_has_derive_case_insensitive() {
    let registry = make_registry_with_derives();
    assert!(type_has_derive(&registry, "User", "clone"));
    assert!(type_has_derive(&registry, "User", "CLONE"));
}

#[test]
fn test_resolved_type_has_derive() {
    let registry = make_registry_with_derives();

    let resolved = ResolvedTypeRef {
        raw_type: "User".to_string(),
        base_type_name: "User".to_string(),
        registry_key: Some("src/user.ts::User".to_string()),
        is_collection: false,
        is_optional: false,
        type_args: vec![],
    };

    assert!(resolved_type_has_derive(&registry, &resolved, "Clone"));
    assert!(!resolved_type_has_derive(&registry, &resolved, "Ord"));
}

#[test]
fn test_collection_element_type() {
    // Array<User> -> User
    let user_ref = ResolvedTypeRef {
        raw_type: "User".to_string(),
        base_type_name: "User".to_string(),
        registry_key: Some("src/user.ts::User".to_string()),
        is_collection: false,
        is_optional: false,
        type_args: vec![],
    };
    let array_ref = ResolvedTypeRef {
        raw_type: "User[]".to_string(),
        base_type_name: "User".to_string(),
        registry_key: Some("src/user.ts::User".to_string()),
        is_collection: true,
        is_optional: false,
        type_args: vec![user_ref.clone()],
    };
    let elem = collection_element_type(&array_ref);
    assert!(elem.is_some());
    assert_eq!(elem.unwrap().base_type_name, "User");

    // Map<string, User> -> User (value type)
    let string_ref = ResolvedTypeRef {
        raw_type: "string".to_string(),
        base_type_name: "string".to_string(),
        registry_key: None,
        is_collection: false,
        is_optional: false,
        type_args: vec![],
    };
    let map_ref = ResolvedTypeRef {
        raw_type: "Map<string, User>".to_string(),
        base_type_name: "Map".to_string(),
        registry_key: None,
        is_collection: true,
        is_optional: false,
        type_args: vec![string_ref.clone(), user_ref.clone()],
    };
    let elem = collection_element_type(&map_ref);
    assert!(elem.is_some());
    assert_eq!(elem.unwrap().base_type_name, "User");

    // Non-collection returns None
    assert!(collection_element_type(&user_ref).is_none());
}

#[test]
fn test_map_key_type() {
    let string_ref = ResolvedTypeRef {
        raw_type: "string".to_string(),
        base_type_name: "string".to_string(),
        registry_key: None,
        is_collection: false,
        is_optional: false,
        type_args: vec![],
    };
    let user_ref = ResolvedTypeRef {
        raw_type: "User".to_string(),
        base_type_name: "User".to_string(),
        registry_key: Some("src/user.ts::User".to_string()),
        is_collection: false,
        is_optional: false,
        type_args: vec![],
    };
    let map_ref = ResolvedTypeRef {
        raw_type: "Map<string, User>".to_string(),
        base_type_name: "Map".to_string(),
        registry_key: None,
        is_collection: true,
        is_optional: false,
        type_args: vec![string_ref.clone(), user_ref.clone()],
    };

    let key = map_key_type(&map_ref);
    assert!(key.is_some());
    assert_eq!(key.unwrap().base_type_name, "string");

    // Non-Map returns None
    assert!(map_key_type(&user_ref).is_none());
}

#[test]
fn test_standalone_fn_name() {
    assert_eq!(standalone_fn_name("User", "Clone"), "userClone");
    assert_eq!(standalone_fn_name("User", "HashCode"), "userHashCode");
    assert_eq!(standalone_fn_name("User", "Equals"), "userEquals");
    assert_eq!(standalone_fn_name("Order", "ToString"), "orderToString");
    assert_eq!(
        standalone_fn_name("MyLongType", "PartialCompare"),
        "myLongTypePartialCompare"
    );
}
