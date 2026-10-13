# test(testground): cover the edge cases of the built-in derives

- **Date:** 2026-10-12 17:25
- **Commit:** 5398802a
- **Area:** Testground fixtures and suites, conformance fixtures

## Problem

The built-in derives were tested over the shapes the testground apps happen to use. Nothing drove
them over the shapes at the edges of what a type can say: collections in collections, keys that are
not strings, tuples, object types written in place, every tagging mode, graphs rather than trees,
classes with guarded, inherited and constructor-declared fields, names that collide with generated
code, enums of every kind of value, newtypes in every position, and every decorator option. The
sweep that wrote these fixtures found the defects the other entries of this branch fix.

## Cause

The fixtures grew with the features they demonstrated, one shape per feature.

## Fix

Eleven fixture modules under `tooling/testground/vanilla/src/edge-cases/`, each a family of shapes
under every derive that applies, and one runtime suite per family in `tooling/testground/tests/`
(`edge-*.test.mjs`): each expands its module through the CLI and asserts what the generated code
does, down to the path every decode error is reported under. The modules are part of the vanilla
project, so the existing `tsc` and stricter-lib checks type-check them, and a new
`tsconfig.exact.json` with `exact-types.test.mjs` checks them under `exactOptionalPropertyTypes` and
`noUncheckedIndexedAccess`.

Conformance gains fixtures for the newly supported shapes and for each new expansion error, and unit
tests cover the recursive coder, property names, inheritance through the registry and the `default`
flag.

## Verified

Ran the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed)
and `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
