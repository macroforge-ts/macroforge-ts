# fix(svelte-preprocessor): expand a JavaScript block under a JavaScript name

- **Date:** 2026-10-13 13:05
- **Commit:** 4b31550d
- **Area:** `@macroforge/svelte-preprocessor`

## Problem

With `processJavaScript: true`, a built-in derive in a `<script>` without `lang="ts"` was expanded
to TypeScript inside a script Svelte reads as JavaScript.

## Cause

The preprocessor hands a script block to the expander under its component's name, `App.svelte`,
whatever language the block is written in, so the expander's refusal of a derive in a JavaScript
file did not see it.

## Introduced

a132f95d (2025-12-09, "Bump version to 0.1.19 and add Svelte preprocessor package") added
`processJavaScript`. The refusal it now meets is added on this branch.

## Fix

A JavaScript block is expanded as `App.svelte.js`. A built-in derive in it is refused, the error is
logged, and the block is left as it was. A macro that writes JavaScript is expanded as before.

## Verified

Ran the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which the preprocessor's tests expand a built-in derive in a JavaScript block
with `processJavaScript` on, and expect the block left as it was and the refusal logged.
