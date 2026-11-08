# fix(core): read [] as the tuple of no elements

- **Date:** 2026-10-13 23:13
- **Commit:** 6dc27177
- **Area:** Clone and Decode of tuples

## Problem

```ts
/** @derive(Clone, Decode) */
export interface Holder {
    none: [];
    pair: [unknown, unknown];
}
```

Clone read `[]` as an array whose element type is empty and mapped it into `never[]`, which is no
`[]`. Decode let any array through for `none` and `pair`, of any length.

## Cause

`array_element_type` read `[]` as an array of nothing, and the tuple decoder skipped the length
check when no element of the tuple needed decoding.

## Introduced

`array_element_type` came with `1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped
derives and shared foreign handlers"); the decoder that skips a tuple came with `2a0d26f5`
(2026-10-12, "fix(endec)!: code a value by the whole structure of its type").

## Fix

`[]` is the tuple of no elements and is cloned as a tuple. Every tuple is decoded through
`decodeTuple`, which holds the input to the tuple's length.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_tuple_is_coded_element_by_element` decodes `[]` and `[unknown, unknown]` through `decodeTuple`,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `tuple_of_no_elements` fixture, and the three groups on the branch's final
tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-collections` clones, defaults and round-trips `[]`, `[[]]` and `[unknown, string]` fields and
refuses each at the wrong length.
