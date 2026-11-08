# fix(syn): lower every field a class or an object type declares

- **Date:** 2026-10-12 17:24
- **Commit:** a526fc18
- **Area:** `macroforge_ts_syn` lowering, class IR

## Problem

A macro never saw some of the fields a type declares, so every built-in derive left them out without
a word:

1. A field keyed by a string or a number, as `'content-type': string` and `0: boolean` are. `Clone`
   on an interface with one returned an object without it, and `Encode` did not write it.
2. A constructor parameter property, `constructor(public name: string)`.
3. A `#private` class field.

A class's IR also did not say which class it extends: `heritage` lists the extended class and the
implemented interfaces alike.

## Cause

The lowering took a member only when its key was a plain identifier
(`PropertyKey::StaticIdentifier`), and read constructor parameters as a method's parameter text
only.

## Introduced

65c22961 (2026-04-07, "Syncing monorepo"), the first commit of this repository that carries the
lowering. The filter was kept when af0a8b4f (2026-09-28, "Remove the swc backend") moved it onto
oxc.

## Fix

A field's name is its key's: an identifier's own, or the value of a string or whole-number literal.
A computed key still names no field. A constructor parameter with `public`, `protected`, `private`
or `readonly` is a field, with its default as the initializer. A `#private` field is a field named
as written, `#secret`, so a macro that cannot reach it has to say so. `ClassIR` gains `superclass`,
the class an `extends` names, defaulting to none when absent from serialized IR.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (161 passed),
`cargo test -p macroforge_ts --features test-macros --lib` (728 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the shape
of each changed line and `pixi run test:rust` (workspace and `tooling/testground/macro` passed).
