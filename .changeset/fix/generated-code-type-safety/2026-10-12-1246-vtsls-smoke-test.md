# fix(typescript-plugin): run the vtsls smoke test

- **Date:** 2026-10-12 12:46
- **Commit:** 9d52e36b
- **Area:** TypeScript plugin tests

## Problem

`vtsls + typescript-plugin survives file switching` was skipped on every run.

## Cause

The test starts vtsls from the workspace's `node_modules` and skips when it is not there. Only the
testground's tests project declared the dependency, and nothing in that project uses it.

## Introduced

5d6d0432 (2025-12-06), `Update plugin to tsserver-plugin-macroforge and remove unused docs`. It
removed vtsls from the root manifest and left the test looking for it there.

## Fix

vtsls is a dev dependency of the plugin, at 0.3.0, and no longer one of the testground's tests
project. The test fails without it and removes the fixture directory it creates.

## Verified

The test ran and passed. Before the change the plugin's suite reported it skipped: 50 pass, 1
skipped.
