# Deno Plugin

`@macroforge/deno-plugin` expands macros in Deno projects. Deno has no Vite-style plugin hook for
arbitrary transforms, so rather than hooking the module graph it expands sources into a cache
directory that you then run or bundle.

## CLI

```bash
deno run -A jsr:@macroforge/deno-plugin/cli expand
deno run -A jsr:@macroforge/deno-plugin/cli watch
deno run -A jsr:@macroforge/deno-plugin/cli clean
```

| Command  | Purpose                          |
| -------- | -------------------------------- |
| `expand` | Expand the project once          |
| `watch`  | Expand, then re-expand on change |
| `clean`  | Remove the output directory      |

### Options

| Flag                       | Default             | Description                                                                                          |
| -------------------------- | ------------------- | ---------------------------------------------------------------------------------------------------- |
| `--root <path>`            | current directory   | Project root to scan                                                                                 |
| `--out <path>`             | `.macroforge/cache` | Output directory for expanded files                                                                  |
| `--build-mode <dev\|prod>` | `prod`              | Expansion mode. `prod` enables reverse monomorphization for declarative macros; `dev` expands inline |
| `--copy-passthrough`       | off                 | Also copy files that contain no macros, so the output tree is complete                               |

Note that `--build-mode` defaults to `prod` here, the opposite of the Vite plugin, which follows
Vite's own dev/build state.

## Programmatic API

```typescript
import { expand, expandFile } from '@macroforge/deno-plugin';
import { expandProject, watchProject } from '@macroforge/deno-plugin/project';

// Expand a source string
const result = expand(source, 'src/user.ts', { projectRoot: Deno.cwd() });

// Read a file from disk and expand it
const fromDisk = await expandFile('src/user.ts', { projectRoot: Deno.cwd() });

// Expand every macro-using file under the root into the mirror
const events = await expandProject({ root: Deno.cwd(), outDir: '.macroforge/cache' });

// Expand, then re-expand on change; the function returned stops watching
const stop = await watchProject({
    root: Deno.cwd(),
    outDir: '.macroforge/cache',
    onFile: (event) => console.log(event.source, event.failed ? 'failed' : 'ok')
});
```

`expand` and `expandFile` return the expanded code with its diagnostics and write nothing.
`expandProject` returns an event per file it visited, and `watchProject` hands each to `onFile`:
whether the file was expanded, skipped or failed, along with its diagnostics.

A file whose macros report an error, or that cannot be read or expanded at all, has `failed` set.
Nothing is written to the mirror for it and the copy an earlier run wrote is removed, so the mirror
never holds code its source no longer expands to. The run goes on to the next file, and the `expand`
command exits with status 1. The watcher also removes the copy of a file that is deleted or renamed
away.

### Types across files

A derive knows a type declared in another file through the project's type registry, which
`macroforge cache` and `macroforge watch` write to `.macroforge/type-registry.json`. `expandProject`
and `watchProject` read it from there, and the watcher reads it again when it is rewritten. Without
it each file is expanded on its own, so a class is expanded without the fields it inherits from a
class in another file, and a warning says so once for the project. Pass `typeRegistryJson` to hand
in a registry of your own.

The config file is read on **every** `expand()` call (it is not cached), so a long-running
`watchProject` picks up `macroforge.config.ts` edits without a restart.

## Attribute macros

The Deno plugin supports the same [attribute macros](/docs/attributes) as the other integrations
(`@cfg`, `@deprecated`, `@mustUse` and `@nonExhaustive`), driven by the same `macroforge.config.ts`.

## Availability

The plugin is published on [JSR](https://jsr.io/@macroforge/deno-plugin) only. It is Deno-specific,
so there is no npm package.
