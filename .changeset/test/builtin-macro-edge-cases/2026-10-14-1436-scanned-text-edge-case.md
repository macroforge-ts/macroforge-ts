# test(testground): derive over type text a derive reads by scanning it

- **Date:** 2026-10-14 14:36
- **Commit:** 3c6d232d
- **Area:** testground

## Fix

A `scanned-text` edge case derives over a union holding `'\\'`, a template literal alias whose
parameter is substituted where it is used, and functions returning a union, and its suite decodes,
checks, clones and compares them.

## Verified

Ran `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
lines that changed and the `edge-scanned`, `exact-types` and `strict-lib` testground suites.
