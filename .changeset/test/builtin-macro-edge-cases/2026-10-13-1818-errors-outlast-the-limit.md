# fix(core): keep an error through the limit on diagnostics

- **Date:** 2026-10-13 18:18
- **Commit:** 74eeb4a9
- **Area:** Expansion host

## Problem

An expansion reports a hundred diagnostics at most, and the rest were cut from the end, whatever
they were. An error that followed a hundred warnings was cut with them: the expansion then counted
no error, and every integration wrote its output as that of one that succeeded.

The notice left in their place said to adjust `limits.maxDiagnostics` in `macroforge.config.ts`, a
setting nothing reads.

## Cause

`enforce_diagnostic_limit` truncated the list as it stood and added a warning, without looking at
what it cut.

## Introduced

`c3d3646f` (2025-11-24, "updates") added the limit with the truncation from the end and the notice
that names the setting.

## Fix

The errors are kept ahead of the rest, so warnings give way to them. The notice says how many
diagnostics are not shown and how many of them are errors, and is itself an error when an error is
among them, so an expansion that failed is never taken for one that did not. It no longer points at
a setting that does not exist.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`diagnostic_limit` tests keep an error that follows twelve warnings and make the notice an error
where errors are cut, and expanded a file of 150 types that each fail through the CLI: 99 errors and
the notice for the other 51, with exit status 1. Ran the three groups on the branch's final tree:
`pixi run test:rust` (the workspace and `tooling/testground/macro` passed),
`pixi run test:testground` (43 Deno suites, 13 validator files of 638 steps and 236 browser tests in
Chromium, all passed) and `pixi run test:packages` (the six packages passed).
