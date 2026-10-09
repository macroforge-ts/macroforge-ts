# Macroforge

## Toolchain And Commands

- Use Pixi and Deno, not npm or pnpm. First-time CI-equivalent setup is `pixi install --locked`,
  `pixi run setup-rust`, `pixi run setup:wasm-bindgen`, then `pixi run build:mf`.
- `mf` (`tooling/scripts`) orchestrates builds, diagnostics, tests, docs, and releases. Most `pixi`
  tasks invoke `target/debug/mf`, so run `pixi run build:mf` after changing it.
- Build the checkout's CLI with `pixi run build:cli` before Rust, package, conformance, testground,
  or full-suite tests; `verify` builds it itself. `MACROFORGE_CLI` deliberately points to that
  binary, never a `macroforge` on `PATH`.
- Use `pixi run verify` only when the complete change is ready: it installs dependencies, builds in
  `tooling/deps.toml` order, applies diagnostics fixes, runs all test groups, builds the WASI
  extensions, and regenerates docs. Use `pixi run verify --check` for a non-mutating gate.
- `pixi run docs:all` generates API data and READMEs; `pixi run docs:mcp` builds the website before
  extracting MCP documentation. Do not hand-edit generated API data, MCP section indexes, or
  generated READMEs.

## Layout

- `crates/macroforge_ts` is the core Rust expansion host, native CLI, and WASM bindings. The
  expansion entry points are `expand_core.rs` and `host/expand`; syntax IR is in
  `macroforge_ts_syn`, quote/template macros in `macroforge_ts_quote`, and the derive proc macro in
  `macroforge_ts_macros`.
- NPM/JSR integrations live in `packages/`; their build dependency order is declared in
  `tooling/deps.toml`. `tooling/testground/macro` is intentionally excluded from the Rust workspace
  because it has its own WASM toolchain and lockfile.
- `tooling/testground/{vanilla,svelte,library,tests}` are consumer projects. The test runner
  rebuilds the macro package and reinstalls each app because Deno copies linked packages at install
  time.

## Tests

- Run the full groups with `pixi run test:rust`, `pixi run test:packages`,
  `pixi run test:testground`, or `pixi run test:all`. The Rust suite requires the locally built CLI
  and rebuilds the testground macro package before it runs.
- Run `pixi run test:conformance` for expansion goldens. Use `pixi run test:conformance:update` only
  to accept intentional snapshot changes, then review every resulting `.snap` diff. Set
  `MACROFORGE_CONFORMANCE_SKIP_TESTGROUND=1` only when the required testground package cannot be
  prepared.
- Testground browser tests use Node's Playwright CLI, not Deno. `pixi run test:testground` prepares
  and builds the apps; set `TESTGROUND_VANILLA_PORT` when port 3000 is occupied. Install Chromium
  from `tooling/testground/tests` with `deno task playwright:install` when required.
- The Svelte language server is a fork with fixture setup wired into `deno task pretest`; use its
  `deno task test` rather than calling Mocha directly.

## Generated And Published Surfaces

- Keep the Rust crate, Deno packages, website, and MCP docs consistent when changing exported
  macro-author APIs. Their Rust doc examples run as doctests in `pixi run test:rust`.
- Workspace Rust uses the pinned toolchain in `rust-toolchain.toml`; its WASM targets are
  `wasm32-unknown-unknown` and `wasm32-wasip1`. The Zed extensions must build for `wasm32-wasip1`,
  which `verify` checks.
