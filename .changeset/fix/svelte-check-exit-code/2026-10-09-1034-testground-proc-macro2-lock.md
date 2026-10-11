# chore(testground): record the macro package's proc-macro2 dependency

- **Date:** 2026-10-09 10:34
- **Commit:** 852d0641
- **Area:** Testground

## Problem

The testground macro package's lockfile was out of date.

## Cause

Its `proc-macro2` dependency was not recorded.

## Introduced

1cc2f3c4 (2026-10-08),
`feat(core)!: release 0.5.0 with Rust-shaped derives and shared foreign handlers`. It added
`proc-macro2` to `macroforge_ts_syn` without refreshing the testground macro package's lockfile.

## Fix

The lockfile records it.

## Verified

The testground macro package builds.
