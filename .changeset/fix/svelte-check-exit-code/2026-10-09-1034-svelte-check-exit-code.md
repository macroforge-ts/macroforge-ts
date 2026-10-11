# fix(cli): keep svelte-check's exit code through the output rewrite

- **Date:** 2026-10-09 10:34
- **Commit:** 9a01abae
- **Area:** CLI, svelte-check wrapper, testground

## Problem

The CLI's svelte-check command exited 0 with type errors, so every check passed.

## Cause

svelte-check 4.7.6 exits from the callback of an empty write, so its output drains first. The
wrapper's rewritten write returned without calling the callback whenever a write held no complete
line, the process ran out of work, and it exited 0.

## Introduced

2aabc445 (2026-10-08), `feat(core)!: add $Newtype brands and check primitive values on decode`. It
made the wrapper rewrite svelte-check's output line by line, returning early on a write with no
complete line. The testground's svelte-check 4.3.5 exits directly, so nothing caught it.

## Fix

A partial write forwards an empty write with its arguments. The testground moves from svelte-check
4.3.5, which exits directly, to 4.7.6.

## Verified

New test `svelte-check: a type error exits non-zero in every format`: red with the previous CLI,
green with this one (7 of 7 svelte-check tests).
