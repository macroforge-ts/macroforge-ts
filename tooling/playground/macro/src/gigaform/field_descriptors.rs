//! Generates the createForm factory function with reactive state and field controllers.

use convert_case::Casing;
use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::GenericInfo;
use crate::gigaform::form_data::extract_base_type;
use crate::gigaform::naming::{
    call_default_value, call_default_value_for_type_ref, call_deserialize, fn_name,
    type_name_prefixed,
};
use crate::gigaform::parser::{GigaformOptions, ParsedField, UnionConfig, UnionMode};

/// Generates the createForm factory function that returns a Gigaform instance.
pub fn generate_factory(
    interface_name: &str,
    fields: &[ParsedField],
    options: &GigaformOptions,
) -> TsStream {
    let create_fn_name = fn_name("createForm", interface_name, "");
    let errors_name = type_name_prefixed(interface_name, "Errors");
    let tainted_name = type_name_prefixed(interface_name, "Tainted");
    let field_controllers_name = type_name_prefixed(interface_name, "FieldControllers");
    let gigaform_name = type_name_prefixed(interface_name, "Gigaform");

    // Generate complete statements for state declarations
    let data_state = generate_data_state(interface_name, options, "", &errors_name, &tainted_name);
    let data_state_reset = generate_data_state_reset(interface_name, options, "");
    let errors_state_reset = generate_errors_state_reset(fields);
    let tainted_state_reset = generate_tainted_state_reset(fields);

    // Generate field controllers object
    let field_controllers_obj =
        generate_field_controllers_object(fields, options, interface_name, &field_controllers_name);

    let validate_call = call_deserialize(interface_name, "", "data");

    ts_template! {
        /** Creates a new Gigaform instance with reactive state and field controllers. */
        export function @{create_fn_name}(overrides?: Partial<@{interface_name}>): @{gigaform_name} {
            {$typescript data_state}
            {$typescript field_controllers_obj}

            // Validate the entire form using Deserialize's deserialize
            function validate(): Exit<@{interface_name}, Array<{ field: string; message: string }>> {
                return toExit(@{validate_call});
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}>): void {
                {$typescript data_state_reset}
                {$typescript errors_state_reset}
                {$typescript tainted_state_reset}
            }

            return {
                get data() { return data; },
                set data(v) { data = v; },
                get errors() { return errors; },
                set errors(v) { errors = v; },
                get tainted() { return tainted; },
                set tainted(v) { tainted = v; },
                fields,
                validate,
                reset,
            };
        }
    }
}

/// Generates the createForm factory with generic support.
pub fn generate_factory_with_generics(
    interface_name: &str,
    fields: &[ParsedField],
    options: &GigaformOptions,
    generics: &GenericInfo,
) -> TsStream {
    if generics.is_empty() {
        return generate_factory(interface_name, fields, options);
    }

    let generic_decl = generics.decl();
    let generic_args = generics.args();
    let create_fn_name = fn_name("createForm", interface_name, &generic_decl);
    let errors_name = format!(
        "{}{}",
        type_name_prefixed(interface_name, "Errors"),
        generic_args
    );
    let tainted_name = format!(
        "{}{}",
        type_name_prefixed(interface_name, "Tainted"),
        generic_args
    );
    let field_controllers_name = format!(
        "{}{}",
        type_name_prefixed(interface_name, "FieldControllers"),
        generic_args
    );
    let gigaform_name = type_name_prefixed(interface_name, "Gigaform");

    // Generate complete statements for state declarations
    let data_state = generate_data_state(
        interface_name,
        options,
        &generic_args,
        &errors_name,
        &tainted_name,
    );
    let data_state_reset = generate_data_state_reset(interface_name, options, &generic_args);
    let errors_state_reset = generate_errors_state_reset(fields);
    let tainted_state_reset = generate_tainted_state_reset(fields);

    // Generate field controllers object
    let field_controllers_obj =
        generate_field_controllers_object(fields, options, interface_name, &field_controllers_name);

    let validate_call = call_deserialize(interface_name, &generic_args, "data");

    ts_template! {
        /** Creates a new Gigaform instance with reactive state and field controllers. */
        export function @{create_fn_name}(overrides?: Partial<@{interface_name}@{generic_args}>): @{gigaform_name}@{generic_args} {
            {$typescript data_state}
            {$typescript field_controllers_obj}

            // Validate the entire form using Deserialize's deserialize
            function validate(): Exit<@{interface_name}@{generic_args}, Array<{ field: string; message: string }>> {
                return toExit(@{validate_call});
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}@{generic_args}>): void {
                {$typescript data_state_reset}
                {$typescript errors_state_reset}
                {$typescript tainted_state_reset}
            }

            return {
                get data() { return data; },
                set data(v) { data = v; },
                get errors() { return errors; },
                set errors(v) { errors = v; },
                get tainted() { return tainted; },
                set tainted(v) { tainted = v; },
                fields,
                validate,
                reset,
            };
        }
    }
}

// =============================================================================
// Union Factory Generation
// =============================================================================

/// Generates the createForm factory for a discriminated union with generic support.
pub fn generate_union_factory_with_generics(
    type_name: &str,
    config: &UnionConfig,
    options: &GigaformOptions,
    _generics: &GenericInfo,
) -> TsStream {
    // Unions typically don't have type parameters
    generate_union_factory(type_name, config, options)
}

/// Generates the createForm factory for a discriminated union.
pub fn generate_union_factory(
    type_name: &str,
    config: &UnionConfig,
    options: &GigaformOptions,
) -> TsStream {
    let create_fn_name = fn_name("createForm", type_name, "");
    let errors_name = type_name_prefixed(type_name, "Errors");
    let tainted_name = type_name_prefixed(type_name, "Tainted");
    let gigaform_name = type_name_prefixed(type_name, "Gigaform");
    let variant_fields_name = type_name_prefixed(type_name, "VariantFields");
    let default_for_variant_fn = fn_name("getDefaultForVariant", type_name, "");

    let variant_literals = config
        .variants
        .iter()
        .map(|v| format!("\"{}\"", v.discriminant_value))
        .collect::<Vec<_>>()
        .join(" | ");

    let discriminant_field = match &config.mode {
        UnionMode::Tagged { field } => field.as_str(),
        UnionMode::Untagged => "_variant",
    };

    let first_variant = config
        .variants
        .first()
        .map(|v| v.discriminant_value.as_str())
        .unwrap_or("unknown");

    let _default_init = generate_default_init_expr(type_name, options, "");

    // Generate per-variant field controllers
    let variant_controllers_obj =
        generate_union_variant_controllers_object(config, options, type_name, &variant_fields_name);

    // Generate getDefaultForVariant function
    let default_for_variant = generate_default_for_variant(
        type_name,
        config,
        discriminant_field,
        &default_for_variant_fn,
    );

    // Generate initial variant detection based on union type
    let initial_variant_expr = if discriminant_field == "_value" {
        // Literal union: the value IS the variant
        format!(r#"(initial as {variant_literals}) ?? "{first_variant}""#)
    } else if discriminant_field == "_type" {
        // Type ref union: can't easily detect, use first variant
        format!(r#""{first_variant}""#)
    } else {
        // Object union with discriminant field
        format!(r#"(initial as any)?.{discriminant_field} ?? "{first_variant}""#)
    };

    // Generate reset logic based on union type
    let reset_data_expr = if discriminant_field == "_value" || discriminant_field == "_type" {
        // For literal/type ref unions, just use the default (can't spread primitives/objects)
        format!("overrides ? overrides as typeof data : {default_for_variant_fn}(currentVariant)")
    } else {
        // For object unions, can spread
        format!(
            "overrides ? {{ ...{default_for_variant_fn}(currentVariant), ...overrides }} : {default_for_variant_fn}(currentVariant)"
        )
    };

    let validate_call = call_deserialize(type_name, "", "data");

    ts_template! {
        /** Gets default value for a specific variant */
        {$typescript default_for_variant}

        /** Creates a new discriminated union Gigaform with variant switching */
        export function @{create_fn_name}(initial?: @{type_name}): @{gigaform_name} {
            // Detect initial variant from discriminant
            const initialVariant: @{variant_literals} = @{initial_variant_expr};

            // Reactive state using Svelte 5 $state
            let currentVariant = $state<@{variant_literals}>(initialVariant);
            let data = $state<@{type_name}>(initial ?? @{default_for_variant_fn}(initialVariant));
            let errors = $state<@{errors_name}>({} as @{errors_name});
            let tainted = $state<@{tainted_name}>({} as @{tainted_name});

            {$typescript variant_controllers_obj}

            // Switch to a different variant
            function switchVariant(variant: @{variant_literals}): void {
                currentVariant = variant;
                data = @{default_for_variant_fn}(variant);
                errors = {} as @{errors_name};
                tainted = {} as @{tainted_name};
            }

            // Validate the entire form using Deserialize's deserialize
            function validate(): Exit<@{type_name}, Array<{ field: string; message: string }>> {
                return toExit(@{validate_call});
            }

            // Reset form
            function reset(overrides?: Partial<@{type_name}>): void {
                data = @{reset_data_expr};
                errors = {} as @{errors_name};
                tainted = {} as @{tainted_name};
            }

            return {
                get currentVariant() { return currentVariant; },
                get data() { return data; },
                set data(v) { data = v; },
                get errors() { return errors; },
                set errors(v) { errors = v; },
                get tainted() { return tainted; },
                set tainted(v) { tainted = v; },
                variants,
                switchVariant,
                validate,
                reset,
            };
        }
    }
}

/// Generates the getDefaultForVariant function for union types.
fn generate_default_for_variant(
    type_name: &str,
    config: &UnionConfig,
    discriminant_field: &str,
    fn_name: &str,
) -> TsStream {
    let is_literal_union = discriminant_field == "_value";
    let is_type_ref_union = discriminant_field == "_type";

    // Generate if-statements for each variant (each returns, so no else needed)
    let variant_checks: Vec<TsStream> = config
        .variants
        .iter()
        .map(|variant| {
            let value = &variant.discriminant_value;
            let return_stmt = if is_literal_union {
                ts_template! { return "@{value}" as @{type_name}; }
            } else if is_type_ref_union {
                let default_expr = get_type_ref_default_with_style(value, type_name);
                ts_template! { return @{default_expr}; }
            } else {
                ts_template! { return { @{discriminant_field}: "@{value}" } as @{type_name}; }
            };
            ts_template! {
                if (variant === "@{value}") { {$typescript return_stmt} }
            }
        })
        .collect();

    // Default fallback uses first variant
    let first_value = config
        .variants
        .first()
        .map(|v| v.discriminant_value.as_str())
        .unwrap_or("unknown");
    let default_return = if is_literal_union {
        ts_template! { return "@{first_value}" as @{type_name}; }
    } else if is_type_ref_union {
        let default_expr = get_type_ref_default_with_style(first_value, type_name);
        ts_template! { return @{default_expr}; }
    } else {
        ts_template! { return { @{discriminant_field}: "@{first_value}" } as @{type_name}; }
    };

    let variant_checks = TsStream::merge_all(variant_checks);

    ts_template! {
        function @{fn_name}(variant: string): @{type_name} {
            {$typescript variant_checks}
            {$typescript default_return}
        }
    }
}

/// Generates per-variant field controllers as a complete const declaration.
fn generate_union_variant_controllers_object(
    config: &UnionConfig,
    _options: &GigaformOptions,
    type_name: &str,
    variant_fields_name: &str,
) -> TsStream {
    ts_template! {
        const variants = {} as @{variant_fields_name};
        {#for variant in &config.variants}
            {$let value = &variant.discriminant_value}
            {$let variant_name = to_pascal_case(value)}
            {$let prop_key = if needs_quoting(value) { format!("\"{}\"", value) } else { value.clone() }}
            {$let fields_type = format!("{}{}FieldControllers", type_name, variant_name)}
            {$let field_assignments = generate_variant_field_assignments(&variant.fields, type_name, &prop_key)}
            variants[@{prop_key}] = { fields: {} as @{fields_type} };
            {$typescript field_assignments}
        {/for}
    }
}

/// Generates field assignment statements for a variant.
fn generate_variant_field_assignments(
    fields: &[ParsedField],
    type_name: &str,
    variant_key: &str,
) -> TsStream {
    ts_template! {
        {#for field in fields}
            {$let assignment = generate_field_controller_assignment(field, type_name, variant_key)}
            {$typescript assignment}
        {/for}
    }
}

/// Generates a single field controller assignment statement.
fn generate_field_controller_assignment(
    field: &ParsedField,
    interface_name: &str,
    variant_key: &str,
) -> TsStream {
    let name = &field.name;
    let label = name.as_str(); // Use field name as label
    let optional = field.optional;
    let is_array = field.is_array;
    let controller_type = get_field_controller_type(field, interface_name);

    ts_template! {
        variants[@{variant_key}].fields.@{name} = {
            label: "@{label}",
            type: "@{controller_type}",
            optional: @{optional},
            array: @{is_array}
        };
    }
}

/// Determines the controller type for a field.
fn get_field_controller_type(field: &ParsedField, _interface_name: &str) -> String {
    let ts_type = field.ts_type.trim();
    let base_type = extract_base_type(ts_type);

    match base_type {
        "string" => "text".to_string(),
        "number" => "number".to_string(),
        "boolean" => "checkbox".to_string(),
        "Date" => "date".to_string(),
        _ => "text".to_string(),
    }
}

/// Returns true if a string needs to be quoted when used as an object property key.
fn needs_quoting(s: &str) -> bool {
    // Needs quoting if it contains special chars or starts with a digit
    s.chars().any(|c| !c.is_alphanumeric() && c != '_')
        || s.chars().next().map(|c| c.is_numeric()).unwrap_or(true)
}

/// Converts a string to a valid PascalCase TypeScript identifier.
/// Handles special characters like `|`, `(`, `)`, `&`, etc.
/// Uses "Or" for `|` and "And" for `&` to make identifiers more readable.
fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '|' {
            // Use "Or" for union types
            result.push_str("Or");
            capitalize_next = true;
        } else if c == '&' {
            // Use "And" for intersection types
            result.push_str("And");
            capitalize_next = true;
        } else if c == '_'
            || c == '-'
            || c == ' '
            || c == '('
            || c == ')'
            || c == '<'
            || c == '>'
            || c == ','
        {
            // Skip these characters but capitalize the next letter
            capitalize_next = true;
        } else if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        // Skip any other non-alphanumeric characters
    }

    // Ensure the result is a valid identifier (starts with letter or underscore)
    if result.is_empty()
        || result
            .chars()
            .next()
            .map(|c| c.is_numeric())
            .unwrap_or(false)
    {
        format!("Variant{}", result)
    } else {
        result
    }
}

fn get_type_ref_default_with_style(type_ref: &str, cast_type: &str) -> String {
    let tr = type_ref.trim();
    match tr {
        "number" => format!("0 as {cast_type}"),
        "string" => format!("\"\" as {cast_type}"),
        "boolean" => format!("false as {cast_type}"),
        "bigint" => format!("0n as {cast_type}"),
        "undefined" => format!("undefined as {cast_type}"),
        "null" => format!("null as {cast_type}"),
        "symbol" => format!("Symbol() as {cast_type}"),
        "object" => format!("{{}} as {cast_type}"),
        "any" | "unknown" => format!("undefined as {cast_type}"),
        "void" | "never" => format!("undefined as {cast_type}"),
        _ => format!("{} as {cast_type}", call_default_value_for_type_ref(tr)),
    }
}

// =============================================================================
// State Declaration Generators (Complete Statements)
// =============================================================================

/// Generates complete state declarations: data, errors, tainted
fn generate_data_state(
    interface_name: &str,
    options: &GigaformOptions,
    generic_args: &str,
    errors_name: &str,
    tainted_name: &str,
) -> TsStream {
    let default_expr = call_default_value(interface_name, generic_args);
    if let Some(override_fn) = &options.default_override {
        ts_template! {
            let data = $state({ ...@{default_expr}, ...@{override_fn}(), ...overrides });
            let errors = $state<@{errors_name}>({ _errors: __gigaform_reexport_Option.none() } as @{errors_name});
            let tainted = $state<@{tainted_name}>({} as @{tainted_name});
        }
    } else {
        ts_template! {
            let data = $state({ ...@{default_expr}, ...overrides });
            let errors = $state<@{errors_name}>({ _errors: __gigaform_reexport_Option.none() } as @{errors_name});
            let tainted = $state<@{tainted_name}>({} as @{tainted_name});
        }
    }
}

/// Generates the data reset statement
fn generate_data_state_reset(
    interface_name: &str,
    options: &GigaformOptions,
    generic_args: &str,
) -> TsStream {
    let default_expr = call_default_value(interface_name, generic_args);
    if let Some(override_fn) = &options.default_override {
        ts_template! {
            data = { ...@{default_expr}, ...@{override_fn}(), ...newOverrides };
        }
    } else {
        ts_template! {
            data = { ...@{default_expr}, ...newOverrides };
        }
    }
}

/// Generates the errors reset statement
fn generate_errors_state_reset(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        errors = {
            _errors: __gigaform_reexport_Option.none(),
            {#for field in fields}
                @{&field.name}: __gigaform_reexport_Option.none(),
            {/for}
        };
    }
}

/// Generates the tainted reset statement
fn generate_tainted_state_reset(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        tainted = {
            {#for field in fields}
                @{&field.name}: __gigaform_reexport_Option.none(),
            {/for}
        };
    }
}

/// Generates the default init expression (for internal use)
fn generate_default_init_expr(
    interface_name: &str,
    options: &GigaformOptions,
    generic_args: &str,
) -> String {
    let default_expr = call_default_value(interface_name, generic_args);
    if let Some(override_fn) = &options.default_override {
        format!("{{ ...{}, ...{}() }}", default_expr, override_fn)
    } else {
        format!("{{ ...{} }}", default_expr)
    }
}

// =============================================================================
// Field Controllers Generators
// =============================================================================

/// Generates field controllers as a complete const declaration.
fn generate_field_controllers_object(
    fields: &[ParsedField],
    _options: &GigaformOptions,
    interface_name: &str,
    field_controllers_name: &str,
) -> TsStream {
    ts_template! {
        const fields = {} as @{field_controllers_name};
        {#for field in fields}
            {$let assignment = generate_simple_field_assignment(field, interface_name)}
            {$typescript assignment}
        {/for}
    }
}

/// Generates a simple field controller assignment.
fn generate_simple_field_assignment(field: &ParsedField, interface_name: &str) -> TsStream {
    let name = &field.name;
    let label = name.as_str(); // Use field name as label
    let optional = field.optional;
    let is_array = field.is_array;
    let controller_type = get_field_controller_type(field, interface_name);

    ts_template! {
        fields.@{name} = {
            label: "@{label}",
            type: "@{controller_type}",
            optional: @{optional},
            array: @{is_array}
        };
    }
}

// =============================================================================
// Enum Factory Generation
// =============================================================================

use crate::gigaform::parser::EnumFormConfig;

/// Generates the createForm factory for an enum step form.
/// Note: Uses prefix-style naming.
pub fn generate_enum_factory(enum_name: &str, config: &EnumFormConfig) -> TsStream {
    // Use prefix-style naming (the new default)
    let create_fn_name = format!("{}CreateForm", enum_name.to_case(convert_case::Case::Camel));
    let gigaform_name = type_name_prefixed(enum_name, "Gigaform");
    let variants_name = type_name_prefixed(enum_name, "Variants");

    let first_variant = config
        .variants
        .first()
        .map(|v| v.value_expr.as_str())
        .unwrap_or("0");

    ts_template! {
        /** Creates a new enum step form with navigation controls */
        export function @{create_fn_name}(initialStep?: @{enum_name}): @{gigaform_name} {
            let currentStep = $state<@{enum_name}>(initialStep ?? @{first_variant});

            return {
                get currentStep() { return currentStep; },
                steps: @{variants_name},
                setStep(step: @{enum_name}): void {
                    currentStep = step;
                },
                nextStep(): boolean {
                    const idx = @{variants_name}.findIndex(v => v.value === currentStep);
                    if (idx < @{variants_name}.length - 1) {
                        currentStep = @{variants_name}[idx + 1].value;
                        return true;
                    }
                    return false;
                },
                prevStep(): boolean {
                    const idx = @{variants_name}.findIndex(v => v.value === currentStep);
                    if (idx > 0) {
                        currentStep = @{variants_name}[idx - 1].value;
                        return true;
                    }
                    return false;
                },
            };
        }
    }
}
