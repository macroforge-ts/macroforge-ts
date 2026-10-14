//! `hasShape` and `is` for a union alias.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::derive::common::function_prefix;
use crate::builtin::derive::common::js_string;
use crate::builtin::derive::endec::decode::helpers::decoded_keyword_test;
use crate::builtin::derive::endec::decode::helpers::{extract_base_type, nested_has_shape_fn_name};
use crate::builtin::derive::endec::type_arguments::{
    declared_param_guards, guarded_param_decoders,
};

/// True when a value has the shape of some member.
pub(super) fn has_shape(u: &Union) -> TsStream {
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
        is_literal_only,
        is_primitive_only,
        is_encodable_only,
        has_literals,
        literals,
        primitive_types,
        regular_encodables,
        foreign_encodables,
        object_variants,
        untagged_object_variants,
        intersection_variants,
        external_object_variants,
        type_name,
        tag_field,
        content_field,
        primitive_check_condition,
        encodable_type_check_condition,
        ..
    } = u;
    let fn_has_shape_ident = ts_ident!("{}HasShape", function_prefix(type_name));
    // A type parameter may be any value, so a union with one among members
    // of several kinds has nothing to ask of the value.
    let takes_any =
        *has_generic_params && !*is_literal_only && !*is_primitive_only && !*is_encodable_only;
    let value_param = ts_ident!(if takes_any { "_value" } else { "value" });
    ts_template! {
        export function @{fn_has_shape_ident}(@{value_param}: unknown): boolean {
                        {#if *is_literal_only}
                            const allowedValues = [@{literals.join(", ")}] as const;
                            return allowedValues.some((allowed) => allowed === value);
                        {:else if *is_primitive_only}
                            return @{primitive_check_condition};
                        {:else if *is_encodable_only}
                            // Foreign types with hasShape may not be objects: check first
                            {#for type_ref in foreign_encodables}
                                {#if let Some(ref shape_callee) = type_ref.foreign_has_shape_callee}
                                    {$let foreign_shape_expr: Expr = Expr::parse(shape_callee).expect("foreign hasShape expr should parse")}
                                    if (@{foreign_shape_expr}(value)) return true;
                                {/if}
                            {/for}

                            {#if *is_externally_tagged}
                                // Externally tagged: check if object has a key matching a variant name
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        {#for ov in external_object_variants}
                                        if (__variantName === "@{ov.name}") return true;
                                    {/for}
                                    if (@{encodable_type_check_condition.replace("__typeName", "__variantName")}) return true;
                                    }
                                }
                                if (typeof value === "string") {
                                    if (@{encodable_type_check_condition.replace("__typeName", "value")}) return true;
                                }
                                return false;
                            {:else if *is_adjacently_tagged}
                                // Adjacently tagged: check for tag and content fields
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}");
                                    if (typeof __typeName === "string" && "@{content_field}" in value) {
                                        return @{encodable_type_check_condition};
                                    }
                                }
                                return false;
                            {:else if *is_untagged}
                                // Untagged: shape matching only
                                {#for type_ref in regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) return true;
                                {/for}
                                return false;
                            {:else}
                                // Internally tagged (default): tag-based + shape matching
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}");
                                    if (typeof __typeName === "string") {
                                        return @{encodable_type_check_condition};
                                    }
                                }

                                // Shape matching works for any value type (including non-objects)
                                {#for type_ref in regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) return true;
                                {/for}
                                return false;
                            {/if}
                        {:else if takes_any}
                            return true;
                        {:else}
                            {#if *has_literals}
                                const allowedLiterals = [@{literals.join(", ")}] as const;
                                if (allowedLiterals.some((allowed) => allowed === value)) return true;
                            {/if}
                            {#if *has_primitives}
                                {#for prim in primitive_types}
                                    {$let is_primitive: Expr = decoded_keyword_test(prim, "value").into()}
                                    if (@{is_primitive}) return true;
                                {/for}
                            {/if}
                            {#if *has_dates}
                                if (value instanceof Date) return true;
                            {/if}
                            {#if *has_encodables}
                                {#if *is_externally_tagged}
                                    // Externally tagged: check if object has a key matching a variant name
                                    if (typeof value === "object" && value !== null) {
                                        const __keys = Object.keys(value);
                                        const __variantName = __keys[0];
                                        if (__variantName !== undefined) {
                                            {#for ov in external_object_variants}
                                        if (__variantName === "@{ov.name}") return true;
                                    {/for}
                                    if (@{encodable_type_check_condition.replace("__typeName", "__variantName")}) return true;
                                        }
                                    }
                                    if (typeof value === "string") {
                                        if (@{encodable_type_check_condition.replace("__typeName", "value")}) return true;
                                    }
                                {:else if *is_adjacently_tagged}
                                    // Adjacently tagged: check for tag and content fields
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}");
                                        if (typeof __typeName === "string" && "@{content_field}" in value) {
                                            if (@{encodable_type_check_condition}) return true;
                                        }
                                    }
                                {:else if *is_untagged}
                                    // Untagged: shape matching only
                                    {#for type_ref in regular_encodables}
                                        {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (@{has_shape_fn}(value)) return true;
                                    {/for}
                                {:else if regular_encodables.is_empty()}
                                    // Internally tagged (default), with no member told by its shape
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}");
                                        if (typeof __typeName === "string" && (@{encodable_type_check_condition})) return true;
                                    }
                                {:else}
                                    // Internally tagged (default)
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}");
                                        if (typeof __typeName === "string") {
                                            if (@{encodable_type_check_condition}) return true;
                                        } else {
                                            let __matchCount = 0;
                                            {#for type_ref in regular_encodables}
                                                {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (@{has_shape_fn}(value)) __matchCount++;
                                            {/for}
                                            if (__matchCount === 1) return true;
                                        }
                                    } else {
                                        // Non-object values: regular encodables may still match (e.g. RecordLink can be a string)
                                        let __matchCount = 0;
                                        {#for type_ref in regular_encodables}
                                            {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (@{has_shape_fn}(value)) __matchCount++;
                                        {/for}
                                        if (__matchCount === 1) return true;
                                    }
                                {/if}
                                // Foreign types with hasShape may not be objects
                                {#for type_ref in foreign_encodables}
                                    {#if let Some(ref shape_callee) = type_ref.foreign_has_shape_callee}
                                        {$let foreign_shape_expr: Expr = Expr::parse(shape_callee).expect("foreign hasShape expr should parse")}
                                        if (@{foreign_shape_expr}(value)) return true;
                                    {/if}
                                {/for}
                            {/if}
                            {#if *has_object_variants || *has_intersection_variants}
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = @{crate::builtin::derive::endec::coder::endec_call("member")}(value, "@{tag_field}");
                                    {$let all_tag_values: Vec<String> = object_variants.iter().map(|ov| js_string(&ov.tag_value)).chain(intersection_variants.iter().map(|iv| js_string(&iv.tag_value))).collect()}
                                    if ([@{all_tag_values.join(", ")}].some((tag) => tag === __typeName)) return true;
                                }
                            {/if}
                            {#if !external_object_variants.is_empty() && *is_externally_tagged && !*has_encodables}
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        {$let all_variant_names: Vec<String> = external_object_variants.iter().map(|ov| js_string(&ov.name)).collect()}
                                        if ([@{all_variant_names.join(", ")}].includes(__variantName)) return true;
                                    }
                                }
                            {/if}
                            {#if !untagged_object_variants.is_empty()}
                                // A test a member, each in a function of its own.
                                const __shapes: ReadonlyArray<(value: unknown) => boolean> = [
                                {#for uv in untagged_object_variants}
                                    (value) => @{uv.shape_condition},
                                {/for}
                                ];
                                if (__shapes.some((shape) => shape(value))) return true;
                            {/if}
                            return false;
        {/if}
        }
    }
}

/// Narrows to the union: the shape check, and a decode as well when an arm
/// carries validators or the union has type parameters, whose values the
/// guards handed in for them check.
pub(super) fn is_guard(u: &Union) -> TsStream {
    let Union {
        has_arm_validators,
        type_name,
        generic_decl,
        full_type_ident,
        param_names,
        ..
    } = u;
    let camel = function_prefix(type_name);
    let fn_is_ident = ts_ident!("{}Is{}", camel, generic_decl);
    let fn_has_shape_expr: Expr = ts_ident!("{}HasShape", camel).into();
    let fn_decode_expr: Expr = ts_ident!("{}Decode", camel).into();
    let guards = declared_param_guards(param_names);
    let guarded = guarded_param_decoders(param_names);
    let decodes = *has_arm_validators || !param_names.is_empty();
    ts_template! {
        export function @{fn_is_ident}(value: unknown@{&guards}): value is @{full_type_ident} {
            {#if decodes}
                return @{fn_has_shape_expr}(value) && @{fn_decode_expr}(value@{&guarded}).success;
            {:else}
                return @{fn_has_shape_expr}(value);
            {/if}
        }
    }
}
