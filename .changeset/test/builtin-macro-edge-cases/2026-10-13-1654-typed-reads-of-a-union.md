# fix(endec): read a tag and a member off a value without `any`

- **Date:** 2026-10-13 16:54
- **Commit:** 6469899d
- **Area:** Union encoders and decoders, flattened fields, variant guards

## Problem

The generated code of a union cast its value to `any` to read a tag, a content field or a variant
off it, to ask whether a literal is one of the union's, and to take an encoded member apart:

```ts
const __typeName = (value as any)['kind'];
if (allowedLiterals.includes(value as any)) return true;
const { __type: __typeName, __id: __idVal, ...fields } = __variant as any;
```

Through `any` the checker says nothing of what follows: an id that is no number was handed on as
one, and a tag compared as whatever it was.

## Cause

The templates were written against `unknown` input and reached past it with a cast at each read.
Sixty-two casts and `any` annotations were emitted in all.

## Introduced

`0a881898` (2025-12-13, "Improve deserialization to support field validations on interfaces and type
aliases") wrote the union decoder's reads through `any`, and the templates after it kept the form.

## Fix

Forty-six of them are gone. A member is read with the runtime's `member(value, key)`, which gives
`unknown` and reads what the value inherits as the cast did, so a tag that is a getter of a class
still tells its member. A literal is found with `some`. An encoded member is taken apart as a
`Record<string, unknown>`, and its id is handed on only where it is a number. A variant guard
extracts by `{ key: unknown }`.

Sixteen remain, each needing the type of a member at hand: the union encoder's calls of a member's
encoder and its reads of an externally tagged payload, and the value a decoder builds field by
field. They are listed for the next change.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which every changed line is a cast become a typed read and the import of `member`,
and the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which `exact-types`, `strict-lib` and `tsc` type-check the result on a rebuilt
engine and `endec`, `edge-variants`, `intersection-types` and `stamped-e2e` run it.
