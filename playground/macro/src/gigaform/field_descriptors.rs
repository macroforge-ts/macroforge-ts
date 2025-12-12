//! Generates the createForm factory function with reactive state and field controllers.

use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::parser::{BaseControllerOptions, GigaformOptions, ParsedField, ValidatorSpec};

/// Generates the createForm factory function that returns a Gigaform instance.
pub fn generate_factory(interface_name: &str, fields: &[ParsedField], options: &GigaformOptions) -> TsStream {
    let field_controllers = generate_field_controllers(fields, options, interface_name);
    let default_init = generate_default_init(interface_name, options);

    ts_template! {
        {>> "Creates a new Gigaform instance with reactive state and field controllers." <<}
        export function createForm(overrides?: Partial<@{interface_name}>): Gigaform {
            // Reactive state using Svelte 5 $state
            let data = $state({ @{default_init}, ...overrides });
            let errors = $state<Errors>({});
            let tainted = $state<Tainted>({});

            // Field controllers with closures capturing reactive state
            const fields: FieldControllers = {
                @{field_controllers}
            };

            // Validate the entire form using Deserialize's fromJSON
            function validate(): Result<@{interface_name}, Array<{ field: string; message: string }>> {
                return @{interface_name}.fromJSON(data);
            }

            // Reset form to defaults
            function reset(newOverrides?: Partial<@{interface_name}>): void {
                data = { @{default_init}, ...newOverrides };
                errors = {};
                tainted = {};
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

/// Generates the default initialization expression.
fn generate_default_init(interface_name: &str, options: &GigaformOptions) -> String {
    if let Some(override_fn) = &options.default_override {
        format!("...{interface_name}.defaultValue(), ...{override_fn}()")
    } else {
        format!("...{interface_name}.defaultValue()")
    }
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
                    getError: () => errors?.{name},
                    setError: (value: Array<string> | undefined) => {{ errors.{name} = value; }},
                    getTainted: () => tainted?.{name} ?? false,
                    setTainted: (value: boolean) => {{ tainted.{name} = value; }},
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

/// Generates the field-level validate function that delegates to form validation.
fn generate_field_validate_function(field_name: &str, interface_name: &str) -> String {
    format!(
        r#"validate: (): Array<string> => {{
                        const result = {interface_name}.fromJSON(data);
                        if (result.isErr()) {{
                            const allErrors = result.unwrapErr();
                            return allErrors.filter(e => e.field === "{field_name}").map(e => e.message);
                        }}
                        return [];
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
                        getError: () => (errors.{name} as Record<number, Array<string>>)?.[index],
                        setError: (value: Array<string> | undefined) => {{
                            errors.{name} ??= {{}};
                            (errors.{name} as Record<number, Array<string>>)[index] = value!;
                        }},
                        getTainted: () => tainted.{name}?.[index] ?? false,
                        setTainted: (value: boolean) => {{
                            tainted.{name} ??= {{}};
                            tainted.{name}[index] = value;
                        }},
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
                getError: (errors: Errors) => errors{optional_accessor},
                setError: (errors: Errors, value: Array<string> | undefined) => {{ errors.{name} = value; }},
                getTainted: (tainted: Tainted) => tainted{optional_accessor} ?? false,
                setTainted: (tainted: Tainted, value: boolean) => {{ tainted.{name} = value; }},
                validate: (_value: {ts_type}): Array<string> => [],
            }}"#,
        optional_accessor = build_optional_accessor_path(path_prefix, name),
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
