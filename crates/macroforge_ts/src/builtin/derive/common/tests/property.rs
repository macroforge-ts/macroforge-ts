//! Property names that are not identifiers, tuple types and the defaults
//! that depend on them.

use crate::builtin::derive::common::{
    Member, get_type_default, local_name_part, property_key, tuple_body,
};
use crate::ts_syn::ToTsString;

/// `source` without its whitespace, so a comparison does not hang on spacing.
fn squeezed(source: &str) -> String {
    source.split_whitespace().collect()
}

#[test]
fn an_identifier_follows_a_dot() {
    for name in ["name", "class", "$dollar", "_under", "a1"] {
        assert_eq!(Member::of(name).to_ts_string(), format!(".{name}"));
        assert_eq!(property_key(name), name);
    }
}

#[test]
fn any_other_name_is_quoted() {
    assert_eq!(Member::of("first-name").to_ts_string(), r#"["first-name"]"#);
    assert_eq!(Member::of("0").to_ts_string(), r#"["0"]"#);
    assert_eq!(Member::of("with space").to_ts_string(), r#"["with space"]"#);
    assert_eq!(Member::of(r#"a"b\c"#).to_ts_string(), r#"["a\"b\\c"]"#);
    assert_eq!(Member::of("ünïcode").to_ts_string(), r#"["ünïcode"]"#);
    assert_eq!(Member::of("").to_ts_string(), r#"[""]"#);
    assert_eq!(property_key("first-name"), r#""first-name""#);
    assert_eq!(property_key("1st"), r#""1st""#);
}

#[test]
fn a_local_is_named_apart_for_every_field_name() {
    assert_eq!(local_name_part("name"), "name");
    assert_eq!(local_name_part("first-name"), "first$2dname");
    assert_ne!(local_name_part("a-b"), local_name_part("a_b"));
    assert_ne!(local_name_part("a b"), local_name_part("a.b"));
    assert_ne!(local_name_part("0"), local_name_part("_0"));
}

#[test]
fn a_tuple_is_the_brackets_around_its_elements() {
    assert_eq!(tuple_body("[A, B]"), Some("A, B"));
    assert_eq!(tuple_body("readonly [A, B]"), Some("A, B"));
    assert_eq!(tuple_body("[[A, B], C[]]"), Some("[A, B], C[]"));
    assert_eq!(tuple_body("[]"), Some(""));
    assert_eq!(tuple_body("[A, B][]"), None);
    assert_eq!(tuple_body("A[]"), None);
    assert_eq!(tuple_body("Array<[A, B]>"), None);
}

#[test]
fn a_readonly_tuple_defaults_as_a_tuple() {
    assert_eq!(
        squeezed(&get_type_default("readonly [number, string]")),
        r#"[0,""]"#
    );
    assert_eq!(get_type_default("readonly number[]"), "[]");
    assert_eq!(get_type_default("[string, number][]"), "[]");
    assert_eq!(
        squeezed(&get_type_default("[[number, number], string | null]")),
        "[[0,0],null]"
    );
}

#[test]
fn a_record_over_literal_keys_defaults_to_each_key() {
    assert_eq!(
        squeezed(&get_type_default("Record<'a' | 'b', number>")),
        "({'a':0,'b':0})"
    );
    assert_eq!(
        squeezed(&get_type_default("Record<1 | 2, string>")),
        r#"({1:"",2:""})"#
    );
    assert_eq!(get_type_default("Record<string, number>"), "({})");
    assert_eq!(get_type_default("Record<number, string[]>"), "({})");
}

#[test]
fn a_readonly_view_defaults_as_its_type() {
    assert_eq!(get_type_default("Readonly<string[]>"), "[]");
    assert_eq!(get_type_default("Readonly<Point>"), "pointDefaultValue()");
}
