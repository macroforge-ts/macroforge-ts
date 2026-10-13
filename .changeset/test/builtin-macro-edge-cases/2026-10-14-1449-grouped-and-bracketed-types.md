# fix(core): read a type in parentheses, or holding brackets in a string, as written

- **Date:** 2026-10-14 14:49
- **Commit:** 6016e368
- **Area:** `macroforge_ts` built-in derives

## Problem

Decode refused a field typed `(Shade)` as a type the file neither declares nor imports, Encode and
Decode refused `(string) | Record<string, ''>` as a type no coder reads, and a tuple holding the
literal `']?)'` was no tuple to the coders, which refused it, while Default wrote a default for it
that does not parse.

## Cause

`ungrouped`, `tuple_body` and `tuple_element` scanned a type's brackets and colons with no regard
for strings, and the check for an undeclared name (`TypeScope::nested`) and `structural_accepts`
read a type without undoing its grouping.

## Introduced

`5a7366fa` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type") added the
bracket count of `ungrouped`; `bb74741f` (2026-10-12, "fix(core): make the value traits and Default
reach every field of a type") added `tuple_body`; `2aabc445` (2026-10-08, "feat(core)!: add $Newtype
brands and check primitive values on decode") added `tuple_element`; `662c47a8` (2026-10-12,
"fix(endec)!: code what a field's type arguments, Required and shared values left uncoded") added
`structural_accepts`; `ec81c97a` (2026-10-14, "fix(endec): refuse a field typed by a name the file
neither declares nor imports") added the check of a name in `nested`.

## Fix

The readers share `scan::unwrapped` and `scan::top_level_letters`, which skip strings, and both
entries undo a type's grouping first.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose test pins
`ungrouped`, `tuple_body` and `tuple_element` over strings holding brackets, colons and arrows;
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed; and the `edge-scanned` testground suite, whose `ScannedGroups` defaults, decodes and
compares a `(ScannedShade)`, a `(string) | Record<string, ''>` and a `[']?)', 4?]`. Found by
expanding a corpus of 60 generated field types under each derive set with `macroforge cache` in the
vanilla testground, where these were the refusals of types a coder reads.
