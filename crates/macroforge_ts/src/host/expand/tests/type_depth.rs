//! How deep a type nests, as the limit on what a built-in derive reads counts it.

use crate::host::expand::derive_scope::type_depth;

#[test]
fn a_type_nests_by_its_brackets_and_its_array_suffixes() {
    assert_eq!(type_depth("number"), 0);
    assert_eq!(type_depth("number[]"), 1);
    assert_eq!(type_depth("number[][][]"), 3);
    assert_eq!(type_depth("Array<Array<number>>"), 2);
    assert_eq!(type_depth("Map<string, Set<number[]>>"), 3);
    assert_eq!(type_depth("{ a: { b: [number, string] } }"), 3);
    assert_eq!(type_depth("((number | null) | null)[]"), 2);
    // Side by side is not deeper.
    assert_eq!(type_depth("[number[], string[], Date[]]"), 2);
    // A bracket in a literal's text is text, and an arrow's `>` closes nothing.
    assert_eq!(type_depth("'[[[[' | \"((((\""), 0);
    assert_eq!(type_depth("(a: string) => Array<number>"), 1);
}

/// A rest element that other elements follow is found wherever its tuple is
/// written, and a rest that ends its tuple is none.
#[test]
fn a_tuple_whose_rest_is_not_its_last_is_found() {
    use crate::host::expand::derive_scope::tuple_with_early_rest;

    for (ts_type, tuple) in [
        (
            "[string, ...number[], boolean]",
            "[string, ...number[], boolean]",
        ),
        ("readonly [...string[], number]", "[...string[], number]"),
        (
            "Map<string, [first: string, ...rest: number[], last: Date]>",
            "[first: string, ...rest: number[], last: Date]",
        ),
        (
            "type Middle = [a: string, ...b: [number, string][], c: boolean];",
            "[a: string, ...b: [number, string][], c: boolean]",
        ),
    ] {
        assert_eq!(tuple_with_early_rest(ts_type), Some(tuple), "{ts_type}");
    }
    for ts_type in [
        "[string, ...number[]]",
        "[...string[]]",
        "[string, number?]",
        "Config['a, ...b, c']",
        "Record<'[x, ...y, z]', number>",
        "string[]",
        "[[string, ...number[]], boolean]",
        "Lookup[Key][]",
    ] {
        assert_eq!(tuple_with_early_rest(ts_type), None, "{ts_type}");
    }
}
