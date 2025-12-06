//! /** @derive(Clone) */ macro implementation

use ts_macro_derive::ts_macro_derive;
use ts_quote::body;
use ts_syn::{Data, DeriveInput, TsMacroError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(Clone, description = "Generates a clone() method for deep cloning")]
pub fn derive_clone_macro(mut input: TsStream) -> Result<TsStream, TsMacroError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let field_names: Vec<&str> = class.field_names().collect();
            let has_fields = !field_names.is_empty();

            Ok(body! {
                clone(): @{class_name} {
                    const cloned = Object.create(Object.getPrototypeOf(this));

                    {#if has_fields}
                        {#for field in field_names}
                            cloned.@{field} = this.@{field};
                        {/for}
                    {/if}

                    return cloned;
                }
            })
        }
        Data::Enum(_) => Err(TsMacroError::new(
            input.error_span(),
            "/** @derive(Clone) */ can only be applied to classes",
        )),
        Data::Interface(_) => Err(TsMacroError::new(
            input.error_span(),
            "/** @derive(Clone) */ can only be applied to classes, not interfaces",
        )),
    }
}
