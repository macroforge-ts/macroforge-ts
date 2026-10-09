//! `Encode` for an enum: its values are their own wire form.

use crate::macros::ts_template;
use crate::ts_syn::{DeriveInput, MacroforgeError, TsStream, ts_ident};

use convert_case::{Case, Casing};

use crate::builtin::return_types::ENCODE_CONTEXT;

pub(super) fn handle_enum(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    // Enums: return the underlying value directly
    let enum_name = input.name();
    let enum_ident = ts_ident!(enum_name);

    let fn_name_ident = ts_ident!("{}Encode", enum_name.to_case(Case::Camel));
    let fn_name_internal_ident = ts_ident!("{}EncodeWithContext", enum_name.to_case(Case::Camel));
    let mut result = ts_template! {
        /** Encodes this enum value to a JSON string. */
        export function @{fn_name_ident}(value: @{enum_ident}): string {
            return JSON.stringify(value);
        }

        /** Encodes with an existing context for nested/cyclic object graphs. */
        export function @{fn_name_internal_ident}(value: @{enum_ident}, ctx: @{ts_ident!(ENCODE_CONTEXT)}): string | number;
        export function @{fn_name_internal_ident}(value: @{enum_ident}): string | number {
            return value;
        }
    };
    result.add_aliased_type_import("EncodeContext", crate::package::ENDEC);
    Ok(result)
}
