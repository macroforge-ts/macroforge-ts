//! `Encode` for an alias of any other shape: a tuple, a function, an alias
//! of another type.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_alias::TypeBody;
use crate::ts_syn::{MacroforgeError, TsStream, ts_ident};

use super::super::text::encode_text_body;
use super::{AliasEncode, view};
use crate::builtin::derive::common::{mentions, parameter_name};

/// Encodes a tuple element by element and an alias of another type as that
/// type, each by its declared type; any other value, such as a function, is
/// kept as it is.
pub(super) fn encode_fallback_alias(alias: &AliasEncode) -> Result<TsStream, MacroforgeError> {
    let AliasEncode {
        type_params_ident,
        full_type_ident,
        fn_encode_ident,
        fn_encode_internal_ident,
        encode_context_ident,
        coders,
        passed_coders,
        ..
    } = alias;
    let fn_encode_internal_expr: Expr = fn_encode_internal_ident.clone().into();
    let encode_text = encode_text_body(&fn_encode_internal_expr, "__type", passed_coders);
    let mut result = if let Some(params) = &type_params_ident {
        let (body, reads_context) = encode_body(alias)?;
        let value_param = ts_ident!(parameter_name("value", body.source()));
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}<@{params}>(value: @{full_type_ident}, keepMetadata?: boolean@{coders}): string {
                {$typescript encode_text}
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            {#if reads_context}
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{encode_context_ident}@{coders}): unknown {
                {$typescript body}
            }
            {:else}
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{encode_context_ident}@{coders}): unknown;
            export function @{fn_encode_internal_ident}<@{params}>(@{&value_param}: @{full_type_ident}): unknown {
                {$typescript body}
            }
            {/if}
        }
    } else {
        let (body, reads_context) = encode_body(alias)?;
        let value_param = ts_ident!(parameter_name("value", body.source()));
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
                {$typescript encode_text}
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            {#if reads_context}
            export function @{fn_encode_internal_ident}(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown {
                {$typescript body}
            }
            {:else}
            export function @{fn_encode_internal_ident}(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown;
            export function @{fn_encode_internal_ident}(@{&value_param}: @{full_type_ident}): unknown {
                {$typescript body}
            }
            {/if}
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    Ok(result)
}

/// The encoder's statements, and whether they read the encoding context: an
/// encoder that never does takes it only in its published signature.
fn encode_body(alias: &AliasEncode) -> Result<(TsStream, bool), MacroforgeError> {
    let body = if let Some(elements) = alias.type_alias.body().as_tuple() {
        view::tuple_body(alias, elements)?
    } else if let Some(ts_type) = alias.type_alias.body().as_alias() {
        view::whole_body(alias, ts_type)?
    } else if let TypeBody::Other(ts_type) = alias.type_alias.body() {
        view::whole_body(alias, ts_type)?
    } else {
        ts_template! { return value; }
    };
    let reads_context = mentions(body.source(), "ctx");
    Ok((body, reads_context))
}
