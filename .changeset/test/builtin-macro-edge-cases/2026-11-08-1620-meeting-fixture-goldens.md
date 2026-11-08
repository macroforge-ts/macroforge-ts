# test(conformance): record main's Meeting fixture with the branch's encoding

- **Date:** 2026-11-08 16:20
- **Commit:** 9dafd94c
- **Area:** testground conformance goldens, testground lockfiles

## Problem

After the rebase onto `main`, `testground_expansions` and `testground_registries` failed on the
library app's `Meeting` type, and the library app's lock still resolved `@sveltejs/package` 2.5.8
against a manifest asking for `^3.0.1`.

## Cause

`main` added the `Meeting` type with goldens written by its own expander, before this branch's
encoding changes existed, and moved the library app to svelte-package 3. The rebase kept this
branch's lockfiles.

## Introduced

`33b41217d` (2026-11-08, "fix(cli): ship the foreign-type handlers under svelte-package 3") added
the `Meeting` fixture, its goldens and the svelte-package 3 manifest.

## Fix

The `Meeting` expansion and the library registry are re-recorded with the shared references, field
selection and own-member decoding this branch emits, and the testground apps are relocked.

## Verified

`pixi run test:conformance:update` re-recorded the two goldens (reviewed: the expansion matches the
branch's other library types), then `pixi run test:rust` passed.
