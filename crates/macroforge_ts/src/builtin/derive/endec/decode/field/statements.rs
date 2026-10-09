//! The statements that decode every field of an object-shaped type onto
//! `instance`, shared by classes, interfaces and object type aliases.
//!
//! The generated code runs inside a `decodeWithContext` body that has `ctx`,
//! `instance`, `obj` and `errors` in scope.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::value::decode_raw_value;
use crate::builtin::derive::endec::TypeCategory;
use crate::builtin::derive::endec::decode::helpers::nested_decode_fn_name;
use crate::builtin::derive::endec::decode::types::DecodeField;
use crate::builtin::derive::endec::decode::validation::generate_field_validations;
use crate::builtin::return_types::DECODE_ERROR;

/// Decodes each regular field from its JSON key, then each flattened field
/// from the whole object. `owner` names the decoded type in messages.
pub(in crate::builtin::derive::endec::decode) fn decode_fields(
    fields: &[DecodeField],
    owner: &str,
) -> TsStream {
    let regular = fields
        .iter()
        .filter(|field| !field.flatten)
        .map(|field| decode_field(field, owner));
    let flattened = fields
        .iter()
        .filter(|field| field.flatten)
        .map(decode_flattened_field);
    TsStream::merge_all(regular.chain(flattened))
}

/// One regular field: its `decodeWith` function, or its raw value decoded by
/// type. An optional field is decoded only when present, else its default.
fn decode_field(field: &DecodeField, owner: &str) -> TsStream {
    if let Some(fn_expr) = &field.decode_with {
        let custom = decode_with(field, fn_expr, owner);
        return if field.optional {
            ts_template! {
                if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                    {$typescript custom}
                }
            }
        } else {
            custom
        };
    }
    let raw_var_ident: Ident = ts_ident!(format!("__raw_{}", field.field_name));
    let value = decode_raw_value(field, owner);
    if field.optional {
        ts_template! {
            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.raw_cast_type};
                {$typescript value}
            }
            {#if let Some(default_expr) = &field.default_expr}
                if (!("@{field.json_key}" in obj) || obj["@{field.json_key}"] === undefined) {
                    instance.@{field.field_ident} = @{default_expr};
                }
            {/if}
        }
    } else {
        ts_template! {
            {
                const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.raw_cast_type};
                {$typescript value}
            }
        }
    }
}

/// A field decoded by its `@endec(decodeWith)` function, validated after. A
/// conversion that throws is reported under the field.
fn decode_with(field: &DecodeField, fn_expr: &Expr, owner: &str) -> TsStream {
    if field.has_validators() {
        let validation_code = generate_field_validations(
            &field.validators,
            "__convertedVal",
            &field.json_key,
            owner,
            field.missing(),
        );
        ts_template! {
            try {
                const __convertedVal = @{fn_expr}(obj["@{field.json_key}"]);
                {$typescript validation_code}
                instance.@{field.field_ident} = __convertedVal;
            } catch (__error) {
                errors.push({ field: "@{field.json_key}", message: __error instanceof Error ? __error.message : String(__error) });
            }
        }
    } else {
        ts_template! {
            try {
                instance.@{field.field_ident} = @{fn_expr}(obj["@{field.json_key}"]);
            } catch (__error) {
                errors.push({ field: "@{field.json_key}", message: __error instanceof Error ? __error.message : String(__error) });
            }
        }
    }
}

/// An `@endec(flatten)` field, decoded from the enclosing object itself.
fn decode_flattened_field(field: &DecodeField) -> TsStream {
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    match &field.type_cat {
        TypeCategory::Encodable(type_name) => {
            let decode_fn: Expr = ts_ident!(nested_decode_fn_name(type_name)).into();
            ts_template! {
                try {
                    const __result = @{decode_fn}(obj, ctx);
                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                } catch (__e) {
                    if (__e instanceof @{decode_error_expr}) {
                        for (const __err of __e.errors) {
                            errors.push(__err);
                        }
                    } else {
                        throw __e;
                    }
                }
            }
        }
        _ => ts_template! {
            instance.@{field.field_ident} = obj as any;
        },
    }
}
