use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, ts_ident};

use super::super::EndecContainerOptions;
use super::field::prepare::to_decode_field;
use super::field::statements::decode_fields;
use super::reference::decode_reference;
use super::types::DecodeField;
use super::validation::generate_field_validations;
use crate::builtin::derive::common::{TypeNames, js_string, rendered};
use crate::builtin::return_types::{
    DECODE_CONTEXT, DECODE_ERROR, DECODE_OPTIONS, PENDING_REF, decode_return_type, is_ok_check,
    root_forward_reference_error, wrap_error, wrap_success,
};

pub(super) fn handle_class(
    input: &DeriveInput,
    class: &crate::ts_syn::DataClass,
) -> Result<TsStream, MacroforgeError> {
    let class_name = input.name();
    let class_names = TypeNames::new(class_name, class.type_params());
    let class_ident = class_names.full_type_ident();
    let class_expr: Expr = ts_ident!(class_name).into();
    let generic_args = class_names.generic_args.as_str();
    let key_param = class_names.key_param();
    let decode_context_ident = ts_ident!(DECODE_CONTEXT);
    let decode_context_expr: Expr = decode_context_ident.clone().into();
    let decode_error_expr: Expr = ts_ident!(DECODE_ERROR).into();
    let pending_ref_ident = ts_ident!(PENDING_REF);
    let pending_ref_expr: Expr = pending_ref_ident.clone().into();
    let decode_options_ident = ts_ident!(DECODE_OPTIONS);
    let container_opts = EndecContainerOptions::from_decorators(&class.inner.decorators);
    let tag_field = container_opts.tag_field_or_default();

    // Generate function names (always prefix style)
    let fn_decode_ident = class_names.generic_function("Decode");
    let fn_decode_internal_ident = class_names.generic_function("DecodeWithContext");
    let fn_is_ident = class_names.generic_function("Is");

    // Check for user-defined constructor with parameters
    if let Some(ctor) = class.method("constructor")
        && !ctor.params_src.trim().is_empty()
    {
        return Err(MacroforgeError::new(
            ctor.span,
            format!(
                "@Derive(Decode) cannot be used on class '{}' with a custom constructor. \
                    Remove the constructor or use @Derive(Decode) on a class without a constructor.",
                class_name
            ),
        ));
    }

    // Collect decodable fields with diagnostic collection
    let type_registry = &input.context.type_registry;
    let caller_file_path = input.context.file_name.as_str();
    let file_imports = input.context.import_registry.file_import_entries();
    let file_imports = file_imports.as_slice();
    let type_params = class_names.params.clone();
    let mut all_diagnostics = DiagnosticCollector::new();
    let fields: Vec<DecodeField> = class
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
                &type_params,
            )
        })
        .collect();

    // Check for errors in field parsing before continuing
    if all_diagnostics.has_errors() {
        return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
    }

    // Separate required vs optional fields
    let required_fields: Vec<_> = fields
        .iter()
        .filter(|f| !f.optional && !f.flatten)
        .cloned()
        .collect();

    // Build known keys for deny_unknown_fields
    let known_keys: Vec<String> = fields
        .iter()
        .filter(|f| !f.flatten)
        .map(|f| f.json_key.clone())
        .collect();

    // All non-flatten fields for assignments
    let all_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();

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
            .map(|f| {
                let key = js_string(&f.json_key);
                rendered(ts_template! { @{key} in o })
            })
            .collect::<Vec<_>>()
            .join(" && ")
    };

    // Compute return type and wrappers
    let return_type = decode_return_type(&class_names.full_type);
    let return_type_ident = ts_ident!(return_type.as_str());
    let success_result = wrap_success("resultOrRef");
    let success_result_expr =
        Expr::parse(&success_result).expect("decode success wrapper should parse");
    let error_root_ref = root_forward_reference_error(class_name);
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
    let known_keys_list: Vec<_> = known_keys.iter().map(|key| js_string(key)).collect();

    // Build typed constructor parameter: { field1: type1; field2?: type2; ... }
    let props_type_parts: Vec<String> = all_fields
        .iter()
        .map(|f| {
            if f.optional {
                rendered(ts_template! { @{&f.field_name}?: @{&f.ts_type} })
            } else {
                rendered(ts_template! { @{&f.field_name}: @{&f.ts_type} })
            }
        })
        .collect();
    let props_type_parts = props_type_parts.join("; ");
    let props_type = rendered(ts_template! { { @{props_type_parts} } });
    let props_type_ident = ts_ident!(props_type.as_str());

    let field_statements = decode_fields(&fields, class_name);
    let reference = decode_reference(
        class_name,
        &rendered(ts_template! { @{&class_names.full_type} | @{PENDING_REF} }),
    );

    let mut result = ts_template!(Within {
        constructor(props: @{props_type_ident}) {
            {#for field in &all_fields}
                this.@{field.field_ident} = props.@{field.field_ident};
            {/for}
        }

        /** Decodes input to an instance of this class. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to decode @param opts - Optional decoding options @returns Result containing the decoded instance or validation errors */
        static @{class_names.generic_method("decode")}(input: unknown, opts?: @{decode_options_ident}): @{return_type_ident} {
            try {
                // Auto-detect: if string, parse as JSON first
                const data = typeof input === "string" ? JSON.parse(input) : input;

                const ctx = @{decode_context_expr}.create();
                const resultOrRef = @{&class_expr}.@{ts_ident!(format!("decodeWithContext{generic_args}"))}(data, ctx);

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
        static @{class_names.generic_method("decodeWithContext")}(value: unknown, ctx: @{decode_context_ident}): @{class_ident} | @{pending_ref_ident} {
            {$typescript reference}

            if (typeof value !== "object" || value === null || Array.isArray(value)) {
                throw new @{decode_error_expr}([{ field: "_root", message: "@{class_name}.decodeWithContext: expected an object" }]);
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

            // Create instance using Object.create to avoid constructor
            const instance = Object.create(@{&class_expr}.prototype) as @{class_ident};

            // Register with context if __id is present
            if (obj.__id !== undefined) {
                ctx.register(obj.__id as number, instance);
            }

            // Track for optional freezing
            ctx.trackForFreeze(instance);

            {$typescript field_statements}

            ctx.pushErrors(errors);

            return instance;
        }

        {#if fields_with_validators.is_empty()}
            static @{ts_ident!(format!("validateField{}", key_param.decl))}(field: @{key_param.name.clone()}, value: unknown): Array<{ field: string; message: string }>;
            static validateField(): Array<{ field: string; message: string }> {
                return [];
            }

            static @{class_names.generic_method("validateFields")}(partial: { readonly [@{key_param.name.clone()} in keyof @{class_ident.clone()}]?: unknown }): Array<{ field: string; message: string }>;
            static validateFields(): Array<{ field: string; message: string }> {
                return [];
            }
        {:else}
            static @{ts_ident!(format!("validateField{}", key_param.decl))}(field: @{key_param.name.clone()}, value: unknown): Array<{ field: string; message: string }> {
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &fields_with_validators}
                    if (field === "@{field.field_name}") {
                        const __val = value as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name, field.missing())}
                        {$typescript validation_code}
                    }
                {/for}
                return errors;
            }

            static @{class_names.generic_method("validateFields")}(partial: { readonly [@{key_param.name.clone()} in keyof @{class_ident.clone()}]?: unknown }): Array<{ field: string; message: string }> {
                const errors: Array<{ field: string; message: string }> = [];
                {#for field in &fields_with_validators}
                    if ("@{field.field_name}" in partial && partial.@{field.field_ident} !== undefined) {
                        const __val = partial.@{field.field_ident} as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name, field.missing())}
                        {$typescript validation_code}
                    }
                {/for}
                return errors;
            }
        {/if}

        static hasShape(obj: unknown): boolean {
            if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                return false;
            }
            const o = obj as Record<string, unknown>;
            return @{shape_check_condition};
        }

        static @{class_names.generic_method("is")}(obj: unknown): obj is @{class_ident} {
            if (obj instanceof @{&class_expr}) {
                return true;
            }
            if (!@{&class_expr}.hasShape(obj)) {
                return false;
            }
            const result = @{&class_expr}.decode(obj);
            return @{Expr::parse(&is_ok_check("result")).expect("decode is_ok expression should parse")};
        }
    });
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);

    // Generate standalone functions that delegate to static methods
    let mut standalone = ts_template! {
        /** Decodes input to an instance. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to decode @param opts - Optional decoding options @returns Result containing the decoded instance or validation errors */
        export function @{fn_decode_ident}(input: unknown, opts?: @{decode_options_ident}): @{return_type_ident} {
            return @{&class_expr}.@{ts_ident!(format!("decode{generic_args}"))}(input, opts);
        }

        /** Decodes with an existing context for nested/cyclic object graphs. @param value - The raw value to decode @param ctx - The decoding context */
        export function @{fn_decode_internal_ident}(value: unknown, ctx: @{decode_context_ident}): @{class_ident} | @{pending_ref_ident} {
            return @{&class_expr}.@{ts_ident!(format!("decodeWithContext{generic_args}"))}(value, ctx);
        }

        /** Type guard: checks if a value can be successfully decoded. @param value - The value to check @returns True if the value can be decoded to this type */
        export function @{fn_is_ident}(value: unknown): value is @{class_ident} {
            return @{&class_expr}.@{ts_ident!(format!("is{generic_args}"))}(value);
        }
    };
    standalone.add_aliased_import("DecodeContext", crate::package::ENDEC);
    standalone.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    standalone.add_aliased_import("PendingRef", crate::package::ENDEC);

    // Combine standalone functions with class body using {$typescript} composition
    // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
    standalone.add_diagnostics(all_diagnostics.into_vec());
    Ok(ts_template! {
        {$typescript standalone}
        {$typescript result}
    })
}
