//! `Encode` for an alias of a primitive, branded or not.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::TypeParamIR;
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::{MacroforgeError, TsStream, ts_ident};

use convert_case::{Case, Casing};

use super::AliasEncode;
use crate::builtin::derive::endec::{TypeCategory, get_foreign_types};

/// A primitive travels as itself, or in the wire form the foreign-type table
/// gives its base (`bigint` as a string).
pub(super) fn encode_primitive_alias(
    alias: &AliasEncode,
    primitive: &str,
) -> Result<TsStream, MacroforgeError> {
    let AliasEncode {
        type_name,
        type_params,
        full_type_ident,
        encode_context_ident,
        ..
    } = alias;
    // A primitive, branded or not, travels as itself, or in the wire
    // form the foreign-type table gives its base (`bigint` as a string).
    let wire_encode: Option<Expr> =
        TypeCategory::match_foreign_type(primitive, &get_foreign_types())
            .config
            .and_then(|foreign| foreign.handler_callee(ForeignHandler::Encode))
            .map(|callee| Expr::parse(&callee).expect("foreign encode expression should parse"));
    let generic_decl = TypeParamIR::declare_all(type_params);
    let fn_encode_decl = ts_ident!("{}Encode{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_encode_internal_decl = ts_ident!(
        "{}EncodeWithContext{}",
        type_name.to_case(Case::Camel),
        generic_decl
    );
    let mut result = ts_template! {
        /** Encodes a value to a JSON string. @param value - The value to encode @param keepMetadata - Accepted for signature parity; primitives carry no metadata @returns JSON string representation */
        export function @{fn_encode_decl}(value: @{full_type_ident}, keepMetadata?: boolean): string;
        export function @{fn_encode_decl}(value: @{full_type_ident}): string {
            {#if let Some(wire_encode) = &wire_encode}
                return JSON.stringify(@{wire_encode}(value));
            {:else}
                return JSON.stringify(value);
            {/if}
        }

        /** Encodes with an existing context for nested/cyclic object graphs. @param value - The value to encode @param ctx - The encoding context, unused for primitives */
        export function @{fn_encode_internal_decl}(value: @{full_type_ident}, ctx: @{encode_context_ident}): unknown;
        export function @{fn_encode_internal_decl}(value: @{full_type_ident}): unknown {
            {#if let Some(wire_encode) = &wire_encode}
                return @{wire_encode}(value);
            {:else}
                return value;
            {/if}
        }
    };
    result.add_aliased_import("EncodeContext", crate::package::ENDEC);
    Ok(result)
}
