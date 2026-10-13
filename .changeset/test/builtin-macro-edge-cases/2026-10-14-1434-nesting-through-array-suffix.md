# fix(core): count a type's nesting through what an array suffix wraps

- **Date:** 2026-10-14 14:34
- **Commit:** c24e999a
- **Area:** `macroforge_ts` expansion host

## Problem

The nesting limit counted `{ a: X[] }[]` as deep as `{ a: X[] }`, so a type of objects in arrays
nested close to twice the limit was let through to a derive. A string ending in an escaped
backslash, as in `'\\'`, ran the scan of the nesting limit and of a tuple's rest element on as
string text.

## Cause

`type_depth` added a `[]` to the brackets open where it stood, not to the depth of the type it
follows, and both scans kept their own string state with "the letter before is no backslash" as
their escape rule.

## Introduced

`601d38b0` (2026-10-13, "fix(core)!: refuse a type nested past sixteen levels") added `type_depth`;
`38729400` (2026-10-13, "fix(core)!: refuse a tuple whose rest element is not its last") added the
tuple scan.

## Fix

`type_depth` keeps, for each bracket open, the depth of the type last written in it, which a `[]`
wraps, and both scans read strings with the shared `scan::Quoting`. A parenthesized union in an
array counts its parentheses as the other brackets count theirs.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose properties hold
`type_depth` and the early-rest scan to the depth and the tuples of generated types, and whose tests
pin `{ a: { b: string[] }[] }[]` at 5 and a string of a lone backslash at no depth.
