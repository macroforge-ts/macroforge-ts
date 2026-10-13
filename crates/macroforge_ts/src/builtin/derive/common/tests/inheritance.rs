//! The fields an instance holds beyond the ones its own type declares.

use super::zero_span;
use crate::builtin::derive::common::{Declared, class_instance_fields, interface_instance_fields};
use crate::ts_syn::abi::ir::class::{FieldIR, MemberKey, Visibility};
use crate::ts_syn::abi::ir::type_registry::{TypeDefinitionIR, TypeRegistry, TypeRegistryEntry};
use crate::ts_syn::abi::ir::{InterfaceFieldIR, TypeParamIR};
use crate::ts_syn::abi::{ClassIR, InterfaceIR};

const FILE: &str = "/project/src/shapes.ts";

fn field(name: &str, visibility: Visibility) -> FieldIR {
    FieldIR {
        name: name.to_string(),
        key: if name.starts_with('#') {
            MemberKey::Private
        } else {
            MemberKey::Named
        },
        span: zero_span(),
        ts_type: "string".to_string(),
        optional: false,
        readonly: false,
        visibility,
        decorators: vec![],
        initializer: None,
        accessor: false,
        untyped: false,
    }
}

fn class(name: &str, superclass: Option<&str>, fields: Vec<FieldIR>) -> ClassIR {
    ClassIR {
        name: name.to_string(),
        span: zero_span(),
        body_span: zero_span(),
        is_abstract: false,
        is_declare: false,
        type_params: vec![],
        heritage: superclass.map(str::to_string).into_iter().collect(),
        superclass: superclass.map(str::to_string),
        decorators: vec![],
        fields,
        methods: vec![],
    }
}

fn interface_field(name: &str, ts_type: &str) -> InterfaceFieldIR {
    InterfaceFieldIR {
        name: name.to_string(),
        key: MemberKey::Named,
        span: zero_span(),
        ts_type: ts_type.to_string(),
        optional: false,
        readonly: false,
        decorators: vec![],
    }
}

fn interface(name: &str, extends: &[&str], fields: Vec<InterfaceFieldIR>) -> InterfaceIR {
    InterfaceIR {
        name: name.to_string(),
        span: zero_span(),
        body_span: zero_span(),
        type_params: vec![],
        heritage: extends.iter().map(|base| base.to_string()).collect(),
        decorators: vec![],
        fields,
        methods: vec![],
    }
}

fn registry_of(definitions: Vec<TypeDefinitionIR>) -> TypeRegistry {
    let mut registry = TypeRegistry::new();
    for definition in definitions {
        let name = match &definition {
            TypeDefinitionIR::Class(class) => class.name.clone(),
            TypeDefinitionIR::Interface(interface) => interface.name.clone(),
            TypeDefinitionIR::TypeAlias(alias) => alias.name.clone(),
            TypeDefinitionIR::Enum(declared) => declared.name.clone(),
        };
        registry.insert(
            TypeRegistryEntry {
                name,
                file_path: FILE.to_string(),
                is_exported: true,
                exported_as: Vec::new(),
                definition,
                file_imports: vec![],
            },
            "/project",
        );
    }
    registry
}

fn declared(registry: &TypeRegistry) -> Declared<'_> {
    Declared {
        registry,
        file_path: FILE,
        file_imports: &[],
    }
}

fn names(fields: &[FieldIR]) -> Vec<&str> {
    fields.iter().map(|field| field.name.as_str()).collect()
}

#[test]
fn a_subclass_holds_its_superclasses_fields_outermost_first() {
    let root = class("Root", None, vec![field("id", Visibility::Public)]);
    let middle = class(
        "Middle",
        Some("Root"),
        vec![field("rank", Visibility::Protected)],
    );
    let leaf = class(
        "Leaf",
        Some("Middle"),
        vec![field("extra", Visibility::Private)],
    );
    let registry = registry_of(vec![
        TypeDefinitionIR::Class(root),
        TypeDefinitionIR::Class(middle),
        TypeDefinitionIR::Class(leaf.clone()),
    ]);
    let fields = class_instance_fields(&leaf, declared(&registry)).expect("the fields resolve");
    assert_eq!(names(&fields), ["id", "rank", "extra"]);
}

#[test]
fn a_superclass_the_registry_lacks_is_an_error_once_a_project_was_scanned() {
    let button = class(
        "Button",
        Some("HTMLElement"),
        vec![field("label", Visibility::Public)],
    );
    let registry = registry_of(vec![TypeDefinitionIR::Class(button.clone())]);
    let message = class_instance_fields(&button, declared(&registry)).expect_err("unknown base");
    assert!(message.contains("extends 'HTMLElement', which this project does not declare"));

    // With no scan there is no registry to miss it from.
    let unscanned = TypeRegistry::new();
    let fields = class_instance_fields(&button, declared(&unscanned)).expect("own fields");
    assert_eq!(names(&fields), ["label"]);
}

#[test]
fn a_field_a_subclass_cannot_reach_is_an_error() {
    let account = class("Account", None, vec![field("balance", Visibility::Private)]);
    let savings = class(
        "Savings",
        Some("Account"),
        vec![field("rate", Visibility::Public)],
    );
    let registry = registry_of(vec![
        TypeDefinitionIR::Class(account),
        TypeDefinitionIR::Class(savings.clone()),
    ]);
    let message = class_instance_fields(&savings, declared(&registry)).expect_err("private");
    assert!(message.contains("field 'balance' of class 'Account' is private to it"));

    let vault = class("Vault", None, vec![field("#secret", Visibility::Public)]);
    let message =
        class_instance_fields(&vault, declared(&TypeRegistry::new())).expect_err("hash-private");
    assert!(message.contains("field '#secret' of class 'Vault'"));
}

#[test]
fn a_generic_superclass_is_an_error() {
    let mut holder = class("Holder", None, vec![field("held", Visibility::Public)]);
    holder.type_params = vec![TypeParamIR {
        name: "T".to_string(),
        constraint: None,
        default: None,
        is_const: false,
    }];
    let named = class("NamedHolder", Some("Holder"), vec![]);
    let registry = registry_of(vec![
        TypeDefinitionIR::Class(holder),
        TypeDefinitionIR::Class(named.clone()),
    ]);
    let message = class_instance_fields(&named, declared(&registry)).expect_err("generic base");
    assert!(message.contains("extends the generic class 'Holder'"));
}

#[test]
fn an_interface_has_the_fields_of_what_it_extends_and_may_narrow_them() {
    let entity = interface(
        "Entity",
        &[],
        vec![
            interface_field("id", "string"),
            interface_field("kind", "string"),
        ],
    );
    let dated = interface("Dated", &[], vec![interface_field("at", "Date")]);
    let account = interface(
        "Account",
        &["Entity", "Dated"],
        vec![
            interface_field("kind", "'account'"),
            interface_field("balance", "number"),
        ],
    );
    let registry = registry_of(vec![
        TypeDefinitionIR::Interface(entity),
        TypeDefinitionIR::Interface(dated),
        TypeDefinitionIR::Interface(account.clone()),
    ]);
    let fields = interface_instance_fields(&account, declared(&registry)).expect("resolves");
    let described: Vec<(&str, &str)> = fields
        .iter()
        .map(|field| (field.name.as_str(), field.ts_type.as_str()))
        .collect();
    assert_eq!(
        described,
        [
            ("id", "string"),
            ("kind", "'account'"),
            ("at", "Date"),
            ("balance", "number"),
        ]
    );

    let row = interface(
        "Row",
        &["ExternalRow"],
        vec![interface_field("id", "string")],
    );
    let registry = registry_of(vec![TypeDefinitionIR::Interface(row.clone())]);
    let message = interface_instance_fields(&row, declared(&registry)).expect_err("unknown base");
    assert!(message.contains("extends 'ExternalRow', which this project does not declare"));
}

#[test]
fn an_untyped_rune_field_takes_the_type_the_rune_holds() {
    let rune = |name: &str, initializer: &str| FieldIR {
        ts_type: "any".to_string(),
        untyped: true,
        initializer: Some(initializer.to_string()),
        ..field(name, Visibility::Public)
    };
    let counter = class(
        "Counter",
        None,
        vec![
            rune("count", "$state(0)"),
            rune("label", "$state<string | null>(null)"),
            rune("later", "$state<number>()"),
            rune("raw", "$state.raw('a')"),
            rune("list", "$state([])"),
            rune("sum", "$derived(1)"),
            FieldIR {
                initializer: Some("$state(0)".to_string()),
                ..field("said", Visibility::Public)
            },
        ],
    );
    let fields = class_instance_fields(&counter, declared(&TypeRegistry::new())).expect("fields");
    assert_eq!(
        fields
            .iter()
            .map(|field| (field.name.as_str(), field.ts_type.as_str(), field.untyped))
            .collect::<Vec<_>>(),
        [
            ("count", "number", false),
            ("label", "string | null", false),
            ("later", "number | undefined", false),
            ("raw", "string", false),
            ("list", "any", true),
            ("sum", "any", true),
            ("said", "string", false),
        ]
    );
}

#[test]
fn a_member_without_a_name_is_an_error_wherever_it_is_declared() {
    let unnamed = |name: &str| FieldIR {
        key: MemberKey::Unnamed,
        ..field(name, Visibility::Public)
    };
    let own = class(
        "Bag",
        None,
        vec![field("id", Visibility::Public), unnamed("[KEY]")],
    );
    let message = class_instance_fields(&own, declared(&TypeRegistry::new())).expect_err("unnamed");
    assert!(message.contains("member `[KEY]` of 'Bag' has no name a derive can read it by"));

    let base = class("Base", None, vec![unnamed("[key: string]")]);
    let derived = class(
        "Derived",
        Some("Base"),
        vec![field("id", Visibility::Public)],
    );
    let registry = registry_of(vec![
        TypeDefinitionIR::Class(base),
        TypeDefinitionIR::Class(derived.clone()),
    ]);
    let message = class_instance_fields(&derived, declared(&registry)).expect_err("inherited");
    assert!(message.contains("member `[key: string]` of 'Derived'"));

    let indexed = interface(
        "Indexed",
        &[],
        vec![InterfaceFieldIR {
            key: MemberKey::Unnamed,
            ..interface_field("[key: string]", "unknown")
        }],
    );
    let extending = interface(
        "Extending",
        &["Indexed"],
        vec![interface_field("id", "string")],
    );
    let registry = registry_of(vec![
        TypeDefinitionIR::Interface(indexed),
        TypeDefinitionIR::Interface(extending.clone()),
    ]);
    let message =
        interface_instance_fields(&extending, declared(&registry)).expect_err("inherited");
    assert!(message.contains("member `[key: string]` of 'Extending'"));

    // A name that only looks bracketed or private is a name.
    let quoted = class(
        "Quoted",
        None,
        vec![
            FieldIR {
                key: MemberKey::Named,
                ..field("#tag", Visibility::Public)
            },
            field("[x]", Visibility::Public),
        ],
    );
    let fields = class_instance_fields(&quoted, declared(&TypeRegistry::new()))
        .expect("quoted names are names");
    assert_eq!(names(&fields), ["#tag", "[x]"]);
}
