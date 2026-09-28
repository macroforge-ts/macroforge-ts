## Complete Example: JSON Derive Macro

Here's a comparison showing how `ts_template!` simplifies code generation:

### Before (Manual String Building)

Rust

```
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();

            let mut body = String::from("const result = {};\\n");
            for field_name in class.field_names() {
                body.push_str(&format!("result.{field_name} = this.{field_name};\\n"));
            }
            body.push_str("return result;");

            let runtime_code = TsStream::from_string(format!(
                "{class_name}.prototype.toJSON = function() {{\\n{body}\\n}};"
            ));

            // ...
        }
    }
}
```

### After (With ts\_template!)

Rust

```
pub fn derive_json_macro(input: TsStream) -> MacroResult {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let fields = class.field_names();

            let runtime_code = ts_template! {
                @{class_name}.prototype.toJSON = function() {
                    const result = {};
                    {#for field in fields}
                        result.@{field} = this.@{field};
                    {/for}
                    return result;
                };
            };

            // ...
        }
    }
}
```

## How It Works

1. **Rust Compile Time:** The template is parsed during macro expansion
2. **String Building:** Generates Rust code that builds a TypeScript string at runtime
3. **Parsing:** The generated string is parsed with oxc to produce a typed AST.
4. **Result:** Returns a `TsStream` that can be returned directly as macro output

## Return Type

`ts_template!` returns a `TsStream`, which is what a macro function returns as its output. If the
generated source fails to parse, the macro reports an error showing the generated TypeScript:

Text

```
Failed to parse generated TypeScript:
User.prototype.toJSON = function( {
    return {};
}
```

This shows you exactly what was generated, making debugging easy!

## Nesting and Regular TypeScript

You can mix template syntax with regular TypeScript. Braces `{}` are recognized as either:

- **Template tags** if they start with `#`, `$`, `:`, or `/`
- **Regular TypeScript blocks** otherwise

Rust

```
ts_template! {
    const config = {
        {#if use_strict}
            strict: true,
        {:else}
            strict: false,
        {/if}
        timeout: 5000
    };
}
```
