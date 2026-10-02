# Contributing to Macroforge

## Project structure

```
crates/                          # Rust workspace
  macroforge_ts/                 # Core macro expansion engine (WASM + CLI)
  macroforge_ts_syn/             # TypeScript syntax types and IR
  macroforge_ts_quote/           # ts_quote! and ts_template! macros
  macroforge_ts_macros/          # #[ts_macro_derive] proc macro
  extensions/
    svelte_macroforge/           # Zed editor extension
    vtsls_macroforge/            # Language server extension
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

# Or directly with cargo. The snapshot suites run the test-only macros, so they
# need the `test-macros` feature; `pixi run test:rust` passes it.
cargo test -p macroforge_ts --features test-macros
```

### Snapshot tests

The conformance suite (`crates/macroforge_ts/tests/conformance.rs`) snapshots with
[insta](https://insta.rs). It expands every fixture and every source file of the playground projects
the way the integrations expand them, and compares the expanded code, declarations, diagnostics,
source mapping, metadata and registry reads against committed goldens. It also snapshots each
playground project's registries and checks that a registry survives a JSON round trip. A change that
only makes expansion faster moves no golden.

```bash
# Run the conformance suite
pixi run test:conformance

# Accept new/changed snapshots, then review them in the diff
pixi run test:conformance:update

# Or use cargo-insta for interactive review
cargo install cargo-insta
cargo insta test -p macroforge_ts --features test-macros --test conformance
cargo insta review
```

The playground corpus resolves the playground's external macro package from each app's
`node_modules`, so it needs the macro package built and installed. `pixi run test:rust` prepares it
when it is missing (after `pixi run build:cli`); `MACROFORGE_CONFORMANCE_SKIP_PLAYGROUND=1` skips
the playground corpus when that is impossible, such as offline.

#### Adding a snapshot test

1. Create a `.ts` file in `crates/macroforge_ts/tests/fixtures/ok/` (expected to expand
   successfully) or `tests/fixtures/error/` (edge cases, bailouts, unknown macros)
2. Run `pixi run test:conformance` -- the test will fail and create a `.snap.new` file
3. Review the snapshot, then accept: `pixi run test:conformance:update`
4. Commit both the fixture and the `.snap` file

Fixtures in `ok/` must contain `@derive` annotations and are expected to produce `changed == true`.
Fixtures in `error/` accept any outcome and snapshot whatever happens.

### Package tests

```bash
pixi run test:packages
```

### Playground tests

```bash
pixi run test:playground
```

The end-to-end step serves the vanilla playground on port 3000. Point it somewhere else when that
port is already taken, or Playwright attaches to whatever is listening and every test fails:

```bash
PLAYGROUND_VANILLA_PORT=3177 pixi run test:playground
```

### All tests

```bash
pixi run test:all
```

## Architecture

The core expansion pipeline:

1. **Input** -- TypeScript source with `/** @derive(Debug, Clone, ...) */` JSDoc decorators
2. **Parse** -- OXC parses the TypeScript into an AST
3. **Lower** -- AST is lowered to `ClassIR`, `InterfaceIR`, `EnumIR`, `TypeAliasIR`
4. **Dispatch** -- `MacroDispatcher` routes each derive name to its registered macro
5. **Expand** -- Each macro produces `Patch` objects (code insertions)
6. **Emit** -- Patches are applied to produce expanded `.ts` and `.d.ts` output

Key types:

- `MacroExpander` (`host/expand/mod.rs`) -- entry point, call `expand_source(code, filename)`
- `MacroExpansion` -- result struct with `code`, `type_output`, `diagnostics`, `changed`
- `ClassIR` / `InterfaceIR` / `EnumIR` / `TypeAliasIR` (`macroforge_ts_syn`) -- intermediate
  representations

## Writing a built-in macro

Built-in macros live in `crates/macroforge_ts/src/builtin/`. Each macro implements the expansion
trait and is registered via `inventory`. See the existing `Debug` or `Clone` macros for the pattern.

## Code style

- Rust 2024 edition
- Default features: `wasm` + `buildtime-boa`
- Don't suppress warnings with `#[allow(...)]` -- fix the root cause
- Don't add `TODO` comments unless you intend to leave them as-is

## Useful commands

```bash
pixi run diagnostics         # Run project diagnostics
pixi run docs:all            # Generate all documentation
```

## Releasing

Every merge request into `main` runs `pixi run verify --check` in CI. Locally, `verify` formats the
tree, applies lint fixes and regenerates the docs; with `--check` it writes nothing and fails when a
file is unformatted or a committed generated file differs from its source, which
`pixi run
docs:check` also reports on its own. A release is one version for every package:

```bash
pixi run bump                # On a branch: raise the version and everything stamped with it
pixi run verify              # Regenerate what the bump changed, then open the merge request
pixi run tag                 # After the merge, on main: tag vX.Y.Z and push the tag
```

`pixi run bump` increments the patch version; `pixi run bump --version X.Y.Z` sets one, and
rerunning it finishes an interrupted bump. When a release pipeline fails and needs a new commit on
`main`, `pixi run tag --retag` moves the tag to it and pushes it again.

The tag's pipeline checks that it is on `main` and matches `crates/macroforge_ts/package.json`,
verifies again, and publishes to crates.io, npm and JSR whatever is not there yet.
`pixi run publish` does the same from this machine.

CI jobs run on a self-hosted GitLab runner tagged `mac-docker`, in the image `pixi run ci:image`
builds from `tooling/ci/Dockerfile` on the runner's host. Rebuild it after changing that file; jobs
refuse an image built from an older one. The release jobs run on `mac-docker-release`, a protected
runner in the same `gitlab-runner` container with cache volumes of its own, so nothing a branch
pipeline writes reaches a release build.

crates.io and npm authenticate the release jobs through trusted publishing, and JSR through the
protected `JSR_TOKEN` CI/CD variable. npm accepts trusted publishing from GitLab-hosted runners
only, so the tag pipeline publishes the crates and builds the packages on `mac-docker-release`, then
publishes the npm and JSR packages from a GitLab-hosted runner with the `mf` it built.
