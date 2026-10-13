# fix(endec): code a member tagged over a type parameter by the coders handed in

- **Date:** 2026-10-14 12:57
- **Commit:** 7ea66595
- **Area:** Encode and Decode of unions with a member `{ tag: 'X' } & T`

## Problem

```ts
/** @derive(Encode, Decode) */
export type Tagged<T> = ({ __type: 'A' } & T) | { __type: 'B'; size: number };
```

The encoder of `Tagged` called `tEncodeWithContext`, which does not exist, the decoder returned the
member as it came, checking nothing, and the guard `taggedAIs` named `T` without declaring it.

## Cause

A member tagged in place was coded through the coder of the type it names, which a type parameter
does not have, and the decoder fell back to the input for one. The per-member guards took no type
parameters.

## Introduced

`662c47a8` (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and shared values
left uncoded"), on the branch this one is stacked on, handed a generic union its parameters' coders
without the tagged members using them.

## Fix

A type parameter's member is encoded and decoded by the coder handed in for it, as the parameter is
everywhere else. The guard of a member of a generic union declares its type parameters and takes a
value of the union, as a tag tells apart its members and says nothing of the type arguments.

## Verified

Expanded and type-checked a union with a member `{ __type: 'A' } & T` before the fix: it called
`tEncodeWithContext` and its guard named an undeclared `T`. Ran
`cargo test -p macroforge_ts --features test-macros --lib` (776 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-generics` encodes and decodes `GenericTagged` through coders
handed in for its parameter, and `exact-types` type-checks it.
