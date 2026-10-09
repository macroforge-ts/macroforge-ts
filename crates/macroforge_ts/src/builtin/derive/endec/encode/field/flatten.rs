//! The statements that encode an `@endec(flatten)` field: its encoded object
//! is merged into the enclosing one, without its own tag and `__id`.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::derive::endec::TypeCategory;
use crate::builtin::derive::endec::encode::types::{EncodeField, nested_encode_fn_name};
use crate::builtin::derive::endec::value_kind::EndecValueKind;

/// Merges `field`'s encoded value into `result`. `tag_field` is the tag the
/// nested encoder writes, which the enclosing object does not take over.
pub(super) fn encode_flattened_field(field: &EncodeField, tag_field: &str) -> TsStream {
    ts_template! {
        {#match &field.type_cat}
            {:case TypeCategory::Encodable(type_name)}
                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(type_name)).into()}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        const __flattened = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                        // Remove tag field and __id from flattened object
                        const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                        Object.assign(result, rest);
                    }
                {:else}
                    {
                        const __flattened = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                        // Remove tag field and __id from flattened object
                        const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                        Object.assign(result, rest);
                    }
                {/if}
            {:case TypeCategory::Record(_, _)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#match field.record_value_kind.unwrap_or(EndecValueKind::Other)}
                            {:case EndecValueKind::PrimitiveLike}
                                Object.assign(result, value.@{field.field_ident});
                            {:case EndecValueKind::Date}
                                Object.assign(result, Object.fromEntries(
                                    Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                ));
                            {:case EndecValueKind::NullableDate}
                                Object.assign(result, Object.fromEntries(
                                    Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                ));
                            {:case _}
                                {#if let Some(value_type) = &field.record_value_encodable_type}
                                    {$let encode_with_context_value: Expr = ts_ident!(nested_encode_fn_name(value_type)).into()}
                                    Object.assign(result, Object.fromEntries(
                                        Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, @{encode_with_context_value}(v, ctx)])
                                    ));
                                {:else}
                                    Object.assign(result, value.@{field.field_ident});
                                {/if}
                        {/match}
                    }
                {:else}
                    {#match field.record_value_kind.unwrap_or(EndecValueKind::Other)}
                        {:case EndecValueKind::PrimitiveLike}
                            Object.assign(result, value.@{field.field_ident});
                        {:case EndecValueKind::Date}
                            Object.assign(result, Object.fromEntries(
                                Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                            ));
                        {:case EndecValueKind::NullableDate}
                            Object.assign(result, Object.fromEntries(
                                Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                            ));
                        {:case _}
                            {#if let Some(value_type) = &field.record_value_encodable_type}
                                {$let encode_with_context_value: Expr = ts_ident!(nested_encode_fn_name(value_type)).into()}
                                Object.assign(result, Object.fromEntries(
                                    Object.entries(value.@{field.field_ident}).map(([k, v]) => [k, @{encode_with_context_value}(v, ctx)])
                                ));
                            {:else}
                                Object.assign(result, value.@{field.field_ident});
                            {/if}
                    {/match}
                {/if}
            {:case TypeCategory::Optional(_)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        {#if let Some(inner_type) = &field.optional_encodable_type}
                            {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                            const __flattened = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                            Object.assign(result, rest);
                        {:else}
                            const __flattened = value.@{field.field_ident};
                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                            Object.assign(result, rest);
                        {/if}
                    }
                {:else}
                    {
                        {#if let Some(inner_type) = &field.optional_encodable_type}
                            {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                            const __flattened = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                            Object.assign(result, rest);
                        {:else}
                            const __flattened = value.@{field.field_ident};
                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                            Object.assign(result, rest);
                        {/if}
                    }
                {/if}
            {:case TypeCategory::Nullable(_)}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        if (value.@{field.field_ident} !== null) {
                            {#if let Some(inner_type) = &field.nullable_encodable_type}
                                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                                const __flattened = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                Object.assign(result, rest);
                            {:else}
                                const __flattened = value.@{field.field_ident};
                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                Object.assign(result, rest);
                            {/if}
                        }
                    }
                {:else}
                    {
                        if (value.@{field.field_ident} !== null) {
                            {#if let Some(inner_type) = &field.nullable_encodable_type}
                                {$let encode_with_context_fn: Expr = ts_ident!(nested_encode_fn_name(inner_type)).into()}
                                const __flattened = @{encode_with_context_fn}(value.@{field.field_ident}, ctx);
                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                Object.assign(result, rest);
                            {:else}
                                const __flattened = value.@{field.field_ident};
                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                Object.assign(result, rest);
                            {/if}
                        }
                    }
                {/if}
            {:case _}
                {#if field.optional}
                    if (value.@{field.field_ident} !== undefined) {
                        const __flattened = value.@{field.field_ident};
                        // Remove tag field and __id from flattened object
                        const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                        Object.assign(result, rest);
                    }
                {:else}
                    {
                        const __flattened = value.@{field.field_ident};
                        // Remove tag field and __id from flattened object
                        const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                        Object.assign(result, rest);
                    }
                {/if}
        {/match}
    }
}
