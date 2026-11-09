# fix(typescript-plugin): pin an error in the generated imports to a decorator's one macro

- **Date:** 2026-10-13 07:20
- **Commit:** e6f7e8da
- **Area:** TypeScript plugin

## Problem

A type error inside the import block an expansion generates was shown on the whole `@derive(...)`
decorator, where an error in generated code is shown on the macro's own name. For `@derive(Debug)`
that is now the common case, since `Debug` imports a helper: three tests of `macro-error-positions`
failed with the error at the decorator's start instead of at `Debug`.

## Cause

The import block is one generated region, named `imports`, which every macro of the file shares. The
plugin looks the region's name up among the decorator's macros and falls back to the whole decorator
when it is none of them. It took a decorator's only macro just when the region had no name at all.

## Introduced

8693be36 (2026-10-12, "fix(core): follow a value that holds itself in the value traits"), on this
branch, made `Debug`, `Clone`, `PartialEq` and `Hash` import a runtime helper, so a file deriving
only those gained an import block. The fallback itself predates it.

## Fix

A generated region that none of the nearest decorator's macros names is that decorator's one
macro's, when it has exactly one.

## Verified

Ran `pixi run test:packages` (every package passed), in which the three `macro-error-positions`
tests of the TypeScript plugin that had failed pass.
