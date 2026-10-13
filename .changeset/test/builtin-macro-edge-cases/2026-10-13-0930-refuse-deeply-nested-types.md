# fix(core)!: refuse a type nested past sixteen levels

- **Date:** 2026-10-13 09:30
- **Commit:** 601d38b0
- **Area:** Expansion host, every built-in derive

## Problem

A built-in derive on a field or an alias nested some hundreds of levels deep, as an array of arrays,
a generic, an object type, a tuple or a nullable union, aborted the expander with a stack overflow:
no diagnostic, exit status 134.

## Cause

Every generator that walks a type recurses once for each level the type nests, and nothing bounded
how deep. The stack a level takes is large: about 70KB in a debug build for an object type or a
tuple written in place, so a 2MB thread holds 28 such levels and runs out at 32.

## Introduced

No single change: the generators have recursed over a type's nesting without a bound since they were
written. 5a7366fa (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on
this branch, added the coder that takes the most stack a level, which the limit is measured against.

## Fix

A built-in derive on a type that nests more than sixteen levels deep is an expansion error naming
the field or the alias and how deep it nests, with the advice to name its inner types with aliases.
Depth counts the brackets of every kind open at the deepest point and the `[]` suffixes of an array
of arrays; brackets in a string literal and the `>` of an arrow do not count. Sixteen levels take
about half the stack of a 2MB thread.

Breaking: a type nested more than sixteen levels deep, which expanded before when it was shallow
enough for the stack, is now an error.

## Verified

Measured on a 2MB thread how deep each shape codes before the stack runs out: an object type and a
tuple written in place to 28 levels, an array, a record and a nullable union to 40. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_type_nested_to_the_limit_is_coded_a_level_at_a_time` codes each shape at the limit on a test
thread, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by
the lines that changed with the `type_nesting_limit` fixture at 17 and at 16 levels, and the three
groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which the `exact-types`, `tsc` and `edge-exotics` suites expand, type-check and
run a type nested 16 deep through the WebAssembly build.
