# fix(endec): leave a date that is none and a number that is not finite to their validators

- **Date:** 2026-10-13 07:03
- **Commit:** d04282bc
- **Area:** Built-in Decode derive

## Problem

A field with a `validDate`, `finite` or `nonNaN` validator no longer reported through it. An invalid
date was reported as a value of the wrong type and `Infinity` as `expected number`, where the
validator's message, `must be a valid date` or the one written beside it with `message`, had been
reported before. So was a newtype alias: decoding `Infinity` as a `Meters` under `@endec(finite)` no
longer said `Meters must be finite`.

## Cause

The field's own type test runs before its validators, and it had been made to refuse an invalid date
and a number that is not finite. A value it refuses never reaches a validator.

## Introduced

Both on this branch: e6845936 (2026-10-12, "fix(endec)!: code what a field's type arguments,
Required and shared values left uncoded") made the date test refuse a `Date` that is none, and
e3719929 (2026-10-12, "fix(endec)!: refuse the numbers and types a value cannot be coded back from")
made the number test refuse what is not finite.

## Fix

The type test leaves to a validator what that validator refuses: an invalid date under `validDate`,
`NaN` and the infinities under `finite`, `NaN` under `nonNaN`. The validator then reports it under
its own message, and the decode fails as before. An infinity under `nonNaN` alone is still refused
by the type test, since `decode` takes finite numbers only.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (734 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line, `pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps
and 236 browser tests in Chromium, all passed), whose validator files had failed on four steps
before the fix, and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
