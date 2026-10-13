# test(testground): cover tagging modes over mixed members, deep inheritance and misused validators

- **Date:** 2026-10-13 00:46
- **Commit:** b23d18ca
- **Area:** Testground fixtures and suites, conformance fixtures and snapshots

## Problem

No fixture put objects, literals and template literal types into one union, ran the same variants
through every tagging mode, or derived over a class three levels deep with fields typed by their
initializers. Nothing pinned what a misspelled or misplaced validator reports.

## Fix

`variant-shapes.ts` with `edge-variants.test.mjs`, and the conformance error fixture
`endec_validator_misuse`.

## Verified

Ran the 25 testground suites selected by
`deno task test edge- exact-types tsc strict-lib shapes value-traits endec ordering` (all passed)
and `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
