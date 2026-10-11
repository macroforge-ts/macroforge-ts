# fix(endec): code an undefined union's member and a nullable wrapper by their own type

- **Date:** 2026-10-11 07:44
- **Commit:** 78d6bae6
- **Area:** Built-in Encode and Decode derives

## Problem

A field typed `T | undefined` decoded any `T` that is not a primitive as raw JSON: a date stayed
text, and a nested type or a collection went undecoded. Encoded, a collection or wrapper member was
passed through whole. `Partial<T> | null` passed through in both directions.

## Cause

The decoder had no case for an undefined union, so it reached the fallback that assigns the value.
The encoder's case handled a primitive, a date and a single nested type. The nullable case prepared
its member as a field of its own for a collection only.

## Introduced

dafd08ef (2025-12-07), `Add Serialize and Deserialize derive macros for JSON support`. It decoded an
undefined union by assigning the value, and encoded one through `toJSON` where the value had it.

## Fix

The member a union's own cases do not handle is prepared as a field of its own and coded by its
category, for an undefined union as well as a nullable one, and for a wrapper as well as a
collection. A primitive member keeps the union's own check.

## Verified

New test `test_undefined_union_goes_through_its_member_coders`; the endec tests pass (84).
`pixi run test:rust` passed, clippy is clean, and no golden moved for this change.
`pixi run test:testground` passed with both decoder changes: 22 Deno suites, the validator tests and
126 browser tests.
