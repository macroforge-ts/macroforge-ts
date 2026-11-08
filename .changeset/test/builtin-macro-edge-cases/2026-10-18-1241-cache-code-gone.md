# fix(cli): expand again a file whose cached code is gone

- **Date:** 2026-10-18 12:41
- **Commit:** 635dc2e2
- **Area:** `macroforge` CLI

## Problem

When a file's cached code was removed while its manifest entry stayed, `macroforge cache` skipped
the file on every later run, its source unchanged, and the cache stayed without its expansion. The
testground's helper then imported the file's source unexpanded.

## Cause

`warm_cache` skipped a file whose entry's source hash matched, without looking for the code the
entry stands for.

## Introduced

`96b1e424` (2026-03-03, "Add watch and cache CLI commands") skipped a file by its source hash.

## Fix

An entry for a file with macros is skipped only while its cached code is there.

## Verified

Ran the CLI integration test that caches a project twice, removing the cached code of a file after
each run, and finds the code written again. Found by a corpus run whose shared file had no cached
code after an earlier run's code was removed.
