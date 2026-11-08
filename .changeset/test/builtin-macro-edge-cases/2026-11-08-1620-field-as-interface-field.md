# feat(syn): convert a class field to the interface member it declares

- **Date:** 2026-11-08 16:20
- **Commit:** e6648840
- **Area:** `macroforge_ts_syn` class IR, derive registry helpers

## Problem

Turning a class's fields into interface members was written out by hand wherever it was needed, and
the field `key` added to both IRs had to be remembered in every copy.

## Cause

`FieldIR` had no conversion of its own, unlike `InterfaceMethodIR::as_field`.

## Fix

`FieldIR::as_interface_field` keeps the name, key, span, type, optional and readonly flags and
decorators, and drops visibility, initializer and accessor storage. The registry's field lookup uses
it.

## Verified

`pixi run test:rust` passed.
