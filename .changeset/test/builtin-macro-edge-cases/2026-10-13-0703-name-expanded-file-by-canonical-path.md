# fix(cli): name an expanded file by its canonical path

- **Date:** 2026-10-13 07:03
- **Commit:** a8b5f6d2
- **Area:** CLI

## Problem

`macroforge expand src/whole.ts` did not resolve a type that another file of the project also
declares by that name, even when `whole.ts` declares it itself. What depends on the type being known
was then skipped without a word: a field flattening such a type collided with a sibling key and
expanded clean, where `macroforge expand /abs/path/src/whole.ts` reported it.

## Cause

A name two files declare is resolved to the declaration in the caller's own file by comparing file
paths. The registry holds canonical paths, and the file being expanded was named as the command line
spelled it.

## Introduced

2aabc445 (2026-10-08, "feat(core)!: add $Newtype brands and check primitive values on decode") has
the CLI hand `expand_project_file` the path as the command line spells it.

## Fix

`macroforge expand` names the file by its canonical path, or its absolute path when it is not on
disk, before it expands it. Its messages still show the path as it was given.

## Verified

Ran `pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser
tests in Chromium, all passed), whose `cli` suite expands one file by a relative and by an absolute
path and reads the same error from both, and `pixi run test:rust` (workspace and
`tooling/testground/macro` passed).
