//! The body of `decodeWithContext` for a union whose members all have
//! decoders: dispatch by tag, by key or by shape, per the tagging mode.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::derive::endec::decode::helpers::{
    extract_base_type, nested_decode_fn_name, nested_has_shape_fn_name,
};

/// Dispatches to the member decoders. Every path either returns a member
/// or throws.
pub(super) fn encodable_dispatch(u: &Union) -> TsStream {
    let Union {
        is_externally_tagged,
        is_adjacently_tagged,
        is_untagged,
        regular_encodables,
        foreign_encodables,
        object_variants,
        intersection_variants,
        external_object_variants,
        type_name,
        decode_error_expr,
        tag_field,
        content_field,
        expected_types_str,
        full_type_ident,
        ..
    } = u;
    ts_template! {
                            // Foreign types may not be objects: check hasShape first
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
                                        throw new @{decode_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.decodeWithContext: unknown variant \"" + __variantName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }
                                // String value may be a unit variant name
                                if (typeof value === "string") {
                                    {#for type_ref in regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (value === "@{type_ref.full_type}") {
                                            return @{decode_with_context_fn}({}, ctx) as @{full_type_ident};
                                        }
                                    {/for}
                                }
                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: expected externally tagged object with variant key. Expected one of: @{expected_types_str}"
                                }]);
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
                                        throw new @{decode_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.decodeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }
                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: expected adjacently tagged object with \"@{tag_field}\" and \"@{content_field}\" fields"
                                }]);
                            {:else if *is_untagged}
                                // Untagged: shape matching only, no tag field
                                const __shapeMatches: Array<string> = [];
                                {#for type_ref in regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                {/for}
                                {#for type_ref in foreign_encodables}
                                    {#if let Some(ref shape_callee) = type_ref.foreign_has_shape_callee}
                                        {$let foreign_shape_expr: Expr = Expr::parse(shape_callee).expect("foreign hasShape expr should parse")}
                                        if (@{foreign_shape_expr}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                    {/if}
                                {/for}

                                if (__shapeMatches.length >= 1) {
                                    {#for type_ref in regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                            try {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in foreign_encodables}
                                        {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                            {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                            if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                                try {
                                                    return @{foreign_deser_expr}(value) as @{full_type_ident};
                                                } catch { /* try next variant */ }
                                            }
                                        {/if}
                                    {/for}
                                }

                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: value does not match any variant shape. Expected one of: @{expected_types_str}"
                                }]);
                            {:else}
                                // Internally tagged (default): tag-based discrimination with shape matching fallback
                                // Tag-based discrimination (only for object values)
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
                                        {#for ov in object_variants}
                                            if (__typeName === "@{ov.tag_value}") {
                                                // Inline object variant: decode fields in place
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
                                                // Intersection variant: decode the type ref and merge with tag
                                                {#if iv.is_encodable}
                                                    {$let iv_deser_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&iv.type_ref))).into()}
                                                    const __inner = @{iv_deser_fn}(value, ctx);
                                                    return { ...(__inner as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {:else}
                                                    return { ...(value as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {/if}
                                            }
                                        {/for}

                                        throw new @{decode_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.decodeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }

                                // Infer variant via structural shape matching (works for any value type)
                                const __shapeMatches: Array<string> = [];
                                {#for type_ref in regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                {/for}
                                {#for type_ref in foreign_encodables}
                                    {#if let Some(ref shape_callee) = type_ref.foreign_has_shape_callee}
                                        {$let foreign_shape_expr: Expr = Expr::parse(shape_callee).expect("foreign hasShape expr should parse")}
                                        if (@{foreign_shape_expr}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                    {/if}
                                {/for}

                                if (__shapeMatches.length === 1) {
                                    {#for type_ref in regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                            return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                        }
                                    {/for}
                                    {#for type_ref in foreign_encodables}
                                        {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                            {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                            if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                return @{foreign_deser_expr}(value) as @{full_type_ident};
                                            }
                                        {/if}
                                    {/for}
                                }

                                if (__shapeMatches.length > 1) {
                                    // Multiple variants match: try each decoder in order, return first success
                                    {#for type_ref in regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                            try {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in foreign_encodables}
                                        {#if let Some(ref decode_callee) = type_ref.foreign_decode_callee}
                                            {$let foreign_deser_expr: Expr = Expr::parse(decode_callee).expect("foreign decode expr should parse")}
                                            if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                                try {
                                                    return @{foreign_deser_expr}(value) as @{full_type_ident};
                                                } catch { /* try next variant */ }
                                            }
                                        {/if}
                                    {/for}

                                    throw new @{decode_error_expr}([{
                                        field: "_root",
                                        message: "@{type_name}.decodeWithContext: missing @{tag_field} field and value matches multiple variants: " + __shapeMatches.join(", ") + ". Add a @{tag_field} field to disambiguate."
                                    }]);
                                }

                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: missing @{tag_field} field and value does not match any variant shape. Expected one of: @{expected_types_str}"
                                }]);
                            {/if}
    }
}
