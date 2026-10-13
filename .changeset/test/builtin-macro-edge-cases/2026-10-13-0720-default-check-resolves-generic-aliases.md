# fix(core): check a field's default by what its generic alias stands for

- **Date:** 2026-10-13 07:20
- **Commit:** 41944fdc
- **Area:** Built-in Default derive

## Problem

`@derive(Default)` on a type with a field typed by a generic alias of a built-in,
`items:
List<string>` for `type List<T> = Array<T>`, failed with "field 'items' has type
'List<string>', which does not derive Default". The default written for such a field is the alias's
own, `[]`.

## Cause

The check that a field's type can be defaulted looked the type up by the name before its type
arguments, found the alias in the registry without a `Default` derive, and stopped there. The code
that writes the default resolves a generic alias to what it stands for first.

## Introduced

3df19fb7 (2026-10-12, "fix(core): default a Required, a Pick and an object type written in place by
their fields"), on this branch, made the check know a generic type by its name. Before it, a type
written with arguments was looked up whole, matched nothing and was let through.

## Fix

The check resolves a generic alias the way the default's writer does, with the file and imports of
the type being expanded, and judges what the alias stands for. Its message still names the type as
the field writes it.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (734 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line and `pixi run test:packages` (every package passed), in which the vite plugin's
`registry-reload` test, which defaults a field typed `List<string>` once the alias is registered,
passes after failing.
