//! `Decode` for an alias of any other shape: a tuple, a function, an
//! alias of another type.

use super::{AliasDecode, view};
use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::type_alias::TypeBody;
use crate::ts_syn::{MacroforgeError, TsStream, ts_ident};

use crate::builtin::derive::common::function_prefix;

use crate::builtin::derive::endec::type_arguments::{
    declared_param_guards, guarded_param_decoders,
};
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
        ..
    } = alias;
    // Fallback for other type alias forms (simple alias, tuple, etc.)
    let fn_decode_ident = ts_ident!("{}Decode{}", function_prefix(type_name), generic_decl);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        function_prefix(type_name),
        generic_args
    );
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.into();
    let fn_is_ident = ts_ident!("{}Is{}", function_prefix(type_name), generic_decl);
    let fn_has_shape_expr: Expr = ts_ident!("{}HasShape", function_prefix(type_name)).into();
    let full_type_ident = ts_ident!(full_type_name);
    // A value `is` says yes to decodes, its type parameters' values checked
    // by the guards handed in for them.
    let param_names: Vec<String> = type_alias
        .type_params()
        .iter()
        .map(|param| param.name.clone())
        .collect();
    let guards = declared_param_guards(&param_names);
    let guarded = guarded_param_decoders(&param_names);
    let fn_decode_ident_expr: Expr = ts_ident!("{}Decode", function_prefix(type_name)).into();

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

    let mut result = ts_template! {
        /** Decodes a value to this type; JSON text decodes through `fromString`. @param input - The value to decode @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}@{&alias.coders}): @{return_type_ident} {
            try {
                const ctx = @{decode_context_expr}.create();
                const result = @{fn_decode_internal_expr}(input, ctx@{&alias.passed_coders});
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

        export function @{fn_is_ident}(value: unknown@{&guards}): value is @{full_type_ident} {
            return @{fn_has_shape_expr}(value) && @{fn_decode_ident_expr}(value@{&guarded}).success;
        }
    };
    // The object, union and intersection bodies have decoders of their own.
    let view = match type_alias.body().unbranded() {
        TypeBody::Tuple(elements) => view::tuple_view(alias, elements)?,
        TypeBody::Alias(ts_type) | TypeBody::Other(ts_type) => view::whole_view(alias, ts_type)?,
        TypeBody::Object { .. }
        | TypeBody::Union(_)
        | TypeBody::Intersection(_)
        | TypeBody::Newtype(_) => {
            return Err(MacroforgeError::new(
                type_alias.inner.span,
                format!("@derive(Decode): type '{type_name}' has a body no decoder takes apart"),
            ));
        }
    };
    result = result.merge(view);
    let from_string_name = format!("{}FromString{}", function_prefix(type_name), generic_decl);
    let decode_call = format!("{}Decode{}", function_prefix(type_name), generic_args);
    result = result.merge(super::super::from_string::from_string_function(
        &super::super::from_string::TextDecoder {
            name: &from_string_name,
            decode: &decode_call,
            coders: &alias.coders,
            passed_coders: &alias.passed_coders,
            return_type: &return_type,
            takes_options: true,
        },
    ));
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    Ok(result)
}
