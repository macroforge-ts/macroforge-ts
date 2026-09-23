# ts\_macro\_attribute

`#[ts_macro_attribute]` registers an attribute macro — the analog of Rust's
`#[proc_macro_attribute]`. Users invoke it with a JSDoc `@name` decorator on a declaration; the
macro rewrites the entire declaration.

## Basic Syntax

Rust

```
use macroforge_ts::macros::ts_macro_attribute;
use macroforge_ts::ts_syn::{
    MacroforgeError, Patch, PatchCode, TargetIR, TsStream,
};

#[ts_macro_attribute(traced, description = "Count calls to the decorated function")]
pub fn traced_macro(input: TsStream) -> Result<TsStream, MacroforgeError> {
    let ctx = input
        .context()
        .ok_or_else(|| MacroforgeError::new_global("no macro context"))?;

    let TargetIR::Function(f) = &ctx.target else {
        return Err(MacroforgeError::new_global(
            "@traced can only be applied to functions",
        ));
    };

    let full = input.source();
    let open = full.find('{').unwrap();
    let close = full.rfind('}').unwrap();
    let sig = &full[..open];
    let body = &full[open + 1..close].trim();

    let replacement = format!(
        "{sig}{{\\n    (globalThis as any).__traced ??= {{}};\\n    \\
         (globalThis as any).__traced[{name:?}] = \\
         ((globalThis as any).__traced[{name:?}] || 0) + 1;\\n    {body}\\n}}",
        sig = sig,
        name = f.name,
        body = body,
    );

    let mut out = TsStream::from_string(String::new());
    out.runtime_patches.push(Patch::Replace {
        span: f.span,
        code: PatchCode::Text(replacement),
        source_macro: Some("traced".to_string()),
    });
    Ok(out)
}
```

## Call Site

Consumers decorate a declaration with a JSDoc `@name`:

TypeScript

```
/** import macro { traced } from "@my/macro-package" */

/** @traced */
export function add(a: number, b: number): number {
    return a + b;
}
```

## Patch-Based Output

Warning

Attribute macros must emit their rewrite as an explicit `Patch::Replace` over the target span.
Unlike derive macros, the expander does **not** auto-convert a returned `TsStream`'s tokens into
patches for attribute macros — your macro is responsible for describing the edit.

Minimal pattern:

Rust

```
let ctx = input.context().ok_or_else(|| ...)?;
let TargetIR::Function(f) = &ctx.target else { ... };

// Build the new declaration text...
let replacement = format!("{sig}{{ /* new body */ }}", sig = signature);

// Emit a Replace patch covering the function's full span:
let mut out = TsStream::from_string(String::new());
out.runtime_patches.push(Patch::Replace {
    span: f.span,
    code: PatchCode::Text(replacement),
    source_macro: Some("my_macro".to_string()),
});
Ok(out)
```

## Target Kinds

Attribute macros can be applied to any top-level declaration. `ctx.target` is a `TargetIR` enum; the
structured `IR` types give you access to names, spans, parameters, fields, etc.

| Decoration | TargetIR variant                   |
| ---------- | ---------------------------------- |
| Function   | `TargetIR::Function(FunctionIR)`   |
| Class      | `TargetIR::Class(ClassIR)`         |
| Interface  | `TargetIR::Interface(InterfaceIR)` |
| Enum       | `TargetIR::Enum(EnumIR)`           |
| Type alias | `TargetIR::TypeAlias(TypeAliasIR)` |

## Attribute Options

### name (required)

Rust

```
#[ts_macro_attribute(reactive)]   // users write /** @reactive */
#[ts_macro_attribute(traced)]     // users write /** @traced */
```

### description

Rust

```
#[ts_macro_attribute(
    reactive,
    description = "Add reactivity tracking to a function"
)]
```

## No-op Exports

Each `#[ts_macro_attribute]` auto-generates a no-op void export in the WASM build so TypeScript can
resolve the decorator name during type-checking:

TypeScript

```
// Generated automatically for #[ts_macro_attribute(traced)]
export function traced(): void;
```

## Example Walkthrough

The `@traced` macro in `tooling/playground/macro/src/attrs.rs` wraps a function so every call
increments a counter on `globalThis.__traced[fnName]`:

TypeScript

```
/** import macro { traced } from "@playground/macro" */

/** @traced */
export function add(a: number, b: number): number {
    return a + b;
}

// Expanded output:
export function add(a: number, b: number): number {
    (globalThis as any).__traced ??= {};
    (globalThis as any).__traced["add"] =
        ((globalThis as any).__traced["add"] || 0) + 1;
    return a + b;
}
```

The wrapper preserves the original signature (including `export`, `async`, parameters, and return
type) and only injects new statements at the top of the body.
