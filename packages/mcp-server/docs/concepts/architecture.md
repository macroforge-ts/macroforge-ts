# Architecture

Macroforge is built as a modular Rust engine with multiple output targets. It parses and generates
TypeScript with oxc, and compiles to WebAssembly, which runs in every JavaScript environment.

## Overview

JavaScript Environments

Node.js

Vite

Browser

Edge

Bindings

wasm-bindgen

Unified API

CoreEngine

Macro Crates

macroforge\_ts\_syn

macroforge\_ts\_quote

macroforge\_ts\_macros

oxc

TypeScript parsing & codegen

## Core Components

### Unified API & Core Engine

At the heart of Macroforge is `CoreEngine`, an output-agnostic facade that the WebAssembly bindings
and the `macroforge` CLI delegate to, so the expansion logic is identical wherever it runs.

### Bindings

**wasm-bindgen** compiles the engine to WebAssembly, which runs in Node, Deno, browsers and edge
workers.

### oxc

The foundation layer provides:

- Fast TypeScript/JavaScript parsing
- AST representation
- Code generation (AST → source code)

### macroforge\_ts\_syn

A Rust crate that provides:

- TypeScript-specific AST types
- Parsing utilities for macro input
- Derive input structures (class fields, decorators, etc.)

### macroforge\_ts\_quote

Template-based code generation similar to Rust's `quote!`:

- `ts_template!` - Generate TypeScript code from templates
- `ts_template!(Within &lbrace; … &rbrace;)` - Generate class body members
- Control flow: `{"{#for}"}`, `{"{#if}"}`, `{"{$let}"}`

### macroforge\_ts\_macros

The procedural macro attributes for defining custom macros:

- `#[ts_macro_derive(Name)]` for derive macros
- `#[ts_macro_attribute(name)]` for attribute macros
- `#[ts_macro(name)]` for call macros
- Automatic registration with the macro system
- Error handling and span tracking

### wasm-bindgen Bindings

Bridges the WebAssembly engine and JavaScript:

- Exposes `expandSync`, `checkSyntax`, `scanProjectSync`, etc.
- Provides the `NativePlugin` class for caching
- Handles data marshaling between Rust and JavaScript

## Data Flow

1\. Source Code

TypeScript with @derive

2\. wasm-bindgen

receives JavaScript string

3\. oxc Parser

parses to AST

4\. Macro Expander

finds @derive decorators

5\. For Each Macro

extract data, run macro, generate AST nodes

6\. Merge

generated nodes into AST

7\. oxc Codegen

generates source code

8\. Return

to JavaScript with source mapping

## Performance Characteristics

- **Isolated Execution**: In the `macroforge` CLI, each expansion runs in a dedicated thread with a
  32MB stack to prevent stack overflow during deep recursion. In WASM, execution is synchronous on
  the main thread.
- **Caching**: `NativePlugin` and API calls support version-based caching to skip redundant work.
- **Binary search**: Position mapping uses optimized O(log n) lookups.
- **Arena allocation**: oxc parses into an arena, so AST nodes borrow from one allocation rather
  than each owning their own.

## Re-exported Crates

For custom macro development, `macroforge_ts` re-exports everything you need:

Rust

```
// Convenient re-exports for macro development
use macroforge_ts::macros::{ts_macro, ts_macro_attribute, ts_macro_derive, ts_quote, ts_template};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

// Also available: the AST crate itself
use macroforge_ts::ts_syn::oxc;
```

## Next Steps

- [Write custom macros](../../docs/custom-macros)
- [Explore the API reference](../../docs/api)
