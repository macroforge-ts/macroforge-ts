# fix(core)!: refuse a field Default has no value for

- **Date:** 2026-10-13 23:13
- **Commit:** cd7766e3
- **Area:** Default, and Decode's defaults

## Problem

```ts
/** @derive(Default) */
export interface Job {
    done: Promise<string>;
    key: keyof Seed;
    brand: string & { __brand: 'Id' };
}
```

Default wrote what it could not write: `Promise.resolve()` for a `Promise<string>`,
`keyofSeedDefaultValue()` and `thisDefaultValue()` (functions that do not exist), `undefined` for
`never`, `ReturnType<...>` and a generic type the registry did not hold, and a parse error for an
intersection or an indexed access.

## Cause

The check for types Default cannot write looked at the top of a type only, and did not resolve
`keyof` and indexed access first. The writer fell back to a function named after whatever text the
type had.

## Introduced

`5b6d9d6f` (2026-10-12, "fix(core): default a Required, a Pick and an object type written in place
by their fields"), on the branch this one is stacked on, added the check.

## Fix

The check walks the whole type (tuple elements, members written in place, the union member that is
defaulted, the arguments of `Promise`, `Readonly`, `Required`, `NonNullable` and finite `Record`s)
after resolving `keyof` and indexed access, and refuses with the reason a type no default can be
written for. It also guards the default of a union member and Decode's defaults. `Promise<T>`
defaults to a promise of `T`'s default, the boxed primitives and `Object` to their empty values, and
a generic type the registry does not hold to its own default function. The unused
`macroforge_ts_syn::type_normalize` module is removed.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose
`a_type_no_default_can_be_written_for_is_told_with_why` covers each refusal and the types that
default at depth, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots
reviewed by the lines that changed with the `default_unwritable_types` and
`default_settled_and_boxed` fixtures, and the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which
`edge-operators` defaults a type whose fields are written through `keyof` and indexed access and
`exact-types` type-checks it.
