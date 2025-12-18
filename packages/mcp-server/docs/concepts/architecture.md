# Architecture

Macroforge is built as a native Node.js module using Rust and NAPI-RS. It leverages SWC for fast TypeScript parsing and code generation.

## Overview

Node.js / Vite

NAPI-RS Bindings

Macro Crates

macroforge\_ts\_syn

macroforge\_ts\_quote

macroforge\_ts\_macros

SWC Core

TypeScript parsing & codegen

## Core Components

### SWC Core

The foundation layer provides:

*   Fast TypeScript/JavaScript parsing
*   AST representation
*   Code generation (AST → source code)

### macroforge\_ts\_syn

A Rust crate that provides:

*   TypeScript-specific AST types
*   Parsing utilities for macro input
*   Derive input structures (class fields, decorators, etc.)

### macroforge\_ts\_quote

Template-based code generation similar to Rust's `quote!`:

*   `ts_template!` - Generate TypeScript code from templates
*   `body!` - Generate class body members
*   Control flow: `{"{#for}"}`, `{"{#if}"}`, `{"{$let}"}`

### macroforge\_ts\_macros

The procedural macro attribute for defining derive macros:

*   `#[ts_macro_derive(Name)]` attribute
*   Automatic registration with the macro system
*   Error handling and span tracking

### NAPI-RS Bindings

Bridges Rust and Node.js:

*   Exposes `expandSync`, `transformSync`, etc.
*   Provides the `NativePlugin` class for caching
*   Handles data marshaling between Rust and JavaScript

## Data Flow

1\. Source Code

TypeScript with @derive

2\. NAPI-RS

receives JavaScript string

3\. SWC Parser

parses to AST

4\. Macro Expander

finds @derive decorators

5\. For Each Macro

extract data, run macro, generate AST nodes

6\. Merge

generated nodes into AST

7\. SWC Codegen

generates source code

8\. Return

to JavaScript with source mapping

## Performance Characteristics

*   **Thread-safe**: Each expansion runs in an isolated thread with a 32MB stack
*   **Caching**: `NativePlugin` caches results by file version
*   **Binary search**: Position mapping uses O(log n) lookups
*   **Zero-copy**: SWC's arena allocator minimizes allocations

## Re-exported Crates

For custom macro development, `macroforge_ts` re-exports everything you need:

Rust

```
// Convenient re-exports for macro development
use macroforge_ts::macros::{ts_macro_derive, body, ts_template, above, below, signature};
use macroforge_ts::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

// Also available: raw crate access and SWC modules
use macroforge_ts::swc_core;
use macroforge_ts::swc_common;
use macroforge_ts::swc_ecma_ast;
```

## Next Steps

*   [Write custom macros](../../docs/custom-macros)
*   [Explore the API reference](../../docs/api)