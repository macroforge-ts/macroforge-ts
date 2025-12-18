//! Generates the createForm factory function with reactive state and field controllers.

use convert_case::Casing;
use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::GenericInfo;
use crate::gigaform::naming::{
    call_default_value, call_default_value_for_type_ref, call_deserialize, call_validate_field,
    fn_name, type_name_prefixed,
};
use crate::gigaform::parser::{
    BaseControllerOptions, ControllerOptions, GigaformOptions, ParsedField, UnionConfig, UnionMode,
    ValidatorSpec,
};

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

    let field_controllers = generate_field_controllers(fields, options, interface_name);
    let default_init = generate_default_init(interface_name, options, "");
    let default_errors_init = generate_default_errors_init(fields);
    let default_tainted_init = generate_default_tainted_init(fields);
    // Generate again for reset function (TsStream doesn't implement Copy)
    let default_init_reset = generate_default_init(interface_name, options, "");
    let default_errors_init_reset = generate_default_errors_init(fields);
    let default_tainted_init_reset = generate_default_tainted_init(fields);
    let validate_call = call_deserialize(interface_name, "", "data");

    ts_template! {
        {>> "Creates a new Gigaform instance with reactive state and field controllers." <<}
        export function @{create_fn_name}(overrides?: Partial<@{interface_name}>): @{gigaform_name} {
            // Reactive state using Svelte 5 $state
            let data = $state({ {$typescript default_init}, ...overrides });
            let errors = $state<@{errors_name}>({$typescript default_errors_init});
            let tainted = $state<@{tainted_name}>({$typescript default_tainted_init});

            // Field controllers with closures capturing reactive state
            const fields: @{field_controllers_name} = {
                {$typescript field_controllers}
            };

            // Validate the entire form using Deserialize's deserialize
            function validate(): Result<@{interface_name}, Array<{ field: string; message: string }>> {
                return @{validate_call};
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}>): void {
                data = { {$typescript default_init_reset}, ...newOverrides };
                errors = {$typescript default_errors_init_reset};
                tainted = {$typescript default_tainted_init_reset};
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
    let create_fn_name = fn_name("createForm", interface_name, &generic_decl);
    let errors_name = type_name_prefixed(interface_name, "Errors");
    let tainted_name = type_name_prefixed(interface_name, "Tainted");
    let field_controllers_name = type_name_prefixed(interface_name, "FieldControllers");
    let gigaform_name = type_name_prefixed(interface_name, "Gigaform");

    let field_controllers = generate_field_controllers(fields, options, interface_name);
    let default_errors_init = generate_default_errors_init(fields);
    let default_tainted_init = generate_default_tainted_init(fields);
    let generic_args = generics.args();
    let default_init = generate_default_init(interface_name, options, &generic_args);
    // Generate again for reset function (TsStream doesn't implement Copy)
    let default_init_reset = generate_default_init(interface_name, options, &generic_args);
    let default_errors_init_reset = generate_default_errors_init(fields);
    let default_tainted_init_reset = generate_default_tainted_init(fields);
    let validate_call = call_deserialize(interface_name, &generic_args, "data");

    ts_template! {
        {>> "Creates a new Gigaform instance with reactive state and field controllers." <<}
        export function @{create_fn_name}(overrides?: Partial<@{interface_name}@{generic_args}>): @{gigaform_name}@{generic_args} {
            // Reactive state using Svelte 5 $state
            let data = $state({ {$typescript default_init}, ...overrides });
            let errors = $state<@{errors_name}@{generic_args}>({$typescript default_errors_init});
            let tainted = $state<@{tainted_name}@{generic_args}>({$typescript default_tainted_init});

            // Field controllers with closures capturing reactive state
            const fields: @{field_controllers_name}@{generic_args} = {
                {$typescript field_controllers}
            };

            // Validate the entire form using Deserialize's deserialize
            function validate(): Result<@{interface_name}@{generic_args}, Array<{ field: string; message: string }>> {
                return @{validate_call};
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}@{generic_args}>): void {
                data = { {$typescript default_init_reset}, ...newOverrides };
                errors = {$typescript default_errors_init_reset};
                tainted = {$typescript default_tainted_init_reset};
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

    let _default_init = generate_default_init(type_name, options, "");

    // Generate per-variant field controllers
    let variant_controllers = generate_union_variant_controllers(config, options, type_name);

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
        {>> "Gets default value for a specific variant" <<}
        {$typescript default_for_variant}

        {>> "Creates a new discriminated union Gigaform with variant switching" <<}
        export function @{create_fn_name}(initial?: @{type_name}): @{gigaform_name} {
            // Detect initial variant from discriminant
            const initialVariant: @{variant_literals} = @{initial_variant_expr};

            // Reactive state using Svelte 5 $state
            let currentVariant = $state<@{variant_literals}>(initialVariant);
            let data = $state<@{type_name}>(initial ?? @{default_for_variant_fn}(initialVariant));
            let errors = $state<@{errors_name}>({} as @{errors_name});
            let tainted = $state<@{tainted_name}>({} as @{tainted_name});

            // Per-variant field controllers
            const variants: @{variant_fields_name} = {
                {$typescript variant_controllers}
            };

            // Switch to a different variant
            function switchVariant(variant: @{variant_literals}): void {
                currentVariant = variant;
                data = @{default_for_variant_fn}(variant);
                errors = {} as @{errors_name};
                tainted = {} as @{tainted_name};
            }

            // Validate the entire form using Deserialize's deserialize
            function validate(): Result<@{type_name}, Array<{ field: string; message: string }>> {
                return @{validate_call};
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

/// Returns the default value expression for a type ref in a union.
/// For primitive types, returns the primitive default (0, "", false, etc.)
/// For named types, returns TypeName.defaultValue()
/// Generates the getDefaultForVariant function.
fn generate_default_for_variant(
    type_name: &str,
    config: &UnionConfig,
    discriminant_field: &str,
    fn_name: &str,
) -> TsStream {
    // Determine how to generate the default value based on the discriminant field
    // - "_value": literal union (e.g., "Home" | "About") - return the literal
    // - "_type": type ref union (e.g., DailyRule | WeeklyRule) - return TypeName.defaultValue()
    // - other: object union with tag field - return { field: "value" }
    let is_literal_union = discriminant_field == "_value";
    let is_type_ref_union = discriminant_field == "_type";

    let first_value = config
        .variants
        .first()
        .map(|v| v.discriminant_value.as_str())
        .unwrap_or("unknown");

    let default_return = if is_literal_union {
        format!(r#"return "{first_value}" as {type_name};"#)
    } else if is_type_ref_union {
        let default_expr = get_type_ref_default_with_style(first_value, type_name);
        format!(r#"return {default_expr};"#)
    } else {
        format!(r#"return {{ {discriminant_field}: "{first_value}" }} as {type_name};"#)
    };

    ts_template! {
        function @{fn_name}(variant: string): @{type_name} {
            switch (variant) {
                {#for variant in &config.variants}
                    {$let value = &variant.discriminant_value}
                    {#if is_literal_union}
                        case "@{value}": return "@{value}" as @{type_name};
                    {:else if is_type_ref_union}
                        {$let default_expr = get_type_ref_default_with_style(value, type_name)}
                        case "@{value}": return @{default_expr};
                    {:else}
                        case "@{value}": return { @{discriminant_field}: "@{value}" } as @{type_name};
                    {/if}
                {/for}
                default: @{default_return}
            }
        }
    }
}

/// Generates per-variant field controllers.
fn generate_union_variant_controllers(
    config: &UnionConfig,
    options: &GigaformOptions,
    type_name: &str,
) -> TsStream {
    ts_template! {
        {#for variant in &config.variants}
            {$let value = &variant.discriminant_value}
            {$let variant_name = to_pascal_case(&variant.discriminant_value)}
            {$let field_controllers = generate_field_controllers(&variant.fields, options, type_name)}
            {$let prop_key = if needs_quoting(value) { format!("\"{}\"", value) } else { value.clone() }}
            @{prop_key}: {
                fields: {
                    {$typescript field_controllers}
                } as {|@{type_name}@{variant_name}FieldControllers|}
            },
        {/for}
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
// Standard Field Factory Generation
// =============================================================================

/// Generates the default initialization expression.
fn generate_default_init(
    interface_name: &str,
    options: &GigaformOptions,
    generic_args: &str,
) -> TsStream {
    let default_expr = call_default_value(interface_name, generic_args);
    if let Some(override_fn) = &options.default_override {
        ts_template! { ...@{default_expr}, ...@{override_fn}() }
    } else {
        ts_template! { ...@{default_expr} }
    }
}

/// Generates the default errors initialization with all fields set to Option.none().
fn generate_default_errors_init(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        {
            _errors: Option.none(),
            {#for field in fields}
                @{&field.name}: Option.none(),
            {/for}
        }
    }
}

/// Generates the default tainted initialization with all fields set to Option.none().
fn generate_default_tainted_init(fields: &[ParsedField]) -> TsStream {
    ts_template! {
        {
            {#for field in fields}
                @{&field.name}: Option.none(),
            {/for}
        }
    }
}

/// Generates all field controller entries.
fn generate_field_controllers(
    fields: &[ParsedField],
    options: &GigaformOptions,
    interface_name: &str,
) -> TsStream {
    ts_template! {
        {#for field in fields}
            {$let controller = generate_field_controller(field, options, interface_name)}
            {$typescript controller}
        {/for}
    }
}

/// Generates a single field controller with closure-based accessors.
fn generate_field_controller(
    field: &ParsedField,
    _options: &GigaformOptions,
    interface_name: &str,
) -> TsStream {
    let name = &field.name;
    let ts_type = &field.ts_type;

    // Generate constraints from validators
    let constraints = generate_constraints(&field.validators, !field.optional);

    // Generate UI metadata from controller options
    let ui_metadata = generate_ui_metadata(field);

    // Generate the validate function that delegates to form validation
    let validate_fn = generate_field_validate_function(name, interface_name);

    // Generate the transform function
    let transform_fn = generate_transform_function(field, ts_type);

    // For array fields, add array-specific methods
    let array_methods = if field.is_array {
        Some(generate_array_methods(field, name))
    } else {
        None
    };

    ts_template! {
        @{name}: {
            path: ["@{name}"] as const,
            name: "@{name}",
            constraints: {$typescript constraints},
            {$typescript ui_metadata}
            get: () => data.@{name},
            set: (value: @{ts_type}) => { data.@{name} = value; },
            {$typescript transform_fn}
            getError: () => errors.@{name},
            setError: (value: Option<Array<string>>) => { errors.@{name} = value; },
            getTainted: () => tainted.@{name},
            setTainted: (value: Option<boolean>) => { tainted.@{name} = value; },
            {$typescript validate_fn}
            {#if let Some(methods) = array_methods}
                {$typescript methods}
            {/if}
        },
    }
}

/// Generates the transform function for a field based on controller options.
fn generate_transform_function(field: &ParsedField, ts_type: &str) -> TsStream {
    let default_base = BaseControllerOptions::default();
    let base = field
        .controller
        .as_ref()
        .map(|c| c.base())
        .unwrap_or(&default_base);

    // Check for transform option in base options
    if let Some(transform) = &base.transform {
        let transform_logic = match transform.as_str() {
            "uppercase" => "typeof value === 'string' ? value.toUpperCase() : value".to_string(),
            "lowercase" => "typeof value === 'string' ? value.toLowerCase() : value".to_string(),
            "trim" => "typeof value === 'string' ? value.trim() : value".to_string(),
            "trimUppercase" | "trim_uppercase" => {
                "typeof value === 'string' ? value.trim().toUpperCase() : value".to_string()
            }
            "trimLowercase" | "trim_lowercase" => {
                "typeof value === 'string' ? value.trim().toLowerCase() : value".to_string()
            }
            // Custom function name - call it directly
            custom_fn => format!("{custom_fn}(value)"),
        };
        return ts_template! {
            transform: (value: @{ts_type}): @{ts_type} => { return @{transform_logic}; },
        };
    }

    // Check for formatter in TextOptions (legacy support)
    if let Some(controller) = &field.controller
        && let ControllerOptions::Text(text_opts) | ControllerOptions::TextArea(text_opts) =
            &controller.options
        && let Some(formatter) = &text_opts.formatter
    {
        return ts_template! {
            transform: (value: @{ts_type}): @{ts_type} => { return @{formatter}(value); },
        };
    }

    // Default: identity transform
    ts_template! { transform: (value: @{ts_type}): @{ts_type} => value, }
}

/// Generates UI metadata properties from controller options.
fn generate_ui_metadata(field: &ParsedField) -> TsStream {
    let default_base = BaseControllerOptions::default();
    let base = field
        .controller
        .as_ref()
        .map(|c| c.base())
        .unwrap_or(&default_base);

    let label = base.get_label();
    let description = &base.description;
    let placeholder = &base.placeholder;
    let disabled = base.disabled;
    let readonly = base.readonly;

    ts_template! {
        {#if let Some(l) = label}
            label: "@{l}",
        {/if}
        {#if let Some(d) = description}
            description: "@{d}",
        {/if}
        {#if let Some(p) = placeholder}
            placeholder: "@{p}",
        {/if}
        {#if let Some(d) = disabled}
            disabled: @{d},
        {/if}
        {#if let Some(r) = readonly}
            readonly: @{r},
        {/if}
    }
}

/// Generates the constraints object from validators.
fn generate_constraints(validators: &[ValidatorSpec], required: bool) -> TsStream {
    // Build a list of constraint strings to join
    let mut constraints = Vec::new();

    if required {
        constraints.push("required: true".to_string());
    }

    for validator in validators {
        match validator.name.as_str() {
            "minLength" => {
                if let Some(n) = validator.args.first() {
                    constraints.push(format!("minlength: {n}"));
                }
            }
            "maxLength" => {
                if let Some(n) = validator.args.first() {
                    constraints.push(format!("maxlength: {n}"));
                }
            }
            "length" => {
                if validator.args.len() == 1 {
                    if let Some(n) = validator.args.first() {
                        constraints.push(format!("minlength: {n}"));
                        constraints.push(format!("maxlength: {n}"));
                    }
                } else if validator.args.len() >= 2 {
                    constraints.push(format!("minlength: {}", validator.args[0]));
                    constraints.push(format!("maxlength: {}", validator.args[1]));
                }
            }
            "pattern" => {
                if let Some(p) = validator.args.first() {
                    let escaped = p.replace('\\', "\\\\").replace('"', "\\\"");
                    constraints.push(format!("pattern: \"{escaped}\""));
                }
            }
            "greaterThanOrEqualTo" | "nonNegative" => {
                let min = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
                constraints.push(format!("min: {min}"));
            }
            "lessThanOrEqualTo" => {
                if let Some(n) = validator.args.first() {
                    constraints.push(format!("max: {n}"));
                }
            }
            "positive" => {
                constraints.push("min: 1".to_string());
            }
            "between" => {
                if validator.args.len() >= 2 {
                    constraints.push(format!("min: {}", validator.args[0]));
                    constraints.push(format!("max: {}", validator.args[1]));
                }
            }
            "multipleOf" => {
                if let Some(n) = validator.args.first() {
                    constraints.push(format!("step: {n}"));
                }
            }
            "int" => {
                constraints.push("step: 1".to_string());
            }
            "email" => {
                constraints.push("type: \"email\"".to_string());
            }
            "url" => {
                constraints.push("type: \"url\"".to_string());
            }
            _ => {}
        }
    }

    let constraints_str = if constraints.is_empty() {
        "{}".to_string()
    } else {
        format!("{{ {} }}", constraints.join(", "))
    };

    ts_template! { @{constraints_str} }
}

/// Generates the field-level validate function that uses per-field validation.
fn generate_field_validate_function(field_name: &str, interface_name: &str) -> TsStream {
    let validate_call = call_validate_field(
        interface_name,
        &format!("\"{field_name}\""),
        &format!("data.{field_name}"),
    );
    ts_template! {
        validate: (): Array<string> => {
            const fieldErrors = @{validate_call};
            return fieldErrors.map((e: { field: string; message: string }) => e.message);
        },
    }
}

/// Generates array-specific methods (at, push, remove, swap) with closures.
fn generate_array_methods(field: &ParsedField, name: &str) -> TsStream {
    let element_type = field.array_element_type.as_deref().unwrap_or("unknown");

    ts_template! {
        at: (index: number) => ({
            path: ["@{name}", index] as const,
            name: "'^@{name}.${index}^'",
            constraints: { required: true },
            get: () => data.@{name}[index]!,
            set: (value: @{element_type}) => { data.@{name}[index] = value; },
            transform: (value: @{element_type}): @{element_type} => value,
            getError: () => errors.@{name},
            setError: (value: Option<Array<string>>) => { errors.@{name} = value; },
            getTainted: () => tainted.@{name},
            setTainted: (value: Option<boolean>) => { tainted.@{name} = value; },
            validate: (): Array<string> => [],
        }),
        push: (item: @{element_type}) => { data.@{name}.push(item); },
        remove: (index: number) => { data.@{name}.splice(index, 1); },
        swap: (a: number, b: number) => {
            const tmp = data.@{name}[a]!;
            data.@{name}[a] = data.@{name}[b]!;
            data.@{name}[b] = tmp;
        },
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
        {>> "Creates a new enum step form with navigation controls" <<}
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
