//! Reading the grouping and the tuples of a type's text.

use crate::builtin::derive::common::{tuple_body, tuple_element, ungrouped};

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
