# fix(core)!: refuse more than one @default marker

- **Date:** 2026-10-13 07:03
- **Commit:** 8c6b9634
- **Area:** Built-in Default derive

## Problem

An enum with two members marked `@default`, or a union with two, took the first and ignored the
other.

## Cause

The default member was the first one found with the marker.

## Introduced

6146ba01 (2025-12-12, "Add utils and traits namespaces with Result and Option types") wrote the
search for the marked member of an enum and of a union.

## Fix

More than one marked member of an enum or a union is an expansion error naming them.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (734 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line with the `default_several_markers` fixture, and `pixi run test:rust` (workspace
and `tooling/testground/macro` passed).
