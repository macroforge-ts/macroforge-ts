//! `Decode` for an alias of a primitive, branded or not.

use super::AliasDecode;
use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::{MacroforgeError, TsStream, ts_ident};

use convert_case::{Case, Casing};

use crate::builtin::derive::endec::decode::validation::{Missing, generate_field_validations};
use crate::builtin::derive::endec::validators::ValidatorSpec;
use crate::builtin::derive::endec::{TypeCategory, get_foreign_types};
use crate::builtin::return_types::{decode_return_type, wrap_error, wrap_success};

/// Decode for an alias of a primitive, optionally symbol-branded
/// (`type Meters = number & { readonly [B]: true }`). The base primitive is
/// checked and alias-level validators run before the value is branded. A base
/// with a wire form in the foreign-type table (`bigint` travels as a string)
/// is converted from it exactly as a field of that type would be.
pub(super) fn handle_primitive_type_alias(
    alias: &AliasDecode,
    primitive: &str,
    validators: &[ValidatorSpec],
) -> Result<TsStream, MacroforgeError> {
    let AliasDecode {
        type_name,
        decode_context_ident,
        decode_context_expr,
        decode_error_expr,
        decode_options_ident,
        generic_decl,
        generic_args,
        full_type_name,
        ..
    } = alias;
    let camel = type_name.to_case(Case::Camel);
    let fn_decode_ident = ts_ident!("{}Decode{}", camel, generic_decl);
    let fn_decode_expr: Expr = ts_ident!("{}Decode", camel).into();
    let fn_decode_internal_ident = ts_ident!("{}DecodeWithContext{}", camel, generic_decl);
    let fn_decode_internal_expr: Expr =
        ts_ident!("{}DecodeWithContext{}", camel, generic_args).into();
    let fn_is_ident = ts_ident!("{}Is{}", camel, generic_decl);
    let fn_has_shape_ident = ts_ident!("{}HasShape", camel);
    let full_type_ident = ts_ident!(full_type_name);

    let return_type_ident = ts_ident!(decode_return_type(full_type_name).as_str());
    let success_result_expr =
        Expr::parse(&wrap_success("result")).expect("decode success wrapper should parse");
    let error_from_catch_expr =
        Expr::parse(&wrap_error("e.errors")).expect("decode catch error wrapper should parse");
    let error_generic_message_expr = Expr::parse(&wrap_error(r#"[{ field: "_root", message }]"#))
        .expect("decode generic error wrapper should parse");
    let error_from_ctx_expr =
        Expr::parse(&wrap_error("__errors")).expect("decode ctx error wrapper should parse");
    let wire_decode: Option<Expr> =
        TypeCategory::match_foreign_type(primitive, &get_foreign_types())
            .config
            .and_then(|foreign| foreign.handler_callee(ForeignHandler::Decode))
            .map(|callee| Expr::parse(&callee).expect("foreign decode expression should parse"));
    // A string input is the value itself, not JSON: always for `string`, and for
    // a wire-converted base, where parsing would round `bigint` digits.
    let data_init_expr = if primitive == "string" || wire_decode.is_some() {
        Expr::parse("input").expect("data init expr should parse")
    } else {
        Expr::parse(r#"typeof input === "string" ? JSON.parse(input) : input"#)
            .expect("data init expr should parse")
    };

    let has_validators = !validators.is_empty();
    let validation_code = generate_field_validations(
        validators,
        "__decoded",
        "_root",
        type_name,
        Missing::Excluded,
    );
    // Only validators read the context; an unread parameter fails `noUnusedParameters`.
    let ctx_param = ts_ident!(if has_validators { "ctx" } else { "_ctx" });

    let mut result = ts_template! {
        /** Decodes input to this type. @param input - Value to decode @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}): @{return_type_ident} {
            try {
                const data = @{data_init_expr};

                const ctx = @{decode_context_expr}.create();
                const result = @{fn_decode_internal_expr}(data, ctx);
                if (opts?.freeze) {
                    ctx.freezeAll();
                }

                const __errors = ctx.getErrors();
                if (__errors.length > 0) {
                    return @{error_from_ctx_expr};
                }

                return @{success_result_expr};
            } catch (e) {
                if (e instanceof @{decode_error_expr}) {
                    return @{error_from_catch_expr};
                }
                const message = e instanceof Error ? e.message : String(e);
                return @{error_generic_message_expr};
            }
        }

        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, @{ctx_param}: @{decode_context_ident}): @{full_type_ident} {
            {#if let Some(wire_decode) = &wire_decode}
                let __decoded: unknown = value;
                if (typeof value !== "@{primitive}") {
                    try {
                        __decoded = @{wire_decode}(value);
                    } catch (__error) {
                        throw new @{decode_error_expr}([{ field: "_root", message: "@{type_name}.decodeWithContext: expected @{primitive}, " + String(__error) }]);
                    }
                }
            {:else}
                const __decoded: unknown = value;
            {/if}
            if (typeof __decoded !== "@{primitive}") {
                throw new @{decode_error_expr}([{ field: "_root", message: "@{type_name}.decodeWithContext: expected @{primitive}" }]);
            }
            {#if has_validators}
                const errors: Array<{ field: string; message: string }> = [];
                {$typescript validation_code}
                ctx.pushErrors(errors);
            {/if}
            return __decoded as @{full_type_ident};
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
            {#if wire_decode.is_some()}
                return typeof value === "@{primitive}" || typeof value === "string";
            {:else}
                return typeof value === "@{primitive}";
            {/if}
        }

        export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
            {#if has_validators}
                return typeof value === "@{primitive}" && @{fn_decode_expr}(value).success;
            {:else}
                return typeof value === "@{primitive}";
            {/if}
        }
    };
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    Ok(result)
}
