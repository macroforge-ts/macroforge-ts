//! The body of `decodeWithContext` for a union mixing literals, primitives,
//! dates, member types and inline variants: each kind is tried in turn.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::derive::endec::decode::helpers::{
    extract_base_type, nested_decode_fn_name, nested_has_shape_fn_name,
};
use crate::builtin::derive::endec::decode::validation::{Missing, generate_field_validations};

/// Tries literals, primitives, dates, member types, inline variants and
/// shapes in order, then throws.
pub(super) fn mixed_dispatch(u: &Union) -> TsStream {
    let Union {
        is_externally_tagged,
        is_adjacently_tagged,
        is_untagged,
        has_primitives,
        has_encodables,
        has_dates,
        has_generic_params,
        has_object_variants,
        has_intersection_variants,
        has_literals,
        literals,
        primitive_arms,
        regular_encodables,
        foreign_encodables,
        object_variants,
        untagged_object_variants,
        intersection_variants,
        external_object_variants,
        type_name,
        decode_error_expr,
        tag_field,
        content_field,
        full_type_ident,
        ..
    } = u;
    ts_template! {
                            {#if *has_literals}
                                const allowedLiterals = [@{literals.join(", ")}] as const;
                                if (allowedLiterals.includes(value as any)) {
                                    return value as @{full_type_ident};
                                }
                            {/if}

                            {#if *has_primitives}
                                {#for (prim, arm_validators) in primitive_arms}
                                    if (typeof value === "@{prim}") {
                                        {#if !arm_validators.is_empty()}
                                            const errors: Array<{ field: string; message: string }> = [];
                                            {$let arm_validation = generate_field_validations(arm_validators, "value", "_root", type_name, Missing::Excluded)}
                                            {$typescript arm_validation}
                                            ctx.pushErrors(errors);
                                        {/if}
                                        return value as @{full_type_ident};
                                    }
                                {/for}
                            {/if}

                            {#if *has_dates}
                                if (value instanceof Date) {
                                    return value as @{full_type_ident};
                                }
                                if (typeof value === "string") {
                                    const __dateVal = new Date(value);
                                    if (!isNaN(__dateVal.getTime())) {
                                        return __dateVal as unknown as @{full_type_ident};
                                    }
                                }
                            {/if}

                            {#if *has_encodables}
                                {#if *is_externally_tagged}
                                    // Externally tagged: { "TypeName": payload }
                                    if (typeof value === "object" && value !== null) {
                                        const __keys = Object.keys(value);
                                        const __variantName = __keys[0];
                                        if (__variantName !== undefined) {
                                            const __inner = (value as any)[__variantName];
                                            {#for ov in external_object_variants}
                                            if (__variantName === "@{ov.name}") {
                                                {#if let Some(ref decode_callee) = ov.inner_foreign_decode_callee}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("inner foreign decode expr should parse")}
                                                    return ({ "@{ov.name}": @{foreign_deser_expr}(__inner) }) as @{full_type_ident};
                                                {:else}
                                                    {#if let Some(ref payload_deser_fn) = ov.payload_decode_fn}
                                                        return ({ "@{ov.name}": @{payload_deser_fn}(__inner ?? {}, ctx) }) as @{full_type_ident};
                                                    {:else}
                                                        return ({ "@{ov.name}": __inner }) as @{full_type_ident};
                                                    {/if}
                                                {/if}
                                            }
                                        {/for}
                                        {#for type_ref in regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__variantName === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(__inner != null && typeof __inner === "object" ? __inner : {}, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in foreign_encodables}
                                                {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                                    if (__variantName === "@{type_ref.full_type}") {
                                                        return @{foreign_deser_expr}(__inner) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        }
                                    }
                                    if (typeof value === "string") {
                                        {#for type_ref in regular_encodables}
                                            {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (value === "@{type_ref.full_type}") {
                                                return @{decode_with_context_fn}({}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                    }
                                {:else if *is_adjacently_tagged}
                                    // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            const __content = (value as any)["@{content_field}"];
                                            {#for type_ref in regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(__content != null && typeof __content === "object" ? __content : {}, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in foreign_encodables}
                                                {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        return @{foreign_deser_expr}(__content) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        }
                                    }
                                {:else if *is_untagged}
                                    // Untagged: shape matching only
                                    {#for type_ref in regular_encodables}
                                        {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (@{has_shape_fn}(value)) {
                                            try {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in foreign_encodables}
                                        {#if let Some(ref shape_callee) = type_ref.foreign_has_shape_callee}
                                            {$let foreign_shape_expr: Expr = Expr::parse(shape_callee).expect("foreign hasShape expr should parse")}
                                            {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                                {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                                if (@{foreign_shape_expr}(value)) {
                                                    try {
                                                        return @{foreign_deser_expr}(value) as @{full_type_ident};
                                                    } catch { /* try next variant */ }
                                                }
                                            {/if}
                                        {/if}
                                    {/for}
                                {:else}
                                    // Internally tagged (default): tag-based + shape matching fallback
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            {#for type_ref in regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in foreign_encodables}
                                                {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        return @{foreign_deser_expr}(value) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        } else {
                                            // No tag field: infer variant via structural shape matching
                                            const __shapeMatches: Array<string> = [];
                                            {#for type_ref in regular_encodables}
                                                {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                            {/for}
                                            if (__shapeMatches.length === 1) {
                                                {#for type_ref in regular_encodables}
                                                    {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                    if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                        return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                                    }
                                                {/for}
                                            }
                                        }
                                    } else {
                                        // Non-object values: regular encodables may still match (e.g. RecordLink can be a string)
                                        const __shapeMatches: Array<string> = [];
                                        {#for type_ref in regular_encodables}
                                            {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                        {/for}
                                        if (__shapeMatches.length === 1) {
                                            {#for type_ref in regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                        }
                                    }
                                {/if}
                            {/if}

                            // Foreign types may not be objects: check hasShape outside the object block
                            {#for type_ref in foreign_encodables}
                                {#if let Some(ref shape_callee) = type_ref.foreign_has_shape_callee}
                                    {$let foreign_shape_expr: Expr = Expr::parse(shape_callee).expect("foreign hasShape expr should parse")}
                                    {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                        {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                        if (@{foreign_shape_expr}(value)) {
                                            return @{foreign_deser_expr}(value) as @{full_type_ident};
                                        }
                                    {/if}
                                {/if}
                            {/for}

                            {#if *has_generic_params}
                                return value as @{full_type_ident};
                            {/if}

                            {#if *has_object_variants || *has_intersection_variants}
                                // Tagged variants with tag-based discrimination
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        {#for ov in object_variants}
                                            if (__typeName === "@{ov.tag_value}") {
                                                const __result: Record<string, unknown> = { "@{tag_field}": "@{ov.tag_value}" };
                                                {#for field in &ov.fields}
                                                    {#if field.name != *tag_field}
                                                        __result["@{field.name}"] = (value as any)["@{field.name}"] ?? null;
                                                    {/if}
                                                {/for}
                                                return __result as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for iv in intersection_variants}
                                            if (__typeName === "@{iv.tag_value}") {
                                                {#if iv.is_encodable}
                                                    {$let iv_deser_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&iv.type_ref))).into()}
                                                    const __inner = @{iv_deser_fn}(value, ctx);
                                                    return { ...(__inner as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {:else}
                                                    return { ...(value as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {/if}
                                            }
                                        {/for}
                                    }
                                }
                            {/if}

                            {#if !external_object_variants.is_empty() && *is_externally_tagged && !*has_encodables}
                                // Externally tagged anonymous-variant union: { "TagName": payload }
                                // (Distinct from the has_encodables case above so types like
                                //  RecurrenceEnd / OrderStage that mix literals + anonymous
                                //  variants still get a decoder body.)
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        const __inner = (value as any)[__variantName];
                                        {#for ov in external_object_variants}
                                            if (__variantName === "@{ov.name}") {
                                                {#if let Some(ref decode_callee) = ov.inner_foreign_decode_callee}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("inner foreign decode expr should parse")}
                                                    return ({ "@{ov.name}": @{foreign_deser_expr}(__inner) }) as @{full_type_ident};
                                                {:else}
                                                    {#if let Some(ref payload_deser_fn) = ov.payload_decode_fn}
                                                        return ({ "@{ov.name}": @{payload_deser_fn}(__inner ?? {}, ctx) }) as @{full_type_ident};
                                                    {:else}
                                                        return ({ "@{ov.name}": __inner }) as @{full_type_ident};
                                                    {/if}
                                                {/if}
                                            }
                                        {/for}
                                    }
                                }
                            {/if}

                            {#for uv in untagged_object_variants}
                                if (@{uv.shape_condition}) {
                                    const __result: Record<string, unknown> = {};
                                    {#for name in &uv.required_fields}
                                        __result["@{name}"] = (value as any)["@{name}"];
                                    {/for}
                                    {#for name in &uv.optional_fields}
                                        if ("@{name}" in value) __result["@{name}"] = (value as any)["@{name}"];
                                    {/for}
                                    return __result as @{full_type_ident};
                                }
                            {/for}

                            throw new @{decode_error_expr}([{
                                field: "_root",
                                message: "@{type_name}.decodeWithContext: value does not match any union member"
                            }]);
    }
}
