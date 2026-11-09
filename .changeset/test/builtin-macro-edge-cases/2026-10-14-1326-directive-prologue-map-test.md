# test(vite-plugin): keep a directive prologue first and its lines mapped

- **Date:** 2026-10-14 13:26
- **Commit:** b0e1f29a
- **Area:** Vite plugin tests

## Problem

No test of the Vite plugin expanded a file that opens with a directive and a license header, which
the generated imports have to follow, and checked the map of the lines after them.

## Fix

A `source-map` test expands such a file, finds `'use client'` first, and traces a field and the
export after the generated imports back to their own lines.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the new test passed.
