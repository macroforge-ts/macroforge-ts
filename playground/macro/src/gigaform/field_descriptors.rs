//! Generates the createForm factory function with reactive state and field controllers.

use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::parser::{BaseControllerOptions, GigaformOptions, ParsedField, UnionConfig, UnionMode, ValidatorSpec};
use crate::gigaform::GenericInfo;

/// Generates the createForm factory function that returns a Gigaform instance.
pub fn generate_factory(interface_name: &str, fields: &[ParsedField], options: &GigaformOptions) -> TsStream {
    let field_controllers = generate_field_controllers(fields, options, interface_name);
    let default_init = generate_default_init(interface_name, options);
    let default_errors_init = generate_default_errors_init(fields);
    let default_tainted_init = generate_default_tainted_init(fields);

    ts_template! {
        {>> "Creates a new Gigaform instance with reactive state and field controllers." <<}
        export function createForm(overrides?: Partial<@{interface_name}>): Gigaform {
            // Reactive state using Svelte 5 $state
            let data = $state({ @{default_init}, ...overrides });
            let errors = $state<Errors>(@{default_errors_init});
            let tainted = $state<Tainted>(@{default_tainted_init});

            // Field controllers with closures capturing reactive state
            const fields: FieldControllers = {
                @{field_controllers}
            };

            // Validate the entire form using Deserialize's fromObject
            function validate(): Result<@{interface_name}, Array<{ field: string; message: string }>> {
                return @{interface_name}.fromObject(data);
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}>): void {
                data = { @{default_init}, ...newOverrides };
                errors = @{default_errors_init};
                tainted = @{default_tainted_init};
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

    let field_controllers = generate_field_controllers(fields, options, interface_name);
    let default_init = generate_default_init(interface_name, options);
    let default_errors_init = generate_default_errors_init(fields);
    let default_tainted_init = generate_default_tainted_init(fields);
    let generic_decl = generics.decl();
    let generic_args = generics.args();

    ts_template! {
        {>> "Creates a new Gigaform instance with reactive state and field controllers." <<}
        export function createForm@{generic_decl}(overrides?: Partial<@{interface_name}@{generic_args}>): Gigaform@{generic_args} {
            // Reactive state using Svelte 5 $state
            let data = $state({ @{default_init}, ...overrides });
            let errors = $state<Errors@{generic_args}>(@{default_errors_init});
            let tainted = $state<Tainted@{generic_args}>(@{default_tainted_init});

            // Field controllers with closures capturing reactive state
            const fields: FieldControllers@{generic_args} = {
                @{field_controllers}
            };

            // Validate the entire form using Deserialize's fromObject
            function validate(): Result<@{interface_name}@{generic_args}, Array<{ field: string; message: string }>> {
                return @{interface_name}.fromObject@{generic_args}(data);
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}@{generic_args}>): void {
                data = { @{default_init}, ...newOverrides };
                errors = @{default_errors_init};
                tainted = @{default_tainted_init};
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
pub fn generate_union_factory(type_name: &str, config: &UnionConfig, options: &GigaformOptions) -> TsStream {
    let variant_literals = config.variants.iter()
        .map(|v| format!("\"{}\"", v.discriminant_value))
        .collect::<Vec<_>>()
        .join(" | ");

    let discriminant_field = match &config.mode {
        UnionMode::Tagged { field } => field.as_str(),
        UnionMode::Untagged => "_variant",
    };

    let first_variant = config.variants.first()
        .map(|v| v.discriminant_value.as_str())
        .unwrap_or("unknown");

    let _default_init = generate_default_init(type_name, options);

    // Generate per-variant field controllers
    let variant_controllers = generate_union_variant_controllers(config, options, type_name);

    // Generate getDefaultForVariant function
    let default_for_variant = generate_default_for_variant(type_name, config, discriminant_field);

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
        "overrides ? overrides as typeof data : getDefaultForVariant(currentVariant)".to_string()
    } else {
        // For object unions, can spread
        "overrides ? { ...getDefaultForVariant(currentVariant), ...overrides } : getDefaultForVariant(currentVariant)".to_string()
    };

    ts_template! {
        {>> "Gets default value for a specific variant" <<}
        @{default_for_variant}

        {>> "Creates a new discriminated union Gigaform with variant switching" <<}
        export function createForm(initial?: @{type_name}): Gigaform {
            // Detect initial variant from discriminant
            const initialVariant: @{variant_literals} = @{initial_variant_expr};

            // Reactive state using Svelte 5 $state
            let currentVariant = $state<@{variant_literals}>(initialVariant);
            let data = $state<@{type_name}>(initial ?? getDefaultForVariant(initialVariant));
            let errors = $state<Errors>({} as Errors);
            let tainted = $state<Tainted>({} as Tainted);

            // Per-variant field controllers
            const variants: VariantFields = {
                @{variant_controllers}
            };

            // Switch to a different variant
            function switchVariant(variant: @{variant_literals}): void {
                currentVariant = variant;
                data = getDefaultForVariant(variant);
                errors = {} as Errors;
                tainted = {} as Tainted;
            }

            // Validate the entire form using Deserialize's fromObject
            function validate(): Result<@{type_name}, Array<{ field: string; message: string }>> {
                return @{type_name}.fromObject(data);
            }

            // Reset form
            function reset(overrides?: Partial<@{type_name}>): void {
                data = @{reset_data_expr};
                errors = {} as Errors;
                tainted = {} as Tainted;
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
fn get_type_ref_default(type_ref: &str, cast_type: &str) -> String {
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
        // Generic type like RecordLink<Service> -> RecordLink.defaultValue<Service>()
        _ if tr.contains('<') => {
            if let Some(bracket_pos) = tr.find('<') {
                let base_type = &tr[..bracket_pos];
                let type_args = &tr[bracket_pos..]; // includes the <>
                format!("{base_type}.defaultValue{type_args}() as {cast_type}")
            } else {
                format!("{tr}.defaultValue() as {cast_type}")
            }
        }
        // Named type - use TypeName.defaultValue()
        _ => format!("{tr}.defaultValue() as {cast_type}"),
    }
}

/// Generates the getDefaultForVariant function.
fn generate_default_for_variant(type_name: &str, config: &UnionConfig, discriminant_field: &str) -> String {
    // Determine how to generate the default value based on the discriminant field
    // - "_value": literal union (e.g., "Home" | "About") - return the literal
    // - "_type": type ref union (e.g., DailyRule | WeeklyRule) - return TypeName.defaultValue()
    // - other: object union with tag field - return { field: "value" }
    let is_literal_union = discriminant_field == "_value";
    let is_type_ref_union = discriminant_field == "_type";

    let cases = config.variants.iter().map(|variant| {
        let value = &variant.discriminant_value;
        if is_literal_union {
            format!(r#"case "{value}": return "{value}" as {type_name};"#)
        } else if is_type_ref_union {
            let default_expr = get_type_ref_default(value, type_name);
            format!(r#"case "{value}": return {default_expr};"#)
        } else {
            format!(r#"case "{value}": return {{ {discriminant_field}: "{value}" }} as {type_name};"#)
        }
    }).collect::<Vec<_>>().join("\n            ");

    let first_value = config.variants.first()
        .map(|v| v.discriminant_value.as_str())
        .unwrap_or("unknown");

    let default_return = if is_literal_union {
        format!(r#"return "{first_value}" as {type_name};"#)
    } else if is_type_ref_union {
        let default_expr = get_type_ref_default(first_value, type_name);
        format!(r#"return {default_expr};"#)
    } else {
        format!(r#"return {{ {discriminant_field}: "{first_value}" }} as {type_name};"#)
    };

    format!(
        r#"function getDefaultForVariant(variant: string): {type_name} {{
        switch (variant) {{
            {cases}
            default: {default_return}
        }}
    }}"#
    )
}

/// Generates per-variant field controllers.
fn generate_union_variant_controllers(config: &UnionConfig, options: &GigaformOptions, type_name: &str) -> String {
    config.variants.iter().map(|variant| {
        let value = &variant.discriminant_value;
        let variant_name = to_pascal_case(&variant.discriminant_value);
        let field_controllers = generate_field_controllers(&variant.fields, options, type_name);

        // Quote the property key if it contains special characters
        let prop_key = if needs_quoting(value) {
            format!("\"{}\"", value)
        } else {
            value.clone()
        };

        format!(
            r#"{prop_key}: {{
                    fields: {{
                        {field_controllers}
                    }} as {variant_name}FieldControllers
                }}"#
        )
    }).collect::<Vec<_>>().join(",\n                ")
}

/// Returns true if a string needs to be quoted when used as an object property key.
fn needs_quoting(s: &str) -> bool {
    // Needs quoting if it contains special chars or starts with a digit
    s.chars().any(|c| !c.is_alphanumeric() && c != '_') ||
    s.chars().next().map(|c| c.is_numeric()).unwrap_or(true)
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
        } else if c == '_' || c == '-' || c == ' ' || c == '(' || c == ')' || c == '<' || c == '>' || c == ',' {
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
    if result.is_empty() || result.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
        format!("Variant{}", result)
    } else {
        result
    }
}

// =============================================================================
// Standard Field Factory Generation
// =============================================================================

/// Generates the default initialization expression.
fn generate_default_init(interface_name: &str, options: &GigaformOptions) -> String {
    if let Some(override_fn) = &options.default_override {
        format!("...{interface_name}.defaultValue(), ...{override_fn}()")
    } else {
        format!("...{interface_name}.defaultValue()")
    }
}

/// Generates the default errors initialization with all fields set to Option.none().
fn generate_default_errors_init(fields: &[ParsedField]) -> String {
    let field_inits: Vec<String> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            format!("{name}: Option.none()")
        })
        .collect();

    format!("{{ _errors: Option.none(), {} }}", field_inits.join(", "))
}

/// Generates the default tainted initialization with all fields set to Option.none().
fn generate_default_tainted_init(fields: &[ParsedField]) -> String {
    let field_inits: Vec<String> = fields
        .iter()
        .map(|field| {
            let name = &field.name;
            format!("{name}: Option.none()")
        })
        .collect();

    format!("{{ {} }}", field_inits.join(", "))
}

/// Generates all field controller entries.
fn generate_field_controllers(
    fields: &[ParsedField],
    options: &GigaformOptions,
    interface_name: &str,
) -> String {
    fields
        .iter()
        .map(|field| generate_field_controller(field, options, interface_name))
        .collect::<Vec<_>>()
        .join(",\n                ")
}

/// Generates a single field controller with closure-based accessors.
fn generate_field_controller(
    field: &ParsedField,
    _options: &GigaformOptions,
    interface_name: &str,
) -> String {
    let name = &field.name;
    let ts_type = &field.ts_type;

    // Build the path array
    let path_array = format!("\"{}\"", name);
    let path_string = name.clone();

    // Generate constraints from validators
    let constraints = generate_constraints(&field.validators, !field.optional);

    // Generate UI metadata from controller options
    let ui_metadata = generate_ui_metadata(field);

    // Generate the validate function that delegates to form validation
    let validate_fn = generate_field_validate_function(name, interface_name);

    // For array fields, add array-specific methods
    let array_methods = if field.is_array {
        generate_array_methods(field, name)
    } else {
        String::new()
    };

    format!(
        r#"{name}: {{
                    path: [{path_array}] as const,
                    name: "{path_string}",
                    constraints: {constraints},
                    {ui_metadata}
                    get: () => data.{name},
                    set: (value: {ts_type}) => {{ data.{name} = value; }},
                    getError: () => errors.{name},
                    setError: (value: Option<Array<string>>) => {{ errors.{name} = value; }},
                    getTainted: () => tainted.{name},
                    setTainted: (value: Option<boolean>) => {{ tainted.{name} = value; }},
                    {validate_fn}
                    {array_methods}
                }}"#
    )
}

/// Generates UI metadata properties from controller options.
fn generate_ui_metadata(field: &ParsedField) -> String {
    let default_base = BaseControllerOptions::default();
    let base = field
        .controller
        .as_ref()
        .map(|c| c.base())
        .unwrap_or(&default_base);

    let mut props = Vec::new();

    if let Some(label) = base.get_label() {
        props.push(format!("label: \"{label}\","));
    }
    if let Some(description) = &base.description {
        props.push(format!("description: \"{description}\","));
    }
    if let Some(placeholder) = &base.placeholder {
        props.push(format!("placeholder: \"{placeholder}\","));
    }
    if let Some(disabled) = base.disabled {
        props.push(format!("disabled: {disabled},"));
    }
    if let Some(readonly) = base.readonly {
        props.push(format!("readonly: {readonly},"));
    }

    props.join("\n                    ")
}

/// Generates the constraints object from validators.
fn generate_constraints(validators: &[ValidatorSpec], required: bool) -> String {
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

    if constraints.is_empty() {
        "{}".to_string()
    } else {
        format!("{{ {} }}", constraints.join(", "))
    }
}

/// Generates the field-level validate function that uses per-field validation.
fn generate_field_validate_function(field_name: &str, interface_name: &str) -> String {
    format!(
        r#"validate: (): Array<string> => {{
                        const fieldErrors = {interface_name}.validateField("{field_name}", data.{field_name});
                        return fieldErrors.map(e => e.message);
                    }},"#
    )
}

/// Generates array-specific methods (at, push, remove, swap) with closures.
fn generate_array_methods(field: &ParsedField, name: &str) -> String {
    let element_type = field.array_element_type.as_deref().unwrap_or("unknown");

    format!(
        r#"at: (index: number) => ({{
                        path: ["{name}", index] as const,
                        name: `{name}.${{index}}`,
                        constraints: {{ required: true }},
                        get: () => data.{name}[index],
                        set: (value: {element_type}) => {{ data.{name}[index] = value; }},
                        getError: () => errors.{name},
                        setError: (value: Option<Array<string>>) => {{ errors.{name} = value; }},
                        getTainted: () => tainted.{name},
                        setTainted: (value: Option<boolean>) => {{ tainted.{name} = value; }},
                        validate: (): Array<string> => [],
                    }}),
                    push: (item: {element_type}) => {{ data.{name}.push(item); }},
                    remove: (index: number) => {{ data.{name}.splice(index, 1); }},
                    swap: (a: number, b: number) => {{
                        [data.{name}[a], data.{name}[b]] = [data.{name}[b], data.{name}[a]];
                    }},"#
    )
}

// Keep the old generate function for backward compatibility during migration
#[allow(dead_code)]
pub fn generate(_interface_name: &str, fields: &[ParsedField], options: &GigaformOptions) -> TsStream {
    let field_descriptors = generate_old_field_descriptors(fields, options, &[]);

    ts_template! {
        {>> "Field descriptors with type-safe accessors and validation." <<}
        export const fields = {
            @{field_descriptors}
        } as const;
    }
}

/// Old field descriptors generation (kept for reference during migration)
#[allow(dead_code)]
fn generate_old_field_descriptors(
    fields: &[ParsedField],
    options: &GigaformOptions,
    path_prefix: &[&str],
) -> String {
    fields
        .iter()
        .map(|field| generate_old_field_descriptor(field, options, path_prefix))
        .collect::<Vec<_>>()
        .join(",\n            ")
}

#[allow(dead_code)]
fn generate_old_field_descriptor(
    field: &ParsedField,
    _options: &GigaformOptions,
    path_prefix: &[&str],
) -> String {
    let name = &field.name;
    let ts_type = &field.ts_type;

    let mut full_path = path_prefix.to_vec();
    full_path.push(name);
    let path_array = full_path
        .iter()
        .map(|s| format!("\"{s}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let path_string = full_path.join(".");

    let constraints = generate_constraints(&field.validators, !field.optional);
    let ui_metadata = generate_ui_metadata(field);
    let accessor = build_accessor_path(path_prefix, name);

    format!(
        r#"{name}: {{
                path: [{path_array}] as const,
                name: "{path_string}",
                constraints: {constraints},
                {ui_metadata}
                get: (data: Data) => data{accessor},
                set: (data: Data, value: {ts_type}) => {{ data{accessor} = value; }},
                getError: (errors: Errors) => errors.{name},
                setError: (errors: Errors, value: Option<Array<string>>) => {{ errors.{name} = value; }},
                getTainted: (tainted: Tainted) => tainted.{name},
                setTainted: (tainted: Tainted, value: Option<boolean>) => {{ tainted.{name} = value; }},
                validate: (_value: {ts_type}): Array<string> => [],
            }}"#
    )
}

fn build_accessor_path(prefix: &[&str], field_name: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(field_name);
    parts.iter().map(|p| format!(".{p}")).collect()
}

fn build_optional_accessor_path(prefix: &[&str], field_name: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(field_name);
    parts.iter().map(|p| format!("?.{p}")).collect()
}

// =============================================================================
// Enum Factory Generation
// =============================================================================

use crate::gigaform::parser::EnumFormConfig;

/// Generates the createForm factory for an enum step form.
pub fn generate_enum_factory(enum_name: &str, config: &EnumFormConfig) -> TsStream {
    let first_variant = config
        .variants
        .first()
        .map(|v| v.value_expr.as_str())
        .unwrap_or("0");

    ts_template! {
        {>> "Creates a new enum step form with navigation controls" <<}
        export function createForm(initialStep?: @{enum_name}): Gigaform {
            let currentStep = $state<@{enum_name}>(initialStep ?? @{first_variant});

            return {
                get currentStep() { return currentStep; },
                steps: variants,
                setStep(step: @{enum_name}): void {
                    currentStep = step;
                },
                nextStep(): boolean {
                    const idx = variants.findIndex(v => v.value === currentStep);
                    if (idx < variants.length - 1) {
                        currentStep = variants[idx + 1].value;
                        return true;
                    }
                    return false;
                },
                prevStep(): boolean {
                    const idx = variants.findIndex(v => v.value === currentStep);
                    if (idx > 0) {
                        currentStep = variants[idx - 1].value;
                        return true;
                    }
                    return false;
                },
            };
        }
    }
}
