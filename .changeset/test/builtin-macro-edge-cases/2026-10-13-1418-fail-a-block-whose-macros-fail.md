# fix(svelte-preprocessor)!: fail a script block whose macros fail

- **Date:** 2026-10-13 14:18
- **Commit:** c334416f
- **Area:** `@macroforge/svelte-preprocessor`

## Problem

A macro error in a component's script did not fail its build. The preprocessor logged the error to
the console and handed Svelte the script with the `@derive` comment removed and nothing generated,
so the component was built without the code its macros write, and broke where that code is used,
with nothing there to say why. An engine that threw, and one that could not be loaded at all, were
logged as warnings the same way, and every block was passed through unexpanded.

## Cause

The preprocessor was written to be resilient: each of the three failures was caught, logged and
followed by the unchanged or half-expanded block. The Vite plugin and `macroforge expand` fail on
the same errors.

## Introduced

d60fa9da (2025-12-14, "Add detailed documentation comments to all public types and functions") is
the earliest commit holding the three handlers as documented behaviour.

## Fix

The preprocessor throws for a macro error, with every message of the block and the component's name;
for an engine that throws, with the file and the cause; and for an engine that fails to load, with
the cause. Svelte reports the error for the component. Warnings are still logged.

Breaking: a component with a macro error no longer builds.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the preprocessor's tests expect a
rejection carrying both messages of a block with two macro errors, one for a script the engine
cannot read and one for a built-in derive in a JavaScript block, and `pixi run test:testground` (43
Deno suites, 13 validator files of 638 steps and 236 browser tests in Chromium, all passed), which
builds the `svelte` and `library` apps through the preprocessor: no component of theirs had a macro
error it was building past.
