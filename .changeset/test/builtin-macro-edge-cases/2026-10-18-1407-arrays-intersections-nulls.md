# fix(core): read an array of objects, an intersection and a union of nulls as they are

- **Date:** 2026-10-18 14:07
- **Commit:** 8b37b066
- **Area:** `macroforge_ts` built-in derives

## Problem

Default gave a field typed by an array of objects written in place,
`{ "key with space"?: string }[]`, the default `{}`, which tsc refuses for an array. Clone copied a
field typed `{ a: string } & bigint[]` as an array, which drops what the intersection adds. Encode
and Decode refused `null | null` with a message naming the empty type, ``, and read
`(User | null) | (null | true)[]` as a type no coder reads.

## Cause

The default writer took a type that opens with a brace for an object written in place,
`array_element_type` took an intersection that ends in `[]` for an array, and
`TypeCategory::classify` took the `null` and `undefined` out of a union's own members only, leaving
none for `null | null` and keeping a grouped union's.

## Introduced

`dafd08ef` (2025-12-07, "Add Serialize and Deserialize derive macros for JSON support") classified a
union's `null` and `undefined` from its own members; `1cc2f3c4` (2026-10-08, "feat(core)!: release
0.5.0 with Rust-shaped derives and shared foreign handlers") added `array_element_type`; `3df19fb7`
(2026-10-12, "fix(core): default a Required, a Pick and an object type written in place by their
fields") read an object by its first brace.

## Fix

An object is read where its braces are around all of the type, an intersection is no array, and a
union is classified over all of its members, a grouped union's among them; a union of nothing but
`null` is `null`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose tests pin
`array_element_type` over an intersection, the default of an array of objects, and the nesting test,
whose grouped nullable number now decodes as `number | null`; `pixi run test:conformance:update` (10
passed), with the regenerated snapshots reviewed by the lines that changed; and the `edge-scanned`,
`exact-types` and `strict-lib` testground suites, whose `ScannedPicks` defaults an array of objects
to `[]`, `ScannedTagged` clones and compares an intersection ending in an array, and
`ScannedNothing` decodes a `null | null` field. Found by type-checking a corpus of 200 generated
field types.
