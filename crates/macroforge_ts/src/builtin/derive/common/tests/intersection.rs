//! Flattening intersections and the fields of a definition.

use super::zero_span;
use crate::builtin::derive::common::{
    Declared, fields_from_definition, flatten_intersection_fields, get_effective_fields,
};
use crate::ts_syn::DataTypeAlias;
use crate::ts_syn::abi::ir::type_alias::{TypeAliasIR, TypeBody, TypeMember, TypeMemberKind};
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
use crate::ts_syn::abi::{InterfaceFieldIR, InterfaceIR};

// ========================================================================
// Intersection Type Flattening Tests
// ========================================================================

/// A file of the project that imports nothing.
fn within(registry: &TypeRegistry) -> Declared<'_> {
    Declared {
        registry,
        file_path: "/project/src/page.ts",
        file_imports: &[],
    }
}

fn interface_entry(
    name: &str,
    file_path: &str,
    fields: Vec<InterfaceFieldIR>,
) -> TypeRegistryEntry {
    TypeRegistryEntry {
        name: name.to_string(),
        file_path: file_path.to_string(),
        is_exported: true,
        exported_as: Vec::new(),
        re_exported: Vec::new(),
        definition: TypeDefinitionIR::Interface(InterfaceIR {
            name: name.to_string(),
            span: zero_span(),
            body_span: zero_span(),
            type_params: vec![],
            heritage: vec![],
            decorators: vec![],
            fields,
            methods: vec![],
        }),
        file_imports: vec![],
    }
}

fn make_field(name: &str, ts_type: &str) -> InterfaceFieldIR {
    InterfaceFieldIR {
        name: name.to_string(),
        key: crate::ts_syn::abi::MemberKey::Named,
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

    let result = flatten_intersection_fields(&members, within(&TypeRegistry::default()));
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

    let result = flatten_intersection_fields(&members, within(&TypeRegistry::default()));
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
        exported_as: Vec::new(),
        re_exported: Vec::new(),
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

    let result = flatten_intersection_fields(&members, within(&registry));
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

    let result = flatten_intersection_fields(&members, within(&TypeRegistry::default()));
    assert!(result.is_none());
}

#[test]
fn test_flatten_intersection_type_ref_not_in_registry() {
    let registry = TypeRegistry::new();

    let members = vec![TypeMember::new(TypeMemberKind::TypeRef(
        "Unknown".to_string(),
    ))];

    let result = flatten_intersection_fields(&members, within(&registry));
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

    let result = flatten_intersection_fields(&members, within(&TypeRegistry::default()));
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

    let result = get_effective_fields(&ta, within(&TypeRegistry::default()));
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

    let result = get_effective_fields(&ta, within(&TypeRegistry::default()));
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

    let result = get_effective_fields(&ta, within(&TypeRegistry::default()));
    assert!(result.is_none());
}

#[test]
fn test_fields_from_definition_interface() {
    let entry = interface_entry(
        "User",
        "/project/src/user.ts",
        vec![make_field("id", "number"), make_field("name", "string")],
    );

    let result = fields_from_definition(&entry, &TypeRegistry::default());
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
                exported_as: Vec::new(),
                re_exported: Vec::new(),
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
    assert!(flatten_intersection_fields(&members, within(&registry)).is_none());
}

/// A member is the type the file's imports lead to under its name, and that
/// type's own members are read by its file's imports in turn.
#[test]
fn an_intersection_member_is_the_type_its_file_imports() {
    use crate::ts_syn::abi::ir::type_registry::FileImportEntry;

    let import = |local: &str, exported: &str, module: &str| FileImportEntry {
        local_name: local.to_string(),
        module_specifier: module.to_string(),
        original_name: (local != exported).then(|| exported.to_string()),
        is_type_only: true,
    };
    let mut registry = TypeRegistry::new();
    registry.insert(
        interface_entry(
            "Account",
            "/project/src/account.ts",
            vec![make_field("id", "number")],
        ),
        "/project",
    );
    // What the page's own name for the account means elsewhere.
    registry.insert(
        interface_entry(
            "Held",
            "/project/src/other.ts",
            vec![make_field("wrong", "string")],
        ),
        "/project",
    );
    // Two bases of one name, each with its own module's `Base` under it.
    for (file_path, base, field) in [
        ("/project/src/left.ts", "./left-base", "left"),
        ("/project/src/right.ts", "./right-base", "right"),
    ] {
        registry.insert(
            TypeRegistryEntry {
                name: "Mixed".to_string(),
                file_path: file_path.to_string(),
                is_exported: true,
                exported_as: Vec::new(),
                re_exported: Vec::new(),
                definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
                    name: "Mixed".to_string(),
                    span: zero_span(),
                    decorators: vec![],
                    type_params: vec![],
                    body: TypeBody::Intersection(vec![TypeMember::new(TypeMemberKind::TypeRef(
                        "Base".to_string(),
                    ))]),
                }),
                file_imports: vec![import("Base", "Base", base)],
            },
            "/project",
        );
        let base_path = file_path.replace(".ts", "-base.ts");
        registry.insert(
            interface_entry("Base", &base_path, vec![make_field(field, "string")]),
            "/project",
        );
    }

    let imports = [
        import("Held", "Account", "./account"),
        import("Mixed", "Mixed", "./right"),
    ];
    let page = Declared {
        registry: &registry,
        file_path: "/project/src/page.ts",
        file_imports: &imports,
    };
    let names = |members: &[TypeMember]| {
        flatten_intersection_fields(members, page).map(|fields| {
            fields
                .into_iter()
                .map(|field| field.name)
                .collect::<Vec<_>>()
        })
    };

    let held = [TypeMember::new(TypeMemberKind::TypeRef("Held".to_string()))];
    assert_eq!(names(&held), Some(vec!["id".to_string()]));
    let mixed = [TypeMember::new(TypeMemberKind::TypeRef(
        "Mixed".to_string(),
    ))];
    assert_eq!(names(&mixed), Some(vec!["right".to_string()]));
}
