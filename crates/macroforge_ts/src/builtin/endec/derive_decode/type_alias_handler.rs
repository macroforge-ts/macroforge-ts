use crate::ast::{Expr, Ident};
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use convert_case::{Case, Casing};

use super::super::validators::ValidatorSpec;
use super::super::value_kind::EndecValueKind;
use super::super::{
    EndecContainerOptions, TaggingMode, TypeCategory, decorator_validators, get_foreign_types,
    primitive_base, rewrite_expression_namespaces,
};
use super::field_processing::to_decode_field;
use super::helpers::{
    extract_base_type, nested_decode_fn_name, nested_decode_result_fn_name,
    nested_has_shape_fn_name, primitive_check, type_accepts_string,
};
use super::types::{DecodeField, EncodableTypeRef};
use super::validation::{Missing, generate_field_validations};
use crate::builtin::return_types::{
    DECODE_CONTEXT, DECODE_ERROR, DECODE_OPTIONS, PENDING_REF, decode_return_type, wrap_error,
    wrap_success,
};
use crate::ts_syn::abi::ir::type_registry::{FileImportEntry, TypeRegistry};

/// What every type alias `Decode` generator shares.
struct AliasDecode<'a> {
    type_alias: &'a crate::ts_syn::DataTypeAlias,
    type_name: &'a str,
    type_ident: Ident,
    decode_context_ident: Ident,
    decode_context_expr: Expr,
    decode_error_expr: Expr,
    pending_ref_ident: Ident,
    pending_ref_expr: Expr,
    decode_options_ident: Ident,
    generic_decl: String,
    generic_args: String,
    full_type_name: String,
    validate_field_generic_decl: String,
    type_registry: &'a TypeRegistry,
    caller_file_path: &'a str,
    file_imports: &'a [FileImportEntry],
}

pub(super) fn handle_type_alias(input: &DeriveInput) -> Result<TsStream, MacroforgeError> {
    let type_alias = match &input.data {
        crate::ts_syn::Data::TypeAlias(ta) => ta,
        _ => unreachable!(),
    };
    let file_imports = input.context.import_registry.file_import_entries();
    let type_name = input.name();
    let decode_context_ident = ts_ident!(DECODE_CONTEXT);
    let pending_ref_ident = ts_ident!(PENDING_REF);

    // Build generic type signature if type has type params
    let type_params = type_alias.type_params();
    let (generic_decl, generic_args) = if type_params.is_empty() {
        (String::new(), String::new())
    } else {
        let params = type_params.join(", ");
        (format!("<{}>", params), format!("<{}>", params))
    };
    let full_type_name = format!("{}{}", type_name, generic_args);

    // Create combined generic declarations for validateField that include K
    let validate_field_generic_decl = if type_params.is_empty() {
        format!("<K extends keyof {}>", type_name)
    } else {
        let params = type_params.join(", ");
        format!("<{}, K extends keyof {}>", params, full_type_name)
    };

    let alias = AliasDecode {
        type_alias,
        type_name,
        type_ident: ts_ident!(type_name),
        decode_context_expr: decode_context_ident.clone().into(),
        decode_context_ident,
        decode_error_expr: ts_ident!(DECODE_ERROR).into(),
        pending_ref_expr: pending_ref_ident.clone().into(),
        pending_ref_ident,
        decode_options_ident: ts_ident!(DECODE_OPTIONS),
        generic_decl,
        generic_args,
        full_type_name,
        validate_field_generic_decl,
        type_registry: &input.context.type_registry,
        caller_file_path: input.context.file_name.as_str(),
        file_imports: &file_imports,
    };

    let mut validator_diagnostics = DiagnosticCollector::new();
    let validators = decorator_validators(
        &type_alias.inner.decorators,
        &format!("type '{type_name}'"),
        &mut validator_diagnostics,
    );
    if validator_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(validator_diagnostics.into_vec()).into());
    }

    if let Some(primitive) = primitive_base(type_alias.body()) {
        return handle_primitive_type_alias(&alias, primitive, &validators);
    }
    if !validators.is_empty() {
        return Err(MacroforgeError::new(
            type_alias.inner.span,
            format!(
                "@endec validators on type '{type_name}' only apply to primitive and branded primitive aliases; put them on the fields instead"
            ),
        ));
    }

    if let Some(fields) = type_alias.as_object() {
        handle_object_type_alias(&alias, fields)
    } else if let Some(members) = type_alias.as_intersection() {
        match crate::builtin::derive_common::flatten_intersection_fields(
            members,
            alias.type_registry,
        ) {
            Some(fields) => handle_object_type_alias(&alias, &fields),
            None => handle_fallback_type_alias(&alias),
        }
    } else if let Some(members) = type_alias.as_union() {
        handle_union_type_alias(&alias, members)
    } else {
        handle_fallback_type_alias(&alias)
    }
}

fn handle_object_type_alias(
    alias: &AliasDecode,
    ir_fields: &[crate::ts_syn::abi::InterfaceFieldIR],
) -> Result<TsStream, MacroforgeError> {
    let AliasDecode {
        type_alias,
        type_name,
        type_ident,
        decode_context_ident,
        decode_context_expr,
        decode_error_expr,
        pending_ref_ident,
        pending_ref_expr,
        decode_options_ident,
        generic_decl,
        full_type_name,
        validate_field_generic_decl,
        type_registry,
        caller_file_path,
        file_imports,
        ..
    } = alias;
    let container_opts = EndecContainerOptions::from_decorators(&type_alias.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Collect decodable fields with diagnostic collection
    let mut all_diagnostics = DiagnosticCollector::new();
    let fields: Vec<DecodeField> = ir_fields
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

    let all_fields: Vec<DecodeField> = fields.iter().filter(|f| !f.flatten).cloned().collect();
    let required_fields: Vec<DecodeField> = fields
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

    let fn_decode_ident = ts_ident!(format!(
        "{}Decode{}",
        type_name.to_case(Case::Camel),
        generic_decl
    ));
    let fn_decode_internal_ident = ts_ident!(format!(
        "{}DecodeWithContext",
        type_name.to_case(Case::Camel)
    ));
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
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

    // Build known keys array string
    let known_keys_list: Vec<_> = known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

    // Flag for whether any required fields exist
    let has_required = !required_fields.is_empty();

    let mut result = {
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

            /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
            export function @{fn_decode_internal_ident}(value: any, ctx: @{decode_context_ident}): @{type_ident} | @{pending_ref_ident} {
                if (value?.__ref !== undefined) {
                    return ctx.getOrDefer(value.__ref) as @{type_ident} | @{pending_ref_ident};
                }

                if (typeof value !== "object" || value === null || Array.isArray(value)) {
                    throw new @{decode_error_expr}([{ field: "_root", message: "@{type_name}.decodeWithContext: expected an object" }]);
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
                                            {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, type_name, field.missing())}
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
                                        {$let validation_code = generate_field_validations(&field.validators, "__convertedVal", &field.json_key, type_name, field.missing())}
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
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
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
                                            {#if let Some(primitive) = primitive_check(&field, &raw_var_name, type_name)}
                                                if (@{primitive.mismatch}) {
                                                    errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                } else {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
                                                        {$typescript validation_code}
                                                    {/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                }
                                            {:else}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
                                                    {$typescript validation_code}
                                                {/if}
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            {/if}
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {
                                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
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

                                    {:case TypeCategory::Encodable(inner_type_name)}
                                        {$let inner_type_expr: Expr = ts_ident!(nested_decode_fn_name(inner_type_name)).into()}
                                        {#if let Some(prim) = &field.primitive_union_guard}
                                            if (typeof @{raw_var_ident} === "@{prim}") {
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                                {#if field.has_union_string_validators()}
                                                    {$let usv_code = generate_field_validations(&field.union_string_validators, &raw_var_name, &field.json_key, type_name, Missing::Excluded)}
                                                    {$typescript usv_code}
                                                {/if}
                                            } else {
                                                ctx.pushScope("@{field.json_key}");
                                                try {
                                                    const __result = @{inner_type_expr}(@{raw_var_ident}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                } finally {
                                                    ctx.popScope();
                                                }
                                            }
                                        {:else}
                                            ctx.pushScope("@{field.json_key}");
                                            try {
                                                const __result = @{inner_type_expr}(@{raw_var_ident}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } finally {
                                                ctx.popScope();
                                            }
                                        {/if}

                                    {:case TypeCategory::Nullable(_)}
                                        {#match field.nullable_inner_kind.unwrap_or(EndecValueKind::Other)}
                                            {:case EndecValueKind::PrimitiveLike}
                                                {#if let Some(primitive) = primitive_check(&field, &raw_var_name, type_name)}
                                                    if (@{primitive.mismatch}) {
                                                        errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                    } else {
                                                        {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                                        instance.@{field.field_ident} = @{raw_var_ident};
                                                    }
                                                {:else}
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                {/if}
                                            {:case EndecValueKind::Date}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident};
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = __dateVal;
                                                }
                                            {:case _}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    {#if let Some(inner_type) = &field.nullable_encodable_type}
                                                        {$let inner_type_expr: Expr = ts_ident!(nested_decode_fn_name(inner_type)).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{inner_type_expr}(@{raw_var_ident}, ctx);
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
                                        {#if let Some(primitive) = primitive_check(&field, &raw_var_name, type_name)}
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
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
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
                                            {#if let Some(primitive) = primitive_check(&field, &raw_var_name, type_name)}
                                                if (@{primitive.mismatch}) {
                                                    errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                } else {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
                                                        {$typescript validation_code}
                                                    {/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                }
                                            {:else}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
                                                    {$typescript validation_code}
                                                {/if}
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                            {/if}
                                        {/if}

                                    {:case TypeCategory::Date}
                                        {
                                            const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                            {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                            instance.@{field.field_ident} = __dateVal;
                                        }

                                    {:case TypeCategory::Array(inner)}
                                        if (Array.isArray(@{raw_var_ident})) {
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}
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

                                    {:case TypeCategory::Encodable(inner_type_name)}
                                        {$let inner_type_expr: Expr = ts_ident!(nested_decode_fn_name(inner_type_name)).into()}
                                        {#if let Some(prim) = &field.primitive_union_guard}
                                            if (typeof @{raw_var_ident} === "@{prim}") {
                                                instance.@{field.field_ident} = @{raw_var_ident};
                                                {#if field.has_union_string_validators()}
                                                    {$let usv_code = generate_field_validations(&field.union_string_validators, &raw_var_name, &field.json_key, type_name, Missing::Excluded)}
                                                    {$typescript usv_code}
                                                {/if}
                                            } else {
                                                ctx.pushScope("@{field.json_key}");
                                                try {
                                                    const __result = @{inner_type_expr}(@{raw_var_ident}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                } finally {
                                                    ctx.popScope();
                                                }
                                            }
                                        {:else}
                                            ctx.pushScope("@{field.json_key}");
                                            try {
                                                const __result = @{inner_type_expr}(@{raw_var_ident}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } finally {
                                                ctx.popScope();
                                            }
                                        {/if}

                                    {:case TypeCategory::Nullable(_)}
                                        {#match field.nullable_inner_kind.unwrap_or(EndecValueKind::Other)}
                                            {:case EndecValueKind::PrimitiveLike}
                                                {#if let Some(primitive) = primitive_check(&field, &raw_var_name, type_name)}
                                                    if (@{primitive.mismatch}) {
                                                        errors.push({ field: "@{field.json_key}", message: @{primitive.message} });
                                                    } else {
                                                        {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                                        instance.@{field.field_ident} = @{raw_var_ident};
                                                    }
                                                {:else}
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = @{raw_var_ident};
                                                {/if}
                                            {:case EndecValueKind::Date}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident};
                                                    {#if has_validators}{$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name, field.missing())}{$typescript validation_code}{/if}
                                                    instance.@{field.field_ident} = __dateVal;
                                                }
                                            {:case _}
                                                if (@{raw_var_ident} === null) {
                                                    instance.@{field.field_ident} = null;
                                                } else {
                                                    {#if let Some(inner_type) = &field.nullable_encodable_type}
                                                        {$let inner_type_expr: Expr = ts_ident!(nested_decode_fn_name(inner_type)).into()}
                                                        ctx.pushScope("@{field.json_key}");
                                                        try {
                                                            const __result = @{inner_type_expr}(@{raw_var_ident}, ctx);
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
                                        {#if let Some(primitive) = primitive_check(&field, &raw_var_name, type_name)}
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
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name, field.missing())}
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
                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name, field.missing())}
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

    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);
    result.add_diagnostics(all_diagnostics.into_vec());
    Ok(result)
}

// The union and fallback type alias handlers are included from the original
// derive_decode implementation. Due to the extreme size of the union handler
// (1000+ lines of template code), it is kept in its own function.
fn handle_union_type_alias(
    alias: &AliasDecode,
    members: &[crate::ts_syn::abi::ir::type_alias::TypeMember],
) -> Result<TsStream, MacroforgeError> {
    let AliasDecode {
        type_alias,
        type_name,
        decode_context_ident,
        decode_context_expr,
        decode_error_expr,
        pending_ref_ident,
        pending_ref_expr,
        decode_options_ident,
        generic_decl,
        full_type_name,
        type_registry,
        caller_file_path,
        file_imports,
        ..
    } = alias;
    // Union type - could be literal union, type ref union, or mixed
    let container_opts = EndecContainerOptions::from_decorators(&type_alias.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Tagging mode variables for template branching
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

    // Separate primitives, generic type params, and encodable types
    let primitive_types: Vec<String> = type_refs
        .iter()
        .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Primitive))
        .cloned()
        .collect();

    // Validators written on an arm (`| /** @endec(email) */ string`) run inside
    // that arm's `typeof` branch. No other arm has a single value to validate.
    let mut arm_diagnostics = DiagnosticCollector::new();
    let mut primitive_arms: Vec<(String, Vec<ValidatorSpec>)> = primitive_types
        .iter()
        .map(|prim| (prim.clone(), Vec::new()))
        .collect();
    for member in members {
        let arm = member.type_name().unwrap_or("object");
        let validators = decorator_validators(
            &member.decorators,
            &format!("type '{type_name}' arm '{arm}'"),
            &mut arm_diagnostics,
        );
        if validators.is_empty() {
            continue;
        }
        match primitive_arms
            .iter_mut()
            .find(|(prim, _)| member.as_type_ref() == Some(prim.as_str()))
        {
            Some((_, arm_validators)) => arm_validators.extend(validators),
            None => arm_diagnostics.error(
                member.decorators.first().map_or(type_alias.inner.span, |d| d.span),
                format!(
                    "@endec validators on arm '{arm}' of type '{type_name}' only apply to primitive arms; put them on the variant's fields instead"
                ),
            ),
        }
    }
    if arm_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(arm_diagnostics.into_vec()).into());
    }
    let has_arm_validators = primitive_arms
        .iter()
        .any(|(_, validators)| !validators.is_empty());

    // Generic type parameters (like T, U) - these are passed through as-is
    let generic_type_params: Vec<String> = type_refs
        .iter()
        .filter(|t| type_param_set.contains(t.as_str()))
        .cloned()
        .collect();

    // Build EncodableTypeRef with both full type and base type for runtime access.
    // Foreign types (from macroforge.config.ts) are detected here so the union
    // template can use their configured expressions instead of generating
    // broken `{camelCase}DecodeWithContext()` function calls.
    let foreign_types_config = get_foreign_types();
    let encodable_types: Vec<EncodableTypeRef> = type_refs
        .iter()
        .filter(|t| {
            !matches!(
                TypeCategory::from_ts_type(t),
                TypeCategory::Primitive | TypeCategory::Date
            ) && !type_param_set.contains(t.as_str())
        })
        .map(|t| {
            let ft_match = TypeCategory::match_foreign_type(t, &foreign_types_config);
            let foreign_decode_inline = ft_match
                .config
                .and_then(|ft| ft.decode_expr.clone())
                .map(|expr| rewrite_expression_namespaces(&expr));
            let foreign_has_shape_inline = ft_match
                .config
                .and_then(|ft| ft.has_shape_expr.clone())
                .map(|expr| rewrite_expression_namespaces(&expr));
            // Strip surrounding quotes from string literal types (e.g., "\"Foo\"" -> "Foo")
            // to prevent double-quoting in template string comparisons.
            let clean_type = if (t.starts_with('"') && t.ends_with('"'))
                || (t.starts_with('\'') && t.ends_with('\''))
            {
                t[1..t.len() - 1].to_string()
            } else {
                t.clone()
            };
            EncodableTypeRef {
                full_type: clean_type,
                is_foreign: ft_match.config.is_some(),
                foreign_decode_inline,
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
    let has_encodables = !encodable_types.is_empty();
    let has_dates = !date_types.is_empty();
    let has_generic_params = !generic_type_params.is_empty();

    // Separate regular and foreign encodable types for different code generation
    let regular_encodables: Vec<&EncodableTypeRef> =
        encodable_types.iter().filter(|t| !t.is_foreign).collect();
    let foreign_encodables: Vec<&EncodableTypeRef> =
        encodable_types.iter().filter(|t| t.is_foreign).collect();

    // Collect inline object variants (e.g., { variant: 'GlobalAdmin' } | { variant: 'AppRoles'; ... })
    // These are tagged by a discriminant field (typically the endec tag field).
    struct ObjectVariant {
        tag_value: String,
        fields: Vec<crate::ts_syn::abi::ir::interface::InterfaceFieldIR>,
    }

    // Collect intersection variants (e.g., { variant: 'AppRoles' } & AppRoles)
    // These have a tag in the inline object part and delegate to a type ref for the data.
    struct IntersectionVariant {
        tag_value: String,
        type_ref: String,
        is_encodable: bool,
    }

    // Inline object members without a tag field (e.g. `string | { id: string }`)
    // can only be told apart by shape: every required field must be present.
    struct UntaggedObjectVariant {
        /// JS condition that is true when `value` has this member's shape.
        shape_condition: String,
        required_fields: Vec<String>,
        optional_fields: Vec<String>,
    }

    let mut object_variants: Vec<ObjectVariant> = Vec::new();
    let mut untagged_object_variants: Vec<UntaggedObjectVariant> = Vec::new();
    let mut intersection_variants: Vec<IntersectionVariant> = Vec::new();

    struct ExternalObjectVariant {
        name: String,
        /// If the variant payload (`fields[0].ts_type`) is a configured foreign
        /// type (e.g. `DateTime.Utc`), this is the inline decode
        /// expression to invoke on `__inner`. Without this, primitive
        /// passthrough returns the raw JSON (e.g. an ISO string) and downstream
        /// code expecting a `DateTime` object breaks.
        inner_foreign_decode_inline: Option<String>,
        /// Set when the payload is a generated type instead: without dispatching
        /// to its decoder the payload is kept verbatim, so decimals stay
        /// strings and dates stay ISO text inside the variant.
        payload_decode_fn: Option<crate::ast::Ident>,
    }
    let mut external_object_variants: Vec<ExternalObjectVariant> = Vec::new();

    for m in members {
        match &m.kind {
            crate::ts_syn::abi::ir::type_alias::TypeMemberKind::Object { fields }
                if is_externally_tagged && !fields.is_empty() =>
            {
                let payload_ts_type = fields[0].ts_type.as_str();
                let inner_foreign_decode_inline =
                    TypeCategory::match_foreign_type(payload_ts_type, &foreign_types_config)
                        .config
                        .and_then(|ft| ft.decode_expr.clone())
                        .map(|expr| rewrite_expression_namespaces(&expr));
                let payload_decode_fn = if inner_foreign_decode_inline.is_some() {
                    None
                } else if let TypeCategory::Encodable(base) =
                    TypeCategory::from_ts_type(payload_ts_type)
                {
                    Some(ts_ident!(nested_decode_fn_name(&extract_base_type(&base))))
                } else {
                    None
                };
                external_object_variants.push(ExternalObjectVariant {
                    name: fields[0].name.clone(),
                    inner_foreign_decode_inline,
                    payload_decode_fn,
                });
            }
            crate::ts_syn::abi::ir::type_alias::TypeMemberKind::Object { fields } => {
                if let Some(tag_value) = fields.iter().find_map(|f| {
                    if f.name == tag_field {
                        let t = f.ts_type.trim().trim_matches('\'').trim_matches('"');
                        Some(t.to_string())
                    } else {
                        None
                    }
                }) {
                    object_variants.push(ObjectVariant {
                        tag_value,
                        fields: fields.clone(),
                    });
                } else {
                    let (optional, required): (Vec<_>, Vec<_>) =
                        fields.iter().partition(|f| f.optional);
                    let required_fields: Vec<String> =
                        required.iter().map(|f| f.name.clone()).collect();
                    let shape_condition = std::iter::once(
                        "typeof value === \"object\" && value !== null && !Array.isArray(value)"
                            .to_string(),
                    )
                    .chain(
                        required_fields
                            .iter()
                            .map(|name| format!("\"{name}\" in value")),
                    )
                    .collect::<Vec<_>>()
                    .join(" && ");
                    untagged_object_variants.push(UntaggedObjectVariant {
                        shape_condition,
                        required_fields,
                        optional_fields: optional.iter().map(|f| f.name.clone()).collect(),
                    });
                }
            }
            crate::ts_syn::abi::ir::type_alias::TypeMemberKind::Intersection(sub_members) => {
                // Look for { tag: 'Value' } & TypeRef pattern
                let mut tag_value = None;
                let mut ref_type = None;

                for sub in sub_members {
                    match &sub.kind {
                        crate::ts_syn::abi::ir::type_alias::TypeMemberKind::Object { fields } => {
                            tag_value = fields.iter().find_map(|f| {
                                if f.name == tag_field {
                                    let t = f.ts_type.trim().trim_matches('\'').trim_matches('"');
                                    Some(t.to_string())
                                } else {
                                    None
                                }
                            });
                        }
                        crate::ts_syn::abi::ir::type_alias::TypeMemberKind::TypeRef(t) => {
                            ref_type = Some(t.clone());
                        }
                        _ => {}
                    }
                }

                if let (Some(tv), Some(rt)) = (tag_value, ref_type) {
                    // A `{ tag: 'X' } & TypeRef` variant's payload must be decoded
                    // through `TypeRef`'s own decoder so nested fields (dates,
                    // records, links) are reconstructed: a shallow `{ ...value, tag }`
                    // spread leaves them as raw JSON. `encodable_types` only holds
                    // DIRECT type-ref union members, so an intersection's inner type is
                    // absent from it; recognize any non-primitive / non-date /
                    // non-type-param / non-foreign type ref (which has a generated
                    // `*DecodeWithContext`) as encodable here.
                    let is_ser = encodable_types.iter().any(|s| s.full_type == rt)
                        || (!matches!(
                            TypeCategory::from_ts_type(&rt),
                            TypeCategory::Primitive | TypeCategory::Date
                        ) && !type_param_set.contains(rt.as_str())
                            && TypeCategory::match_foreign_type(&rt, &foreign_types_config)
                                .config
                                .is_none());
                    intersection_variants.push(IntersectionVariant {
                        tag_value: tv,
                        type_ref: rt,
                        is_encodable: is_ser,
                    });
                }
            }
            _ => {}
        }
    }
    let has_object_variants = !object_variants.is_empty();
    let has_untagged_object_variants = !untagged_object_variants.is_empty();
    let has_intersection_variants = !intersection_variants.is_empty();

    // Per-variant `Is` type guards for inline variants. Type ref variants
    // (regular_encodables) already expose their own `*Is` from their own
    // `@derive(Decode)`, so we only synthesize guards for the inline
    // shapes that have nowhere else to live: internally tagged objects,
    // `{ tag: 'X' } & TypeRef` intersections, and externally tagged objects.
    //
    // Naming follows `{typeCamel}{TagPascal}Is` so downstream consumers can
    // derive the function name from the union and tag value alone, without
    // needing any extra metadata from this crate.
    let per_variant_is_stream = {
        let camel_name = type_name.to_case(Case::Camel);
        let mut guards: Vec<TsStream> = Vec::new();

        // Internally tagged inline object variants: discriminate by the tag
        // field equalling the variant's discriminant value.
        for ov in &object_variants {
            let variant_pascal = ov.tag_value.to_case(Case::Pascal);
            let fn_ident = ts_ident!("{}{}Is", camel_name, variant_pascal);
            let extract_ident = ts_ident!(format!(
                "Extract<{full_type_name}, {{ {tag_field}: '{}' }}>",
                ov.tag_value
            ));
            let tag = tag_field;
            let value = ov.tag_value.as_str();
            guards.push(ts_template! {
                export function @{fn_ident}(__v: unknown): __v is @{extract_ident} {
                    return __v !== null
                        && typeof __v === "object"
                        && (__v as Record<string, unknown>)["@{tag}"] === "@{value}";
                }
            });
        }

        // Intersection variants (`{ tag: 'X' } & TypeRef`): same shape, the
        // tag field discriminates among union members.
        for iv in &intersection_variants {
            let variant_pascal = iv.tag_value.to_case(Case::Pascal);
            let fn_ident = ts_ident!("{}{}Is", camel_name, variant_pascal);
            let extract_ident = ts_ident!(format!(
                "Extract<{full_type_name}, {{ {tag_field}: '{}' }}>",
                iv.tag_value
            ));
            let tag = tag_field;
            let value = iv.tag_value.as_str();
            guards.push(ts_template! {
                export function @{fn_ident}(__v: unknown): __v is @{extract_ident} {
                    return __v !== null
                        && typeof __v === "object"
                        && (__v as Record<string, unknown>)["@{tag}"] === "@{value}";
                }
            });
        }

        // Externally tagged inline objects (`{ TypeName: { ...fields } }`) :
        // discriminate on whether the variant's key is present.
        for ov in &external_object_variants {
            let variant_pascal = ov.name.to_case(Case::Pascal);
            let fn_ident = ts_ident!("{}{}Is", camel_name, variant_pascal);
            let extract_ident =
                ts_ident!(format!("Extract<{full_type_name}, {{ {}: any }}>", ov.name));
            let key = ov.name.as_str();
            guards.push(ts_template! {
                export function @{fn_ident}(__v: unknown): __v is @{extract_ident} {
                    return __v !== null
                        && typeof __v === "object"
                        && "@{key}" in __v;
                }
            });
        }

        TsStream::merge_all(guards)
    };

    let has_tagged_variants = has_object_variants
        || has_untagged_object_variants
        || has_intersection_variants
        || !external_object_variants.is_empty();
    let is_literal_only = !literals.is_empty() && type_refs.is_empty() && !has_tagged_variants;
    let is_primitive_only = has_primitives
        && !has_encodables
        && !has_dates
        && !has_generic_params
        && literals.is_empty()
        && !has_tagged_variants;
    let is_encodable_only = !has_primitives
        && !has_dates
        && !has_generic_params
        && has_encodables
        && literals.is_empty()
        && !has_tagged_variants;
    let has_literals = !literals.is_empty();

    // Pre-compute the expected types string for error messages
    let expected_types_str = {
        let mut parts: Vec<String> = Vec::new();
        if has_encodables {
            parts.extend(encodable_types.iter().map(|t| t.full_type.clone()));
        } else if !type_refs.is_empty() {
            parts.extend(type_refs.iter().cloned());
        }
        for ov in &object_variants {
            parts.push(format!("{{ {}: '{}' }}", tag_field, ov.tag_value));
        }
        for ov in &external_object_variants {
            parts.push(format!("{{ {}: any }}", ov.name));
        }
        for iv in &intersection_variants {
            parts.push(format!(
                "{{ {}: '{}' }} & {}",
                tag_field, iv.tag_value, iv.type_ref
            ));
        }
        if parts.is_empty() {
            literals.join(", ")
        } else {
            parts.join(", ")
        }
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

    let encodable_type_check_condition: String = if encodable_types.is_empty() {
        "false".to_string()
    } else {
        encodable_types
            .iter()
            .map(|type_ref| format!("__typeName === \"{}\"", type_ref.full_type))
            .collect::<Vec<_>>()
            .join(" || ")
    };

    // ── Literal-only unions: emit a simple switch-case block ──
    // No JSON.parse, no DecodeContext, no PendingRef: just
    // validate input against the known variants directly.
    if is_literal_only {
        let fn_decode_ident = ts_ident!("{}Decode{}", type_name.to_case(Case::Camel), generic_decl);
        let fn_decode_internal_ident = ts_ident!(
            "{}DecodeWithContext{}",
            type_name.to_case(Case::Camel),
            generic_decl
        );
        let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
        let fn_has_shape_ident =
            ts_ident!("{}HasShape{}", type_name.to_case(Case::Camel), generic_decl);
        let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();
        let full_type_ident = ts_ident!(full_type_name);
        let return_type = decode_return_type(full_type_name);
        let return_type_ident = ts_ident!(return_type.as_str());

        let mut result = ts_template! {
            /** Decodes a literal union value. Validates input against known variants. @param input - Value to validate @returns Result containing the validated value or error */
            export function @{fn_decode_ident}(input: unknown): @{return_type_ident} {
                switch (input) {
                    {#for lit in &literals}
                    case @{lit}:
                    {/for}
                        return { success: true, value: input as @{full_type_ident} };
                    default:
                        return { success: false, errors: [{ field: "_root", message: "Invalid value for @{type_name}: expected one of " + [@{literals.join(", ")}].map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(input) }] };
                }
            }

            /** Decodes with an existing context (validates against known variants). A literal holds no references, so the context goes unread. */
            export function @{fn_decode_internal_ident}(value: any, _ctx: @{decode_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
                switch (value) {
                    {#for lit in &literals}
                    case @{lit}:
                    {/for}
                        return value as @{full_type_ident};
                    default:
                        throw new @{decode_error_expr}([{
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
        result.add_aliased_import("DecodeContext", crate::package::ENDEC);
        result.add_aliased_import("DecodeError", crate::package::ENDEC);
        result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
        result.add_aliased_import("PendingRef", crate::package::ENDEC);
        return Ok(result);
    }

    let fn_decode_ident = ts_ident!("{}Decode{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_decode_internal_ident = ts_ident!(
        "{}DecodeWithContext{}",
        type_name.to_case(Case::Camel),
        generic_decl
    );
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
    let fn_is_ident = ts_ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_ident =
        ts_ident!("{}HasShape{}", type_name.to_case(Case::Camel), generic_decl);
    let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();
    let fn_decode_expr: Expr = ts_ident!("{}Decode", type_name.to_case(Case::Camel)).into();

    // Compute return type and wrappers
    let return_type = decode_return_type(full_type_name);
    let return_type_ident = ts_ident!(return_type.as_str());
    let full_type_ident = ts_ident!(full_type_name);
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

    // If string is a valid variant, skip JSON.parse: the string IS the value.
    // Check foreign encodable types directly (their hasShape inline tells us
    // if they accept strings) because the type registry may not be available
    // during cache builds.
    let has_string_variant = primitive_types.iter().any(|p| p == "string")
        || has_literals
        || encodable_types.iter().any(|st| {
            st.is_foreign
                && st
                    .foreign_has_shape_inline
                    .as_ref()
                    .is_some_and(|hs| hs.contains("typeof") && hs.contains("\"string\""))
        })
        || type_accepts_string(
            type_name,
            type_registry,
            caller_file_path,
            file_imports,
            &foreign_types_config,
        );
    let data_init_expr = if has_string_variant {
        Expr::parse("input").expect("data init expr should parse")
    } else {
        Expr::parse(r#"typeof input === "string" ? JSON.parse(input) : input"#)
            .expect("data init expr should parse")
    };

    let result = ts_template! {
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

        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: any, ctx: @{decode_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref) as @{full_type_ident} | @{pending_ref_ident};
                        }

                        {#if is_primitive_only}
                            {#for (prim, arm_validators) in &primitive_arms}
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
                        {:else if is_encodable_only}
                            // Foreign types may not be objects: check hasShape first
                            {#for type_ref in &foreign_encodables}
                                {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                    {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                    {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                        {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) {
                                            return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                        }
                                    {/if}
                                {/if}
                            {/for}

                            {#if is_externally_tagged}
                                // Externally tagged: { "TypeName": payload }
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        const __inner = (value as any)[__variantName];
                                        {#for ov in &external_object_variants}
                                            if (__variantName === "@{ov.name}") {
                                                {#if let Some(ref deser_inline) = ov.inner_foreign_decode_inline}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("inner foreign decode expr should parse")}
                                                    return ({ "@{ov.name}": (@{foreign_deser_expr})(__inner) }) as @{full_type_ident};
                                                {:else}
                                                    {#if let Some(ref payload_deser_fn) = ov.payload_decode_fn}
                                                        return ({ "@{ov.name}": @{payload_deser_fn}(__inner ?? {}, ctx) }) as @{full_type_ident};
                                                    {:else}
                                                        return ({ "@{ov.name}": __inner }) as @{full_type_ident};
                                                    {/if}
                                                {/if}
                                            }
                                        {/for}
                                        {#for type_ref in &regular_encodables}
                                            {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__variantName === "@{type_ref.full_type}") {
                                                return @{decode_with_context_fn}(__inner != null && typeof __inner === "object" ? __inner : {}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for type_ref in &foreign_encodables}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                                if (__variantName === "@{type_ref.full_type}") {
                                                    return (@{foreign_deser_expr})(__inner) as @{full_type_ident};
                                                }
                                            {/if}
                                        {/for}
                                        throw new @{decode_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.decodeWithContext: unknown variant \"" + __variantName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }
                                // String value may be a unit variant name
                                if (typeof value === "string") {
                                    {#for type_ref in &regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (value === "@{type_ref.full_type}") {
                                            return @{decode_with_context_fn}({}, ctx) as @{full_type_ident};
                                        }
                                    {/for}
                                }
                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: expected externally tagged object with variant key. Expected one of: @{expected_types_str}"
                                }]);
                            {:else if is_adjacently_tagged}
                                // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        const __content = (value as any)["@{content_field}"];
                                        {#for type_ref in &regular_encodables}
                                            {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__typeName === "@{type_ref.full_type}") {
                                                return @{decode_with_context_fn}(__content != null && typeof __content === "object" ? __content : {}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for type_ref in &foreign_encodables}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return (@{foreign_deser_expr})(__content) as @{full_type_ident};
                                                }
                                            {/if}
                                        {/for}
                                        throw new @{decode_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.decodeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }
                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: expected adjacently tagged object with \"@{tag_field}\" and \"@{content_field}\" fields"
                                }]);
                            {:else if is_untagged}
                                // Untagged: shape matching only, no tag field
                                const __shapeMatches: Array<string> = [];
                                {#for type_ref in &regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                {/for}
                                {#for type_ref in &foreign_encodables}
                                    {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                        {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) __shapeMatches.push("@{type_ref.full_type}");
                                    {/if}
                                {/for}

                                if (__shapeMatches.length >= 1) {
                                    {#for type_ref in &regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                            try {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_encodables}
                                        {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                            {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                            if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                                try {
                                                    return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                } catch { /* try next variant */ }
                                            }
                                        {/if}
                                    {/for}
                                }

                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: value does not match any variant shape. Expected one of: @{expected_types_str}"
                                }]);
                            {:else}
                                // Internally tagged (default): tag-based discrimination with shape matching fallback
                                // Tag-based discrimination (only for object values)
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        {#for type_ref in &regular_encodables}
                                            {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__typeName === "@{type_ref.full_type}") {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for type_ref in &foreign_encodables}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                }
                                            {/if}
                                        {/for}
                                        {#for ov in &object_variants}
                                            if (__typeName === "@{ov.tag_value}") {
                                                // Inline object variant: decode fields in place
                                                const __result: Record<string, unknown> = { "@{tag_field}": "@{ov.tag_value}" };
                                                {#for field in &ov.fields}
                                                    {#if field.name != tag_field}
                                                        __result["@{field.name}"] = (value as any)["@{field.name}"] ?? null;
                                                    {/if}
                                                {/for}
                                                return __result as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for iv in &intersection_variants}
                                            if (__typeName === "@{iv.tag_value}") {
                                                // Intersection variant: decode the type ref and merge with tag
                                                {#if iv.is_encodable}
                                                    {$let iv_deser_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&iv.type_ref))).into()}
                                                    const __inner = @{iv_deser_fn}(value, ctx);
                                                    return { ...(__inner as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {:else}
                                                    return { ...(value as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {/if}
                                            }
                                        {/for}

                                        throw new @{decode_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.decodeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    }
                                }

                                // Infer variant via structural shape matching (works for any value type)
                                const __shapeMatches: Array<string> = [];
                                {#for type_ref in &regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                {/for}
                                {#for type_ref in &foreign_encodables}
                                    {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                        {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) __shapeMatches.push("@{type_ref.full_type}");
                                    {/if}
                                {/for}

                                if (__shapeMatches.length === 1) {
                                    {#for type_ref in &regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                            return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_encodables}
                                        {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                            {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                            if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                            }
                                        {/if}
                                    {/for}
                                }

                                if (__shapeMatches.length > 1) {
                                    // Multiple variants match: try each decoder in order, return first success
                                    {#for type_ref in &regular_encodables}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                            try {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_encodables}
                                        {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                            {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                            if (__shapeMatches.includes("@{type_ref.full_type}")) {
                                                try {
                                                    return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                } catch { /* try next variant */ }
                                            }
                                        {/if}
                                    {/for}

                                    throw new @{decode_error_expr}([{
                                        field: "_root",
                                        message: "@{type_name}.decodeWithContext: missing @{tag_field} field and value matches multiple variants: " + __shapeMatches.join(", ") + ". Add a @{tag_field} field to disambiguate."
                                    }]);
                                }

                                throw new @{decode_error_expr}([{
                                    field: "_root",
                                    message: "@{type_name}.decodeWithContext: missing @{tag_field} field and value does not match any variant shape. Expected one of: @{expected_types_str}"
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
                                {#for (prim, arm_validators) in &primitive_arms}
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

                            {#if has_encodables}
                                {#if is_externally_tagged}
                                    // Externally tagged: { "TypeName": payload }
                                    if (typeof value === "object" && value !== null) {
                                        const __keys = Object.keys(value);
                                        const __variantName = __keys[0];
                                        if (__variantName !== undefined) {
                                            const __inner = (value as any)[__variantName];
                                            {#for ov in &external_object_variants}
                                            if (__variantName === "@{ov.name}") {
                                                {#if let Some(ref deser_inline) = ov.inner_foreign_decode_inline}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("inner foreign decode expr should parse")}
                                                    return ({ "@{ov.name}": (@{foreign_deser_expr})(__inner) }) as @{full_type_ident};
                                                {:else}
                                                    {#if let Some(ref payload_deser_fn) = ov.payload_decode_fn}
                                                        return ({ "@{ov.name}": @{payload_deser_fn}(__inner ?? {}, ctx) }) as @{full_type_ident};
                                                    {:else}
                                                        return ({ "@{ov.name}": __inner }) as @{full_type_ident};
                                                    {/if}
                                                {/if}
                                            }
                                        {/for}
                                        {#for type_ref in &regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__variantName === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(__inner != null && typeof __inner === "object" ? __inner : {}, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in &foreign_encodables}
                                                {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                                    if (__variantName === "@{type_ref.full_type}") {
                                                        return (@{foreign_deser_expr})(__inner) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        }
                                    }
                                    if (typeof value === "string") {
                                        {#for type_ref in &regular_encodables}
                                            {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (value === "@{type_ref.full_type}") {
                                                return @{decode_with_context_fn}({}, ctx) as @{full_type_ident};
                                            }
                                        {/for}
                                    }
                                {:else if is_adjacently_tagged}
                                    // Adjacently tagged: { tag: "TypeName", content: { ...fields } }
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            const __content = (value as any)["@{content_field}"];
                                            {#for type_ref in &regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(__content != null && typeof __content === "object" ? __content : {}, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in &foreign_encodables}
                                                {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        return (@{foreign_deser_expr})(__content) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        }
                                    }
                                {:else if is_untagged}
                                    // Untagged: shape matching only
                                    {#for type_ref in &regular_encodables}
                                        {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (@{has_shape_fn}(value)) {
                                            try {
                                                return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                            } catch { /* try next variant */ }
                                        }
                                    {/for}
                                    {#for type_ref in &foreign_encodables}
                                        {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                            {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                            {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
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
                                            {#for type_ref in &regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__typeName === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                            {#for type_ref in &foreign_encodables}
                                                {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                                    }
                                                {/if}
                                            {/for}
                                        } else {
                                            // No tag field: infer variant via structural shape matching
                                            const __shapeMatches: Array<string> = [];
                                            {#for type_ref in &regular_encodables}
                                                {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                            {/for}
                                            if (__shapeMatches.length === 1) {
                                                {#for type_ref in &regular_encodables}
                                                    {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                    if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                        return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                                    }
                                                {/for}
                                            }
                                        }
                                    } else {
                                        // Non-object values: regular encodables may still match (e.g. RecordLink can be a string)
                                        const __shapeMatches: Array<string> = [];
                                        {#for type_ref in &regular_encodables}
                                            {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (@{has_shape_fn}(value)) __shapeMatches.push("@{type_ref.full_type}");
                                        {/for}
                                        if (__shapeMatches.length === 1) {
                                            {#for type_ref in &regular_encodables}
                                                {$let decode_with_context_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (__shapeMatches[0] === "@{type_ref.full_type}") {
                                                    return @{decode_with_context_fn}(value, ctx) as @{full_type_ident};
                                                }
                                            {/for}
                                        }
                                    }
                                {/if}
                            {/if}

                            // Foreign types may not be objects: check hasShape outside the object block
                            {#for type_ref in &foreign_encodables}
                                {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                    {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                    {#if let Some(ref deser_inline) = type_ref.foreign_decode_inline}
                                        {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("foreign decode expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) {
                                            return (@{foreign_deser_expr})(value) as @{full_type_ident};
                                        }
                                    {/if}
                                {/if}
                            {/for}

                            {#if has_generic_params}
                                return value as @{full_type_ident};
                            {/if}

                            {#if has_object_variants || has_intersection_variants}
                                // Tagged variants with tag-based discrimination
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        {#for ov in &object_variants}
                                            if (__typeName === "@{ov.tag_value}") {
                                                const __result: Record<string, unknown> = { "@{tag_field}": "@{ov.tag_value}" };
                                                {#for field in &ov.fields}
                                                    {#if field.name != tag_field}
                                                        __result["@{field.name}"] = (value as any)["@{field.name}"] ?? null;
                                                    {/if}
                                                {/for}
                                                return __result as @{full_type_ident};
                                            }
                                        {/for}
                                        {#for iv in &intersection_variants}
                                            if (__typeName === "@{iv.tag_value}") {
                                                {#if iv.is_encodable}
                                                    {$let iv_deser_fn: Expr = ts_ident!(nested_decode_fn_name(&extract_base_type(&iv.type_ref))).into()}
                                                    const __inner = @{iv_deser_fn}(value, ctx);
                                                    return { ...(__inner as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {:else}
                                                    return { ...(value as any), "@{tag_field}": "@{iv.tag_value}" } as @{full_type_ident};
                                                {/if}
                                            }
                                        {/for}
                                    }
                                }
                            {/if}

                            {#if !external_object_variants.is_empty() && is_externally_tagged && !has_encodables}
                                // Externally tagged anonymous-variant union: { "TagName": payload }
                                // (Distinct from the has_encodables case above so types like
                                //  RecurrenceEnd / OrderStage that mix literals + anonymous
                                //  variants still get a decoder body.)
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        const __inner = (value as any)[__variantName];
                                        {#for ov in &external_object_variants}
                                            if (__variantName === "@{ov.name}") {
                                                {#if let Some(ref deser_inline) = ov.inner_foreign_decode_inline}
                                                    {$let foreign_deser_expr: Expr = Expr::parse(deser_inline).expect("inner foreign decode expr should parse")}
                                                    return ({ "@{ov.name}": (@{foreign_deser_expr})(__inner) }) as @{full_type_ident};
                                                {:else}
                                                    {#if let Some(ref payload_deser_fn) = ov.payload_decode_fn}
                                                        return ({ "@{ov.name}": @{payload_deser_fn}(__inner ?? {}, ctx) }) as @{full_type_ident};
                                                    {:else}
                                                        return ({ "@{ov.name}": __inner }) as @{full_type_ident};
                                                    {/if}
                                                {/if}
                                            }
                                        {/for}
                                    }
                                }
                            {/if}

                            {#for uv in &untagged_object_variants}
                                if (@{uv.shape_condition}) {
                                    const __result: Record<string, unknown> = {};
                                    {#for name in &uv.required_fields}
                                        __result["@{name}"] = (value as any)["@{name}"];
                                    {/for}
                                    {#for name in &uv.optional_fields}
                                        if ("@{name}" in value) __result["@{name}"] = (value as any)["@{name}"];
                                    {/for}
                                    return __result as @{full_type_ident};
                                }
                            {/for}

                            throw new @{decode_error_expr}([{
                                field: "_root",
                                message: "@{type_name}.decodeWithContext: value does not match any union member"
                            }]);
        {/if}
        }

        export function @{fn_has_shape_ident}(value: unknown): boolean {
                        {#if is_literal_only}
                            const allowedValues = [@{literals.join(", ")}] as const;
                            return allowedValues.includes(value as any);
                        {:else if is_primitive_only}
                            return @{primitive_check_condition};
                        {:else if is_encodable_only}
                            // Foreign types with hasShape may not be objects: check first
                            {#for type_ref in &foreign_encodables}
                                {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                    {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                    if ((@{foreign_shape_expr})(value)) return true;
                                {/if}
                            {/for}

                            {#if is_externally_tagged}
                                // Externally tagged: check if object has a key matching a variant name
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        {#for ov in &external_object_variants}
                                        if (__variantName === "@{ov.name}") return true;
                                    {/for}
                                    if (@{encodable_type_check_condition.replace("__typeName", "__variantName")}) return true;
                                    }
                                }
                                if (typeof value === "string") {
                                    if (@{encodable_type_check_condition.replace("__typeName", "value")}) return true;
                                }
                                return false;
                            {:else if is_adjacently_tagged}
                                // Adjacently tagged: check for tag and content fields
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string" && "@{content_field}" in (value as any)) {
                                        return @{encodable_type_check_condition};
                                    }
                                }
                                return false;
                            {:else if is_untagged}
                                // Untagged: shape matching only
                                let __matchCount = 0;
                                {#for type_ref in &regular_encodables}
                                    {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                    if (@{has_shape_fn}(value)) __matchCount++;
                                {/for}
                                return __matchCount >= 1;
                            {:else}
                                // Internally tagged (default): tag-based + shape matching
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    if (typeof __typeName === "string") {
                                        return @{encodable_type_check_condition};
                                    }
                                }

                                // Shape matching works for any value type (including non-objects)
                                let __matchCount = 0;
                                {#for type_ref in &regular_encodables}
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
                            {#if has_encodables}
                                {#if is_externally_tagged}
                                    // Externally tagged: check if object has a key matching a variant name
                                    if (typeof value === "object" && value !== null) {
                                        const __keys = Object.keys(value);
                                        const __variantName = __keys[0];
                                        if (__variantName !== undefined) {
                                            {#for ov in &external_object_variants}
                                        if (__variantName === "@{ov.name}") return true;
                                    {/for}
                                    if (@{encodable_type_check_condition.replace("__typeName", "__variantName")}) return true;
                                        }
                                    }
                                    if (typeof value === "string") {
                                        if (@{encodable_type_check_condition.replace("__typeName", "value")}) return true;
                                    }
                                {:else if is_adjacently_tagged}
                                    // Adjacently tagged: check for tag and content fields
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string" && "@{content_field}" in (value as any)) {
                                            if (@{encodable_type_check_condition}) return true;
                                        }
                                    }
                                {:else if is_untagged}
                                    // Untagged: shape matching only
                                    let __matchCount = 0;
                                    {#for type_ref in &regular_encodables}
                                        {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                        if (@{has_shape_fn}(value)) __matchCount++;
                                    {/for}
                                    if (__matchCount >= 1) return true;
                                {:else}
                                    // Internally tagged (default)
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any)["@{tag_field}"];
                                        if (typeof __typeName === "string") {
                                            if (@{encodable_type_check_condition}) return true;
                                        } else {
                                            let __matchCount = 0;
                                            {#for type_ref in &regular_encodables}
                                                {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                if (@{has_shape_fn}(value)) __matchCount++;
                                            {/for}
                                            if (__matchCount === 1) return true;
                                        }
                                    } else {
                                        // Non-object values: regular encodables may still match (e.g. RecordLink can be a string)
                                        let __matchCount = 0;
                                        {#for type_ref in &regular_encodables}
                                            {$let has_shape_fn: Expr = ts_ident!(nested_has_shape_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (@{has_shape_fn}(value)) __matchCount++;
                                        {/for}
                                        if (__matchCount === 1) return true;
                                    }
                                {/if}
                                // Foreign types with hasShape may not be objects
                                {#for type_ref in &foreign_encodables}
                                    {#if let Some(ref shape_inline) = type_ref.foreign_has_shape_inline}
                                        {$let foreign_shape_expr: Expr = Expr::parse(shape_inline).expect("foreign hasShape expr should parse")}
                                        if ((@{foreign_shape_expr})(value)) return true;
                                    {/if}
                                {/for}
                            {/if}
                            {#if has_object_variants || has_intersection_variants}
                                if (typeof value === "object" && value !== null) {
                                    const __typeName = (value as any)["@{tag_field}"];
                                    {$let all_tag_values: Vec<String> = object_variants.iter().map(|ov| format!("\"{}\"", ov.tag_value)).chain(intersection_variants.iter().map(|iv| format!("\"{}\"", iv.tag_value))).collect()}
                                    if ([@{all_tag_values.join(", ")}].includes(__typeName)) return true;
                                }
                            {/if}
                            {#if !external_object_variants.is_empty() && is_externally_tagged && !has_encodables}
                                if (typeof value === "object" && value !== null) {
                                    const __keys = Object.keys(value);
                                    const __variantName = __keys[0];
                                    if (__variantName !== undefined) {
                                        {$let all_variant_names: Vec<String> = external_object_variants.iter().map(|ov| format!("\"{}\"", ov.name)).collect()}
                                        if ([@{all_variant_names.join(", ")}].includes(__variantName)) return true;
                                    }
                                }
                            {/if}
                            {#for uv in &untagged_object_variants}
                                if (@{uv.shape_condition}) return true;
                            {/for}
                            {#if has_generic_params}
                                return true;
                            {:else}
                                return false;
                            {/if}
        {/if}
        }

        export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
            {#if has_arm_validators}
                return @{fn_has_shape_expr}(value) && @{fn_decode_expr}(value).success;
            {:else}
                return @{fn_has_shape_expr}(value);
            {/if}
        }
    };
    let mut result = result.merge(per_variant_is_stream);
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);
    Ok(result)
}

/// Decode for an alias of a primitive, optionally symbol-branded
/// (`type Meters = number & { readonly [B]: true }`). The base primitive is
/// checked and alias-level validators run before the value is branded. A base
/// with a wire form in the foreign-type table (`bigint` travels as a string)
/// is converted from it exactly as a field of that type would be.
fn handle_primitive_type_alias(
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
        validate_field_generic_decl,
        type_ident,
        ..
    } = alias;
    let camel = type_name.to_case(Case::Camel);
    let fn_decode_ident = ts_ident!("{}Decode{}", camel, generic_decl);
    let fn_decode_expr: Expr = ts_ident!("{}Decode", camel).into();
    let fn_decode_internal_ident = ts_ident!("{}DecodeWithContext{}", camel, generic_args);
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
    let fn_validate_field_ident =
        ts_ident!("{}ValidateField{}", camel, validate_field_generic_decl);
    let fn_validate_fields_ident = ts_ident!("{}ValidateFields", camel);
    let fn_is_ident = ts_ident!("{}Is{}", camel, generic_decl);
    let fn_has_shape_ident = ts_ident!("{}HasShape{}", camel, generic_decl);
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
            .and_then(|foreign| foreign.decode_expr.as_deref())
            .map(|expr| {
                Expr::parse(&rewrite_expression_namespaces(expr))
                    .expect("foreign decode expression should parse")
            });
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
                        __decoded = (@{wire_decode})(value);
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

        export function @{fn_validate_field_ident}(
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

fn handle_fallback_type_alias(alias: &AliasDecode) -> Result<TsStream, MacroforgeError> {
    let AliasDecode {
        type_name,
        type_ident,
        decode_context_ident,
        decode_context_expr,
        decode_error_expr,
        decode_options_ident,
        generic_decl,
        generic_args,
        full_type_name,
        validate_field_generic_decl,
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
    let fn_decode_internal_expr: Expr = fn_decode_internal_ident.clone().into();
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

        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: any, ctx: @{decode_context_ident}): @{full_type_ident} {
            if (value?.__ref !== undefined) {
                return ctx.getOrDefer(value.__ref) as @{full_type_ident};
            }
            return value as @{type_ident};
        }

        export function @{fn_validate_field_ident}(
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
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    Ok(result)
}
