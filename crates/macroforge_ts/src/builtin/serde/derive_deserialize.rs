//! /** @derive(Deserialize) */ macro implementation
//!
//! Generates JSON deserialization methods:
//! - For classes: `static fromJSON(data: unknown): Result<ClassName, string[]>`
//! - For interfaces: `namespace InterfaceName { export function is(data: unknown): data is InterfaceName; export function fromJSON(data: unknown): Result<InterfaceName, string[]> }`

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, TsStream, parse_ts_macro_input};

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory, Validator, ValidatorSpec};

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
    validators: Vec<ValidatorSpec>,
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

    /// Check if this field has any validators
    fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Generate JavaScript validation code for a single validator
/// Returns the condition that should trigger an error (when validation FAILS)
fn generate_validation_condition(validator: &Validator, value_var: &str) -> String {
    match validator {
        // String validators
        Validator::Email => {
            format!(r#"!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test({value_var})"#)
        }
        Validator::Url => {
            format!(
                r#"(() => {{ try {{ new URL({value_var}); return false; }} catch {{ return true; }} }})()"#
            )
        }
        Validator::Uuid => {
            format!(
                r#"!/^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[1-5][0-9a-f]{{3}}-[89ab][0-9a-f]{{3}}-[0-9a-f]{{12}}$/i.test({value_var})"#
            )
        }
        Validator::MaxLength(n) => format!("{value_var}.length > {n}"),
        Validator::MinLength(n) => format!("{value_var}.length < {n}"),
        Validator::Length(n) => format!("{value_var}.length !== {n}"),
        Validator::LengthRange(min, max) => {
            format!("{value_var}.length < {min} || {value_var}.length > {max}")
        }
        Validator::Pattern(regex) => {
            // Escape backslashes for JS regex
            let escaped = regex.replace('\\', "\\\\");
            format!("!/{escaped}/.test({value_var})")
        }
        Validator::NonEmpty => format!("{value_var}.length === 0"),
        Validator::Trimmed => format!("{value_var} !== {value_var}.trim()"),
        Validator::Lowercase => format!("{value_var} !== {value_var}.toLowerCase()"),
        Validator::Uppercase => format!("{value_var} !== {value_var}.toUpperCase()"),
        Validator::Capitalized => {
            format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toUpperCase()")
        }
        Validator::Uncapitalized => {
            format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toLowerCase()")
        }
        Validator::StartsWith(prefix) => format!(r#"!{value_var}.startsWith("{prefix}")"#),
        Validator::EndsWith(suffix) => format!(r#"!{value_var}.endsWith("{suffix}")"#),
        Validator::Includes(substr) => format!(r#"!{value_var}.includes("{substr}")"#),

        // Number validators
        Validator::GreaterThan(n) => format!("{value_var} <= {n}"),
        Validator::GreaterThanOrEqualTo(n) => format!("{value_var} < {n}"),
        Validator::LessThan(n) => format!("{value_var} >= {n}"),
        Validator::LessThanOrEqualTo(n) => format!("{value_var} > {n}"),
        Validator::Between(min, max) => format!("{value_var} < {min} || {value_var} > {max}"),
        Validator::Int => format!("!Number.isInteger({value_var})"),
        Validator::NonNaN => format!("Number.isNaN({value_var})"),
        Validator::Finite => format!("!Number.isFinite({value_var})"),
        Validator::Positive => format!("{value_var} <= 0"),
        Validator::NonNegative => format!("{value_var} < 0"),
        Validator::Negative => format!("{value_var} >= 0"),
        Validator::NonPositive => format!("{value_var} > 0"),
        Validator::MultipleOf(n) => format!("{value_var} % {n} !== 0"),
        Validator::Uint8 => {
            format!("!Number.isInteger({value_var}) || {value_var} < 0 || {value_var} > 255")
        }

        // Array validators
        Validator::MaxItems(n) => format!("{value_var}.length > {n}"),
        Validator::MinItems(n) => format!("{value_var}.length < {n}"),
        Validator::ItemsCount(n) => format!("{value_var}.length !== {n}"),

        // Date validators
        Validator::ValidDate => format!("isNaN({value_var}.getTime())"),
        Validator::GreaterThanDate(date) => {
            format!(r#"{value_var}.getTime() <= new Date("{date}").getTime()"#)
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!(r#"{value_var}.getTime() < new Date("{date}").getTime()"#)
        }
        Validator::LessThanDate(date) => {
            format!(r#"{value_var}.getTime() >= new Date("{date}").getTime()"#)
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!(r#"{value_var}.getTime() > new Date("{date}").getTime()"#)
        }
        Validator::BetweenDate(min, max) => {
            format!(
                r#"{value_var}.getTime() < new Date("{min}").getTime() || {value_var}.getTime() > new Date("{max}").getTime()"#
            )
        }

        // BigInt validators
        Validator::GreaterThanBigInt(n) => format!("{value_var} <= BigInt({n})"),
        Validator::GreaterThanOrEqualToBigInt(n) => format!("{value_var} < BigInt({n})"),
        Validator::LessThanBigInt(n) => format!("{value_var} >= BigInt({n})"),
        Validator::LessThanOrEqualToBigInt(n) => format!("{value_var} > BigInt({n})"),
        Validator::BetweenBigInt(min, max) => {
            format!("{value_var} < BigInt({min}) || {value_var} > BigInt({max})")
        }
        Validator::PositiveBigInt => format!("{value_var} <= 0n"),
        Validator::NonNegativeBigInt => format!("{value_var} < 0n"),
        Validator::NegativeBigInt => format!("{value_var} >= 0n"),
        Validator::NonPositiveBigInt => format!("{value_var} > 0n"),

        // Custom validator - handled specially
        Validator::Custom(_) => String::new(),
    }
}

/// Get default error message for a validator
fn get_validator_message(validator: &Validator) -> String {
    match validator {
        Validator::Email => "must be a valid email".to_string(),
        Validator::Url => "must be a valid URL".to_string(),
        Validator::Uuid => "must be a valid UUID".to_string(),
        Validator::MaxLength(n) => format!("must have at most {n} characters"),
        Validator::MinLength(n) => format!("must have at least {n} characters"),
        Validator::Length(n) => format!("must have exactly {n} characters"),
        Validator::LengthRange(min, max) => {
            format!("must have between {min} and {max} characters")
        }
        Validator::Pattern(_) => "must match the required pattern".to_string(),
        Validator::NonEmpty => "must not be empty".to_string(),
        Validator::Trimmed => "must be trimmed (no leading/trailing whitespace)".to_string(),
        Validator::Lowercase => "must be lowercase".to_string(),
        Validator::Uppercase => "must be uppercase".to_string(),
        Validator::Capitalized => "must be capitalized".to_string(),
        Validator::Uncapitalized => "must not be capitalized".to_string(),
        Validator::StartsWith(s) => format!("must start with '{s}'"),
        Validator::EndsWith(s) => format!("must end with '{s}'"),
        Validator::Includes(s) => format!("must include '{s}'"),
        Validator::GreaterThan(n) => format!("must be greater than {n}"),
        Validator::GreaterThanOrEqualTo(n) => format!("must be greater than or equal to {n}"),
        Validator::LessThan(n) => format!("must be less than {n}"),
        Validator::LessThanOrEqualTo(n) => format!("must be less than or equal to {n}"),
        Validator::Between(min, max) => format!("must be between {min} and {max}"),
        Validator::Int => "must be an integer".to_string(),
        Validator::NonNaN => "must not be NaN".to_string(),
        Validator::Finite => "must be finite".to_string(),
        Validator::Positive => "must be positive".to_string(),
        Validator::NonNegative => "must be non-negative".to_string(),
        Validator::Negative => "must be negative".to_string(),
        Validator::NonPositive => "must be non-positive".to_string(),
        Validator::MultipleOf(n) => format!("must be a multiple of {n}"),
        Validator::Uint8 => "must be a uint8 (0-255)".to_string(),
        Validator::MaxItems(n) => format!("must have at most {n} items"),
        Validator::MinItems(n) => format!("must have at least {n} items"),
        Validator::ItemsCount(n) => format!("must have exactly {n} items"),
        Validator::ValidDate => "must be a valid date".to_string(),
        Validator::GreaterThanDate(d) => format!("must be after {d}"),
        Validator::GreaterThanOrEqualToDate(d) => format!("must be on or after {d}"),
        Validator::LessThanDate(d) => format!("must be before {d}"),
        Validator::LessThanOrEqualToDate(d) => format!("must be on or before {d}"),
        Validator::BetweenDate(min, max) => format!("must be between {min} and {max}"),
        Validator::GreaterThanBigInt(n) => format!("must be greater than {n}"),
        Validator::GreaterThanOrEqualToBigInt(n) => format!("must be greater than or equal to {n}"),
        Validator::LessThanBigInt(n) => format!("must be less than {n}"),
        Validator::LessThanOrEqualToBigInt(n) => format!("must be less than or equal to {n}"),
        Validator::BetweenBigInt(min, max) => format!("must be between {min} and {max}"),
        Validator::PositiveBigInt => "must be positive".to_string(),
        Validator::NonNegativeBigInt => "must be non-negative".to_string(),
        Validator::NegativeBigInt => "must be negative".to_string(),
        Validator::NonPositiveBigInt => "must be non-positive".to_string(),
        Validator::Custom(_) => "failed custom validation".to_string(),
    }
}

/// Generate validation code snippet for a field
/// Returns a string of JS code that pushes errors to an 'errors' array
fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    json_key: &str,
    class_name: &str,
) -> String {
    let mut code = String::new();

    for spec in validators {
        let message = spec
            .custom_message
            .clone()
            .unwrap_or_else(|| get_validator_message(&spec.validator));

        let error_msg = format!("{class_name}.fromJSON: field '{json_key}' {message}");

        if let Validator::Custom(fn_name) = &spec.validator {
            // Custom validator - call function and check result
            code.push_str(&format!(
                r#"
                {{
                    const __customResult = {fn_name}({value_var});
                    if (__customResult === false) {{
                        errors.push("{error_msg}");
                    }}
                }}
"#
            ));
        } else {
            let condition = generate_validation_condition(&spec.validator, value_var);
            code.push_str(&format!(
                r#"
                if ({condition}) {{
                    errors.push("{error_msg}");
                }}
"#
            ));
        }
    }

    code
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
                        validators: opts.validators.clone(),
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

            let mut result = body! {
                static fromJSON(data: unknown): Result<@{class_name}, string[]> {
                    if (typeof data !== "object" || data === null || Array.isArray(data)) {
                        return Result.err(["@{class_name}.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data)]);
                    }

                    const obj = data as Record<string, unknown>;
                    const errors: string[] = [];

                    {#if deny_unknown}
                        const knownKeys = new Set([{#for key in known_keys}"@{key}", {/for}]);
                        for (const key of Object.keys(obj)) {
                            if (!knownKeys.has(key)) {
                                errors.push("@{class_name}.fromJSON: unknown field \"" + key + "\"");
                            }
                        }
                    {/if}

                    {#if has_required}
                        {#for field in &required_fields}
                            if (!("@{field.json_key}" in obj)) {
                                errors.push("@{class_name}.fromJSON: missing required field \"@{field.json_key}\"");
                            }
                        {/for}
                    {/if}

                    {#if has_required}
                        {#for field in &required_fields}
                            {%let raw_var = format!("__raw_{}", field.field_name)}
                            {%let val_var = format!("__val_{}", field.field_name)}
                            {%let has_validators = field.has_validators()}
                            {#match &field.type_cat}
                                {:case TypeCategory::Primitive}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#if has_validators}
                                        const @{val_var} = @{raw_var} as @{ts_type};
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {/if}

                                {:case TypeCategory::Date}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#if has_validators}
                                        let @{val_var}: Date;
                                        if (typeof @{raw_var} === "string") {
                                            @{val_var} = new Date(@{raw_var});
                                        } else if (@{raw_var} instanceof Date) {
                                            @{val_var} = @{raw_var};
                                        } else {
                                            errors.push("@{class_name}.fromJSON: field \"@{field.json_key}\" must be a Date or ISO string");
                                            @{val_var} = new Date();
                                        }
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {:else}
                                        if (typeof @{raw_var} !== "string" && !(@{raw_var} instanceof Date)) {
                                            errors.push("@{class_name}.fromJSON: field \"@{field.json_key}\" must be a Date or ISO string");
                                        }
                                    {/if}

                                {:case TypeCategory::Array(inner)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (!Array.isArray(@{raw_var})) {
                                        errors.push("@{class_name}.fromJSON: field \"@{field.json_key}\" must be an array");
                                    }
                                    {#if has_validators}
                                        const @{val_var} = @{raw_var} as @{inner}[];
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {/if}

                                {:case TypeCategory::Map(_key_type, _value_type)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (typeof @{raw_var} !== "object" || @{raw_var} === null) {
                                        errors.push("@{class_name}.fromJSON: field \"@{field.json_key}\" must be an object for Map");
                                    }

                                {:case TypeCategory::Set(inner)}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    if (!Array.isArray(@{raw_var})) {
                                        errors.push("@{class_name}.fromJSON: field \"@{field.json_key}\" must be an array for Set");
                                    }
                                    {#if has_validators}
                                        const @{val_var} = new Set(@{raw_var} as @{inner}[]);
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {/if}

                                {:case TypeCategory::Serializable(_type_name)}
                                    const @{raw_var} = obj["@{field.json_key}"];

                                {:case TypeCategory::Optional(_)}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#if has_validators}
                                        const @{val_var} = @{raw_var} as @{ts_type};
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {/if}

                                {:case TypeCategory::Nullable(_)}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#if has_validators}
                                        const @{val_var} = @{raw_var} as @{ts_type};
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {/if}

                                {:case TypeCategory::Unknown}
                                    {%let ts_type = &field.ts_type}
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#if has_validators}
                                        const @{val_var} = @{raw_var} as @{ts_type};
                                        {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                        @{validation_code}
                                    {/if}
                            {/match}
                        {/for}
                    {/if}

                    {#if has_optional}
                        {#for field in &optional_fields}
                            {%let raw_var = format!("__raw_{}", field.field_name)}
                            {%let val_var = format!("__val_{}", field.field_name)}
                            {%let has_validators = field.has_validators()}
                            if ("@{field.json_key}" in obj) {
                                const @{raw_var} = obj["@{field.json_key}"];
                                if (@{raw_var} !== undefined) {
                                    {#if has_validators}
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Primitive}
                                                {%let ts_type = &field.ts_type}
                                                const @{val_var} = @{raw_var} as @{ts_type};
                                                {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                                @{validation_code}

                                            {:case TypeCategory::Date}
                                                let @{val_var}: Date;
                                                if (typeof @{raw_var} === "string") {
                                                    @{val_var} = new Date(@{raw_var});
                                                } else if (@{raw_var} instanceof Date) {
                                                    @{val_var} = @{raw_var};
                                                } else {
                                                    @{val_var} = new Date();
                                                }
                                                {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                                @{validation_code}

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    const @{val_var} = @{raw_var} as @{inner}[];
                                                    {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                                    @{validation_code}
                                                }

                                            {:case _}
                                                {%let ts_type = &field.ts_type}
                                                const @{val_var} = @{raw_var} as @{ts_type};
                                                {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, class_name)}
                                                @{validation_code}
                                        {/match}
                                    {/if}
                                }
                            }
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        return Result.err(errors);
                    }

                    const instance = new @{class_name}();

                    {#if has_required}
                        {#for field in &required_fields}
                            {%let raw_var = format!("__raw_{}", field.field_name)}
                            {#match &field.type_cat}
                                {:case TypeCategory::Primitive}
                                    {%let ts_type = &field.ts_type}
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                {:case TypeCategory::Date}
                                    if (typeof @{raw_var} === "string") {
                                        instance.@{field.field_name} = new Date(@{raw_var});
                                    } else {
                                        instance.@{field.field_name} = @{raw_var} as Date;
                                    }

                                {:case TypeCategory::Array(inner)}
                                    instance.@{field.field_name} = (@{raw_var} as unknown[]).map((item) =>
                                        typeof (item as any)?.constructor?.fromJSON === "function"
                                            ? (item as any).constructor.fromJSON(item)
                                            : item as @{inner}
                                    );

                                {:case TypeCategory::Map(key_type, value_type)}
                                    instance.@{field.field_name} = new Map(
                                        Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                    );

                                {:case TypeCategory::Set(inner)}
                                    instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                {:case TypeCategory::Serializable(type_name)}
                                    if (typeof (@{type_name} as any)?.fromJSON === "function") {
                                        instance.@{field.field_name} = (@{type_name} as any).fromJSON(@{raw_var});
                                    } else {
                                        instance.@{field.field_name} = @{raw_var} as @{type_name};
                                    }

                                {:case TypeCategory::Optional(_)}
                                    {%let ts_type = &field.ts_type}
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                {:case TypeCategory::Nullable(_)}
                                    {%let ts_type = &field.ts_type}
                                    instance.@{field.field_name} = @{raw_var} as @{ts_type};

                                {:case TypeCategory::Unknown}
                                    {%let ts_type = &field.ts_type}
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

                    return Result.ok(instance);
                }
            };
            result.add_import("Result", "macroforge/result");
            Ok(result)
        }
        Data::Enum(_) => {
            // Enums: iterate through enum values at runtime to validate
            let enum_name = input.name();
            Ok(ts_template! {
                export namespace @{enum_name} {
                    export function fromJSON(data: unknown): @{enum_name} {
                        // Check if the value matches any enum member
                        for (const key of Object.keys(@{enum_name})) {
                            const enumValue = @{enum_name}[key as keyof typeof @{enum_name}];
                            if (enumValue === data) {
                                return data as @{enum_name};
                            }
                        }
                        throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
                    }
                }
            })
        }
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
                        validators: opts.validators.clone(),
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

            // Fields with validators for validation in fromJSON
            let validated_fields: Vec<_> = fields
                .iter()
                .filter(|f| !f.flatten && f.has_validators())
                .cloned()
                .collect();
            let has_validations = !validated_fields.is_empty();

            let mut result = ts_template! {
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

                    export function fromJSON(data: unknown): Result<@{interface_name}, string[]> {
                        if (!is(data)) {
                            return Result.err(["@{interface_name}.fromJSON: type validation failed"]);
                        }

                        const obj = data as Record<string, unknown>;
                        const errors: string[] = [];

                        {#if has_validations}
                            {#for field in &validated_fields}
                                {%let val_var = format!("__val_{}", field.field_name)}
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Date}
                                                const @{val_var} = typeof obj["@{field.json_key}"] === "string" ? new Date(obj["@{field.json_key}"] as string) : obj["@{field.json_key}"] as Date;
                                                {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, interface_name)}
                                                @{validation_code}
                                            {:case _}
                                                {%let ts_type = &field.ts_type}
                                                const @{val_var} = obj["@{field.json_key}"] as @{ts_type};
                                                {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, interface_name)}
                                                @{validation_code}
                                        {/match}
                                    }
                                {:else}
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Date}
                                            const @{val_var} = typeof obj["@{field.json_key}"] === "string" ? new Date(obj["@{field.json_key}"] as string) : obj["@{field.json_key}"] as Date;
                                            {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, interface_name)}
                                            @{validation_code}
                                        {:case _}
                                            {%let ts_type = &field.ts_type}
                                            const @{val_var} = obj["@{field.json_key}"] as @{ts_type};
                                            {%let validation_code = generate_field_validations(&field.validators, &val_var, &field.json_key, interface_name)}
                                            @{validation_code}
                                    {/match}
                                {/if}
                            {/for}
                        {/if}

                        if (errors.length > 0) {
                            return Result.err(errors);
                        }

                        {#if has_transforms}
                            return Result.ok({
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
                            });
                        {:else}
                            return Result.ok(data);
                        {/if}
                    }
                }
            };
            result.add_import("Result", "macroforge/result");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                // Object type: validate fields and return
                let field_names: Vec<&str> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|f| f.name.as_str())
                    .collect();
                let has_fields = !field_names.is_empty();

                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function is(data: unknown): data is @{type_name} {
                            if (typeof data !== "object" || data === null || Array.isArray(data)) {
                                return false;
                            }

                            {#if has_fields}
                                const obj = data as Record<string, unknown>;
                                {#for field in field_names}
                                    if (!("@{field}" in obj)) {
                                        return false;
                                    }
                                {/for}
                            {/if}

                            return true;
                        }

                        export function fromJSON(data: unknown): @{type_name} {
                            if (!is(data)) {
                                throw new Error("Invalid @{type_name}: type validation failed");
                            }
                            return data;
                        }
                    }
                })
            } else {
                // Union, intersection, tuple, or simple alias
                Ok(ts_template! {
                    export namespace @{type_name} {
                        export function fromJSON(data: unknown): @{type_name} {
                            // For unions and other complex types, return as-is
                            // Runtime validation should be done by the caller if needed
                            return data as @{type_name};
                        }
                    }
                })
            }
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
            validators: vec![],
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
            validators: vec![],
        };
        assert!(!primitive_field.needs_transformation());
    }

    #[test]
    fn test_validation_condition_generation() {
        // Test email validator
        let condition = generate_validation_condition(&Validator::Email, "value");
        assert!(condition.contains("test(value)"));

        // Test maxLength validator
        let condition = generate_validation_condition(&Validator::MaxLength(255), "str");
        assert_eq!(condition, "str.length > 255");

        // Test between validator
        let condition = generate_validation_condition(&Validator::Between(0.0, 100.0), "num");
        assert_eq!(condition, "num < 0 || num > 100");
    }

    #[test]
    fn test_validator_message() {
        assert_eq!(
            get_validator_message(&Validator::Email),
            "must be a valid email"
        );
        assert_eq!(
            get_validator_message(&Validator::MaxLength(255)),
            "must have at most 255 characters"
        );
        assert_eq!(
            get_validator_message(&Validator::Between(0.0, 100.0)),
            "must be between 0 and 100"
        );
    }

    #[test]
    fn test_get_js_typeof() {
        assert_eq!(get_js_typeof("string"), "string");
        assert_eq!(get_js_typeof("number"), "number");
        assert_eq!(get_js_typeof("boolean"), "boolean");
        assert_eq!(get_js_typeof("User"), "object");
    }
}
