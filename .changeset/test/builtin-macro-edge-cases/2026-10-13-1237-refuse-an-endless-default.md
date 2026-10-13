# fix(core)!: refuse a default that would need itself

- **Date:** 2026-10-13 12:37
- **Commit:** 5920bd04
- **Area:** Built-in Default derive

## Problem

`@derive(Default)` on a type that requires a value of its own type generated a default that calls
itself:

```ts
export function chainDefaultValue(): Chain {
    return { next: chainDefaultValue() };
}
```

The first call overflows the stack. Two types that require each other did the same between them.

## Cause

A required field of a type that derives `Default` is filled by that type's default, and nothing
looked whether the round comes back to where it started.

## Introduced

752a6148 (2025-12-14, "Remove unused @derive decorators from vanilla macros and demo validator forms
to clean up code style") is the earliest commit holding the call to a field type's own default.

## Fix

For each field `Default` fills from its type, the types whose defaults that default calls are
followed through the registry, by the fields their own defaults fill the same way. A round that
returns to the type is an expansion error on the field: "field 'right' needs a default of 'Left'
itself, through `Right.left`, so a default of it would never end". A nullable, an optional or a
collection field, an initializer and a `@default` each end the round. A file expanded without a
project scan knows no type but its own, and finds the rounds that stay in one type.

Breaking: such a type expanded before, to a default that could not be called.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `default_requires_itself` fixture, which holds a type requiring itself
straight and through a tuple, and four fields that end the round, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which the `cli`
suite expands two types that require each other in a scanned project and reads both errors.
