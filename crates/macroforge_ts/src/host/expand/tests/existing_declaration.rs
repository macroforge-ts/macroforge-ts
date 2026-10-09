use super::super::helpers::has_existing_namespace_or_const;

#[test]
fn a_longer_name_first_does_not_hide_the_declaration() {
    let source = "export const FooBar = 1;\nexport const Foo = { a: 1 };\n";
    assert!(has_existing_namespace_or_const(source, "Foo"));
}

#[test]
fn namespaces_and_typed_consts_count() {
    assert!(has_existing_namespace_or_const("namespace Foo {}", "Foo"));
    assert!(has_existing_namespace_or_const(
        "const Foo: Bar = x;",
        "Foo"
    ));
    assert!(has_existing_namespace_or_const(
        "export const\n  Foo = 1;",
        "Foo"
    ));
}

#[test]
fn other_uses_of_the_name_do_not() {
    assert!(!has_existing_namespace_or_const("interface Foo {}", "Foo"));
    assert!(!has_existing_namespace_or_const("let x: Foo = y;", "Foo"));
    assert!(!has_existing_namespace_or_const("myconst Foo = 1;", "Foo"));
    assert!(!has_existing_namespace_or_const("const FooBar = 1;", "Foo"));
}
