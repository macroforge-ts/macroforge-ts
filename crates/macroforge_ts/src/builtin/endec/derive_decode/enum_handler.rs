use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{DeriveInput, MacroforgeError, TsStream, ts_ident};

use crate::builtin::return_types::DECODE_CONTEXT;
use convert_case::{Case, Casing};

pub(super) fn handle_enum(
    input: &DeriveInput,
    enum_data: &crate::ts_syn::derive::DataEnum,
) -> Result<TsStream, MacroforgeError> {
    let enum_name = input.name();
    let enum_ident = ts_ident!(enum_name);
    let fn_decode_ident = ts_ident!("{}Decode", enum_name.to_case(Case::Camel));
    let fn_decode_internal_ident = ts_ident!("{}DecodeWithContext", enum_name.to_case(Case::Camel));
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
    let fn_is_ident = ts_ident!("{}Is", enum_name.to_case(Case::Camel));
    let variant_expressions: Vec<Expr> = enum_data
        .variants()
        .iter()
        .map(|variant| {
            serde_json::to_string(&variant.name)
                .map(|name| Expr::Source(format!("{enum_name}[{name}]")))
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
            return @{fn_decode_internal_expr}(input);
        }

        /** Decodes with an existing context (for consistency with other types). */
        export function @{fn_decode_internal_ident}(data: unknown, ctx?: @{ts_ident!(DECODE_CONTEXT)}): @{&enum_ident};
        export function @{fn_decode_internal_ident}(data: unknown): @{&enum_ident} {
            {#for variant_expression in &variant_expressions}
                if (@{variant_expression} === data) {
                    return @{variant_expression};
                }
            {/for}
            throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
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
    Ok(result)
}
