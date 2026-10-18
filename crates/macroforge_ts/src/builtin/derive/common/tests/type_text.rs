//! Reading the grouping and the tuples of a type's text.

use crate::builtin::derive::common::{
    array_element_type, get_type_default, parse_generic_type, tuple_body, tuple_element, ungrouped,
};

/// A string holding a bracket, a colon or an arrow is text to the readers of
/// a type's grouping and of a tuple's elements.
#[test]
fn a_string_in_a_type_is_text_to_its_grouping_and_its_tuple() {
    assert_eq!(ungrouped("((Shade))"), "Shade");
    assert_eq!(ungrouped("(\")\" | \"(\")"), "\")\" | \"(\"");
    assert_eq!(ungrouped("(\"=>\")"), "\"=>\"");
    assert_eq!(ungrouped("(() => A)"), "(() => A)");
    assert_eq!(ungrouped("(a: A) => (B)"), "(a: A) => (B)");
    assert_eq!(tuple_body("[\"]?)\", 4?]"), Some("\"]?)\", 4?"));
    assert_eq!(tuple_body("[A][]"), None);
    let element = tuple_element("\"a?: b\"");
    assert_eq!((element.ts_type, element.optional), ("\"a?: b\"", false));
    let element = tuple_element("name?: { a: string }");
    assert_eq!((element.ts_type, element.optional), ("{ a: string }", true));
}

/// A function's or a conditional's last part keeps a `[]` written after it,
/// and a generic's `<` closes at the end of its text or it is no generic.
#[test]
fn a_type_is_an_array_or_a_generic_only_as_a_whole() {
    assert_eq!(array_element_type("(value: true) => string[]"), None);
    assert_eq!(array_element_type("T extends U ? A : B[]"), None);
    // A function keeps the parentheses that make it one element.
    assert_eq!(
        array_element_type("((value: true) => string)[]"),
        Some("((value: true) => string)")
    );
    assert_eq!(
        array_element_type("ReadonlyArray<(value: X) => string[]>"),
        Some("(value: X) => string[]")
    );
    assert_eq!(
        parse_generic_type("Partial<User> | Array<(value: User) => Date>"),
        None
    );
    assert_eq!(
        parse_generic_type("Map<string, (a: A) => B>"),
        Some(("Map", "string, (a: A) => B"))
    );
    assert_eq!(parse_generic_type("NS.Box<'>'>"), Some(("NS.Box", "'>'")));
    assert_eq!(parse_generic_type("Box<A>[]"), None);
    assert_eq!(array_element_type("{ a: string } & bigint[]"), None);
}

/// A union whose first member is an object written in place defaults to that
/// object, each key it requires in it.
#[test]
fn a_union_of_an_object_defaults_to_the_object() {
    assert_eq!(
        get_type_default("{ \"key with space\": undefined; b?: string } | ReadonlyArray<string>"),
        "({ \"key with space\": undefined })"
    );
    // An array of objects is an array, its first `{` no object around it all.
    assert_eq!(get_type_default("{ \"key with space\"?: string }[]"), "[]");
}
