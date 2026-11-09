//! A union of literals only, decoded by matching each literal.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::derive::common::function_prefix;
use crate::builtin::return_types::decode_return_type;

/// `decode`, `fromString`, `decodeWithContext`, `hasShape` and `is` for a
/// union of literals: no context and no references.
pub(super) fn literal_union(u: &Union) -> TsStream {
    let Union {
        literals,
        type_name,
        decode_context_ident,
        decode_error_expr,
        pending_ref_ident,
        generic_decl,
        coders,
        full_type_name,
        full_type_ident,
        ..
    } = u;
    let fn_decode_ident = ts_ident!("{}Decode{}", function_prefix(type_name), generic_decl);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        function_prefix(type_name),
        generic_decl
    );
    let fn_is_ident = ts_ident!("{}Is{}", function_prefix(type_name), generic_decl);
    let fn_has_shape_ident = ts_ident!("{}HasShape", function_prefix(type_name));
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();
    let return_type = decode_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());

    let mut result = ts_template! {
        /** Decodes a value to a member of this literal union; JSON text decodes through `fromString`. @param input - The value to check @returns Result containing the validated value or error */
        export function @{fn_decode_ident}(input: unknown): @{return_type_ident} {
            switch (input) {
                {#for lit in literals}
                case @{lit}:
                {/for}
                    return { success: true, value: input as @{full_type_ident} };
                default:
                    return { success: false, errors: [{ field: "_root", message: "Invalid value for @{type_name}: expected one of " + [@{literals.join(", ")}].map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(input) }] };
            }
        }

        /** Decodes with an existing context (validates against known variants). A literal holds no references, so the context goes unread. */
        export function @{fn_decode_internal_ident}(value: unknown, _ctx: @{decode_context_ident}@{coders}): @{full_type_ident} | @{pending_ref_ident} {
            switch (value) {
                {#for lit in literals}
                case @{lit}:
                {/for}
                    return value as @{full_type_ident};
                default:
                    throw new @{decode_error_expr}([{
                        field: "_root",
                        message: "Invalid value for @{type_name}: expected one of " + [@{literals.join(", ")}].map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
                    }]);
            }
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
            switch (value) {
                {#for lit in literals}
                case @{lit}:
                {/for}
                    return true;
                default:
                    return false;
            }
        }

        export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
            return @{fn_has_shape_expr}(value);
        }
    };
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);
    let from_string_name = format!("{}FromString{}", function_prefix(type_name), generic_decl);
    let decode_call = format!("{}Decode", function_prefix(type_name));
    result.merge(super::super::super::from_string::from_string_function(
        &super::super::super::from_string::TextDecoder {
            name: &from_string_name,
            decode: &decode_call,
            coders: "",
            passed_coders: "",
            return_type: &return_type,
            takes_options: false,
        },
    ))
}
