# Template Syntax

The `macroforge_ts_quote` crate provides template-based code generation for TypeScript. The
`ts_template!` macro uses Svelte + Rust-inspired syntax for control flow and interpolation, making
it easy to generate complex TypeScript code.

## Available Macros

| Macro                                      | Output              | Use Case                |
| ------------------------------------------ | ------------------- | ----------------------- |
| `ts_template!`                             | Any TypeScript code | General code generation |
| `ts_template!(Within &lbrace; … &rbrace;)` | Class body members  | Methods and properties  |
