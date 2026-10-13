# test(testground): cover directive texts, skipped fields, hostile input and module shapes

- **Date:** 2026-10-13 07:03
- **Commit:** 6dfcf346
- **Area:** Testground fixtures and suites

## Problem

No suite ran a directive whose strings hold brackets, commas or option names, a declaration with
several `@derive` directives, a required field `Decode` skips, a flattened type that may be `null`,
input a decoder cannot trust, or the ways a module can declare a derived type.

## Fix

- `directive-shapes.ts` and the `edge-directives` suite: options and defaults whose text holds `(`,
  `]`, `,` and the words `skip` and `flatten`, stacked and repeated `@derive` directives, skipped
  required fields of an interface and of a class, and a flattened `T | null`.
- `hostile-shapes.ts` and the `edge-hostile` suite: roots that are no object, `__proto__` and
  prototype-named keys in a record, a map and an object, inherited members, sparse arrays and
  array-likes, scalars of the wrong kind, each misspelling of a `bigint`, throwing getters and
  proxies, references to nothing and to another type, nesting past the stack, a class's own members
  named in its input, unknown keys under `denyUnknownFields`, and `freeze`.
- `module-shapes.ts` and the `edge-modules` suite: types a module does not export, static members
  and static blocks, `implements`, a superclass that derives nothing, overloads and optional
  methods.
- A CLI test that `expand` reports the same error for a relative and an absolute path.

## Verified

Ran `pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser
tests in Chromium, all passed) and `pixi run test:conformance:update` (10 passed), with the
regenerated snapshots reviewed by the shape of each changed line.
