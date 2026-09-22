# macroforge_ts_syn

TypeScript syntax types for compile-time macro code generation

[![Crates.io](https://img.shields.io/crates/v/macroforge_ts_syn.svg)](https://crates.io/crates/macroforge_ts_syn)
[![Documentation](https://docs.rs/macroforge_ts_syn/badge.svg)](https://docs.rs/macroforge_ts_syn)

## Overview

TypeScript syntax types for compile-time macro code generation.

This crate provides a [`syn`](https://docs.rs/syn)-like API for parsing and manipulating TypeScript
code, enabling macro authors to work with TypeScript AST in a familiar way. It is the core
infrastructure crate for the Macroforge TypeScript macro system.

## Overview

The crate is organized into several modules:

- [`abi`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=abi) - Application
  Binary Interface types for stable macro communication
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
  directive parsing shared by both lowering backends
- [`lower`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=lower) - AST lowering
  from SWC types to IR representations (`swc` feature)
- [`lower_oxc`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=lower_oxc) - AST
  lowering from OXC types to IR representations (`oxc` feature)
- [`parse`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=parse) - TypeScript
  parsing utilities wrapping SWC (`swc` feature)
- [`quote_helpers`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=quote_helpers) -
  Macros for ergonomic code generation
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
│  (lower_classes_oxc / lower_classes, ...)                   │
├─────────────────────────────────────────────────────────────┤
│                    IR Types (ABI Stable)                    │
│  (ClassIR, InterfaceIR, EnumIR, TypeAliasIR, ...)           │
├─────────────────────────────────────────────────────────────┤
│                    Parser Backend                           │
│  (OXC by default; SWC behind the `swc` feature)             │
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

## Helper Macros

This crate provides several helper macros for working with SWC AST nodes:

- [`ts_ident!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=ts_ident) -
  Create an identifier with optional formatting
- [`ts_private_ident!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=ts_private_ident) -
  Create a private (marked) identifier
- [`stmt_block!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=stmt_block) -
  Create a block statement from statements
- [`fn_expr!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=fn_expr) - Create
  an anonymous function expression
- [`member_expr!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=member_expr) -
  Create a member access expression (obj.prop)
- [`assign_stmt!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=assign_stmt) -
  Create an assignment statement
- [`fn_assign!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=fn_assign) -
  Create a function assignment (obj.prop = function() {...})
- [`proto_method!`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=proto_method) -
  Create a prototype method assignment

## Feature Flags

- `oxc` - Enables the OXC parser backend (enabled by default)
- `swc` - Enables SWC integration and the SWC-based helper macros (opt-in)

## Re-exports

For convenience, the crate re-exports commonly used SWC types when the `swc` feature is enabled (it
is not part of the default feature set):

- [`swc_core`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=swc_core) - The
  full SWC core crate
- [`swc_common`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=swc_common) -
  Common SWC types (Span, SourceMap, etc.)
- [`swc_ecma_ast`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=swc_ecma_ast) -
  ECMAScript/TypeScript AST types
- `quote!` - SWC's quote macro for AST generation

## Installation

```bash
cargo add macroforge_ts_syn
```

## Key Exports

### Structs

- **`StmtVec`** - A wrapper type for passing a `Vec<Stmt>` to be used inline in function bodies.
- **`TsExpr`** - Wrapper for
  [`swc_core::ecma::ast::Expr`](https://docs.rs/swc_core/latest/swc_core/?search=Expr) that
  implements [`Display`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=Display)
  and [`ToTsString`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=ToTsString).
- **`TsIdent`** - Wrapper for
  [`swc_core::ecma::ast::Ident`](https://docs.rs/swc_core/latest/swc_core/?search=Ident) that
  implements
  [`Display`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=Display).
- **`TsTypeWrapper`** - Wrapper for
  [`swc_core::ecma::ast::TsType`](https://docs.rs/swc_core/latest/swc_core/?search=TsType) that
  implements
  [`Display`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=Display).
- **`TsStmt`** - Wrapper for
  [`swc_core::ecma::ast::Stmt`](https://docs.rs/swc_core/latest/swc_core/?search=Stmt) that
  implements
  [`Display`](https://docs.rs/macroforge_ts_syn/latest/macroforge_ts_syn/?search=Display).

### Functions

- **`to_ts_expr`** - Convert a value into a TypeScript
  [`Expr`](https://docs.rs/swc_core/latest/swc_core/?search=Expr).
- **`to_ts_type`** - Convert a value into a TypeScript
  [`TsType`](https://docs.rs/swc_core/latest/swc_core/?search=TsType).
- **`to_ts_ident`** - Convert a value into a TypeScript
  [`Ident`](https://docs.rs/swc_core/latest/swc_core/?search=Ident).
- **`to_ts_stmt`** - Convert a value into a TypeScript
  [`Stmt`](https://docs.rs/swc_core/latest/swc_core/?search=Stmt).
- **`expr_to_string`** - Converts an expression to its TypeScript string representation.
- **`type_to_string`** - Converts a type to its TypeScript string representation.
- **`ident_to_string`** - Converts an identifier to its TypeScript string representation.
- **`emit_module_items`** - Emits a list of module items to a TypeScript source string.
- **`emit_expr`** - Emits an expression to a string representation.
- **`emit_ts_type`** - Emits a TypeScript type to a string representation.
- ... and 4 more

### Traits

- **`ToTsExpr`** - Converts common Rust values into SWC
  [`Expr`](https://docs.rs/swc_core/latest/swc_core/?search=Expr) nodes.
- **`ToTsType`** - Converts common Rust values into SWC
  [`TsType`](https://docs.rs/swc_core/latest/swc_core/?search=TsType) nodes.
- **`ToTsIdent`** - Converts common Rust values into SWC
  [`Ident`](https://docs.rs/swc_core/latest/swc_core/?search=Ident) nodes.
- **`ToTsStmt`** - Converts common Rust values into SWC
  [`Stmt`](https://docs.rs/swc_core/latest/swc_core/?search=Stmt) nodes.
- **`ToTsTypeName`** - Trait for converting values to type name strings.
- **`ToTsString`** - Trait for converting values to TypeScript string representations.

## API Reference

See the [full API documentation](https://docs.rs/macroforge_ts_syn) on docs.rs.

## License

MIT
