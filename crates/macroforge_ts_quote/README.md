# macroforge_ts_quote

Quote macro for generating TypeScript code at compile time

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_quote.svg)](https://crates.io/crates/macroforge_ts_quote)
[![Documentation](https://docs.rs/macroforge_ts_quote/badge.svg)](https://docs.rs/macroforge_ts_quote)

TypeScript code generation macros for macroforge.

This crate provides procedural macros for generating TypeScript code from Rust.
It offers two primary approaches:

- [`ts_quote!`] - A thin wrapper around SWC's `quote!` macro with enhanced
  interpolation syntax for compile-time validated TypeScript generation.

- [`ts_template!`] - A Rust-style template syntax with control flow (`{#if}`,
  `{#for}`, `{#match}`) and expression interpolation (`@{expr}`).

Additionally, scoped template macros are provided for code injection:
- [`above!`] - Inject code above a definition
- [`below!`] - Inject code below a definition
- [`body!`] - Inject code into method/function bodies
- [`signature!`] - Inject code into function signatures

# Architecture

The crate is designed to decouple code generation utilities from the heavier
parsing utilities in `ts_syn`. Templates compile to string-building Rust code
at macro expansion time, then produce [`TsStream`] objects at runtime that can
be parsed by SWC into typed AST nodes.

# Examples

Using `ts_quote!` for simple interpolation:

```ignore
let name = quote_ident("MyClass");
let stmt = ts_quote!(class $name {} as Stmt, name = name);
```

Using `ts_template!` for complex code generation:

```ignore
let fields = vec!["name", "age"];
let stream = ts_template! {
    {#for field in &fields}
        this.@{field} = @{field};
    {/for}
};
```

[`TsStream`]: macroforge_ts::ts_syn::TsStream

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
macroforge_ts_quote = "0.1.38"
```

## Key Exports

### Functions

- **`ts_quote`** - Generates TypeScript AST nodes using SWC's `quote!` macro with enhanced interpolation.
- **`ts_template`** - Template-style macro for TypeScript code generation.
- **`above`** - Generates code to be inserted **above** a class or function definition.
- **`below`** - Generates code to be inserted **below** a class or function definition.
- **`body`** - Generates code to be inserted into a method or function **body**.
- **`signature`** - Generates code to be inserted into a function **signature**.

## API Reference

See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/macroforge_ts_quote) on the Macroforge website.

## License

MIT
