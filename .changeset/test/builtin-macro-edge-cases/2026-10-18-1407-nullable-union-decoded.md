# fix(endec): decode a nullable union through what tells its members apart

- **Date:** 2026-10-18 14:07
- **Commit:** 21362c40
- **Area:** `macroforge_ts` Decode

## Problem

A field typed by a union with `null`, as `Circle | Square | null` or
`User | null | (null | true)[]`, was decoded by taking any value other than `null` as it stood: no
member's decoder ran and no check that it is a member. Encode refused the same field, and Decode
refused the union without its `null`.

## Cause

The field decoder sent a union to `decode_expr`, which tells its members apart or refuses it, only
when the field's type was the union itself; a `null` or an `undefined` around it took the field to
the branch of a nullable type, which carries a value it has no decoder for as it is.

## Introduced

`662c47a8` (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and shared values
left uncoded") sent a union to `decode_expr` only when it was the field's whole type.

## Fix

A union under a `null` or an `undefined` is decoded as the union is, and the field-level guard of a
nullable union, which nothing reaches any more, is removed.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed);
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which `endec_inline_union` refuses `Circle | Square | null` in Decode as Encode
does, and `Circle | null | (null | true)[]` grouped or not, and the Svelte and generic edge cases
decode their nullable unions of a guarded member through the same guard as before; and the
`edge-scanned`, `exact-types` and `strict-lib` testground suites. Found by probing a union the
corpus refused with a misleading message, whose form without the grouping Decode took raw.
