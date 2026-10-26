# fix(core): generate Encode and class Clone code that type-checks without lib `any`s

- **Date:** 2026-10-12 12:46
- **Commit:** 060f8168, dec3e37c (fixup), 5560604d (fixup), 05cfe063 (fixup)
- **Area:** Built-in Encode and Clone derives, endec and structural runtime, testground

## Problem

Checked against a standard library with no `any` in a value position, the vanilla and svelte
testgrounds had 365 type errors, all in generated code but six.

`encode` also returned `undefined`, typed as a `string`, for a top-level `undefined`.

## Cause

Every `encode` built its JSON text inline with `JSON.stringify` and a replacer that drops the tag
and `__id`. With the replacer typed, it matches no overload, and `JSON.stringify` of an `unknown` is
`string | undefined`, which is what it returns at run time for `undefined`.

A class `clone` assigned fields onto `Object.create(Object.getPrototypeOf(value))`, which has a type
only because `getPrototypeOf` returns `any`.

## Introduced

The clone body dates from 0decde5a (2025-11-24), `implementing macros`. The `keepMetadata` replacer
predates the monorepo sync, 65c22961 (2026-04-07).

## Fix

The six inline copies become two endec helpers, `encodedText` and `encodedTextWithoutMetadata`,
which every encoder calls. They throw a `TypeError` for a value with no JSON text.

A class `clone` builds its fields as a literal, checked against the class, and hands them to
`instanceWith` in the structural runtime. An optional field absent from the source stays absent, as
it already did for interfaces.

The two testgrounds install such a library and gain a `tsconfig.strict-lib.json` with
`libReplacement`, so expanded code is checked against both libraries. Five places in their own code
that leaned on an `any` are narrowed.

## Verified

- On the stricter library and TypeScript 6.0.3: `macroforge tsc` on the vanilla testground went from
  67 errors to exit 0, and `macroforge svelte-check` on the svelte testground from 298 errors to 0
  in 915 files. Both still pass on TypeScript's own library.
- New suite `strict-lib` (5 tests): both projects type-check, every lib either project loads is a
  replacement, and a line only the stock library accepts fails through `macroforge tsc`.
- New runtime tests in `value-traits` (21 of 21 pass): a class clone keeps its class without running
  the constructor, `encode` drops or keeps the metadata, and `encode` throws for `undefined`.
- `pixi run test:conformance` passes with 140 snapshots moved: the encode text, the import lines,
  the class clone body and the offsets those shift.
- The testground's Deno suites: 23 of 23 pass.
- `pixi run test:all` then failed twice, each fixed by a fixup:
  - `foreign_type_fixtures`: three snapshots of generated handlers, which the conformance update
    does not cover, differed in the endec import line alone. Accepted through the test.
  - The vanilla browser test `accepts valid newtype values and keeps every bigint digit`: the result
    renderer rewritten here walked plain objects only, and the decoded form is a class instance. It
    now walks any object without a JSON form of its own.
- After the fixups: the Rust workspace and the testground macro crate pass, the six package suites
  pass, and the vanilla browser suite passes, 126 of 126.
