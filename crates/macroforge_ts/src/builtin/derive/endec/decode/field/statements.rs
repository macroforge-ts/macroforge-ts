//! The statements that decode every field of an object-shaped type onto
//! `instance`, shared by classes, interfaces and object type aliases.
//!
//! The generated code runs inside a `decodeWithContext` body that has `ctx`,
//! `instance`, `obj` and `errors` in scope.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::value::decode_raw_value;
use crate::builtin::derive::common::{js_string, rendered};
use crate::builtin::derive::endec::TypeCategory;
use crate::builtin::derive::endec::decode::helpers::nested_decode_fn_name;
use crate::builtin::derive::endec::decode::types::DecodeField;
use crate::builtin::derive::endec::decode::validation::generate_field_validations;
use crate::builtin::return_types::DECODE_ERROR;

/// Which of a type's fields the decoder being generated reads.
#[derive(Clone, Copy)]
pub(in crate::builtin::derive::endec::decode) enum Required {
    /// Every field, each required as its type says.
    All,
    /// Those the `__fields` requirement in scope keeps, when the caller gave
    /// one: the decoder also decodes a `Partial`, `Pick` or `Omit` of its type.
    Selected,
}

/// The condition under which the call being generated requires `field`.
pub(in crate::builtin::derive::endec::decode) fn requires(field: &DecodeField) -> Expr {
    let name = js_string(&field.field_name);
    rendered(ts_template! { (__fields === undefined || __fields.requires(@{name})) }).into()
}

/// The condition under which the call being generated reads `field` at all.
fn keeps(field: &DecodeField) -> Expr {
    let name = js_string(&field.field_name);
    rendered(ts_template! { (__fields === undefined || __fields.keeps(@{name})) }).into()
}

/// Decodes each regular field from its JSON key, then each flattened field
/// from the whole object. `owner` names the decoded type in messages.
pub(in crate::builtin::derive::endec::decode) fn decode_fields(
    fields: &[DecodeField],
    owner: &str,
    required: Required,
) -> TsStream {
    let regular = fields
        .iter()
        .filter(|field| !field.flatten)
        .map(|field| decode_field(field, owner, required));
    let flattened = fields
        .iter()
        .filter(|field| field.flatten)
        .map(|field| when_kept(field, required, decode_flattened_field(field)));
    TsStream::merge_all(regular.chain(flattened))
}

/// One regular field: its `decodeWith` function, or its raw value decoded by
/// type. An optional field is decoded only when present, else its default.
/// A field the call does not keep is not read, so a value never carries a
/// field its shape lacks, and one it keeps without requiring is decoded only
/// when present and gets no default.
fn decode_field(field: &DecodeField, owner: &str, required: Required) -> TsStream {
    let present: Expr = rendered(ts_template! {
        ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined)
    })
    .into();
    let decoded = match &field.decode_with {
        Some(fn_expr) => decode_with(field, fn_expr, owner),
        None => {
            let raw_var_ident: Ident = ts_ident!(format!("__raw_{}", field.field_name));
            let value = decode_raw_value(field, owner);
            ts_template! {
                const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.raw_cast_type};
                {$typescript value}
            }
        }
    };
    let default_expr = field
        .default_expr
        .as_ref()
        .filter(|_| field.decode_with.is_none());
    if field.optional {
        let defaulted: Expr = match required {
            Required::All => rendered(ts_template! { !@{&present} }),
            Required::Selected => {
                let is_required = requires(field);
                rendered(ts_template! { @{is_required} && !@{&present} })
            }
        }
        .into();
        return when_kept(
            field,
            required,
            ts_template! {
                if (@{&present}) {
                    {$typescript decoded}
                }
                {#if let Some(default_expr) = default_expr}
                    if (@{defaulted}) {
                        instance.@{field.field_ident} = @{default_expr};
                    }
                {/if}
            },
        );
    }
    match required {
        Required::All => ts_template! {
            {
                {$typescript decoded}
            }
        },
        Required::Selected => {
            let is_required = requires(field);
            let name = js_string(&field.field_name);
            ts_template! {
                if (@{is_required} || (__fields.keeps(@{name}) && obj["@{field.json_key}"] !== undefined)) {
                    {$typescript decoded}
                }
            }
        }
    }
}

/// `statements`, run only when the call keeps `field`.
fn when_kept(field: &DecodeField, required: Required, statements: TsStream) -> TsStream {
    match required {
        Required::All => statements,
        Required::Selected => {
            let is_kept = keeps(field);
            ts_template! {
                if (@{is_kept}) {
                    {$typescript statements}
                }
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
