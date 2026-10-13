//! `Encode` for an enum: its values are their own wire form.

use crate::macros::ts_template;
use crate::ts_syn::{DeriveInput, MacroforgeError, TsStream, ts_ident};

use crate::builtin::derive::common::function_prefix;

use super::text::encoded_text_helper;
use crate::builtin::return_types::ENCODE_CONTEXT;

pub(super) fn handle_enum(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    // Enums: return the underlying value directly
    let enum_name = input.name();
    let enum_ident = ts_ident!(enum_name);

    let fn_name_ident = ts_ident!("{}Encode", function_prefix(enum_name));
    let fn_name_internal_ident = ts_ident!("{}EncodeWithContext", function_prefix(enum_name));
    let encoded_text = encoded_text_helper();
    let mut result = ts_template! {
        /** Encodes this enum value to a JSON string. */
        export function @{fn_name_ident}(value: @{enum_ident}): string {
            return @{encoded_text}(value);
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
