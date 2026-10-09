use super::{
    HashMap, TypeBody, TypeDefinitionIR, TypeMember, TypeMemberKind, TypeRegistry,
    resolve_generic_aliases, split_top_level_commas, split_top_level_intersection,
    split_top_level_union, substitute_tokens,
};
use crate::abi::SpanIR;
use crate::abi::ir::TypeParamIR;
use crate::abi::ir::type_alias::TypeAliasIR;
use crate::abi::ir::type_registry::TypeRegistryEntry;

fn alias_entry(
    name: &str,
    type_params: Vec<&str>,
    body: TypeBody,
    file_path: &str,
) -> TypeRegistryEntry {
    TypeRegistryEntry {
        name: name.to_string(),
        file_path: file_path.to_string(),
        is_exported: true,
        definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
            name: name.to_string(),
            span: SpanIR::new(0, 0),
            decorators: vec![],
            type_params: type_params.into_iter().map(TypeParamIR::named).collect(),
            body,
        }),
        file_imports: vec![],
    }
}

fn record_link_registry() -> TypeRegistry {
    let mut registry = TypeRegistry::new();
    // type RecordLink<T> = string | T
    let body = TypeBody::Union(vec![
        TypeMember::new(TypeMemberKind::TypeRef("string".to_string())),
        TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
    ]);
    registry.insert(
        alias_entry("RecordLink", vec!["T"], body, "/p/record-link.ts"),
        "/p",
    );
    registry
}

#[test]
fn expands_record_link_to_union() {
    let reg = record_link_registry();
    assert_eq!(
        resolve_generic_aliases("RecordLink<ErrandMessage>", &reg, "", &[]),
        "string | ErrandMessage"
    );
}

#[test]
fn expands_record_link_inside_array() {
    let reg = record_link_registry();
    assert_eq!(
        resolve_generic_aliases("Array<RecordLink<ErrandMessage>>", &reg, "", &[]),
        "Array<string | ErrandMessage>"
    );
}

#[test]
fn expands_record_link_inside_array_suffix() {
    let reg = record_link_registry();
    assert_eq!(
        resolve_generic_aliases("RecordLink<Foo>[]", &reg, "", &[]),
        "(string | Foo)[]"
    );
}

#[test]
fn expands_record_link_inside_map() {
    let reg = record_link_registry();
    assert_eq!(
        resolve_generic_aliases("Map<string, RecordLink<Foo>>", &reg, "", &[]),
        "Map<string, string | Foo>"
    );
}

#[test]
fn a_left_out_parameter_takes_its_default() {
    let mut registry = TypeRegistry::new();
    // type Page<T, M = Meta> = { items: T } | M
    let mut entry = alias_entry(
        "Page",
        vec!["T", "M"],
        TypeBody::Union(vec![
            TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
            TypeMember::new(TypeMemberKind::TypeRef("M".to_string())),
        ]),
        "/p/page.ts",
    );
    if let TypeDefinitionIR::TypeAlias(alias) = &mut entry.definition
        && let Some(meta) = alias.type_params.get_mut(1)
    {
        meta.default = Some("Meta".to_string());
    }
    registry.insert(entry, "/p");
    assert_eq!(
        resolve_generic_aliases("Page<User>", &registry, "", &[]),
        "User | Meta"
    );
    assert_eq!(
        resolve_generic_aliases("Page<User, Info>", &registry, "", &[]),
        "User | Info"
    );
}

#[test]
fn passes_through_missing_alias() {
    let reg = TypeRegistry::new();
    assert_eq!(
        resolve_generic_aliases("RecordLink<Foo>", &reg, "", &[]),
        "RecordLink<Foo>"
    );
}

#[test]
fn passes_through_non_generic() {
    let reg = record_link_registry();
    assert_eq!(resolve_generic_aliases("User", &reg, "", &[]), "User");
    assert_eq!(resolve_generic_aliases("string", &reg, "", &[]), "string");
}

#[test]
fn passes_through_lowercase_base() {
    let reg = record_link_registry();
    assert_eq!(
        resolve_generic_aliases("partial<User>", &reg, "", &[]),
        "partial<User>"
    );
}

#[test]
fn passes_through_alias_with_unrenderable_member() {
    let mut reg = TypeRegistry::new();
    // type Outcome<T> = { ok: T } | string
    let body = TypeBody::Union(vec![
        TypeMember::new(TypeMemberKind::Object { fields: vec![] }),
        TypeMember::new(TypeMemberKind::TypeRef("string".to_string())),
    ]);
    reg.insert(
        alias_entry("Outcome", vec!["T"], body, "/p/outcome.ts"),
        "/p",
    );
    // type Branded<T> = T & { readonly [B]: true }
    let body = TypeBody::Intersection(vec![
        TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
        TypeMember::new(TypeMemberKind::Brand(vec!["B".to_string()])),
    ]);
    reg.insert(
        alias_entry("Branded", vec!["T"], body, "/p/branded.ts"),
        "/p",
    );

    assert_eq!(
        resolve_generic_aliases("Outcome<Foo>", &reg, "", &[]),
        "Outcome<Foo>"
    );
    assert_eq!(
        resolve_generic_aliases("Branded<number>", &reg, "", &[]),
        "Branded<number>"
    );
}

#[test]
fn passes_through_arity_mismatch() {
    let reg = record_link_registry();
    assert_eq!(
        resolve_generic_aliases("RecordLink<A, B>", &reg, "", &[]),
        "RecordLink<A, B>"
    );
}

#[test]
fn nested_substitution_collapses() {
    let mut reg = TypeRegistry::new();
    reg.insert(
        alias_entry(
            "Inner",
            vec!["T"],
            TypeBody::Union(vec![
                TypeMember::new(TypeMemberKind::TypeRef("T".to_string())),
                TypeMember::new(TypeMemberKind::Literal("null".to_string())),
            ]),
            "/p/inner.ts",
        ),
        "/p",
    );
    reg.insert(
        alias_entry(
            "Outer",
            vec!["U"],
            TypeBody::Alias("Inner<U>".to_string()),
            "/p/outer.ts",
        ),
        "/p",
    );
    assert_eq!(
        resolve_generic_aliases("Outer<User>", &reg, "", &[]),
        "User | null"
    );
}

#[test]
fn skips_object_body() {
    let mut reg = TypeRegistry::new();
    reg.insert(
        alias_entry(
            "Boxed",
            vec!["T"],
            TypeBody::Object { fields: vec![] },
            "/p/boxed.ts",
        ),
        "/p",
    );
    assert_eq!(
        resolve_generic_aliases("Boxed<User>", &reg, "", &[]),
        "Boxed<User>"
    );
}

#[test]
fn substitute_tokens_respects_word_boundaries() {
    let mut subs = HashMap::new();
    subs.insert("T", "ErrandMessage");
    assert_eq!(substitute_tokens("T", &subs), "ErrandMessage");
    assert_eq!(substitute_tokens("Array<T>", &subs), "Array<ErrandMessage>");
    assert_eq!(substitute_tokens("MyT", &subs), "MyT");
    assert_eq!(substitute_tokens("TFoo", &subs), "TFoo");
}

#[test]
fn split_top_level_union_skips_arrow_types() {
    assert_eq!(
        split_top_level_union("(() => void) | null"),
        Some(vec!["(() => void)", "null"])
    );
    assert_eq!(
        split_top_level_union("Map<string, (x: number) => string> | undefined"),
        Some(vec!["Map<string, (x: number) => string>", "undefined"])
    );
    assert_eq!(
        split_top_level_intersection("number & { readonly [B]: true }"),
        Some(vec!["number", "{ readonly [B]: true }"])
    );
}

#[test]
fn split_top_level_commas_respects_nesting() {
    assert_eq!(
        split_top_level_commas("string, Map<string, number>"),
        vec!["string", "Map<string, number>"]
    );
    assert_eq!(split_top_level_commas("A"), vec!["A"]);
}
