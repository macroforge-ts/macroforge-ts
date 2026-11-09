# fix(syn): keep the type-name classifiers macros outside the crate call

- **Date:** 2026-11-08 16:20
- **Commit:** b69871db
- **Area:** `macroforge_ts_syn::type_normalize`

## Problem

A derive crate built on `ts_syn` that calls `type_normalize::is_ts_builtin_type` to tell a built-in
generic from a type the user declared no longer compiled.

## Cause

The module was removed as unused, which looked only at callers inside the workspace; it is part of
`ts_syn`'s public API.

## Introduced

`cd7766e3` (2026-10-13, "fix(core)!: refuse a field Default has no value for") removed
`type_normalize` with the Default rework.

## Fix

`is_ts_builtin_type` and `is_ts_object_utility_type` are restored with unit tests.
`parse_index_signature` and `base_type_name` had no caller and stay removed.

## Verified

`pixi run test:rust` passed, and a downstream derive crate calling `is_ts_builtin_type` compiled
against the patched crate.
