# macroforge_ts_syn

TypeScript syntax types for compile-time macro code generation

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_syn.svg)](https://crates.io/crates/macroforge_ts_syn)
[![Documentation](https://docs.rs/macroforge_ts_syn/badge.svg)](https://docs.rs/macroforge_ts_syn)

## Overview

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
use macroforge_ts_syn::{parse_ts_macro_input, DeriveInput, MacroResult, Patch, Data, MacroContextIR};

// This function signature shows a typical derive macro entry point
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
macroforge_ts_syn = "0.1.81"
```

## Key Exports

### Structs

- **`StmtVec`** - A wrapper type for passing a `Vec<Stmt>` to be used inline in function bodies.
- **`TsExpr`** - Wrapper for [`swc_core::ecma::ast::Expr`] that implements [`Display`] and [`ToTsString`].
- **`TsIdent`** - Wrapper for [`swc_core::ecma::ast::Ident`] that implements [`Display`].
- **`TsTypeWrapper`** - Wrapper for [`swc_core::ecma::ast::TsType`] that implements [`Display`].
- **`TsStmt`** - Wrapper for [`swc_core::ecma::ast::Stmt`] that implements [`Display`].

### Functions

- **`to_ts_expr`** - Convert a value into a TypeScript [`Expr`](swc_core::ecma::ast::Expr).
- **`to_ts_type`** - Convert a value into a TypeScript [`TsType`](swc_core::ecma::ast::TsType).
- **`to_ts_ident`** - Convert a value into a TypeScript [`Ident`](swc_core::ecma::ast::Ident).
- **`to_ts_stmt`** - Convert a value into a TypeScript [`Stmt`](swc_core::ecma::ast::Stmt).
- **`expr_to_string`** - Converts an expression to its TypeScript string representation.
- **`type_to_string`** - Converts a type to its TypeScript string representation.
- **`ident_to_string`** - Converts an identifier to its TypeScript string representation.
- **`emit_module_items`** - Emits a list of module items to a TypeScript source string.
- **`emit_expr`** - Emits an expression to a string representation.
- **`emit_ts_type`** - Emits a TypeScript type to a string representation.
- ... and 4 more

### Traits

- **`ToTsExpr`** - Converts common Rust values into SWC [`Expr`](swc_core::ecma::ast::Expr) nodes.
- **`ToTsType`** - Converts common Rust values into SWC [`TsType`](swc_core::ecma::ast::TsType) nodes.
- **`ToTsIdent`** - Converts common Rust values into SWC [`Ident`](swc_core::ecma::ast::Ident) nodes.
- **`ToTsStmt`** - Converts common Rust values into SWC [`Stmt`](swc_core::ecma::ast::Stmt) nodes.
- **`ToTsTypeName`** - Trait for converting values to type name strings.
- **`ToTsString`** - Trait for converting values to TypeScript string representations.

## API Reference

See the [full API documentation](https://macroforge.dev/docs/api/reference/rust/macroforge_ts_syn) on
the Macroforge website.

## License

MIT
