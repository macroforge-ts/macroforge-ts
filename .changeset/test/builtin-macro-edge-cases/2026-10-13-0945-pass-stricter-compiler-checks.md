# fix(core): generate code that passes three stricter compiler checks

- **Date:** 2026-10-13 09:45
- **Commit:** 2e9a785c
- **Area:** Generated code of every built-in derive on a class, Encode and Decode

## Problem

A project that turns on compiler checks beyond `strict` could not type-check what the derives
generate. Over the testground's edge-case modules:

- `noPropertyAccessFromIndexSignature`: 259 errors, "Property '__id' comes from an index signature,
  so it must be accessed with ['__id']" in every Decode, and the same for `value`, `e0` and `rest`
  in the Encode of a tuple or a primitive alias.
- `noImplicitOverride`: 55 errors, "This member must have an 'override' modifier because it
  overrides a member in the base class", on each static a class derives when its superclass derives
  the same macro, and on the `toString` Debug gives any class that has a superclass.
- `allowUnreachableCode: false`: "Unreachable code detected" in the decoder of a union with a type
  parameter among its members, such as `T | null`.

## Cause

Decode read `obj.__id` off the record of a value's own members, and the Encode of an alias read
`result.value` off the record it had just filled: dot reads of keys an index signature gives.

A static was written `static clone(...)` whatever the class extends. A class's static side inherits
its superclass's, and every class has `toString` as a function, so each was an override without the
mark.

The decoder of a union with a type parameter ends that arm on `return value as T`, with the arms for
tagged members and the closing `throw` written after it regardless.

## Introduced

No single change: none of the three checks had been run against generated code. The dot read of
`__id` dates from 773e843b (2025-12-11, "Add serde runtime helpers and update to v0.1.30"); the read
of `result.value`, the unmarked statics and the return ahead of the throw from 1cc2f3c4 (2026-10-08,
"feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign handlers").

## Fix

The keys of a record are read in brackets. A generated static is marked `override` when a class the
type extends hands one of its name down: one in the registry that derives the same macro or declares
the static itself, followed up the chain of superclasses, and `toString` for any class with a
superclass. A class without a superclass, and a static no superclass has, stay unmarked, since
`override` on a member that overrides nothing is an error under every configuration. The arms after
a type parameter's return are no longer written.

The testground's `tsconfig.exact.json` turns the three checks on, together with `noImplicitReturns`
and `verbatimModuleSyntax`, which the generated code already passed. `erasableSyntaxOnly` was tried
as well: it fails only on the namespace merged with an enum, and an enum is itself not erasable.

## Verified

Ran `macroforge tsc` over the testground's edge-case modules with the three checks on before the fix
(259, 55 and 1 errors, all in generated code) and after it (none). Ran
`cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `exact-types` type-checks under the five options
`tsconfig.exact.json` now adds and `edge-classes` calls a derived static over a hand-written one.
