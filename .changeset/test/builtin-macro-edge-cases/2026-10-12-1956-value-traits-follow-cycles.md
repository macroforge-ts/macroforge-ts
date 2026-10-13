# fix(core): follow a value that holds itself in the value traits

- **Date:** 2026-10-12 19:56
- **Commit:** 25112298
- **Area:** Built-in Debug, Clone, PartialEq, Hash, PartialOrd and Ord derives, structural runtime

## Problem

`Clone`, `PartialEq`, `Hash`, `Debug`, `PartialOrd` and `Ord` on a value that holds itself, a child
pointing back at its parent or two instances naming each other, recursed until the stack overflowed
with a `RangeError`. `Clone` of a value held in two places returned two copies, so the copy did not
have the shape of the original.

## Cause

Each derive generates one function per type, and a field of a derived type calls that type's
function. Nothing was shared between those calls, so none could know a value it was already inside.
The structural runtime kept a record for the values it walks itself and lost it at every call into a
generated function.

## Introduced

1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") gave the derives their per-type functions with no record of the walk.

## Fix

The structural runtime keeps one record per kind of walk, from the outermost call to its return, and
the generated function of a class, an interface or an object alias runs its body through it:
`clonedWith`, `equalWithin`, `comparedWithin`, `hashedWithin` and `debuggedWithin`. A clone makes
the copy before its fields, so a value met again is that copy: a cycle closes on the copy, and a
value held twice is one value in it. A pair already being compared is equal as far as that goes, a
value already being hashed adds nothing, and one already being printed is `[Circular]`.
`structuralClone` shares the clone's record, so a cycle through an untyped field closes too.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-graphs` suite clones, compares, hashes and prints values that hold themselves,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
