# fix(svelte-preprocessor): name the line and column of a macro error

- **Date:** 2026-10-14 13:12
- **Commit:** 5debbf99
- **Area:** Errors of `@macroforge/svelte-preprocessor`

## Problem

A macro error in a component failed its script block with the message alone, so nothing said where
in the component it was.

## Cause

The preprocessor collected each error's message and dropped the place the engine gave with it, which
is an offset in the script, not in the component.

## Introduced

`045cd406` (2026-10-13, "fix(svelte-preprocessor)!: fail a script block whose macros fail"), on the
branch this one is stacked on, failed the block with the messages alone.

## Fix

Each message opens with the component, line and column, read from where the script stands in the
component's markup.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the Svelte preprocessor's test
fails a component whose field needs a default of its own type and finds the message naming
`chain.svelte:5:5`, the field's place in the component, and the three groups on the branch's final
tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (48 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
