use crate::builtin::derive::common::{OrdField, Ordering, generate_field_order};
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;

fn compare(name: &str, ts_type: &str) -> String {
    let field = OrdField {
        name: name.to_string(),
        ts_type: ts_type.to_string(),
        optional: false,
    };
    generate_field_order(
        Ordering::Total,
        &field,
        "a",
        "b",
        None,
        &TypeRegistry::default(),
    )
}

#[test]
fn ord_on_a_class_emits_a_static_compare() {
    let source = "/** @derive(PartialEq, Eq, PartialOrd, Ord) */\nexport class Version {\n  major: number = 1;\n}\n";
    let result =
        crate::expand_core::expand_inner(source, "probe.ts", None).expect("source should expand");
    assert!(result.diagnostics.is_empty(), "{:?}", result.diagnostics);
    assert!(
        result
            .code
            .contains("static compare(a: Version, b: Version): number {"),
        "{}",
        result.code
    );
    assert!(!result.code.contains("compareTo"), "{}", result.code);
}

#[test]
fn ord_without_eq_and_partial_ord_is_an_error() {
    let source =
        "/** @derive(PartialEq, Ord) */\nexport class Version {\n  major: number = 1;\n}\n";
    let result =
        crate::expand_core::expand_inner(source, "probe.ts", None).expect("source should expand");
    assert!(
        result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.level == "error"
                && diagnostic.message
                    == "`Ord` requires `Eq` and `PartialOrd`, which `Version` does not derive"),
        "{:?}",
        result.diagnostics
    );
}

#[test]
fn test_field_compare_number() {
    let result = compare("id", "number");
    assert!(result.contains("a.id < b.id"));
    assert!(result.contains("a.id > b.id"));
    assert!(!result.contains("null")); // Total ordering - no null
}

#[test]
fn strings_order_by_code_units_like_equality() {
    // `localeCompare` can call distinct strings equal, which `===` does not.
    assert_eq!(
        compare("name", "string"),
        "(a.name < b.name ? -1 : a.name > b.name ? 1 : 0)"
    );
}

#[test]
fn an_unregistered_object_orders_structurally() {
    assert_eq!(
        compare("user", "User"),
        "__mf_structuralCompare(a.user, b.user)"
    );
}

#[test]
fn an_optional_field_orders_absent_values_structurally() {
    let field = OrdField {
        name: "at".to_string(),
        ts_type: "Date".to_string(),
        optional: true,
    };
    let result = generate_field_order(
        Ordering::Total,
        &field,
        "a",
        "b",
        None,
        &TypeRegistry::default(),
    );
    assert!(
        result.starts_with("(a.at == null || b.at == null ? __mf_structuralCompare(a.at, b.at) :"),
        "{result}"
    );
}

#[test]
fn array_elements_compare_by_their_own_type() {
    let strings = compare("tags", "Array<string>");
    assert!(strings.contains("__item0 < __other0"), "{strings}");

    let dates = compare("seen", "Date[]");
    assert!(
        dates.contains("__item0.getTime() < __other0.getTime()"),
        "{dates}"
    );

    let nested = compare("grid", "number[][]");
    assert!(nested.contains("for (let __index0 = 0;"), "{nested}");
    assert!(nested.contains("for (let __index1 = 0;"), "{nested}");
    assert!(nested.contains("const __left1 = __item0;"), "{nested}");
}
