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
