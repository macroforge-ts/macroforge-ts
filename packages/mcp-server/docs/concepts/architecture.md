# Architecture
 *Macroforge is built as a native Node.js module using Rust and NAPI-RS. It leverages SWC for fast TypeScript parsing and code generation.*
 ## Overview
 <div class="border border-border bg-card p-4 text-center rounded-t-lg  "><div class="font-semibold text-foreground">Node.js / Vite <div class="font-semibold text-foreground">NAPI-RS Bindings <div class="font-semibold text-foreground">Macro Crates <div class="px-3 py-1.5 bg-muted rounded text-sm text-muted-foreground font-mono">macroforge_ts_synmacroforge_ts_quotemacroforge_ts_macros<div class="font-semibold text-foreground">SWC Core <div class="px-3 py-1.5 bg-muted rounded text-sm text-muted-foreground font-mono">TypeScript parsing & codegen ## Core Components
 ### SWC Core
 The foundation layer provides:
 - Fast TypeScript/JavaScript parsing
 - AST representation
 - Code generation (AST → source code)
 ### macroforge_ts_syn
 A Rust crate that provides:
 - TypeScript-specific AST types
 - Parsing utilities for macro input
 - Derive input structures (class fields, decorators, etc.)
 ### macroforge_ts_quote
 Template-based code generation similar to Rust's `quote!`:
 - `ts_template!` - Generate TypeScript code from templates
 - `body!` - Generate class body members
 - Control flow: `{#for}`, `{#if}`, `{$let}`
 ### macroforge_ts_macros
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
 <div class="w-full max-w-md border border-border rounded-lg bg-card p-4 text-center shadow-sm"><div class="font-semibold text-foreground">1. Source Code TypeScript with @derive  <div class="font-semibold text-foreground">2. NAPI-RS receives JavaScript string  <div class="font-semibold text-foreground">3. SWC Parser parses to AST  <div class="font-semibold text-foreground">4. Macro Expander finds @derive decorators  <div class="font-semibold text-foreground">5. For Each Macro extract data, run macro, generate AST nodes  <div class="font-semibold text-foreground">6. Merge generated nodes into AST  <div class="font-semibold text-foreground">7. SWC Codegen generates source code  <div class="font-semibold text-foreground">8. Return to JavaScript with source mapping  ## Performance Characteristics
 - **Thread-safe**: Each expansion runs in an isolated thread with a 32MB stack
 - **Caching**: `NativePlugin` caches results by file version
 - **Binary search**: Position mapping uses O(log n) lookups
 - **Zero-copy**: SWC's arena allocator minimizes allocations
 ## Re-exported Crates
 For custom macro development, `macroforge_ts` re-exports everything you need:
 ```
// Convenient re-exports for macro development
use macroforge_ts::macros::&#123;ts_macro_derive, body, ts_template, above, below, signature&#125;;
use macroforge_ts::ts_syn::&#123;Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input&#125;;

// Also available: raw crate access and SWC modules
use macroforge_ts::swc_core;
use macroforge_ts::swc_common;
use macroforge_ts::swc_ecma_ast;
``` ## Next Steps
 - [Write custom macros](../../docs/custom-macros)
 - [Explore the API reference](../../docs/api)