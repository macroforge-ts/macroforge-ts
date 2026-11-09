# fix(core)!: cover the fields, enums and names the built-in derives misread

- **Date:** 2026-10-12 21:45
- **Commit:** fedc3585
- **Area:** Lowering, expansion host, shared derive helpers, built-in Debug and Default derives

## Problem

- A class field with no type annotation (`count = 0`) was typed `any`. `Decode` accepted any JSON
  value for it, `Encode` wrote whatever it held, and the constructor `Decode` generates took it as
  `any`.
- An `accessor` field was not a field to any derive: `Clone` dropped it, `PartialEq` and `Hash`
  ignored it and `Encode` left it out, without a diagnostic.
- `Debug` on a string enum printed `Status(active)` instead of `Status.Active`, and for
  `enum E { A = 'B', B = 'x' }` printed `E.x` for `E.A`. On a `const enum` the generated code did
  not type-check: it indexed the enum by a value, and merged a namespace of functions into it, which
  TypeScript refuses (TS2567).
- A derive on a `declare enum` generated functions that read an enum the module does not hold, so
  `decode` threw for every value.
- Two types whose names differ only in case (`HTTPResponse`, `HttpResponse`) generate the same
  function names, and a module could already declare one of them. The later declaration replaced the
  earlier.
- A field named `__proto__` was read and assigned as the prototype.
- `Default` reported only the first field whose initializer reads `this`.

## Cause

Lowering kept a class member only when it was a property definition, and gave a field without an
annotation the type `any`. `Debug` looked a member's name up in the enum object by its value, which
only a numeric enum maps back. Nothing compared the names the derives generate, checked how an enum
is declared, or told `__proto__` from a field name.

## Introduced

af0a8b4f (2026-09-28, "Remove the swc backend") wrote the oxc lowering that keeps property
definitions alone and types an unannotated field `any`. bb3be3c2 (2025-12-08, "Support enums and
type aliases in all built-in macros") gave `Debug` its lookup by value, and 6d0bd6e4 (2025-12-16,
"Make enums emit namespaces to stop declaration conflicts, update serde macros to have *With
options") merges a namespace into every enum. d2502dd7 (2025-12-17, "refactor(macros): update
builtin derive macro implementations") names the generated functions after the type in camel case.
1cc2f3c4 (2026-10-08, "feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign
handlers") added the `this` check that returns at the first field.

## Fix

A field without an annotation takes the type its initializer makes plain: `number`, `string`,
`boolean` and `bigint` for a literal (the literal's own type when the field is `readonly`), the
asserted type of `x as T`, `RegExp` for a pattern, and the type of `new Date()`, `new URL(..)` or a
`new` with type arguments such as `new Set<string>()`. A parameter property is typed by its default
the same way, and a field set up by `$state` by the rune's type argument or the value it is given. A
field whose type stays unsaid is marked `untyped` on `FieldIR`.

An `accessor` is lowered as a field, marked `accessor` on `FieldIR`. The derives that read an
instance work over it. `Clone`, `Default` and `Decode`, which build an instance without running its
constructor, report an error naming the field: its storage is the constructor's to set up.

`Debug` on an enum finds the member whose value it is given, so every kind of enum prints
`Enum.Member`. A `const enum` gets its functions and no namespace. A built-in derive on a
`declare enum` is an expansion error, as is a generated function name another derived type or the
module already has, and a field named `__proto__`. `Default` reports every field whose initializer
reads `this`.

`Debug` of a string enum member is now `Enum.Member`, a `const enum` no longer has `Enum.toString`
and its siblings, and `Decode` refuses a value of the wrong type for a field typed by its
initializer.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (161 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground suites
selected by `deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering`
(all passed), whose `edge-classes` and `edge-enums` suites decode wrong-typed input for each
inferred field, read accessor fields and name the members of every kind of enum,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `generated_name_collisions`, `prototype_named_field`,
`class_accessor_field_built` and `declare_enum` fixtures, and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
