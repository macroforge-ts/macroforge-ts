# fix(endec): report a collection field whose value is not that collection

- **Date:** 2026-10-11 07:44
- **Commit:** 1c71df91
- **Area:** Built-in Decode derive

## Problem

Decoding an object whose array, set, map or record field held a value of another type succeeded. The
field was left unset, so the caller held an object missing a field its type requires, and nothing
said so.

## Cause

Each collection decoder tested the value's shape and decoded it when it matched, with no branch for
a value that did not.

## Introduced

dafd08ef (2025-12-07), `Add Serialize and Deserialize derive macros for JSON support`. It wrote the
array case as an `Array.isArray` test with nothing after it, and the set, map and record cases
followed that shape.

## Fix

A value of another type is reported under the field, with the message a primitive of the wrong type
already gets: required when it is missing from a field that needs it, the expected type otherwise.

## Verified

New test `test_collection_decoder_reports_a_value_of_the_wrong_type`; the endec tests pass (84).
`pixi run test:rust` passed and clippy is clean. Thirty goldens moved and their diff was summarized:
98 added error branches and no removed line. With them recorded, `pixi run test:conformance` passed,
10 of 10. `pixi run test:testground` passed with both decoder changes: 22 Deno suites, the validator
tests and 126 browser tests.
