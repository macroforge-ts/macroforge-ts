//! The statements that decode one field's raw value onto `instance`, shared by
//! every object-shaped `Decode`: classes, interfaces and object type aliases.
//!
//! The generated code runs inside a `decodeWithContext` body that has `ctx`,
//! `instance`, `obj` and `errors` in scope.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::derive::endec::TypeCategory;
use crate::builtin::derive::endec::decode::helpers::{
    nested_decode_fn_name, nested_decode_result_fn_name, primitive_check,
};
use crate::builtin::derive::endec::decode::types::DecodeField;
use crate::builtin::derive::endec::decode::validation::{Missing, generate_field_validations};
use crate::builtin::derive::endec::value_kind::EndecValueKind;
use crate::builtin::return_types::{DECODE_ERROR, PENDING_REF};

/// Decodes `field`'s raw value, held in `__raw_{field}`, onto `instance`.
/// `owner` names the decoded type in validation messages.
pub(super) fn decode_raw_value(field: &DecodeField, owner: &str) -> TsStream {
    let raw_var_name = format!("__raw_{}", field.field_name);
    let raw_var_ident: Ident = ts_ident!(raw_var_name);
    let has_validators = field.has_validators();
    let pending_ref_expr: Expr = ts_ident!(PENDING_REF).into();
    ts_template! {
        {#match &field.type_cat}
            {:case TypeCategory::Primitive}
                {#if field.decimal_format}
                    {#if has_validators}
                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                        {$typescript validation_code}
                    {/if}
                    {
                        const __numVal = globalThis.Number(@{raw_var_ident});
                        if (globalThis.Number.isNaN(__numVal)) {
                            errors.push({ field: "@{field.json_key}", message: "expected a numeric string, got " + JSON.stringify(@{raw_var_ident}) });
                        } else {
                            instance.@{field.field_ident} = __numVal;
                        }
                    }
                {:else}
                    {#if let Some(primitive) = primitive_check(field, &raw_var_name, owner)}
                        if (@{primitive.mismatch}) {
                            errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                        } else {
                            {#if has_validators}
                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                                {$typescript validation_code}
                            {/if}
                            instance.@{field.field_ident} = @{raw_var_ident};
                        }
                    {:else}
                        {#if has_validators}
                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                            {$typescript validation_code}
                        {/if}
                        instance.@{field.field_ident} = @{raw_var_ident};
                    {/if}
                {/if}

            {:case TypeCategory::Date}
                {
                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                    instance.@{field.field_ident} = __dateVal;
                }

            {:case TypeCategory::Array(inner)}
                if (Array.isArray(@{raw_var_ident})) {
                    {#if has_validators}
                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}
                        {$typescript validation_code}

                    {/if}

                    {#match field.array_elem_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                        {:case EndecValueKind::Date}
                            instance.@{field.field_ident} = @{raw_var_ident}.map(
                                (item) => typeof item === "string" ? new Date(item) : item as Date
                            );
                        {:case EndecValueKind::NullableDate}
                            instance.@{field.field_ident} = @{raw_var_ident}.map(
                                (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                            );
                        {:case _}
                            {#if let Some(elem_type) = &field.array_elem_encodable_type}
                                {$let elem_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(elem_type)).into()}
                                const __arr = @{raw_var_ident}.map((item: @{inner} | { __ref: number }, idx) => {
                                    {#if let Some(guard) = &field.array_elem_union_guard}
                                        // The other member of an element union is recognised
                                        // first; only the user type needs its decoder.
                                        {$let item_expr: Expr = ts_ident!("item").into()}
                                        {$let is_member: Expr = guard.test(&item_expr)}
                                        {$let member_value: Expr = guard.convert(&item_expr)}
                                        if (@{is_member}) {
                                            return @{member_value};
                                        }
                                    {/if}
                                    if (typeof item === "object" && item !== null && "__ref" in item) {
                                        const result = ctx.getOrDefer(item.__ref);
                                        if (@{pending_ref_expr}.is(result)) {
                                            return { __pendingIdx: idx, __refId: result.id };
                                        }
                                        return result;
                                    }
                                    const __elemResult = @{elem_deser_result_fn}(item);
                                    if (!__elemResult.success) {
                                        for (const __err of __elemResult.errors) {
                                            errors.push({
                                                field: __err.field === "_root"
                                                    ? "@{field.json_key}[" + idx + "]"
                                                    : "@{field.json_key}[" + idx + "]." + __err.field,
                                                message: __err.message
                                            });
                                        }
                                    }
                                    return __elemResult.success ? __elemResult.value : item;
                                });
                                instance.@{field.field_ident} = __arr;
                                __arr.forEach((item, idx) => {
                                    if (item && typeof item === "object" && "__pendingIdx" in item) {
                                        ctx.addPatch(instance.@{field.field_ident}, idx, (item as any).__refId);
                                    }
                                });
                            {:else}
                                const __arr = @{raw_var_ident}.map((item: @{inner} | { __ref: number }, idx) => {
                                    if (typeof item === "object" && item !== null && "__ref" in item) {
                                        const result = ctx.getOrDefer(item.__ref);
                                        if (@{pending_ref_expr}.is(result)) {
                                            return { __pendingIdx: idx, __refId: result.id };
                                        }
                                        return result;
                                    }
                                    return item as @{inner};
                                });
                                instance.@{field.field_ident} = __arr;
                                __arr.forEach((item, idx) => {
                                    if (item && typeof item === "object" && "__pendingIdx" in item) {
                                        ctx.addPatch(instance.@{field.field_ident}, idx, (item as any).__refId);
                                    }
                                });
                            {/if}
                    {/match}
                }

            {:case TypeCategory::Map(key_type, value_type)}
                if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                    {#match field.map_value_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            instance.@{field.field_ident} = new Map(
                                Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                            );
                        {:case EndecValueKind::Date}
                            instance.@{field.field_ident} = new Map(
                                Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, typeof v === "string" ? new Date(v) : v as Date])
                            );
                        {:case _}
                            {#if let Some(value_type_name) = &field.map_value_encodable_type}
                                {$let value_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(value_type_name)).into()}
                                instance.@{field.field_ident} = new Map(
                                    Object.entries(@{raw_var_ident}).map(([k, v]) => {
                                        const __vResult = @{value_deser_result_fn}(v);
                                        if (!__vResult.success) {
                                            for (const __err of __vResult.errors) {
                                                errors.push({
                                                    field: __err.field === "_root"
                                                        ? "@{field.json_key}." + k
                                                        : "@{field.json_key}." + k + "." + __err.field,
                                                    message: __err.message
                                                });
                                            }
                                        }
                                        return [k as @{key_type}, __vResult.success ? __vResult.value : v as @{value_type}];
                                    })
                                );
                            {:else}
                                instance.@{field.field_ident} = new Map(
                                    Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                );
                            {/if}
                    {/match}
                }

            {:case TypeCategory::Set(inner)}
                if (Array.isArray(@{raw_var_ident})) {
                    {#match field.set_elem_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                        {:case EndecValueKind::Date}
                            instance.@{field.field_ident} = new Set(
                                @{raw_var_ident}.map((item) => typeof item === "string" ? new Date(item) : item as Date)
                            );
                        {:case _}
                            {#if let Some(elem_type) = &field.set_elem_encodable_type}
                                {$let elem_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(elem_type)).into()}
                                instance.@{field.field_ident} = new Set(
                                    @{raw_var_ident}.map((item, __setIdx) => {
                                        const __elemResult = @{elem_deser_result_fn}(item);
                                        if (!__elemResult.success) {
                                            for (const __err of __elemResult.errors) {
                                                errors.push({
                                                    field: __err.field === "_root"
                                                        ? "@{field.json_key}[" + __setIdx + "]"
                                                        : "@{field.json_key}[" + __setIdx + "]." + __err.field,
                                                    message: __err.message
                                                });
                                            }
                                        }
                                        return __elemResult.success ? __elemResult.value : item;
                                    })
                                );
                            {:else}
                                instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                            {/if}
                    {/match}
                }

            {:case TypeCategory::Record(_key_type, _value_type)}
                if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                    {#if let Some(value_type_name) = &field.record_value_encodable_type}
                        {$let value_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(value_type_name)).into()}
                        instance.@{field.field_ident} = Object.fromEntries(
                            Object.entries(@{raw_var_ident}).map(([k, v]) => {
                                const __vResult = @{value_deser_result_fn}(v);
                                if (!__vResult.success) {
                                    for (const __err of __vResult.errors) {
                                        errors.push({
                                            field: __err.field === "_root"
                                                ? "@{field.json_key}." + k
                                                : "@{field.json_key}." + k + "." + __err.field,
                                            message: __err.message
                                        });
                                    }
                                }
                                return [k, __vResult.success ? __vResult.value : v];
                            })
                        ) as @{field.ts_type};
                    {:else}
                        instance.@{field.field_ident} = @{raw_var_ident};
                    {/if}
                }

            {:case TypeCategory::Wrapper(_)}
                {#if let Some(inner_type_name) = &field.wrapper_encodable_type}
                    {$let inner_deser_fn: Expr = ts_ident!(nested_decode_fn_name(inner_type_name)).into()}
                    {$let nested = decode_nested(field, &inner_deser_fn, &raw_var_ident)}
                    {$typescript nested}
                {:else}
                    instance.@{field.field_ident} = @{raw_var_ident};
                {/if}

            {:case TypeCategory::Encodable(type_name)}
                {$let type_expr: Expr = ts_ident!(nested_decode_fn_name(type_name)).into()}
                {$let nested = decode_nested(field, &type_expr, &raw_var_ident)}
                {#if let Some(guard) = &field.union_guard}
                    {$let raw_expr: Expr = raw_var_ident.clone().into()}
                    {$let is_member: Expr = guard.test(&raw_expr)}
                    {$let member_value: Expr = guard.convert(&raw_expr)}
                    if (@{is_member}) {
                        instance.@{field.field_ident} = @{member_value};
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
                                errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                            } else {
                                {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                                instance.@{field.field_ident} = @{raw_var_ident};
                            }
                        {:else}
                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                            instance.@{field.field_ident} = @{raw_var_ident};
                        {/if}
                    {:case EndecValueKind::Date}
                        if (@{raw_var_ident} === null) {
                            instance.@{field.field_ident} = null;
                        } else {
                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident};
                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, owner, field.missing())}{$typescript validation_code}{/if}
                            instance.@{field.field_ident} = __dateVal;
                        }
                    {:case _}
                        if (@{raw_var_ident} === null) {
                            instance.@{field.field_ident} = null;
                        } else {
                            {#if let Some(inner_type) = &field.nullable_encodable_type}
                                {$let inner_type_expr: Expr = ts_ident!(nested_decode_fn_name(inner_type)).into()}
                                {$let nested = decode_nested(field, &inner_type_expr, &raw_var_ident)}
                                {#if let Some(guard) = &field.nullable_union_guard}
                                    {$let raw_expr: Expr = raw_var_ident.clone().into()}
                                    {$let is_member: Expr = guard.test(&raw_expr)}
                                    {$let member_value: Expr = guard.convert(&raw_expr)}
                                    if (@{is_member}) {
                                        instance.@{field.field_ident} = @{member_value};
                                    } else {
                                        {$typescript nested}
                                    }
                                {:else}
                                    {$typescript nested}
                                {/if}
                            {:else}
                                instance.@{field.field_ident} = @{raw_var_ident};
                            {/if}
                        }
                {/match}

            {:case _}
                {#if let Some(primitive) = primitive_check(field, &raw_var_name, owner)}
                    if (@{primitive.mismatch}) {
                        errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                    } else {
                        instance.@{field.field_ident} = @{raw_var_ident};
                    }
                {:else}
                    instance.@{field.field_ident} = @{raw_var_ident};
                {/if}
        {/match}
    }
}

/// Decodes `raw` onto the field through `decode_fn`, a nested
/// `decodeWithContext`, reporting its errors under the field's key.
fn decode_nested(field: &DecodeField, decode_fn: &Expr, raw: &Ident) -> TsStream {
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    ts_template! {
        ctx.pushScope("@{field.json_key}");
        try {
            const __result = @{decode_fn}(@{raw}, ctx);
            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
        } catch (__e) {
            if (__e instanceof @{decode_error_expr}) {
                for (const __err of __e.errors) {
                    errors.push({
                        field: __err.field === "_root" ? "@{field.json_key}" : "@{field.json_key}." + __err.field,
                        message: __err.message
                    });
                }
            } else {
                throw __e;
            }
        } finally {
            ctx.popScope();
        }
    }
}
