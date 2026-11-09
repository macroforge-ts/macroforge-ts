# fix(cli): write the cache manifest in the order of its paths

- **Date:** 2026-10-13 18:36
- **Commit:** cf14a75c
- **Area:** CLI cache

## Problem

`macroforge cache` over the same project wrote a `manifest.json` that differed from one run to the
next: the same entries, in another order each time. Every expanded file and the type registry came
out byte for byte the same, and the manifest beside them did not, so a build that compares or stores
its cache saw a change where there was none.

## Cause

The manifest holds its entries, and the fingerprints of the engines that write it, in hash maps, and
was serialized in the order they iterate.

## Introduced

`65c22961` (2026-04-07, "Syncing monorepo") carries the manifest with its entries in a hash map.

## Fix

Both are ordered maps: the entries are written by path, the engines by name.

## Verified

Ran `macroforge cache` over a copy of the vanilla testground from nothing three times before the
fix, whose expanded files and type registry were identical and whose manifests differed in order
alone, and twice after it, whose cache directories were identical. Ran the CLI's own tests (83
passed), whose `test_manifest_entries_are_written_in_path_order` checks the order, and the three
groups on the branch's final tree: `pixi run test:rust` (the workspace and
`tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator files
of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the six
packages passed).
