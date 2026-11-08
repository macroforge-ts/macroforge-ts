# fix(endec)!: check a generic type's parameters in its is guard

- **Date:** 2026-10-14 12:43
- **Commit:** 12597d08
- **Area:** `is` of generic types

## Problem

```ts
/** @derive(Decode) */
export type Maybe<T> = T | null;

maybeIs<number>('x'); // true
```

The `is` of a generic type narrowed a value to any type argument the caller named, without checking
anything of it: a union with a type parameter as a member said yes to every value, an interface's
`is` decoded the value without a decoder for its parameters, and a generic class's `is` said yes to
any instance.

## Cause

Nothing in a value says what its type arguments are, and `is` was given nothing to check them by.

## Introduced

`e6845936` (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and shared values
left uncoded"), on the branch this one is stacked on, gave a generic type's coders their parameters'
coders and left `is` without them.

## Fix

The `is` of a generic type takes a guard for each type parameter, `(value: unknown) => value is
T`,
and decodes the value with decoders made from them, so every value of a parameter, at any depth, is
checked by its guard. A generic class's `is` no longer takes an instance on its class alone.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which the `is` of every generic type took its guards, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-generics` checks `GenericMaybe`, `GenericList`, `GenericBox`, `GenericResult` and
`GenericStack` against a guard of numbers and one of text, at the top, in a collection, a tuple, a
map, a record and a union member, and `exact-types` type-checks them.
