//! The standalone `Encode` functions of an object-shaped type: `encode`,
//! `encodeWithContext` and `encodePartialWithContext`.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::field::statements::encode_fields;
use super::text::encode_text_body;
use super::types::EncodeField;
use crate::builtin::derive::common::{TypeNames, js_string, rendered};
use crate::builtin::return_types::{ENCODE_CONTEXT, FIELD_REQUIREMENT, FIELD_SELECTION};

/// `{camelName}Encode`, which encodes to JSON text without the tag and
/// `__id` unless asked to keep them, and the context-taking encoders of
/// [`encode_with_context_functions`].
pub(super) fn object_encode_functions(
    names: &TypeNames,
    tag_field: &str,
    fields: &[EncodeField],
) -> TsStream {
    let fn_encode_ident = names.generic_function("Encode");
    let fn_encode_internal_expr: Expr = names.generic_call("EncodeWithContext").into();
    let full_type_ident = names.full_type_ident();
    let encode_text = encode_text_body(&fn_encode_internal_expr, tag_field);
    let with_context = encode_with_context_functions(names, tag_field, fields);
    ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
        export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
            {$typescript encode_text}
        }

        {$typescript with_context}
    }
}

/// `{camelName}EncodeWithContext`, which encodes a whole value, and
/// `{camelName}EncodePartialWithContext`, which it runs: that one encodes each
/// field the value has, so it also encodes a `Partial` of the type, and a
/// `Pick` or `Omit` given the selection, whose other fields it leaves out.
pub(super) fn encode_with_context_functions(
    names: &TypeNames,
    tag_field: &str,
    fields: &[EncodeField],
) -> TsStream {
    let fn_encode_internal_ident = names.generic_function("EncodeWithContext");
    let fn_encode_partial_ident = names.generic_function("EncodePartialWithContext");
    let fn_encode_partial_expr: Expr = names.generic_call("EncodePartialWithContext").into();
    let encode_context_ident = ts_ident!(ENCODE_CONTEXT);
    let field_selection_ident = ts_ident!(FIELD_SELECTION);
    let field_requirement_expr: Expr = ts_ident!(FIELD_REQUIREMENT).into();
    let full_type_ident = names.full_type_ident();
    let type_name = names.type_name.as_str();
    // A partial value may lack any field, so each is written only when
    // present, and only when the selection keeps it.
    let field_statements = TsStream::merge_all(fields.iter().map(|field| {
        let present = EncodeField {
            optional: true,
            ..field.clone()
        };
        let statements = encode_fields(std::slice::from_ref(&present), tag_field);
        let name = js_string(&rendered(ts_template! { @{&field.field_ident} }));
        ts_template! {
            if (__fields === undefined || __fields.keeps(@{name})) {
                {$typescript statements}
            }
        }
    }));
    let mut result = ts_template! {
        /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
        export function @{fn_encode_internal_ident}(value: @{&full_type_ident}, ctx: @{&encode_context_ident}): Record<string, unknown> {
            return @{fn_encode_partial_expr}(value, ctx);
        }

        /** Encodes the fields a partial value has, with an existing context. @param value - The value to encode @param ctx - The encoding context @param fields - The fields a `Pick` or `Omit` keeps: every field when left out */
        export function @{fn_encode_partial_ident}(value: Partial<@{&full_type_ident}>, ctx: @{&encode_context_ident}, fields?: @{field_selection_ident}<keyof @{&full_type_ident}>): Record<string, unknown> {
            const __fields = fields === undefined ? undefined : @{field_requirement_expr}(fields);

            // An object encoded by selection is not the one other references
            // to it stand for, so it takes no part in cycle detection.
            let __id: number | undefined;
            if (__fields === undefined) {
                const existingId = ctx.getId(value);
                if (existingId !== undefined) {
                    return { __ref: existingId };
                }
                __id = ctx.register(value);
            }

            const result: Record<string, unknown> = {
                "@{tag_field}": "@{type_name}",
                __id,
            };

            {$typescript field_statements}

            return result;
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    result.add_aliased_type_import("FieldSelection", crate::package::ENDEC);
    result.add_aliased_import("fieldRequirement", crate::package::ENDEC);
    result
}
