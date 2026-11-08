# fix(endec): report a validator whose arguments never close

- **Date:** 2026-10-13 09:30
- **Commit:** db26fadc
- **Area:** Endec validators

## Problem

`@endec({ validate: [")("] })` panicked the expander: "slice index starts at 2 but ends at 0". A
validator with text after its arguments, `"minLength(2) or so"`, was read as `minLength(2`.

## Cause

A validator's arguments were sliced from its first `(` to its last `)`, wherever in the text those
stood. With the last `)` before the first `(` the slice ran backwards.

## Introduced

bb3be3c2 (2025-12-08, "Support enums and type aliases in all built-in macros") parsed a validator by
the positions of its first `(` and last `)`.

## Fix

The arguments are what stands between the first `(` and a `)` that ends the validator. One that does
not end in `)` is "invalid arguments for 'minLength': missing closing parenthesis", and one with no
name before its `(` is an unknown validator.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_validator_whose_arguments_never_close_is_an_error` covers the three spellings,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `reversed` field of the `endec_option_values` fixture, and the three groups on
the branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
