# fix(endec)!: let is say yes only to a value its decoder takes

- **Date:** 2026-10-14 12:57
- **Commit:** 3d0cedf9
- **Area:** `is` of union aliases, aliases of another type and primitive aliases

## Problem

```ts
/** @derive(Decode) */
export type When = Date;

whenIs('not a date'); // true, and whenDecode('not a date') fails
```

The `is` of a union alias, an alias of another type and a primitive alias was its shape check alone,
which lets through values the decoder refuses, so a value it narrowed could fail to decode and was
not of the type. The `is` of an object type already asked the decoder.

## Cause

The shape check tells apart the members of a union by what a value looks like, and `is` took it for
the whole question; it asked the decoder only when an arm carried validators.

## Introduced

`1cc2f3c4` (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") wrote these guards.

## Fix

Every `is` asks the shape check and then the decoder, so it says yes only to a value that decodes.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which the `is` of every union, alias and primitive alias asks its decoder, and the
three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which `edge-aliases` checks that `is` of a date alias and of a date or `null`
refuses text that is no date.
