use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{DeriveInput, MacroforgeError, TsStream, ts_ident};

use crate::builtin::derive::common::function_prefix;
use crate::builtin::derive::common::rendered;
use crate::builtin::return_types::{DECODE_CONTEXT, DECODE_ERROR};

pub(super) fn handle_enum(
    input: &DeriveInput,
    enum_data: &crate::ts_syn::derive::DataEnum,
) -> Result<TsStream, MacroforgeError> {
    let enum_name = input.name();
    let enum_ident = ts_ident!(enum_name);
    let fn_decode_ident = ts_ident!("{}Decode", function_prefix(enum_name));
    let fn_decode_internal_ident = ts_ident!("{}DecodeWithContext", function_prefix(enum_name));
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
    let fn_is_ident = ts_ident!("{}Is", function_prefix(enum_name));
    let fn_is_expr: Expr = fn_is_ident.clone().into();
    let text_or_value: Expr = ts_ident!(crate::builtin::derive::endec::coder::endec_helper(
        "textOrValue"
    ))
    .into();
    let variant_expressions: Vec<Expr> = enum_data
        .variants()
        .iter()
        .map(|variant| {
            serde_json::to_string(&variant.name)
                .map(|name| Expr::Source(rendered(ts_template! { @{enum_name}[@{name}] })))
                .map_err(|error| {
                    MacroforgeError::new_global(format!(
                        "failed to quote enum member {}.{}: {error}",
                        enum_name, variant.name
                    ))
                })
        })
        .collect::<Result<_, _>>()?;
    let mut result = ts_template! {
        /** Decodes input to an enum value. @param input - Value to decode @returns The enum value @throws Error if the value is not a valid enum member */
        export function @{fn_decode_ident}(input: unknown): @{&enum_ident} {
            // A string may be a member itself or the JSON text of one.
            return @{fn_decode_internal_expr}(@{text_or_value}(input, @{&fn_is_expr}));
        }

        /** Decodes with an existing context (for consistency with other types). */
        export function @{fn_decode_internal_ident}(data: unknown, ctx?: @{ts_ident!(DECODE_CONTEXT)}): @{&enum_ident};
        export function @{fn_decode_internal_ident}(data: unknown): @{&enum_ident} {
            {#for variant_expression in &variant_expressions}
                if (@{variant_expression} === data) {
                    return @{variant_expression};
                }
            {/for}
            throw new @{ts_ident!(DECODE_ERROR)}([{ field: "_root", message: "Invalid @{enum_name} value: " + JSON.stringify(data) }]);
        }

        export function @{fn_is_ident}(value: unknown): value is @{&enum_ident} {
            {#for variant_expression in &variant_expressions}
                if (@{variant_expression} === value) {
                    return true;
                }
            {/for}
            return false;
        }
    };

    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    Ok(result)
}
