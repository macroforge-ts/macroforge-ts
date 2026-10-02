## Returning Errors

Use `MacroforgeError` to report errors with source locations:

Rust

```
#[ts_macro_derive(ClassOnly)]
pub fn class_only(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(_) => {
            // Generate code...
            Ok(ts_template!(Within { /* ... */ }))
        }
        _ => Err(MacroforgeError::new(
            input.error_span(),
            "@derive(ClassOnly) can only be used on classes",
        )),
    }
}
```

Reporting several problems at once, and warnings on success, are covered in
[Errors and Diagnostics](../../docs/custom-macros/diagnostics).
