# fix(cli): report the errors of an expansion that changed nothing

- **Date:** 2026-10-13 11:14
- **Commit:** 2b78ebe9
- **Area:** `macroforge expand`

## Problem

`macroforge expand` on a file whose only `@derive` is attached to nothing a derive expands printed
"no macros found" and exited 2. The error the expansion reports for that derive was not shown.

## Cause

The command took an expansion that changed nothing for a file without macros, before looking at what
the expansion reported.

## Introduced

da8f0fda (2026-10-13, "fix(core)!: report an @derive that expands nothing or reads part of a type"),
on this branch, added errors for a derive that expands nothing, which are the errors of an expansion
that changes nothing. The command's shortcut dates from c963b3b3 (2025-12-11, "Bump macroforge
version to 0.1.29 and improve CLI exit codes") and was right until then.

## Fix

An expansion that changed nothing is a file without macros only when it reports no error. One that
does fails the command with its errors, and writes nothing.

## Verified

Ran the three groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed), in which the `cli` suite expands a file whose `@derive` is attached to nothing and
one whose class has no name, and expects exit 1 with the error and no file written, and still
expects exit 2 for a file without macros.
