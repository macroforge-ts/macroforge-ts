## Complete Example

```rust
use macroforge_ts::macros::{ts_macro_derive, body};
use macroforge_ts::ts_syn::{
    Data, DeriveInput, FieldIR, MacroforgeError, TsStream, parse_ts_macro_input,
};

// Helper function to check if a field has a decorator
fn has_decorator(field: &FieldIR, name: &str) -> bool {
    field.decorators.iter().any(|d| d.name.eq_ignore_ascii_case(name))
}

#[ts_macro_derive(
    Validate,
    description = "Generates a validate() method",
    attributes(validate)
)]
pub fn derive_validate(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let validations: Vec<_> = class.fields()
                .iter()
                .filter(|f| has_decorator(f, "validate"))
                .collect();

            Ok(body! {
                validate(): string[] {
                    const errors: string[] = [];
                    {#for field in validations}
                        if (!this.@{field.name}) {
                            errors.push("@{field.name} is required");
                        }
                    {/for}
                    return errors;
                }
            })
        }
        _ => Err(MacroforgeError::new(
            input.decorator_span(),
            "@derive(Validate) only works on classes",
        )),
    }
}
```

## Next Steps

- [Learn the template syntax]({base}/docs/custom-macros/ts-quote)