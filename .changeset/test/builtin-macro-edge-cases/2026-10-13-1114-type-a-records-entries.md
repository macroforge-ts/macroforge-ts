# fix(endec): say what the entries of a record hold

- **Date:** 2026-10-13 11:14
- **Commit:** 5e4feaa5
- **Area:** Built-in Encode derive, records

## Problem

Under `libReplacement` with a stricter `lib`, the Encode of a record over named keys whose values
are coded did not type-check: for `finite: Record<'a' | 'b', Date | undefined>`, "'__item0' is
possibly 'null'" and "Property 'toISOString' does not exist on type '{}'".

## Cause

The encoder maps over `Object.entries(value)`. The stock `lib` types each entry's value as the
record's value type; a stricter one types it `unknown` unless the record's key type is `string` or
`number`, since an object may hold more keys than its type names. The encoder relied on the stock
typing.

## Introduced

2a0d26f5 (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on this
branch, wrote the record's encoder as a map over `Object.entries`. No type of the testground held a
record over named keys with values that are coded, so the strict-lib suite did not meet it until one
was added.

## Fix

The encoder states what it maps over: `(Object.entries(value) as [string, V][])`, with `V` the
record's value type.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `strict-lib` type-checks the `finite` field of `SpecialHoles` after
failing on it.
