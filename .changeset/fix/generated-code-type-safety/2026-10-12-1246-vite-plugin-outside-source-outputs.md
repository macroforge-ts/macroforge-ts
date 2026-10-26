# fix(vite-plugin): keep an outside source's declarations and metadata inside the project

- **Date:** 2026-10-12 12:46
- **Commit:** dad9eaa5
- **Area:** Vite plugin, declaration and metadata output

## Problem

For a source outside the project root, such as a shared package in a sibling directory, the plugin
wrote its `.d.ts` and `.macro-ir.json` outside the output directories: elsewhere in the project, or
beside it.

## Cause

Both paths were the source's project-relative path joined under the output directory. That path
starts with `..` for an outside source, and the join climbed back out.

## Introduced

bc882ce6 (2025-11-23), `Updates`, for declarations and 401ce8df (2025-11-25),
`Working on swapping
strings for ast`, for metadata. cc18a03f (2026-07-21),
`fix(packages): align public types and docs
with actual behavior`, folded the same escape under
`_external/` for the cache alone.

## Fix

The cache's containment becomes one helper that declarations and metadata go through as well.

## Verified

New test `keeps the outputs of a source outside the project root inside the project`: both files
land under `_external/` and nothing is written beside the project. It passes. The escape itself was
seen first as an `EACCES` on a path outside the project, in the test for the fix before this.
