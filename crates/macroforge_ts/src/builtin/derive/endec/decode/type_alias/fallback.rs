//! `Decode` for an alias of any other shape: a tuple, a function, an
//! alias of another type.

use super::{AliasDecode, view};
use crate::ast::Expr;
use crate::builtin::derive::endec::decode::reference::decode_reference;
use crate::macros::ts_template;
use crate::ts_syn::{MacroforgeError, TsStream, ts_ident};

use convert_case::{Case, Casing};

use crate::builtin::derive::endec::decode::helpers::type_accepts_string;
use crate::builtin::derive::endec::get_foreign_types;
use crate::builtin::return_types::{decode_return_type, wrap_error, wrap_success};

pub(super) fn handle_fallback_type_alias(alias: &AliasDecode) -> Result<TsStream, MacroforgeError> {
    let AliasDecode {
        type_alias,
        type_name,
        decode_context_expr,
        decode_error_expr,
        decode_options_ident,
        generic_decl,
        generic_args,
        full_type_name,
        type_registry,
        caller_file_path,
        file_imports,
        ..
    } = alias;
    // Fallback for other type alias forms (simple alias, tuple, etc.)
    let fn_decode_ident = ts_ident!("{}Decode{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        type_name.to_case(Case::Camel),
        generic_args
    );
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.into();
    let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_expr: Expr = ts_ident!("{}HasShape", type_name.to_case(Case::Camel)).into();
    let full_type_ident = ts_ident!(full_type_name);

    // Compute return type and wrappers
    let return_type = decode_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("result");
    let success_result_expr =
        Expr::parse(&success_result).expect("decode success wrapper should parse");
    let error_from_catch = wrap_error("e.errors");
    let error_from_catch_expr =
        Expr::parse(&error_from_catch).expect("decode catch error wrapper should parse");
    let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
    let error_generic_message_expr =
        Expr::parse(&error_generic_message).expect("decode generic error wrapper should parse");
    let error_from_ctx = wrap_error("__errors");
    let error_from_ctx_expr =
        Expr::parse(&error_from_ctx).expect("decode ctx error wrapper should parse");

    // Use the type registry and foreign types to determine if this type accepts strings.
    let foreign_types_config = get_foreign_types();
    let accepts_string = type_accepts_string(
        type_name,
        type_registry,
        caller_file_path,
        file_imports,
        &foreign_types_config,
    );
    let data_init_expr = if accepts_string {
        Expr::parse("input").expect("data init expr should parse")
    } else {
        Expr::parse(r#"typeof input === "string" ? JSON.parse(input) : input"#)
            .expect("data init expr should parse")
    };

    let mut result = ts_template! {
        /** Decodes input to this type. @param input - Value to decode @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}): @{return_type_ident} {
            try {
                const data = @{data_init_expr};

                const ctx = @{decode_context_expr}.create();
                const result = @{fn_decode_internal_expr}(data, ctx);
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

        export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
            return @{fn_has_shape_expr}(value);
        }
    };
    let view = if let Some(elements) = type_alias.body().as_tuple() {
        view::tuple_view(alias, elements)?
    } else if let Some(ts_type) = type_alias.body().as_alias() {
        view::whole_view(alias, ts_type)?
    } else {
        passthrough(alias)
    };
    result = result.merge(view);
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    Ok(result)
}

/// The view functions for a value with no fields to decode, such as a
/// function: it is kept as it is.
fn passthrough(alias: &AliasDecode) -> TsStream {
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        alias.type_name.to_case(Case::Camel),
        alias.generic_decl
    );
    let fn_has_shape_ident = ts_ident!("{}HasShape", alias.type_name.to_case(Case::Camel));
    let decode_context_ident = &alias.decode_context_ident;
    let full_type_ident = ts_ident!(alias.full_type_name.as_str());
    let type_ident = &alias.type_ident;
    let reference = decode_reference(alias.type_name, &alias.full_type_name);
    ts_template! {
        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}): @{full_type_ident.clone()} {
            {$typescript reference}
            return value as @{type_ident};
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
            return value != null;
        }
    }
}
