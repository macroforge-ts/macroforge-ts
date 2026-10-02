# Errors and Diagnostics

A macro reports problems as diagnostics: an error fails the macro, while a warning or note is shown
alongside its output. Each one points at the source it is about.

## Diagnostic

Rust

```
pub struct Diagnostic {
    pub level: DiagnosticLevel,   // Error, Warning or Info
    pub message: String,
    pub span: Option<SpanIR>,     // the source it is about
    pub notes: Vec<String>,       // extra lines of context
    pub help: Option<String>,     // a suggested fix
}
```

Point a diagnostic at the narrowest span that explains it: the field with the bad type, not the
whole class. For a problem with the macro's use as a whole, `input.error_span()` is the macro's name
inside `@derive(...)`, falling back to the whole decorator.

## Failing With One Error

Return a `MacroforgeError`:

Rust

```
use macroforge_ts::ts_syn::{Data, MacroforgeError};

match &input.data {
    Data::Class(class) => { /* ... */ }
    _ => {
        return Err(MacroforgeError::new(
            input.error_span(),
            "@derive(Validate) can only be used on classes",
        ));
    }
}

// Not about any one place in the source:
return Err(MacroforgeError::new_global("the macro configuration is missing"));
```

## Reporting Every Problem at Once

When several fields can be wrong, report them all in one run rather than one per build. Collect them
with `DiagnosticCollector`, then fail if any is an error:

Rust

```
use macroforge_ts::ts_syn::{DiagnosticCollector, MacroforgeErrors};

let mut diagnostics = DiagnosticCollector::new();
for field in class.fields() {
    if field.ts_type.is_empty() {
        diagnostics.error_with_help(
            field.span,
            format!("field \`{}\` has no type", field.name),
            "add a type annotation",
        );
    }
    if field.ts_type == "any" {
        diagnostics.warning(field.span, format!("field \`{}\` is \`any\`", field.name));
    }
}

if diagnostics.has_errors() {
    return Err(MacroforgeErrors::new(diagnostics.into_vec()).into());
}
```

`MacroforgeErrors` converts into a `MacroforgeError` that keeps every diagnostic, warnings included,
so the function can keep returning `Result<TsStream, MacroforgeError>`. A function can also return
`Result<TsStream, MacroforgeErrors>` directly.

| `DiagnosticCollector` method           | Adds                               |
| -------------------------------------- | ---------------------------------- |
| `error(span, message)`                 | An error                           |
| `error_with_help(span, message, help)` | An error with a suggested fix      |
| `warning(span, message)`               | A warning                          |
| `push(diagnostic)`                     | Any `Diagnostic`                   |
| `extend(other)`                        | Everything another collector holds |

`has_errors()`, `is_empty()` and `len()` inspect it, and `into_vec()` hands the diagnostics over.

## Warnings on Success

When the macro succeeds, report warnings and notes on the stream it returns. With the collector
above, after the error check:

Rust

```
let mut output = ts_template!(Within { /* ... */ });
output.add_diagnostics(diagnostics.into_vec());
Ok(output)
```

`add_diagnostic` adds a single one. Streams injected with `{$typescript}` or combined with `merge`
keep their diagnostics.

## Declarative Macro Errors

[Declarative macros](../../docs/declarative-macros) report their own errors. Tooling that builds on
`macroforge_ts_syn`'s declarative parser receives a `DeclarativeError`, which carries a span, a
message, an optional `with_help(...)` fix and any number of `with_note(...)` lines.

Tip

Run `macroforge expand` on a file to see a macro's diagnostics with their positions, without
building the whole project.

## Next Steps

- [Output and Imports](../../docs/custom-macros/output)
- [Testing and Debugging](../../docs/custom-macros/testing-and-debugging)
