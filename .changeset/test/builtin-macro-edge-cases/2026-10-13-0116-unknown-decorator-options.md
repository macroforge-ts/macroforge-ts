# fix(core)!: report an option a built-in decorator does not take

- **Date:** 2026-10-13 01:16
- **Commit:** 3915d6f6
- **Area:** Expansion host, built-in Default derive, endec option lists

## Problem

- An option a decorator does not take was ignored: `@debug({ skp: true })` printed the field,
  `@ord({ skipp: true })` compared it, and `@endec({ tagg: "kind" })` on an interface tagged
  nothing.
- `@default(1, 2)` on a field wrote both values into the generated object, `twice: 1, 2,`, and the
  expansion failed with an internal error that its patches do not parse.

## Cause

Each decorator's options are looked up by name in its arguments, and nothing read the names that
were there. A default's arguments were taken whole as its value, and two expressions with a comma
between them parse as one.

## Introduced

c600200e (2025-11-24, "Improve decorator span handling and error reporting") wrote `has_flag`, which
looks an option up by name wherever it stands in a decorator's arguments.

## Fix

A type with a built-in derive has the options of its `@debug`, `@hash`, `@partialEq` and `@ord`
decorators, and of an `@endec` on the type itself, checked against what each takes, on the type and
on its members at any depth. An option it does not take is an expansion error on the decorator. A
default written as several values is an expansion error saying a default is one.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (728 passed), the 25 testground
suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `decorator_options` fixture, and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
