//! /** @derive(Clone) */ macro implementation

use crate::macros::{ts_macro_derive, body, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

#[ts_macro_derive(Clone, description = "Generates a clone() method for deep cloning")]
pub fn derive_clone_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
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
        Data::Enum(_) => Err(MacroforgeError::new(
            input.error_span(),
            "/** @derive(Clone) */ can only be applied to classes or interfaces",
        )),
        Data::Interface(interface) => {
            let interface_name = input.name();
            let field_names: Vec<&str> = interface.field_names().collect();
            let has_fields = !field_names.is_empty();

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function clone(self: @{interface_name}): @{interface_name} {
                        return {
                            {#if has_fields}
                                {#for field in field_names}
                                    @{field}: self.@{field},
                                {/for}
                            {/if}
                        };
                    }
                }
            })
        }
    }
}
