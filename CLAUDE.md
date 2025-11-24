# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a TypeScript Macros framework that implements a Rust-like procedural macro system for TypeScript. It uses SWC for AST transformations and Rust for macro implementations, connected via NAPI-RS bindings. The goal is to achieve Rust-level developer experience with `@Derive(Debug)`-style syntax and reusable macro crates.

## Architecture

### Core Components

1. **Rust Macro Host** (`crates/swc-napi-macros/`)
   - SWC-based TypeScript/JavaScript parser and transformer
   - Compiles to native `.node` module via NAPI-RS
   - Handles macro dispatch and AST manipulation
   - When modifying macro behavior, update this crate first

2. **Vite Plugin** (`packages/vite-plugin/`)
   - TypeScript wrapper that loads the native module
   - Integrates with Vite build pipeline
   - Keep API aligned with Rust bindings

3. **Macro ABI Crates** (`crates/`)
   - `ts_macro_abi/` - Stable IR and patch format (public ABI)
   - `ts_syn/` - Parse/lower SWC AST to IR (like Rust's `syn`)
   - `ts_quote/` - Codegen helpers and quasiquote (like Rust's `quote`)

4. **Extensions** (`crates/extensions/`)
   - `ts-derive-ts/` - TypeScript-specific derive macros
   - `ts-derive-svelte/` - Svelte-specific derive macros

5. **Playground Apps** (`playground/`)
   - `vanilla/` - Plain TypeScript/Vite test app
   - `svelte/` - Svelte app for testing Svelte macros
   - `debug/` - Minimal Rust debug workspace member

## Development Commands

### Building
```bash
# Full build (Rust + TypeScript)
npm run build

# Build Rust native module only
npm run build:rust
# Or directly: cd crates/swc-napi-macros && napi build --platform --release

# Build TypeScript plugin only
npm run build:plugin

# Build specific workspace
cargo build -p swc-napi-macros
cargo build -p ts_macro_abi
```

### Testing
```bash
# Run all Rust tests
cargo test --workspace

# Run tests for specific crate
cargo test -p swc-napi-macros
cargo test -p ts_syn

# Run TypeScript tests (if present)
npm test

# Run tests with output
cargo test -- --nocapture
```

### Development
```bash
# Start playground with hot reload
npm run dev

# Or start specific playground
cd playground/vanilla && npm run dev
cd playground/svelte && npm run dev

# Watch TypeScript compilation
cd packages/vite-plugin && npm run dev
```

### Code Quality
```bash
# Format Rust code
cargo fmt --all

# Lint Rust code
cargo clippy --all-targets --workspace

# Check TypeScript types
cd packages/vite-plugin && npx tsc --noEmit
```

## Key Implementation Details

### Macro Types (planned)
- **Derive macros**: `@Derive(Debug, Clone)` on classes/interfaces/enums
- **Attribute macros**: `@log`, `@sqlTable`, etc.
- **Call macros**: Future support for `macro.sql` / tagged templates

### Current Macro Flow
1. Vite plugin intercepts TypeScript files during build
2. Native module parses code with SWC
3. Identifies `@Derive` decorators and other macros
4. Transforms AST by injecting generated code
5. Returns transformed TypeScript/JavaScript

### Working with Macros

When implementing new macros:
1. Define the macro in the appropriate crate under `crates/extensions/`
2. Update the SWC transformer in `crates/swc-napi-macros/src/`
3. Rebuild the native module: `npm run build:rust`
4. Test in playground: `npm run dev`

### Project Conventions

- **Rust**: Edition 2024, use `cargo fmt` and `cargo clippy` before commits
- **TypeScript**: ES modules, explicit return types for exported functions
- **Testing**: Co-locate Rust tests in `#[cfg(test)]` modules
- **Commits**: Imperative, present-tense, <72 char subjects

### Important Files

- `TODO.md` - Comprehensive roadmap and implementation status
- `AGENTS.md` - Repository guidelines and coding standards
- `Cargo.toml` (root) - Workspace configuration
- `package.json` (root) - NPM workspace configuration

### Troubleshooting

**Native module loading issues:**
- Ensure `npm run build:rust` succeeded for your architecture
- Check Node.js version (requires >=24.9.0)
- Verify `.node` file exists in expected location

**Macro not working:**
- Check that decorator syntax is correct
- Verify native module is rebuilt after Rust changes
- Look for errors in browser console (playground)
- Use `cargo test` to verify Rust implementation

**Build failures:**
- Clear build artifacts: `cargo clean && rm -rf node_modules`
- Reinstall dependencies: `npm install`
- Rebuild everything: `npm run build`

### Security Notes

- Macros will support both WASM (sandboxed) and native execution
- Native macros require explicit opt-in via configuration
- All macro packages must be explicitly allowed in config