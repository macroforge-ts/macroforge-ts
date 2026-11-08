# fix(endec): read a nested type once a level when decoding

- **Date:** 2026-10-13 09:30
- **Commit:** 61502858
- **Area:** Built-in Decode derive

## Problem

`@derive(Decode)` took time that doubled with each level a field's type nests: 0.24s for `number[]`
ten levels deep, 3.1s at fourteen, and no end at thirty-two, where an expansion was stopped after 65
minutes. The code generated grows by a few bytes a level.

## Cause

For an array, a record, a tuple, an object type written in place and a generic type's arguments, the
coder first asked whether an element decodes as itself, by building its decoder and looking whether
one came back, and then built the decoder again to use it. Each level read the level below twice.

## Introduced

2a0d26f5 (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on this
branch, added the coder with the question and the build as two calls.

## Fix

The decoder of an element is built once and is `None` for an element that decodes as itself, which
answers the question. The generated code is the same; where a record's keys are checked, two imports
change places.

## Verified

Timed `Decode` on `number[]` nested 32 deep: 0.07s, where it had not finished in 65 minutes. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which one import line of `keyed-shapes` changed order and no other generated line
changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
