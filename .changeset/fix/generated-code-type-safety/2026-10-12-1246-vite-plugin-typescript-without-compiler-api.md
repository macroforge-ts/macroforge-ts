# fix(vite-plugin): skip declarations when typescript has no compiler API

- **Date:** 2026-10-12 12:46
- **Commit:** 9e2d6d28, 5eb39647 (fixup)
- **Area:** Vite plugin, declaration generation

## Problem

With typescript 7 installed, the first file whose macros generated code failed to transform.

## Cause

The plugin treated any `typescript` it could `require` as the compiler. Version 7's package resolves
but has no `findConfigFile` or `createProgram`, so the declaration emit threw.

## Introduced

By typescript 7.0.2 (2026-07-08), not by a commit here.

## Fix

A package without `createProgram` is passed over for the next resolution root. When none has the
API, the plugin warns with the version it found and skips the declarations, as it already did when
typescript is missing. Macro expansion is unaffected.

## Verified

New test `skips .d.ts generation, by name, when typescript has no compiler API`. It runs a copy of
the plugin in a directory outside the repository, since from inside it the plugin's own location
always resolves the workspace's typescript. It passes, and the expansion in it still succeeds.
