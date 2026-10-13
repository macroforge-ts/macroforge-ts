# test(core): pin the errors for unknown decorator options

- **Date:** 2026-10-13 01:16
- **Commit:** aa81c272
- **Area:** Conformance fixtures and snapshots

## Problem

No fixture wrote an option a decorator does not take, or a default of several values.

## Fix

The conformance error fixture `decorator_options`, with a misspelled option on each decorator, a
default of two values, and the options and defaults that stay valid beside them.

## Verified

Ran `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
shape of each changed line.
