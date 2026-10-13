# fix(core): write generated code in the line endings of its file

- **Date:** 2026-10-13 16:30
- **Commit:** d6cd0f71
- **Area:** Patch application, generated imports, type output

## Problem

A source whose lines end in `\r\n` came out with its own lines ending that way and every generated
line ending in `\n`: one file with two conventions, which editors and diffs then show as changed
throughout.

## Cause

The derives write their code with `\n`, and it was inserted as written.

## Introduced

No single change: generated code has been inserted as written since the first derive.

## Fix

Where the first line of the source ends in `\r\n`, the code of each patch, the generated imports and
the `.d.ts` output are written with `\r\n`. The patches are rewritten before they are applied, so
the source mapping measures the code as it stands in the output.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`generated_code_is_written_in_the_line_endings_of_its_source` checks the code and the length its
mapping gives it, `pixi run test:conformance:update` (10 passed), with the regenerated snapshots
reviewed by the lines that changed, in which no snapshot changed, and the three groups on the
branch's final tree: `pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed), in which the `cli`
suite expands a CRLF source and finds no line of the output or of the declarations ending in a bare
LF.
