use crate::builtin::derive::common::{OrdField, Ordering, generate_field_order};
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;

fn compare(name: &str, ts_type: &str) -> String {
    let field = OrdField {
        name: name.to_string(),
        ts_type: ts_type.to_string(),
        optional: false,
    };
    generate_field_order(
        Ordering::Partial,
        &field,
        "a",
        "b",
        None,
        &TypeRegistry::default(),
    )
}

#[test]
fn partial_ord_and_ord_on_a_class_emit_distinct_methods() {
    let source = "/** @derive(PartialEq, Eq, PartialOrd, Ord) */\nexport class Version {\n  major: number = 1;\n}\n";
    let result = crate::expand_core::expand_inner(source, "probe.ts", None).unwrap();
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert_eq!(
        result
            .code
            .matches("static partialCompare(a: Version, b: Version): number | null")
            .count(),
        1,
        "{}",
        result.code
    );
    assert_eq!(
        result
            .code
            .matches("static compare(a: Version, b: Version): number")
            .count(),
        1,
        "{}",
        result.code
    );
}

#[test]
fn numbers_leave_nan_unordered() {
    assert_eq!(
        compare("id", "number"),
        "(a.id < b.id ? -1 : a.id > b.id ? 1 : a.id === b.id ? 0 : null)"
    );
}

#[test]
fn strings_order_by_code_units_like_equality() {
    assert_eq!(
        compare("name", "string"),
        "(a.name < b.name ? -1 : a.name > b.name ? 1 : 0)"
    );
}

#[test]
fn booleans_order_false_first() {
    assert_eq!(
        compare("active", "boolean"),
        "(a.active === b.active ? 0 : a.active ? 1 : -1)"
    );
}

#[test]
fn dates_order_by_timestamp() {
    assert!(compare("createdAt", "Date").contains("a.createdAt.getTime() < b.createdAt.getTime()"));
}

#[test]
fn an_unregistered_object_orders_structurally() {
    assert_eq!(
        compare("user", "User"),
        "__mf_structuralCompare(a.user, b.user)"
    );
}

#[test]
fn an_unordered_element_makes_arrays_unordered() {
    let result = compare("items", "Item[]");
    assert!(
        result.contains("__mf_structuralCompare(__item0, __other0)"),
        "{result}"
    );
    assert!(
        result.contains("if (__order0 !== 0) return __order0;"),
        "{result}"
    );
}

#[test]
fn string_array_elements_compare_as_strings() {
    let result = compare("tags", "readonly string[]");
    assert!(result.contains("__item0 < __other0"), "{result}");
}

#[test]
fn a_union_ending_in_an_array_is_not_an_array() {
    assert_eq!(
        compare("value", "string | number[]"),
        "__mf_structuralCompare(a.value, b.value)"
    );
}
