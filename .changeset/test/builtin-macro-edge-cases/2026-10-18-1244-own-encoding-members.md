# fix(endec): code a union as its own encoding only where each member is one

- **Date:** 2026-10-18 12:44
- **Commit:** 43b25348
- **Area:** `macroforge_ts` Encode and Decode

## Problem

A field typed by a bigint literal, `1n` or `-1n | 1_000n`, was encoded raw, and `JSON.stringify`
threw "Do not know how to serialize a BigInt"; `bigint | number[]` and `bigint | string` did the
same, and Decode took `bigint | string` as text. A union of a primitive and a tuple,
`string | [number, string]`, was refused as a type no coder reads. A union holding
`(number | undefined)[]` was carried as itself, so `[1, undefined]` encoded as `[1, null]` and was
refused on decode.

## Cause

`primitive_accepts` and `untold_member` took a bigint, and a bigint literal, for a value of its own
encoding, which JSON has no form for. `structural_accepts`, which tells the values a union carries
as they are, had no arm for a tuple, and took a collection whose element admits `undefined` for its
own encoding.

## Introduced

`5a7366fa` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type") decoded a
literal or a union of literals by testing the raw value; `662c47a8` (2026-10-12, "fix(endec)!: code
what a field's type arguments, Required and shared values left uncoded") added `untold_member` and
`structural_accepts`.

## Fix

A bigint literal is coded as a bigint is, through its text, and decoded to a bigint checked to be
one of the literals, nullable or not. A union holding a bigint, a bigint literal or a collection
whose element admits `undefined` holds a coded member, which a field tells only through an alias.
`structural_accepts` reads a tuple by its length and each element.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed);
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which `literal_number_forms` encodes `1n | -2n` through its text where it had
written the value raw, and `endec_inline_union` refuses `bigint | number[]`, `bigint | string`,
`1n | 'one'`, `string | (number | undefined)[]` and `string | [number | undefined]` in both coders;
and the `edge-scanned` testground suite, which round-trips `string | [number, string?]`,
`string | ['a', ...number[]]` and bigint literal fields, one nullable, and refuses wrong lengths and
other bigints. Found by type-checking and running a corpus of 120 generated field types, where
`bigint | ['a', ...number[]]` threw in `JSON.stringify`.
