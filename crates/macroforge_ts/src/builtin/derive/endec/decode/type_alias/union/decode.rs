//! `decode` and `decodeWithContext` for a union alias that is not literals only.

use crate::ast::Expr;
use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::members::Union;
use crate::builtin::derive::endec::decode::validation::{Missing, generate_field_validations};
use convert_case::{Case, Casing};

use super::encodable::encodable_dispatch;
use super::mixed::mixed_dispatch;
use crate::builtin::derive::endec::decode::helpers::type_accepts_string;
use crate::builtin::derive::endec::decode::reference::decode_reference;
use crate::builtin::return_types::{PENDING_REF, decode_return_type, wrap_error, wrap_success};

/// `{camelName}Decode`: parses JSON text unless a string is itself a member
/// value, then decodes through the context-taking decoder.
pub(super) fn decode_entry(u: &Union) -> TsStream {
    let Union {
        has_literals,
        primitive_types,
        encodable_types,
        type_name,
        decode_context_expr,
        decode_error_expr,
        pending_ref_expr,
        decode_options_ident,
        generic_decl,
        generic_args,
        full_type_name,
        foreign_types_config,
        type_registry,
        caller_file_path,
        file_imports,
        ..
    } = u;
    let fn_decode_ident = ts_ident!("{}Decode{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_decode_internal_expr: Expr = ts_ident!(
        "{}DecodeWithContext{}",
        type_name.to_case(Case::Camel),
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

    // A string input is the value itself when a member accepts strings, and
    // JSON text otherwise. A foreign member's hasShape decides that at runtime,
    // since its handler is imported rather than visible here.
    let has_string_variant = primitive_types.iter().any(|p| p == "string")
        || *has_literals
        || type_accepts_string(
            type_name,
            type_registry,
            caller_file_path,
            file_imports,
            foreign_types_config,
        );
    let foreign_shape_checks: Vec<String> = encodable_types
        .iter()
        .filter(|member| member.is_foreign)
        .filter_map(|member| member.foreign_has_shape_callee.as_deref())
        .map(|callee| rendered(ts_template! { @{callee}(input) }))
        .collect();
    let data_init = if has_string_variant {
        "input".to_string()
    } else if foreign_shape_checks.is_empty() {
        r#"typeof input === "string" ? JSON.parse(input) : input"#.to_string()
    } else {
        let shaped = foreign_shape_checks.join(" || ");
        rendered(
            ts_template! { typeof input === "string" && !(@{shaped}) ? JSON.parse(input) : input },
        )
    };
    let data_init_expr = Expr::parse(&data_init).expect("data init expr should parse");
    ts_template! {
        /** Decodes input to this type. @param input - Value to decode @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}): @{return_type_ident} {
            try {
                const data = @{data_init_expr};

                const ctx = @{decode_context_expr}.create();
                const resultOrRef = @{fn_decode_internal_expr}(data, ctx);

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
    }
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
        full_type_ident,
        full_type_name,
        ..
    } = u;
    let returns = rendered(ts_template! { @{full_type_name} | @{PENDING_REF} });
    let reference = decode_reference(type_name, &returns);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        type_name.to_case(Case::Camel),
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
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
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

                            throw new @{decode_error_expr}([{
                                field: "_root",
                                message: "@{type_name}.decodeWithContext: expected @{expected_types_str}, got " + typeof value
                            }]);
    }
}
