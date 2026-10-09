//! Flattening intersections and the fields of a definition.

use super::zero_span;
use crate::builtin::derive::common::{
    fields_from_definition, flatten_intersection_fields, get_effective_fields,
};
use crate::ts_syn::DataTypeAlias;
use crate::ts_syn::abi::ir::type_alias::{TypeAliasIR, TypeBody, TypeMember, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
use crate::ts_syn::abi::{InterfaceFieldIR, InterfaceIR};

// ========================================================================
// Intersection Type Flattening Tests
// ========================================================================

fn make_field(name: &str, ts_type: &str) -> InterfaceFieldIR {
    InterfaceFieldIR {
        name: name.to_string(),
        span: zero_span(),
        ts_type: ts_type.to_string(),
        optional: false,
        readonly: false,
        decorators: vec![],
    }
}

#[test]
fn test_flatten_intersection_all_inline_objects() {
    let members = vec![
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("variant", "'Account'")],
        }),
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("id", "number"), make_field("name", "string")],
        }),
    ];

    let result = flatten_intersection_fields(&members, &TypeRegistry::default());
    assert!(result.is_some());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].name, "variant");
    assert_eq!(fields[1].name, "id");
    assert_eq!(fields[2].name, "name");
}

#[test]
fn test_flatten_intersection_deduplicates_fields() {
    let members = vec![
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("id", "number"), make_field("name", "string")],
        }),
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("id", "number"), make_field("email", "string")],
        }),
    ];

    let result = flatten_intersection_fields(&members, &TypeRegistry::default());
    assert!(result.is_some());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 3); // id, name, email (id deduped)
    assert_eq!(fields[0].name, "id");
    assert_eq!(fields[1].name, "name");
    assert_eq!(fields[2].name, "email");
}

#[test]
fn test_flatten_intersection_with_type_ref_resolved() {
    let mut registry = TypeRegistry::new();
    let account_entry = TypeRegistryEntry {
        name: "Account".to_string(),
        file_path: "/project/src/account.ts".to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::Interface(InterfaceIR {
            name: "Account".to_string(),
            span: zero_span(),
            body_span: zero_span(),
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            fields: vec![make_field("id", "number"), make_field("balance", "number")],
            methods: vec![],
        }),
        file_imports: vec![],
    };
    registry.insert(account_entry, "/project");

    let members = vec![
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("variant", "'Account'")],
        }),
        TypeMember::new(TypeMemberKind::TypeRef("Account".to_string())),
    ];

    let result = flatten_intersection_fields(&members, &registry);
    assert!(result.is_some());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].name, "variant");
    assert_eq!(fields[1].name, "id");
    assert_eq!(fields[2].name, "balance");
}

#[test]
fn test_flatten_intersection_unresolvable_type_ref_returns_none() {
    // No registry provided
    let members = vec![
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("variant", "'Account'")],
        }),
        TypeMember::new(TypeMemberKind::TypeRef("Account".to_string())),
    ];

    let result = flatten_intersection_fields(&members, &TypeRegistry::default());
    assert!(result.is_none());
}

#[test]
fn test_flatten_intersection_type_ref_not_in_registry() {
    let registry = TypeRegistry::new();

    let members = vec![TypeMember::new(TypeMemberKind::TypeRef(
        "Unknown".to_string(),
    ))];

    let result = flatten_intersection_fields(&members, &registry);
    assert!(result.is_none());
}

#[test]
fn test_flatten_intersection_skips_literals() {
    let members = vec![
        TypeMember::new(TypeMemberKind::Literal("'active'".to_string())),
        TypeMember::new(TypeMemberKind::Object {
            fields: vec![make_field("id", "number")],
        }),
    ];

    let result = flatten_intersection_fields(&members, &TypeRegistry::default());
    assert!(result.is_some());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].name, "id");
}

#[test]
fn test_get_effective_fields_object_type() {
    let ta = DataTypeAlias {
        inner: TypeAliasIR {
            name: "Point".to_string(),
            span: zero_span(),
            decorators: vec![],
            type_params: vec![],
            body: TypeBody::Object {
                fields: vec![make_field("x", "number"), make_field("y", "number")],
            },
        },
    };

    let result = get_effective_fields(&ta, &TypeRegistry::default());
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 2);
}

#[test]
fn test_get_effective_fields_intersection_type() {
    let ta = DataTypeAlias {
        inner: TypeAliasIR {
            name: "AdminUser".to_string(),
            span: zero_span(),
            decorators: vec![],
            type_params: vec![],
            body: TypeBody::Intersection(vec![
                TypeMember::new(TypeMemberKind::Object {
                    fields: vec![make_field("role", "'admin'")],
                }),
                TypeMember::new(TypeMemberKind::Object {
                    fields: vec![make_field("name", "string")],
                }),
            ]),
        },
    };

    let result = get_effective_fields(&ta, &TypeRegistry::default());
    assert!(result.is_some());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "role");
    assert_eq!(fields[1].name, "name");
}

#[test]
fn test_get_effective_fields_union_returns_none() {
    let ta = DataTypeAlias {
        inner: TypeAliasIR {
            name: "Status".to_string(),
            span: zero_span(),
            decorators: vec![],
            type_params: vec![],
            body: TypeBody::Union(vec![
                TypeMember::new(TypeMemberKind::Literal("'active'".to_string())),
                TypeMember::new(TypeMemberKind::Literal("'inactive'".to_string())),
            ]),
        },
    };

    let result = get_effective_fields(&ta, &TypeRegistry::default());
    assert!(result.is_none());
}

#[test]
fn test_fields_from_definition_interface() {
    let def = TypeDefinitionIR::Interface(InterfaceIR {
        name: "User".to_string(),
        span: zero_span(),
        body_span: zero_span(),
        type_params: vec![],
        heritage: vec![],
        decorators: vec![],
        fields: vec![make_field("id", "number"), make_field("name", "string")],
        methods: vec![],
    });

    let result = fields_from_definition(&def, &TypeRegistry::default());
    assert!(result.is_some());
    let fields = result.unwrap();
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].name, "id");
}

#[test]
fn flatten_intersection_stops_at_a_circular_alias() {
    let mut registry = TypeRegistry::new();
    for (name, other) in [("Left", "Right"), ("Right", "Left")] {
        registry.insert(
            TypeRegistryEntry {
                name: name.to_string(),
                file_path: "/project/src/cycle.ts".to_string(),
                is_exported: true,
                definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
                    name: name.to_string(),
                    span: zero_span(),
                    decorators: vec![],
                    type_params: vec![],
                    body: TypeBody::Intersection(vec![TypeMember::new(TypeMemberKind::TypeRef(
                        other.to_string(),
                    ))]),
                }),
                file_imports: vec![],
            },
            "/project",
        );
    }

    let members = vec![TypeMember::new(TypeMemberKind::TypeRef("Left".to_string()))];
    assert!(flatten_intersection_fields(&members, &registry).is_none());
}
