# test(testground): cover type arguments, unreadable members and shared values

- **Date:** 2026-10-12 18:40
- **Commit:** 6abdb8b4
- **Area:** Testground fixtures and suites, conformance fixtures

## Problem

The edge-case suites left some shapes undriven: a generic type's arguments in each position that can
hold them, `Required`, a union written in place, a type named in lowercase held from another module,
a value held twice or holding itself under each derive, a class of Svelte runes under the derives
that only read it, and the declarations a derive has to refuse.

## Cause

The first sweep took a type parameter's values and a shared value's references as given, and wrote
its fixtures around what expanded.

## Fix

- `generic-shapes.ts` gains `GenericWrapper`, `GenericOutcome` and `GenericArguments`, which holds
  each kind of generic type with arguments that need coding: an interface, a class, an object union,
  a tuple-free pair, a `Partial` and a `Pick` of one, a nullable one, one beside a `string`, and a
  generic union alias alone and in an array. `edge-generics.test.mjs` asserts what each decodes to
  and the path every failure is reported under.
- `partial-shapes.ts` gains `EdgeOptional` and `EdgeRequired` for `Required` over a type, a `Pick`
  and a `Partial`.
- `name-imports.ts` holds the lowercase `point2D` from another module under every derive.
- `edge-graphs.test.mjs` asserts that the value traits overflow on a value that holds itself, that
  text without metadata writes a shared value out and decodes, and that it refuses a cycle.
- The svelte testground gains `RuneCounter`, a class of `$state` and `$derived` fields under
  `Debug`, `PartialEq`, `Hash` and `Encode`, shown on the home page and driven by a Playwright test
  that clicks its button and reads the derives' output.
- Conformance gains error fixtures for unnamed members, shadowed globals, reserved keys and unions
  written in place, and expansion fixtures for generic arguments, union members with arguments,
  unions carried as they are, and the single missing-field error.

## Verified

Ran the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line and
`playwright test --config=playwright.svelte.config.ts e2e/svelte-macros.spec.ts` against the built
svelte testground in Chromium (10 passed), which includes the new test of the rune class.
