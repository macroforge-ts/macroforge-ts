# fix(core)!: refuse a tuple whose rest element is not its last

- **Date:** 2026-10-13 18:14
- **Commit:** 38729400
- **Area:** Expansion host

## Problem

`middle: [string, ...number[], boolean]` under `Default` generated `[""]`, which is no value of the
type, and the other derives read the `boolean` as one more of the numbers: compared as one, and
decoded by the number's decoder, which fails for it.

## Cause

Every derive reads a tuple's rest element as all that follows the elements before it. TypeScript
lets elements follow a rest, and nothing told a derive apart from one where none do.

## Introduced

`2aabc445` (2026-10-08, "feat(core)!: add $Newtype brands and check primitive values on decode")
added the reading of tuple elements with a rest that ends the tuple, and `3ca86021` (2026-10-25,
squashed from "fix(core): generate type-correct, working code for eight uncovered shapes") the
default of a tuple by its required prefix.

## Fix

A type with a built-in derive that holds such a tuple, in a field or as its own body, at any depth,
is an error at the derive that shows the tuple and says how to write it. It is found by the pass
that refuses a type nested too deep, over the same text.

**Breaking:** a type holding such a tuple no longer expands.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_tuple_whose_rest_is_not_its_last_is_found` covers four tuples that have one and eight types that
do not, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by
the lines that changed with the `tuple_rest_not_last` fixture, which reports its three and reads the
two whose rest ends them, and in which no snapshot of either testground changed, and the three
groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
