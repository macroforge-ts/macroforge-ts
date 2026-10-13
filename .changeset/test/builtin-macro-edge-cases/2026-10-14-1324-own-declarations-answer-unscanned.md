# fix(core): let a file's own declarations answer for it where the scan did not read it

- **Date:** 2026-10-14 13:24
- **Commit:** a8c20279
- **Area:** Expansion of a file the scan has not read

## Problem

```ts
/** @derive(Clone, Default) */
export class Base {
    id: string = '';
}

/** @derive(Clone, Default) */
export class Sub extends Base {
    name: string = '';
}
```

Expanded without a scan of the project, as the TypeScript plugin and a single expansion do, or in a
component or a file newer than the scan, the statics of `Sub` lacked the `override` their
counterparts on `Base` need under `noImplicitOverride`, and a field's type declared in the same file
went unresolved: a cycle of defaults or two fields under one key went unreported there.

## Cause

The registry holds what the scan read, and a name the file being expanded declares resolved to the
file's scanned entry, or to none where the scan had not read the file.

## Introduced

`de62444c` (2026-10-13, "fix(core): let a file's own declaration answer for its name, scanned or
not"), on the branch this one is stacked on, made such a name resolve to none, where before it the
type the project declared under it answered.

## Fix

The expander hands the registry the declarations the file holds, which answer for its names where
the scan has no entry of them.

## Verified

Ran `cargo test -p macroforge_ts_syn --lib` (195 passed), whose
`a_name_a_file_declares_is_not_another_files_type` resolves a component's own type to its own
declaration and a scanned file's to its entry,
`cargo test -p macroforge_ts --features test-macros --lib` (776 passed),
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, in which three error fixtures report the cycle and the colliding keys a scan reported
and the named members of unions are told by their fields, and `pixi run test:packages` (the six
packages passed), in which the TypeScript plugin finds no `override` missing on a subclass of a base
in its file.
