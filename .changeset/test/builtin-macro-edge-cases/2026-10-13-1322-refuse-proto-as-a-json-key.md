# fix(endec)!: refuse `__proto__` as the key a field is coded under

- **Date:** 2026-10-13 13:22
- **Commit:** 30335758
- **Area:** Built-in Encode and Decode derives, reserved keys

## Problem

A field given the JSON key `__proto__`, `@endec({ rename: "__proto__" })`, was left out of the
encoding without a word. The same key as a union's `tag` or `content` lost the tag.

## Cause

The encoder writes a field with `result["__proto__"] = value`, and a tag in an object literal. On an
ordinary object that key is the prototype's setter: a string is ignored, and an object becomes the
prototype. A field named `__proto__` was refused; one renamed to it was not.

## Introduced

e6845936 (2026-10-12, "fix(endec)!: code what a field's type arguments, Required and shared values
left uncoded") added the check of the keys a field cannot be coded under, with `__id`, `__ref` and
the tag, and without this one.

## Fix

`__proto__` is one of the keys a field cannot be written under or read from, with the error that
names the field and the key, and `tag` and `content` cannot be it.

Breaking: a type with such a field or tag expanded before, to an encoding that lacked it.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed) and
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed with the `endec_reserved_keys` fixture, which renames a field to `__proto__` and tags a
union by it, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed). No type of the testground uses the key.
