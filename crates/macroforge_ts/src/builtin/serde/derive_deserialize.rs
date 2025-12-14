//! /** @derive(Deserialize) */ macro implementation
//!
//! Generates JSON deserialization methods with cycle/forward-reference support:
//! - For classes: `static fromStringifiedJSON(json: string, opts?)`, `static __deserialize(value, ctx)`
//! - For interfaces: `namespace InterfaceName { fromStringifiedJSON, __deserialize }`
//!
//! Uses deferred patching to handle cycles and forward references.

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::{Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_macro_input};
use crate::ts_syn::abi::DiagnosticCollector;

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory, Validator, ValidatorSpec};

/// Field info for deserialization
#[derive(Clone)]
struct DeserializeField {
    json_key: String,
    field_name: String,
    #[allow(dead_code)]
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
    fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Generate JavaScript validation code for a single validator
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

        // Date validators (null-safe: JSON.stringify converts Invalid Date to null)
        Validator::ValidDate => format!("{value_var} == null || isNaN({value_var}.getTime())"),
        Validator::GreaterThanDate(date) => {
            format!(r#"{value_var} == null || {value_var}.getTime() <= new Date("{date}").getTime()"#)
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!(r#"{value_var} == null || {value_var}.getTime() < new Date("{date}").getTime()"#)
        }
        Validator::LessThanDate(date) => {
            format!(r#"{value_var} == null || {value_var}.getTime() >= new Date("{date}").getTime()"#)
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!(r#"{value_var} == null || {value_var}.getTime() > new Date("{date}").getTime()"#)
        }
        Validator::BetweenDate(min, max) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() < new Date("{min}").getTime() || {value_var}.getTime() > new Date("{max}").getTime()"#
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
/// Generates code that pushes `{ field: string, message: string }` objects to the errors array
fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    json_key: &str,
    _class_name: &str,
) -> String {
    let mut code = String::new();

    for spec in validators {
        let message = spec
            .custom_message
            .clone()
            .unwrap_or_else(|| get_validator_message(&spec.validator));

        if let Validator::Custom(fn_name) = &spec.validator {
            code.push_str(&format!(
                r#"
                {{
                    const __customResult = {fn_name}({value_var});
                    if (__customResult === false) {{
                        errors.push({{ field: "{json_key}", message: "{message}" }});
                    }}
                }}
"#
            ));
        } else {
            let condition = generate_validation_condition(&spec.validator, value_var);
            code.push_str(&format!(
                r#"
                if ({condition}) {{
                    errors.push({{ field: "{json_key}", message: "{message}" }});
                }}
"#
            ));
        }
    }

    code
}

#[ts_macro_derive(
    Deserialize,
    description = "Generates deserialization methods with cycle/forward-reference support (fromStringifiedJSON, __deserialize)",
    attributes((serde, "Configure deserialization for this field. Options: skip, rename, flatten, default, validate"))
)]
pub fn derive_deserialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Check for user-defined constructor with parameters
            if let Some(ctor) = class.method("constructor")
                && !ctor.params_src.trim().is_empty()
            {
                return Err(MacroforgeError::new(
                    ctor.span,
                    format!(
                        "@Derive(Deserialize) cannot be used on class '{}' with a custom constructor. \
                            Remove the constructor or use @Derive(Deserialize) on a class without a constructor.",
                        class_name
                    ),
                ));
            }

            // Collect deserializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<DeserializeField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result = SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

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
            let _has_optional = !optional_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();
            let deny_unknown = container_opts.deny_unknown_fields;

            // All non-flatten fields for assignments
            let all_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let has_fields = !all_fields.is_empty();

            let mut result = body! {
                constructor(props: { {#for field in &all_fields} @{field.field_name}{#if field.optional}?{/if}: @{field.ts_type}; {/for} }) {
                    {#for field in &all_fields}
                        this.@{field.field_name} = props.@{field.field_name}{#if field.optional} as @{field.ts_type}{/if};
                    {/for}
                }

                static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<@{class_name}, Array<{ field: string; message: string }>> {
                    try {
                        const raw = JSON.parse(json);
                        return @{class_name}.fromObject(raw, opts);
                    } catch (e) {
                        if (e instanceof DeserializeError) {
                            return Result.err(e.errors);
                        }
                        const message = e instanceof Error ? e.message : String(e);
                        return Result.err([{ field: "_root", message }]);
                    }
                }

                static fromObject(obj: unknown, opts?: DeserializeOptions): Result<@{class_name}, Array<{ field: string; message: string }>> {
                    try {
                        const ctx = DeserializeContext.create();
                        const resultOrRef = @{class_name}.__deserialize(obj, ctx);

                        if (PendingRef.is(resultOrRef)) {
                            return Result.err([{ field: "_root", message: "@{class_name}.fromObject: root cannot be a forward reference" }]);
                        }

                        ctx.applyPatches();
                        if (opts?.freeze) {
                            ctx.freezeAll();
                        }

                        return Result.ok(resultOrRef);
                    } catch (e) {
                        if (e instanceof DeserializeError) {
                            return Result.err(e.errors);
                        }
                        const message = e instanceof Error ? e.message : String(e);
                        return Result.err([{ field: "_root", message }]);
                    }
                }

                static __deserialize(value: any, ctx: DeserializeContext): @{class_name} | PendingRef {
                    // Handle reference to already-deserialized object
                    if (value?.__ref !== undefined) {
                        return ctx.getOrDefer(value.__ref);
                    }

                    if (typeof value !== "object" || value === null || Array.isArray(value)) {
                        throw new DeserializeError([{ field: "_root", message: "@{class_name}.__deserialize: expected an object" }]);
                    }

                    const obj = value as Record<string, unknown>;
                    const errors: Array<{ field: string; message: string }> = [];

                    {#if deny_unknown}
                        const knownKeys = new Set(["__type", "__id", "__ref", {#for key in known_keys}"@{key}", {/for}]);
                        for (const key of Object.keys(obj)) {
                            if (!knownKeys.has(key)) {
                                errors.push({ field: key, message: "unknown field" });
                            }
                        }
                    {/if}

                    {#if has_required}
                        {#for field in &required_fields}
                            if (!("@{field.json_key}" in obj)) {
                                errors.push({ field: "@{field.json_key}", message: "missing required field" });
                            }
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        throw new DeserializeError(errors);
                    }

                    // Create instance using Object.create to avoid constructor
                    const instance = Object.create(@{class_name}.prototype) as @{class_name};

                    // Register with context if __id is present
                    if (obj.__id !== undefined) {
                        ctx.register(obj.__id as number, instance);
                    }

                    // Track for optional freezing
                    ctx.trackForFreeze(instance);

                    // Assign fields
                    {#if has_fields}
                        {#for field in all_fields}
                            {$let raw_var = format!("__raw_{}", field.field_name)}
                            {$let has_validators = field.has_validators()}
                            {#if field.optional}
                                if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                @{validation_code}
                                            {/if}
                                            (instance as any).@{field.field_name} = @{raw_var};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                (instance as any).@{field.field_name} = __dateVal;
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                const __arr = (@{raw_var} as any[]).map((item, idx) => {
                                                    if (typeof item?.__deserialize === "function") {
                                                        const result = item.__deserialize(item, ctx);
                                                        if (PendingRef.is(result)) {
                                                            ctx.addPatch((instance as any).@{field.field_name}, idx, result.id);
                                                            return null;
                                                        }
                                                        return result;
                                                    }
                                                    // Check for __ref in array items
                                                    if (item?.__ref !== undefined) {
                                                        const result = ctx.getOrDefer(item.__ref);
                                                        if (PendingRef.is(result)) {
                                                            // Will be patched after array is assigned
                                                            return { __pendingIdx: idx, __refId: result.id };
                                                        }
                                                        return result;
                                                    }
                                                    return item as @{inner};
                                                });
                                                (instance as any).@{field.field_name} = __arr;
                                                // Patch array items that were pending
                                                __arr.forEach((item, idx) => {
                                                    if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                        ctx.addPatch((instance as any).@{field.field_name}, idx, (item as any).__refId);
                                                    }
                                                });
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                (instance as any).@{field.field_name} = new Map(
                                                    Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                );
                                            }

                                        {:case TypeCategory::Set(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                (instance as any).@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                            }

                                        {:case TypeCategory::Serializable(type_name)}
                                            if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } else {
                                                (instance as any).@{field.field_name} = @{raw_var};
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            if (@{raw_var} === null) {
                                                (instance as any).@{field.field_name} = null;
                                            } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } else {
                                                (instance as any).@{field.field_name} = @{raw_var};
                                            }

                                        {:case _}
                                            (instance as any).@{field.field_name} = @{raw_var};
                                    {/match}
                                }
                                {#if let Some(default_expr) = &field.default_expr}
                                    else {
                                        (instance as any).@{field.field_name} = @{default_expr};
                                    }
                                {/if}
                            {:else}
                                {
                                    const @{raw_var} = obj["@{field.json_key}"];
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                @{validation_code}
                                            {/if}
                                            (instance as any).@{field.field_name} = @{raw_var};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                (instance as any).@{field.field_name} = __dateVal;
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                const __arr = (@{raw_var} as any[]).map((item, idx) => {
                                                    if (item?.__ref !== undefined) {
                                                        const result = ctx.getOrDefer(item.__ref);
                                                        if (PendingRef.is(result)) {
                                                            return { __pendingIdx: idx, __refId: result.id };
                                                        }
                                                        return result;
                                                    }
                                                    return item as @{inner};
                                                });
                                                (instance as any).@{field.field_name} = __arr;
                                                __arr.forEach((item, idx) => {
                                                    if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                        ctx.addPatch((instance as any).@{field.field_name}, idx, (item as any).__refId);
                                                    }
                                                });
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            (instance as any).@{field.field_name} = new Map(
                                                Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                            );

                                        {:case TypeCategory::Set(inner)}
                                            (instance as any).@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                        {:case TypeCategory::Serializable(type_name)}
                                            if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } else {
                                                (instance as any).@{field.field_name} = @{raw_var};
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            if (@{raw_var} === null) {
                                                (instance as any).@{field.field_name} = null;
                                            } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            } else {
                                                (instance as any).@{field.field_name} = @{raw_var};
                                            }

                                        {:case _}
                                            (instance as any).@{field.field_name} = @{raw_var};
                                    {/match}
                                }
                            {/if}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                        const __result = (@{type_name} as any).__deserialize(obj, ctx);
                                        ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                    }
                                {:case _}
                                    (instance as any).@{field.field_name} = obj as any;
                            {/match}
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        throw new DeserializeError(errors);
                    }

                    return instance;
                }
            };
            result.add_import("Result", "macroforge/utils");
            result.add_import("DeserializeContext", "macroforge/serde");
            result.add_import("DeserializeError", "macroforge/serde");
            result.add_type_import("DeserializeOptions", "macroforge/serde");
            result.add_import("PendingRef", "macroforge/serde");
            Ok(result)
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let mut result = ts_template! {
                export namespace @{enum_name} {
                    export function fromStringifiedJSON(json: string): @{enum_name} {
                        const data = JSON.parse(json);
                        return __deserialize(data);
                    }

                    export function __deserialize(data: unknown): @{enum_name} {
                        for (const key of Object.keys(@{enum_name})) {
                            const enumValue = @{enum_name}[key as keyof typeof @{enum_name}];
                            if (enumValue === data) {
                                return data as @{enum_name};
                            }
                        }
                        throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
                    }
                }
            };
            result.add_import("DeserializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect deserializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<DeserializeField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result = SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

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

            let has_required = !required_fields.is_empty();
            let has_fields = !all_fields.is_empty();
            let deny_unknown = container_opts.deny_unknown_fields;

            let mut result = ts_template! {
                export namespace @{interface_name} {
                    export function fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<@{interface_name}, Array<{ field: string; message: string }>> {
                        try {
                            const raw = JSON.parse(json);
                            return fromObject(raw, opts);
                        } catch (e) {
                            if (e instanceof DeserializeError) {
                                return Result.err(e.errors);
                            }
                            const message = e instanceof Error ? e.message : String(e);
                            return Result.err([{ field: "_root", message }]);
                        }
                    }

                    export function fromObject(obj: unknown, opts?: DeserializeOptions): Result<@{interface_name}, Array<{ field: string; message: string }>> {
                        try {
                            const ctx = DeserializeContext.create();
                            const resultOrRef = __deserialize(obj, ctx);

                            if (PendingRef.is(resultOrRef)) {
                                return Result.err([{ field: "_root", message: "@{interface_name}.fromObject: root cannot be a forward reference" }]);
                            }

                            ctx.applyPatches();
                            if (opts?.freeze) {
                                ctx.freezeAll();
                            }

                            return Result.ok(resultOrRef);
                        } catch (e) {
                            if (e instanceof DeserializeError) {
                                return Result.err(e.errors);
                            }
                            const message = e instanceof Error ? e.message : String(e);
                            return Result.err([{ field: "_root", message }]);
                        }
                    }

                    export function __deserialize(value: any, ctx: DeserializeContext): @{interface_name} | PendingRef {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref);
                        }

                        if (typeof value !== "object" || value === null || Array.isArray(value)) {
                            throw new DeserializeError([{ field: "_root", message: "@{interface_name}.__deserialize: expected an object" }]);
                        }

                        const obj = value as Record<string, unknown>;
                        const errors: Array<{ field: string; message: string }> = [];

                        {#if deny_unknown}
                            const knownKeys = new Set(["__type", "__id", "__ref", {#for key in known_keys}"@{key}", {/for}]);
                            for (const key of Object.keys(obj)) {
                                if (!knownKeys.has(key)) {
                                    errors.push({ field: key, message: "unknown field" });
                                }
                            }
                        {/if}

                        {#if has_required}
                            {#for field in &required_fields}
                                if (!("@{field.json_key}" in obj)) {
                                    errors.push({ field: "@{field.json_key}", message: "missing required field" });
                                }
                            {/for}
                        {/if}

                        if (errors.length > 0) {
                            throw new DeserializeError(errors);
                        }

                        const instance: any = {};

                        if (obj.__id !== undefined) {
                            ctx.register(obj.__id as number, instance);
                        }

                        ctx.trackForFreeze(instance);

                        {#if has_fields}
                            {#for field in all_fields}
                                {$let raw_var = format!("__raw_{}", field.field_name)}
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        const @{raw_var} = obj["@{field.json_key}"];
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Date}
                                                instance.@{field.field_name} = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var};

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                instance.@{field.field_name} = new Map(
                                                    Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                );

                                            {:case TypeCategory::Set(inner)}
                                                instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                            {:case TypeCategory::Serializable(type_name)}
                                                if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                    const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                } else {
                                                    instance.@{field.field_name} = @{raw_var};
                                                }

                                            {:case _}
                                                instance.@{field.field_name} = @{raw_var};
                                        {/match}
                                    }
                                    {#if let Some(default_expr) = &field.default_expr}
                                        else {
                                            instance.@{field.field_name} = @{default_expr};
                                        }
                                    {/if}
                                {:else}
                                    {
                                        const @{raw_var} = obj["@{field.json_key}"];
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Date}
                                                instance.@{field.field_name} = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var};

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                instance.@{field.field_name} = new Map(
                                                    Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                );

                                            {:case TypeCategory::Set(inner)}
                                                instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                            {:case TypeCategory::Serializable(type_name)}
                                                if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                    const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                } else {
                                                    instance.@{field.field_name} = @{raw_var};
                                                }

                                            {:case _}
                                                instance.@{field.field_name} = @{raw_var};
                                        {/match}
                                    }
                                {/if}
                            {/for}
                        {/if}

                        if (errors.length > 0) {
                            throw new DeserializeError(errors);
                        }

                        return instance as @{interface_name};
                    }
                }
            };
            result.add_import("Result", "macroforge/utils");
            result.add_import("DeserializeContext", "macroforge/serde");
            result.add_import("DeserializeError", "macroforge/serde");
            result.add_type_import("DeserializeOptions", "macroforge/serde");
            result.add_import("PendingRef", "macroforge/serde");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            if type_alias.is_object() {
                let mut result = ts_template! {
                    export namespace @{type_name} {
                        export function fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<@{type_name}, Array<{ field: string; message: string }>> {
                            try {
                                const raw = JSON.parse(json);
                                return fromObject(raw, opts);
                            } catch (e) {
                                if (e instanceof DeserializeError) {
                                    return Result.err(e.errors);
                                }
                                const message = e instanceof Error ? e.message : String(e);
                                return Result.err([{ field: "_root", message }]);
                            }
                        }

                        export function fromObject(obj: unknown, opts?: DeserializeOptions): Result<@{type_name}, Array<{ field: string; message: string }>> {
                            try {
                                const ctx = DeserializeContext.create();
                                const result = __deserialize(obj, ctx);
                                ctx.applyPatches();
                                if (opts?.freeze) {
                                    ctx.freezeAll();
                                }
                                return Result.ok(result);
                            } catch (e) {
                                if (e instanceof DeserializeError) {
                                    return Result.err(e.errors);
                                }
                                const message = e instanceof Error ? e.message : String(e);
                                return Result.err([{ field: "_root", message }]);
                            }
                        }

                        export function __deserialize(value: any, ctx: DeserializeContext): @{type_name} {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{type_name};
                            }

                            const instance = { ...value };
                            delete instance.__type;
                            delete instance.__id;

                            if (value.__id !== undefined) {
                                ctx.register(value.__id as number, instance);
                            }

                            ctx.trackForFreeze(instance);
                            return instance as @{type_name};
                        }
                    }
                };
                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                Ok(result)
            } else {
                // Union type (including string literal unions) - dispatch based on __type or return as-is
                let mut result = ts_template! {
                    export namespace @{type_name} {
                        export function fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<@{type_name}, Array<{ field: string; message: string }>> {
                            try {
                                const raw = JSON.parse(json);
                                return fromObject(raw, opts);
                            } catch (e) {
                                if (e instanceof DeserializeError) {
                                    return Result.err(e.errors);
                                }
                                const message = e instanceof Error ? e.message : String(e);
                                return Result.err([{ field: "_root", message }]);
                            }
                        }

                        export function fromObject(obj: unknown, opts?: DeserializeOptions): Result<@{type_name}, Array<{ field: string; message: string }>> {
                            try {
                                const ctx = DeserializeContext.create();
                                const result = __deserialize(obj, ctx);
                                ctx.applyPatches();
                                if (opts?.freeze) {
                                    ctx.freezeAll();
                                }
                                return Result.ok(result);
                            } catch (e) {
                                if (e instanceof DeserializeError) {
                                    return Result.err(e.errors);
                                }
                                const message = e instanceof Error ? e.message : String(e);
                                return Result.err([{ field: "_root", message }]);
                            }
                        }

                        export function __deserialize(value: any, ctx: DeserializeContext): @{type_name} {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{type_name};
                            }

                            // For union types with __type, delegate to the appropriate type
                            if (typeof (value as any)?.__type === "string") {
                                // Look up deserializer by type name
                                // This requires the types in the union to be imported and have __deserialize
                                throw new Error("@{type_name}.__deserialize: polymorphic deserialization requires type registry (TODO)");
                            }

                            return value as @{type_name};
                        }
                    }
                };
                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                Ok(result)
            }
        }
    }
}

/// Get JavaScript typeof string for a TypeScript primitive type
#[allow(dead_code)]
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
    fn test_deserialize_field_has_validators() {
        let field = DeserializeField {
            json_key: "email".into(),
            field_name: "email".into(),
            ts_type: "string".into(),
            type_cat: TypeCategory::Primitive,
            optional: false,
            has_default: false,
            default_expr: None,
            flatten: false,
            validators: vec![ValidatorSpec {
                validator: Validator::Email,
                custom_message: None,
            }],
        };
        assert!(field.has_validators());

        let field_no_validators = DeserializeField {
            validators: vec![],
            ..field
        };
        assert!(!field_no_validators.has_validators());
    }

    #[test]
    fn test_validation_condition_generation() {
        let condition = generate_validation_condition(&Validator::Email, "value");
        assert!(condition.contains("test(value)"));

        let condition = generate_validation_condition(&Validator::MaxLength(255), "str");
        assert_eq!(condition, "str.length > 255");
    }

    #[test]
    fn test_validator_message() {
        assert_eq!(
            get_validator_message(&Validator::Email),
            "must be a valid email"
        );
    }
}
