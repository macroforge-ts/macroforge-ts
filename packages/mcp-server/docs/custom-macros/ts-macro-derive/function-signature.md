## Function Signature

Rust

```
pub fn my_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError>
```

| Parameter                           | Description                                             |
| ----------------------------------- | ------------------------------------------------------- |
| `input: TsStream`                   | Token stream containing the class/interface AST         |
| `Result<TsStream, MacroforgeError>` | Returns generated code or an error with source location |