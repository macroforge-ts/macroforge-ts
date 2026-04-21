# @macroforge/deno-plugin

Deno-native macro expansion. For projects that don't use Vite.

Deno doesn't have a Node-style `--loader` hook, so this package takes a
file-on-disk approach: walk the project, expand every `.ts` / `.tsx` file that
contains macro annotations, and write the output to a mirror directory
(`.macroforge/cache/` by default). Point Deno at the mirror via an `imports`
entry in `deno.json`.

## CLI

```sh
# One-shot expansion
deno run -A jsr:@macroforge/deno-plugin/cli expand --root .

# Re-expand on file changes
deno run -A jsr:@macroforge/deno-plugin/cli watch --root .

# Drop the mirror
deno run -A jsr:@macroforge/deno-plugin/cli clean --root .
```

| Flag                  | Default             | Notes                                                             |
| --------------------- | ------------------- | ----------------------------------------------------------------- |
| `--root <dir>`        | `cwd`               | Project root — also where `macroforge.config.*` is discovered.    |
| `--out <dir>`         | `.macroforge/cache` | Output directory, relative to `--root`.                           |
| `--build-mode <mode>` | `prod`              | `dev` runs declarative macros expand-only for diagnostics.        |
| `--copy-passthrough`  | `false`             | Copy non-macro files to the mirror so it's a self-contained tree. |

## Programmatic

```ts
import { expand, expandFile } from "@macroforge/deno-plugin";
import { expandProject, watchProject } from "@macroforge/deno-plugin/project";

// Single string:
const { code, hasMacros, diagnostics } = expand(source, "/abs/path/User.ts");

// Whole project:
const events = await expandProject({ root: Deno.cwd(), outDir: "build" });

// Watch:
const stop = await watchProject({ root: Deno.cwd() });
// ... later:
stop();
```

## Attribute macros

Alongside the derive macros, macroforge ships four Rust-inspired attribute
macros driven by keys on `macroforge.config.*`. Each runs as a pre-pass before
`@buildtime` and derive expansion:

### `@cfg({ ... })`

Strips a declaration when the predicate doesn't match the configured flags.
Implicit AND across keys; `features` is a membership check.

```ts
// macroforge.config.ts
export default {
  cfg: {
    features: ['ssr'],
    target: 'web',
    debugAssertions: false,
    custom: { tenant: 'acme' }
  }
};

/** @cfg({ feature: 'ssr' }) */
export function render() { ... }            // kept

/** @cfg({ feature: 'experimental' }) */
export function labs() { ... }               // stripped (not in features)

/** @cfg({ target: 'web', debugAssertions: true }) */
function debugLog() { ... }                  // stripped (debugAssertions mismatch)
```

### `@deprecated('message', { since: '...' })`

Rewrites to a tsc-visible `/** @deprecated message */` so consumers see
strikethrough in IDEs. With `deprecated.failOnUse = true`, the annotation
becomes a macro-expansion error.

```ts
/** @deprecated('use render2 instead') */
export function render() { ... }
// → /** @deprecated use render2 instead */
//   export function render() { ... }
```

### `@mustUse` / `@mustUse('reason')`

Emits a build-time diagnostic at any call site whose return value is discarded
(v1: same-file top-level calls).

```ts
/** @mustUse('connection handle must be closed') */
export function openConnection() { ... }

openConnection();                 // error: return value is being discarded
const conn = openConnection();    // ok
```

### `@nonExhaustive`

Intersects a type alias's RHS with a brand sentinel so external matches require
a `default:` arm.

```ts
/** @nonExhaustive */
export type Kind = "a" | "b" | "c";
// → type Kind = ('a' | 'b' | 'c') & { readonly __nonExhaustive: unique symbol }
```

The brand name is configurable via `nonExhaustive.brand`.

## Notes

- The npm `macroforge` package ships both a NAPI (Node-native) and a WASM build.
  Deno loads the WASM build through `node:` compat; the host-side
  `setupBuildtimeFs` and `setupExternalMacros` callbacks are wired up
  automatically on the first `expand()` call.
- The pre-filter that decides whether to call the engine catches JSDoc
  annotations (`@derive`, `@cfg`, `@deprecated`, `@mustUse`, `@nonExhaustive`),
  `import macro`, and `$ident(` call macros. Anything matching gets handed to
  the engine; non-matches are skipped (or copied through with
  `--copy-passthrough`).
- For the Vite workflow, use `@macroforge/vite-plugin` instead — that plugin
  runs in-process and supports HMR, a feature with no Deno equivalent.
