# fix(endec): read the raw value of a field of nothing but undefined as unknown

- **Date:** 2026-10-18 13:24
- **Commit:** b3dfd16d
- **Area:** `macroforge_ts` Decode

## Problem

The decoder of a field `field?: undefined` did not type-check: "Conversion of type '{} | null' to
type 'undefined' may be a mistake".

## Cause

The decoder reads a value it finds there as the field's type, after a test that the value is not
`undefined`, which leaves no value of that type.

## Introduced

`db92fd6c` (2026-03-03, "Fix deserialize codegen for containers, cast types, and validators") added
the raw cast of each field.

## Fix

A field of nothing but `undefined` reads what stands there as `unknown`, which the decoder then
refuses.

## Verified

Ran `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
lines that changed and the `edge-scanned`, `exact-types` and `strict-lib` testground suites, which
decode a `ScannedNothing` without its `none?: undefined` field and refuse one with `none: 1`. Found
by type-checking a corpus of 150 generated field types, where tsc refused the decoder of
`field?: undefined`.
