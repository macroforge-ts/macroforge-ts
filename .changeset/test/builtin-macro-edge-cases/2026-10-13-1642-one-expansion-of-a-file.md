# fix(deno-plugin)!: report every file the watcher cannot expand, and keep no stale copy

- **Date:** 2026-10-13 16:42
- **Commit:** 51927a3c
- **Area:** Deno plugin project expansion and watcher

## Problem

- A change the watcher could not expand at all, because reading the file or the engine threw, was
  written to the console and sent no event, so a caller's `onFile` never heard of it.
- A file that expanded once and then began to fail kept the copy the earlier run wrote. The mirror
  went on serving code its source no longer expands to, in `expand` and in `watch`.
- A file removed or renamed away left its copy in the mirror, and a save that replaces the file by a
  rename was not seen at all.
- A file whose expansion threw in `expand` ended the whole run at that file.

## Cause

`watchProject` held a second copy of the per-file steps of `expandProject`, with its own handling of
a failure, its own reading of `exclude` (a substring where the walk uses a pattern), and no step for
a file that is gone. Neither copy removed what an earlier run wrote.

## Introduced

`d9f76335` (2026-04-20, "Add @macroforge/deno-plugin + four Rust-inspired attribute macros") wrote
both functions with a copy of the steps each. `9ddf75bd` on this branch stopped a failed file from
being written, and left the copy of an earlier run where it was.

## Fix

One function expands a file for both. A file that cannot be read or expanded is a failed event with
the error as its diagnostic, and the run goes on to the next. A file that fails, or that has no
macros left where non-macro files are not copied, has its copy in the mirror removed. The watcher
reads a `rename` and a `remove` as well, and decides by whether the file is there: a file that is
gone takes its copy with it. `exclude` is read as the walk reads it.

The plugin's docs page showed a programmatic example that imported `expandProject` and
`watchProject` from the package root, where they are under `/project`, named options that do not
exist (`root` for `expand`, `out`), and awaited `expand`, which is synchronous. It is corrected
against the source.

**Breaking:** `expandProject` no longer rejects when a file's expansion throws; the file is among
the events with `failed: true`, and the CLI exits 1 as it does for a macro error. A mirror no longer
keeps the last good expansion of a file that now fails.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the plugin's `project` tests expand
a file that stops expanding and expect its copy gone, and watch one through a failing change, a good
one and its removal.
