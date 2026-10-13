# fix(endec): type a tuple's optional and rest elements as the tuple does

- **Date:** 2026-10-13 23:12
- **Commit:** cfd99fdb
- **Area:** Encode and Decode of tuples with optional or rest elements

## Problem

```ts
/** @derive(Encode, Decode) */
export type Row = [label: string, count?: number, ...flags: boolean[]];
```

The coders of `Row` failed the testground's strict checks, and encoded some values wrongly:

- The rest was read with `value.slice(2)`, typed as an array of every element type of the tuple, so
  an encoder of the rest's element type did not accept it.
- The view's `count?: number` did not take the `undefined` it is assigned under
  `exactOptionalPropertyTypes`, and the decoder wrote `obj.rest` on a record, which
  `noPropertyAccessFromIndexSignature` refuses.
- `['label', undefined, true]` was encoded as `['label', true]`, moving `true` into `count`.
- A tuple field `[string, Item?, ...Date[]]` holding `['head']` was encoded as `['head', null]` and
  decoded one element longer.

## Cause

The tuple coders took the rest of a tuple as a slice, wrote the view's optional member as a plain
optional property, and decided whether an optional element was in the tuple by what its encoder
wrote, which leaves out `undefined`, or not at all.

## Introduced

`5a7366fa` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on the
branch this one is stacked on, wrote these coders.

## Fix

The rest is taken by destructuring (`(([, , ...rest]) => rest)(value)`), which TypeScript types as
the tuple's rest. An optional element is `T | undefined` in the view, and the decoder writes the
rest by its key. An optional element is written when the tuple reaches it, as `null` when it is
`undefined`, so every element keeps its place and the decoded tuple its length.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_tuple_is_coded_element_by_element` pins the encoder of `[User, Date?, ...string[]]`,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `tuple_optional_and_rest` fixture, in which the other snapshots that changed
hold a tuple view and change only in the lines that build the tuple, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-aliases` round-trips `AliasRow` with and without its optional element, with it `undefined`
before the rest and at the end, and decodes `['label', undefined, true]` handed to it as a value,
`edge-collections` round-trips a tuple field that stops before its optional element, and
`exact-types`, `strict-lib` and `tsc` type-check them.
