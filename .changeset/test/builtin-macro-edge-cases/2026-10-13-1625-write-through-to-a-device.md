# fix(cli): write through to a destination that is a device

- **Date:** 2026-10-13 16:25
- **Commit:** 7aca530e
- **Area:** CLI output

## Problem

`macroforge expand src/user.ts --out /dev/null` failed with "failed to create a temporary file in
/dev for /dev/null". The same went for a pipe or a terminal given as `--out` or `--types-out`.

## Cause

Every file the CLI writes goes through `write_atomic`, which writes a temporary file beside the
destination and renames it into place. A device's directory seldom takes files, and where it does
the rename would put a plain file where the device was.

## Introduced

`594b8de4` (2026-08-11, "feat(cli): expand macros through svelte-package, safely under concurrency")
routed the CLI's writes through `write_atomic`, whatever the destination is.

## Fix

A destination that is there and is neither a file nor a directory is opened and written to. What
reads a stream has no half-written file to see, which is all the rename is for.

The first version asked the path what it names and then opened it by the path. A commit security
review found that a destination changed between the two, for a link to a file, would be written
through. The fixup asks the opened handle what it is, and a file goes by the rename.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --bin macroforge write_atomic` (5 passed),
whose two new tests write to `/dev/null`, which stays a device, and through a link to a file, which
is replaced and leaves the file it pointed at as it was, and the three groups on the branch's final
tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which the `cli`
suite expands with `--out /dev/null`.
