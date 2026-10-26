# fix(cli): reject a typescript package without the compiler API

- **Date:** 2026-10-12 12:45
- **Commit:** dca9af44
- **Area:** CLI, tsc, svelte-check and svelte-package wrappers

## Problem

`macroforge tsc`, `svelte-check` and `svelte-package` in a project on typescript 7 died with a
`TypeError` from inside the wrapper script.

## Cause

The wrappers expand macros inside the compiler's file reads, through `createProgram` and `ts.sys`.
The typescript 7 package exports a version and nothing else, so `ts.sys` is undefined.

## Introduced

By typescript 7.0.2 (2026-07-08), not by a commit here: the wrappers have always assumed the package
carries the compiler.

## Fix

The three wrappers load TypeScript through one shared check, which names the version it found and
says the command needs typescript 5 or 6. The svelte-package wrapper now imports the shared setup
the other two already used.

## Verified

New test `tsc: a typescript package without the compiler API is rejected by name`, against a project
whose `typescript` is the main module version 7 ships. The `tsc` suite passes, 4 of 4.
