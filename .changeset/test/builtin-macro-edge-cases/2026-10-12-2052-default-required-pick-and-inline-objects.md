# fix(core): default a Required, a Pick and an object type written in place by their fields

- **Date:** 2026-10-12 20:52
- **Commit:** 5b6d9d6f
- **Area:** Built-in Default derive

## Problem

`Default` gave `{}` for a field typed `Required<T>`, `Pick<T, K>`, `Omit<T, K>`, `NonNullable<T>` or
an object type written in place, `{ x: number }`. None of those is a value of its type: each lacks
the fields the type requires. A field typed by a generic type that derives `Default`, `Box<string>`,
defaulted to `undefined`, and one that does not derive it was not reported. An alias of another
type, `type Tags = string[]` or `type Pair = [string, number]`, refused to derive `Default` without
`@default(value)`, though a field of the same type defaults.

## Cause

Every utility type shared one branch that returned an empty object, an object type written in place
returned `{}` without being read, and a generic instantiation returned `undefined` because its type
was looked up in the registry by its whole text, arguments included. An alias that is not an object,
a union or a primitive only ever read its `@default`.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo") gave every utility type and every object type written in
place an empty object, and a generic instantiation `undefined`. 059d68c9 (2026-07-20,
"refactor(syn): centralize TS type-string helpers in type_normalize") moved the utility-type test
without changing what it returns.

## Fix

- An object type written in place defaults to its required members, each by its own default. One
  that is only an index signature has none, and stays an empty object.
- `Required<T>` and `NonNullable<T>` default as what they wrap. A `Required` of a type that leaves
  fields optional is an expansion error naming them, since no default of the type holds them.
- `Pick<T, K>` and `Omit<T, K>` default to the fields of `T` they keep, read from the registry. When
  `T` is not known there, the field is an expansion error asking for `@default(value)`.
- A generic type the registry holds deriving `Default` defaults through its own function with the
  same type arguments, and one that does not derive it is reported like any other field type.
- An alias of another type, or of a tuple, defaults as a field of that type does. One whose type has
  no default, a function or a type that does not derive `Default`, still needs `@default`.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-options` suite reads the default of each kind of field,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
