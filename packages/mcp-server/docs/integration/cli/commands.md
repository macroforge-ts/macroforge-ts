## Commands

### macroforge expand

Expands macros in a TypeScript file and outputs the transformed code.

Bash

```
macroforge expand <input> [options]
```

#### Arguments

| Argument  | Description                                  |
| --------- | -------------------------------------------- |
| `<input>` | Path to the TypeScript or TSX file to expand |

#### Options

| Option               | Description                                           |
| -------------------- | ----------------------------------------------------- |
| `--out <path>`       | Write the expanded JavaScript/TypeScript to a file    |
| `--types-out <path>` | Write the generated `.d.ts` declarations to a file    |
| `--print`            | Print output to stdout even when `--out` is specified |
| `--scan`             | Scan directory for TypeScript files with macros       |
| `--include-ignored`  | Include files ignored by .gitignore when scanning     |
| `-q, --quiet`        | Suppress output when no macros are found              |

#### Examples

Expand a file (writes a sibling `src/user.expanded.ts`):

Bash

```
macroforge expand src/user.ts
```

Print the expansion to stdout instead:

Bash

```
macroforge expand src/user.ts --print
```

Expand and write to a file:

Bash

```
macroforge expand src/user.ts --out dist/user.js
```

Expand with both runtime output and type declarations:

Bash

```
macroforge expand src/user.ts --out dist/user.js --types-out dist/user.d.ts
```

Note

Expansion runs natively in Rust — no Node.js process is spawned. External macro packages are loaded
from `node_modules` via FFI, so run the CLI from your project root when your code uses them.

### macroforge tsc

Runs TypeScript type checking with macro expansion. This wraps `tsc --noEmit` and expands macros
before type checking, so your generated methods are properly type-checked.

Bash

```
macroforge tsc [options]
```

#### Options

| Option                 | Description                                                                |
| ---------------------- | -------------------------------------------------------------------------- |
| `-p, --project <path>` | Path to `tsconfig.json` (defaults to `tsconfig.json` in current directory) |

#### Examples

Type check with default tsconfig.json:

Bash

```
macroforge tsc
```

Type check with a specific config:

Bash

```
macroforge tsc -p tsconfig.build.json
```

### macroforge svelte-check

Runs `svelte-check` with macro expansion, so Svelte components using macros are properly
type-checked.

Bash

```
macroforge svelte-check [options]
```

#### Options

| Option               | Description                                                           |
| -------------------- | --------------------------------------------------------------------- |
| `--workspace <path>` | Workspace directory (defaults to current directory)                   |
| `--tsconfig <path>`  | Path to `tsconfig.json`                                               |
| `--output <format>`  | Output format: `human`, `human-verbose`, `machine`, `machine-verbose` |
| `--fail-on-warnings` | Exit with error on warnings (not just errors)                         |

### macroforge svelte-package

Runs `svelte-package` with macro expansion, so a published Svelte library ships fully expanded
source and type declarations.

Bash

```
macroforge svelte-package [options]
```

#### Options

| Option                | Description                                        |
| --------------------- | -------------------------------------------------- |
| `-i, --input <path>`  | Source directory (defaults to `src/lib`)           |
| `-o, --output <path>` | Output directory (defaults to `dist`)              |
| `--tsconfig <path>`   | Path to `tsconfig.json`                            |
| `--no-types`          | Skip generating type declarations                  |
| `--full-rebuild`      | Ignore the previous build and repackage everything |

#### Incremental builds

Packaging is incremental. Each run records what it consumed and produced under
`.macroforge/svelte-package/`, and a run whose inputs all match the previous one exits without
repackaging. When a rebuild is needed, only the files that changed are re-expanded — the rest keep
the expanded output from last time.

A file that differs only in formatting — trailing whitespace, runs of blank lines — does not count
as a change. Note that `.ts` is transpiled on the way into the package so its layout is discarded
anyway, but `.svelte` and `.js` are copied through verbatim: a formatting-only edit to those will
not reach the package until the next real change or a `--full-rebuild`.

Any of these forces a full rebuild on its own:

- a changed macroforge version, `macroforge.config.*`, or external macro binary
- a changed `svelte.config.*`, `package.json`, or tsconfig
- a changed `@sveltejs/package`, `macroforge`, or `@macroforge/svelte-preprocessor` version
- different command-line options
- a changed project source outside the input directory
- an output directory that was deleted or modified behind the CLI's back

A locally rebuilt linked package whose version did not change is the one thing this cannot see;
`--full-rebuild` is the escape hatch. `macroforge refresh` also discards the build state along with
the expansion cache.

Expansion failures fail the build. A module that cannot be expanded has no correct packaged form,
and shipping its unexpanded source publishes a library whose generated runtime is silently missing.

### macroforge watch

Watches source files and maintains the macro expansion cache, keeping it up to date as files change.

Bash

```
macroforge watch [root] [options]
```

#### Options

| Option               | Description                                      |
| -------------------- | ------------------------------------------------ |
| `--debounce-ms <ms>` | Debounce interval in milliseconds (default: 100) |

### macroforge cache

Builds the `.macroforge/cache` directory once for all source files. Useful for CI or pre-build
steps.

Bash

```
macroforge cache [root] [options]
```

#### Options

| Option | Description |
| ------ | ----------- |

### macroforge refresh

Deletes and rebuilds the macro cache from scratch.

Bash

```
macroforge refresh [root] [options]
```

#### Options

| Option | Description |
| ------ | ----------- |

### macroforge build

Builds a macro crate to WebAssembly with `wasm-bindgen` and post-processes the output to add
`$`-prefixed re-exports for function-like (Call) macros. Used when distributing your own macro
packages.

Bash

```
macroforge build [crate_dir] [options]
```

Steps:

1. `cargo build --release --target wasm32-unknown-unknown`
2. Runs `wasm-bindgen --target nodejs` into `pkg/` (or the directory given via `-o`)
3. Parses the generated `.d.ts` to discover Call macros and appends
   `export { state as $state }`-style aliases so consumers can import both forms.

#### Options

| Option            | Description                                                           |
| ----------------- | --------------------------------------------------------------------- |
| `[crate_dir]`     | Path to the macro crate (defaults to `.`)                             |
| `-o, --out <out>` | Output directory for the WASM package (defaults to `<crate_dir>/pkg`) |

#### Examples

Bash

```
# Build the current macro crate
macroforge build

# Build a specific crate into a custom directory
macroforge build ./packages/my-macros -o dist/wasm
```
