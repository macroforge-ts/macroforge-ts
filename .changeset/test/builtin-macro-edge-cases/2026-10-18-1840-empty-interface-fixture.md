# test(core): derive on an empty interface in a fixture

- **Date:** 2026-10-18 18:40
- **Commit:** 9cb94464
- **Area:** testground and expansion fixtures

## Problem

`pixi run diagnostics` reported deno lint's no-empty-interface warning on
`tooling/testground/vanilla/src/edge-cases/exotic-shapes.ts`.

## Cause

The edge case of a type with nothing in it was written as `export interface ExoticEmpty {}` in a
file deno lint reads.

## Introduced

`c3298abc` (2026-10-12, "test(testground): cover unusual field types and the numbers decode
refuses") added `ExoticEmpty` as an empty interface.

## Fix

The empty interface is the `interface_of_no_members` expansion fixture, under `fixtures/`, which
deno lint does not read, and `ExoticEmpty` is the empty object type `{}`, whose derives the
`edge-exotics` suite still runs.

## Verified

Ran `pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the
lines that changed with the `interface_of_no_members` fixture, `pixi run diagnostics` (no deno lint
warning left), and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (48 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-exotics` derives on the empty object type.
