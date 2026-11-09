//! `decode` and `decodeWithContext` for a union alias that is not literals only.

use crate::ast::Expr;
use crate::builtin::derive::common::rendered;
use crate::builtin::derive::endec::decode::helpers::decoded_keyword_test;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::derive::common::function_prefix;
use crate::builtin::derive::endec::decode::validation::{Missing, generate_field_validations};

use super::encodable::encodable_dispatch;
use super::mixed::mixed_dispatch;
use crate::builtin::derive::endec::decode::reference::decode_reference;
use crate::builtin::return_types::{PENDING_REF, decode_return_type, wrap_error, wrap_success};

/// `{camelName}Decode`: decodes a value through the context-taking decoder,
/// and `{camelName}FromString`, which decodes JSON text.
pub(super) fn decode_entry(u: &Union) -> TsStream {
    let Union {
        type_name,
        decode_context_expr,
        decode_error_expr,
        pending_ref_expr,
        decode_options_ident,
        generic_decl,
        generic_args,
        coders,
        passed_coders,
        full_type_name,
        ..
    } = u;
    let fn_decode_ident = ts_ident!("{}Decode{}", function_prefix(type_name), generic_decl);
    let fn_decode_internal_expr: Expr = ts_ident!(
        "{}DecodeWithContext{}",
        function_prefix(type_name),
        generic_args
    )
    .into();

    // Compute return type and wrappers
    let return_type = decode_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("resultOrRef");
    let success_result_expr =
        Expr::parse(&success_result).expect("decode success wrapper should parse");
    let error_root_ref = wrap_error(&format!(
        r#"[{{ field: "_root", message: "{}.decode: root cannot be a forward reference" }}]"#,
        type_name
    ));
    let error_root_ref_expr =
        Expr::parse(&error_root_ref).expect("decode root error wrapper should parse");
    let error_from_catch = wrap_error("e.errors");
    let error_from_catch_expr =
        Expr::parse(&error_from_catch).expect("decode catch error wrapper should parse");
    let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
    let error_generic_message_expr =
        Expr::parse(&error_generic_message).expect("decode generic error wrapper should parse");
    let error_from_ctx = wrap_error("__errors");
    let error_from_ctx_expr =
        Expr::parse(&error_from_ctx).expect("decode ctx error wrapper should parse");

    let decode = ts_template! {
        /** Decodes a value to this type; JSON text decodes through `fromString`. @param input - The value to decode @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}@{coders}): @{return_type_ident} {
            try {
                const ctx = @{decode_context_expr}.create();
                const resultOrRef = @{fn_decode_internal_expr}(input, ctx@{passed_coders});

                if (@{pending_ref_expr}.is(resultOrRef)) {
                    return @{error_root_ref_expr};
                }

                ctx.applyPatches();
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
    };
    let from_string_name = format!("{}FromString{}", function_prefix(type_name), generic_decl);
    let decode_call = format!("{}Decode{}", function_prefix(type_name), generic_args);
    decode.merge(super::super::super::from_string::from_string_function(
        &super::super::super::from_string::TextDecoder {
            name: &from_string_name,
            decode: &decode_call,
            coders,
            passed_coders,
            return_type: &return_type,
            takes_options: true,
        },
    ))
}

/// `{camelName}DecodeWithContext`: resolves references, then decodes by the
/// union's kind of members.
pub(super) fn decode_with_context(u: &Union) -> TsStream {
    let Union {
        is_primitive_only,
        is_encodable_only,
        type_name,
        decode_context_ident,
        pending_ref_ident,
        generic_decl,
        coders,
        full_type_ident,
        full_type_name,
        ..
    } = u;
    let returns = rendered(ts_template! { @{full_type_name} | @{PENDING_REF} });
    let reference = decode_reference(type_name, &returns);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        function_prefix(type_name),
        generic_decl
    );
    let body = if *is_primitive_only {
        primitive_only(u)
    } else if *is_encodable_only {
        encodable_dispatch(u)
    } else {
        mixed_dispatch(u)
    };
    ts_template! {
        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}@{coders}): @{full_type_ident} | @{pending_ref_ident} {
            {$typescript reference}

            {$typescript body}
        }
    }
}

/// A union of primitives: the value's `typeof` picks the arm, whose
/// validators then run.
fn primitive_only(u: &Union) -> TsStream {
    let Union {
        primitive_arms,
        type_name,
        decode_error_expr,
        expected_types_str,
        full_type_ident,
        ..
    } = u;
    ts_template! {
                            {#for (prim, arm_validators) in primitive_arms}
                                {$let is_primitive: Expr = decoded_keyword_test(prim, "value").into()}
                                if (@{is_primitive}) {
                                    {#if !arm_validators.is_empty()}
                                        const errors: Array<{ field: string; message: string }> = [];
                                        {$let arm_validation = generate_field_validations(arm_validators, "value", "_root", type_name, Missing::Excluded)}
                                        {$typescript arm_validation}
                                        ctx.pushErrors(errors);
                                    {/if}
                                    return value as @{full_type_ident};
                                }
                            {/for}

                            throw new @{decode_error_expr}([{
                                field: "_root",
                                message: "@{type_name}.decodeWithContext: expected @{expected_types_str}, got " + typeof value
                            }]);
    }
}
