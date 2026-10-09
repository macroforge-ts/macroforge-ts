use crate::abi::SpanIR;
use crate::abi::ir::TypeParamIR;
use crate::abi::ir::type_alias::{TypeAliasIR, TypeBody, TypeMember, TypeMemberKind};
use crate::abi::ir::type_alias_resolve::resolve_generic_aliases;
use crate::abi::ir::type_registry::{
    FileImportEntry, TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
};
use crate::import_registry::{ImportRegistry, SourceImportEntry, install_registry, with_registry};

fn import(local_name: &str, module_specifier: &str) -> FileImportEntry {
    FileImportEntry {
        local_name: local_name.to_string(),
        module_specifier: module_specifier.to_string(),
        original_name: None,
        is_type_only: false,
    }
}

/// `type Link<T> = <first> | T`, declared in `file_path` with `imports`.
fn link_alias(first: &str, file_path: &str, imports: Vec<FileImportEntry>) -> TypeRegistry {
    let mut registry = TypeRegistry::new();
    registry.insert(
        TypeRegistryEntry {
            name: "Link".to_string(),
            file_path: file_path.to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
                name: "Link".to_string(),
                span: SpanIR::new(0, 0),
                decorators: vec![],
                type_params: vec![TypeParamIR::named("T")],
                body: TypeBody::Union(vec![
                    TypeMember::new(TypeMemberKind::TypeRef(first.to_string())),
                    TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
                ]),
            }),
            file_imports: imports,
        },
        "/p",
    );
    registry
}

/// The generated imports, as `(local, original, module)`.
fn generated() -> Vec<(String, Option<String>, String)> {
    with_registry(|imports| {
        imports
            .generated_imports()
            .map(|generated| {
                (
                    generated.local_name.clone(),
                    generated.original_name.clone(),
                    generated.source_module.clone(),
                )
            })
            .collect()
    })
}

fn fresh_imports(source: Vec<SourceImportEntry>) {
    let mut imports = ImportRegistry::new();
    imports.install_source_imports(source);
    install_registry(imports);
}

#[test]
fn a_package_import_of_the_alias_module_is_imported_where_it_expands() {
    fresh_imports(vec![]);
    let registry = link_alias("Id", "/p/lib/index.ts", vec![import("Id", "ids")]);

    let expanded = resolve_generic_aliases("Link<Order>", &registry, "/p/app/order.ts", &[]);

    assert_eq!(expanded, "Id | Order");
    assert_eq!(
        generated(),
        vec![("Id".to_string(), None, "ids".to_string())]
    );
    assert_eq!(
        with_registry(|imports| imports.get_source("Id").map(str::to_string)),
        Some("ids".to_string())
    );
}

#[test]
fn a_name_the_caller_already_imports_the_same_way_is_reused() {
    fresh_imports(vec![SourceImportEntry {
        local_name: "Id".to_string(),
        source_module: "ids".to_string(),
        original_name: None,
        is_type_only: false,
    }]);
    let registry = link_alias("Id", "/p/lib/index.ts", vec![import("Id", "ids")]);

    let expanded = resolve_generic_aliases("Link<Order>", &registry, "/p/app/order.ts", &[]);

    assert_eq!(expanded, "Id | Order");
    assert!(generated().is_empty());
}

#[test]
fn a_name_the_caller_binds_otherwise_is_imported_under_another_name() {
    fresh_imports(vec![SourceImportEntry {
        local_name: "Id".to_string(),
        source_module: "./local-ids.js".to_string(),
        original_name: None,
        is_type_only: true,
    }]);
    let registry = link_alias("Id", "/p/lib/index.ts", vec![import("Id", "ids")]);

    let expanded = resolve_generic_aliases("Link<Order>", &registry, "/p/app/order.ts", &[]);

    assert_eq!(expanded, "__mf_scope_Id | Order");
    assert_eq!(
        generated(),
        vec![(
            "__mf_scope_Id".to_string(),
            Some("Id".to_string()),
            "ids".to_string()
        )]
    );
}

#[test]
fn a_relative_import_is_rebased_onto_the_caller() {
    fresh_imports(vec![]);
    let registry = link_alias(
        "Thing",
        "/p/lib/index.ts",
        vec![import("Thing", "./models/thing.js")],
    );

    let expanded = resolve_generic_aliases("Link<Order>", &registry, "/p/app/order.ts", &[]);

    assert_eq!(expanded, "Thing | Order");
    assert_eq!(
        generated(),
        vec![(
            "Thing".to_string(),
            None,
            "../lib/models/thing.js".to_string()
        )]
    );
}

#[test]
fn a_relative_import_with_no_caller_file_is_left_unexpanded() {
    fresh_imports(vec![]);
    let registry = link_alias(
        "Thing",
        "/p/lib/index.ts",
        vec![import("Thing", "./thing.js")],
    );

    assert_eq!(
        resolve_generic_aliases("Link<Order>", &registry, "", &[]),
        "Link<Order>"
    );
    assert!(generated().is_empty());
}

#[test]
fn a_type_declared_beside_the_alias_leaves_it_unexpanded() {
    fresh_imports(vec![]);
    let mut registry = link_alias("Sibling", "/p/lib/index.ts", vec![]);
    registry.insert(
        TypeRegistryEntry {
            name: "Sibling".to_string(),
            file_path: "/p/lib/index.ts".to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
                name: "Sibling".to_string(),
                span: SpanIR::new(0, 0),
                decorators: vec![],
                type_params: vec![],
                body: TypeBody::Alias("string".to_string()),
            }),
            file_imports: vec![],
        },
        "/p",
    );

    assert_eq!(
        resolve_generic_aliases("Link<Order>", &registry, "/p/app/order.ts", &[]),
        "Link<Order>"
    );
}

#[test]
fn literal_members_are_not_renamed_or_garbled() {
    fresh_imports(vec![SourceImportEntry {
        local_name: "Id".to_string(),
        source_module: "./local-ids.js".to_string(),
        original_name: None,
        is_type_only: true,
    }]);
    let registry = link_alias(
        "Id | \"Id\" | \"Café\"",
        "/p/lib/index.ts",
        vec![import("Id", "ids")],
    );

    let expanded = resolve_generic_aliases("Link<Order>", &registry, "/p/app/order.ts", &[]);

    assert_eq!(expanded, "__mf_scope_Id | \"Id\" | \"Café\" | Order");
}
