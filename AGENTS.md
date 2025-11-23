# Repository Guidelines

## Project Structure & Module Organization
Rust bindings live in `crates/swc-napi-macros/src`, compiled into a `.node` artifact by `build.rs` and consumed from JavaScript via the generated `index.js`. The Vite integration lives in `packages/vite-plugin/src`, exposing the plugin that loads the Rust transformer. `playground/` is a Vite app wired to the plugin for manual verification. Build outputs land in `target/` (Rust) and `packages/*/dist`. Keep cross-language changes scoped: update the Rust crate first, then mirror any API surface in the plugin and playground.

## Build, Test, and Development Commands
- `npm run dev` — launches the Vite playground with the compiled plugin.
- `npm run build:rust` — runs `napi build --platform --release` in `crates/swc-napi-macros` to produce the native module.
- `npm run build:plugin` — TypeScript compile of `packages/vite-plugin`.
- `npm run build` — convenience wrapper for the previous two steps.
- `npm test` — forwards to every workspace’s `test` script when present; add one when introducing tests.
- `cargo test` (from `crates/swc-napi-macros`) — executes Rust unit/integration tests.

## Coding Style & Naming Conventions
TypeScript files use ES modules, two-space indentation, and explicit return types for exported functions (`napiMacrosPlugin`). Prefer PascalCase for interfaces (`NapiMacrosPluginOptions`), camelCase for variables/functions, and kebab-case for package names. Rust code targets edition 2024; run `cargo fmt` and `cargo clippy --all-targets --workspace` before opening a PR. Keep transformer logic side-effect free and document non-obvious SWC passes with concise comments.

## Testing Guidelines
Augment Rust logic with focused `#[test]` modules inside the crate and run `cargo test` plus `cargo clippy` for safety. For TypeScript, add `vitest` (or `tsd`) suites under `packages/vite-plugin/tests` when extending plugin behavior, and wire them into the package’s `npm test`. Use the `playground` for smoke-testing transformation flows; capture repro snippets in `playground/src` but avoid committing large fixtures. Aim for tests that cover new macro syntax or error paths introduced by your change.

## Commit & Pull Request Guidelines
Git history is sparse, so follow the conventional “imperative, present-tense, <72 char subject” style (e.g., `Add SWC visit pass`). Each PR should include: summary of the change, affected language boundaries (Rust/TypeScript/Playground), reproduction or verification steps (`npm run build`, `cargo test`, playground screenshots when UI-visible), and linked issues if applicable. Keep PRs narrowly scoped and note any follow-up TODOs in `TODO.md`.

## Security & Configuration Tips
Use Node.js ≥18 and a recent Rust toolchain (matching `rust-toolchain` defaults). Always rebuild the native module after touching Rust code; stale `.node` files lead to runtime load failures. Avoid committing generated artifacts or secrets—`.node` binaries are rebuilt via CI and should stay ignored. When troubleshooting `napi` loading, ensure `npm run build:rust` succeeded for your current architecture and rerun `npm run dev` from the repo root.
