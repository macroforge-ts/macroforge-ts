# Architecture

*Macroforge is built as a native Node.js module using Rust and NAPI-RS. It leverages SWC for fast TypeScript parsing and code generation.*

## Overview

```text
┌─────────────────────────────────────────────────────────┐
│                    Node.js / Vite                        │
├─────────────────────────────────────────────────────────┤
│                   NAPI-RS Bindings                       │
├─────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────┐  ┌───────────────┐   │
│  │   ts_syn    │  │   ts_quote   │  │ts_macro_derive│   │
│  │  (parsing)  │  │ (templating) │  │  (proc-macro) │   │
│  └─────────────┘  └──────────────┘  └───────────────┘   │
├─────────────────────────────────────────────────────────┤
│                    SWC Core                              │
│            (TypeScript parsing & codegen)                │
└─────────────────────────────────────────────────────────┘
```

## Core Components

### SWC Core

The foundation layer provides:

- Fast TypeScript/JavaScript parsing

- AST representation

- Code generation (AST → source code)

### ts_syn

A Rust crate that provides:

- TypeScript-specific AST types

- Parsing utilities for macro input

- Derive input structures (class fields, decorators, etc.)

### ts_quote

Template-based code generation similar to Rust's `quote!`:

- `ts_template!` - Generate TypeScript code from templates

- `body!` - Generate class body members

- Control flow: `{"{#for}"}`, `{"{#if}"}`, `{"{%let}"}`

### ts_macro_derive

The procedural macro attribute for defining derive macros:

- `#[ts_macro_derive(Name)]` attribute

- Automatic registration with the macro system

- Error handling and span tracking

### NAPI-RS Bindings

Bridges Rust and Node.js:

- Exposes `expandSync`, `transformSync`, etc.

- Provides the `NativePlugin` class for caching

- Handles data marshaling between Rust and JavaScript

## Data Flow

```text
1. Source Code (TypeScript with @derive)
   │
   ▼
2. NAPI-RS receives JavaScript string
   │
   ▼
3. SWC parses to AST
   │
   ▼
4. Macro expander finds @derive decorators
   │
   ▼
5. For each macro:
   │  a. Extract class/interface data
   │  b. Run macro function
   │  c. Generate new AST nodes
   │
   ▼
6. Merge generated nodes into AST
   │
   ▼
7. SWC generates source code
   │
   ▼
8. Return to JavaScript with source mapping
```

## Performance Characteristics

- **Thread-safe**: Each expansion runs in an isolated thread with a 32MB stack

- **Caching**: `NativePlugin` caches results by file version

- **Binary search**: Position mapping uses O(log n) lookups

- **Zero-copy**: SWC's arena allocator minimizes allocations

## Re-exported Crates

For custom macro development, `macroforge_ts` re-exports everything you need:

```rust
// All available via macroforge_ts::*
pub extern crate ts_syn;         // AST types, parsing
pub extern crate ts_quote;       // Code generation templates
pub extern crate ts_macro_derive; // #[ts_macro_derive] attribute
pub extern crate inventory;       // Macro registration
pub extern crate serde_json;      // Serialization
pub extern crate napi;            // Node.js bindings
pub extern crate napi_derive;     // NAPI proc-macros

// SWC modules
pub use ts_syn::swc_core;
pub use ts_syn::swc_common;
pub use ts_syn::swc_ecma_ast;
```

## Next Steps

- [Write custom macros](/docs/custom-macros)

- [Explore the API reference](/docs/api)