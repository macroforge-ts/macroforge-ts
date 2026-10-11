# fix(endec): encode and decode a nullable collection's elements

- **Date:** 2026-10-11 07:22
- **Commit:** e0b8b258
- **Area:** Built-in Encode and Decode derives

## Problem

A field typed `Array<T> | null`, or a nullable map, set or record, kept its elements as they were in
both directions. Decoded, a nested value stayed raw JSON, so a record id inside it was text and a
call such as `.equals` on it threw. Encoded, the elements reached `JSON.stringify` unencoded.

## Cause

The nullable case handled a primitive, a date and a single nested type as the non-null member. A
collection there fell through to the branch that assigns the value whole.

## Introduced

dafd08ef (2025-12-07), `Add Serialize and Deserialize derive macros for JSON support`. It wrote the
nullable case as a pass-through, and the cases added to it since never covered a collection.

## Fix

A nullable collection is prepared as a field of its own, the collection once null is ruled out, and
encoded or decoded by that category's existing code. A field with its own `encodeWith` or
`decodeWith` is left to that function.

## Verified

New test `test_nullable_collection_goes_through_its_element_coders`; the endec tests pass (82).
`pixi run test:rust` passed and clippy is clean. Six testground goldens moved and were read: each
lost only its pass-through assignments and gained the element coders and their imports. With them
recorded, `pixi run test:conformance` passed, 10 of 10. In dealdraft, where `Resource.techs` is such
a field, the calendar page that threw on it loads: its `user` e2e module passed against types
rebuilt with this CLI.
