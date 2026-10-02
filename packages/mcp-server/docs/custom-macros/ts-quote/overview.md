# Template Syntax

The `macroforge_ts_quote` crate provides two macros for generating TypeScript. `ts_template!` writes
TypeScript as text, with Svelte-like control flow and Rust interpolation; it is what most macros
use. `ts_quote!` builds a single AST node and checks its syntax when your crate compiles.

## Available Macros

| Macro                                      | Output                             | Use Case                                                         |
| ------------------------------------------ | ---------------------------------- | ---------------------------------------------------------------- |
| `ts_template!`                             | A `TsStream` of TypeScript source  | Generating code: methods, functions, declarations                |
| `ts_template!(Within &lbrace; … &rbrace;)` | The same, placed in the class body | Methods and properties; see [Positions](#positions)              |
| `ts_quote!`                                | One oxc AST node                   | Building or inspecting syntax trees; see [ts\_quote!](#ts-quote) |
| `ts_ident!`                                | An identifier                      | Names built from strings; see [ts\_ident!](#ts-ident)            |
