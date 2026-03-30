use crate::macros::ts_template;
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_expr, ts_ident,
};

use convert_case::{Case, Casing};

use super::super::{
    SerdeContainerOptions, SerdeFieldOptions, TypeCategory, get_foreign_types,
    rewrite_expression_namespaces,
};
use super::helpers::{
    classify_serde_value_kind, get_serializable_type_name, nested_deserialize_fn_name,
    nested_deserialize_result_fn_name, parse_default_expr, try_composite_foreign_deserialize,
};
use super::types::{DeserializeField, SerdeValueKind, raw_cast_type};
use super::validation::generate_field_validations;
use crate::builtin::return_types::{
    DESERIALIZE_CONTEXT, DESERIALIZE_ERROR, DESERIALIZE_OPTIONS, PENDING_REF,
    deserialize_return_type, is_ok_check, wrap_error, wrap_success,
};

pub(super) fn handle_interface(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    let interface = match &input.data {
        crate::ts_syn::Data::Interface(i) => i,
        _ => unreachable!(),
    };

    let interface_name = input.name();
    let interface_ident = ts_ident!(interface_name);
    let deserialize_context_ident = ts_ident!(DESERIALIZE_CONTEXT);
    let deserialize_context_expr: Expr = deserialize_context_ident.clone().into();
    let deserialize_error_expr: Expr = ts_ident!(DESERIALIZE_ERROR).into();
    let pending_ref_ident = ts_ident!(PENDING_REF);
    let pending_ref_expr: Expr = pending_ref_ident.clone().into();
    let deserialize_options_ident = ts_ident!(DESERIALIZE_OPTIONS);
    let container_opts =
        SerdeContainerOptions::from_decorators(&interface.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Collect deserializable fields with diagnostic collection
    let mut all_diagnostics = DiagnosticCollector::new();
    let fields: Vec<DeserializeField> = interface
        .fields()
        .iter()
        .filter_map(|field| {
            let parse_result =
                SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
            all_diagnostics.extend(parse_result.diagnostics);
            let opts = parse_result.options;

            if !opts.should_deserialize() {
                return None;
            }

            let json_key = opts
                .rename
                .clone()
                .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

            let type_cat = TypeCategory::from_ts_type(&field.ts_type);

            let nullable_inner_kind = match &type_cat {
                TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                _ => None,
            };
            let array_elem_kind = match &type_cat {
                TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                _ => None,
            };

            // Extract serializable type names for direct function calls
            let nullable_serializable_type = match &type_cat {
                TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                _ => None,
            };

            // Collection element type tracking for recursive deserialization
            let array_elem_serializable_type = match &type_cat {
                TypeCategory::Array(inner) => get_serializable_type_name(inner),
                _ => None,
            };
            let set_elem_kind = match &type_cat {
                TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                _ => None,
            };
            let set_elem_serializable_type = match &type_cat {
                TypeCategory::Set(inner) => get_serializable_type_name(inner),
                _ => None,
            };
            let map_value_kind = match &type_cat {
                TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                _ => None,
            };
            let map_value_serializable_type = match &type_cat {
                TypeCategory::Map(_, value) => get_serializable_type_name(value),
                _ => None,
            };
            let record_value_kind = match &type_cat {
                TypeCategory::Record(_, value) => Some(classify_serde_value_kind(value)),
                _ => None,
            };
            let record_value_serializable_type = match &type_cat {
                TypeCategory::Record(_, value) => get_serializable_type_name(value),
                _ => None,
            };
            let wrapper_inner_kind = match &type_cat {
                TypeCategory::Wrapper(inner) => Some(classify_serde_value_kind(inner)),
                _ => None,
            };
            let wrapper_serializable_type = match &type_cat {
                TypeCategory::Wrapper(inner) => get_serializable_type_name(inner),
                _ => None,
            };
            let optional_inner_kind = match &type_cat {
                TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                _ => None,
            };
            let optional_serializable_type = match &type_cat {
                TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                _ => None,
            };

            // Check for foreign type deserializer if no explicit deserialize_with
            let deserialize_with_src = if opts.deserialize_with.is_some() {
                opts.deserialize_with.clone()
            } else {
                // Check if the field's type matches a configured foreign type
                let foreign_types = get_foreign_types();
                let ft_match =
                    TypeCategory::match_foreign_type(&field.ts_type, &foreign_types);
                // Error if import source mismatch (type matches but wrong import)
                if let Some(error) = ft_match.error {
                    all_diagnostics.error(field.span, error);
                }
                // Log warning for informational hints
                if let Some(warning) = ft_match.warning {
                    all_diagnostics.warning(field.span, warning);
                }
                // Rewrite namespace references to use generated aliases
                ft_match
                    .config
                    .and_then(|ft| ft.deserialize_expr.clone())
                    .map(|expr| rewrite_expression_namespaces(&expr))
                    // If no direct match, try composite patterns (e.g., Utc[] | null)
                    .or_else(|| try_composite_foreign_deserialize(&field.ts_type))
            };

            let deserialize_with = deserialize_with_src.as_ref().and_then(|expr_src| {
                match parse_ts_expr(expr_src) {
                    Ok(expr) => Some(*expr),
                    Err(err) => {
                        all_diagnostics.error(
                            field.span,
                            format!(
                                "@serde(deserializeWith): invalid expression for '{}': {err:?}",
                                field.name
                            ),
                        );
                        None
                    }
                }
            });

            let default_expr = opts.default_expr.as_ref().and_then(|expr_src| {
                match parse_default_expr(expr_src) {
                    Ok(expr) => Some(expr),
                    Err(err) => {
                        all_diagnostics.error(
                            field.span,
                            format!(
                                "@serde({{default: ...}}): invalid expression for '{}': {err:?}",
                                field.name
                            ),
                        );
                        None
                    }
                }
            });

            Some(DeserializeField {
                json_key,
                field_name: field.name.clone(),
                field_ident: ts_ident!(field.name.as_str()),
                raw_cast_type: raw_cast_type(&field.ts_type, &type_cat),
                ts_type: field.ts_type.clone(),
                type_cat,
                optional: field.optional || opts.default || opts.default_expr.is_some(),
                has_default: opts.default || opts.default_expr.is_some(),
                default_expr,
                flatten: opts.flatten,
                validators: opts.validators.clone(),
                nullable_inner_kind,
                array_elem_kind,
                nullable_serializable_type,
                deserialize_with,
                decimal_format: opts.format.as_deref() == Some("decimal"),
                array_elem_serializable_type,
                set_elem_kind,
                set_elem_serializable_type,
                map_value_kind,
                map_value_serializable_type,
                record_value_kind,
                record_value_serializable_type,
                wrapper_inner_kind,
                wrapper_serializable_type,
                optional_inner_kind,
                optional_serializable_type,
            })
        })
        .collect();

    // Check for errors in field parsing before continuing
    if all_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
    }

    let all_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
    let required_fields: Vec<_> = fields
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

    // Generate shape check condition for hasShape method
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
        "{}Deserialize",
        interface_name.to_case(Case::Camel)
    ));
    let fn_deserialize_internal_ident = ts_ident!(format!(
        "{}DeserializeWithContext",
        interface_name.to_case(Case::Camel)
    ));
    let fn_deserialize_expr: Expr = fn_deserialize_ident.clone().into();
    let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
    let fn_validatefield_ident = ts_ident!(format!(
        "{}ValidateField",
        interface_name.to_case(Case::Camel)
    ));
    let fn_validate_fields_ident = ts_ident!(format!(
        "{}ValidateFields",
        interface_name.to_case(Case::Camel)
    ));
    let fn_is_ident = ts_ident!(format!("{}Is", interface_name.to_case(Case::Camel)));
    let fn_has_shape_ident =
        ts_ident!(format!("{}HasShape", interface_name.to_case(Case::Camel)));
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();

    // Compute return type and wrappers
    let return_type = deserialize_return_type(interface_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("resultOrRef");
    let success_result_expr =
        parse_ts_expr(&success_result).expect("deserialize success wrapper should parse");
    let error_root_ref = wrap_error(&format!(
        r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
        interface_name
    ));
    let error_root_ref_expr = parse_ts_expr(&error_root_ref)
        .expect("deserialize root error wrapper should parse");
    let error_from_catch = wrap_error("e.errors");
    let error_from_catch_expr = parse_ts_expr(&error_from_catch)
        .expect("deserialize catch error wrapper should parse");
    let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
    let error_generic_message_expr = parse_ts_expr(&error_generic_message)
        .expect("deserialize generic error wrapper should parse");
    let error_from_ctx = wrap_error("__errors");
    let error_from_ctx_expr =
        parse_ts_expr(&error_from_ctx).expect("deserialize ctx error wrapper should parse");

    // Build known keys array string
    let known_keys_list: Vec<_> = known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

    let mut result = {
        ts_template! {
            /** Deserializes input to this interface type.Automatically detects whether input is a JSON string or object.@param input - JSON string or object to deserialize@param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
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
            export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{interface_ident} | @{pending_ref_ident} {
                if (value?.__ref !== undefined) {
                    return ctx.getOrDefer(value.__ref);
                }

                if (typeof value !== "object" || value === null || Array.isArray(value)) {
                    throw new @{deserialize_error_expr}([{ field: "_root", message: "@{interface_name}.deserializeWithContext: expected an object" }]);
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
                                            {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, interface_name)}
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
                                        {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, interface_name)}
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
                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
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
                                                {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                {$typescript validation_code}

                                            {/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
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
                                                            if (!__elemResult.success) {
                                                                for (const __err of __elemResult.errors) {
                                                                    errors.push({
                                                                        field: __err.field === "_root"
                                                                            ? "@{field.json_key}[" + idx + "]"
                                                                            : "@{field.json_key}[" + idx + "]." + __err.field,
                                                                        message: __err.message
                                                                    });
                                                                }
                                                            }
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
                                                                if (!__vResult.success) {
                                                                    for (const __err of __vResult.errors) {
                                                                        errors.push({
                                                                            field: __err.field === "_root"
                                                                                ? "@{field.json_key}." + k
                                                                                : "@{field.json_key}." + k + "." + __err.field,
                                                                            message: __err.message
                                                                        });
                                                                    }
                                                                }
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
                                                            @{raw_var_ident}.map((item, __setIdx) => {
                                                                const __elemResult = @{elem_deser_result_fn}(item);
                                                                if (!__elemResult.success) {
                                                                    for (const __err of __elemResult.errors) {
                                                                        errors.push({
                                                                            field: __err.field === "_root"
                                                                                ? "@{field.json_key}[" + __setIdx + "]"
                                                                                : "@{field.json_key}[" + __setIdx + "]." + __err.field,
                                                                            message: __err.message
                                                                        });
                                                                    }
                                                                }
                                                                return __elemResult.success ? __elemResult.value : item;
                                                            })
                                                        );
                                                    {:else}
                                                        instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Serializable(type_name)}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(type_name)).into()}
                                        ctx.pushScope("@{field.json_key}");
                                        try {
                                            const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                        } catch (__e) {
                                            if (__e instanceof @{deserialize_error_expr}) {
                                                for (const __err of __e.errors) {
                                                    errors.push({
                                                        field: __err.field === "_root" ? "@{field.json_key}" : "@{field.json_key}." + __err.field,
                                                        message: __err.message
                                                    });
                                                }
                                            } else {
                                                throw __e;
                                            }
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
                                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(inner_type)).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        } catch (__e) {
                                                            if (__e instanceof @{deserialize_error_expr}) {
                                                                for (const __err of __e.errors) {
                                                                    errors.push({
                                                                        field: __err.field === "_root" ? "@{field.json_key}" : "@{field.json_key}." + __err.field,
                                                                        message: __err.message
                                                                    });
                                                                }
                                                            } else {
                                                                throw __e;
                                                            }
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
                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
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
                                                {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                {$typescript validation_code}

                                            {/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
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
                                                            if (!__elemResult.success) {
                                                                for (const __err of __elemResult.errors) {
                                                                    errors.push({
                                                                        field: __err.field === "_root"
                                                                            ? "@{field.json_key}[" + idx + "]"
                                                                            : "@{field.json_key}[" + idx + "]." + __err.field,
                                                                        message: __err.message
                                                                    });
                                                                }
                                                            }
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
                                                                if (!__vResult.success) {
                                                                    for (const __err of __vResult.errors) {
                                                                        errors.push({
                                                                            field: __err.field === "_root"
                                                                                ? "@{field.json_key}." + k
                                                                                : "@{field.json_key}." + k + "." + __err.field,
                                                                            message: __err.message
                                                                        });
                                                                    }
                                                                }
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
                                                            @{raw_var_ident}.map((item, __setIdx) => {
                                                                const __elemResult = @{elem_deser_result_fn}(item);
                                                                if (!__elemResult.success) {
                                                                    for (const __err of __elemResult.errors) {
                                                                        errors.push({
                                                                            field: __err.field === "_root"
                                                                                ? "@{field.json_key}[" + __setIdx + "]"
                                                                                : "@{field.json_key}[" + __setIdx + "]." + __err.field,
                                                                            message: __err.message
                                                                        });
                                                                    }
                                                                }
                                                                return __elemResult.success ? __elemResult.value : item;
                                                            })
                                                        );
                                                    {:else}
                                                        instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                    {/if}
                                            {/match}
                                        }

                                    {:case TypeCategory::Serializable(type_name)}
                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(type_name)).into()}
                                        ctx.pushScope("@{field.json_key}");
                                        try {
                                            const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                        } catch (__e) {
                                            if (__e instanceof @{deserialize_error_expr}) {
                                                for (const __err of __e.errors) {
                                                    errors.push({
                                                        field: __err.field === "_root" ? "@{field.json_key}" : "@{field.json_key}." + __err.field,
                                                        message: __err.message
                                                    });
                                                }
                                            } else {
                                                throw __e;
                                            }
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
                                                        {$let deserialize_with_context_fn: Expr = ts_ident!(nested_deserialize_fn_name(inner_type)).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        } catch (__e) {
                                                            if (__e instanceof @{deserialize_error_expr}) {
                                                                for (const __err of __e.errors) {
                                                                    errors.push({
                                                                        field: __err.field === "_root" ? "@{field.json_key}" : "@{field.json_key}." + __err.field,
                                                                        message: __err.message
                                                                    });
                                                                }
                                                            } else {
                                                                throw __e;
                                                            }
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

                return instance as @{interface_ident};
            }

            export function @{fn_validatefield_ident}<K extends keyof @{interface_ident}>(
                _field: K,
                _value: @{interface_ident}[K]
            ): Array<{ field: string; message: string }> {
                {#if !fields_with_validators.is_empty()}
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &fields_with_validators}
                if (_field === "@{field.field_name}") {
                    const __val = _value as @{field.ts_type};
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                    {$typescript validation_code}

                }
                {/for}
                return errors;
                {:else}
                return [];
                {/if}
            }

            export function @{fn_validate_fields_ident}(
                _partial: Partial<@{interface_ident}>
            ): Array<{ field: string; message: string }> {
                {#if !fields_with_validators.is_empty()}
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &fields_with_validators}
                if ("@{field.field_name}" in _partial && _partial.@{field.field_ident} !== undefined) {
                    const __val = _partial.@{field.field_ident} as @{field.ts_type};
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
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
                const o = obj as Record<string, unknown>;
                return @{shape_check_condition};
            }

            export function @{fn_is_ident}(obj: unknown): obj is @{interface_ident} {
                if (!@{fn_has_shape_expr}(obj)) {
                    return false;
                }
                const result = @{fn_deserialize_expr}(obj);
                return @{parse_ts_expr(&is_ok_check("result")).expect("deserialize is_ok expression should parse")};
            }
        }
    };

    result.add_aliased_import("DeserializeContext", "macroforge/serde");
    result.add_aliased_import("DeserializeError", "macroforge/serde");
    result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
    result.add_aliased_import("PendingRef", "macroforge/serde");
    Ok(result)
}
