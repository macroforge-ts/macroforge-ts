# fix(core)!: derive over literal unions, empty types and an interface's methods as they are

- **Date:** 2026-10-12 22:08
- **Commit:** 018f5131
- **Area:** Built-in Debug, Clone, PartialEq, Hash and Default derives, shared derive helpers

## Problem

- `Hash` on a field typed as a union of literals of several kinds (`'a' | 3 | true`) called `.split`
  on whatever the field held, and threw for a number or a boolean.
- `PartialEq` on an alias of the empty tuple generated `return ;`, so two empty tuples compared as
  `undefined`.
- `Default` wrote a template literal type out as a value (`` `id-${number}` ``, which reads a
  variable named `number`), and called `nullDefaultValue()` and `undefinedDefaultValue()` for a
  field typed `null` or `undefined`.
- `Clone` on an interface that declares a method returned a value without it, which does not
  type-check, and `PartialEq`, `Hash` and `Debug` did not see the method.
- A `get name(): T` signature in an interface or an object type was lowered as a method, so no
  derive saw the property it declares, and a method of an object type alias was dropped.
- For a type with no fields, or an alias whose value needs no reading, the generated functions
  declared a `value` parameter they never read, which fails under `noUnusedParameters`.

## Cause

A type was read as a string type when its text started with a quote, which a union of literals does.
The tuple equality joined its comparisons with `&&` and had none. The default of a literal type was
the type's own text, and `null` and `undefined` fell through to the named-type default. An
interface's method signatures were lowered apart from its fields and no derive read them.

## Introduced

1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") wrote the value-type classification and the tuple equality. cd52355f (2025-12-12, "Bump
version to 0.1.31") wrote the default table without `null`, `undefined` or template holes. 65c22961
(2026-04-07, "Syncing monorepo") lowered interface methods apart from fields.

## Fix

A union is the primitive all of its members are, and is handled by its runtime structure when they
differ. Two empty tuples are equal. A template literal type defaults to its text with each
`${string}`, `${number}`, `${bigint}` and `${boolean}` hole filled, and a hole of any other type is
an error naming it. `null` and `undefined` default to themselves.

A method an interface declares or inherits is a member that holds a function, as a field typed as
one is: `Clone` keeps it, `PartialEq` compares it by identity, and `Default` asks for it like any
function-typed field. A `get` signature is lowered as a field of its return type, `readonly` without
a `set` beside it, and a method of an object type alias as a field that holds a function. A
parameter a generated function does not read is named `_value`.

`PartialEq`, `Hash` and `Debug` of an interface with methods now take the methods into account.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
whose `edge-exotics` suite hashes each kind of literal, compares empty tuples and clones an
interface with methods, `pixi run test:conformance:update` (10 passed), with the regenerated
snapshots reviewed by the shape of each changed line and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
