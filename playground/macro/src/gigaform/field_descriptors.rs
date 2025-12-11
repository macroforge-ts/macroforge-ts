//! Generates field descriptors with get/set/validate accessors and UI metadata.

use macroforge_ts::macros::ts_template;
use macroforge_ts::ts_syn::TsStream;

use crate::gigaform::i18n;
use crate::gigaform::parser::{BaseControllerOptions, GigaformOptions, ParsedField, ValidatorSpec};

/// Generates the fields object with descriptors for each field.
pub fn generate(_interface_name: &str, fields: &[ParsedField], options: &GigaformOptions) -> TsStream {
    let field_descriptors = generate_field_descriptors(fields, options, &[]);

    ts_template! {
        {>> Field descriptors with type-safe accessors and validation. <<}
        export const fields = {
            @{field_descriptors}
        } as const;
    }
}

/// Generates all field descriptor entries.
fn generate_field_descriptors(
    fields: &[ParsedField],
    options: &GigaformOptions,
    path_prefix: &[&str],
) -> String {
    fields
        .iter()
        .map(|field| generate_field_descriptor(field, options, path_prefix))
        .collect::<Vec<_>>()
        .join(",\n            ")
}

/// Generates a single field descriptor.
fn generate_field_descriptor(
    field: &ParsedField,
    options: &GigaformOptions,
    path_prefix: &[&str],
) -> String {
    let name = &field.name;
    let _ts_type = &field.ts_type;

    // Build the path array
    let mut full_path = path_prefix.to_vec();
    full_path.push(name);
    let path_array = full_path
        .iter()
        .map(|s| format!("\"{s}\""))
        .collect::<Vec<_>>()
        .join(", ");
    let path_string = full_path.join(".");

    // Generate constraints from validators
    let constraints = generate_constraints(&field.validators, !field.optional);

    // Generate the validator function
    let validate_fn = generate_validate_function(field, options);

    // Generate async validator if present
    let async_validate = if !field.async_validators.is_empty() {
        generate_async_validate_function(field, options)
    } else {
        String::new()
    };

    // Generate accessors
    let get_fn = generate_get_accessor(field, path_prefix);
    let set_fn = generate_set_accessor(field, path_prefix);
    let get_error_fn = generate_get_error_accessor(field, path_prefix);
    let set_error_fn = generate_set_error_accessor(field, path_prefix);
    let get_tainted_fn = generate_get_tainted_accessor(field, path_prefix);
    let set_tainted_fn = generate_set_tainted_accessor(field, path_prefix);

    // For array fields, add array-specific methods
    let array_methods = if field.is_array {
        generate_array_methods(field, path_prefix)
    } else {
        String::new()
    };

    // Generate UI metadata from controller options
    let ui_metadata = generate_ui_metadata(field);

    format!(
        r#"{name}: {{
                path: [{path_array}] as const,
                name: "{path_string}",
                constraints: {constraints},
                {ui_metadata}
                {get_fn}
                {set_fn}
                {get_error_fn}
                {set_error_fn}
                {get_tainted_fn}
                {set_tainted_fn}
                {validate_fn}
                {async_validate}
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

    props.join("\n                ")
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
                    // Escape for JSON string
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

/// Generates the validate function for sync validators.
fn generate_validate_function(field: &ParsedField, options: &GigaformOptions) -> String {
    if field.validators.is_empty() {
        return format!(
            "validate: (_value: {}): Array<string> => [],",
            field.ts_type
        );
    }

    let checks = field
        .validators
        .iter()
        .map(|v| generate_validator_check(v, "value", &field.name, options))
        .collect::<Vec<_>>()
        .join("\n                    ");

    format!(
        r#"validate: (value: {}): Array<string> => {{
                    const errors: Array<string> = [];
                    {checks}
                    return errors;
                }},"#,
        field.ts_type
    )
}

/// Generates a single validator check statement.
fn generate_validator_check(
    validator: &ValidatorSpec,
    var_name: &str,
    field_name: &str,
    options: &GigaformOptions,
) -> String {
    let error_msg = validator.message.clone().unwrap_or_else(|| {
        i18n::get_validator_message(&validator.name, &validator.args, field_name, options)
    });

    match validator.name.as_str() {
        // String validators
        "email" => {
            format!(r#"if (typeof {var_name} === "string" && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test({var_name})) errors.push("{error_msg}");"#)
        }
        "url" => {
            format!(r#"if (typeof {var_name} === "string") {{ try {{ new URL({var_name}); }} catch {{ errors.push("{error_msg}"); }} }}"#)
        }
        "uuid" => {
            format!(r#"if (typeof {var_name} === "string" && !/^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{12}}$/i.test({var_name})) errors.push("{error_msg}");"#)
        }
        "nonEmpty" => {
            format!(r#"if (typeof {var_name} === "string" && {var_name}.length === 0) errors.push("{error_msg}");"#)
        }
        "trimmed" => {
            format!(r#"if (typeof {var_name} === "string" && {var_name} !== {var_name}.trim()) errors.push("{error_msg}");"#)
        }
        "lowercase" => {
            format!(r#"if (typeof {var_name} === "string" && {var_name} !== {var_name}.toLowerCase()) errors.push("{error_msg}");"#)
        }
        "uppercase" => {
            format!(r#"if (typeof {var_name} === "string" && {var_name} !== {var_name}.toUpperCase()) errors.push("{error_msg}");"#)
        }
        "minLength" => {
            let min = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (typeof {var_name} === "string" && {var_name}.length < {min}) errors.push("{error_msg}");"#)
        }
        "maxLength" => {
            let max = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (typeof {var_name} === "string" && {var_name}.length > {max}) errors.push("{error_msg}");"#)
        }
        "length" => {
            if validator.args.len() == 1 {
                let len = &validator.args[0];
                format!(r#"if (typeof {var_name} === "string" && {var_name}.length !== {len}) errors.push("{error_msg}");"#)
            } else if validator.args.len() >= 2 {
                let min = &validator.args[0];
                let max = &validator.args[1];
                format!(r#"if (typeof {var_name} === "string" && ({var_name}.length < {min} || {var_name}.length > {max})) errors.push("{error_msg}");"#)
            } else {
                String::new()
            }
        }
        "pattern" => {
            let pattern = validator.args.first().map(|s| s.as_str()).unwrap_or("");
            // Escape for use in RegExp constructor
            let escaped = pattern.replace('\\', "\\\\");
            format!(r#"if (typeof {var_name} === "string" && !new RegExp("{escaped}").test({var_name})) errors.push("{error_msg}");"#)
        }
        "startsWith" => {
            let prefix = validator.args.first().map(|s| s.as_str()).unwrap_or("");
            format!(r#"if (typeof {var_name} === "string" && !{var_name}.startsWith("{prefix}")) errors.push("{error_msg}");"#)
        }
        "endsWith" => {
            let suffix = validator.args.first().map(|s| s.as_str()).unwrap_or("");
            format!(r#"if (typeof {var_name} === "string" && !{var_name}.endsWith("{suffix}")) errors.push("{error_msg}");"#)
        }
        "includes" => {
            let substr = validator.args.first().map(|s| s.as_str()).unwrap_or("");
            format!(r#"if (typeof {var_name} === "string" && !{var_name}.includes("{substr}")) errors.push("{error_msg}");"#)
        }

        // Number validators
        "positive" => {
            format!(r#"if (typeof {var_name} === "number" && {var_name} <= 0) errors.push("{error_msg}");"#)
        }
        "negative" => {
            format!(r#"if (typeof {var_name} === "number" && {var_name} >= 0) errors.push("{error_msg}");"#)
        }
        "nonNegative" => {
            format!(r#"if (typeof {var_name} === "number" && {var_name} < 0) errors.push("{error_msg}");"#)
        }
        "nonPositive" => {
            format!(r#"if (typeof {var_name} === "number" && {var_name} > 0) errors.push("{error_msg}");"#)
        }
        "int" => {
            format!(r#"if (typeof {var_name} === "number" && !Number.isInteger({var_name})) errors.push("{error_msg}");"#)
        }
        "finite" => {
            format!(r#"if (typeof {var_name} === "number" && !Number.isFinite({var_name})) errors.push("{error_msg}");"#)
        }
        "nonNaN" => {
            format!(r#"if (typeof {var_name} === "number" && Number.isNaN({var_name})) errors.push("{error_msg}");"#)
        }
        "greaterThan" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (typeof {var_name} === "number" && {var_name} <= {n}) errors.push("{error_msg}");"#)
        }
        "greaterThanOrEqualTo" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (typeof {var_name} === "number" && {var_name} < {n}) errors.push("{error_msg}");"#)
        }
        "lessThan" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (typeof {var_name} === "number" && {var_name} >= {n}) errors.push("{error_msg}");"#)
        }
        "lessThanOrEqualTo" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (typeof {var_name} === "number" && {var_name} > {n}) errors.push("{error_msg}");"#)
        }
        "between" => {
            if validator.args.len() >= 2 {
                let min = &validator.args[0];
                let max = &validator.args[1];
                format!(r#"if (typeof {var_name} === "number" && ({var_name} < {min} || {var_name} > {max})) errors.push("{error_msg}");"#)
            } else {
                String::new()
            }
        }
        "multipleOf" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("1");
            format!(r#"if (typeof {var_name} === "number" && {var_name} % {n} !== 0) errors.push("{error_msg}");"#)
        }

        // Array validators
        "minItems" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (Array.isArray({var_name}) && {var_name}.length < {n}) errors.push("{error_msg}");"#)
        }
        "maxItems" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (Array.isArray({var_name}) && {var_name}.length > {n}) errors.push("{error_msg}");"#)
        }
        "itemsCount" => {
            let n = validator.args.first().map(|s| s.as_str()).unwrap_or("0");
            format!(r#"if (Array.isArray({var_name}) && {var_name}.length !== {n}) errors.push("{error_msg}");"#)
        }

        // Date validators
        "validDate" => {
            format!(r#"if ({var_name} instanceof Date && Number.isNaN({var_name}.getTime())) errors.push("{error_msg}");"#)
        }
        "greaterThanDate" => {
            let date = validator.args.first().map(|s| s.as_str()).unwrap_or("");
            format!(r#"if ({var_name} instanceof Date && {var_name}.getTime() <= new Date("{date}").getTime()) errors.push("{error_msg}");"#)
        }
        "lessThanDate" => {
            let date = validator.args.first().map(|s| s.as_str()).unwrap_or("");
            format!(r#"if ({var_name} instanceof Date && {var_name}.getTime() >= new Date("{date}").getTime()) errors.push("{error_msg}");"#)
        }

        // Custom validator
        "custom" => {
            let fn_name = validator.args.first().map(|s| s.as_str()).unwrap_or("() => true");
            format!(r#"if (!{fn_name}({var_name})) errors.push("{error_msg}");"#)
        }

        _ => {
            // Unknown validator - skip
            String::new()
        }
    }
}

/// Generates the async validate function.
fn generate_async_validate_function(field: &ParsedField, options: &GigaformOptions) -> String {
    let checks = field
        .async_validators
        .iter()
        .map(|fn_name| {
            let error_msg = i18n::get_async_validator_message(fn_name, &field.name, options);
            format!(r#"if (!(await {fn_name}(value))) errors.push("{error_msg}");"#)
        })
        .collect::<Vec<_>>()
        .join("\n                    ");

    format!(
        r#"validateAsync: async (value: {}): Promise<Array<string>> => {{
                    const errors: Array<string> = [];
                    {checks}
                    return errors;
                }},"#,
        field.ts_type
    )
}

/// Generates the get accessor.
fn generate_get_accessor(field: &ParsedField, path_prefix: &[&str]) -> String {
    let accessor = build_accessor_path(path_prefix, &field.name);
    format!(
        "get: (data: Data) => data{accessor},",
    )
}

/// Generates the set accessor.
fn generate_set_accessor(field: &ParsedField, path_prefix: &[&str]) -> String {
    let accessor = build_accessor_path(path_prefix, &field.name);
    format!(
        "set: (data: Data, value: {}) => {{ data{accessor} = value; }},",
        field.ts_type
    )
}

/// Generates the getError accessor.
fn generate_get_error_accessor(field: &ParsedField, path_prefix: &[&str]) -> String {
    let accessor = build_optional_accessor_path(path_prefix, &field.name);
    format!(
        "getError: (errors: Errors) => errors{accessor},",
    )
}

/// Generates the setError accessor.
fn generate_set_error_accessor(field: &ParsedField, path_prefix: &[&str]) -> String {
    // Need to ensure parent objects exist
    let name = &field.name;
    if path_prefix.is_empty() {
        format!(
            "setError: (errors: Errors, value: Array<string> | undefined) => {{ errors.{name} = value; }},",
        )
    } else {
        let ensure_path = build_ensure_path_code("errors", path_prefix);
        let accessor = build_accessor_path(path_prefix, name);
        format!(
            "setError: (errors: Errors, value: Array<string> | undefined) => {{ {ensure_path} errors{accessor} = value; }},",
        )
    }
}

/// Generates the getTainted accessor.
fn generate_get_tainted_accessor(field: &ParsedField, path_prefix: &[&str]) -> String {
    let accessor = build_optional_accessor_path(path_prefix, &field.name);
    format!(
        "getTainted: (tainted: Tainted) => tainted{accessor} ?? false,",
    )
}

/// Generates the setTainted accessor.
fn generate_set_tainted_accessor(field: &ParsedField, path_prefix: &[&str]) -> String {
    let name = &field.name;
    if path_prefix.is_empty() {
        format!(
            "setTainted: (tainted: Tainted, value: boolean) => {{ tainted.{name} = value; }},",
        )
    } else {
        let ensure_path = build_ensure_path_code("tainted", path_prefix);
        let accessor = build_accessor_path(path_prefix, name);
        format!(
            "setTainted: (tainted: Tainted, value: boolean) => {{ {ensure_path} tainted{accessor} = value; }},",
        )
    }
}

/// Generates array-specific methods (at, push, remove, swap).
fn generate_array_methods(field: &ParsedField, path_prefix: &[&str]) -> String {
    let name = &field.name;
    let accessor = build_accessor_path(path_prefix, name);
    let element_type = field.array_element_type.as_deref().unwrap_or("unknown");

    format!(
        r#"at: (index: number) => ({{
                    path: [...[{path_parts}], index] as const,
                    name: `{path_string}.${{index}}`,
                    constraints: {{ required: true }},
                    get: (data: Data) => data{accessor}[index],
                    set: (data: Data, value: {element_type}) => {{ data{accessor}[index] = value; }},
                    getError: (errors: Errors) => (errors.{name} as Record<number, Array<string>>)?.[index],
                    setError: (errors: Errors, value: Array<string> | undefined) => {{
                        errors.{name} ??= {{}};
                        (errors.{name} as Record<number, Array<string>>)[index] = value!;
                    }},
                    getTainted: (tainted: Tainted) => tainted.{name}?.[index] ?? false,
                    setTainted: (tainted: Tainted, value: boolean) => {{
                        tainted.{name} ??= {{}};
                        tainted.{name}[index] = value;
                    }},
                    validate: (_value: {element_type}): Array<string> => [],
                }}),
                push: (data: Data, item: {element_type}) => {{ data{accessor}.push(item); }},
                remove: (data: Data, index: number) => {{ data{accessor}.splice(index, 1); }},
                swap: (data: Data, a: number, b: number) => {{
                    [data{accessor}[a], data{accessor}[b]] = [data{accessor}[b], data{accessor}[a]];
                }},"#,
        path_parts = build_path_parts(path_prefix, name),
        path_string = build_path_string(path_prefix, name),
    )
}

/// Builds an accessor path like .foo.bar.field
fn build_accessor_path(prefix: &[&str], field_name: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(field_name);
    parts.iter().map(|p| format!(".{p}")).collect()
}

/// Builds an optional accessor path like ?.foo?.bar?.field
fn build_optional_accessor_path(prefix: &[&str], field_name: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(field_name);
    parts.iter().map(|p| format!("?.{p}")).collect()
}

/// Builds code to ensure nested path objects exist.
fn build_ensure_path_code(var: &str, prefix: &[&str]) -> String {
    prefix
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let path = prefix[..=i].iter().map(|p| format!(".{p}")).collect::<String>();
            format!("{var}{path} ??= {{}};")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Builds path parts for array item descriptor.
fn build_path_parts(prefix: &[&str], field_name: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(field_name);
    parts
        .iter()
        .map(|p| format!("\"{p}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Builds path string for array item descriptor.
fn build_path_string(prefix: &[&str], field_name: &str) -> String {
    let mut parts = prefix.to_vec();
    parts.push(field_name);
    parts.join(".")
}
