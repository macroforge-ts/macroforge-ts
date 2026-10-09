//! `Encode` for an alias of any other shape: a tuple, a function, an alias
//! of another type.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{MacroforgeError, TsStream};

use super::{AliasEncode, view};

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
        encode_context_expr,
        ..
    } = alias;
    let fn_encode_internal_expr: Expr = fn_encode_internal_ident.clone().into();
    let mut result = if let Some(params) = &type_params_ident {
        let (body, reads_context) = encode_body(alias)?;
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}<@{params}>(value: @{full_type_ident}, keepMetadata?: boolean): string {
                const ctx = @{encode_context_expr}.create();
                const __raw = @{fn_encode_internal_expr}(value, ctx);
                if (keepMetadata) return JSON.stringify(__raw);
                return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            {#if reads_context}
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown {
                {$typescript body}
            }
            {:else}
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown;
            export function @{fn_encode_internal_ident}<@{params}>(value: @{full_type_ident}): unknown {
                {$typescript body}
            }
            {/if}
        }
    } else {
        let (body, reads_context) = encode_body(alias)?;
        ts_template! {
            /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
            export function @{fn_encode_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
                const ctx = @{encode_context_expr}.create();
                const __raw = @{fn_encode_internal_expr}(value, ctx);
                if (keepMetadata) return JSON.stringify(__raw);
                return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
            }

            /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context */
            {#if reads_context}
            export function @{fn_encode_internal_ident}(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown {
                {$typescript body}
            }
            {:else}
            export function @{fn_encode_internal_ident}(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown;
            export function @{fn_encode_internal_ident}(value: @{full_type_ident}): unknown {
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
    } else {
        ts_template! { return value; }
    };
    // The field encoders read the context only as a call's last argument.
    let reads_context = body.source().contains("ctx)");
    Ok((body, reads_context))
}
