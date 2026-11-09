# fix(deno-plugin): expand a project with the registry the CLI wrote for it

- **Date:** 2026-10-13 18:41
- **Commit:** dc5bf385
- **Area:** Deno plugin project expansion and watcher

## Problem

```ts
// base.ts
export class Base {
    id: string = '';
}

// child.ts
/** @derive(Encode) */
export class Child extends Base {
    name: string = '';
}
```

`deno run -A jsr:@macroforge/deno-plugin/cli expand` wrote a `Child` whose encoder left `id` out.
Every file was expanded as if it were the only one: a class without the fields it inherits from
another file, two types of one name not told apart, a type behind a barrel not found, and nothing
said of any of it.

## Cause

`expandProject` and `watchProject` hand the engine a type registry when their caller hands them one,
and the plugin's own CLI never does. The engine the plugin runs is the WebAssembly one, whose
project scan reads no files (it returns an empty registry and a warning), so the plugin had no way
to make one either.

## Introduced

`d9f76335` (2026-04-20, "Add @macroforge/deno-plugin + four Rust-inspired attribute macros") wrote
the project expansion without a registry.

## Fix

Both read the registries `macroforge cache` and `macroforge watch` write under `.macroforge/` when
their caller hands none in, as the Vite plugin does, and the watcher reads them again when they are
rewritten. A project without them is expanded as before, and a warning says once what is lost and
how to write them. A registry that lists files outside the project was scanned at another location:
it is left out, and said. The docs page describes this under "Types across files".

## Verified

Probed a project of a base class and a subclass in two files before the fix: the subclass was
encoded without the inherited field, and the engine's own scan read no file from Deno. Ran
`pixi run test:packages` (the six packages passed), in which the plugin's `project` test expands
that project without a registry, expects the warning and the field left out, writes the registry
with the checkout's CLI, and expects the field encoded.
