use crate::macros::ts_template;
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_expr, ts_ident,
};

use convert_case::{Case, Casing};

use super::super::{
    SerdeContainerOptions, TaggingMode, TypeCategory, get_foreign_types,
    rewrite_expression_namespaces,
};
use super::field_processing::interface_field_to_deserialize_field;
use super::helpers::{
    extract_base_type, nested_deserialize_fn_name, nested_deserialize_result_fn_name,
    nested_has_shape_fn_name, type_accepts_string,
};
use super::types::{DeserializeField, SerdeValueKind, SerializableTypeRef};
use super::validation::generate_field_validations;
use crate::builtin::return_types::{
    DESERIALIZE_CONTEXT, DESERIALIZE_ERROR, DESERIALIZE_OPTIONS, PENDING_REF,
    deserialize_return_type, wrap_error, wrap_success,
};

pub(super) fn handle_type_alias(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    let type_alias = match &input.data {
        crate::ts_syn::Data::TypeAlias(ta) => ta,
        _ => unreachable!(),
    };
    let type_registry = input.context.type_registry.as_ref();

    let type_name = input.name();
    let type_ident = ts_ident!(type_name);
    let deserialize_context_ident = ts_ident!(DESERIALIZE_CONTEXT);
    let deserialize_context_expr: Expr = deserialize_context_ident.clone().into();
    let deserialize_error_expr: Expr = ts_ident!(DESERIALIZE_ERROR).into();
    let pending_ref_ident = ts_ident!(PENDING_REF);
    let pending_ref_expr: Expr = pending_ref_ident.clone().into();
    let deserialize_options_ident = ts_ident!(DESERIALIZE_OPTIONS);

    // Build generic type signature if type has type params
    let type_params = type_alias.type_params();
    let (generic_decl, generic_args) = if type_params.is_empty() {
        (String::new(), String::new())
    } else {
        let params = type_params.join(", ");
        (format!("<{}>", params), format!("<{}>", params))
    };
    let full_type_name = format!("{}{}", type_name, generic_args);
    let _full_type_ident = ts_ident!(full_type_name.as_str());

    // Create combined generic declarations for validateField that include K
    let validate_field_generic_decl = if type_params.is_empty() {
        format!("<K extends keyof {}>", type_name)
    } else {
        let params = type_params.join(", ");
        format!("<{}, K extends keyof {}>", params, full_type_name)
    };

    if type_alias.is_object() {
        handle_object_type_alias(
            input,
            type_alias,
            type_name,
            &type_ident,
            &deserialize_context_ident,
            &deserialize_context_expr,
            &deserialize_error_expr,
            &pending_ref_ident,
            &pending_ref_expr,
            &deserialize_options_ident,
            &generic_decl,
            &generic_args,
            &full_type_name,
            &validate_field_generic_decl,
        )
    } else if let Some(members) = type_alias.as_union() {
        handle_union_type_alias(
            input,
            type_alias,
            type_name,
            &type_ident,
            &deserialize_context_ident,
            &deserialize_context_expr,
            &deserialize_error_expr,
            &pending_ref_ident,
            &pending_ref_expr,
            &deserialize_options_ident,
            &generic_decl,
            &generic_args,
            &full_type_name,
            type_registry,
            members,
        )
    } else {
        handle_fallback_type_alias(
            input,
            type_alias,
            type_name,
            &type_ident,
            &deserialize_context_ident,
            &deserialize_context_expr,
            &deserialize_error_expr,
            &pending_ref_ident,
            &pending_ref_expr,
            &deserialize_options_ident,
            &generic_decl,
            &generic_args,
            &full_type_name,
            &validate_field_generic_decl,
            type_registry,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_object_type_alias(
    _input: &DeriveInput,
    type_alias: &crate::ts_syn::DataTypeAlias,
    type_name: &str,
    type_ident: &crate::swc_ecma_ast::Ident,
    deserialize_context_ident: &crate::swc_ecma_ast::Ident,
    deserialize_context_expr: &Expr,
    deserialize_error_expr: &Expr,
    pending_ref_ident: &crate::swc_ecma_ast::Ident,
    pending_ref_expr: &Expr,
    deserialize_options_ident: &crate::swc_ecma_ast::Ident,
    generic_decl: &str,
    _generic_args: &str,
    full_type_name: &str,
    validate_field_generic_decl: &str,
) -> Result<TsStream, MacroforgeError> {
    let container_opts = SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Collect deserializable fields with diagnostic collection
    let mut all_diagnostics = DiagnosticCollector::new();
    let fields: Vec<DeserializeField> = type_alias
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|field| {
            interface_field_to_deserialize_field(field, &container_opts, &mut all_diagnostics)
        })
        .collect();

    // Check for errors in field parsing before continuing
    if all_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
    }

    let all_fields: Vec<DeserializeField> = fields.iter().filter(|f| !f.flatten).cloned().collect();
    let required_fields: Vec<DeserializeField> = fields
        .iter()
        .filter(|f| !f.optional && !f.flatten)
        .cloned()
        .collect();

    let known_keys: Vec<String> = fields
        .iter()
        .filter(|f| !f.flatten)
        .map(|f| f.json_key.clone())
        .collect();

    // Fields with validators for per-field validation
    let fields_with_validators: Vec<_> = all_fields
        .iter()
        .filter(|f| f.has_validators())
        .cloned()
        .collect();

    let shape_check_condition: String = if required_fields.is_empty() {
        "true".to_string()
    } else {
        required_fields
            .iter()
            .map(|f| format!("\"{}\" in o", f.json_key))
            .collect::<Vec<_>>()
            .join(" && ")
    };

    let fn_deserialize_ident = ts_ident!(format!(
        "{}Deserialize{}",
        type_name.to_case(Case::Camel),
        generic_decl
    ));
    let fn_deserialize_internal_ident = ts_ident!(format!(
        "{}DeserializeWithContext",
        type_name.to_case(Case::Camel)
    ));
    let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
    let fn_validate_field_ident = ts_ident!(format!(
        "{}ValidateField{}",
        type_name.to_case(Case::Camel),
        validate_field_generic_decl
    ));
    let fn_validate_fields_ident =
        ts_ident!(format!("{}ValidateFields", type_name.to_case(Case::Camel)));
    let fn_is_ident = ts_ident!(format!(
        "{}Is{}",
        type_name.to_case(Case::Camel),
        generic_decl
    ));
    let fn_has_shape_ident = ts_ident!(format!(
        "{}HasShape{}",
        type_name.to_case(Case::Camel),
        generic_decl
    ));
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();

    // Compute return type and wrappers
    let full_type_ident = ts_ident!(full_type_name);
    let return_type = deserialize_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("resultOrRef");
    let success_result_expr =
        parse_ts_expr(&success_result).expect("deserialize success wrapper should parse");
    let error_root_ref = wrap_error(&format!(
        r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
        type_name
    ));
    let error_root_ref_expr =
        parse_ts_expr(&error_root_ref).expect("deserialize root error wrapper should parse");
    let error_from_catch = wrap_error("e.errors");
    let error_from_catch_expr =
        parse_ts_expr(&error_from_catch).expect("deserialize catch error wrapper should parse");
    let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
    let error_generic_message_expr = parse_ts_expr(&error_generic_message)
        .expect("deserialize generic error wrapper should parse");
    let error_from_ctx = wrap_error("__errors");
    let error_from_ctx_expr =
        parse_ts_expr(&error_from_ctx).expect("deserialize ctx error wrapper should parse");

    // Build known keys array string
    let known_keys_list: Vec<_> = known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

    // Flag for whether any required fields exist
    let has_required = !required_fields.is_empty();

    let mut result = {
        ts_template! {
            /** Deserializes input to this type. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
            export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                try {
                    // Auto-detect: if string, parse as JSON first
                    const data = typeof input === "string" ? JSON.parse(input) : input;

                    const ctx = @{deserialize_context_expr}.create();
                    const resultOrRef = @{fn_deserialize_internal_expr}(data, ctx);

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
                    if (e instanceof @{deserialize_error_expr}) {
                        return @{error_from_catch_expr};
                    }
                    const message = e instanceof Error ? e.message : String(e);
                    return @{error_generic_message_expr};
                }
            }

            /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
            export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{type_ident} | @{pending_ref_ident} {
                if (value?.__ref !== undefined) {
                    return ctx.getOrDefer(value.__ref) as @{type_ident} | @{pending_ref_ident};
                }

                if (typeof value !== "object" || value === null || Array.isArray(value)) {
                    throw new @{deserialize_error_expr}([{ field: "_root", message: "@{type_name}.deserializeWithContext: expected an object" }]);
                }

                const obj = value as Record<string, unknown>;
                const errors: Array<{ field: string; message: string }> = [];

                {#if container_opts.deny_unknown_fields}
                    const knownKeys = new Set(["@{tag_field}", "__id", "__ref", @{known_keys_list.join(", ")}]);
                    for (const key of Object.keys(obj)) {
                        if (!knownKeys.has(key)) {
                            errors.push({ field: key, message: "unknown field" });
                        }
                    }
                {/if}

                {#if !required_fields.is_empty()}
                    {#for field in &required_fields}
                        if (!("@{field.json_key}" in obj)) {
                            errors.push({ field: "@{field.json_key}", message: "missing required field" });
                        }
                    {/for}
                {/if}

                const instance: any = {};

                if (obj.__id !== undefined) {
                    ctx.register(obj.__id as number, instance);
                }

                ctx.trackForFreeze(instance);

                {#if !all_fields.is_empty()}
                    {#for field in all_fields}
                        {$let raw_var_name = format!("__raw_{}", field.field_name)}
                        {$let raw_var_ident: Ident = ts_ident!(raw_var_name)}
                        {$let has_validators = field.has_validators()}
                        {#if let Some(fn_expr) = &field.deserialize_with}
                            // Custom deserialization function (deserializeWith)
                            {#if field.optional}
                                if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                    {#if has_validators}
                                        {
                                            const __convertedVal = (@{fn_expr})(obj["@{field.json_key}"]);
                                            {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, type_name)}
                                            {$typescript validation_code}
                                            instance.@{field.field_ident} = __convertedVal;
                                        }
                                    {:else}
                                        instance.@{field.field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                    {/if}
                                }
                            {:else}
                                {#if has_validators}
                                    {
                                        const __convertedVal = (@{fn_expr})(obj["@{field.json_key}"]);
                                        {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, type_name)}
                                        {$typescript validation_code}
                                        instance.@{field.field_ident} = __convertedVal;
                                    }
                                {:else}
                                    instance.@{field.field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                {/if}
                            {/if}
                        {:else}
                        {#if field.optional}
                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.raw_cast_type};
                                {#match &field.type_cat}
                                    {:case TypeCategory::Primitive}
                                        {#if has_validators}
                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                            {$typescript validation_code}

                                        {/if}
                                        {#if field.decimal_format}
                                            {
                                                const __numVal = globalThis.Number(@{raw_var_ident});
                                                if (globalThis.Number.isNaN(__numVal)) {
                                                    errors.push({ field: "@{field.json_key}", message: "expected a numeric string, got " + JSON.stringify(@{raw_var_ident}) });
                                                } else {
                                                    instance.@{field.field_ident} = __numVal;
                                                }
                                            }
                                        {:else}
                                            instance.@{field.field_ident} = @{raw_var_ident};
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {
                                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                {$typescript validation_code}

                                            {/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                                {$typescript validation_code}

                                            {/if}

                                            {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                                                {:case SerdeValueKind::Date}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => typeof item === "string" ? new Date(item) : item as Date
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_deserialize_result_fn_name(elem_type)).into()}
                                                        const __arr = @{raw_var_ident}.map((item: @{inner} | { __ref: number }, idx) => {
                                                            if (typeof item === "object" && item !== null && "__ref" in item) {
                                                                const result = ctx.getOrDefer(item.__ref);
                                                                if (@{pending_ref_expr}.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            const __elemResult = @{elem_deser_result_fn}(item);
                                                            return __elemResult.success ? __elemResult.value : item;
                                                        });
                                                        instance.@{field.field_ident} = __arr;
                                                        __arr.forEach((item, idx) => {
                                                            if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                                ctx.addPatch(instance.@{field.field_ident}, idx, (item as any).__refId);
                                                            }
                                                        });
                                                    {:else}
                                                        instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Map(key_type, value_type)}
                                        if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                            {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                {:case SerdeValueKind::Date}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, typeof v === "string" ? new Date(v) : v as Date])
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type_name) = &field.map_value_serializable_type}
                                                        {$let value_deser_result_fn: Expr = ts_ident!(nested_deserialize_result_fn_name(value_type_name)).into()}
                                                        instance.@{field.field_ident} = new Map(
                                                            Object.entries(@{raw_var_ident}).map(([k, v]) => {
                                                                const __vResult = @{value_deser_result_fn}(v);
                                                                return [k as @{key_type}, __vResult.success ? __vResult.value : v as @{value_type}];
                                                            })
                                                        );
                                                    {:else}
                                                        instance.@{field.field_ident} = new Map(
                                                            Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Set(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                {:case SerdeValueKind::Date}
                                                    instance.@{field.field_ident} = new Set(
                                                        @{raw_var_ident}.map((item) => typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.set_elem_serializable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_deserialize_result_fn_name(elem_type)).into()}
                                                        instance.@{field.field_ident} = new Set(
                                                            @{raw_var_ident}.map((item) => {
                                                                const __elemResult = @{elem_deser_result_fn}(item);
                                                                return __elemResult.success ? __elemResult.value : item;
                                                            })
                                                        );
                                                    {:else}
                                                        instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Serializable(inner_type_name)}
                                        {$let inner_type_expr: Expr = ts_ident!(inner_type_name).into()}
                                        ctx.pushScope("@{field.json_key}");
                                        try {
                                            const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                        } finally {
                                            ctx.popScope();
                                        }

                                    {:case TypeCategory::Nullable(_)}
                                        {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            {:case SerdeValueKind::Date}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    instance.@{field.field_ident} = typeof @{raw_var_ident} === "string"
                                                        ? new Date(@{raw_var_ident})
                                                        : @{raw_var_ident};
                                                }
                                            {:case _}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                        {$let inner_type_expr: Expr = ts_ident!(inner_type).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        } finally {
                                                            ctx.popScope();
                                                        }
                                                    {:else}
                                                        instance.@{field.field_ident} = @{raw_var_ident};
                                                    {/if}
                                                }
                                        {/match}

                                    {:case _}
                                        instance.@{field.field_ident} = @{raw_var_ident};
                                {/match}
                            }
                            {#if let Some(default_expr) = &field.default_expr}
                                if (!("@{field.json_key}" in obj) || obj["@{field.json_key}"] === undefined) {
                                    instance.@{field.field_ident} = @{default_expr};
                                }
                            {/if}
                        {:else}
                            {
                                const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.raw_cast_type};
                                {#match &field.type_cat}
                                    {:case TypeCategory::Primitive}
                                        {#if has_validators}
                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                            {$typescript validation_code}

                                        {/if}
                                        {#if field.decimal_format}
                                            {
                                                const __numVal = globalThis.Number(@{raw_var_ident});
                                                if (globalThis.Number.isNaN(__numVal)) {
                                                    errors.push({ field: "@{field.json_key}", message: "expected a numeric string, got " + JSON.stringify(@{raw_var_ident}) });
                                                } else {
                                                    instance.@{field.field_ident} = __numVal;
                                                }
                                            }
                                        {:else}
                                            instance.@{field.field_ident} = @{raw_var_ident};
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {
                                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                {$typescript validation_code}

                                            {/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                                {$typescript validation_code}

                                            {/if}

                                            {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                                                {:case SerdeValueKind::Date}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => typeof item === "string" ? new Date(item) : item as Date
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_deserialize_result_fn_name(elem_type)).into()}
                                                        const __arr = @{raw_var_ident}.map((item: @{inner} | { __ref: number }, idx) => {
                                                            if (typeof item === "object" && item !== null && "__ref" in item) {
                                                                const result = ctx.getOrDefer(item.__ref);
                                                                if (@{pending_ref_expr}.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            const __elemResult = @{elem_deser_result_fn}(item);
                                                            return __elemResult.success ? __elemResult.value : item;
                                                        });
                                                        instance.@{field.field_ident} = __arr;
                                                        __arr.forEach((item, idx) => {
                                                            if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                                ctx.addPatch(instance.@{field.field_ident}, idx, (item as any).__refId);
                                                            }
                                                        });
                                                    {:else}
                                                        instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Map(key_type, value_type)}
                                        if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                            {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                {:case SerdeValueKind::Date}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, typeof v === "string" ? new Date(v) : v as Date])
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type_name) = &field.map_value_serializable_type}
                                                        {$let value_deser_result_fn: Expr = ts_ident!(nested_deserialize_result_fn_name(value_type_name)).into()}
                                                        instance.@{field.field_ident} = new Map(
                                                            Object.entries(@{raw_var_ident}).map(([k, v]) => {
                                                                const __vResult = @{value_deser_result_fn}(v);
                                                                return [k as @{key_type}, __vResult.success ? __vResult.value : v as @{value_type}];
                                                            })
                                                        );
                                                    {:else}
                                                        instance.@{field.field_ident} = new Map(
                                                            Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Set(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                {:case SerdeValueKind::Date}
                                                    instance.@{field.field_ident} = new Set(
                                                        @{raw_var_ident}.map((item) => typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.set_elem_serializable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_deserialize_result_fn_name(elem_type)).into()}
                                                        instance.@{field.field_ident} = new Set(
                                                            @{raw_var_ident}.map((item) => {
                                                                const __elemResult = @{elem_deser_result_fn}(item);
                                                                return __elemResult.success ? __elemResult.value : item;
                                                            })
                                                        );
                                                    {:else}
                                                        instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Serializable(inner_type_name)}
                                        {$let inner_type_expr: Expr = ts_ident!(inner_type_name).into()}
                                        ctx.pushScope("@{field.json_key}");
                                        try {
                                            const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                        } finally {
                                            ctx.popScope();
                                        }

                                    {:case TypeCategory::Nullable(_)}
                                        {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            {:case SerdeValueKind::Date}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    instance.@{field.field_ident} = typeof @{raw_var_ident} === "string"
                                                        ? new Date(@{raw_var_ident})
                                                        : @{raw_var_ident};
                                                }
                                            {:case _}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                        {$let inner_type_expr: Expr = ts_ident!(inner_type).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        } finally {
                                                            ctx.popScope();
                                                        }
                                                    {:else}
                                                        instance.@{field.field_ident} = @{raw_var_ident};
                                                    {/if}
                                                }
                                        {/match}

                                    {:case _}
                                        instance.@{field.field_ident} = @{raw_var_ident};
                                {/match}
                            }
                        {/if}
                        {/if}
                    {/for}
                {/if}

                ctx.pushErrors(errors);

                return instance as @{type_ident};
            }

            export function @{fn_validate_field_ident}(
                _field: K,
                _value: @{type_ident}[K]
            ): Array<{ field: string; message: string }> {
                {#if !fields_with_validators.is_empty()}
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &fields_with_validators}
                if (_field === "@{field.field_name}") {
                    const __val = _value as @{field.ts_type};
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                    {$typescript validation_code}

                }
                {/for}
                return errors;
                {:else}
                return [];
                {/if}
            }

            export function @{fn_validate_fields_ident}(
                _partial: Partial<@{type_ident}>
            ): Array<{ field: string; message: string }> {
                {#if !fields_with_validators.is_empty()}
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &fields_with_validators}
                if ("@{field.field_name}" in _partial && _partial.@{field.field_ident} !== undefined) {
                    const __val = _partial.@{field.field_ident} as @{field.ts_type};
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                    {$typescript validation_code}

                }
                {/for}
                return errors;
                {:else}
                return [];
                {/if}
            }

            export function @{fn_has_shape_ident}(obj: unknown): boolean {
                if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                    return false;
                }
                {#if has_required}
                    const o = obj as Record<string, unknown>;
                    return @{shape_check_condition};
                {:else}
                    return true;
                {/if}
            }

            export function @{fn_is_ident}(obj: unknown): obj is @{full_type_ident} {
                return @{fn_has_shape_expr}(obj);
            }
        }
    };

    result.add_aliased_import("DeserializeContext", "macroforge/serde");
    result.add_aliased_import("DeserializeError", "macroforge/serde");
    result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
    result.add_aliased_import("PendingRef", "macroforge/serde");
    Ok(result)
}

// The union and fallback type alias handlers are included from the original
// derive_deserialize implementation. Due to the extreme size of the union handler
// (1000+ lines of template code), it is kept in its own function.
#[allow(clippy::too_many_arguments)]
fn handle_union_type_alias(
    _input: &DeriveInput,
    type_alias: &crate::ts_syn::DataTypeAlias,
    type_name: &str,
    _type_ident: &crate::swc_ecma_ast::Ident,
    deserialize_context_ident: &crate::swc_ecma_ast::Ident,
    deserialize_context_expr: &Expr,
    deserialize_error_expr: &Expr,
    pending_ref_ident: &crate::swc_ecma_ast::Ident,
    pending_ref_expr: &Expr,
    deserialize_options_ident: &crate::swc_ecma_ast::Ident,
    generic_decl: &str,
    _generic_args: &str,
    full_type_name: &str,
    type_registry: Option<&crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
    members: &[crate::ts_syn::abi::ir::type_alias::TypeMember],
) -> Result<TsStream, MacroforgeError> {
    // Union type - could be literal union, type ref union, or mixed
    let container_opts = SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Tagging mode variables for template branching
    let _is_internally_tagged =
        matches!(container_opts.tagging, TaggingMode::InternallyTagged { .. });
    let is_externally_tagged = matches!(container_opts.tagging, TaggingMode::ExternallyTagged);
    let is_adjacently_tagged =
        matches!(container_opts.tagging, TaggingMode::AdjacentlyTagged { .. });
    let is_untagged = matches!(container_opts.tagging, TaggingMode::Untagged);
    let content_field = container_opts.content_field().unwrap_or("").to_string();

    // Create a set of type parameter names for filtering
    let type_params = type_alias.type_params();
    let type_param_set: std::collections::HashSet<&str> =
        type_params.iter().map(|s: &String| s.as_str()).collect();

    let literals: Vec<String> = members
        .iter()
        .filter_map(|m| m.as_literal().map(|s| s.to_string()))
        .collect();
    let type_refs: Vec<String> = members
        .iter()
        .filter_map(|m| m.as_type_ref().map(|s| s.to_string()))
        .collect();

    // Separate primitives, generic type params, and serializable types
    let primitive_types: Vec<String> = type_refs
        .iter()
        .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Primitive))
        .cloned()
        .collect();

    // Generic type parameters (like T, U) - these are passed through as-is
    let generic_type_params: Vec<String> = type_refs
        .iter()
        .filter(|t| type_param_set.contains(t.as_str()))
        .cloned()
        .collect();

    // Build SerializableTypeRef with both full type and base type for runtime access.
    // Foreign types (from macroforge.config.ts) are detected here so the union
    // template can use their configured expressions instead of generating
    // broken `{camelCase}DeserializeWithContext()` function calls.
    let foreign_types_config = get_foreign_types();
    let serializable_types: Vec<SerializableTypeRef> = type_refs
        .iter()
        .filter(|t| {
            !matches!(
                TypeCategory::from_ts_type(t),
                TypeCategory::Primitive | TypeCategory::Date
            ) && !type_param_set.contains(t.as_str())
        })
        .map(|t| {
            let ft_match = TypeCategory::match_foreign_type(t, &foreign_types_config);
            let foreign_deserialize_inline = ft_match
                .config
                .and_then(|ft| ft.deserialize_expr.clone())
                .map(|expr| rewrite_expression_namespaces(&expr));
            let foreign_has_shape_inline = ft_match
                .config
                .and_then(|ft| ft.has_shape_expr.clone())
                .map(|expr| rewrite_expression_namespaces(&expr));
            SerializableTypeRef {
                full_type: t.clone(),
                is_foreign: ft_match.config.is_some(),
                foreign_deserialize_inline,
                foreign_has_shape_inline,
            }
        })
        .collect();

    let date_types: Vec<String> = type_refs
        .iter()
        .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Date))
        .cloned()
        .collect();

    let has_primitives = !primitive_types.is_empty();
    let has_serializables = !serializable_types.is_empty();
    let has_dates = !date_types.is_empty();
    let has_generic_params = !generic_type_params.is_empty();

    // Separate regular and foreign serializable types for different code generation
    let regular_serializables: Vec<&SerializableTypeRef> = serializable_types
        .iter()
        .filter(|t| !t.is_foreign)
        .collect();
    let foreign_serializables: Vec<&SerializableTypeRef> =
        serializable_types.iter().filter(|t| t.is_foreign).collect();

    let is_literal_only = !literals.is_empty() && type_refs.is_empty();
    let is_primitive_only = has_primitives
        && !has_serializables
        && !has_dates
        && !has_generic_params
        && literals.is_empty();
    let is_serializable_only = !has_primitives
        && !has_dates
        && !has_generic_params
        && has_serializables
        && literals.is_empty();
    let has_literals = !literals.is_empty();

    // Pre-compute the expected types string for error messages
    let expected_types_str = if has_serializables {
        serializable_types
            .iter()
            .map(|t| t.full_type.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        type_refs.join(", ")
    };

    let primitive_check_condition: String = if primitive_types.is_empty() {
        "false".to_string()
    } else {
        primitive_types
            .iter()
            .map(|prim| format!("typeof value === \"{}\"", prim))
            .collect::<Vec<_>>()
            .join(" || ")
    };

    let serializable_type_check_condition: String = if serializable_types.is_empty() {
        "false".to_string()
    } else {
        serializable_types
            .iter()
            .map(|type_ref| format!("__typeName === \"{}\"", type_ref.full_type))
            .collect::<Vec<_>>()
            .join(" || ")
    };

    // ── Literal-only unions: emit a simple switch-case block ──
    // No JSON.parse, no DeserializeContext, no PendingRef — just
    // validate input against the known variants directly.
    if is_literal_only {
        let fn_deserialize_ident = ts_ident!(
            "{}Deserialize{}",
            type_name.to_case(Case::Camel),
            generic_decl
        );
        let fn_deserialize_internal_ident = ts_ident!(
            "{}DeserializeWithContext{}",
            type_name.to_case(Case::Camel),
            generic_decl
        );
        let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
        let fn_has_shape_ident =
            ts_ident!("{}HasShape{}", type_name.to_case(Case::Camel), generic_decl);
        let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();
        let full_type_ident = ts_ident!(full_type_name);
        let return_type = deserialize_return_type(full_type_name);
        let return_type_ident = ts_ident!(return_type.as_str());

        let mut result = ts_template! {
            /** Deserializes a literal union value. Validates input against known variants. @param input - Value to validate @returns Result containing the validated value or error */
            export function @{fn_deserialize_ident}(input: unknown): @{return_type_ident} {
                switch (input) {
                    {#for lit in &literals}
                    case @{lit}:
                    {/for}
                        return { success: true, value: input as @{full_type_ident} };
                    default:
                        return { success: false, errors: [{ field: "_root", message: "Invalid value for @{type_name}: expected one of " + [@{literals.join(", ")}].map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(input) }] };
                }
            }

            /** Deserializes with an existing context (validates against known variants). */
            export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
                switch (value) {
                    {#for lit in &literals}
                    case @{lit}:
                    {/for}
                        return value as @{full_type_ident};
                    default:
                        throw new @{deserialize_error_expr}([{
                            field: "_root",
                            message: "Invalid value for @{type_name}: expected one of " + [@{literals.join(", ")}].map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
                        }]);
                }
            }

            export function @{fn_has_shape_ident}(value: unknown): boolean {
                switch (value) {
                    {#for lit in &literals}
                    case @{lit}:
                    {/for}
                        return true;
                    default:
                        return false;
                }
            }

            export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
                return @{fn_has_shape_expr}(value);
            }
        };
        result.add_aliased_import("DeserializeContext", "macroforge/serde");
        result.add_aliased_import("DeserializeError", "macroforge/serde");
        result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
        result.add_aliased_import("PendingRef", "macroforge/serde");
        return Ok(result);
    }

    let fn_deserialize_ident = ts_ident!(
        "{}Deserialize{}",
        type_name.to_case(Case::Camel),
        generic_decl
    );
    let fn_deserialize_internal_ident = ts_ident!(
        "{}DeserializeWithContext{}",
        type_name.to_case(Case::Camel),
        generic_decl
    );
    let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
    let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_ident =
        ts_ident!("{}HasShape{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();

    // Compute return type and wrappers
    let return_type = deserialize_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let full_type_ident = ts_ident!(full_type_name);
    let success_result = wrap_success("resultOrRef");
    let success_result_expr =
        parse_ts_expr(&success_result).expect("deserialize success wrapper should parse");
    let error_root_ref = wrap_error(&format!(
        r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
        type_name
    ));
    let error_root_ref_expr =
        parse_ts_expr(&error_root_ref).expect("deserialize root error wrapper should parse");
    let error_from_catch = wrap_error("e.errors");
    let error_from_catch_expr =
        parse_ts_expr(&error_from_catch).expect("deserialize catch error wrapper should parse");
    let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
    let error_generic_message_expr = parse_ts_expr(&error_generic_message)
        .expect("deserialize generic error wrapper should parse");
    let error_from_ctx = wrap_error("__errors");
    let error_from_ctx_expr =
        parse_ts_expr(&error_from_ctx).expect("deserialize ctx error wrapper should parse");

    // If string is a valid variant, skip JSON.parse — the string IS the value.
    // Check foreign serializable types directly (their hasShape inline tells us
    // if they accept strings) because the type registry may not be available
    // during cache builds.
    let has_string_variant = primitive_types.iter().any(|p| p == "string")
        || has_literals
        || serializable_types.iter().any(|st| {
            st.is_foreign
                && st
                    .foreign_has_shape_inline
                    .as_ref()
                    .is_some_and(|hs| hs.contains("typeof") && hs.contains("\"string\""))
        })
        || type_accepts_string(type_name, type_registry, &foreign_types_config);
    let data_init_expr = if has_string_variant {
        parse_ts_expr("input").expect("data init expr should parse")
    } else {
        parse_ts_expr(r#"typeof input === "string" ? JSON.parse(input) : input"#)
            .expect("data init expr should parse")
    };

    let mut result = ts_template! {
        /** Deserializes input to this type. @param input - Value to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
        export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
            try {
                const data = @{data_init_expr};

                const ctx = @{deserialize_context_expr}.create();
                const resultOrRef = @{fn_deserialize_internal_expr}(data, ctx);

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
                if (e instanceof @{deserialize_error_expr}) {
                    return @{error_from_catch_expr};
                }
                const message = e instanceof Error ? e.message : String(e);
                return @{error_generic_message_expr};
            }
        }

        /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
        export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref) as @{full_type_ident} | @{pending_ref_ident};
                        }

                        {#if is_primitive_only}
                            {#for prim in &primitive_types}
                                if (typeof value === "@{prim}") {
                                    return value as @{full_type_ident};
                                }
                            {/for}

                            throw new @{deserialize_error_expr}([{
                                field: "_root",
                                message: "@{type_name}.deserializeWithContext: expected @{expected_types_str}, got " + typeof value
                            }]);
                        {:else if is_serializable_only}
                            // Foreign types may not be objects — check hasShape first
                            {#for type_ref in &foreign_serializables}
                                {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                    {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                    {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                        {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) {
                                            return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                        }
                                    {/if}
                                {/if}
                            {/for}

                            {#if is_externally_tagged}
                                // Externally tagged: { "TypeName": { ...fields } }
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    if (__keys.length >= 1) {
                                        const __variantName = __keys[0];
                                        const __inner = (value as any)[__variantName];
                                        {#for type_ref in &regular_serializables}
                                            {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__variantName === "@{type_ref.full_type}") {
                                                return @{deserialize_with_context_fn}(__inner != null && typeof __inner === "object" ? __inner : {}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for type_ref in &foreign_serializables}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                if (__variantName === "@{type_ref.full_type}") {
                                                    return (@{foreign_deser_expr})(__inner) as @{full_type_ident};
                                                }
                                            {/if}
                                        {/for}
                                        throw new @{deserialize_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: unknown variant \"" + __variantName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }
                                // String value may be a unit variant name
                                if (typeof value === "string") {
                                    {#for type_ref in &regular_serializables}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (value === "@{type_ref.full_type}") {
                                            return @{deserialize_with_context_fn}({}, ctx) as @{full_type_ident};
                                        }
                                    {/for}
                                }
                                throw new @{deserialize_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.deserializeWithContext: expected externally tagged object with variant key. Expected one of: @{expected_types_str}"
                                }]);
                            {:else if is_adjacently_tagged}
                                // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        const __content = (value as any)["@{content_field}"];
                                        {#for type_ref in &regular_serializables}
                                            {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__typeName === "@{type_ref.full_type}") {
                                                return @{deserialize_with_context_fn}(__content != null && typeof __content === "object" ? __content : {}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for type_ref in &foreign_serializables}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return (@{foreign_deser_expr})(__content) as @{full_type_ident};
                                                }
                                            {/if}
                                        {/for}
                                        throw new @{deserialize_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }
                                throw new @{deserialize_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.deserializeWithContext: expected adjacently tagged object with \"@{tag_field}\" and \"@{content_field}\" fields"
                                }]);
                            {:else if is_untagged}
                                // Untagged: shape matching only, no tag field
                                const __shapeMatches: Array<string> = [];
                                {#for type_ref in &regular_serializables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                {/for}
                                {#for type_ref in &foreign_serializables}
                                    {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                        {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) __shapeMatches.push("@{type_ref.full_type}");
                                    {/if}
                                {/for}

                                if (__shapeMatches.length >= 1) {
                                    {#for type_ref in &regular_serializables}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                            try {
                                                return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_serializables}
                                        {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                            {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                            if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                                try {
                                                    return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                } catch { /* try next variant */ }
                                            }
                                        {/if}
                                    {/for}
                                }

                                throw new @{deserialize_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.deserializeWithContext: value does not match any variant shape. Expected one of: @{expected_types_str}"
                                }]);
                            {:else}
                                // Internally tagged (default): tag-based discrimination with shape matching fallback
                                // Tag-based discrimination (only for object values)
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        {#for type_ref in &regular_serializables}
                                            {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__typeName === "@{type_ref.full_type}") {
                                                return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for type_ref in &foreign_serializables}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                }
                                            {/if}
                                        {/for}

                                        throw new @{deserialize_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }

                                // Infer variant via structural shape matching (works for any value type)
                                const __shapeMatches: Array<string> = [];
                                {#for type_ref in &regular_serializables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                {/for}
                                {#for type_ref in &foreign_serializables}
                                    {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                        {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) __shapeMatches.push("@{type_ref.full_type}");
                                    {/if}
                                {/for}

                                if (__shapeMatches.length === 1) {
                                    {#for type_ref in &regular_serializables}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                            return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_serializables}
                                        {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                            {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                            if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                            }
                                        {/if}
                                    {/for}
                                }

                                if (__shapeMatches.length > 1) {
                                    // Multiple variants match — try each deserializer in order, return first success
                                    {#for type_ref in &regular_serializables}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                            try {
                                                return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_serializables}
                                        {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                            {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                            if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                                try {
                                                    return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                } catch { /* try next variant */ }
                                            }
                                        {/if}
                                    {/for}

                                    throw new @{deserialize_error_expr}([{
                                        field: "_root",
                                        message: "@{type_name}.deserializeWithContext: missing @{tag_field} field and value matches multiple variants: " + __shapeMatches.join(", ") + ". Add a @{tag_field} field to disambiguate."
                                    }]);
                                }

                                throw new @{deserialize_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.deserializeWithContext: missing @{tag_field} field and value does not match any variant shape. Expected one of: @{expected_types_str}"
                                }]);
                            {/if}
                        {:else}
                            {#if has_literals}
                                const allowedLiterals = [@{literals.join(", ")}] as const;
                                if (allowedLiterals.includes(value as any)) {
                                    return value as @{full_type_ident};
                                }
                            {/if}

                            {#if has_primitives}
                                {#for prim in &primitive_types}
                                    if (typeof value === "@{prim}") {
                                        return value as @{full_type_ident};
                                    }
                                {/for}
                            {/if}

                            {#if has_dates}
                                if (value instanceof Date) {
                                    return value as @{full_type_ident};
                                }
                                if (typeof value === "string") {
                                    const __dateVal = new Date(value);
                                    if (!isNaN(__dateVal.getTime())) {
                                        return __dateVal as unknown as @{full_type_ident};
                                    }
                                }
                            {/if}

                            {#if has_serializables}
                                {#if is_externally_tagged}
                                    // Externally tagged: { "TypeName": { ...fields } }
                                    if (typeof value === "object" && value !== null) {
                                        const __keys = Object.keys(value);
                                        if (__keys.length >= 1) {
                                            const __variantName = __keys[0];
                                            const __inner = (value as any)[__variantName];
                                            {#for type_ref in &regular_serializables}
                                                {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__variantName === "@{type_ref.full_type}") {
                                                    return @{deserialize_with_context_fn}(__inner != null && typeof __inner === "object" ? __inner : {}, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in &foreign_serializables}
                                                {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                    {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                    if (__variantName === "@{type_ref.full_type}") {
                                                        return (@{foreign_deser_expr})(__inner) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        }
                                    }
                                    if (typeof value === "string") {
                                        {#for type_ref in &regular_serializables}
                                            {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (value === "@{type_ref.full_type}") {
                                                return @{deserialize_with_context_fn}({}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                    }
                                {:else if is_adjacently_tagged}
                                    // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            const __content = (value as any)["@{content_field}"];
                                            {#for type_ref in &regular_serializables}
                                                {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return @{deserialize_with_context_fn}(__content != null && typeof __content === "object" ? __content : {}, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in &foreign_serializables}
                                                {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                    {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        return (@{foreign_deser_expr})(__content) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        }
                                    }
                                {:else if is_untagged}
                                    // Untagged: shape matching only
                                    {#for type_ref in &regular_serializables}
                                        {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (@{has_shape_fn}(value)) {
                                            try {
                                                return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_serializables}
                                        {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                            {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                if ((@{foreign_shape_expr})(value)) {
                                                    try {
                                                        return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                    } catch { /* try next variant */ }
                                                }
                                            {/if}
                                        {/if}
                                    {/for}
                                {:else}
                                    // Internally tagged (default): tag-based + shape matching fallback
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            {#for type_ref in &regular_serializables}
                                                {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in &foreign_serializables}
                                                {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                                    {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        } else {
                                            // No tag field — infer variant via structural shape matching
                                            const __shapeMatches: Array<string> = [];
                                            {#for type_ref in &regular_serializables}
                                                {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                            {/for}
                                            if (__shapeMatches.length === 1) {
                                                {#for type_ref in &regular_serializables}
                                                    {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                    if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                        return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                                    }
                                                {/for}
                                            }
                                        }
                                    } else {
                                        // Non-object values — regular serializables may still match (e.g. RecordLink can be a string)
                                        const __shapeMatches: Array<string> = [];
                                        {#for type_ref in &regular_serializables}
                                            {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                        {/for}
                                        if (__shapeMatches.length === 1) {
                                            {#for type_ref in &regular_serializables}
                                                {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                    return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                        }
                                    }
                                {/if}
                            {/if}

                            // Foreign types may not be objects — check hasShape outside the object block
                            {#for type_ref in &foreign_serializables}
                                {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                    {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                    {#if let Some(ref deser_inline) = type_ref.foreign_deserialize_inline}
                                        {$let foreign_deser_expr: Expr = *parse_ts_expr(deser_inline).expect("foreign deserialize expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) {
                                            return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                        }
                                    {/if}
                                {/if}
                            {/for}

                            {#if has_generic_params}
                                return value as @{full_type_ident};
                            {/if}

                            throw new @{deserialize_error_expr}([{
                                field: "_root",
                                message: "@{type_name}.deserializeWithContext: value does not match any union member"
                            }]);
        {/if}
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
                        {#if is_literal_only}
                            const allowedValues = [@{literals.join(", ")}] as const;
                            return allowedValues.includes(value as any);
                        {:else if is_primitive_only}
                            return @{primitive_check_condition};
                        {:else if is_serializable_only}
                            // Foreign types with hasShape may not be objects — check first
                            {#for type_ref in &foreign_serializables}
                                {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                    {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                    if ((@{foreign_shape_expr})(value)) return true;
                                {/if}
                            {/for}

                            {#if is_externally_tagged}
                                // Externally tagged: check if object has a key matching a variant name
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    if (__keys.length >= 1) {
                                        const __variantName = __keys[0];
                                        if (@{serializable_type_check_condition.replace("__typeName", "__variantName")}) return true;
                                    }
                                }
                                if (typeof value === "string") {
                                    if (@{serializable_type_check_condition.replace("__typeName", "value")}) return true;
                                }
                                return false;
                            {:else if is_adjacently_tagged}
                                // Adjacently tagged: check for tag and content fields
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string" && "@{content_field}" in (value as any)) {
                                        return @{serializable_type_check_condition};
                                    }
                                }
                                return false;
                            {:else if is_untagged}
                                // Untagged: shape matching only
                                let __matchCount = 0;
                                {#for type_ref in &regular_serializables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __matchCount++;
                                {/for}
                                return __matchCount >= 1;
                            {:else}
                                // Internally tagged (default): tag-based + shape matching
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        return @{serializable_type_check_condition};
                                    }
                                }

                                // Shape matching works for any value type (including non-objects)
                                let __matchCount = 0;
                                {#for type_ref in &regular_serializables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __matchCount++;
                                {/for}
                                return __matchCount >= 1;
                            {/if}
                        {:else}
                            {#if has_literals}
                                const allowedLiterals = [@{literals.join(", ")}] as const;
                                if (allowedLiterals.includes(value as any)) return true;
                            {/if}
                            {#if has_primitives}
                                {#for prim in &primitive_types}
                                    if (typeof value === "@{prim}") return true;
                                {/for}
                            {/if}
                            {#if has_dates}
                                if (value instanceof Date) return true;
                            {/if}
                            {#if has_serializables}
                                {#if is_externally_tagged}
                                    // Externally tagged: check if object has a key matching a variant name
                                    if (typeof value === "object" && value !== null) {
                                        const __keys = Object.keys(value);
                                        if (__keys.length >= 1) {
                                            const __variantName = __keys[0];
                                            if (@{serializable_type_check_condition.replace("__typeName", "__variantName")}) return true;
                                        }
                                    }
                                    if (typeof value === "string") {
                                        if (@{serializable_type_check_condition.replace("__typeName", "value")}) return true;
                                    }
                                {:else if is_adjacently_tagged}
                                    // Adjacently tagged: check for tag and content fields
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string" && "@{content_field}" in (value as any)) {
                                            if (@{serializable_type_check_condition}) return true;
                                        }
                                    }
                                {:else if is_untagged}
                                    // Untagged: shape matching only
                                    let __matchCount = 0;
                                    {#for type_ref in &regular_serializables}
                                        {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (@{has_shape_fn}(value)) __matchCount++;
                                    {/for}
                                    if (__matchCount >= 1) return true;
                                {:else}
                                    // Internally tagged (default)
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            if (@{serializable_type_check_condition}) return true;
                                        } else {
                                            let __matchCount = 0;
                                            {#for type_ref in &regular_serializables}
                                                {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (@{has_shape_fn}(value)) __matchCount++;
                                            {/for}
                                            if (__matchCount === 1) return true;
                                        }
                                    } else {
                                        // Non-object values — regular serializables may still match (e.g. RecordLink can be a string)
                                        let __matchCount = 0;
                                        {#for type_ref in &regular_serializables}
                                            {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (@{has_shape_fn}(value)) __matchCount++;
                                        {/for}
                                        if (__matchCount === 1) return true;
                                    }
                                {/if}
                                // Foreign types with hasShape may not be objects
                                {#for type_ref in &foreign_serializables}
                                    {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                        {$let foreign_shape_expr: Expr = *parse_ts_expr(shape_inline).expect("foreign hasShape expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) return true;
                                    {/if}
                                {/for}
                            {/if}
                            {#if has_generic_params}
                                return true;
                            {:else}
                                return false;
                            {/if}
        {/if}
        }

        export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
            return @{fn_has_shape_expr}(value);
        }
    };
    result.add_aliased_import("DeserializeContext", "macroforge/serde");
    result.add_aliased_import("DeserializeError", "macroforge/serde");
    result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
    result.add_aliased_import("PendingRef", "macroforge/serde");
    Ok(result)
}

#[allow(clippy::too_many_arguments)]
fn handle_fallback_type_alias(
    _input: &DeriveInput,
    _type_alias: &crate::ts_syn::DataTypeAlias,
    type_name: &str,
    type_ident: &crate::swc_ecma_ast::Ident,
    deserialize_context_ident: &crate::swc_ecma_ast::Ident,
    deserialize_context_expr: &Expr,
    deserialize_error_expr: &Expr,
    _pending_ref_ident: &crate::swc_ecma_ast::Ident,
    _pending_ref_expr: &Expr,
    deserialize_options_ident: &crate::swc_ecma_ast::Ident,
    generic_decl: &str,
    generic_args: &str,
    full_type_name: &str,
    validate_field_generic_decl: &str,
    type_registry: Option<&crate::ts_syn::abi::ir::type_registry::TypeRegistry>,
) -> Result<TsStream, MacroforgeError> {
    // Fallback for other type alias forms (simple alias, tuple, etc.)
    let fn_deserialize_ident = ts_ident!(
        "{}Deserialize{}",
        type_name.to_case(Case::Camel),
        generic_decl
    );
    let fn_deserialize_internal_ident = ts_ident!(
        "{}DeserializeWithContext{}",
        type_name.to_case(Case::Camel),
        generic_args
    );
    let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
    let fn_validate_field_ident = ts_ident!(
        "{}ValidateField{}",
        type_name.to_case(Case::Camel),
        validate_field_generic_decl
    );
    let fn_validate_fields_ident = ts_ident!("{}ValidateFields", type_name.to_case(Case::Camel));
    let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_ident =
        ts_ident!("{}HasShape{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();
    let full_type_ident = ts_ident!(full_type_name);

    // Compute return type and wrappers
    let return_type = deserialize_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("result");
    let success_result_expr =
        parse_ts_expr(&success_result).expect("deserialize success wrapper should parse");
    let error_from_catch = wrap_error("e.errors");
    let error_from_catch_expr =
        parse_ts_expr(&error_from_catch).expect("deserialize catch error wrapper should parse");
    let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
    let error_generic_message_expr = parse_ts_expr(&error_generic_message)
        .expect("deserialize generic error wrapper should parse");
    let error_from_ctx = wrap_error("__errors");
    let error_from_ctx_expr =
        parse_ts_expr(&error_from_ctx).expect("deserialize ctx error wrapper should parse");

    // Use the type registry and foreign types to determine if this type accepts strings.
    let foreign_types_config = get_foreign_types();
    let accepts_string = type_accepts_string(type_name, type_registry, &foreign_types_config);
    let data_init_expr = if accepts_string {
        parse_ts_expr("input").expect("data init expr should parse")
    } else {
        parse_ts_expr(r#"typeof input === "string" ? JSON.parse(input) : input"#)
            .expect("data init expr should parse")
    };

    let mut result = ts_template! {
        /** Deserializes input to this type. @param input - Value to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
        export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
            try {
                const data = @{data_init_expr};

                const ctx = @{deserialize_context_expr}.create();
                const result = @{fn_deserialize_internal_expr}(data, ctx);
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
                if (e instanceof @{deserialize_error_expr}) {
                    return @{error_from_catch_expr};
                }
                const message = e instanceof Error ? e.message : String(e);
                return @{error_generic_message_expr};
            }
        }

        /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
        export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{full_type_ident} {
            if (value?.__ref !== undefined) {
                return ctx.getOrDefer(value.__ref) as @{full_type_ident};
            }
            return value as @{type_ident};
        }

        export function @{fn_validate_field_ident}<K extends keyof @{type_ident}>(
            _field: K,
            _value: @{type_ident}[K]
        ): Array<{ field: string; message: string }> {
            return [];
        }

        export function @{fn_validate_fields_ident}(
            _partial: Partial<@{type_ident}>
        ): Array<{ field: string; message: string }> {
            return [];
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
            return value != null;
        }

        export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
            return @{fn_has_shape_expr}(value);
        }
    };
    result.add_aliased_import("DeserializeContext", "macroforge/serde");
    result.add_aliased_import("DeserializeError", "macroforge/serde");
    result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
    Ok(result)
}
