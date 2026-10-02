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
