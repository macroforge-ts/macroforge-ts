# macroforge_ts_macros

Derive macros for TypeScript compile-time code generation

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_macros.svg)](https://crates.io/crates/macroforge_ts_macros)
[![Documentation](https://docs.rs/macroforge_ts_macros/badge.svg)](https://docs.rs/macroforge_ts_macros)

This crate provides procedural macros for generating TypeScript macro infrastructure
in the Macroforge ecosystem. It simplifies the creation of derive macros that can
transform TypeScript classes at compile time.

## Overview

The primary macro provided is [`ts_macro_derive`], which transforms a Rust function
into a fully-fledged TypeScript macro that integrates with the Macroforge runtime.

## Example

```rust,ignore
use macroforge_ts_macros::ts_macro_derive;

#[ts_macro_derive(Debug, description = "Generates debug formatting")]
fn debug_macro(input: TsStream) -> Result<TsStream, MacroError> {
    // Transform the input TypeScript class
    Ok(input)
}
```

This generates:
- A struct implementing the [`Macroforge`] trait
- A NAPI function for JavaScript interop
- Registration with the macro registry via `inventory`

## Architecture

The generated code follows this pattern:

1. **Macro Struct**: A unit struct that implements the `Macroforge` trait
2. **NAPI Bridge**: A function exposed to JavaScript that handles JSON serialization
3. **Descriptor**: Static metadata about the macro for runtime discovery
4. **Registration**: Automatic registration with the `inventory` crate

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
macroforge_ts_macros = "0.1.37"
```

## Key Exports

### Functions

- **`ts_macro_derive`** - A procedural macro attribute that transforms a function into a TypeScript derive macro.

## API Reference

See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/macroforge_ts_macros) on the Macroforge website.

## License

MIT
