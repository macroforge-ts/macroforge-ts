# macroforge_ts

TypeScript macro expansion engine - write compile-time macros in Rust

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts.svg)](https://crates.io/crates/macroforge_ts)
[![Documentation](https://docs.rs/macroforge_ts/badge.svg)](https://docs.rs/macroforge_ts)

## Overview

This crate provides a TypeScript macro expansion engine that brings Rust-like derive macros
to TypeScript. It supports multiple output targets via feature flags:
- `wasm`: (Default) Universal WebAssembly module via wasm-bindgen for browser and edge environments.
- `node`: Optional native Node.js bindings via NAPI-RS.

## Overview

Macroforge processes TypeScript source files containing `@derive` decorators and expands them
into concrete implementations. For example, a class decorated with `@derive(Debug, Clone)`
will have `toString()` and `clone()` methods automatically generated.

## Architecture

The crate is organized into several key components:

- **Unified API** (`api` module): An output-agnostic trait-based interface (`MacroforgeApi`)
  that defines all macro operations.
- **Target Bindings**:
  - `bindings_napi`: Node.js specific entry points using NAPI-RS.
  - `bindings_wasm`: Universal entry points using `wasm-bindgen`.
- **Position Mapping** (`api_types::SourceMappingResult`): Bidirectional source mapping
  for IDE integration.
- **Macro Host** (`host` module): Core expansion engine with registry and dispatcher.
- **Built-in Macros** (`builtin` module): Standard derive macros (Debug, Clone, Serialize, etc.).

## Usage

### From Node.js

```javascript
const { expandSync } = require('macroforge');
const result = expandSync(code, filepath, { keep_decorators: false });
```

### From WASM

```javascript
import init, { expand_sync } from './pkg/macroforge_ts.js';
await init();
const result = expand_sync(code, filepath, { keep_decorators: false });
```

## Re-exports for Macro Authors

This crate re-exports several dependencies for convenience when writing custom macros:
- `ts_syn`: TypeScript syntax types for AST manipulation
- `macros`: Macro attributes and quote templates
- `swc_core`, `swc_common`, `swc_ecma_ast`: SWC compatibility infrastructure

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
macroforge_ts = "0.1.79"
```

## Key Exports

### Functions

- **`__macroforge_ffi_free`** - Free a buffer allocated by an FFI function.
- **`__macroforge_ffi_get_manifest`** - Returns the full MacroManifest as JSON via FFI.

## API Reference

See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/macroforge_ts) on
the Macroforge website.

## License

MIT
