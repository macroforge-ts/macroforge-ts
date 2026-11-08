# fix(endec): read a generic's arguments past the arrows and strings they hold

- **Date:** 2026-10-18 14:17
- **Commit:** 46197146
- **Area:** `macroforge_ts` Encode and Decode

## Problem

A field typed by a `Map` keyed by a function,
`Map<(value: undefined) => (value: Shade) => 7, string>`, was refused as "`Map` is a type this file
neither declares nor imports". A union of two Records, `Record<string, A> | Record<string, B>`, was
classified as one Record, and a Partial or a Pick was read between its first `<` and its last letter
whatever stood after its own `>`.

## Cause

`find_top_level_comma` counted angle brackets alone, so the `>` of an arrow closed one and a comma
in a string or in parentheses split the arguments. The Record, Partial and Pick arms of
`TypeCategory::classify` sliced the text instead of parsing the generic.

## Introduced

`dafd08ef` (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") added
`find_top_level_comma`, counting angle brackets alone; `89cd39fa` (2025-12-17, "refactor(macros):
update serde macro implementations") read a Record by its first and last letters.

## Fix

`find_top_level_comma` finds the comma with `scan::top_level_letters`, and the arms read a generic
with `parse_generic_type`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose test classifies a
Map keyed by a function, a Record keyed by a string holding a comma, a union of Records and a Pick
of a union of keys; `pixi run test:conformance:update` (10 passed), with the regenerated snapshots
reviewed by the lines that changed; and the corpus of 200 generated field types whose run found it,
in which the Map keyed by a function is now refused as a key JSON has no form for.
