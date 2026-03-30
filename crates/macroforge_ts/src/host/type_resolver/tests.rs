use super::helpers::split_type_args;
use super::resolver::TypeResolver;
use crate::ts_syn::abi::ClassIR;
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};

fn make_test_registry() -> TypeRegistry {
    let mut registry = TypeRegistry::new();

    let zero_span = crate::ts_syn::abi::SpanIR { start: 0, end: 0 };

    // Add a User class
    let user_entry = TypeRegistryEntry {
        name: "User".to_string(),
        file_path: "/project/src/models/user.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Class(ClassIR {
            name: "User".to_string(),
            span: zero_span,
            body_span: zero_span,
            is_abstract: false,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            decorators_ast: vec![],
            fields: vec![],
            methods: vec![],
            members: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(user_entry, "/project");

    // Add an Order interface
    let order_entry = TypeRegistryEntry {
        name: "Order".to_string(),
        file_path: "/project/src/models/order.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Interface(crate::ts_syn::abi::InterfaceIR {
            name: "Order".to_string(),
            span: zero_span,
            body_span: zero_span,
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            fields: vec![],
            methods: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(order_entry, "/project");

    registry
}

#[test]
fn test_resolve_primitive() {
    let registry = make_test_registry();
    let resolver = TypeResolver::new(&registry);

    let resolved = resolver.resolve("string");
    assert_eq!(resolved.base_type_name, "string");
    assert!(resolved.registry_key.is_none());
    assert!(!resolved.is_collection);
    assert!(!resolved.is_optional);
}

#[test]
fn test_resolve_known_type() {
    let registry = make_test_registry();
    let resolver = TypeResolver::new(&registry);

    let resolved = resolver.resolve("User");
    assert_eq!(resolved.base_type_name, "User");
    assert!(resolved.registry_key.is_some());
    assert!(!resolved.is_collection);
}

#[test]
fn test_resolve_array() {
    let registry = make_test_registry();
    let resolver = TypeResolver::new(&registry);

    let resolved = resolver.resolve("User[]");
    assert_eq!(resolved.base_type_name, "User");
    assert!(resolved.registry_key.is_some());
    assert!(resolved.is_collection);
    assert_eq!(resolved.type_args.len(), 1);
}

#[test]
fn test_resolve_optional() {
    let registry = make_test_registry();
    let resolver = TypeResolver::new(&registry);

    let resolved = resolver.resolve("User | undefined");
    assert_eq!(resolved.base_type_name, "User");
    assert!(resolved.registry_key.is_some());
    assert!(resolved.is_optional);
}

#[test]
fn test_resolve_generic() {
    let registry = make_test_registry();
    let resolver = TypeResolver::new(&registry);

    let resolved = resolver.resolve("Map<string, User>");
    assert_eq!(resolved.base_type_name, "Map");
    assert!(resolved.registry_key.is_none()); // Map is a primitive
    assert!(resolved.is_collection);
    assert_eq!(resolved.type_args.len(), 2);
    assert_eq!(resolved.type_args[0].base_type_name, "string");
    assert!(resolved.type_args[1].registry_key.is_some()); // User resolves
}

#[test]
fn test_resolve_unknown_type() {
    let registry = make_test_registry();
    let resolver = TypeResolver::new(&registry);

    let resolved = resolver.resolve("UnknownType");
    assert_eq!(resolved.base_type_name, "UnknownType");
    assert!(resolved.registry_key.is_none());
}

#[test]
fn test_split_type_args() {
    assert_eq!(split_type_args("string, number"), vec!["string", "number"]);
    assert_eq!(
        split_type_args("Map<string, number>, User"),
        vec!["Map<string, number>", "User"]
    );
    assert_eq!(split_type_args("string"), vec!["string"]);
}
