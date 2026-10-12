//! The statements that decode one field's raw value onto `instance`, shared by
//! every object-shaped `Decode`: classes, interfaces and object type aliases.
//!
//! The generated code runs inside a `decodeWithContext` body that has `ctx`,
//! `instance`, `obj` and `errors` in scope.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::derive::common::rendered;
use crate::builtin::derive::endec::TypeCategory;
use crate::builtin::derive::endec::decode::helpers::{
    nested_decode_fn_name, primitive_check, wrong_type_message,
};
use crate::builtin::derive::endec::decode::types::{CompositeDecode, DecodeField, RawShape};
use crate::builtin::derive::endec::decode::validation::{Missing, generate_field_validations};
use crate::builtin::derive::endec::value_kind::EndecValueKind;
use crate::builtin::return_types::DECODE_ERROR;

/// Decodes `field`'s raw value, held in `__raw_{field}`, onto `instance`.
/// `owner` names the decoded type in validation messages.
pub(super) fn decode_raw_value(field: &DecodeField, owner: &str) -> TsStream {
    let raw_var_name = field.raw_var_name();
    let raw_var_ident: Ident = ts_ident!(raw_var_name);
    let has_validators = field.has_validators();
    if let (TypeCategory::Optional(_), Some(member)) = (&field.type_cat, &field.present_inner) {
        let member_value = decode_raw_value(member, owner);
        return ts_template! {
            if (@{raw_var_ident} === undefined) {
                instance@{field.member} = undefined;
            } else {
                {$typescript member_value}
            }
        };
    }
    if let Some(composite) = &field.composite {
        return decode_composite(field, composite, owner);
    }
    if let Some(decoded) = &field.by_parameter {
        return decode_nested_call(field, &Expr::Source(decoded.clone()));
    }
    // A date is a `Date`, or the text of one: anything else, and text that
    // spells no date, is no value of the field.
    let decoded_date: Expr = rendered(ts_template! {
        (@{&raw_var_ident} instanceof Date ? @{&raw_var_ident} : typeof @{&raw_var_ident} === "string" ? new Date(@{&raw_var_ident}) : undefined)
    })
    .into();
    let date_mismatch = wrong_type_message(field, &raw_var_name, owner);
    ts_template! {
        {#match &field.type_cat}
            {:case TypeCategory::Primitive}
                {#if field.decimal_format}
                    {#if has_validators}
                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                        {$typescript validation_code}
                    {/if}
                    {
                        // A decimal is a number, or the text of one: `Number` alone
                        // would take `""`, `null` and `true` for numbers too.
                        const __rawNum: unknown = @{raw_var_ident};
                        const __numVal = typeof __rawNum === "number" ? __rawNum : typeof __rawNum === "string" && __rawNum.trim() !== "" ? globalThis.Number(__rawNum) : globalThis.Number.NaN;
                        if (globalThis.Number.isNaN(__numVal)) {
                            errors.push({ field: "@{field.json_key_text()}", message: "expected a numeric string, got " + JSON.stringify(@{raw_var_ident}) });
                        } else {
                            instance@{field.member} = __numVal;
                        }
                    }
                {:else}
                    {#if let Some(primitive) = primitive_check(field, &raw_var_name, owner)}
                        if (@{primitive.mismatch}) {
                            errors.push({ field: "@{field.json_key_text()}", message: @{primitive.message} });
                        } else {
                            {#if has_validators}
                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                                {$typescript validation_code}
                            {/if}
                            instance@{field.member} = @{raw_var_ident};
                        }
                    {:else}
                        {#if has_validators}
                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                            {$typescript validation_code}
                        {/if}
                        instance@{field.member} = @{raw_var_ident};
                    {/if}
                {/if}

            {:case TypeCategory::Date}
                {
                    const __dateVal = @{&decoded_date};
                    if (__dateVal === undefined || globalThis.Number.isNaN(__dateVal.getTime())) {
                        errors.push({ field: "@{field.json_key_text()}", message: @{&date_mismatch} });
                    } else {
                        {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                        instance@{field.member} = __dateVal;
                    }
                }

            {:case TypeCategory::Wrapper(_)}
                {#if let Some(wrapped) = &field.wrapped}
                    {$let decoded: Expr = Expr::Source(wrapped.decode_call(&raw_var_name))}
                    {$let nested = decode_nested_call(field, &decoded)}
                    {$typescript nested}
                {:else}
                    instance@{field.member} = @{raw_var_ident};
                {/if}

            {:case TypeCategory::Encodable(type_name)}
                {$let nested = match &field.wrapped {
                    Some(wrapped) => {
                        decode_nested_call(field, &Expr::Source(wrapped.decode_call(&raw_var_name)))
                    }
                    // A type named in lowercase is only known by its name.
                    None => {
                        let type_expr: Expr = ts_ident!(nested_decode_fn_name(type_name)).into();
                        decode_nested(field, &type_expr, &raw_var_ident)
                    }
                }}
                {#if let Some(guard) = &field.union_guard}
                    {$let raw_expr: Expr = raw_var_ident.clone().into()}
                    {$let is_member: Expr = guard.test(&raw_expr)}
                    {$let member_value: Expr = guard.convert(&raw_expr)}
                    if (@{is_member}) {
                        instance@{field.member} = @{member_value};
                        {#if field.has_union_string_validators()}
                            {$let usv_code = generate_field_validations(&field.union_string_validators, &raw_var_name, &field.json_key, owner, Missing::Excluded)}
                            {$typescript usv_code}
                        {/if}
                    } else {
                        {$typescript nested}
                    }
                {:else}
                    {$typescript nested}
                {/if}

            {:case TypeCategory::Nullable(_)}
                {#match field.nullable_inner_kind.unwrap_or(EndecValueKind::Other)}
                    {:case EndecValueKind::PrimitiveLike}
                        {#if let Some(primitive) = primitive_check(field, &raw_var_name, owner)}
                            if (@{primitive.mismatch}) {
                                errors.push({ field: "@{field.json_key_text()}", message: @{primitive.message} });
                            } else {
                                {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                                instance@{field.member} = @{raw_var_ident};
                            }
                        {:else}
                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                            instance@{field.member} = @{raw_var_ident};
                        {/if}
                    {:case EndecValueKind::Date}
                        if (@{raw_var_ident} === null) {
                            instance@{field.member} = null;
                        } else {
                            const __dateVal = @{&decoded_date};
                            if (__dateVal === undefined || globalThis.Number.isNaN(__dateVal.getTime())) {
                                errors.push({ field: "@{field.json_key_text()}", message: @{&date_mismatch} });
                            } else {
                                {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                                instance@{field.member} = __dateVal;
                            }
                        }
                    {:case _}
                        if (@{raw_var_ident} === null) {
                            instance@{field.member} = null;
                        } else {
                            {#if let Some(member) = &field.present_inner}
                                {$let member_value = decode_raw_value(member, owner)}
                                {$typescript member_value}
                            {:else}
                            {#if let Some(inner_type) = &field.nullable_encodable_type}
                                {$let nested = match &field.nullable_nested {
                                    Some(nullable_nested) => decode_nested_call(
                                        field,
                                        &Expr::Source(nullable_nested.decode_call(&raw_var_name)),
                                    ),
                                    None => {
                                        let inner_type_expr: Expr =
                                            ts_ident!(nested_decode_fn_name(inner_type)).into();
                                        decode_nested(field, &inner_type_expr, &raw_var_ident)
                                    }
                                }}
                                {#if let Some(guard) = &field.nullable_union_guard}
                                    {$let raw_expr: Expr = raw_var_ident.clone().into()}
                                    {$let is_member: Expr = guard.test(&raw_expr)}
                                    {$let member_value: Expr = guard.convert(&raw_expr)}
                                    if (@{is_member}) {
                                        instance@{field.member} = @{member_value};
                                    } else {
                                        {$typescript nested}
                                    }
                                {:else}
                                    {$typescript nested}
                                {/if}
                            {:else}
                                instance@{field.member} = @{raw_var_ident};
                            {/if}
                            {/if}
                        }
                {/match}

            {:case _}
                {#if let Some(primitive) = primitive_check(field, &raw_var_name, owner)}
                    if (@{primitive.mismatch}) {
                        errors.push({ field: "@{field.json_key_text()}", message: @{primitive.message} });
                    } else {
                        {#if has_validators}
                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                            {$typescript validation_code}
                        {/if}
                        instance@{field.member} = @{raw_var_ident};
                    }
                {:else}
                    {#if has_validators}
                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                        {$typescript validation_code}
                    {/if}
                    instance@{field.member} = @{raw_var_ident};
                {/if}
        {/match}
    }
}

/// Decodes `raw` onto the field through `decode_fn`, a nested
/// `decodeWithContext`, reporting its errors under the field's key.
fn decode_nested(field: &DecodeField, decode_fn: &Expr, raw: &Ident) -> TsStream {
    let decoded: Expr = rendered(ts_template! { @{decode_fn}(@{raw}, ctx) }).into();
    decode_nested_call(field, &decoded)
}

/// Decodes the raw value of a collection or a tuple onto the field, once it
/// is known to be an array or an object, with its members' errors reported
/// under the field's key.
fn decode_composite(field: &DecodeField, composite: &CompositeDecode, owner: &str) -> TsStream {
    let raw_var_name = field.raw_var_name();
    let raw_var_ident: Ident = ts_ident!(&raw_var_name);
    let wrong_type = wrong_type_message(field, &raw_var_name, owner);
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    let validation_code = field.has_validators().then(|| {
        generate_field_validations(
            &field.validators,
            &raw_var_name,
            &field.json_key,
            owner,
            field.missing(),
        )
    });
    let has_shape = match composite.shape {
        RawShape::Array => rendered(ts_template! { Array.isArray(@{&raw_var_ident}) }),
        RawShape::Object => rendered(
            ts_template! { typeof @{&raw_var_ident} === "object" && @{&raw_var_ident} !== null },
        ),
    };
    ts_template! {
        if (@{has_shape}) {
            {#if let Some(validation_code) = validation_code}
                {$typescript validation_code}
            {/if}
            {#if let Some(decoded) = &composite.decoded}
                ctx.pushScope("@{field.json_key_text()}");
                try {
                    instance@{field.member} = @{decoded};
                } catch (__e) {
                    if (__e instanceof @{decode_error_expr}) {
                        for (const __err of __e.errors) {
                            errors.push({
                                field: __err.field === "_root" ? "@{field.json_key_text()}" : "@{field.json_key_text()}." + __err.field,
                                message: __err.message
                            });
                        }
                    } else {
                        throw __e;
                    }
                } finally {
                    ctx.popScope();
                }
            {:else}
                instance@{field.member} = @{&raw_var_ident} as @{field.ts_type};
            {/if}
        } else {
            errors.push({ field: "@{field.json_key_text()}", message: @{wrong_type} });
        }
    }
}

/// Runs `decoded`, a call decoding the field's raw value through another
/// type's decoder, and assigns its result, with that type's errors, and
/// anything else it throws, reported under this field.
fn decode_nested_call(field: &DecodeField, decoded: &Expr) -> TsStream {
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    ts_template! {
        ctx.pushScope("@{field.json_key_text()}");
        try {
            const __result = @{decoded};
            ctx.assignOrDefer(instance, "@{field.field_name_text()}", __result);
        } catch (__e) {
            if (__e instanceof @{decode_error_expr}) {
                for (const __err of __e.errors) {
                    errors.push({
                        field: __err.field === "_root" ? "@{field.json_key_text()}" : "@{field.json_key_text()}." + __err.field,
                        message: __err.message
                    });
                }
            } else {
                // Whatever a decoder throws is the field's failure, as a
                // collection takes its member's: a foreign handler handed in
                // for a type parameter throws what it likes.
                errors.push({ field: "@{field.json_key_text()}", message: __e instanceof Error ? __e.message : String(__e) });
            }
        } finally {
            ctx.popScope();
        }
    }
}
