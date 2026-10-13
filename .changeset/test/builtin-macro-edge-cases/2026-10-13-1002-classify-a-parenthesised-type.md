# fix(endec): read a parenthesised type as the type in it

- **Date:** 2026-10-13 10:02
- **Commit:** c8ebd4f5
- **Area:** Built-in Encode and Decode derives, type classification

## Problem

A field written `note: (string | undefined)` did not decode from its own encoding: Encode leaves the
key of an `undefined` out, and Decode answered "missing required field". The same field without the
parentheses round-trips.

## Cause

`TypeCategory` classified a type by its text as written. A union in parentheses has no member at its
top level, so it was not `Optional` or `Nullable`, and Decode held the field's key to be required.

## Introduced

dafd08ef (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") wrote the
classifier over the type's text and never set grouping parentheses aside.

## Fix

The classifier reads the type inside the parentheses that only group it. The parentheses of an
array's element type, `(A | B)[]`, group the element and are left as they are.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`test_type_category_reads_a_grouped_type_as_the_type_in_it` covers one and two pairs of parentheses
and an array's element, and the three groups on the branch's final tree: `pixi run test:rust` (the
workspace and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13
validator files of 638 steps and 236 browser tests in Chromium, all passed) and
`pixi run test:packages` (the six packages passed), in which `edge-specials` round-trips a field
typed `(string | undefined)` holding `undefined`.
