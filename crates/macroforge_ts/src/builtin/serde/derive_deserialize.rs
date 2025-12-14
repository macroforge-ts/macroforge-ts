//! # Deserialize Macro Implementation
//!
//! The `Deserialize` macro generates JSON deserialization methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! ## Generated Methods
//!
//! | Type | Generated Methods | Description |
//! |------|-------------------|-------------|
//! | Class | `static fromStringifiedJSON()`, `static fromObject()`, `static __deserialize()` | Static factory methods |
//! | Enum | `EnumName.fromStringifiedJSON()`, `__deserialize()` | Namespace functions |
//! | Interface | `InterfaceName.fromStringifiedJSON()`, etc. | Namespace functions |
//! | Type Alias | `TypeName.fromStringifiedJSON()`, etc. | Namespace functions |
//!
//! ## Return Type
//!
//! All public deserialization methods return `Result<T, Array<{ field: string; message: string }>>`:
//!
//! - `Result.ok(value)` - Successfully deserialized value
//! - `Result.err(errors)` - Array of validation errors with field names and messages
//!
//! ## Cycle/Forward-Reference Support
//!
//! Uses deferred patching to handle references:
//!
//! 1. When encountering `{ "__ref": id }`, returns a `PendingRef` marker
//! 2. Continues deserializing other fields
//! 3. After all objects are created, `ctx.applyPatches()` resolves all pending references
//!
//! ## Validation
//!
//! The macro supports 30+ validators via `@serde(validate(...))`:
//!
//! ### String Validators
//! - `email`, `url`, `uuid` - Format validation
//! - `minLength(n)`, `maxLength(n)`, `length(n)` - Length constraints
//! - `pattern("regex")` - Regular expression matching
//! - `nonEmpty`, `trimmed`, `lowercase`, `uppercase` - String properties
//!
//! ### Number Validators
//! - `gt(n)`, `gte(n)`, `lt(n)`, `lte(n)`, `between(min, max)` - Range checks
//! - `int`, `positive`, `nonNegative`, `finite` - Number properties
//!
//! ### Array Validators
//! - `minItems(n)`, `maxItems(n)`, `itemsCount(n)` - Collection size
//!
//! ### Date Validators
//! - `validDate`, `afterDate("ISO")`, `beforeDate("ISO")` - Date validation
//!
//! ## Field-Level Options
//!
//! The `@serde` decorator supports:
//!
//! - `skip` / `skip_deserializing` - Exclude field from deserialization
//! - `rename = "jsonKey"` - Read from different JSON property
//! - `default` / `default = expr` - Use default value if missing
//! - `flatten` - Read fields from parent object level
//! - `validate(...)` - Apply validators
//!
//! ## Container-Level Options
//!
//! - `deny_unknown_fields` - Error on unrecognized JSON properties
//! - `rename_all = "camelCase"` - Apply naming convention to all fields
//!
//! ## Example
//!
//! ```typescript
//! @derive(Deserialize)
//! @serde(deny_unknown_fields)
//! class User {
//!     id: number;
//!
//!     @serde(validate(email, maxLength(255)))
//!     email: string;
//!
//!     @serde(default = "guest")
//!     name: string;
//!
//!     @serde(validate(positive))
//!     age?: number;
//! }
//!
//! // Usage:
//! const result = User.fromStringifiedJSON('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error);  // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! ## Required Imports
//!
//! The generated code automatically imports:
//! - `Result` from `macroforge/utils`
//! - `DeserializeContext`, `DeserializeError`, `PendingRef` from `macroforge/serde`

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    parse_ts_macro_input, Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream,
};

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory, Validator, ValidatorSpec};

/// Contains field information needed for JSON deserialization code generation.
///
/// Each field that should be deserialized is represented by this struct,
/// capturing all the information needed to generate parsing, validation,
/// and assignment code.
#[derive(Clone)]
struct DeserializeField {
    /// The JSON property name to read from the input object.
    /// This may differ from `field_name` if `@serde(rename = "...")` is used.
    json_key: String,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property assignments like `instance.fieldName = value`.
    field_name: String,

    /// The TypeScript type annotation string (e.g., "string", "number[]").
    /// Used for type casting in generated code.
    #[allow(dead_code)]
    ts_type: String,

    /// The category of the field's type, used to select the appropriate
    /// deserialization strategy (primitive, Date, Array, Map, Set, etc.).
    type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier or `@serde(default)`).
    /// Optional fields don't require the JSON property to be present.
    optional: bool,

    /// Whether the field has a default value specified.
    #[allow(dead_code)]
    has_default: bool,

    /// The default value expression to use if the field is missing.
    /// Example: `Some("\"guest\"".to_string())` for `@serde(default = "guest")`.
    default_expr: Option<String>,

    /// Whether the field should be read from the parent object level.
    /// Flattened fields look for their properties directly in the parent JSON.
    flatten: bool,

    /// List of validators to apply after parsing the field value.
    /// Each validator generates a condition check and error message.
    validators: Vec<ValidatorSpec>,
}

impl DeserializeField {
    /// Returns true if this field has any validators that need to be applied.
    fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Generates a JavaScript boolean expression that evaluates to `true` when validation fails.
///
/// This function produces the *failure condition* - the expression should be used in
/// `if (condition) { errors.push(...) }` to detect invalid values.
///
/// # Arguments
///
/// * `validator` - The validator type to generate a condition for
/// * `value_var` - The variable name containing the value to validate
///
/// # Returns
///
/// A string containing a JavaScript boolean expression. The expression evaluates to
/// `true` when the value is **invalid** (fails validation).
///
/// # Example
///
/// ```ignore
/// let condition = generate_validation_condition(&Validator::Email, "email");
/// // Returns: "!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)"
///
/// let condition = generate_validation_condition(&Validator::MaxLength(100), "name");
/// // Returns: "name.length > 100"
/// ```
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

/// Returns the default human-readable error message for a validator.
///
/// These messages are used when no custom message is provided via
/// `@serde(validate(..., message = "custom"))`.
///
/// # Arguments
///
/// * `validator` - The validator to get a message for
///
/// # Returns
///
/// A user-friendly error message describing the validation requirement.
///
/// # Example Messages
///
/// - `Validator::Email` → "must be a valid email"
/// - `Validator::MaxLength(100)` → "must have at most 100 characters"
/// - `Validator::Between(1, 10)` → "must be between 1 and 10"
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

/// Generates JavaScript code that validates a field and collects errors.
///
/// This function produces a series of `if` statements that check each validator
/// and push error objects to the `errors` array when validation fails.
///
/// # Arguments
///
/// * `validators` - List of validators to apply, each with optional custom message
/// * `value_var` - The variable name containing the value to validate
/// * `json_key` - The JSON property name (used in error messages)
/// * `_class_name` - The class name (reserved for future use)
///
/// # Returns
///
/// A string containing JavaScript code that performs validation checks.
/// The generated code assumes an `errors` array is in scope.
///
/// # Example Output
///
/// ```javascript
/// if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
///     errors.push({ field: "email", message: "must be a valid email" });
/// }
/// if (email.length > 255) {
///     errors.push({ field: "email", message: "must have at most 255 characters" });
/// }
/// ```
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

            // Fields with validators for per-field validation
            let fields_with_validators: Vec<_> = all_fields
                .iter()
                .filter(|f| f.has_validators())
                .cloned()
                .collect();
            let has_validators = !fields_with_validators.is_empty();

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
                                            instance.@{field.field_name} = @{raw_var};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = __dateVal;
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
                                                            ctx.deferPatch(result.id, (v) => { instance.@{field.field_name}[idx] = v; });
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
                                                instance.@{field.field_name} = __arr;
                                                // Patch array items that were pending
                                                __arr.forEach((item, idx) => {
                                                    if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                        ctx.deferPatch((item as any).__refId, (v) => { instance.@{field.field_name}[idx] = v; });
                                                    }
                                                });
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                instance.@{field.field_name} = new Map(
                                                    Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                );
                                            }

                                        {:case TypeCategory::Set(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                            }

                                        {:case TypeCategory::Serializable(type_name)}
                                            if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                if (PendingRef.is(__result)) {
                                                    instance.@{field.field_name} = null as @{field.ts_type};
                                                    ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                                } else {
                                                    instance.@{field.field_name} = __result;
                                                }
                                            } else {
                                                instance.@{field.field_name} = @{raw_var};
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            if (@{raw_var} === null) {
                                                instance.@{field.field_name} = null;
                                            } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
                                                if (PendingRef.is(__result)) {
                                                    instance.@{field.field_name} = null as @{field.ts_type};
                                                    ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                                } else {
                                                    instance.@{field.field_name} = __result;
                                                }
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
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                @{validation_code}
                                            {/if}
                                            instance.@{field.field_name} = @{raw_var};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = __dateVal;
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
                                                instance.@{field.field_name} = __arr;
                                                __arr.forEach((item, idx) => {
                                                    if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                        ctx.deferPatch((item as any).__refId, (v) => { instance.@{field.field_name}[idx] = v; });
                                                    }
                                                });
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            instance.@{field.field_name} = new Map(
                                                Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                            );

                                        {:case TypeCategory::Set(inner)}
                                            instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                        {:case TypeCategory::Serializable(type_name)}
                                            if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                if (PendingRef.is(__result)) {
                                                    instance.@{field.field_name} = null as @{field.ts_type};
                                                    ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                                } else {
                                                    instance.@{field.field_name} = __result;
                                                }
                                            } else {
                                                instance.@{field.field_name} = @{raw_var};
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            if (@{raw_var} === null) {
                                                instance.@{field.field_name} = null;
                                            } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
                                                if (PendingRef.is(__result)) {
                                                    instance.@{field.field_name} = null as @{field.ts_type};
                                                    ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                                } else {
                                                    instance.@{field.field_name} = __result;
                                                }
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

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                        const __result = (@{type_name} as any).__deserialize(obj, ctx);
                                        if (PendingRef.is(__result)) {
                                            instance.@{field.field_name} = null as @{field.ts_type};
                                            ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                        } else {
                                            instance.@{field.field_name} = __result;
                                        }
                                    }
                                {:case _}
                                    instance.@{field.field_name} = obj as any;
                            {/match}
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        throw new DeserializeError(errors);
                    }

                    return instance;
                }

                static validateField<K extends keyof @{class_name}>(
                    field: K,
                    value: @{class_name}[K]
                ): Array<{ field: string; message: string }> {
                    {#if has_validators}
                    const errors: Array<{ field: string; message: string }> = [];
                    switch (field) {
                        {#for field in &fields_with_validators}
                        case "@{field.field_name}": {
                            const __val = value as @{field.ts_type};
                            {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name)}
                            @{validation_code}
                            break;
                        }
                        {/for}
                    }
                    return errors;
                    {:else}
                    return [];
                    {/if}
                }

                static validateFields(
                    partial: Partial<@{class_name}>
                ): Array<{ field: string; message: string }> {
                    {#if has_validators}
                    const errors: Array<{ field: string; message: string }> = [];
                    {#for field in &fields_with_validators}
                    if ("@{field.field_name}" in partial && partial.@{field.field_name} !== undefined) {
                        const __val = partial.@{field.field_name} as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name)}
                        @{validation_code}
                    }
                    {/for}
                    return errors;
                    {:else}
                    return [];
                    {/if}
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

            // Fields with validators for per-field validation
            let fields_with_validators: Vec<_> = all_fields
                .iter()
                .filter(|f| f.has_validators())
                .cloned()
                .collect();
            let has_validators = !fields_with_validators.is_empty();

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
                                {$let has_validators = field.has_validators()}
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        const @{raw_var} = obj["@{field.json_key}"];
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Primitive}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = @{raw_var};

                                            {:case TypeCategory::Date}
                                                {
                                                    const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = __dateVal;
                                                }

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                }

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                    instance.@{field.field_name} = new Map(
                                                        Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                }

                                            {:case TypeCategory::Set(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                }

                                            {:case TypeCategory::Serializable(type_name)}
                                                if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                    const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                    if (PendingRef.is(__result)) {
                                                        instance.@{field.field_name} = null;
                                                        ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                                    } else {
                                                        instance.@{field.field_name} = __result;
                                                    }
                                                } else {
                                                    instance.@{field.field_name} = @{raw_var};
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                if (@{raw_var} === null) {
                                                    instance.@{field.field_name} = null;
                                                } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                    const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
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
                                            {:case TypeCategory::Primitive}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = @{raw_var};

                                            {:case TypeCategory::Date}
                                                {
                                                    const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = __dateVal;
                                                }

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                }

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                    instance.@{field.field_name} = new Map(
                                                        Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                }

                                            {:case TypeCategory::Set(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                }

                                            {:case TypeCategory::Serializable(type_name)}
                                                if (typeof (@{type_name} as any)?.__deserialize === "function") {
                                                    const __result = (@{type_name} as any).__deserialize(@{raw_var}, ctx);
                                                    if (PendingRef.is(__result)) {
                                                        instance.@{field.field_name} = null;
                                                        ctx.deferPatch(__result.id, (v) => { instance.@{field.field_name} = v; });
                                                    } else {
                                                        instance.@{field.field_name} = __result;
                                                    }
                                                } else {
                                                    instance.@{field.field_name} = @{raw_var};
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                if (@{raw_var} === null) {
                                                    instance.@{field.field_name} = null;
                                                } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                    const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
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

                    export function validateField<K extends keyof @{interface_name}>(
                        field: K,
                        value: @{interface_name}[K]
                    ): Array<{ field: string; message: string }> {
                        {#if has_validators}
                        const errors: Array<{ field: string; message: string }> = [];
                        switch (field) {
                            {#for field in &fields_with_validators}
                            case "@{field.field_name}": {
                                const __val = value as @{field.ts_type};
                                {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                                @{validation_code}
                                break;
                            }
                            {/for}
                        }
                        return errors;
                        {:else}
                        return [];
                        {/if}
                    }

                    export function validateFields(
                        partial: Partial<@{interface_name}>
                    ): Array<{ field: string; message: string }> {
                        {#if has_validators}
                        const errors: Array<{ field: string; message: string }> = [];
                        {#for field in &fields_with_validators}
                        if ("@{field.field_name}" in partial && partial.@{field.field_name} !== undefined) {
                            const __val = partial.@{field.field_name} as @{field.ts_type};
                            {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                            @{validation_code}
                        }
                        {/for}
                        return errors;
                        {:else}
                        return [];
                        {/if}
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

            if type_alias.is_object() {
                let container_opts =
                    SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);

                // Collect deserializable fields with diagnostic collection
                let mut all_diagnostics = DiagnosticCollector::new();
                let fields: Vec<DeserializeField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let parse_result =
                            SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
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

                // Fields with validators for per-field validation
                let fields_with_validators: Vec<_> = all_fields
                    .iter()
                    .filter(|f| f.has_validators())
                    .cloned()
                    .collect();
                let has_validators = !fields_with_validators.is_empty();

                let mut result = ts_template! {
                    export namespace @{type_name} {
                        export function {|fromStringifiedJSON@{generic_decl}|}(json: string, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
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

                        export function {|fromObject@{generic_decl}|}(obj: unknown, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
                            try {
                                const ctx = DeserializeContext.create();
                                const resultOrRef = __deserialize(obj, ctx);

                                if (PendingRef.is(resultOrRef)) {
                                    return Result.err([{ field: "_root", message: "@{type_name}.fromObject: root cannot be a forward reference" }]);
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

                        export function __deserialize(value: any, ctx: DeserializeContext): @{type_name} | PendingRef<@{type_name}> {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{type_name} | PendingRef<@{type_name}>;
                            }

                            if (typeof value !== "object" || value === null || Array.isArray(value)) {
                                throw new DeserializeError([{ field: "_root", message: "@{type_name}.__deserialize: expected an object" }]);
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
                                    {$let has_validators = field.has_validators()}
                                    {#if field.optional}
                                        if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                            const @{raw_var} = obj["@{field.json_key}"];
                                            {#match &field.type_cat}
                                                {:case TypeCategory::Primitive}
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var};

                                                {:case TypeCategory::Date}
                                                    {
                                                        const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = __dateVal;
                                                    }

                                                {:case TypeCategory::Array(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                    }

                                                {:case TypeCategory::Map(key_type, value_type)}
                                                    if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                        instance.@{field.field_name} = new Map(
                                                            Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    }

                                                {:case TypeCategory::Set(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                    }

                                                {:case TypeCategory::Serializable(inner_type_name)}
                                                    if (typeof (@{inner_type_name} as any)?.__deserialize === "function") {
                                                        const __result = (@{inner_type_name} as any).__deserialize(@{raw_var}, ctx);
                                                        ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                    } else {
                                                        instance.@{field.field_name} = @{raw_var};
                                                    }

                                                {:case TypeCategory::Nullable(_)}
                                                    if (@{raw_var} === null) {
                                                        instance.@{field.field_name} = null;
                                                    } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                        const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
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
                                                {:case TypeCategory::Primitive}
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var};

                                                {:case TypeCategory::Date}
                                                    {
                                                        const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = __dateVal;
                                                    }

                                                {:case TypeCategory::Array(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                    }

                                                {:case TypeCategory::Map(key_type, value_type)}
                                                    if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                        instance.@{field.field_name} = new Map(
                                                            Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    }

                                                {:case TypeCategory::Set(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                    }

                                                {:case TypeCategory::Serializable(inner_type_name)}
                                                    if (typeof (@{inner_type_name} as any)?.__deserialize === "function") {
                                                        const __result = (@{inner_type_name} as any).__deserialize(@{raw_var}, ctx);
                                                        ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                    } else {
                                                        instance.@{field.field_name} = @{raw_var};
                                                    }

                                                {:case TypeCategory::Nullable(_)}
                                                    if (@{raw_var} === null) {
                                                        instance.@{field.field_name} = null;
                                                    } else if (typeof (@{raw_var} as any)?.__ref !== "undefined") {
                                                        const __result = ctx.getOrDefer((@{raw_var} as any).__ref);
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

                            return instance as @{type_name};
                        }

                        export function validateField@{validate_field_generic_decl}(
                            field: K,
                            value: @{type_name}[K]
                        ): Array<{ field: string; message: string }> {
                            {#if has_validators}
                            const errors: Array<{ field: string; message: string }> = [];
                            switch (field) {
                                {#for field in &fields_with_validators}
                                case "@{field.field_name}": {
                                    const __val = value as @{field.ts_type};
                                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                                    @{validation_code}
                                    break;
                                }
                                {/for}
                            }
                            return errors;
                            {:else}
                            return [];
                            {/if}
                        }

                        export function validateFields(
                            partial: Partial<@{type_name}>
                        ): Array<{ field: string; message: string }> {
                            {#if has_validators}
                            const errors: Array<{ field: string; message: string }> = [];
                            {#for field in &fields_with_validators}
                            if ("@{field.field_name}" in partial && partial.@{field.field_name} !== undefined) {
                                const __val = partial.@{field.field_name} as @{field.ts_type};
                                {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                                @{validation_code}
                            }
                            {/for}
                            return errors;
                            {:else}
                            return [];
                            {/if}
                        }
                    }
                };
                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                result.add_import("PendingRef", "macroforge/serde");
                Ok(result)
            } else if let Some(members) = type_alias.as_union() {
                // Union type - could be literal union, type ref union, or mixed
                let literals: Vec<String> = members
                    .iter()
                    .filter_map(|m| m.as_literal().map(|s| s.to_string()))
                    .collect();
                let type_refs: Vec<String> = members
                    .iter()
                    .filter_map(|m| m.as_type_ref().map(|s| s.to_string()))
                    .collect();

                let is_literal_only = !literals.is_empty() && type_refs.is_empty();
                let is_type_ref_only = literals.is_empty() && !type_refs.is_empty();
                let has_literals = !literals.is_empty();
                let has_type_refs = !type_refs.is_empty();

                // Pre-compute the expected types string for error messages
                let expected_types_str = type_refs.join(", ");

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
                                const resultOrRef = __deserialize(obj, ctx);

                                if (PendingRef.is(resultOrRef)) {
                                    return Result.err([{ field: "_root", message: "@{type_name}.fromObject: root cannot be a forward reference" }]);
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

                        export function __deserialize(value: any, ctx: DeserializeContext): @{type_name} | PendingRef<@{type_name}> {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{type_name} | PendingRef<@{type_name}>;
                            }

                            {#if is_literal_only}
                                // Literal-only union: validate value is one of the allowed literals
                                const allowedValues = [{#for lit in literals}@{lit}, {/for}] as const;
                                if (!allowedValues.includes(value)) {
                                    throw new DeserializeError([{
                                        field: "_root",
                                        message: "Invalid value for @{type_name}: expected one of " + allowedValues.map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
                                    }]);
                                }
                                return value as @{type_name};
                            {:else if is_type_ref_only}
                                // Type reference union: dispatch based on __type
                                if (typeof value !== "object" || value === null) {
                                    throw new DeserializeError([{
                                        field: "_root",
                                        message: "@{type_name}.__deserialize: expected an object"
                                    }]);
                                }

                                const __typeName = (value as any).__type;
                                if (typeof __typeName !== "string") {
                                    throw new DeserializeError([{
                                        field: "_root",
                                        message: "@{type_name}.__deserialize: missing __type field for union dispatch"
                                    }]);
                                }

                                // Dispatch to the appropriate type's deserializer
                                {#for type_ref in type_refs.clone()}
                                    if (__typeName === "@{type_ref}") {
                                        if (typeof (@{type_ref} as any)?.__deserialize === "function") {
                                            return (@{type_ref} as any).__deserialize(value, ctx) as @{type_name};
                                        }
                                        return value as @{type_name};
                                    }
                                {/for}

                                throw new DeserializeError([{
                                    field: "_root",
                                    message: "@{type_name}.__deserialize: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                }]);
                            {:else}
                                // Mixed union: try literal check first, then type dispatch
                                {#if has_literals}
                                    const allowedLiterals = [{#for lit in literals}@{lit}, {/for}] as const;
                                    if (typeof value !== "object" && allowedLiterals.includes(value as any)) {
                                        return value as @{type_name};
                                    }
                                {/if}

                                {#if has_type_refs}
                                    if (typeof value === "object" && value !== null) {
                                        const __typeName = (value as any).__type;
                                        if (typeof __typeName === "string") {
                                            {#for type_ref in type_refs}
                                                if (__typeName === "@{type_ref}") {
                                                    if (typeof (@{type_ref} as any)?.__deserialize === "function") {
                                                        return (@{type_ref} as any).__deserialize(value, ctx) as @{type_name};
                                                    }
                                                    return value as @{type_name};
                                                }
                                            {/for}
                                        }
                                    }
                                {/if}

                                throw new DeserializeError([{
                                    field: "_root",
                                    message: "@{type_name}.__deserialize: value does not match any union member"
                                }]);
                            {/if}
                        }
                    }
                };
                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                result.add_import("PendingRef", "macroforge/serde");
                Ok(result)
            } else {
                // Fallback for other type alias forms (simple alias, tuple, etc.)
                let mut result = ts_template! {
                    export namespace @{type_name} {
                        export function {|fromStringifiedJSON@{generic_decl}|}(json: string, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
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

                        export function {|fromObject@{generic_decl}|}(obj: unknown, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
                            try {
                                const ctx = DeserializeContext.create();
                                const result = __deserialize@{generic_args}(obj, ctx);
                                ctx.applyPatches();
                                if (opts?.freeze) {
                                    ctx.freezeAll();
                                }
                                return Result.ok<@{full_type_name}>(result);
                            } catch (e) {
                                if (e instanceof DeserializeError) {
                                    return Result.err(e.errors);
                                }
                                const message = e instanceof Error ? e.message : String(e);
                                return Result.err([{ field: "_root", message }]);
                            }
                        }

                        export function {|__deserialize@{generic_decl}|}(value: any, ctx: DeserializeContext): @{full_type_name} {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{full_type_name};
                            }
                            return value as @{type_name};
                        }

                        // Union types don't have field-level validators on the union itself
                        // These stubs are provided for consistency with the API
                        export function validateField@{validate_field_generic_decl}(
                            field: K,
                            value: @{type_name}[K]
                        ): Array<{ field: string; message: string }> {
                            return [];
                        }

                        export function validateFields(
                            partial: Partial<@{type_name}>
                        ): Array<{ field: string; message: string }> {
                            return [];
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
