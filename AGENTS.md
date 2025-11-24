# Repository Guidelines

## Project Structure & Module Organization
- `crates/swc-napi-macros/src` contains the Rust transformer compiled into a `.node` module via `build.rs`. Update this crate first when adjusting macro behavior.
- `packages/vite-plugin/src` exposes the Vite plugin that loads the native module; keep its API aligned with the Rust bindings. Build artifacts land in `packages/*/dist`.
- `playground/` is a Vite app wired to the plugin for manual verification; drop reproduction snippets in `playground/src` when needed.
- Rust build output lives under `target/`; never commit generated binaries.

## Build, Test, and Development Commands
- `npm run dev` — launches the playground with the compiled plugin for end-to-end smoke tests.
- `npm run build:rust` — runs `napi build --platform --release` to emit the native module for the current architecture.
- `npm run build:plugin` — TypeScript compile of the Vite plugin; `npm run build` runs both build steps.
- `cargo test` (inside `crates/swc-napi-macros`) — executes Rust unit/integration tests; add new tests next to the code they cover.
- `npm test` — cascades to each workspace’s `test` script; wire Vitest or tsd suites under `packages/vite-plugin/tests` as needed.

## Coding Style & Naming Conventions
- TypeScript uses ES modules, two-space indentation, and explicit return types for exported functions like `napiMacrosPlugin`.
- Rust targets edition 2024; run `cargo fmt` and `cargo clippy --all-targets --workspace` before sending a PR.
- Prefer PascalCase for interfaces/types, camelCase for variables/functions, and kebab-case for package names. Keep transformer logic side-effect free and document non-obvious SWC passes briefly.

## Testing Guidelines
- Co-locate Rust tests in `#[cfg(test)]` modules near the logic. Cover new macro syntax or error paths and keep fixtures minimal.
- For plugin changes, add Vitest or type tests under `packages/vite-plugin/tests` and ensure `npm test` stays green. Use the playground for manual verification, but do not commit large fixtures.

## Commit & Pull Request Guidelines
- Follow “imperative, present-tense, <72 char” commit subjects (e.g., `Add SWC visit pass`).
- Each PR should summarize scope, call out affected language boundaries (Rust/TypeScript/Playground), list verification steps (`npm run build`, `cargo test`, screenshots when UI-visible), and link related issues. Use `TODO.md` for deferred work instead of bloating the PR.

## Security & Configuration Tips
- Require Node.js ≥ 18 and the toolchain defined in `rust-toolchain`. After touching Rust code, rerun `npm run build:rust` to avoid stale `.node` files.
- Do not commit secrets or build outputs; CI rebuilds native modules. When troubleshooting `napi` loading, ensure the native build succeeded for your architecture before re-running `npm run dev`.
