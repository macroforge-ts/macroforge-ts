# macroforge_ts_syn

TypeScript syntax types for compile-time macro code generation

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_syn.svg)](https://crates.io/crates/macroforge_ts_syn)
[![Documentation](https://docs.rs/macroforge_ts_syn/badge.svg)](https://docs.rs/macroforge_ts_syn)

TypeScript syntax types for compile-time macro code generation.

This crate provides a [`syn`](https://docs.rs/syn)-like API for parsing and manipulating
TypeScript code, enabling macro authors to work with TypeScript AST in a familiar way.
It is the core infrastructure crate for the Macroforge TypeScript macro system.

## Overview

The crate is organized into several modules:

- [`abi`] - Application Binary Interface types for stable macro communication
- [`derive`] - Derive input types that mirror Rust's `syn::DeriveInput`
- [`errors`] - Error types and diagnostics for macro expansion
- [`lower`] - AST lowering from SWC types to IR representations
- [`parse`] - TypeScript parsing utilities wrapping SWC
- [`quote_helpers`] - Macros for ergonomic code generation
- [`stream`] - Parsing stream abstraction similar to `syn::parse::ParseBuffer`

## Architecture

The crate follows a layered architecture:

```text
┌─────────────────────────────────────────────────────────────┐
│                    User-Facing API                          │
│  (DeriveInput, TsStream, parse_ts_macro_input!)             │
├─────────────────────────────────────────────────────────────┤
│                    Lowering Layer                           │
│  (lower_classes, lower_interfaces, lower_enums, ...)        │
├─────────────────────────────────────────────────────────────┤
│                    IR Types (ABI Stable)                    │
│  (ClassIR, InterfaceIR, EnumIR, TypeAliasIR, ...)           │
├─────────────────────────────────────────────────────────────┤
│                    SWC Parser                               │
│  (swc_core for TypeScript/JavaScript parsing)               │
└─────────────────────────────────────────────────────────────┘
```

## Usage Example

Here's how to use this crate in a derive macro:

```rust,ignore
use macroforge_ts_syn::{parse_ts_macro_input, DeriveInput, MacroResult, Patch};

pub fn my_derive_macro(ctx: MacroContextIR) -> MacroResult {
    // Parse the input using the syn-like API
    let input = parse_ts_macro_input!(ctx);

    // Access type information
    println!("Processing type: {}", input.name());

    // Match on the type kind
    match &input.data {
        Data::Class(class) => {
            for field in class.fields() {
                println!("Field: {}", field.name);
            }
        }
        Data::Interface(iface) => {
            // Handle interface...
        }
        Data::Enum(enum_) => {
            // Handle enum...
        }
        Data::TypeAlias(alias) => {
            // Handle type alias...
        }
    }

    // Generate code and return patches
    MacroResult::ok()
}
```

## Helper Macros

This crate provides several helper macros for working with SWC AST nodes:

- [`ident!`] - Create an identifier with optional formatting
- [`private_ident!`] - Create a private (marked) identifier
- [`stmt_block!`] - Create a block statement from statements
- [`fn_expr!`] - Create an anonymous function expression
- [`member_expr!`] - Create a member access expression (obj.prop)
- [`assign_stmt!`] - Create an assignment statement
- [`fn_assign!`] - Create a function assignment (obj.prop = function() {...})
- [`proto_method!`] - Create a prototype method assignment

## Feature Flags

- `swc` - Enables SWC integration and the helper macros (enabled by default)

## Re-exports

For convenience, the crate re-exports commonly used SWC types when the `swc` feature
is enabled:

- [`swc_core`] - The full SWC core crate
- [`swc_common`] - Common SWC types (Span, SourceMap, etc.)
- [`swc_ecma_ast`] - ECMAScript/TypeScript AST types
- `quote!` - SWC's quote macro for AST generation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
macroforge_ts_syn = "0.1.37"
```

## Key Exports

### Structs

- **`StmtVec`** - A wrapper type for passing a `Vec<Stmt>` to be used inline in function bodies.

## API Reference

See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/macroforge_ts_syn) on the Macroforge website.

## License

MIT
