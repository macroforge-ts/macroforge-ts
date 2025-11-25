# TypeScript Macro DX Guide

This guide demonstrates how to write TypeScript macros with a Rust-like developer experience using `ts_syn` for parsing and SWC's built-in tools for code generation.

## Overview

The macro system provides:

- **`ts_syn`**: Parse TypeScript code with a `syn`-like API (`TsStream`, `ParseTs` trait)
- **`swc_core::ecma::quote`**: Generate typed TypeScript AST nodes (when needed)
- **`format!`**: Simple string-based code generation (for most cases)

## Code Generation

### Simple String Generation (Recommended for Most Cases)

For most macros, simple string formatting is the easiest approach:

```rust
let field_name = "userId";
let field_type = "string";

let code = format!(
    "get {}(): {} {{ return this.{}; }}",
    field_name, field_type, field_name
);
// Result: "get userId(): string { return this.userId; }"
```

### Typed AST Generation (Advanced)

When you need typed AST nodes, use SWC's quote macro:

```rust
use swc_core::ecma::quote::quote;
use swc_core::ecma::ast::*;
use swc_common::DUMMY_SP;

// Generate typed expressions
let expr: Box<Expr> = quote!("x + y" as Expr);

// Generate statements
let stmt: Stmt = quote!("const x = 5" as Stmt);

// Generate identifiers
let ident: Ident = quote!("myVar" as Ident);
```

## Parsing with `ts_syn`

The `ts_syn` crate provides a `syn`-like API for parsing TypeScript:

### Using `TsStream`

```rust
use ts_syn::{TsStream, ParseTs};
use swc_ecma_ast::*;

// Create a stream from source code
let mut stream = TsStream::new("const x = 5;", "input.ts")?;

// Parse using the trait
let stmt: Stmt = stream.parse()?;

// Or parse specific items
let expr = stream.parse_expr()?;
let ident = stream.parse_ident()?;
```

### The `ParseTs` Trait

Implement `ParseTs` for custom parsing logic:

```rust
use ts_syn::{ParseTs, TsStream, TsSynError};
use swc_ecma_ast::*;

struct TimeoutConfig {
    timeout_ms: u32,
}

impl ParseTs for TimeoutConfig {
    fn parse(input: &mut TsStream) -> Result<Self, TsSynError> {
        // Parse: { timeout: 5000 }
        let expr = input.parse_expr()?;

        // Extract timeout value from object literal
        // ... parsing logic ...

        Ok(TimeoutConfig { timeout_ms: 5000 })
    }
}

// Now you can parse it easily
let mut stream = TsStream::new("{ timeout: 5000 }", "input.ts")?;
let config: TimeoutConfig = stream.parse()?;
```

### Helper Functions

For quick parsing without creating a stream:

```rust
use ts_syn::{parse_ts_str, parse_ts_expr, parse_ts_stmt};

// Generic parsing
let ident: Ident = parse_ts_str("myVariable")?;
let module: Module = parse_ts_str("export class Foo {}")?;

// Specific helpers
let expr = parse_ts_expr("x + y")?;
let stmt = parse_ts_stmt("const x = 5;")?;
```

## Complete Macro Example

Here's a complete example using both approaches:

```rust
use ts_macro_abi::{ClassIR, MacroContextIR, MacroResult, Patch, SpanIR, insert_into_class};
use ts_syn::{TsStream, ParseTs};

#[ts_macro_derive(Debug)]
pub fn derive_debug(ctx: MacroContextIR) -> MacroResult {
    let class = match ctx.as_class() {
        Some(c) => c,
        None => return error_result("@Derive(Debug) can only target classes"),
    };

    let class_name = &class.name;

    // Generate field representations using simple string formatting
    let fields: Vec<String> = class.fields.iter()
        .map(|f| format!("{}: ${{this.{}}}", f.name, f.name))
        .collect();

    let fields_str = fields.join(", ");

    // Generate the debug method
    let debug_method = format!(
        r#"
    debug(): string {{
        return `{}({})`;
    }}
"#,
        class_name,
        fields_str
    );

    // Use helper to insert into class
    MacroResult {
        runtime_patches: vec![
            Patch::Delete { span: ctx.decorator_span },
            insert_into_class(class.body_span, debug_method),
        ],
        ..Default::default()
    }
}
```

## When to Use Each Approach

### Use `format!()` when:
- ✅ Generating simple code snippets
- ✅ Building methods, getters, setters
- ✅ Creating type definitions
- ✅ Most macro use cases

### Use `swc_core::ecma::quote` when:
- ✅ You need typed AST nodes for transformations
- ✅ You're working with SWC visitors
- ✅ You need to manipulate AST before generating code
- ✅ Type safety is critical

## Comparison with Rust `syn` + `quote`

### Rust Macro

```rust
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Debug)]
pub fn derive_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", stringify!(#name))
            }
        }
    };

    TokenStream::from(expanded)
}
```

### TypeScript Macro

```rust
use ts_syn::{TsStream, ParseTs};
use ts_macro_abi::*;

#[ts_macro_derive(Debug)]
pub fn derive_debug(ctx: MacroContextIR) -> MacroResult {
    let class = ctx.as_class().unwrap();
    let name = &class.name;

    let code = format!(
        "debug(): string {{ return '{}'; }}",
        name
    );

    MacroResult {
        runtime_patches: vec![
            insert_into_class(class.body_span, code)
        ],
        ..Default::default()
    }
}
```

## Key Features

### TsStream (like `syn::ParseBuffer`)
- ✅ Owns all data (no lifetime issues)
- ✅ `parse::<T>()` generic method
- ✅ Specific helpers: `parse_expr()`, `parse_stmt()`, `parse_ident()`
- ✅ Works with `ParseTs` trait

### Code Generation
- ✅ `format!()` for simple strings
- ✅ `swc_core::ecma::quote` for typed AST
- ✅ Direct integration with SWC

### Helper Utilities
- ✅ `insert_into_class()` - Insert code before closing brace
- ✅ `parse_ts_str::<T>()` - Quick parsing without stream
- ✅ `parse_ts_expr()`, `parse_ts_stmt()` - Common helpers

## Best Practices

1. **Use `format!()` for most code generation**: Simple, readable, no dependencies
2. **Use `TsStream` for complex parsing**: When you need to parse multiple items or custom syntax
3. **Use `parse_ts_str::<T>()` for simple parsing**: One-off parsing of standard TypeScript
4. **Implement `ParseTs` for reusable parsers**: Makes your parsing logic clean and composable
5. **Use `swc_core::ecma::quote` only when you need typed AST**: Most macros don't need this

## Dependencies

```toml
[dependencies]
ts_macro_abi = { path = "../ts_macro_abi" }  # IR types + helpers
ts_syn = { path = "../ts_syn" }              # Parsing with TsStream

# Optional: Only if you need typed AST generation
swc_core = { version = "0.100", features = ["ecma_quote"] }
```

## Further Reading

- [SWC Core Documentation](https://swc.rs/)
- [SWC AST types](https://rustdoc.swc.rs/swc_ecma_ast/)
- [Rust `syn` documentation](https://docs.rs/syn/) - Similar concepts
- [Rust `format!` macro](https://doc.rust-lang.org/std/macro.format.html)
