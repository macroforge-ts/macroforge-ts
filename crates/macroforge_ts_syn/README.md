# macroforge_ts_syn

TypeScript syntax types for build-time macro code generation

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_syn.svg)](https://crates.io/crates/macroforge_ts_syn)
[![Documentation](https://docs.rs/macroforge_ts_syn/badge.svg)](https://docs.rs/macroforge_ts_syn)

## Overview

TypeScript syntax types for build-time macro code generation.

This crate provides a [`syn`](https://docs.rs/syn)-like API for parsing and manipulating TypeScript
code, enabling macro authors to work with TypeScript AST in a familiar way. It is the core
infrastructure crate for the Macroforge TypeScript macro system.

## Overview

The crate is organized into several modules:

- [`abi`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=abi) - Application
  Binary Interface types for stable macro communication
- [`ast`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=ast) - Source-backed
  expression and identifier values for templates
- [`config`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=config) -
  Serializable configuration types shared between host and macro processes
- [`context_registry`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=context_registry) -
  Thread-local storage for the active
  [`MacroContextIR`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=MacroContextIR)
- [`declarative`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=declarative) -
  Grammar and parser for declarative (pattern-matching) macros
- [`derive`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=derive) - Derive
  input types that mirror Rust's `syn::DeriveInput`
- [`errors`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=errors) - Error
  types and diagnostics for macro expansion
- [`import_registry`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=import_registry) -
  Unified import registry built during IR lowering
- [`jsdoc`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=jsdoc) - JSDoc
  directive parsing used by lowering
- [`lower`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=lower) - AST lowering
  from OXC types to IR representations
- [`stream`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=stream) - Parsing
  stream abstraction similar to `syn::parse::ParseBuffer`
- [`type_normalize`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=type_normalize) -
  Helpers for splitting TS type-string snippets into structural pieces

## Architecture

The crate follows a layered architecture:

```text
┌─────────────────────────────────────────────────────────────┐
│                    User-Facing API                          │
│  (DeriveInput, TsStream, parse_ts_macro_input!)             │
├─────────────────────────────────────────────────────────────┤
│                    Lowering Layer                           │
│  (lower_classes, lower_interfaces, ...)                     │
├─────────────────────────────────────────────────────────────┤
│                    IR Types (ABI Stable)                    │
│  (ClassIR, InterfaceIR, EnumIR, TypeAliasIR, ...)           │
├─────────────────────────────────────────────────────────────┤
│                    Parser Backend                           │
│  (OXC)                                                      │
└─────────────────────────────────────────────────────────────┘
```

## Usage Example

Here's how to use this crate in a derive macro:

```rust
use macroforge_ts_syn::{parse_ts_macro_input, Data, DeriveInput, MacroforgeError, TsStream};

// A typical derive macro entry point (normally annotated with
// `#[ts_macro_derive(...)]` from the `macroforge_ts_macros` crate)
pub fn my_derive_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    // Parse the input using the syn-like API
    let input = parse_ts_macro_input!(input as DeriveInput);

    // Access type information
    println!("Processing type: {}", input.name());

    // Match on the type kind
    match &input.data {
        Data::Class(class) => {
            for field in class.fields() {
                println!("Field: {}", field.name);
            }
        }
        Data::Interface(_iface) => {
            // Handle interface...
        }
        Data::Enum(_enum_) => {
            // Handle enum...
        }
        Data::TypeAlias(_alias) => {
            // Handle type alias...
        }
    }

    // Return the generated code as a TsStream
    Ok(TsStream::from_string(String::new()))
}
```

## Re-exports

- [`oxc`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=oxc) - The OXC crate,
  for macros that work on the parsed AST directly

## Installation

```bash
cargo add macroforge_ts_syn
```

## Key Exports

### Traits

- **`ToTsString`** - Trait for converting values to TypeScript string representations.

## API Reference

See the [full API documentation](https://docs.rs/macroforge_ts_syn) on docs.rs.

## License

MIT
