## Function Signature

Rust

```
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>
```

| Parameter             | Description                                                                                                                                      |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `mut input: TsStream` | The target's source, with the [macro context](../../docs/custom-macros/context-and-ir) attached                                                  |
| `Result<TsStream, E>` | The generated code, or an error. `E` is usually `MacroforgeError`; any type that converts into a `MacroResult` works, such as `MacroforgeErrors` |

The attribute turns the function into a `Macroforge` implementation named after it (`derive_json`
becomes `DeriveJson`), registers it, and adds the exports a package built with `macroforge build`
needs.
