# fix(shared): map every line of the source an expansion keeps

- **Date:** 2026-10-14 13:12
- **Commit:** b7ae8150
- **Area:** Source maps of the Vite plugin and the Svelte preprocessor

## Problem

In the source map of an expanded file, only the first line of each stretch the expansion kept was
mapped: a position on any later line of it, a field of a class after its first line, mapped to no
place in the source, so an error or a breakpoint there was lost.

## Cause

The Vite plugin and the Svelte preprocessor each turned the engine's segments into a map with one
mapping at the start of each segment, though a segment is the original text as written over as many
lines as it spans.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") wrote the Svelte preprocessor's map, and `ef145565`
(2026-04-11, "Add Vite plugin source maps and HMR cache invalidation") the Vite plugin's, each with
the one mapping a segment.

## Fix

One builder in `@macroforge/shared` maps each line a segment spans to the line of the original it
holds, and both use it. The Vite plugin no longer needs `@jridgewell/sourcemap-codec`.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the shared `source-map` test
decodes the map of a segment over four lines and finds each mapped, and the Vite plugin's and the
Svelte preprocessor's tests trace two field lines of an expanded class back to their own lines, and
the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
