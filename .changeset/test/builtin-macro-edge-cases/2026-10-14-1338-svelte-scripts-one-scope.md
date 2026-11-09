# fix(svelte-preprocessor): expand the module and instance scripts of a component as one scope

- **Date:** 2026-10-14 13:38
- **Commit:** e8aebc2c
- **Area:** `@macroforge/svelte-preprocessor`

## Problem

A component that derived in both its module script and its instance script did not compile:
"`__mf_debuggedWithin` has already been declared". One that derived a type of one name in both
declared its functions twice.

## Cause

A component's two scripts are one scope, and the preprocessor expanded each alone, so each imported
the runtime helpers it uses.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") expanded the scripts of a component one at a time.

## Fix

The instance script leaves out the helpers the module script imports, with its map moved to the text
that stays, and a type derived in both scripts fails, naming the functions both would declare.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the Svelte preprocessor's tests
preprocess and compile a component deriving in both scripts and fail one deriving a type of one name
in both, naming `itemToString`.
