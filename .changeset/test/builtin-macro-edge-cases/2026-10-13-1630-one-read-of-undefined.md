# fix(endec): read a collection's undefined back once

- **Date:** 2026-10-13 16:30
- **Commit:** 00b84957
- **Area:** Decode of collection elements

## Problem

An element that may be `undefined` is `null` once encoded, and its decoder turned the `null` back at
every place it used the value:

```ts
((
    __raw0
) => (__mf_decodeChecked(
    __raw0 ?? undefined,
    (typeof (__raw0 ?? undefined) === 'number' &&
        globalThis.Number.isFinite(__raw0 ?? undefined)) ||
        (__raw0 ?? undefined) === undefined,
    'number | undefined'
) as number | undefined));
```

## Cause

The decoder of the element was written over the expression `(__raw0 ?? undefined)` in place of the
name of the value, so the expression was repeated wherever the name would have been.

## Introduced

`f2aaff55` (2026-10-13, "fix(endec): bring an undefined held in a collection back as one"), on this
branch, added the read as an expression put in place of the value's name.

## Fix

The callback binds the value once, `const __read0 = __raw0 ?? undefined;`, and the decoder is
written over the name.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`an_undefined_in_a_collection_is_coded_as_null` expects one read a decoder,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-specials` round-trips nine values holding `undefined`.
