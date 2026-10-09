use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use convert_case::{Case, Casing};

use super::super::value_kind::EndecValueKind;
use super::super::{EndecContainerOptions, TypeCategory};
use super::field_processing::to_decode_field;
use super::helpers::{nested_decode_fn_name, nested_decode_result_fn_name, primitive_check};
use super::types::DecodeField;
use super::validation::{Missing, generate_field_validations};
use crate::builtin::return_types::{
    DECODE_CONTEXT, DECODE_ERROR, DECODE_OPTIONS, PENDING_REF, decode_return_type, is_ok_check,
    wrap_error, wrap_success,
};

pub(super) fn handle_interface(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    let interface = match &input.data {
        crate::ts_syn::Data::Interface(i) => i,
        _ => unreachable!(),
    };

    let interface_name = input.name();
    let interface_ident = ts_ident!(interface_name);
    let decode_context_ident = ts_ident!(DECODE_CONTEXT);
    let decode_context_expr: Expr = decode_context_ident.clone().into();
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    let pending_ref_ident = ts_ident!(PENDING_REF);
    let pending_ref_expr: Expr = pending_ref_ident.clone().into();
    let decode_options_ident = ts_ident!(DECODE_OPTIONS);
    let container_opts = EndecContainerOptions::from_decorators(&interface.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Collect decodable fields with diagnostic collection
    let type_registry = &input.context.type_registry;
    let caller_file_path = input.context.file_name.as_str();
    let file_imports = input.context.import_registry.file_import_entries();
    let file_imports = file_imports.as_slice();
    let mut all_diagnostics = DiagnosticCollector::new();
    let fields: Vec<DecodeField> = interface
        .fields()
        .iter()
        .filter_map(|field| {
            to_decode_field(
                field.into(),
                &container_opts,
                &mut all_diagnostics,
                type_registry,
                caller_file_path,
                file_imports,
            )
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

    let fn_decode_ident = ts_ident!(format!("{}Decode", interface_name.to_case(Case::Camel)));
    let fn_decode_internal_ident = ts_ident!(format!(
        "{}DecodeWithContext",
        interface_name.to_case(Case::Camel)
    ));
    let fn_decode_expr: Expr = fn_decode_ident.clone().into();
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
    let fn_validatefield_ident = ts_ident!(format!(
        "{}ValidateField",
        interface_name.to_case(Case::Camel)
    ));
    let fn_validate_fields_ident = ts_ident!(format!(
        "{}ValidateFields",
        interface_name.to_case(Case::Camel)
    ));
    let fn_is_ident = ts_ident!(format!("{}Is", interface_name.to_case(Case::Camel)));
    let fn_has_shape_ident = ts_ident!(format!("{}HasShape", interface_name.to_case(Case::Camel)));
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();

    // Compute return type and wrappers
    let return_type = decode_return_type(interface_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("resultOrRef");
    let success_result_expr =
        Expr::parse(&success_result).expect("decode success wrapper should parse");
    let error_root_ref = wrap_error(&format!(
        r#"[{{ field: "_root", message: "{}.decode: root cannot be a forward reference" }}]"#,
        interface_name
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

    // Build known keys array string
    let known_keys_list: Vec<_> = known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

    let mut result = {
        ts_template! {
            /** Decodes input to this interface type.Automatically detects whether input is a JSON string or object.@param input - JSON string or object to decode@param opts - Optional decoding options @returns Result containing the decoded value or validation errors */
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

            /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
            export function @{fn_decode_internal_ident}(value: any, ctx: @{decode_context_ident}): @{interface_ident} | @{pending_ref_ident} {
                if (value?.__ref !== undefined) {
                    return ctx.getOrDefer(value.__ref);
                }

                if (typeof value !== "object" || value === null || Array.isArray(value)) {
                    throw new @{decode_error_expr}([{ field: "_root", message: "@{interface_name}.decodeWithContext: expected an object" }]);
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
                        {#if let Some(fn_expr) = &field.decode_with}
                            // Custom decoding function (decodeWith)
                            {#if field.optional}
                                if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                    {#if has_validators}
                                        try {
                                            const __convertedVal = (@{fn_expr})(obj["@{field.json_key}"]);
                                            {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, interface_name, field.missing())}
                                            {$typescript validation_code}
                                            instance.@{field.field_ident} = __convertedVal;
                                        } catch (__error) {
                                            errors.push({ field: "@{field.json_key}", message: __error instanceof Error ? __error.message : String(__error) });
                                        }
                                    {:else}
                                        try {
                                            instance.@{field.field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                        } catch (__error) {
                                            errors.push({ field: "@{field.json_key}", message: __error instanceof Error ? __error.message : String(__error) });
                                        }
                                    {/if}
                                }
                            {:else}
                                {#if has_validators}
                                    try {
                                        const __convertedVal = (@{fn_expr})(obj["@{field.json_key}"]);
                                        {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, interface_name, field.missing())}
                                        {$typescript validation_code}
                                        instance.@{field.field_ident} = __convertedVal;
                                    } catch (__error) {
                                        errors.push({ field: "@{field.json_key}", message: __error instanceof Error ? __error.message : String(__error) });
                                    }
                                {:else}
                                    try {
                                        instance.@{field.field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                    } catch (__error) {
                                        errors.push({ field: "@{field.json_key}", message: __error instanceof Error ? __error.message : String(__error) });
                                    }
                                {/if}
                            {/if}
                        {:else}
                        {#if field.optional}
                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.raw_cast_type};
                                {#match &field.type_cat}
                                    {:case TypeCategory::Primitive}
                                        {#if field.decimal_format}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                {$typescript validation_code}
                                            {/if}
                                            {
                                                const __numVal = globalThis.Number(@{raw_var_ident});
                                                if (globalThis.Number.isNaN(__numVal)) {
                                                    errors.push({ field: "@{field.json_key}", message: "expected a numeric string, got " + JSON.stringify(@{raw_var_ident}) });
                                                } else {
                                                    instance.@{field.field_ident} = __numVal;
                                                }
                                            }
                                        {:else}
                                            {#if let Some(primitive) = primitive_check(&field, &raw_var_name, interface_name)}
                                                if (@{primitive.mismatch}) {
                                                    errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                } else {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                        {$typescript validation_code}
                                                    {/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                }
                                            {:else}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                    {$typescript validation_code}
                                                {/if}
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            {/if}
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {
                                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                {$typescript validation_code}

                                            {/if}

                                            {#match field.array_elem_kind.unwrap_or(EndecValueKind::Other)}
                                                {:case EndecValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                                                {:case EndecValueKind::Date}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => typeof item === "string" ? new Date(item) : item as Date
                                                    );
                                                {:case EndecValueKind::NullableDate}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.array_elem_encodable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(elem_type)).into()}
                                                        const __arr = @{raw_var_ident}.map((item: @{inner} | { __ref: number }, idx) => {
                                                            {#if let Some(prim) = &field.array_elem_primitive_union_guard}
                                                                // For elements whose static type is a primitive-plus-encodable
                                                                // union, primitive-side items pass through as-is; the typeof guard
                                                                // matches whatever primitive (string, number, boolean, …) was
                                                                // detected at codegen. Only objects need per-element decoding.
                                                                if (typeof item === "@{prim}") {
                                                                    return item;
                                                                }
                                                            {/if}
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
                                            {#match field.map_value_kind.unwrap_or(EndecValueKind::Other)}
                                                {:case EndecValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                {:case EndecValueKind::Date}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, typeof v === "string" ? new Date(v) : v as Date])
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type_name) = &field.map_value_encodable_type}
                                                        {$let value_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(value_type_name)).into()}
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
                                            {#match field.set_elem_kind.unwrap_or(EndecValueKind::Other)}
                                                {:case EndecValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                {:case EndecValueKind::Date}
                                                    instance.@{field.field_ident} = new Set(
                                                        @{raw_var_ident}.map((item) => typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.set_elem_encodable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(elem_type)).into()}
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

                                    {:case TypeCategory::Encodable(type_name)}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(type_name)).into()}
                                        {#if let Some(prim) = &field.primitive_union_guard}
                                            if (typeof @{raw_var_ident} === "@{prim}") {
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                                {#if field.has_union_string_validators()}
                                                    {$let usv_code = generate_field_validations(&field.union_string_validators, &raw_var_name, &field.json_key, interface_name, Missing::Excluded)}
                                                    {$typescript usv_code}
                                                {/if}
                                            } else {
                                                ctx.pushScope("@{field.json_key}");
                                                try {
                                                    const __result = @{decode_with_context_fn}(@{raw_var_ident}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                } catch (__e) {
                                                    if (__e instanceof @{decode_error_expr}) {
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
                                            }
                                        {:else}
                                            ctx.pushScope("@{field.json_key}");
                                            try {
                                                const __result = @{decode_with_context_fn}(@{raw_var_ident}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } catch (__e) {
                                                if (__e instanceof @{decode_error_expr}) {
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
                                        {/if}

                                    {:case TypeCategory::Nullable(_)}
                                        {#match field.nullable_inner_kind.unwrap_or(EndecValueKind::Other)}
                                            {:case EndecValueKind::PrimitiveLike}
                                                {#if let Some(primitive) = primitive_check(&field, &raw_var_name, interface_name)}
                                                    if (@{primitive.mismatch}) {
                                                        errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                    } else {
                                                        {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                                        instance.@{field.field_ident} = @{raw_var_ident};
                                                    }
                                                {:else}
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                {/if}
                                            {:case EndecValueKind::Date}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident};
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = __dateVal;
                                                }
                                            {:case _}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    {#if let Some(inner_type) = &field.nullable_encodable_type}
                                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(inner_type)).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{decode_with_context_fn}(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        } catch (__e) {
                                                            if (__e instanceof @{decode_error_expr}) {
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
                                        {#if let Some(primitive) = primitive_check(&field, &raw_var_name, interface_name)}
                                            if (@{primitive.mismatch}) {
                                                errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                            } else {
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            }
                                        {:else}
                                            instance.@{field.field_ident} = @{raw_var_ident};
                                        {/if}
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
                                        {#if field.decimal_format}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                {$typescript validation_code}
                                            {/if}
                                            {
                                                const __numVal = globalThis.Number(@{raw_var_ident});
                                                if (globalThis.Number.isNaN(__numVal)) {
                                                    errors.push({ field: "@{field.json_key}", message: "expected a numeric string, got " + JSON.stringify(@{raw_var_ident}) });
                                                } else {
                                                    instance.@{field.field_ident} = __numVal;
                                                }
                                            }
                                        {:else}
                                            {#if let Some(primitive) = primitive_check(&field, &raw_var_name, interface_name)}
                                                if (@{primitive.mismatch}) {
                                                    errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                } else {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                        {$typescript validation_code}
                                                    {/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                }
                                            {:else}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                    {$typescript validation_code}
                                                {/if}
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            {/if}
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {
                                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}
                                                {$typescript validation_code}

                                            {/if}

                                            {#match field.array_elem_kind.unwrap_or(EndecValueKind::Other)}
                                                {:case EndecValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = @{raw_var_ident} as @{inner}[];
                                                {:case EndecValueKind::Date}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => typeof item === "string" ? new Date(item) : item as Date
                                                    );
                                                {:case EndecValueKind::NullableDate}
                                                    instance.@{field.field_ident} = @{raw_var_ident}.map(
                                                        (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.array_elem_encodable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(elem_type)).into()}
                                                        const __arr = @{raw_var_ident}.map((item: @{inner} | { __ref: number }, idx) => {
                                                            {#if let Some(prim) = &field.array_elem_primitive_union_guard}
                                                                // For elements whose static type is a primitive-plus-encodable
                                                                // union, primitive-side items pass through as-is; the typeof guard
                                                                // matches whatever primitive (string, number, boolean, …) was
                                                                // detected at codegen. Only objects need per-element decoding.
                                                                if (typeof item === "@{prim}") {
                                                                    return item;
                                                                }
                                                            {/if}
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
                                            {#match field.map_value_kind.unwrap_or(EndecValueKind::Other)}
                                                {:case EndecValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                {:case EndecValueKind::Date}
                                                    instance.@{field.field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident}).map(([k, v]) => [k as @{key_type}, typeof v === "string" ? new Date(v) : v as Date])
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type_name) = &field.map_value_encodable_type}
                                                        {$let value_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(value_type_name)).into()}
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
                                            {#match field.set_elem_kind.unwrap_or(EndecValueKind::Other)}
                                                {:case EndecValueKind::PrimitiveLike}
                                                    instance.@{field.field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                {:case EndecValueKind::Date}
                                                    instance.@{field.field_ident} = new Set(
                                                        @{raw_var_ident}.map((item) => typeof item === "string" ? new Date(item) : item as Date)
                                                    );
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.set_elem_encodable_type}
                                                        {$let elem_deser_result_fn: Expr = ts_ident!(nested_decode_result_fn_name(elem_type)).into()}
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

                                    {:case TypeCategory::Encodable(type_name)}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(type_name)).into()}
                                        {#if let Some(prim) = &field.primitive_union_guard}
                                            if (typeof @{raw_var_ident} === "@{prim}") {
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                                {#if field.has_union_string_validators()}
                                                    {$let usv_code = generate_field_validations(&field.union_string_validators, &raw_var_name, &field.json_key, interface_name, Missing::Excluded)}
                                                    {$typescript usv_code}
                                                {/if}
                                            } else {
                                                ctx.pushScope("@{field.json_key}");
                                                try {
                                                    const __result = @{decode_with_context_fn}(@{raw_var_ident}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                } catch (__e) {
                                                    if (__e instanceof @{decode_error_expr}) {
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
                                            }
                                        {:else}
                                            ctx.pushScope("@{field.json_key}");
                                            try {
                                                const __result = @{decode_with_context_fn}(@{raw_var_ident}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } catch (__e) {
                                                if (__e instanceof @{decode_error_expr}) {
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
                                        {/if}

                                    {:case TypeCategory::Nullable(_)}
                                        {#match field.nullable_inner_kind.unwrap_or(EndecValueKind::Other)}
                                            {:case EndecValueKind::PrimitiveLike}
                                                {#if let Some(primitive) = primitive_check(&field, &raw_var_name, interface_name)}
                                                    if (@{primitive.mismatch}) {
                                                        errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                    } else {
                                                        {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                                        instance.@{field.field_ident} = @{raw_var_ident};
                                                    }
                                                {:else}
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                {/if}
                                            {:case EndecValueKind::Date}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident};
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = __dateVal;
                                                }
                                            {:case _}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    {#if let Some(inner_type) = &field.nullable_encodable_type}
                                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(inner_type)).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{decode_with_context_fn}(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        } catch (__e) {
                                                            if (__e instanceof @{decode_error_expr}) {
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
                                        {#if let Some(primitive) = primitive_check(&field, &raw_var_name, interface_name)}
                                            if (@{primitive.mismatch}) {
                                                errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                            } else {
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            }
                                        {:else}
                                            instance.@{field.field_ident} = @{raw_var_ident};
                                        {/if}
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
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name, field.missing())}
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
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name, field.missing())}
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
                const result = @{fn_decode_expr}(obj);
                return @{Expr::parse(&is_ok_check("result")).expect("decode is_ok expression should parse")};
            }
        }
    };

    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);
    result.add_diagnostics(all_diagnostics.into_vec());
    Ok(result)
}
