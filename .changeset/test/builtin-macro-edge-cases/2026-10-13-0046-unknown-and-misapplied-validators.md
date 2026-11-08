# fix(endec)!: report an unknown validator name and a validator its field cannot take

- **Date:** 2026-10-13 00:46
- **Commit:** c17a7a26
- **Area:** Built-in Decode derive, validator parsing

## Problem

- A name in `@endec(...)` that was neither an option nor a validator was dropped without a word, so
  a misspelled validator, `@endec(emial)`, checked nothing.
- A validator on a field of another kind than it checks, `email` on a `number` or `positive` on a
  `string[]`, generated its check anyway: a regular expression tested against a number, an array
  compared with zero. The code failed to type-check, or passed and reported every value.

## Cause

The shorthand parser read an item as a validator only when it was a known validator's name or was
written as a call, and skipped anything else. Nothing compared a validator with the type of the
field it was written on.

## Introduced

0a881898 (2025-12-13, "Improve deserialization to support field validations on interfaces and type
aliases") wrote the shorthand parser that skips an item it does not know.

## Fix

A name that is neither an option nor a validator is an expansion error, with the validator it is
closest to as a suggestion. Each validator knows what it looks at: text, the length of text or of an
array, a number, the items of an array, a date or a bigint. One written on a field, or on a
primitive alias, whose type is plainly of another kind is an expansion error naming both. A type
whose kind is not written in it, such as a named type, is left to the type checker.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `endec_validator_misuse` fixture, whose snapshot holds an error for
each misplaced validator and none for the fields they fit, and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
