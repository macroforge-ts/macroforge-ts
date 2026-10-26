# fix(testground): run the svelte browser suite with the testground tests

- **Date:** 2026-10-12 13:47
- **Commit:** 69d62b63
- **Area:** Testground tests

## Problem

`mf test testground` built the svelte app for the browser tests and never ran them.

## Cause

Its browser step is the `test:e2e` task, which was `playwright test` with no config. Playwright then
reads `playwright.config.ts`, the vanilla suite's, and the svelte suite has its own.

## Introduced

39503eee (2026-04-12), `Auto-resolve $ call macros from imports; fix block-body IIFE returns`, which
gave the svelte suite its own config and task without adding it to `test:e2e`.

## Fix

`test:e2e` runs the vanilla task and then the svelte one.

## Verified

`deno task test:e2e:svelte` passes, 109 of 109, and `deno task test:e2e:vanilla` passes, 126 of 126.
The combined task has not been run as one command.
