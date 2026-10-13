# fix(core)!: hold NaN and an invalid date equal to themselves

- **Date:** 2026-10-12 21:45
- **Commit:** 2217dc42
- **Area:** Built-in PartialEq, PartialOrd and Ord derives, structural runtime

## Problem

A value with a `NaN` in a number field, or an invalid `Date`, was unequal to its own clone:
`PartialEq` returned `false` for the pair, while `Hash` gave both the same hash and a `Set` held
them as one. `PartialOrd` returned `null` for the pair, and `Ord` returned `0` for `NaN` against any
number, so a sort left a list with a `NaN` in whatever order it came in.

## Cause

Numbers and the times of dates were compared with `===` and `<`, and `NaN` is the one number those
hold unequal to itself and unordered against every other.

## Introduced

1b9ff6a1 (2025-12-08, "Add built-in derive macros for PartialEq, Hash, Ord, PartialOrd, and
Default") compared number fields and `getTime()` with `===`. 1cc2f3c4 (2026-10-08, "feat(core)!:
release 0.5.0 with Rust-shaped derives and shared foreign handlers") carried the same comparison
into the structural runtime and the shared ordering helpers.

## Fix

Two numbers are equal when `===` holds or both are `NaN`, in the generated code and in the
structural runtime, and a date is compared by its time the same way. `PartialOrd` holds `NaN` equal
to `NaN` and unordered (`null`) against a number. `Ord`, which has to order every pair, puts `NaN`
after every number.

`PartialEq` of two `NaN`s is now `true`, and `PartialOrd` of them `0`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-specials` suite clones, compares, orders and hashes values holding `NaN`, the infinities
and an invalid date, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots
reviewed by the shape of each changed line and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
