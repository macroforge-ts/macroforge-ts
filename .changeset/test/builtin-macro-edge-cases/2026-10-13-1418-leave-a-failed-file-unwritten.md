# fix(deno-plugin)!: leave a file whose macros fail unwritten, and fail the command

- **Date:** 2026-10-13 14:18
- **Commit:** 3c90e75d
- **Area:** `@macroforge/deno-plugin`

## Problem

`expand` wrote a file to the mirror whatever its expansion reported, printed a macro error as a
warning, and exited 0. A project with a macro error got a mirror holding the file without the code
its macros generate, and a command that said it had succeeded.

## Cause

`expandProject` and the watcher wrote `result.code` without looking at the diagnostics, an event
carried no word of failure, and the command printed every diagnostic with `console.warn` and never
set a status.

## Introduced

d9f76335 (2026-04-20, "Add @macroforge/deno-plugin + four Rust-inspired attribute macros") added the
plugin this way.

## Fix

A file whose expansion reports an error is not written, by `expandProject` and by the watcher, and
its event has `failed` set and `expanded` unset. The command prints the file as failed and its
errors on standard error, counts the failed files in its summary, and exits 1 when there is one. The
watcher keeps watching.

Breaking: `ExpandFileEvent` gains `failed`, and `expand` exits 1 for a project with a macro error.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the plugin's new `project` test
expands a project of one file that expands and one whose default would never end, and expects the
first written and the second reported as failed with nothing in the mirror.
