use crate::macros::ts_template;
use crate::swc_ecma_ast::Expr;
use crate::ts_syn::{DeriveInput, MacroforgeError, TsStream, ts_ident};

use convert_case::{Case, Casing};

pub(super) fn handle_enum(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    let enum_name = input.name();
    let enum_ident = ts_ident!(enum_name);
    let enum_expr: Expr = enum_ident.clone().into();
    let fn_deserialize_ident = ts_ident!("{}Deserialize", enum_name.to_case(Case::Camel));
    let fn_deserialize_internal_ident =
        ts_ident!("{}DeserializeWithContext", enum_name.to_case(Case::Camel));
    let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
    let fn_is_ident = ts_ident!("{}Is", enum_name.to_case(Case::Camel));
    let mut result = ts_template! {
        /** Deserializes input to an enum value. @param input - Value to deserialize @returns The enum value @throws Error if the value is not a valid enum member */
        export function @{fn_deserialize_ident}(input: unknown): @{&enum_ident} {
            return @{fn_deserialize_internal_expr}(input);
        }

        /** Deserializes with an existing context (for consistency with other types). */
        export function @{fn_deserialize_internal_ident}(data: unknown): @{&enum_ident} {
            for (const key of Object.keys(@{&enum_expr})) {
                const enumValue = @{&enum_expr}[key as keyof typeof @{&enum_ident}];
                if (enumValue === data) {
                    return data as @{&enum_ident};
                }
            }
            throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
        }

        export function @{fn_is_ident}(value: unknown): value is @{&enum_ident} {
            for (const key of Object.keys(@{&enum_expr})) {
                const enumValue = @{&enum_expr}[key as keyof typeof @{&enum_ident}];
                if (enumValue === value) {
                    return true;
                }
            }
            return false;
        }
    };

    result.add_aliased_import("DeserializeContext", "macroforge/serde");
    Ok(result)
}
