# Testing and Debugging

A macro is ordinary Rust, so it can be unit-tested like any function. For a quick look at what it
generates, run it on a file.

## Running a Macro on a File

After `macroforge build`, expand one file of a project that uses the macro and print the result:

Bash

```
macroforge expand src/user.ts --print
```

Diagnostics are printed with their file, line and column, followed by any notes and help. `--out`
writes the expansion to a file instead, and `--types-out` writes the generated type declarations.

## Unit Tests

A test builds the input a macro would get, calls the macro function and checks the stream it
returns. To build the input, lower a TypeScript snippet to IR and wrap it in a context:

Rust

```
use macroforge_ts::ts_syn::abi::{MacroContextIR, SpanIR};
use macroforge_ts::ts_syn::oxc::allocator::Allocator;
use macroforge_ts::ts_syn::oxc::parser::Parser;
use macroforge_ts::ts_syn::oxc::span::SourceType;
use macroforge_ts::ts_syn::{TsStream, lower_classes};

/// The input a derive gets for the first class in \`source\`.
fn class_input(source: &str) -> TsStream {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    let class = lower_classes(&parsed.program, source, None)
        .expect("the class lowers")
        .into_iter()
        .next()
        .expect("the source has a class");
    let context = MacroContextIR::new_derive_class(
        "Validate".to_string(),
        "my-macros".to_string(),
        SpanIR::new(0, 0),
        class.span,
        "test.ts".to_string(),
        class,
        source.to_string(),
    );
    TsStream::with_context(source, "test.ts", context).expect("the input stream builds")
}

#[test]
fn validate_checks_every_field() {
    let output = derive_validate(class_input("class User { name: string; email: string; }"))
        .expect("the macro succeeds");
    assert!(output.source().contains("this.name"));
    assert!(output.source().contains("this.email"));
}
```

`lower_interfaces`, `lower_enums`, `lower_type_aliases` and `lower_functions` lower the other kinds,
and `MacroContextIR` has a matching constructor for each: `new_derive_interface`, `new_derive_enum`,
`new_derive_type_alias`, and `new_attribute_class`, `new_attribute_function` and the rest for
attribute macros.

The context starts with empty imports and an empty type registry. To test a macro that reads them,
fill them in: `context.import_registry.install_source_imports(...)` gives the file imports, and
`context.type_registry.insert(...)` adds project types.

## Debug Logging

`macroforge_ts::debug` writes timestamped lines to the project's `.macroforge/debug.log`, in the
nearest directory holding a `macroforge.config`:

Rust

```
use macroforge_ts::debug;

debug::log("Validate", "starting");
debug::log_ctx("Validate", &input.context);   // macro, file, target and field count
macroforge_ts::debug_log!("Validate", "{} has {} fields", input.name(), class.fields().len());

let result = output.clone().into_result();
debug::log_result("Validate", &result);       // patch, token and diagnostic counts

debug::clear();                               // empty the log
```

A macro package runs in a WebAssembly sandbox with no file access, so its lines travel back with its
result. When the CLI runs the macro they are written to `debug.log` as usual; in a JavaScript host
such as the Vite plugin they are printed to the console instead.

## Implementing a Macro by Hand

The `#[ts_macro_*]` attributes generate an implementation of the `Macroforge` trait, plus the
registration and exports a macro package needs. Writing the trait yourself is only needed when you
embed Macroforge in your own Rust program:

Rust

```
use std::sync::Arc;
use macroforge_ts::host::{Macroforge, MacroRegistry, Result};
use macroforge_ts::ts_syn::{MacroKind, MacroResult, TsStream};

struct Stamp;

impl Macroforge for Stamp {
    fn name(&self) -> &str { "Stamp" }
    fn kind(&self) -> MacroKind { MacroKind::Derive }
    fn run(&self, input: TsStream) -> MacroResult {
        let name = input.context().map_or("Unknown", |ctx| ctx.macro_name.as_str());
        TsStream::from_string(format!("export const stampedBy = \"{name}\";")).into_result()
    }
}

fn register(registry: &MacroRegistry) -> Result<()> {
    registry.register("my-macros", "Stamp", Arc::new(Stamp))
}

macroforge_ts::register_macro_package!("my-macros", register);
```

Warning

`register_macro_package!` registers macros with the expander in the same program. A package built
with `macroforge build` exposes only the macros declared with the `#[ts_macro_*]` attributes.

## Next Steps

- [Errors and Diagnostics](../../docs/custom-macros/diagnostics)
- [Output and Imports](../../docs/custom-macros/output)
