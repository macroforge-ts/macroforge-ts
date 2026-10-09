//! The standalone `Encode` functions of an object-shaped interface or type
//! alias: `encode` and `encodeWithContext`.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::field::statements::encode_fields;
use super::types::EncodeField;
use crate::builtin::derive::common::TypeNames;
use crate::builtin::return_types::ENCODE_CONTEXT;

/// `{camelName}Encode`, which encodes to JSON text without the tag and
/// `__id` unless asked to keep them, and `{camelName}EncodeWithContext`,
/// which tracks identity for cycles and encodes each field.
pub(super) fn object_encode_functions(
    names: &TypeNames,
    tag_field: &str,
    fields: &[EncodeField],
) -> TsStream {
    let fn_encode_ident = names.generic_function("Encode");
    let fn_encode_internal_ident = names.generic_function("EncodeWithContext");
    let fn_encode_internal_expr: Expr = names.generic_call("EncodeWithContext").into();
    let encode_context_ident = ts_ident!(ENCODE_CONTEXT);
    let encode_context_expr: Expr = encode_context_ident.clone().into();
    let full_type_ident = names.full_type_ident();
    let type_name = names.type_name.as_str();
    let field_statements = encode_fields(fields, tag_field);
    let mut result = ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
            const ctx = @{encode_context_expr}.create();
            const __raw = @{fn_encode_internal_expr}(value, ctx);
            if (keepMetadata) return JSON.stringify(__raw);
            return JSON.stringify(__raw, (key, val) => key === "@{tag_field}" || key === "__id" ? undefined : val);
        }

        /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
        export function @{fn_encode_internal_ident}(value: @{full_type_ident}, ctx: @{encode_context_ident}): Record<string, unknown> {
            // Check if already encoded (cycle detection)
            const existingId = ctx.getId(value);
            if (existingId !== undefined) {
                return { __ref: existingId };
            }

            // Register this object
            const __id = ctx.register(value);

            const result: Record<string, unknown> = {
                "@{tag_field}": "@{type_name}",
                __id,
            };

            {$typescript field_statements}

            return result;
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    result
}
