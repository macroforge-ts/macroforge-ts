//! /** @derive(Deserialize) */ macro implementation
//!
//! Generates JSON deserialization methods:
//! - For classes: `static fromJSON(data: unknown): ClassName`
//! - For interfaces: `namespace InterfaceName { export function is(data: unknown): data is InterfaceName; export function fromJSON(data: unknown): InterfaceName }`

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory};

/// Field info for deserialization
#[derive(Clone)]
struct DeserializeField {
    json_key: String,
    field_name: String,
    ts_type: String,
    type_cat: TypeCategory,
    optional: bool,
    #[allow(dead_code)]
    has_default: bool,
    default_expr: Option<String>,
    flatten: bool,
}

impl DeserializeField {
    /// Check if this type needs transformation (Date, Map, Set, Serializable)
    fn needs_transformation(&self) -> bool {
        matches!(
            self.type_cat,
            TypeCategory::Date
                | TypeCategory::Map(_, _)
                | TypeCategory::Set(_)
                | TypeCategory::Serializable(_)
        )
    }
}

#[ts_macro_derive(
    Deserialize,
    description = "Generates a static fromJSON() method for JSON deserialization with runtime validation",
    attributes(serde)
)]
pub fn derive_deserialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Collect deserializable fields
            let fields: Vec<DeserializeField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = SerdeFieldOptions::from_decorators(&field.decorators);
                    if !opts.should_deserialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    Some(DeserializeField {
                        json_key,
                        field_name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        type_cat,
                        optional: field.optional || opts.default || opts.default_expr.is_some(),
                        has_default: opts.default || opts.default_expr.is_some(),
                        default_expr: opts.default_expr.clone(),
                        flatten: opts.flatten,
                    })
                })
                .collect();

            // Separate required vs optional fields
            let required_fields: Vec<_> = fields
                .iter()
                .filter(|f| !f.optional && !f.flatten)
                .cloned()
                .collect();
            let optional_fields: Vec<_> = fields
                .iter()
                .filter(|f| f.optional && !f.flatten)
                .cloned()
                .collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            // Build known keys for deny_unknown_fields
            let known_keys: Vec<String> = fields
                .iter()
                .filter(|f| !f.flatten)
                .map(|f| f.json_key.clone())
                .collect();

            let has_required = !required_fields.is_empty();
            let has_optional = !optional_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();
            let deny_unknown = container_opts.deny_unknown_fields;

            Ok(body! {
                static fromJSON(data: unknown): @{class_name} {
                    if (typeof data !== "object" || data === null || Array.isArray(data)) {
                        throw new Error("@{class_name}.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data));
                    }

                    const obj = data as Record<string, unknown>;

                    {#if deny_unknown}
                        const knownKeys = new Set([{#for key in known_keys}"@{key}", {/for}]);
                        for (const key of Object.keys(obj)) {
                            if (!knownKeys.has(key)) {
                                throw new Error("@{class_name}.fromJSON: unknown field \"" + key + "\"");
                            }
                        }
                    {/if}

                    {#if has_required}
                        {#for field in &required_fields}
                            if (!("@{field.json_key}" in obj)) {
                                throw new Error("@{class_name}.fromJSON: missing required field \"@{field.json_key}\"");
                            }
                        {/for}
                    {/if}

                    const instance = new @{class_name}();

                    {#if has_required}
                        {#for field in &required_fields}
                            {%let raw_var = format!("__raw_{}", field.field_name)}
                            {#match &field.type_cat}
                                {:case TypeCategory::Primitive}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                {:case TypeCategory::Date}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (typeof @{raw_var} === "string") {
                                        instance.@{field.field_name} = new Date(@{raw_var});
                                    } else if (@{raw_var} instanceof Date) {
                                        instance.@{field.field_name} = @{raw_var};
                                    } else {
                                        throw new Error("@{class_name}.fromJSON: field \"@{field.json_key}\" must be a Date or ISO string");
                                    }

                                {:case TypeCategory::Array(inner)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (!Array.isArray(@{raw_var})) {
                                        throw new Error("@{class_name}.fromJSON: field \"@{field.json_key}\" must be an array");
                                    }
                                    instance.@{field.field_name} = (@{raw_var} as unknown[]).map((item) =>
                                        typeof (item as any)?.constructor?.fromJSON === "function"
                                            ? (item as any).constructor.fromJSON(item)
                                            : item as @{inner}
                                    );

                                {:case TypeCategory::Map(key_type, value_type)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (typeof @{raw_var} !== "object" || @{raw_var} === null) {
                                        throw new Error("@{class_name}.fromJSON: field \"@{field.json_key}\" must be an object for Map");
                                    }
                                    instance.@{field.field_name} = new Map(
                                        Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                    );

                                {:case TypeCategory::Set(inner)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (!Array.isArray(@{raw_var})) {
                                        throw new Error("@{class_name}.fromJSON: field \"@{field.json_key}\" must be an array for Set");
                                    }
                                    instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                {:case TypeCategory::Serializable(type_name)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (typeof (@{type_name} as any)?.fromJSON === "function") {
                                        instance.@{field.field_name} = (@{type_name} as any).fromJSON(@{raw_var});
                                    } else {
                                        instance.@{field.field_name} = @{raw_var} as @{type_name};
                                    }

                                {:case TypeCategory::Optional(_)}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                {:case TypeCategory::Nullable(_)}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                {:case TypeCategory::Unknown}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};
                            {/match}
                        {/for}
                    {/if}

                    {#if has_optional}
                        {#for field in optional_fields}
                            {%let raw_var = format!("__raw_{}", field.field_name)}
                            if ("@{field.json_key}" in obj) {
                                const @{raw_var} = obj["@{field.json_key}"];
                                if (@{raw_var} !== undefined) {
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {%let ts_type = &field.ts_type}
                                            instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                        {:case TypeCategory::Date}
                                            if (typeof @{raw_var} === "string") {
                                                instance.@{field.field_name} = new Date(@{raw_var});
                                            } else if (@{raw_var} instanceof Date) {
                                                instance.@{field.field_name} = @{raw_var};
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                            }

                                        {:case TypeCategory::Serializable(type_name)}
                                            if (typeof (@{type_name} as any)?.fromJSON === "function") {
                                                instance.@{field.field_name} = (@{type_name} as any).fromJSON(@{raw_var});
                                            } else {
                                                instance.@{field.field_name} = @{raw_var} as @{type_name};
                                            }

                                        {:case _}
                                            {%let ts_type = &field.ts_type}
                                            instance.@{field.field_name} = @{raw_var} as @{ts_type};
                                    {/match}
                                }
                            }
                            {#if let Some(default_expr) = &field.default_expr}
                                else {
                                    instance.@{field.field_name} = @{default_expr};
                                }
                            {/if}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    if (typeof (@{type_name} as any)?.fromJSON === "function") {
                                        instance.@{field.field_name} = (@{type_name} as any).fromJSON(obj);
                                    }
                                {:case _}
                                    instance.@{field.field_name} = obj as any;
                            {/match}
                        {/for}
                    {/if}

                    return instance;
                }
            })
        }
        Data::Enum(_) => Err(MacroforgeError::new(
            input.decorator_span(),
            "/** @derive(Deserialize) */ can only be applied to classes or interfaces",
        )),
        Data::Interface(interface) => {
            let interface_name = input.name();
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect deserializable fields from interface
            let fields: Vec<DeserializeField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let opts = SerdeFieldOptions::from_decorators(&field.decorators);
                    if !opts.should_deserialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    Some(DeserializeField {
                        json_key,
                        field_name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        type_cat,
                        optional: field.optional || opts.default || opts.default_expr.is_some(),
                        has_default: opts.default || opts.default_expr.is_some(),
                        default_expr: opts.default_expr.clone(),
                        flatten: opts.flatten,
                    })
                })
                .collect();

            // Non-flattened fields for type guard checks
            let check_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();

            // Fields that need transformation
            let transform_fields: Vec<_> = fields
                .iter()
                .filter(|f| !f.flatten && f.needs_transformation())
                .cloned()
                .collect();

            // Build known keys for deny_unknown_fields
            let known_keys: Vec<String> = fields
                .iter()
                .filter(|f| !f.flatten)
                .map(|f| f.json_key.clone())
                .collect();

            let has_checks = !check_fields.is_empty();
            let has_transforms = !transform_fields.is_empty();
            let deny_unknown = container_opts.deny_unknown_fields;

            Ok(ts_template! {
                export namespace @{interface_name} {
                    export function is(data: unknown): data is @{interface_name} {
                        if (typeof data !== "object" || data === null || Array.isArray(data)) {
                            return false;
                        }
                        const obj = data as Record<string, unknown>;

                        {#if deny_unknown}
                            const knownKeys = new Set([{#for key in known_keys}"@{key}", {/for}]);
                            for (const key of Object.keys(obj)) {
                                if (!knownKeys.has(key)) {
                                    return false;
                                }
                            }
                        {/if}

                        {#if has_checks}
                            {#for field in check_fields}
                                {#if field.optional}
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {%let expected_type = get_js_typeof(&field.ts_type)}
                                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined && typeof obj["@{field.json_key}"] !== "@{expected_type}") {
                                                return false;
                                            }
                                        {:case TypeCategory::Date}
                                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined && typeof obj["@{field.json_key}"] !== "string" && !(obj["@{field.json_key}"] instanceof Date)) {
                                                return false;
                                            }
                                        {:case TypeCategory::Array(_)}
                                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined && !Array.isArray(obj["@{field.json_key}"])) {
                                                return false;
                                            }
                                        {:case _}
                                    {/match}
                                {:else}
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {%let expected_type = get_js_typeof(&field.ts_type)}
                                            if (!("@{field.json_key}" in obj) || typeof obj["@{field.json_key}"] !== "@{expected_type}") {
                                                return false;
                                            }
                                        {:case TypeCategory::Date}
                                            if (!("@{field.json_key}" in obj) || (typeof obj["@{field.json_key}"] !== "string" && !(obj["@{field.json_key}"] instanceof Date))) {
                                                return false;
                                            }
                                        {:case TypeCategory::Array(_)}
                                            if (!("@{field.json_key}" in obj) || !Array.isArray(obj["@{field.json_key}"])) {
                                                return false;
                                            }
                                        {:case _}
                                            if (!("@{field.json_key}" in obj)) {
                                                return false;
                                            }
                                    {/match}
                                {/if}
                            {/for}
                        {/if}

                        return true;
                    }

                    export function fromJSON(data: unknown): @{interface_name} {
                        if (!is(data)) {
                            throw new Error("@{interface_name}.fromJSON: validation failed");
                        }

                        {#if has_transforms}
                            const obj = data as Record<string, unknown>;
                            return {
                                ...data,
                                {#for field in transform_fields}
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Date}
                                            {#if field.optional}
                                                @{field.field_name}: obj["@{field.json_key}"] !== undefined ? new Date(obj["@{field.json_key}"] as string) : undefined,
                                            {:else}
                                                @{field.field_name}: new Date(obj["@{field.json_key}"] as string),
                                            {/if}
                                        {:case TypeCategory::Map(key_type, value_type)}
                                            @{field.field_name}: new Map(Object.entries(obj["@{field.json_key}"] as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])),
                                        {:case TypeCategory::Set(inner)}
                                            @{field.field_name}: new Set(obj["@{field.json_key}"] as @{inner}[]),
                                        {:case TypeCategory::Serializable(type_name)}
                                            @{field.field_name}: typeof (@{type_name} as any)?.fromJSON === "function" ? (@{type_name} as any).fromJSON(obj["@{field.json_key}"]) : obj["@{field.json_key}"] as @{type_name},
                                        {:case _}
                                    {/match}
                                {/for}
                            };
                        {:else}
                            return data;
                        {/if}
                    }
                }
            })
        }
    }
}

/// Get JavaScript typeof string for a TypeScript primitive type
fn get_js_typeof(ts_type: &str) -> &'static str {
    match ts_type.trim() {
        "string" => "string",
        "number" => "number",
        "boolean" => "boolean",
        "bigint" => "bigint",
        _ => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_field_needs_transformation() {
        let field = DeserializeField {
            json_key: "date".into(),
            field_name: "date".into(),
            ts_type: "Date".into(),
            type_cat: TypeCategory::Date,
            optional: false,
            has_default: false,
            default_expr: None,
            flatten: false,
        };
        assert!(field.needs_transformation());

        let primitive_field = DeserializeField {
            json_key: "name".into(),
            field_name: "name".into(),
            ts_type: "string".into(),
            type_cat: TypeCategory::Primitive,
            optional: false,
            has_default: false,
            default_expr: None,
            flatten: false,
        };
        assert!(!primitive_field.needs_transformation());
    }

    #[test]
    fn test_get_js_typeof() {
        assert_eq!(get_js_typeof("string"), "string");
        assert_eq!(get_js_typeof("number"), "number");
        assert_eq!(get_js_typeof("boolean"), "boolean");
        assert_eq!(get_js_typeof("User"), "object");
    }
}
