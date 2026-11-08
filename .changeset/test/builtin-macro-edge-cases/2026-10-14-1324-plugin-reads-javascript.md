# fix(typescript-plugin): report a derive in a JavaScript file

- **Date:** 2026-10-14 13:24
- **Commit:** 5feec8c2
- **Area:** `@macroforge/typescript-plugin`

## Problem

A built-in derive in a JavaScript file, which the expander refuses and the build fails on, showed no
error in the editor.

## Cause

The plugin read only `.ts`, `.tsx` and `.svelte` files, and left a JavaScript file to TypeScript as
it was.

## Introduced

`6d28c6ea` (2026-10-13, "fix(core)!: refuse a derive where the code it generates cannot go"), on the
branch this one is stacked on, refused the derive in a JavaScript file, which the plugin never read.

## Fix

The plugin reads `.js` and `.jsx` files, and the refusal appears at the derive.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the TypeScript plugin reports the
refusal of a derive in a JavaScript file at the derive.
