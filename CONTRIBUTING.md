# Contributing to Macroforge

## Project structure

```
crates/                          # Rust workspace
  macroforge_ts/                 # Core macro expansion engine (WASM + CLI)
  macroforge_ts_syn/             # TypeScript syntax types and IR
  macroforge_ts_quote/           # ts_quote! and ts_template! macros
  macroforge_ts_macros/          # #[ts_macro_derive] proc macro
  extensions/
    svelte-macroforge/           # Zed editor extension
    vtsls-macroforge/            # Language server extension
packages/                        # NPM packages (TypeScript/Deno)
  vite-plugin/                   # Vite integration
  typescript-plugin/             # TypeScript language service plugin
  svelte-language-server/        # Svelte IDE support
  svelte-preprocessor/           # Svelte preprocessor
  mcp-server/                    # Claude MCP server
  shared/                        # Shared utilities
tooling/
  scripts/                       # `mf` CLI tool (Rust)
  playground/                    # Demo projects (macro, svelte, vanilla)
  tests/                         # E2E tests
website/                         # Documentation site (SvelteKit)
```

## Prerequisites

- [Pixi](https://pixi.sh) -- task runner and environment manager
- [Deno](https://deno.com) -- used for build coordination
- Rust toolchain (stable, 2024 edition)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) -- for building the WASM output

Build the `mf` CLI first (many pixi tasks depend on it):

```bash
pixi run build:mf
```

## Building

```bash
# Build the WASM package (core engine)
pixi run build:rust

# Build the Vite plugin
pixi run build:plugin

# Build both
pixi run build
```

## Running tests

### Rust tests

```bash
# All Rust tests
pixi run test:rust

# Or directly with cargo
cargo test -p macroforge_ts
```

### Snapshot tests

Snapshot tests use the [insta](https://insta.rs) crate with a fixture-based system.

```bash
# Run snapshot tests
cargo test -p macroforge_ts --test spec_tests

# Accept new/changed snapshots
INSTA_UPDATE=always cargo test -p macroforge_ts --test spec_tests

# Or use cargo-insta for interactive review
cargo install cargo-insta
cargo insta test -p macroforge_ts
cargo insta review
```

#### Adding a snapshot test

1. Create a `.ts` file in `crates/macroforge_ts/tests/fixtures/ok/` (expected to expand successfully) or `tests/fixtures/error/` (edge cases, bailouts, unknown macros)
2. Run `cargo test -p macroforge_ts --test spec_tests` -- the test will fail and create a `.snap.new` file
3. Review the snapshot, then accept: `INSTA_UPDATE=always cargo test -p macroforge_ts --test spec_tests`
4. Commit both the fixture and the `.snap` file

Fixtures in `ok/` must contain `@derive` annotations and are expected to produce `changed == true`. Fixtures in `error/` accept any outcome and snapshot whatever happens.

### Package tests

```bash
pixi run test:packages
```

### All tests

```bash
pixi run test:all
```

## Architecture

The core expansion pipeline:

1. **Input** -- TypeScript source with `/** @derive(Debug, Clone, ...) */` JSDoc decorators
2. **Parse** -- OXC (default) or SWC parses the TypeScript into an AST
3. **Lower** -- AST is lowered to `ClassIR`, `InterfaceIR`, `EnumIR`, `TypeAliasIR`
4. **Dispatch** -- `MacroDispatcher` routes each derive name to its registered macro
5. **Expand** -- Each macro produces `Patch` objects (code insertions)
6. **Emit** -- Patches are applied to produce expanded `.ts` and `.d.ts` output

Key types:
- `MacroExpander` (`host/expand/mod.rs`) -- entry point, call `expand_source(code, filename)`
- `MacroExpansion` -- result struct with `code`, `type_output`, `diagnostics`, `changed`
- `ClassIR` / `InterfaceIR` / `EnumIR` / `TypeAliasIR` (`macroforge_ts_syn`) -- intermediate representations

## Writing a built-in macro

Built-in macros live in `crates/macroforge_ts/src/builtin/`. Each macro implements the expansion trait and is registered via `inventory`. See the existing `Debug` or `Clone` macros for the pattern.

## Code style

- Rust 2024 edition
- Default features: `wasm` + `oxc`
- Don't suppress warnings with `#[allow(...)]` -- fix the root cause
- Don't add `TODO` comments unless you intend to leave them as-is

## Useful commands

```bash
pixi run diagnostics         # Run project diagnostics
pixi run verify              # Check release readiness
pixi run docs:all            # Generate all documentation
pixi run scripts             # Interactive TUI dashboard
```
