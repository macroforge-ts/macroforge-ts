//! Derive lookups and names through the type registry.

use super::{make_decorator, zero_span};
use crate::builtin::derive::common::{
    collection_element_type, resolved_type_has_derive, standalone_fn_name, type_has_derive,
};
use crate::ts_syn::ResolvedTypeRef;
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
use crate::ts_syn::abi::{ClassIR, InterfaceIR};

// ========================================================================
// Type Registry Helper Tests
// ========================================================================

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
            fields: vec![],
            methods: vec![],
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
            fields: vec![],
            methods: vec![],
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
                "Default, Encode, Decode, Gigaform",
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
                "Default, Encode, Decode, Gigaform",
            )],
            fields: vec![],
            methods: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(barrel_entry, "/project");

    // Must be marked ambiguous
    assert!(registry.ambiguous_names.contains("PhoneNumber"));

    // type_has_derive should still return true
    assert!(type_has_derive(&registry, "PhoneNumber", "Gigaform"));
    assert!(type_has_derive(&registry, "PhoneNumber", "Default"));
    assert!(type_has_derive(&registry, "PhoneNumber", "Encode"));
    assert!(type_has_derive(&registry, "PhoneNumber", "Decode"));
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
