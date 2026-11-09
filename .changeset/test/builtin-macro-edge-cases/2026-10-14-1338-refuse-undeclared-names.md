# fix(endec): refuse a field typed by a name the file neither declares nor imports

- **Date:** 2026-10-14 13:38
- **Commit:** 66056680
- **Area:** Encode and Decode of fields typed by unknown names

## Problem

```svelte
<script lang="ts" generics="T">
  /** @derive(Encode) */
  interface Held { value: T; }
</script>
```

The encoder of `Held` called `tEncodeWithContext`, which nothing provides, and so did any field
typed by a name the file neither declared nor imported and the project declared nowhere.

## Cause

A field typed by a name is coded through the functions named after it, which only a type the project
declares has.

## Introduced

`2a0d26f5` (2026-10-12, "fix(endec)!: code a value by the whole structure of its type"), on the
branch this one is stacked on, coded a named type through its functions wherever it stands.

## Fix

Where the expansion knows the file's own declarations, a field typed by a name no declaration,
import or the project gives is an expansion error naming it, the component's type parameter among
the cases, with `encodeWith`, `decodeWith` and `skip` as the ways out.

## Verified

Ran `cargo test -p macroforge_ts --features test-macros --lib` (776 passed), whose foreign-type
union tests now declare the type they name, `pixi run test:conformance:update` (10 passed), with the
regenerated snapshots reviewed by the lines that changed, in which two error fixtures declare the
types they had named without declaring, and `pixi run test:packages` (the six packages passed), in
which the Svelte preprocessor refuses a field typed by the component's type parameter, naming it.
