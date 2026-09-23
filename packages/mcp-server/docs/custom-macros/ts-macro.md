# ts\_macro

`#[ts_macro]` registers a function-like macro — the TypeScript equivalent of Rust's `#[proc_macro]`.
Users invoke it as `$name(args)` in their source; the macro receives the argument text as a
`TsStream` and emits a replacement `TsStream` that takes over the call site.

## Basic Syntax

Rust

```
use macroforge_ts::macros::ts_macro;
use macroforge_ts::ts_syn::{MacroforgeError, TsStream};

#[ts_macro(stringify, description = "Quote the argument source as a string literal")]
pub fn stringify_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source().trim();
    Ok(TsStream::from_string(format!("\\"{src}\\"")))
}
```

## Call Site

Consumers invoke the macro with a `$` prefix:

TypeScript

```
/** import macro { $stringify } from "@my/macro-package" */

export const quoted = $stringify(1 + 2 * 3);
//    ↓ expands to
export const quoted = "1 + 2 * 3";
```

Info

The `$` prefix at call sites distinguishes macro invocations from regular function calls. The prefix
is purely syntactic — the macro name itself (as declared in `#[ts_macro(name)]`) does not contain
it.

## Attribute Options

### name (required)

The first argument is the macro name, a bare identifier:

Rust

```
#[ts_macro(sql)]              // users write $sql(...)
#[ts_macro(stringify)]        // users write $stringify(...)
```

### description

Documentation surfaced in IDE tooling:

Rust

```
#[ts_macro(
    sql,
    description = "Compile-time SQL validation"
)]
```

### kind

`#[ts_macro]` defaults to `kind = "call"`. You can change the kind explicitly (for example to share
the same macro in multiple positions), but `"call"` is the natural default.

Rust

```
#[ts_macro(name, kind = "call")]   // default
#[ts_macro(name, kind = "attribute")]  // same as #[ts_macro_attribute]
#[ts_macro(name, kind = "derive")]     // same as #[ts_macro_derive]
```

## Input and Output

The input `TsStream` contains the raw source text of the arguments between the parentheses. Access
it via `input.source()`:

Rust

```
#[ts_macro(concat_names)]
pub fn concat_names_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source();                    // e.g. "foo, bar"
    let (left, right) = src.split_once(',').ok_or_else(|| {
        MacroforgeError::new_global("expected two args")
    })?;
    Ok(TsStream::from_string(format!(
        "\\"{}_{}\\"",
        left.trim(),
        right.trim()
    )))
}
```

Call-macro expansion works by replacing the entire `$name(...)` span with the returned stream's
source. The engine inlines the output verbatim — no auto-wrapping.

## No-op Exports

Each `#[ts_macro]` auto-generates a no-op identity function in the WASM and NAPI builds so consumers
can type-check their code before expansion:

TypeScript

```
// Generated automatically for #[ts_macro(stringify)]
export function stringify(value: any): any;
// The \`macroforge build\` CLI also appends:
export { stringify as $stringify };
```

Consumers can import the `$`-prefixed alias from the generated package and both the runtime (as a
passthrough) and the compile-time expansion will work.

## Importing Call Macros in Consumer Code

Callable macros from external packages are advertised to the expander via the `import macro` JSDoc
comment, including the `$` prefix:

TypeScript

```
/** import macro { $stringify, $concat_names } from "@my/macros" */

const s = $stringify(1 + 2 * 3);
const k = $concat_names(user, name);
```

Note

Unlike the declarative `macroRules` system, proc macros ship inside a Rust crate compiled to
WebAssembly. Publish the crate with `macroforge build` so the `$` aliases are exported correctly.

## Complete Examples

### $stringify — quote source text

Rust

```
#[ts_macro(stringify)]
pub fn stringify_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let src = input.source().trim();
    let escaped = src
        .replace('\\\\', "\\\\\\\\")
        .replace('"', "\\\\\\"");
    Ok(TsStream::from_string(format!("\\"{escaped}\\"")))
}
```

### $state — Svelte-runes-style reactive signal

Rust

```
#[ts_macro(state)]
pub fn state_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let args = input.source().trim();
    Ok(TsStream::from_string(format!("createSignal({})", args)))
}

// Call site: let count = $state(0);
// Expanded:  let count = createSignal(0);
```
