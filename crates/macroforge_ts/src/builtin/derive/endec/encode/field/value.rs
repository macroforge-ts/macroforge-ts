//! The statement that encodes one regular field of an object-shaped type
//! onto `result`, shared by classes, interfaces and object type aliases.
//!
//! The generated code runs inside an `encodeWithContext` body that has
//! `value`, `ctx` and `result` in scope.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::derive::common::rendered;
use crate::builtin::derive::endec::TypeCategory;
use crate::builtin::derive::endec::encode::types::{EncodeField, nested_encode_fn_name};
use crate::builtin::derive::endec::union_guard::UnionGuard;
use crate::builtin::derive::endec::value_kind::EndecValueKind;

/// Encodes `field` from `value` onto `result` under its JSON key: through its
/// `encodeWith` function, or by its type.
pub(super) fn encode_field(field: &EncodeField) -> TsStream {
    ts_template! {
        {#if let Some(fn_name) = &field.encode_with}
            // Custom encoding function (encodeWith) - wrapped as IIFE for arrow functions
            {#if field.optional}
                if (value.@{field.field_ident} !== undefined) {
                    result.@{field.json_key_ident} = @{fn_name}(value.@{field.field_ident});
                }
            {:else}
                result.@{field.json_key_ident} = @{fn_name}(value.@{field.field_ident});
            {/if}
        {:else}
        {#match &field.type_cat}
            {:case TypeCategory::Primitive}
                {#if field.decimal_format}
                    {#if field.optional}
                        if (value.@{field.field_ident} !== undefined) {
                            result.@{field.json_key_ident} = String(value.@{field.field_ident});
                        }
                    {:else}
                        result.@{field.json_key_ident} = String(value.@{field.field_ident});
                    {/if}
                {:else}
                    {#if field.optional}
                        if (value.@{field.field_ident} !== undefined) {
                            result.@{field.json_key_ident} = value.@{field.field_ident};
                        }
                    {:else}
                        result.@{field.json_key_ident} = value.@{field.field_ident};
                    {/if}
                {/if}

            {:case TypeCategory::Date}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        result.@{field.json_key_ident} = value.@{field.field_ident}.toISOString();
                    }
                {:else}
                    result.@{field.json_key_ident} = value.@{field.field_ident}.toISOString();
                {/if}

            {:case TypeCategory::Array(_)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#match field.array_elem_kind.unwrap_or(EndecValueKind::Other)}
                            {:case EndecValueKind::PrimitiveLike}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {:case EndecValueKind::Date}
                                result.@{field.json_key_ident} = value.@{field.field_ident}.map((item: Date) => item.toISOString());
                            {:case EndecValueKind::NullableDate}
                                result.@{field.json_key_ident} = value.@{field.field_ident}.map((item: Date | null) => item === null ? null : item.toISOString());
                            {:case _}
                                {#if let Some(elem_type) = &field.array_elem_encodable_type}
                                    {$let encode_with_context_elem: Expr = ts_ident!(nested_encode_fn_name(elem_type)).into()}
                                    {$let item_expr: Expr = ts_ident!("item").into()}
                                    {$let encoded_item = guarded(field.array_elem_union_guard.as_ref(), &item_expr, &encode_with_context_elem)}
                                    result.@{field.json_key_ident} = value.@{field.field_ident}.map(
                                        (item) => {$typescript encoded_item}
                                    );
                                {:else}
                                    result.@{field.json_key_ident} = value.@{field.field_ident};
                                {/if}
                        {/match}
                    }
                {:else}
                    {#match field.array_elem_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            result.@{field.json_key_ident} = value.@{field.field_ident};
                        {:case EndecValueKind::Date}
                            result.@{field.json_key_ident} = value.@{field.field_ident}.map((item: Date) => item.toISOString());
                        {:case EndecValueKind::NullableDate}
                            result.@{field.json_key_ident} = value.@{field.field_ident}.map((item: Date | null) => item === null ? null : item.toISOString());
                        {:case _}
                            {#if let Some(elem_type) = &field.array_elem_encodable_type}
                                {$let encode_with_context_elem: Expr = ts_ident!(nested_encode_fn_name(elem_type)).into()}
                                {$let item_expr: Expr = ts_ident!("item").into()}
                                {$let encoded_item = guarded(field.array_elem_union_guard.as_ref(), &item_expr, &encode_with_context_elem)}
                                result.@{field.json_key_ident} = value.@{field.field_ident}.map(
                                    (item) => {$typescript encoded_item}
                                );
                            {:else}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {/if}
                    {/match}
                {/if}

            {:case TypeCategory::Map(_, _)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#match field.map_value_kind.unwrap_or(EndecValueKind::Other)}
                            {:case EndecValueKind::PrimitiveLike}
                                result.@{field.json_key_ident} = Object.fromEntries(value.@{field.field_ident}.entries());
                            {:case EndecValueKind::Date}
                                result.@{field.json_key_ident} = Object.fromEntries(
                                    Array.from(value.@{field.field_ident}.entries()).map(
                                        ([k, v]) => [k, (v as Date).toISOString()]
                                    )
                                );
                            {:case EndecValueKind::NullableDate}
                                result.@{field.json_key_ident} = Object.fromEntries(
                                    Array.from(value.@{field.field_ident}.entries()).map(
                                        ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                    )
                                );
                            {:case _}
                                {#if let Some(value_type) = &field.map_value_encodable_type}
                                    {$let encode_with_context_value: Expr = ts_ident!(nested_encode_fn_name(value_type)).into()}
                                    result.@{field.json_key_ident} = Object.fromEntries(
                                        Array.from(value.@{field.field_ident}.entries()).map(
                                            ([k, v]) => [k, @{encode_with_context_value}(v, ctx)]
                                        )
                                    );
                                {:else}
                                    result.@{field.json_key_ident} = Object.fromEntries(value.@{field.field_ident}.entries());
                                {/if}
                        {/match}
                    }
                {:else}
                    {#match field.map_value_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            result.@{field.json_key_ident} = Object.fromEntries(value.@{field.field_ident}.entries());
                        {:case EndecValueKind::Date}
                            result.@{field.json_key_ident} = Object.fromEntries(
                                Array.from(value.@{field.field_ident}.entries()).map(
                                    ([k, v]) => [k, (v as Date).toISOString()]
                                )
                            );
                        {:case EndecValueKind::NullableDate}
                            result.@{field.json_key_ident} = Object.fromEntries(
                                Array.from(value.@{field.field_ident}.entries()).map(
                                    ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                )
                            );
                        {:case _}
                            {#if let Some(value_type) = &field.map_value_encodable_type}
                                {$let encode_with_context_value: Expr = ts_ident!(nested_encode_fn_name(value_type)).into()}
                                result.@{field.json_key_ident} = Object.fromEntries(
                                    Array.from(value.@{field.field_ident}.entries()).map(
                                        ([k, v]) => [k, @{encode_with_context_value}(v, ctx)]
                                    )
                                );
                            {:else}
                                result.@{field.json_key_ident} = Object.fromEntries(value.@{field.field_ident}.entries());
                            {/if}
                    {/match}
                {/if}

            {:case TypeCategory::Set(_)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#match field.set_elem_kind.unwrap_or(EndecValueKind::Other)}
                            {:case EndecValueKind::PrimitiveLike}
                                result.@{field.json_key_ident} = Array.from(value.@{field.field_ident});
                            {:case EndecValueKind::Date}
                                result.@{field.json_key_ident} = Array.from(value.@{field.field_ident}).map((item: Date) => item.toISOString());
                            {:case EndecValueKind::NullableDate}
                                result.@{field.json_key_ident} = Array.from(value.@{field.field_ident}).map((item: Date | null) => item === null ? null : item.toISOString());
                            {:case _}
                                {#if let Some(elem_type) = &field.set_elem_encodable_type}
                                    {$let encode_with_context_elem: Expr = ts_ident!(nested_encode_fn_name(elem_type)).into()}
                                    result.@{field.json_key_ident} = Array.from(value.@{field.field_ident}).map(
                                        (item) => @{encode_with_context_elem}(item, ctx)
                                    );
                                {:else}
                                    result.@{field.json_key_ident} = Array.from(value.@{field.field_ident});
                                {/if}
                        {/match}
                    }
                {:else}
                    {#match field.set_elem_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            result.@{field.json_key_ident} = Array.from(value.@{field.field_ident});
                        {:case EndecValueKind::Date}
                            result.@{field.json_key_ident} = Array.from(value.@{field.field_ident}).map((item: Date) => item.toISOString());
                        {:case EndecValueKind::NullableDate}
                            result.@{field.json_key_ident} = Array.from(value.@{field.field_ident}).map((item: Date | null) => item === null ? null : item.toISOString());
                        {:case _}
                            {#if let Some(elem_type) = &field.set_elem_encodable_type}
                                {$let encode_with_context_elem: Expr = ts_ident!(nested_encode_fn_name(elem_type)).into()}
                                result.@{field.json_key_ident} = Array.from(value.@{field.field_ident}).map(
                                    (item) => @{encode_with_context_elem}(item, ctx)
                                );
                            {:else}
                                result.@{field.json_key_ident} = Array.from(value.@{field.field_ident});
                            {/if}
                    {/match}
                {/if}

            {:case TypeCategory::Optional(_)}
                if (value.@{field.field_ident} !== undefined) {
                    {#match field.optional_inner_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            result.@{field.json_key_ident} = value.@{field.field_ident};
                        {:case EndecValueKind::Date}
                            result.@{field.json_key_ident} = (value.@{field.field_ident} as Date).toISOString();
                        {:case _}
                            {#if let Some(inner_type) = &field.optional_encodable_type}
                                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                                result.@{field.json_key_ident} = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                            {:else}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {/if}
                    {/match}
                }

            {:case TypeCategory::Nullable(_)}
                {#match field.nullable_inner_kind.unwrap_or(EndecValueKind::Other)}
                    {:case EndecValueKind::PrimitiveLike}
                        result.@{field.json_key_ident} = value.@{field.field_ident};
                    {:case EndecValueKind::Date}
                        result.@{field.json_key_ident} = value.@{field.field_ident} === null
                            ? null
                            : (value.@{field.field_ident} as Date).toISOString();
                    {:case _}
                        if (value.@{field.field_ident} !== null) {
                            {#if let Some(inner_type) = &field.nullable_encodable_type}
                                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                                {$let field_value: Expr = rendered(ts_template! { value.@{field.field_ident} }).into()}
                                {$let encoded = guarded(field.nullable_union_guard.as_ref(), &field_value, &encode_with_context_fn)}
                                result.@{field.json_key_ident} = {$typescript encoded};
                            {:else}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {/if}
                        } else {
                            result.@{field.json_key_ident} = null;
                        }
                {/match}

            {:case TypeCategory::Encodable(type_name)}
                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(type_name)).into()}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {$let field_value: Expr = rendered(ts_template! { value.@{field.field_ident} }).into()}
                        {$let encoded = guarded(field.union_guard.as_ref(), &field_value, &encode_with_context_fn)}
                        result.@{field.json_key_ident} = {$typescript encoded};
                    }
                {:else}
                    {$let field_value: Expr = rendered(ts_template! { value.@{field.field_ident} }).into()}
                    {$let encoded = guarded(field.union_guard.as_ref(), &field_value, &encode_with_context_fn)}
                    result.@{field.json_key_ident} = {$typescript encoded};
                {/if}

            {:case TypeCategory::Record(_, _)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#match field.record_value_kind.unwrap_or(EndecValueKind::Other)}
                            {:case EndecValueKind::PrimitiveLike}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {:case EndecValueKind::Date}
                                result.@{field.json_key_ident} = Object.fromEntries(
                                    Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                );
                            {:case EndecValueKind::NullableDate}
                                result.@{field.json_key_ident} = Object.fromEntries(
                                    Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                );
                            {:case _}
                                {#if let Some(value_type) = &field.record_value_encodable_type}
                                    {$let encode_with_context_value: Expr = ts_ident!(nested_encode_fn_name(value_type)).into()}
                                    result.@{field.json_key_ident} = Object.fromEntries(
                                        Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, @{encode_with_context_value}(v, ctx)])
                                    );
                                {:else}
                                    result.@{field.json_key_ident} = value.@{field.field_ident};
                                {/if}
                        {/match}
                    }
                {:else}
                    {#match field.record_value_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            result.@{field.json_key_ident} = value.@{field.field_ident};
                        {:case EndecValueKind::Date}
                            result.@{field.json_key_ident} = Object.fromEntries(
                                Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                            );
                        {:case EndecValueKind::NullableDate}
                            result.@{field.json_key_ident} = Object.fromEntries(
                                Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                            );
                        {:case _}
                            {#if let Some(value_type) = &field.record_value_encodable_type}
                                {$let encode_with_context_value: Expr = ts_ident!(nested_encode_fn_name(value_type)).into()}
                                result.@{field.json_key_ident} = Object.fromEntries(
                                    Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, @{encode_with_context_value}(v, ctx)])
                                );
                            {:else}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {/if}
                    {/match}
                {/if}

            {:case TypeCategory::Wrapper(_)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#match field.wrapper_inner_kind.unwrap_or(EndecValueKind::Other)}
                            {:case EndecValueKind::PrimitiveLike}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {:case EndecValueKind::Date}
                                result.@{field.json_key_ident} = (value.@{field.field_ident} as Date).toISOString();
                            {:case _}
                                {#if let Some(inner_type) = &field.wrapper_encodable_type}
                                    {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                                    result.@{field.json_key_ident} = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                                {:else}
                                    result.@{field.json_key_ident} = value.@{field.field_ident};
                                {/if}
                        {/match}
                    }
                {:else}
                    {#match field.wrapper_inner_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            result.@{field.json_key_ident} = value.@{field.field_ident};
                        {:case EndecValueKind::Date}
                            result.@{field.json_key_ident} = (value.@{field.field_ident} as Date).toISOString();
                        {:case _}
                            {#if let Some(inner_type) = &field.wrapper_encodable_type}
                                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                                result.@{field.json_key_ident} = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                            {:else}
                                result.@{field.json_key_ident} = value.@{field.field_ident};
                            {/if}
                    {/match}
                {/if}

            {:case TypeCategory::Unknown}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        result.@{field.json_key_ident} = value.@{field.field_ident};
                    }
                {:else}
                    result.@{field.json_key_ident} = value.@{field.field_ident};
                {/if}
        {/match}
        {/if}
    }
}

/// `value` encoded through `encode_fn`, a nested `encodeWithContext`, or
/// through `guard` when it is the union's other member.
fn guarded(guard: Option<&UnionGuard>, value: &Expr, encode_fn: &Expr) -> TsStream {
    match guard {
        Some(guard) => {
            let (is_member, member_value) = (guard.test(value), guard.convert(value));
            ts_template! {
                @{is_member}
                    ? @{member_value}
                    : @{encode_fn}(@{value}, ctx)
            }
        }
        None => ts_template! { @{encode_fn}(@{value}, ctx) },
    }
}
