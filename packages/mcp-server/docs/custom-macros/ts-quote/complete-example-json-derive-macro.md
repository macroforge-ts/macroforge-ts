## Complete Example: JSON Derive Macro

Here's a comparison showing how `ts_template!` simplifies code generation:

### Before (Manual String Building)

Rust

```
#[ts_macro_derive(JSON)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(input.error_span(), "@derive(JSON) needs a class"));
    };

    let mut body = String::from("const result = {};\\n");
    for field_name in class.field_names() {
        body.push_str(&format!("result.{field_name} = this.{field_name};\\n"));
    }
    body.push_str("return result;");

    Ok(TsStream::from_string(format!(
        "{}.prototype.toJSON = function() {{\\n{body}\\n}};",
        input.name()
    )))
}
```

### After (With ts\_template!)

Rust

```
#[ts_macro_derive(JSON)]
pub fn derive_json_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);
    let Some(class) = input.as_class() else {
        return Err(MacroforgeError::new(input.error_span(), "@derive(JSON) needs a class"));
    };
    let class_name = input.name();

    Ok(ts_template! {
        @{class_name}.prototype.toJSON = function() {
            const result = {};
            {#for field in class.field_names()}
                result.@{field} = this.@{field};
            {/for}
            return result;
        };
    })
}
```
