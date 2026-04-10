# macroforge_ts_quote

Quote macro for generating TypeScript code at compile time

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_quote.svg)](https://crates.io/crates/macroforge_ts_quote)
[![Documentation](https://docs.rs/macroforge_ts_quote/badge.svg)](https://docs.rs/macroforge_ts_quote)

## Overview

TypeScript code generation macros for macroforge.

This crate provides procedural macros for generating TypeScript code from Rust.
It offers two primary approaches:

- [`ts_quote!`] - A thin wrapper around SWC's `quote!` macro with enhanced
  interpolation syntax for compile-time validated TypeScript generation.

- [`ts_template!`] - A Rust-style template syntax with control flow (`{#if}`,
  `{#for}`, `{#match}`) and expression interpolation (`@{expr}`).

# Architecture

The quote implementation uses TypeScript parsing (`Syntax::Typescript`) instead
of JavaScript, enabling native support for type annotations and TypeScript syntax.

# Insert Positions

`ts_template!` supports an optional position keyword to control where generated
code is inserted:

```ignore
// Insert inside the class body
ts_template!(Within { ... })

// Insert at the top of the file (for imports)
ts_template!(Top { ... })

// Default: insert after the target (Below)
ts_template! { ... }
```

Available positions: `Top`, `Above`, `Within`, `Below`, `Bottom`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
macroforge_ts_quote = "0.1.81"
```

## Key Exports

### Functions

- **`ts_quote`** - Parse and generate code for a TypeScript quote.
- **`ts_template`** - Generate TypeScript code with Rust-style control flow and interpolation.

## API Reference

See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/macroforge_ts_quote) on
the Macroforge website.

## License

MIT
