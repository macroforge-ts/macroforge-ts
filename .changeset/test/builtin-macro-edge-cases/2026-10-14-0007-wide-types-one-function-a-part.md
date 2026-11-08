# fix(core): give each field and member of a wide type a function of its own

- **Date:** 2026-10-14 00:07
- **Commit:** 4609f9ca
- **Area:** Every derive on wide object types and unions

## Problem

Generated code for wide types did not type-check:

- The decoder of an interface of 250 fields, and its encoder and `partialCompare` at 1000, were "too
  large for control flow analysis" (TS2563).
- `tsc` ran out of stack ("Maximum call stack size exceeded") on the `equals` of 1000 fields.
- Type-checking the decoder of a union of 500 object members did not finish in 90 seconds.

## Cause

`tsc` reads a function body back from each use of a variable to its start, one level of recursion a
condition on the way, and stops at 2000; it walks an `&&` chain with a level a term. The decoder
checked each required key with an `if` of its own and decoded every field in one body, the encoder
wrote every field in one body, `partialCompare` and `compare` ran a condition a field in one body,
`equals` and `hasShape` joined a test a field with `&&`, and the union decoder and `hasShape` tried
the members in one body, each test narrowing the value again after all the ones before it. A
function called where it is written is read as part of the body around it, so wrapping each part in
one would not have helped.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") wrote all of these.

## Fix

Each field is coded, compared and ordered, and each union member decoded and shape-tested, in a
function of its own, run from a list: `equals` with `every`, `partialCompare` and `compare` by the
first order that is not `0`, the union decoder through `decodedMember`. `hasShape` asks `holdsAll`
of the list of required keys, the missing required keys are checked in one loop, and a tuple's
equality is a statement an element.

## Verified

Type-checked each derive alone over interfaces of 100 to 2000 fields and unions of 25 to 500 members
before and after the fix: before it, Decode failed at 250 fields, Encode and the comparisons at
1000, Encode took over four minutes on a union of 250 members and Decode did not finish one of 500;
after it, all of them type-check, the slowest a union of 500 members under Encode in 11 seconds. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`parts_each_run_in_a_function_of_their_own` covers the list the parts run from,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which the snapshots changed in the lines that wrap each part and read the key
lists, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which `edge-wide` codes, compares, hashes and defaults an interface of 300
fields and two unions of 120 members, and `exact-types` type-checks them.
