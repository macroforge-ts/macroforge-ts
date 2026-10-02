# Output and Imports

A macro returns a `TsStream`: generated TypeScript plus everything that travels with it, such as
imports, patches and warnings. This page covers each part.

## What a Macro Returns

A function marked with `#[ts_macro_derive]`, `#[ts_macro_attribute]` or `#[ts_macro]` returns
`Result<TsStream, E>`. On `Ok`, the host turns the stream into a `MacroResult`; on `Err`, the error
becomes the result's diagnostics. `E` is anything that converts into a `MacroResult`, usually
`MacroforgeError`; see [Errors and Diagnostics](../../docs/custom-macros/diagnostics).

What happens to the stream's source depends on the kind of macro:

| Kind      | The stream's source                                                               |
| --------- | --------------------------------------------------------------------------------- |
| Derive    | Inserted next to the target, at the stream's [insert position](#insert-positions) |
| Attribute | Ignored: an attribute macro changes code through [patches](#patches)              |
| Call      | Replaces the `$name(...)` call expression                                         |

Imports, patches, cross-module suffixes and warnings on the stream apply for every kind.

## Insert Positions

A derive's code goes in one of five places, named by `InsertPos`:

TypeScript

```
// ─── Top ────────────────────────
import { foo } from "./runtime";

// ─── Above ──────────────────────
/** @derive(Debug) */
class User {
    // ─── Within ─────────────────
    name: string;
}
// ─── Below (the default) ────────
export function userToString(value: User): string { ... }

// ─── Bottom ─────────────────────
```

`ts_template!` takes the position as its first word, as in `ts_template!(Within { ... })`; without
one the code goes `Below`. A stream built by hand takes it from `TsStream::with_insert_pos`.

Rust

```
use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::{InsertPos, TsStream};

// Members of the class body
let members = ts_template!(Within {
    toString(): string { return "User"; }
});

// A function after the class
let standalone = ts_template! {
    export function describeUser(): string { return "a user"; }
};

// The same, built from a string
let by_hand = TsStream::with_insert_pos(
    "export const userVersion = 1;".to_string(),
    InsertPos::Below,
);
```

### Combining Streams

`merge` appends one stream to another and keeps the first stream's position; `TsStream::merge_all`
folds a list. Everything else on the streams (imports, patches, suffixes, warnings) is combined too.
Code for two different positions goes in two streams, injected into one template with
`{$typescript}`:

Rust

```
Ok(ts_template! {
    {$typescript standalone}
    {$typescript members}
})
```

A template with an explicit position starts its code with a marker comment, such as
`/* @macroforge:body */` for `Within`, and the host splits a combined stream back up at the markers.
Code before the first marker takes the combined stream's own position, so put a stream without an
explicit position first, as above.

### Other Stream Methods

| Method                          | Use                                                                             |
| ------------------------------- | ------------------------------------------------------------------------------- |
| `TsStream::from_string(source)` | A stream of hand-written code, positioned `Below`                               |
| `source()`                      | The generated code so far                                                       |
| `take_source()`                 | Takes the code out, leaving the rest of the stream to merge elsewhere           |
| `context()`                     | The [macro context](../../docs/custom-macros/context-and-ir) the host passed in |
| `parse_stmt(&allocator)`        | Parses the first statement of the stream with oxc, to inspect generated code    |

## Imports

Generated code that calls a helper needs an import for it. The import methods on `TsStream` register
the import, and the host writes every requested import at the top of the file when the expansion is
done. A request is skipped when the file already imports, or another macro already requested, the
same local name, so asking twice is harmless.

### Plain Imports

Rust

```
// import { validate } from "my-validation-lib";
output.add_import("validate", "my-validation-lib");

// import type { Result } from "my-validation-lib";
output.add_type_import("Result", "my-validation-lib");

// import { validate as runValidate } from "my-validation-lib";
output.add_import("validate as runValidate", "my-validation-lib");
```

### Aliased Imports

A helper imported under its own name can clash with a name the user's file already uses. An alias
avoids that:

Rust

```
// import { resultOk as __mf_resultOk } from "@my/runtime";
output.add_aliased_import("resultOk", "@my/runtime");
output.add_aliased_type_import("Options", "@my/runtime"); // __mf_Options

// Any alias you choose
output.add_import_as("resultOk", "myResultOk", "@my/runtime");
output.add_type_import_as("Options", "MyOptions", "@my/runtime");
```

Generated code then refers to the alias, as in `__mf_resultOk(value)`. A macro with a fixed set of
runtime imports can declare them once:

Rust

```
use macroforge_ts::ts_syn::ImportConfig;

const RUNTIME_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("resultOk", "__mf_resultOk", "@my/runtime"),
    ImportConfig::type_only("Options", "__mf_Options", "@my/runtime"),
];

output.add_imports(RUNTIME_IMPORTS);
```

### Imports Resolved From a Type

A helper generated beside a type, such as `userValidate` next to `User`, lives in the type's module.
These methods find that module through the project's
[type registry](../../docs/custom-macros/type-aware) and the file's own imports, and do nothing when
the type is in the same file or unknown:

Rust

```
// The module the current file imports \`User\` from, if any
let module: Option<String> = output.module_specifier_for("User");

// import { userValidate } from "<the module of User>";
let added: bool = output.add_import_for("userValidate", "User");
output.add_type_import_for("UserErrors", "User");

// Several helpers from one module, resolving it once: (name, type-only?)
output.add_helpers_for("User", &[("userValidate", false), ("UserErrors", true)]);
```

### Cross-Module Suffixes

When a macro generates calls to helpers that other macros generate for other types, it can name the
helpers' suffix instead of resolving each one. With the suffix `GetFields` registered, a generated
call to `companyNameGetFields()` gets `import { companyNameGetFields }` from wherever the file
imports `CompanyName`:

Rust

```
// {camelCaseType}GetFields(...) calls are imported from the type's module
output.add_cross_module_suffix("GetFields");

// {PascalCaseType}Errors type references get an \`import type\`
output.add_cross_module_type_suffix("Errors");
```

## Patches

A patch edits the user's source directly: inserts code at a position, or replaces or deletes a span.
Attribute macros work this way, and any macro can add patches to its stream's `runtime_patches`:

Rust

```
use macroforge_ts::ts_syn::{Patch, SpanIR};

// Replace the target with new code
output.runtime_patches.push(Patch::Replace {
    span: ctx.target_span,
    code: rewritten,
    source_macro: Some("traced".to_string()),
});

// Insert before the closing brace of a class body
output.runtime_patches.push(
    macroforge_ts::ts_syn::insert_into_class(class.body_span(), "static version = 1;")
        .with_source_macro("Versioned"),
);

// Delete a span
output.runtime_patches.push(Patch::Delete { span: decorator_span });
```

`source_macro` names the macro in source maps and in errors about the patch. Patches from one
expansion must not overlap, and a patch outside the file is an error.

### Spans

A `SpanIR` marks a range of the file in _positions_: byte offsets plus one, so the first byte of the
file is position 1. Spans from the IR are already in positions, and `source_range()` turns one into
the 0-based byte range of the file it came from.

A macro sees the target's own text, not the whole file: `ctx.target_source` is the file's text from
`ctx.target_span.start` to `ctx.target_span.end`. To read the text of a span inside the target,
count from the target's start:

Rust

```
let offset = ctx.target_span.start;
let field_text = &ctx.target_source
    [(field.span.start - offset) as usize..(field.span.end - offset) as usize];
```

Note

An insertion point is a zero-width span, with `start == end`. Code inserted at position `n` goes
before the byte at offset `n - 1`.

## Warnings

A macro that succeeds can still report something, such as a hint about a likely mistake, with
`add_diagnostic`. The expansion goes ahead and the warning is shown with it:

Rust

```
use macroforge_ts::ts_syn::{Diagnostic, DiagnosticLevel};

output.add_diagnostic(Diagnostic {
    level: DiagnosticLevel::Warning,
    message: "field \`id\` has no type; it is serialized as unknown".to_string(),
    span: Some(field.span),
    notes: vec![],
    help: Some("give \`id\` a type annotation".to_string()),
});
```

To fail the macro instead, return an error; see
[Errors and Diagnostics](../../docs/custom-macros/diagnostics).

## Next Steps

- [Template syntax](../../docs/custom-macros/ts-quote) for writing the generated code
- [Context and IR](../../docs/custom-macros/context-and-ir) for reading the target
- [Errors and Diagnostics](../../docs/custom-macros/diagnostics)
