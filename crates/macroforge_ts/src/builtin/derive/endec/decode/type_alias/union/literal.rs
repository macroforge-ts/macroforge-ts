//! A union of literals only, decoded by matching each literal.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::return_types::decode_return_type;
use convert_case::{Case, Casing};

/// `decode`, `decodeWithContext`, `hasShape` and `is` for a union of
/// literals: no JSON parsing, no context and no references.
pub(super) fn literal_union(u: &Union) -> TsStream {
    let Union {
        literals,
        type_name,
        decode_context_ident,
        decode_error_expr,
        pending_ref_ident,
        generic_decl,
        full_type_name,
        full_type_ident,
        ..
    } = u;
    let fn_decode_ident = ts_ident!("{}Decode{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        type_name.to_case(Case::Camel),
        generic_decl
    );
    let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_ident = ts_ident!("{}HasShape", type_name.to_case(Case::Camel));
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();
    let return_type = decode_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());

    let mut result = ts_template! {
        /** Decodes a literal union value. Validates input against known variants. @param input - Value to validate @returns Result containing the validated value or error */
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
        export function @{fn_decode_internal_ident}(value: unknown, _ctx: @{decode_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
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
    result
}
