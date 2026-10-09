//! The standalone `Decode` functions of an object-shaped interface or type
//! alias: `decode`, `decodeWithContext`, `validateField`, `validateFields`,
//! `hasShape` and `is`.

use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use super::super::EndecContainerOptions;
use super::field::statements::decode_fields;
use super::reference::decode_reference;
use super::types::DecodeField;
use super::validation::generate_field_validations;
use crate::builtin::derive::common::{TypeNames, js_string, rendered};
use crate::builtin::return_types::{
    DECODE_CONTEXT, DECODE_ERROR, DECODE_OPTIONS, PENDING_REF, decode_return_type,
    root_forward_reference_error, wrap_error, wrap_success,
};

/// Every standalone `Decode` function of an object-shaped type.
pub(super) fn object_decode_functions(
    names: &TypeNames,
    container_opts: &EndecContainerOptions,
    fields: &[DecodeField],
) -> TsStream {
    let mut result = decode_entry(names)
        .merge(decode_with_context(names, container_opts, fields))
        .merge(field_validators(names, fields))
        .merge(shape_guards(names, fields));
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);
    result
}

/// `{camelName}Decode`: parses JSON text, decodes through
/// `{camelName}DecodeWithContext` and resolves forward references.
fn decode_entry(names: &TypeNames) -> TsStream {
    let fn_decode_ident = names.generic_function("Decode");
    let fn_decode_internal_expr: Expr = names.generic_call("DecodeWithContext").into();
    let decode_options_ident = ts_ident!(DECODE_OPTIONS);
    let decode_context_expr: Expr = ts_ident!(DECODE_CONTEXT).into();
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    let pending_ref_expr: Expr = ts_ident!(PENDING_REF).into();
    let return_type_ident = ts_ident!(decode_return_type(&names.full_type).as_str());
    let success_result_expr =
        Expr::parse(&wrap_success("resultOrRef")).expect("decode success wrapper should parse");
    let error_root_ref_expr = Expr::parse(&root_forward_reference_error(&names.type_name))
        .expect("decode root error wrapper should parse");
    let error_from_catch_expr =
        Expr::parse(&wrap_error("e.errors")).expect("decode catch error wrapper should parse");
    let error_generic_message_expr = Expr::parse(&wrap_error(r#"[{ field: "_root", message }]"#))
        .expect("decode generic error wrapper should parse");
    let error_from_ctx_expr =
        Expr::parse(&wrap_error("__errors")).expect("decode ctx error wrapper should parse");
    ts_template! {
        /** Decodes input to this type. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to decode @param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}): @{return_type_ident} {
            try {
                // Auto-detect: if string, parse as JSON first
                const data = typeof input === "string" ? JSON.parse(input) : input;

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

/// `{camelName}DecodeWithContext`: checks the object's keys, then decodes
/// each field onto a fresh object registered for references.
fn decode_with_context(
    names: &TypeNames,
    container_opts: &EndecContainerOptions,
    fields: &[DecodeField],
) -> TsStream {
    let fn_decode_internal_ident = names.generic_function("DecodeWithContext");
    let decode_context_ident = ts_ident!(DECODE_CONTEXT);
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    let pending_ref_ident = ts_ident!(PENDING_REF);
    let full_type_ident = names.full_type_ident();
    let type_name = names.type_name.as_str();
    let tag_field = container_opts.tag_field_or_default();
    let known_keys: Vec<String> = fields
        .iter()
        .filter(|field| !field.flatten)
        .map(|field| js_string(&field.json_key))
        .collect();
    let required_fields: Vec<&DecodeField> = fields
        .iter()
        .filter(|field| !field.optional && !field.flatten)
        .collect();
    let field_statements = decode_fields(fields, type_name);
    let returns = rendered(ts_template! { @{&names.full_type} | @{PENDING_REF} });
    let reference = decode_reference(type_name, &returns);
    ts_template! {
        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
            {$typescript reference}

            if (typeof value !== "object" || value === null || Array.isArray(value)) {
                throw new @{decode_error_expr}([{ field: "_root", message: "@{type_name}.decodeWithContext: expected an object" }]);
            }

            const obj = value as Record<string, unknown>;
            const errors: Array<{ field: string; message: string }> = [];

            {#if container_opts.deny_unknown_fields}
                const knownKeys = new Set(["@{tag_field}", "__id", "__ref", @{known_keys.join(", ")}]);
                for (const key of Object.keys(obj)) {
                    if (!knownKeys.has(key)) {
                        errors.push({ field: key, message: "unknown field" });
                    }
                }
            {/if}

            {#for field in &required_fields}
                if (!("@{field.json_key}" in obj)) {
                    errors.push({ field: "@{field.json_key}", message: "missing required field" });
                }
            {/for}

            const instance: any = {};

            if (obj.__id !== undefined) {
                ctx.register(obj.__id as number, instance);
            }

            ctx.trackForFreeze(instance);

            {$typescript field_statements}

            ctx.pushErrors(errors);

            return instance as @{full_type_ident};
        }
    }
}

/// `{camelName}ValidateField` and `{camelName}ValidateFields`: a field's
/// validators run on its value alone, for form-style checks.
fn field_validators(names: &TypeNames, fields: &[DecodeField]) -> TsStream {
    let full_type_ident = names.full_type_ident();
    let key_param = names.key_param();
    let key = key_param.name;
    let fn_validate_field_ident: Ident = ts_ident!(format!(
        "{}{}",
        names.function("ValidateField"),
        key_param.decl
    ));
    let fn_validate_fields_ident = names.generic_function("ValidateFields");
    let type_name = names.type_name.as_str();
    let validated: Vec<&DecodeField> = fields
        .iter()
        .filter(|field| !field.flatten && field.has_validators())
        .collect();
    let fn_validate_field_impl = ts_ident!(names.function("ValidateField"));
    let fn_validate_fields_impl = ts_ident!(names.function("ValidateFields"));
    ts_template! {
        {#if validated.is_empty()}
            export function @{fn_validate_field_ident}(field: @{key.clone()}, value: unknown): Array<{ field: string; message: string }>;
            export function @{fn_validate_field_impl}(): Array<{ field: string; message: string }> {
                return [];
            }

            export function @{fn_validate_fields_ident}(partial: { readonly [@{key.clone()} in keyof @{full_type_ident.clone()}]?: unknown }): Array<{ field: string; message: string }>;
            export function @{fn_validate_fields_impl}(): Array<{ field: string; message: string }> {
                return [];
            }
        {:else}
            export function @{fn_validate_field_ident}(field: @{key.clone()}, value: unknown): Array<{ field: string; message: string }> {
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &validated}
                    if (field === "@{field.field_name}") {
                        const __val = value as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name, field.missing())}
                        {$typescript validation_code}
                    }
                {/for}
                return errors;
            }

            export function @{fn_validate_fields_ident}(partial: { readonly [@{key.clone()} in keyof @{full_type_ident.clone()}]?: unknown }): Array<{ field: string; message: string }> {
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &validated}
                    if ("@{field.field_name}" in partial && partial.@{field.field_ident} !== undefined) {
                        const __val = partial.@{field.field_ident} as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name, field.missing())}
                        {$typescript validation_code}
                    }
                {/for}
                return errors;
            }
        {/if}
    }
}

/// `{camelName}HasShape`, true when every required key is present, and
/// `{camelName}Is`, which also requires the value to decode, so its
/// validators hold for whatever it narrows.
fn shape_guards(names: &TypeNames, fields: &[DecodeField]) -> TsStream {
    // The shape check reads no type parameter, so it declares none.
    let fn_has_shape_ident = ts_ident!(names.function("HasShape"));
    let fn_has_shape_expr: Expr = ts_ident!(names.function("HasShape")).into();
    let fn_is_ident = names.generic_function("Is");
    let fn_decode_expr: Expr = ts_ident!(names.function("Decode")).into();
    let full_type_ident = names.full_type_ident();
    let required_keys: Vec<String> = fields
        .iter()
        .filter(|field| !field.optional && !field.flatten)
        .map(|field| {
            let key = js_string(&field.json_key);
            rendered(ts_template! { @{key} in o })
        })
        .collect();
    let shape_check_condition = required_keys.join(" && ");
    ts_template! {
        export function @{fn_has_shape_ident}(obj: unknown): boolean {
            if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                return false;
            }
            {#if required_keys.is_empty()}
                return true;
            {:else}
                const o = obj as Record<string, unknown>;
                return @{shape_check_condition};
            {/if}
        }

        export function @{fn_is_ident}(obj: unknown): obj is @{full_type_ident} {
            return @{fn_has_shape_expr}(obj) && @{fn_decode_expr}(obj).success;
        }
    }
}
