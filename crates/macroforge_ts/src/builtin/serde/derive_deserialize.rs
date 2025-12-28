//! # Deserialize Macro Implementation
//!
//! The `Deserialize` macro generates JSON deserialization methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameDeserialize(input)` + `static deserialize(input)` | Standalone function + static factory method |
//! | Enum | `enumNameDeserialize(input)`, `enumNameDeserializeWithContext(data)`, `enumNameIs(value)` | Standalone functions |
//! | Interface | `interfaceNameDeserialize(input)`, etc. | Standalone functions |
//! | Type Alias | `typeNameDeserialize(input)`, etc. | Standalone functions |
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
//! References only apply to object-shaped, serializable values. The generator avoids probing for
//! `__ref` on primitive-like fields (including literal unions and `T | null` where `T` is primitive-like),
//! and it parses `Date` / `Date | null` from ISO strings without treating them as references.
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
//! - `skip` / `skipDeserializing` - Exclude field from deserialization
//! - `rename = "jsonKey"` - Read from different JSON property
//! - `default` / `default = expr` - Use default value if missing
//! - `flatten` - Read fields from parent object level
//! - `validate(...)` - Apply validators
//!
//! ## Container-Level Options
//!
//! - `denyUnknownFields` - Error on unrecognized JSON properties
//! - `renameAll = "camelCase"` - Apply naming convention to all fields
//!
//! ## Union Type Deserialization
//!
//! Union types are deserialized based on their member types:
//!
//! ### Literal Unions
//! For unions of literal values (`"A" | "B" | 123`), the value is validated against
//! the allowed literals directly.
//!
//! ### Primitive Unions
//! For unions containing primitive types (`string | number`), the deserializer uses
//! `typeof` checks to validate the value type. No `__type` discriminator is needed.
//!
//! ### Class/Interface Unions
//! For unions of serializable types (`User | Admin`), the deserializer requires a
//! `__type` field in the JSON to dispatch to the correct type's `deserializeWithContext` method.
//!
//! ### Generic Type Parameters
//! For generic unions like `type Result<T> = T | Error`, the generic type parameter `T`
//! is passed through as-is since its concrete type is only known at the call site.
//!
//! ### Mixed Unions
//! Mixed unions (e.g., `string | Date | User`) check in order:
//! 1. Literal values
//! 2. Primitives (via `typeof`)
//! 3. Date (via `instanceof` or ISO string parsing)
//! 4. Serializable types (via `__type` dispatch)
//! 5. Generic type parameters (pass-through)
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Deserialize) @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     /** @serde({ validate: { email: true, maxLength: 255 } }) */
//!     email: string;
//!
//!     /** @serde({ default: "guest" }) */
//!     name: string;
//!
//!     /** @serde({ validate: { positive: true } }) */
//!     age?: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//!
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     email: string;
//!
//!     name: string;
//!
//!     age?: number;
//!
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//!
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: @{DESERIALIZE_OPTIONS}
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//!
//!             const ctx = @{DESERIALIZE_CONTEXT}.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (@{PENDING_REF}.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof @{DESERIALIZE_ERROR}) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//!
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: @{DESERIALIZE_CONTEXT}): User | @{PENDING_REF} {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new @{DESERIALIZE_ERROR}([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new @{deserialize_error_expr}(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = "guest";
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new @{deserialize_error_expr}(errors);
//!         }
//!         return instance;
//!     }
//!
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//!
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//!
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! ## Required Imports
//!
//! The generated code automatically imports:
//! - `DeserializeContext`, `DeserializeError`, `PendingRef` from `macroforge/serde`

use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, TsSynError, ident,
    parse_ts_expr, parse_ts_macro_input,
};

use convert_case::{Case, Casing};

use super::{
    SerdeContainerOptions, SerdeFieldOptions, TypeCategory, Validator, ValidatorSpec,
    get_foreign_types, rewrite_expression_namespaces,
};
use crate::builtin::return_types::{
    DESERIALIZE_CONTEXT, DESERIALIZE_ERROR, DESERIALIZE_OPTIONS, PENDING_REF,
    deserialize_return_type, is_ok_check, wrap_error, wrap_success,
};

fn parse_default_expr(expr_src: &str) -> Result<Expr, TsSynError> {
    let expr = parse_ts_expr(expr_src)?;
    if matches!(*expr, Expr::Ident(_)) {
        let literal_src = format!("{expr_src:?}");
        return parse_ts_expr(&literal_src).map(|expr| *expr);
    }
    Ok(*expr)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SerdeValueKind {
    PrimitiveLike,
    Date,
    NullableDate,
    Other,
}

fn is_ts_primitive_keyword(s: &str) -> bool {
    matches!(
        s.trim(),
        "string" | "number" | "boolean" | "bigint" | "null" | "undefined"
    )
}

fn is_ts_literal(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    if matches!(s, "true" | "false") {
        return true;
    }

    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        return true;
    }

    // Very small heuristic: numeric / bigint literals
    if let Some(digits) = s.strip_suffix('n') {
        return !digits.is_empty()
            && digits
                .chars()
                .all(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '+');
    }

    s.chars()
        .all(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '+' || c == '.')
}

fn is_union_of_primitive_like(s: &str) -> bool {
    if !s.contains('|') {
        return false;
    }
    s.split('|').all(|part| {
        let part = part.trim();
        is_ts_primitive_keyword(part) || is_ts_literal(part)
    })
}

fn classify_serde_value_kind(ts_type: &str) -> SerdeValueKind {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive => SerdeValueKind::PrimitiveLike,
        TypeCategory::Date => SerdeValueKind::Date,
        TypeCategory::Nullable(inner) => match classify_serde_value_kind(&inner) {
            SerdeValueKind::Date => SerdeValueKind::NullableDate,
            SerdeValueKind::PrimitiveLike => SerdeValueKind::PrimitiveLike,
            _ => SerdeValueKind::Other,
        },
        TypeCategory::Optional(inner) => classify_serde_value_kind(&inner),
        _ => {
            if is_union_of_primitive_like(ts_type) {
                SerdeValueKind::PrimitiveLike
            } else {
                SerdeValueKind::Other
            }
        }
    }
}

/// If the given type string is a Serializable type, return its name.
/// Returns None for primitives, Date, and other non-serializable types.
fn get_serializable_type_name(ts_type: &str) -> Option<String> {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Serializable(name) => Some(name),
        _ => None,
    }
}

/// Extracts the base type name from a potentially generic type.
/// For example: "User<T>" -> "User", "Map<string, number>" -> "Map"
fn extract_base_type(ts_type: &str) -> String {
    if let Some(idx) = ts_type.find('<') {
        ts_type[..idx].to_string()
    } else {
        ts_type.to_string()
    }
}

/// Generates the DeserializeWithContext function name for a nested deserializable type.
/// For example: "User" -> "userDeserializeWithContext"
fn nested_deserialize_fn_name(type_name: &str) -> String {
    format!("{}DeserializeWithContext", type_name.to_case(Case::Camel))
}

/// Contains field information needed for JSON deserialization code generation.
///
/// Each field that should be deserialized is represented by this struct,
/// capturing all the information needed to generate parsing, validation,
/// and assignment code.
#[derive(Clone)]
struct DeserializeField {
    /// The JSON property name to read from the input object.
    /// This may differ from `field_name` if `@serde({rename: "..."})` is used.
    json_key: String,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property assignments like `instance.fieldName = value`.
    _field_name: String,
    /// The field name as an AST identifier for property access.
    _field_ident: Ident,

    /// The TypeScript type annotation string (e.g., "string", "number[]").
    /// Used for type casting in generated code.
    #[allow(dead_code)]
    ts_type: String,

    /// The category of the field's type, used to select the appropriate
    /// deserialization strategy (primitive, Date, Array, Map, Set, etc.).
    _type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier or `@serde(default)`).
    /// Optional fields don't require the JSON property to be present.
    optional: bool,

    /// Whether the field has a default value specified.
    #[allow(dead_code)]
    has_default: bool,

    /// The default value expression to use if the field is missing.
    /// Example: expression for `@serde(default = "guest")`.
    _default_expr: Option<Expr>,

    /// Whether the field should be read from the parent object level.
    /// Flattened fields look for their properties directly in the parent JSON.
    flatten: bool,

    /// List of validators to apply after parsing the field value.
    /// Each validator generates a condition check and error message.
    validators: Vec<ValidatorSpec>,

    /// For `T | null` unions: classification of `T`.
    _nullable_inner_kind: Option<SerdeValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    _array_elem_kind: Option<SerdeValueKind>,

    // --- Serializable type tracking for direct function calls ---
    /// For `T | null` where T is Serializable: the type name.
    _nullable_serializable_type: Option<String>,

    /// Custom deserialization function expression (from `@serde({deserializeWith: "fn"})`)
    /// When set, this function is called instead of type-based deserialization.
    _deserialize_with: Option<Expr>,
}

impl DeserializeField {
    /// Returns true if this field has any validators that need to be applied.
    fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Holds information about a serializable type reference in a union.
///
/// For parameterized types like `RecordLink<Product>`, we need both:
/// - The full type string for `__type` comparison and type casting
/// - The base type name for runtime namespace access
#[derive(Clone)]
struct SerializableTypeRef {
    /// The full type reference string (e.g., "RecordLink<Product>")
    full_type: String,
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
/// ```rust
/// use macroforge_ts::builtin::serde::Validator;
/// use macroforge_ts::builtin::serde::derive_deserialize::generate_validation_condition;
///
/// let condition = generate_validation_condition(&Validator::Email, "email");
/// assert!(condition.contains("test(email)"));
///
/// let condition = generate_validation_condition(&Validator::MaxLength(100), "name");
/// assert_eq!(condition, "name.length > 100");
/// ```
pub fn generate_validation_condition(validator: &Validator, value_var: &str) -> String {
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
            format!(
                r#"{value_var} == null || {value_var}.getTime() <= new Date("{date}").getTime()"#
            )
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() < new Date("{date}").getTime()"#
            )
        }
        Validator::LessThanDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() >= new Date("{date}").getTime()"#
            )
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() > new Date("{date}").getTime()"#
            )
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

#[ts_macro_derive(
    Deserialize,
    description = "Generates deserialization methods with cycle/forward-reference support (fromStringifiedJSON, deserializeWithContext)",
    attributes((serde, "Configure deserialization for this field. Options: skip, rename, flatten, default, validate"))
)]
pub fn derive_deserialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ident!(class_name);
            let class_expr: Expr = class_ident.clone().into();
            let deserialize_context_ident = ident!(DESERIALIZE_CONTEXT);
            let deserialize_context_expr: Expr = deserialize_context_ident.clone().into();
            let deserialize_error_expr: Expr = ident!(DESERIALIZE_ERROR).into();
            let pending_ref_ident = ident!(PENDING_REF);
            let pending_ref_expr: Expr = pending_ref_ident.clone().into();
            let deserialize_options_ident = ident!(DESERIALIZE_OPTIONS);
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Generate function names (always prefix style)
            let fn_deserialize_ident = ident!("{}Deserialize", class_name.to_case(Case::Camel));
            let fn_deserialize_internal_ident =
                ident!("{}DeserializeWithContext", class_name.to_case(Case::Camel));
            let fn_is_ident = ident!("{}Is", class_name.to_case(Case::Camel));

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

                    let nullable_inner_kind = match &type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let array_elem_kind = match &type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let nullable_serializable_type = match &type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };

                    // Check for foreign type deserializer if no explicit deserialize_with
                    let deserialize_with_src = if opts.deserialize_with.is_some() {
                        opts.deserialize_with.clone()
                    } else {
                        // Check if the field's type matches a configured foreign type
                        let foreign_types = get_foreign_types();
                        let ft_match =
                            TypeCategory::match_foreign_type(&field.ts_type, &foreign_types);
                        // Error if import source mismatch (type matches but wrong import)
                        if let Some(error) = ft_match.error {
                            all_diagnostics.error(field.span, error);
                        }
                        // Log warning for informational hints
                        if let Some(warning) = ft_match.warning {
                            all_diagnostics.warning(field.span, warning);
                        }
                        // Rewrite namespace references to use generated aliases
                        ft_match
                            .config
                            .and_then(|ft| ft.deserialize_expr.clone())
                            .map(|expr| rewrite_expression_namespaces(&expr))
                    };

                    let deserialize_with = deserialize_with_src.as_ref().and_then(|expr_src| {
                        match parse_ts_expr(expr_src) {
                            Ok(expr) => Some(*expr),
                            Err(err) => {
                                all_diagnostics.error(
                                    field.span,
                                    format!(
                                        "@serde(deserializeWith): invalid expression for '{}': {err:?}",
                                        field.name
                                    ),
                                );
                                None
                            }
                        }
                    });

                        let default_expr = opts.default_expr.as_ref().and_then(|expr_src| {
                            match parse_default_expr(expr_src) {
                                Ok(expr) => Some(expr),
                                Err(err) => {
                                    all_diagnostics.error(
                                        field.span,
                                        format!(
                                            "@serde({{default: ...}}): invalid expression for '{}': {err:?}",
                                            field.name
                                        ),
                                    );
                                    None
                                }
                            }
                        });

                    Some(DeserializeField {
                        json_key,
                        _field_name: field.name.clone(),
                        _field_ident: ident!(field.name.as_str()),
                        ts_type: field.ts_type.clone(),
                        _type_cat: type_cat,
                        optional: field.optional || opts.default || opts.default_expr.is_some(),
                        has_default: opts.default || opts.default_expr.is_some(),
                        _default_expr: default_expr,
                        flatten: opts.flatten,
                        validators: opts.validators.clone(),
                        _nullable_inner_kind: nullable_inner_kind,
                        _array_elem_kind: array_elem_kind,
                        _nullable_serializable_type: nullable_serializable_type,
                        _deserialize_with: deserialize_with,
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
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

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
                    .map(|f| format!("\"{}\" in o", f.json_key))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            // Compute return type and wrappers
            let return_type = deserialize_return_type(class_name);
            let return_type_ident = ident!(return_type.as_str());
            let success_result = wrap_success("resultOrRef");
            let success_result_expr =
                parse_ts_expr(&success_result).expect("deserialize success wrapper should parse");
            let error_root_ref = wrap_error(&format!(
                r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
                class_name
            ));
            let error_root_ref_expr = parse_ts_expr(&error_root_ref)
                .expect("deserialize root error wrapper should parse");
            let error_from_catch = wrap_error("e.errors");
            let error_from_catch_expr = parse_ts_expr(&error_from_catch)
                .expect("deserialize catch error wrapper should parse");
            let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
            let error_generic_message_expr = parse_ts_expr(&error_generic_message)
                .expect("deserialize generic error wrapper should parse");

            // Build known keys array string
            let known_keys_list: Vec<_> = known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

            let mut result = ts_template!(Within {
                constructor(props: Record<string, unknown>) {
                    {#for field in &all_fields}
                        {#if field.optional}
                            this.@{field._field_ident} = props.@{field._field_ident} as @{field.ts_type};
                        {:else}
                            this.@{field._field_ident} = props.@{field._field_ident};
                        {/if}
                    {/for}
                }

                /** Deserializes input to an instance of this class. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized instance or validation errors */
                static deserialize(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                    try {
                        // Auto-detect: if string, parse as JSON first
                        const data = typeof input === "string" ? JSON.parse(input) : input;

                        const ctx = @{deserialize_context_expr}.create();
                        const resultOrRef = @{&class_expr}.deserializeWithContext(data, ctx);

                        if (@{pending_ref_expr}.is(resultOrRef)) {
                            return @{error_root_ref_expr};
                        }

                        ctx.applyPatches();
                        if (opts?.freeze) {
                            ctx.freezeAll();
                        }

                        return @{success_result_expr};
                    } catch (e) {
                        if (e instanceof @{deserialize_error_expr}) {
                            return @{error_from_catch_expr};
                        }
                        const message = e instanceof Error ? e.message : String(e);
                        return @{error_generic_message_expr};
                    }
                }

                /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                static deserializeWithContext(value: any, ctx: @{deserialize_context_ident}): @{class_ident} | @{pending_ref_ident} {
                    // Handle reference to already-deserialized object
                    if (value?.__ref !== undefined) {
                        return ctx.getOrDefer(value.__ref);
                    }

                    if (typeof value !== "object" || value === null || Array.isArray(value)) {
                        throw new @{deserialize_error_expr}([{ field: "_root", message: "@{class_name}.deserializeWithContext: expected an object" }]);
                    }

                    const obj = value as Record<string, unknown>;
                    const errors: Array<{ field: string; message: string }> = [];

                    {#if container_opts.deny_unknown_fields}
                        const knownKeys = new Set(["__type", "__id", "__ref", @{known_keys_list.join(", ")}]);
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

                    if (errors.length > 0) {
                        throw new @{deserialize_error_expr}(errors);
                    }

                    // Create instance using Object.create to avoid constructor
                    const instance = Object.create(@{&class_expr}.prototype) as @{class_ident};

                    // Register with context if __id is present
                    if (obj.__id !== undefined) {
                        ctx.register(obj.__id as number, instance);
                    }

                    // Track for optional freezing
                    ctx.trackForFreeze(instance);

                    // Assign fields
                    {#if !all_fields.is_empty()}
                        {#for field in all_fields}
                            {$let raw_var_name = format!("__raw_{}", field._field_name)}
                            {$let raw_var_ident: Ident = ident!(raw_var_name)}
                            {$let has_validators = field.has_validators()}
                            {#if let Some(fn_expr) = &field._deserialize_with}
                                // Custom deserialization function (deserializeWith)
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        instance.@{field._field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                    }
                                {:else}
                                    instance.@{field._field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                {/if}
                            {:else}
                            {#if field.optional}
                                if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                    const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.ts_type};
                                    {#match &field._type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, class_name)}
                                                {$typescript validation_code}

                                            {/if}
                                            instance.@{field._field_ident} = @{raw_var_ident};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    {$typescript validation_code}

                                                {/if}
                                                instance.@{field._field_ident} = __dateVal;
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var_ident})) {
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, class_name)}
                                                    {$typescript validation_code}

                                                {/if}

                                                {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];
                                                    {:case SerdeValueKind::Date}
                                                        instance.@{field._field_ident} = (@{raw_var_ident} as any[]).map(
                                                            (item) => typeof item === "string" ? new Date(item) : item as Date
                                                        ) as any;
                                                    {:case SerdeValueKind::NullableDate}
                                                        instance.@{field._field_ident} = (@{raw_var_ident} as any[]).map(
                                                            (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                        ) as any;
                                                    {:case _}
                                                        const __arr = (@{raw_var_ident} as any[]).map((item, idx) => {
                                                            if (typeof item?.deserializeWithContext === "function") {
                                                                const result = item.deserializeWithContext(item, ctx);
                                                                if (@{pending_ref_expr}.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            // Check for __ref in array items
                                                            if (item?.__ref !== undefined) {
                                                                const result = ctx.getOrDefer(item.__ref);
                                                                if (@{pending_ref_expr}.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            return item as @{inner};
                                                        });
                                                        instance.@{field._field_ident} = __arr;
                                                        // Patch array items that were pending
                                                        __arr.forEach((item, idx) => {
                                                            if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                                ctx.addPatch(instance.@{field._field_ident}, idx, (item as any).__refId);
                                                            }
                                                        });
                                                {/match}
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                                instance.@{field._field_ident} = new Map(
                                                    Object.entries(@{raw_var_ident} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                );
                                            }

                                        {:case TypeCategory::Set(inner)}
                                            if (Array.isArray(@{raw_var_ident})) {
                                                instance.@{field._field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                            }

                                        {:case TypeCategory::Record(_, _)}
                                            if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                                instance.@{field._field_ident} = @{raw_var_ident} as any;
                                            }

                                        {:case TypeCategory::Wrapper(_)}
                                            // Wrapper types (Partial<T>, Required<T>, etc.) preserve structure
                                            // Pass through directly - the value has the same shape as T
                                            instance.@{field._field_ident} = @{raw_var_ident} as any;

                                        {:case TypeCategory::Serializable(type_name)}
                                            {$let type_expr: Expr = ident!(type_name).into()}
                                            {
                                                const __result = @{type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field._field_ident} = @{raw_var_ident};
                                                {:case SerdeValueKind::Date}
                                                    if (@{raw_var_ident} === null) {
                                                        instance.@{field._field_ident} = null;
                                                    } else {
                                                        instance.@{field._field_ident} = typeof @{raw_var_ident} === "string"
                                                            ? new Date(@{raw_var_ident} as any)
                                                            : @{raw_var_ident} as any;
                                                    }
                                                {:case _}
                                                    if (@{raw_var_ident} === null) {
                                                        instance.@{field._field_ident} = null;
                                                    } else {
                                                        {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                            {$let inner_type_expr: Expr = ident!(inner_type).into()}
                                                            const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                        {:else}
                                                            instance.@{field._field_ident} = @{raw_var_ident};
                                                        {/if}
                                                    }
                                            {/match}

                                        {:case _}
                                            instance.@{field._field_ident} = @{raw_var_ident};
                                    {/match}
                                }
                                {#if let Some(default_expr) = &field._default_expr}
                                    if (!("@{field.json_key}" in obj) || obj["@{field.json_key}"] === undefined) {
                                        instance.@{field._field_ident} = @{default_expr};
                                    }
                                {/if}
                            {:else}
                                {
                                    const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.ts_type};
                                    {#match &field._type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, class_name)}
                                                {$typescript validation_code}

                                            {/if}
                                            instance.@{field._field_ident} = @{raw_var_ident};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    {$typescript validation_code}

                                                {/if}
                                                instance.@{field._field_ident} = __dateVal;
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var_ident})) {
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, class_name)}
                                                    {$typescript validation_code}

                                                {/if}

                                                {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];
                                                    {:case SerdeValueKind::Date}
                                                        instance.@{field._field_ident} = (@{raw_var_ident} as any[]).map(
                                                            (item) => typeof item === "string" ? new Date(item) : item as Date
                                                        ) as any;
                                                    {:case SerdeValueKind::NullableDate}
                                                        instance.@{field._field_ident} = (@{raw_var_ident} as any[]).map(
                                                            (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                        ) as any;
                                                    {:case _}
                                                        const __arr = (@{raw_var_ident} as any[]).map((item, idx) => {
                                                            if (item?.__ref !== undefined) {
                                                                const result = ctx.getOrDefer(item.__ref);
                                                                if (@{pending_ref_expr}.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            return item as @{inner};
                                                        });
                                                        instance.@{field._field_ident} = __arr;
                                                        __arr.forEach((item, idx) => {
                                                            if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                                ctx.addPatch(instance.@{field._field_ident}, idx, (item as any).__refId);
                                                            }
                                                        });
                                                {/match}
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            instance.@{field._field_ident} = new Map(
                                                Object.entries(@{raw_var_ident} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                            );

                                        {:case TypeCategory::Set(inner)}
                                            instance.@{field._field_ident} = new Set(@{raw_var_ident} as @{inner}[]);

                                        {:case TypeCategory::Serializable(type_name)}
                                            {$let type_expr: Expr = ident!(type_name).into()}
                                            {
                                                const __result = @{type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field._field_ident} = @{raw_var_ident};
                                                {:case SerdeValueKind::Date}
                                                    if (@{raw_var_ident} === null) {
                                                        instance.@{field._field_ident} = null;
                                                    } else {
                                                        instance.@{field._field_ident} = typeof @{raw_var_ident} === "string"
                                                            ? new Date(@{raw_var_ident} as any)
                                                            : @{raw_var_ident} as any;
                                                    }
                                                {:case _}
                                                    if (@{raw_var_ident} === null) {
                                                        instance.@{field._field_ident} = null;
                                                    } else {
                                                        {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                            {$let inner_type_expr: Expr = ident!(inner_type).into()}
                                                            const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                        {:else}
                                                            instance.@{field._field_ident} = @{raw_var_ident};
                                                        {/if}
                                                    }
                                            {/match}

                                        {:case _}
                                            instance.@{field._field_ident} = @{raw_var_ident};
                                    {/match}
                                }
                            {/if}
                            {/if}
                        {/for}
                    {/if}

                    {#if !flatten_fields.is_empty()}
                        {#for field in flatten_fields}
                            {#match &field._type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    {$let type_expr: Expr = ident!(type_name).into()}
                                    {
                                        const __result = @{type_expr}.deserializeWithContext(obj, ctx);
                                        ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                    }
                                {:case _}
                                    instance.@{field._field_ident} = obj as any;
                            {/match}
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        throw new @{deserialize_error_expr}(errors);
                    }

                    return instance;
                }

                static validateField<K extends keyof @{class_ident}>(
                    _field: K,
                    _value: @{class_ident}[K]
                ): Array<{ field: string; message: string }> {
                    {#if !fields_with_validators.is_empty()}
                    const errors: Array<{ field: string; message: string }> = [];
                    {#for field in &fields_with_validators}
                    if (_field === "@{field._field_name}") {
                        const __val = _value as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name)}
                        {$typescript validation_code}

                    }
                    {/for}
                    return errors;
                    {:else}
                    return [];
                    {/if}
                }

                static validateFields(
                    _partial: Partial<@{class_ident}>
                ): Array<{ field: string; message: string }> {
                    {#if !fields_with_validators.is_empty()}
                    const errors: Array<{ field: string; message: string }> = [];
                    {#for field in &fields_with_validators}
                    if ("@{field._field_name}" in _partial && _partial.@{field._field_ident} !== undefined) {
                        const __val = _partial.@{field._field_ident} as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name)}
                        {$typescript validation_code}

                    }
                    {/for}
                    return errors;
                    {:else}
                    return [];
                    {/if}
                }

                static hasShape(obj: unknown): boolean {
                    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                        return false;
                    }
                    const o = obj as Record<string, unknown>;
                    return @{shape_check_condition};
                }

                static is(obj: unknown): obj is @{class_ident} {
                    if (obj instanceof @{&class_expr}) {
                        return true;
                    }
                    if (!@{&class_expr}.hasShape(obj)) {
                        return false;
                    }
                    const result = @{&class_expr}.deserialize(obj);
                    return @{parse_ts_expr(&is_ok_check("result")).expect("deserialize is_ok expression should parse")};
                }
            });
            result.add_aliased_import("DeserializeContext", "macroforge/serde");
            result.add_aliased_import("DeserializeError", "macroforge/serde");
            result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
            result.add_aliased_import("PendingRef", "macroforge/serde");

            // Generate standalone functions that delegate to static methods
            let mut standalone = ts_template! {
                /** Deserializes input to an instance. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized instance or validation errors */
                export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                    return @{&class_expr}.deserialize(input, opts);
                }

                /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{class_ident} | @{pending_ref_ident} {
                    return @{&class_expr}.deserializeWithContext(value, ctx);
                }

                /** Type guard: checks if a value can be successfully deserialized. @param value - The value to check @returns True if the value can be deserialized to this type */
                export function @{fn_is_ident}(value: unknown): value is @{class_ident} {
                    return @{&class_expr}.is(value);
                }
            };
            standalone.add_aliased_import("DeserializeContext", "macroforge/serde");
            standalone.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
            standalone.add_aliased_import("PendingRef", "macroforge/serde");

            // Combine standalone functions with class body using {$typescript} composition
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(ts_template! {
                {$typescript standalone}
                {$typescript result}
            })
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let enum_ident = ident!(enum_name);
            let enum_expr: Expr = enum_ident.clone().into();
            let fn_deserialize_ident = ident!("{}Deserialize", enum_name.to_case(Case::Camel));
            let fn_deserialize_internal_ident =
                ident!("{}DeserializeWithContext", enum_name.to_case(Case::Camel));
            let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
            let fn_is_ident = ident!("{}Is", enum_name.to_case(Case::Camel));
            let mut result = ts_template! {
                /** Deserializes input to an enum value. Automatically detects whether input is a JSON string or value. @param input - JSON string or value to deserialize @returns The enum value @throws Error if the value is not a valid enum member */
                export function @{fn_deserialize_ident}(input: unknown): @{&enum_ident} {
                    const data = typeof input === "string" ? JSON.parse(input) : input;
                    return @{fn_deserialize_internal_expr}(data);
                }

                /** Deserializes with an existing context (for consistency with other types). */
                export function @{fn_deserialize_internal_ident}(data: unknown): @{&enum_ident} {
                    for (const key of Object.keys(@{&enum_expr})) {
                        const enumValue = @{&enum_expr}[key as keyof typeof @{&enum_ident}];
                        if (enumValue === data) {
                            return data as @{&enum_ident};
                        }
                    }
                    throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
                }

                export function @{fn_is_ident}(value: unknown): value is @{&enum_ident} {
                    for (const key of Object.keys(@{&enum_expr})) {
                        const enumValue = @{&enum_expr}[key as keyof typeof @{&enum_ident}];
                        if (enumValue === value) {
                            return true;
                        }
                    }
                    return false;
                }
            };

            result.add_aliased_import("DeserializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let _type_name = interface_name; // Alias for template consistency
            let interface_ident = ident!(interface_name);
            let _type_ident = interface_ident.clone(); // Alias for template consistency
            let deserialize_context_ident = ident!(DESERIALIZE_CONTEXT);
            let deserialize_context_expr: Expr = deserialize_context_ident.clone().into();
            let deserialize_error_expr: Expr = ident!(DESERIALIZE_ERROR).into();
            let pending_ref_ident = ident!(PENDING_REF);
            let pending_ref_expr: Expr = pending_ref_ident.clone().into();
            let deserialize_options_ident = ident!(DESERIALIZE_OPTIONS);
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect deserializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<DeserializeField> = interface
                .fields()
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

                    let nullable_inner_kind = match &type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let array_elem_kind = match &type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let nullable_serializable_type = match &type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };

                    // Check for foreign type deserializer if no explicit deserialize_with
                    let deserialize_with_src = if opts.deserialize_with.is_some() {
                        opts.deserialize_with.clone()
                    } else {
                        // Check if the field's type matches a configured foreign type
                        let foreign_types = get_foreign_types();
                        let ft_match =
                            TypeCategory::match_foreign_type(&field.ts_type, &foreign_types);
                        // Error if import source mismatch (type matches but wrong import)
                        if let Some(error) = ft_match.error {
                            all_diagnostics.error(field.span, error);
                        }
                        // Log warning for informational hints
                        if let Some(warning) = ft_match.warning {
                            all_diagnostics.warning(field.span, warning);
                        }
                        // Rewrite namespace references to use generated aliases
                        ft_match
                            .config
                            .and_then(|ft| ft.deserialize_expr.clone())
                            .map(|expr| rewrite_expression_namespaces(&expr))
                    };

                    let deserialize_with = deserialize_with_src.as_ref().and_then(|expr_src| {
                        match parse_ts_expr(expr_src) {
                            Ok(expr) => Some(*expr),
                            Err(err) => {
                                all_diagnostics.error(
                                    field.span,
                                    format!(
                                        "@serde(deserializeWith): invalid expression for '{}': {err:?}",
                                        field.name
                                    ),
                                );
                                None
                            }
                        }
                    });

                    let default_expr = opts.default_expr.as_ref().and_then(|expr_src| {
                        match parse_default_expr(expr_src) {
                            Ok(expr) => Some(expr),
                            Err(err) => {
                                all_diagnostics.error(
                                    field.span,
                                    format!(
                                        "@serde({{default: ...}}): invalid expression for '{}': {err:?}",
                                        field.name
                                    ),
                                );
                                None
                            }
                        }
                    });

                    Some(DeserializeField {
                        json_key,
                        _field_name: field.name.clone(),
                        _field_ident: ident!(field.name.as_str()),
                        ts_type: field.ts_type.clone(),
                        _type_cat: type_cat,
                        optional: field.optional || opts.default || opts.default_expr.is_some(),
                        has_default: opts.default || opts.default_expr.is_some(),
                        _default_expr: default_expr,
                        flatten: opts.flatten,
                        validators: opts.validators.clone(),
                        _nullable_inner_kind: nullable_inner_kind,
                        _array_elem_kind: array_elem_kind,
                        _nullable_serializable_type: nullable_serializable_type,
                        _deserialize_with: deserialize_with,
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
                    .map(|f| format!("\"{}\" in o", f.json_key))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            let fn_deserialize_ident = ident!(format!(
                "{}Deserialize",
                interface_name.to_case(Case::Camel)
            ));
            let fn_deserialize_internal_ident = ident!(format!(
                "{}DeserializeWithContext",
                interface_name.to_case(Case::Camel)
            ));
            let fn_deserialize_expr: Expr = fn_deserialize_ident.clone().into();
            let fn_deserialize_internal_expr: Expr = fn_deserialize_internal_ident.clone().into();
            let fn_validate_field_ident = ident!(format!(
                "{}ValidateField",
                interface_name.to_case(Case::Camel)
            ));
            let fn_validate_fields_ident = ident!(format!(
                "{}ValidateFields",
                interface_name.to_case(Case::Camel)
            ));
            let fn_is_ident = ident!(format!("{}Is", interface_name.to_case(Case::Camel)));
            let fn_has_shape_ident =
                ident!(format!("{}HasShape", interface_name.to_case(Case::Camel)));
            let fn_has_shape_expr: Expr = fn_has_shape_ident.clone().into();

            // Compute return type and wrappers
            let return_type = deserialize_return_type(interface_name);
            let return_type_ident = ident!(return_type.as_str());
            let success_result = wrap_success("resultOrRef");
            let success_result_expr =
                parse_ts_expr(&success_result).expect("deserialize success wrapper should parse");
            let error_root_ref = wrap_error(&format!(
                r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
                interface_name
            ));
            let error_root_ref_expr = parse_ts_expr(&error_root_ref)
                .expect("deserialize root error wrapper should parse");
            let error_from_catch = wrap_error("e.errors");
            let error_from_catch_expr = parse_ts_expr(&error_from_catch)
                .expect("deserialize catch error wrapper should parse");
            let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
            let error_generic_message_expr = parse_ts_expr(&error_generic_message)
                .expect("deserialize generic error wrapper should parse");

            // Build known keys array string
            let known_keys_list: Vec<_> = known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

            let mut result = {
                ts_template! {
                    /** Deserializes input to this interface type.Automatically detects whether input is a JSON string or object.@param input - JSON string or object to deserialize@param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
                    export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                        try {
                            // Auto-detect: if string, parse as JSON first
                            const data = typeof input === "string" ? JSON.parse(input) : input;

                            const ctx = @{deserialize_context_expr}.create();
                            const resultOrRef = @{fn_deserialize_internal_expr}(data, ctx);

                            if (@{pending_ref_expr}.is(resultOrRef)) {
                                return @{error_root_ref_expr};
                            }

                            ctx.applyPatches();
                            if (opts?.freeze) {
                                ctx.freezeAll();
                            }

                            return @{success_result_expr};
                        } catch (e) {
                            if (e instanceof @{deserialize_error_expr}) {
                                return @{error_from_catch_expr};
                            }
                            const message = e instanceof Error ? e.message : String(e);
                            return @{error_generic_message_expr};
                        }
                    }

                    /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                    export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{interface_ident} | @{pending_ref_ident} {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref);
                        }

                        if (typeof value !== "object" || value === null || Array.isArray(value)) {
                            throw new @{deserialize_error_expr}([{ field: "_root", message: "@{interface_name}.deserializeWithContext: expected an object" }]);
                        }

                        const obj = value as Record<string, unknown>;
                        const errors: Array<{ field: string; message: string }> = [];

                        {#if container_opts.deny_unknown_fields}
                            const knownKeys = new Set(["__type", "__id", "__ref", @{known_keys_list.join(", ")}]);
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

                        if (errors.length > 0) {
                            throw new @{deserialize_error_expr}(errors);
                        }

                        const instance: any = {};

                        if (obj.__id !== undefined) {
                            ctx.register(obj.__id as number, instance);
                        }

                        ctx.trackForFreeze(instance);

                        {#if !all_fields.is_empty()}
                            {#for field in all_fields}
                                {$let raw_var_name = format!("__raw_{}", field._field_name)}
                                {$let raw_var_ident: Ident = ident!(raw_var_name)}
                                {$let has_validators = field.has_validators()}
                                {#if let Some(fn_expr) = &field._deserialize_with}
                                    // Custom deserialization function (deserializeWith)
                                    {#if field.optional}
                                        if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                            instance.@{field._field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                        }
                                    {:else}
                                        instance.@{field._field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                    {/if}
                                {:else}
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.ts_type};
                                        {#match &field._type_cat}
                                            {:case TypeCategory::Primitive}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
                                                    {$typescript validation_code}

                                                {/if}
                                                instance.@{field._field_ident} = @{raw_var_ident};

                                            {:case TypeCategory::Date}
                                                {
                                                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                        {$typescript validation_code}

                                                    {/if}
                                                    instance.@{field._field_ident} = __dateVal;
                                                }

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var_ident})) {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
                                                        {$typescript validation_code}

                                                    {/if}
                                                    instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];
                                                }

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                                    instance.@{field._field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                }

                                            {:case TypeCategory::Set(inner)}
                                                if (Array.isArray(@{raw_var_ident})) {
                                                    instance.@{field._field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                }

                                            {:case TypeCategory::Serializable(type_name)}
                                                {$let deserialize_with_context_fn: Expr = ident!(nested_deserialize_fn_name(type_name)).into()}
                                                {
                                                    const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field._field_ident} = @{raw_var_ident};
                                                    {:case SerdeValueKind::Date}
                                                        if (@{raw_var_ident} === null) {
                                                            instance.@{field._field_ident} = null;
                                                        } else {
                                                            instance.@{field._field_ident} = typeof @{raw_var_ident} === "string"
                                                                ? new Date(@{raw_var_ident} as any)
                                                                : @{raw_var_ident} as any;
                                                        }
                                                    {:case _}
                                                        if (@{raw_var_ident} === null) {
                                                            instance.@{field._field_ident} = null;
                                                        } else {
                                                            {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                                {$let deserialize_with_context_fn: Expr = ident!(nested_deserialize_fn_name(inner_type)).into()}
                                                                const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                                                ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                            {:else}
                                                                instance.@{field._field_ident} = @{raw_var_ident};
                                                            {/if}
                                                        }
                                                {/match}

                                            {:case _}
                                                instance.@{field._field_ident} = @{raw_var_ident};
                                        {/match}
                                    }
                                    {#if let Some(default_expr) = &field._default_expr}
                                        if (!("@{field.json_key}" in obj) || obj["@{field.json_key}"] === undefined) {
                                            instance.@{field._field_ident} = @{default_expr};
                                        }
                                    {/if}
                                {:else}
                                    {
                                        const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.ts_type};
                                        {#match &field._type_cat}
                                            {:case TypeCategory::Primitive}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
                                                    {$typescript validation_code}

                                                {/if}
                                                instance.@{field._field_ident} = @{raw_var_ident};

                                            {:case TypeCategory::Date}
                                                {
                                                    const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                        {$typescript validation_code}

                                                    {/if}
                                                    instance.@{field._field_ident} = __dateVal;
                                                }

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var_ident})) {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, interface_name)}
                                                        {$typescript validation_code}

                                                    {/if}
                                                    instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];
                                                }

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                                    instance.@{field._field_ident} = new Map(
                                                        Object.entries(@{raw_var_ident} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                }

                                            {:case TypeCategory::Set(inner)}
                                                if (Array.isArray(@{raw_var_ident})) {
                                                    instance.@{field._field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                }

                                            {:case TypeCategory::Serializable(type_name)}
                                                {$let deserialize_with_context_fn: Expr = ident!(nested_deserialize_fn_name(type_name)).into()}
                                                {
                                                    const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field._field_ident} = @{raw_var_ident};
                                                    {:case SerdeValueKind::Date}
                                                        if (@{raw_var_ident} === null) {
                                                            instance.@{field._field_ident} = null;
                                                        } else {
                                                            instance.@{field._field_ident} = typeof @{raw_var_ident} === "string"
                                                                ? new Date(@{raw_var_ident} as any)
                                                                : @{raw_var_ident} as any;
                                                        }
                                                    {:case _}
                                                        if (@{raw_var_ident} === null) {
                                                            instance.@{field._field_ident} = null;
                                                        } else {
                                                            {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                                {$let deserialize_with_context_fn: Expr = ident!(nested_deserialize_fn_name(inner_type)).into()}
                                                                const __result = @{deserialize_with_context_fn}(@{raw_var_ident}, ctx);
                                                                ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                            {:else}
                                                                instance.@{field._field_ident} = @{raw_var_ident};
                                                            {/if}
                                                        }
                                                {/match}

                                            {:case _}
                                                instance.@{field._field_ident} = @{raw_var_ident};
                                        {/match}
                                    }
                                {/if}
                                {/if}
                            {/for}
                        {/if}

                        if (errors.length > 0) {
                            throw new @{deserialize_error_expr}(errors);
                        }

                        return instance as @{interface_ident};
                    }

                    export function @{fn_validate_field_ident}<K extends keyof @{interface_ident}>(
                        _field: K,
                        _value: @{interface_ident}[K]
                    ): Array<{ field: string; message: string }> {
                        {#if !fields_with_validators.is_empty()}
                        const errors: Array<{ field: string; message: string }> = [];
                        {#for field in &fields_with_validators}
                        if (_field === "@{field._field_name}") {
                            const __val = _value as @{field.ts_type};
                            {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                            {$typescript validation_code}

                        }
                        {/for}
                        return errors;
                        {:else}
                        return [];
                        {/if}
                    }

                    export function @{fn_validate_fields_ident}(
                        _partial: Partial<@{interface_ident}>
                    ): Array<{ field: string; message: string }> {
                        {#if !fields_with_validators.is_empty()}
                        const errors: Array<{ field: string; message: string }> = [];
                        {#for field in &fields_with_validators}
                        if ("@{field._field_name}" in _partial && _partial.@{field._field_ident} !== undefined) {
                            const __val = _partial.@{field._field_ident} as @{field.ts_type};
                            {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                            {$typescript validation_code}

                        }
                        {/for}
                        return errors;
                        {:else}
                        return [];
                        {/if}
                    }

                    export function @{fn_has_shape_ident}(obj: unknown): boolean {
                        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                            return false;
                        }
                        const o = obj as Record<string, unknown>;
                        return @{shape_check_condition};
                    }

                    export function @{fn_is_ident}(obj: unknown): obj is @{interface_ident} {
                        if (!@{fn_has_shape_expr}(obj)) {
                            return false;
                        }
                        const result = @{fn_deserialize_expr}(obj);
                        return @{parse_ts_expr(&is_ok_check("result")).expect("deserialize is_ok expression should parse")};
                    }
                }
            };

            result.add_aliased_import("DeserializeContext", "macroforge/serde");
            result.add_aliased_import("DeserializeError", "macroforge/serde");
            result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
            result.add_aliased_import("PendingRef", "macroforge/serde");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let type_ident = ident!(type_name);
            let deserialize_context_ident = ident!(DESERIALIZE_CONTEXT);
            let deserialize_context_expr: Expr = deserialize_context_ident.clone().into();
            let deserialize_error_expr: Expr = ident!(DESERIALIZE_ERROR).into();
            let pending_ref_ident = ident!(PENDING_REF);
            let pending_ref_expr: Expr = pending_ref_ident.clone().into();
            let deserialize_options_ident = ident!(DESERIALIZE_OPTIONS);

            // Build generic type signature if type has type params
            let type_params = type_alias.type_params();
            let (generic_decl, generic_args) = if type_params.is_empty() {
                (String::new(), String::new())
            } else {
                let params = type_params.join(", ");
                (format!("<{}>", params), format!("<{}>", params))
            };
            let full_type_name = format!("{}{}", type_name, generic_args);
            let full_type_ident = ident!(full_type_name.as_str());

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

                        let nullable_inner_kind = match &type_cat {
                            TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let array_elem_kind = match &type_cat {
                            TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };

                        // Extract serializable type names for direct function calls
                        let nullable_serializable_type = match &type_cat {
                            TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };

                        let deserialize_with = opts.deserialize_with.as_ref().and_then(|expr_src| {
                            match parse_ts_expr(expr_src) {
                                Ok(expr) => Some(*expr),
                                Err(err) => {
                                    all_diagnostics.error(
                                        field.span,
                                        format!(
                                            "@serde(deserializeWith): invalid expression for '{}': {err:?}",
                                            field.name
                                        ),
                                    );
                                    None
                                }
                            }
                        });

                        let default_expr = opts.default_expr.as_ref().and_then(|expr_src| {
                            match parse_default_expr(expr_src) {
                                Ok(expr) => Some(expr),
                                Err(err) => {
                                    all_diagnostics.error(
                                        field.span,
                                        format!(
                                            "@serde({{default: ...}}): invalid expression for '{}': {err:?}",
                                            field.name
                                        ),
                                    );
                                    None
                                }
                            }
                        });

                        Some(DeserializeField {
                            json_key,
                            _field_name: field.name.clone(),
                            _field_ident: ident!(field.name.as_str()),
                            ts_type: field.ts_type.clone(),
                            _type_cat: type_cat,
                            optional: field.optional || opts.default || opts.default_expr.is_some(),
                            has_default: opts.default || opts.default_expr.is_some(),
                            _default_expr: default_expr,
                            flatten: opts.flatten,
                            validators: opts.validators.clone(),
                            _nullable_inner_kind: nullable_inner_kind,
                            _array_elem_kind: array_elem_kind,
                            _nullable_serializable_type: nullable_serializable_type,
                            _deserialize_with: deserialize_with,
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

                // Fields with validators for per-field validation
                let fields_with_validators: Vec<_> = all_fields
                    .iter()
                    .filter(|f| f.has_validators())
                    .cloned()
                    .collect();

                let shape_check_condition: String = if required_fields.is_empty() {
                    "true".to_string()
                } else {
                    required_fields
                        .iter()
                        .map(|f| format!("\"{}\" in o", f.json_key))
                        .collect::<Vec<_>>()
                        .join(" && ")
                };

                let fn_deserialize_ident = ident!(format!(
                    "{}Deserialize{}",
                    type_name.to_case(Case::Camel),
                    generic_decl
                ));
                let fn_deserialize_internal_ident = ident!(format!(
                    "{}DeserializeWithContext",
                    type_name.to_case(Case::Camel)
                ));
                let fn_deserialize_internal_expr: Expr =
                    fn_deserialize_internal_ident.clone().into();
                let fn_validate_field_ident = ident!(format!(
                    "{}ValidateField{}",
                    type_name.to_case(Case::Camel),
                    validate_field_generic_decl
                ));
                let fn_validate_fields_ident =
                    ident!(format!("{}ValidateFields", type_name.to_case(Case::Camel)));
                let fn_is_ident = ident!(format!(
                    "{}Is{}",
                    type_name.to_case(Case::Camel),
                    generic_decl
                ));

                // Compute return type and wrappers
                let return_type = deserialize_return_type(&full_type_name);
                let return_type_ident = ident!(return_type.as_str());
                let success_result = wrap_success("resultOrRef");
                let success_result_expr = parse_ts_expr(&success_result)
                    .expect("deserialize success wrapper should parse");
                let error_root_ref = wrap_error(&format!(
                    r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
                    type_name
                ));
                let error_root_ref_expr = parse_ts_expr(&error_root_ref)
                    .expect("deserialize root error wrapper should parse");
                let error_from_catch = wrap_error("e.errors");
                let error_from_catch_expr = parse_ts_expr(&error_from_catch)
                    .expect("deserialize catch error wrapper should parse");
                let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
                let error_generic_message_expr = parse_ts_expr(&error_generic_message)
                    .expect("deserialize generic error wrapper should parse");

                // Build known keys array string
                let known_keys_list: Vec<_> =
                    known_keys.iter().map(|k| format!("\"{}\"", k)).collect();

                // Flag for whether any required fields exist
                let has_required = !required_fields.is_empty();

                let mut result = {
                    ts_template! {
                        /** Deserializes input to this type. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
                        export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                            try {
                                // Auto-detect: if string, parse as JSON first
                                const data = typeof input === "string" ? JSON.parse(input) : input;

                                const ctx = @{deserialize_context_expr}.create();
                                const resultOrRef = @{fn_deserialize_internal_expr}(data, ctx);

                                if (@{pending_ref_expr}.is(resultOrRef)) {
                                    return @{error_root_ref_expr};
                                }

                                ctx.applyPatches();
                                if (opts?.freeze) {
                                    ctx.freezeAll();
                                }

                                return @{success_result_expr};
                            } catch (e) {
                                if (e instanceof @{deserialize_error_expr}) {
                                    return @{error_from_catch_expr};
                                }
                                const message = e instanceof Error ? e.message : String(e);
                                return @{error_generic_message_expr};
                            }
                        }

                        /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                        export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{type_ident} | @{pending_ref_ident} {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{type_ident} | @{pending_ref_ident};
                            }

                            if (typeof value !== "object" || value === null || Array.isArray(value)) {
                                throw new @{deserialize_error_expr}([{ field: "_root", message: "@{type_name}.deserializeWithContext: expected an object" }]);
                            }

                            const obj = value as Record<string, unknown>;
                            const errors: Array<{ field: string; message: string }> = [];

                            {#if container_opts.deny_unknown_fields}
                                const knownKeys = new Set(["__type", "__id", "__ref", @{known_keys_list.join(", ")}]);
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

                            if (errors.length > 0) {
                                throw new @{deserialize_error_expr}(errors);
                            }

                            const instance: any = {};

                            if (obj.__id !== undefined) {
                                ctx.register(obj.__id as number, instance);
                            }

                            ctx.trackForFreeze(instance);

                            {#if !all_fields.is_empty()}
                                {#for field in all_fields}
                                    {$let raw_var_name = format!("__raw_{}", field._field_name)}
                                    {$let raw_var_ident: Ident = ident!(raw_var_name)}
                                    {$let has_validators = field.has_validators()}
                                    {#if let Some(fn_expr) = &field._deserialize_with}
                                        // Custom deserialization function (deserializeWith)
                                        {#if field.optional}
                                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                                instance.@{field._field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                            }
                                        {:else}
                                            instance.@{field._field_ident} = (@{fn_expr})(obj["@{field.json_key}"]);
                                        {/if}
                                    {:else}
                                    {#if field.optional}
                                        if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                            const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.ts_type};
                                            {#match &field._type_cat}
                                                {:case TypeCategory::Primitive}
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                                        {$typescript validation_code}

                                                    {/if}
                                                    instance.@{field._field_ident} = @{raw_var_ident};

                                                {:case TypeCategory::Date}
                                                    {
                                                        const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                            {$typescript validation_code}

                                                        {/if}
                                                        instance.@{field._field_ident} = __dateVal;
                                                    }

                                                {:case TypeCategory::Array(inner)}
                                                    if (Array.isArray(@{raw_var_ident})) {
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                                            {$typescript validation_code}

                                                        {/if}
                                                        instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];
                                                    }

                                                {:case TypeCategory::Map(key_type, value_type)}
                                                    if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                                        instance.@{field._field_ident} = new Map(
                                                            Object.entries(@{raw_var_ident} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    }

                                                {:case TypeCategory::Set(inner)}
                                                    if (Array.isArray(@{raw_var_ident})) {
                                                        instance.@{field._field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                    }

                                                {:case TypeCategory::Serializable(inner_type_name)}
                                                    {$let inner_type_expr: Expr = ident!(inner_type_name).into()}
                                                    {
                                                        const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                        ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                    }

                                                {:case TypeCategory::Nullable(_)}
                                                    {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            instance.@{field._field_ident} = @{raw_var_ident};
                                                        {:case SerdeValueKind::Date}
                                                            if (@{raw_var_ident} === null) {
                                                                instance.@{field._field_ident} = null;
                                                            } else {
                                                                instance.@{field._field_ident} = typeof @{raw_var_ident} === "string"
                                                                    ? new Date(@{raw_var_ident} as any)
                                                                    : @{raw_var_ident} as any;
                                                            }
                                                        {:case _}
                                                            if (@{raw_var_ident} === null) {
                                                                instance.@{field._field_ident} = null;
                                                            } else {
                                                                {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                                    {$let inner_type_expr: Expr = ident!(inner_type).into()}
                                                                    const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                                    ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                                {:else}
                                                                    instance.@{field._field_ident} = @{raw_var_ident};
                                                                {/if}
                                                            }
                                                    {/match}

                                                {:case _}
                                                    instance.@{field._field_ident} = @{raw_var_ident};
                                            {/match}
                                        }
                                        {#if let Some(default_expr) = &field._default_expr}
                                            if (!("@{field.json_key}" in obj) || obj["@{field.json_key}"] === undefined) {
                                                instance.@{field._field_ident} = @{default_expr};
                                            }
                                        {/if}
                                    {:else}
                                        {
                                            const @{raw_var_ident} = obj["@{field.json_key}"] as @{field.ts_type};
                                            {#match &field._type_cat}
                                                {:case TypeCategory::Primitive}
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                                        {$typescript validation_code}

                                                    {/if}
                                                    instance.@{field._field_ident} = @{raw_var_ident};

                                                {:case TypeCategory::Date}
                                                    {
                                                        const __dateVal = typeof @{raw_var_ident} === "string" ? new Date(@{raw_var_ident}) : @{raw_var_ident} as Date;
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                            {$typescript validation_code}

                                                        {/if}
                                                        instance.@{field._field_ident} = __dateVal;
                                                    }

                                                {:case TypeCategory::Array(inner)}
                                                    if (Array.isArray(@{raw_var_ident})) {
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var_name, &field.json_key, type_name)}
                                                            {$typescript validation_code}

                                                        {/if}
                                                        instance.@{field._field_ident} = @{raw_var_ident} as @{inner}[];
                                                    }

                                                {:case TypeCategory::Map(key_type, value_type)}
                                                    if (typeof @{raw_var_ident} === "object" && @{raw_var_ident} !== null) {
                                                        instance.@{field._field_ident} = new Map(
                                                            Object.entries(@{raw_var_ident} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    }

                                                {:case TypeCategory::Set(inner)}
                                                    if (Array.isArray(@{raw_var_ident})) {
                                                        instance.@{field._field_ident} = new Set(@{raw_var_ident} as @{inner}[]);
                                                    }

                                                {:case TypeCategory::Serializable(inner_type_name)}
                                                    {$let inner_type_expr: Expr = ident!(inner_type_name).into()}
                                                    {
                                                        const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                        ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                    }

                                                {:case TypeCategory::Nullable(_)}
                                                    {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            instance.@{field._field_ident} = @{raw_var_ident};
                                                        {:case SerdeValueKind::Date}
                                                            if (@{raw_var_ident} === null) {
                                                                instance.@{field._field_ident} = null;
                                                            } else {
                                                                instance.@{field._field_ident} = typeof @{raw_var_ident} === "string"
                                                                    ? new Date(@{raw_var_ident} as any)
                                                                    : @{raw_var_ident} as any;
                                                            }
                                                        {:case _}
                                                            if (@{raw_var_ident} === null) {
                                                                instance.@{field._field_ident} = null;
                                                            } else {
                                                                {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                                    {$let inner_type_expr: Expr = ident!(inner_type).into()}
                                                                    const __result = @{inner_type_expr}.deserializeWithContext(@{raw_var_ident}, ctx);
                                                                    ctx.assignOrDefer(instance, "@{field._field_name}", __result);
                                                                {:else}
                                                                    instance.@{field._field_ident} = @{raw_var_ident};
                                                                {/if}
                                                            }
                                                    {/match}

                                                {:case _}
                                                    instance.@{field._field_ident} = @{raw_var_ident};
                                            {/match}
                                        }
                                    {/if}
                                    {/if}
                                {/for}
                            {/if}

                            if (errors.length > 0) {
                                throw new @{deserialize_error_expr}(errors);
                            }

                            return instance as @{type_ident};
                        }

                        export function @{fn_validate_field_ident}(
                            _field: K,
                            _value: @{type_ident}[K]
                        ): Array<{ field: string; message: string }> {
                            {#if !fields_with_validators.is_empty()}
                            const errors: Array<{ field: string; message: string }> = [];
                            {#for field in &fields_with_validators}
                            if (_field === "@{field._field_name}") {
                                const __val = _value as @{field.ts_type};
                                {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                                {$typescript validation_code}

                            }
                            {/for}
                            return errors;
                            {:else}
                            return [];
                            {/if}
                        }

                        export function @{fn_validate_fields_ident}(
                            _partial: Partial<@{type_ident}>
                        ): Array<{ field: string; message: string }> {
                            {#if !fields_with_validators.is_empty()}
                            const errors: Array<{ field: string; message: string }> = [];
                            {#for field in &fields_with_validators}
                            if ("@{field._field_name}" in _partial && _partial.@{field._field_ident} !== undefined) {
                                const __val = _partial.@{field._field_ident} as @{field.ts_type};
                                {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                                {$typescript validation_code}

                            }
                            {/for}
                            return errors;
                            {:else}
                            return [];
                            {/if}
                        }

                        export function @{fn_is_ident}(obj: unknown): obj is @{full_type_ident} {
                            if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                                return false;
                            }
                            {#if has_required}
                                const o = obj as Record<string, unknown>;
                                return @{shape_check_condition};
                            {:else}
                                return true;
                            {/if}
                        }
                    }
                };

                result.add_aliased_import("DeserializeContext", "macroforge/serde");
                result.add_aliased_import("DeserializeError", "macroforge/serde");
                result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
                result.add_aliased_import("PendingRef", "macroforge/serde");
                Ok(result)
            } else if let Some(members) = type_alias.as_union() {
                // Union type - could be literal union, type ref union, or mixed

                // Build generic type signature if type has type params
                let type_params = type_alias.type_params();
                let (generic_decl, generic_args) = if type_params.is_empty() {
                    (String::new(), String::new())
                } else {
                    let params = type_params.join(", ");
                    (format!("<{}>", params), format!("<{}>", params))
                };
                let full_type_name = format!("{}{}", type_name, generic_args);

                // Create a set of type parameter names for filtering
                let type_param_set: std::collections::HashSet<&str> =
                    type_params.iter().map(|s| s.as_str()).collect();

                let literals: Vec<String> = members
                    .iter()
                    .filter_map(|m| m.as_literal().map(|s| s.to_string()))
                    .collect();
                let type_refs: Vec<String> = members
                    .iter()
                    .filter_map(|m| m.as_type_ref().map(|s| s.to_string()))
                    .collect();

                // Separate primitives, generic type params, and serializable types
                let primitive_types: Vec<String> = type_refs
                    .iter()
                    .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Primitive))
                    .cloned()
                    .collect();

                // Generic type parameters (like T, U) - these are passed through as-is
                let generic_type_params: Vec<String> = type_refs
                    .iter()
                    .filter(|t| type_param_set.contains(t.as_str()))
                    .cloned()
                    .collect();

                // Build SerializableTypeRef with both full type and base type for runtime access
                let serializable_types: Vec<SerializableTypeRef> = type_refs
                    .iter()
                    .filter(|t| {
                        !matches!(
                            TypeCategory::from_ts_type(t),
                            TypeCategory::Primitive | TypeCategory::Date
                        ) && !type_param_set.contains(t.as_str())
                    })
                    .map(|t| SerializableTypeRef {
                        full_type: t.clone(),
                    })
                    .collect();

                let date_types: Vec<String> = type_refs
                    .iter()
                    .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Date))
                    .cloned()
                    .collect();

                let has_primitives = !primitive_types.is_empty();
                let has_serializables = !serializable_types.is_empty();
                let has_dates = !date_types.is_empty();
                let has_generic_params = !generic_type_params.is_empty();

                let is_literal_only = !literals.is_empty() && type_refs.is_empty();
                let _is_primitive_only = has_primitives
                    && !has_serializables
                    && !has_dates
                    && !has_generic_params
                    && literals.is_empty();
                let _is_serializable_only = !has_primitives
                    && !has_dates
                    && !has_generic_params
                    && has_serializables
                    && literals.is_empty();
                let has_literals = !literals.is_empty();

                // Pre-compute the expected types string for error messages
                let _expected_types_str = if has_serializables {
                    serializable_types
                        .iter()
                        .map(|t| t.full_type.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    type_refs.join(", ")
                };

                let _primitive_check_condition: String = if primitive_types.is_empty() {
                    "false".to_string()
                } else {
                    primitive_types
                        .iter()
                        .map(|prim| format!("typeof value === \"{}\"", prim))
                        .collect::<Vec<_>>()
                        .join(" || ")
                };

                let serializable_type_check_condition: String = if serializable_types.is_empty() {
                    "false".to_string()
                } else {
                    serializable_types
                        .iter()
                        .map(|type_ref| format!("__typeName === \"{}\"", type_ref.full_type))
                        .collect::<Vec<_>>()
                        .join(" || ")
                };

                let fn_deserialize_ident = ident!(
                    "{}Deserialize{}",
                    type_name.to_case(Case::Camel),
                    generic_decl
                );
                let fn_deserialize_internal_ident = ident!(
                    "{}DeserializeWithContext{}",
                    type_name.to_case(Case::Camel),
                    generic_decl
                );
                let fn_deserialize_internal_expr: Expr =
                    fn_deserialize_internal_ident.clone().into();
                let fn_is_ident = ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);

                // Compute return type and wrappers
                let return_type = deserialize_return_type(&full_type_name);
                let return_type_ident = ident!(return_type.as_str());
                let full_type_ident = ident!(full_type_name.as_str());
                let success_result = wrap_success("resultOrRef");
                let success_result_expr = parse_ts_expr(&success_result)
                    .expect("deserialize success wrapper should parse");
                let error_root_ref = wrap_error(&format!(
                    r#"[{{ field: "_root", message: "{}.deserialize: root cannot be a forward reference" }}]"#,
                    type_name
                ));
                let error_root_ref_expr = parse_ts_expr(&error_root_ref)
                    .expect("deserialize root error wrapper should parse");
                let error_from_catch = wrap_error("e.errors");
                let error_from_catch_expr = parse_ts_expr(&error_from_catch)
                    .expect("deserialize catch error wrapper should parse");
                let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
                let error_generic_message_expr = parse_ts_expr(&error_generic_message)
                    .expect("deserialize generic error wrapper should parse");

                let mut result = ts_template! {
                    /** Deserializes input to this type. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
                    export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                        try {
                            // Auto-detect: if string, parse as JSON first
                            const data = typeof input === "string" ? JSON.parse(input) : input;

                            const ctx = @{deserialize_context_expr}.create();
                            const resultOrRef = @{fn_deserialize_internal_expr}(data, ctx);

                            if (@{pending_ref_expr}.is(resultOrRef)) {
                                return @{error_root_ref_expr};
                            }

                            ctx.applyPatches();
                            if (opts?.freeze) {
                                ctx.freezeAll();
                            }
                            return @{success_result_expr};
                        } catch (e) {
                            if (e instanceof @{deserialize_error_expr}) {
                                return @{error_from_catch_expr};
                            }
                            const message = e instanceof Error ? e.message : String(e);
                            return @{error_generic_message_expr};
                        }
                    }

                    /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                    export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{full_type_ident} | @{pending_ref_ident} {
                                    if (value?.__ref !== undefined) {
                                        return ctx.getOrDefer(value.__ref) as @{full_type_ident} | @{pending_ref_ident};
                                    }

                                    {#if is_literal_only}
                                        const allowedValues = [@{literals.join(", ")}] as const;
                                        if (!allowedValues.includes(value)) {
                                            throw new @{deserialize_error_expr}([{
                                                field: "_root",
                                                message: "Invalid value for @{type_name}: expected one of " + allowedValues.map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
                                            }]);
                                        }
                                        return value as @{full_type_ident};
                                    {:else if is_primitive_only}
                                        {#for prim in &primitive_types}
                                            if (typeof value === "@{prim}") {
                                                return value as @{full_type_ident};
                                            }
                                        {/for}

                                        throw new @{deserialize_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: expected @{expected_types_str}, got " + typeof value
                                        }]);
                                    {:else if is_serializable_only}
                                        if (typeof value !== "object" || value === null) {
                                            throw new @{deserialize_error_expr}([{
                                                field: "_root",
                                                message: "@{type_name}.deserializeWithContext: expected an object"
                                            }]);
                                        }

                                        const __typeName = (value as any).__type;
                                        if (typeof __typeName !== "string") {
                                            throw new @{deserialize_error_expr}([{
                                                field: "_root",
                                                message: "@{type_name}.deserializeWithContext: missing __type field for union dispatch"
                                            }]);
                                        }

                                        {#for type_ref in &serializable_types}
                                            {$let deserialize_with_context_fn: Expr = ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                            if (__typeName === "@{type_ref.full_type}") {
                                                return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                            }
                                        {/for}

                                        throw new @{deserialize_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                        }]);
                                    {:else}
                                        {#if has_literals}
                                            const allowedLiterals = [@{literals.join(", ")}] as const;
                                            if (allowedLiterals.includes(value as any)) {
                                                return value as @{full_type_ident};
                                            }
                                        {/if}

                                        {#if has_primitives}
                                            {#for prim in &primitive_types}
                                                if (typeof value === "@{prim}") {
                                                    return value as @{full_type_ident};
                                                }
                                            {/for}
                                        {/if}

                                        {#if has_dates}
                                            if (value instanceof Date) {
                                                return value as @{full_type_ident};
                                            }
                                            if (typeof value === "string") {
                                                const __dateVal = new Date(value);
                                                if (!isNaN(__dateVal.getTime())) {
                                                    return __dateVal as unknown as @{full_type_ident};
                                                }
                                            }
                                        {/if}

                                        {#if has_serializables}
                                            if (typeof value === "object" && value !== null) {
                                                const __typeName = (value as any).__type;
                                                if (typeof __typeName === "string") {
                                                    {#for type_ref in &serializable_types}
                                                        {$let deserialize_with_context_fn: Expr = ident!(nested_deserialize_fn_name(&extract_base_type(&type_ref.full_type))).into()}
                                                        if (__typeName === "@{type_ref.full_type}") {
                                                            return @{deserialize_with_context_fn}(value, ctx) as @{full_type_ident};
                                                        }
                                                    {/for}
                                                }
                                            }
                                        {/if}

                                        {#if has_generic_params}
                                            return value as @{full_type_ident};
                                        {/if}

                                        throw new @{deserialize_error_expr}([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: value does not match any union member"
                                        }]);
                        {/if}
                    }

                    export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
                                    {#if is_literal_only}
                                        const allowedValues = [@{literals.join(", ")}] as const;
                                        return allowedValues.includes(value as any);
                                    {:else if is_primitive_only}
                                        return @{primitive_check_condition};
                                    {:else if is_serializable_only}
                                        if (typeof value !== "object" || value === null) {
                                            return false;
                                        }
                                        const __typeName = (value as any).__type;
                                        return @{serializable_type_check_condition};
                                    {:else}
                                        {#if has_literals}
                                            const allowedLiterals = [@{literals.join(", ")}] as const;
                                            if (allowedLiterals.includes(value as any)) return true;
                                        {/if}
                                        {#if has_primitives}
                                            {#for prim in &primitive_types}
                                                if (typeof value === "@{prim}") return true;
                                            {/for}
                                        {/if}
                                        {#if has_dates}
                                            if (value instanceof Date) return true;
                                        {/if}
                                        {#if has_serializables}
                                            if (typeof value === "object" && value !== null) {
                                                const __typeName = (value as any).__type;
                                                if (@{serializable_type_check_condition}) return true;
                                            }
                                        {/if}
                                        {#if has_generic_params}
                                            return true;
                                        {:else}
                                            return false;
                                        {/if}
                        {/if}
                    }
                };
                result.add_aliased_import("DeserializeContext", "macroforge/serde");
                result.add_aliased_import("DeserializeError", "macroforge/serde");
                result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
                result.add_aliased_import("PendingRef", "macroforge/serde");
                Ok(result)
            } else {
                // Fallback for other type alias forms (simple alias, tuple, etc.)
                let fn_deserialize_ident = ident!(
                    "{}Deserialize{}",
                    type_name.to_case(Case::Camel),
                    generic_decl
                );
                let fn_deserialize_internal_ident = ident!(
                    "{}DeserializeWithContext{}",
                    type_name.to_case(Case::Camel),
                    generic_args
                );
                let fn_deserialize_internal_expr: Expr =
                    fn_deserialize_internal_ident.clone().into();
                let fn_validate_field_ident = ident!(
                    "{}ValidateField{}",
                    type_name.to_case(Case::Camel),
                    validate_field_generic_decl
                );
                let fn_validate_fields_ident =
                    ident!("{}ValidateFields", type_name.to_case(Case::Camel));
                let fn_is_ident = ident!("{}Is{}", type_name.to_case(Case::Camel), generic_decl);

                // Compute return type and wrappers
                let return_type = deserialize_return_type(&full_type_name);
                let return_type_ident = ident!(return_type.as_str());
                let success_result = wrap_success("result");
                let success_result_expr = parse_ts_expr(&success_result)
                    .expect("deserialize success wrapper should parse");
                let error_from_catch = wrap_error("e.errors");
                let error_from_catch_expr = parse_ts_expr(&error_from_catch)
                    .expect("deserialize catch error wrapper should parse");
                let error_generic_message = wrap_error(r#"[{ field: "_root", message }]"#);
                let error_generic_message_expr = parse_ts_expr(&error_generic_message)
                    .expect("deserialize generic error wrapper should parse");

                let mut result = ts_template! {
                    /** Deserializes input to this type. Automatically detects whether input is a JSON string or object. @param input - JSON string or object to deserialize @param opts - Optional deserialization options @returns Result containing the deserialized value or validation errors */
                    export function @{fn_deserialize_ident}(input: unknown, opts?: @{deserialize_options_ident}): @{return_type_ident} {
                        try {
                            // Auto-detect: if string, parse as JSON first
                            const data = typeof input === "string" ? JSON.parse(input) : input;

                            const ctx = @{deserialize_context_expr}.create();
                            const result = @{fn_deserialize_internal_expr}(data, ctx);
                            ctx.applyPatches();
                            if (opts?.freeze) {
                                ctx.freezeAll();
                            }
                            return @{success_result_expr};
                        } catch (e) {
                            if (e instanceof @{deserialize_error_expr}) {
                                return @{error_from_catch_expr};
                            }
                            const message = e instanceof Error ? e.message : String(e);
                            return @{error_generic_message_expr};
                        }
                    }

                    /** Deserializes with an existing context for nested/cyclic object graphs. @param value - The raw value to deserialize @param ctx - The deserialization context */
                    export function @{fn_deserialize_internal_ident}(value: any, ctx: @{deserialize_context_ident}): @{full_type_ident} {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref) as @{full_type_ident};
                        }
                        return value as @{type_ident};
                    }

                    export function @{fn_validate_field_ident}<K extends keyof @{type_ident}>(
                        _field: K,
                        _value: @{type_ident}[K]
                    ): Array<{ field: string; message: string }> {
                        return [];
                    }

                    export function @{fn_validate_fields_ident}(
                        _partial: Partial<@{type_ident}>
                    ): Array<{ field: string; message: string }> {
                        return [];
                    }

                    export function @{fn_is_ident}(value: unknown): value is @{full_type_ident} {
                        return value != null;
                    }
                };
                result.add_aliased_import("DeserializeContext", "macroforge/serde");
                result.add_aliased_import("DeserializeError", "macroforge/serde");
                result.add_aliased_type_import("DeserializeOptions", "macroforge/serde");
                Ok(result)
            }
        }
    }
}

/// Generate TypeScript validation code for a field
///
/// This function generates inline validation code that pushes errors to an `errors` array
/// when validation fails. It's used in the deserialization process to validate field values.
///
/// # Arguments
/// * `validators` - List of validators to apply
/// * `value_var` - Variable name containing the value to validate (e.g., "__val", "__raw")
/// * `field_name` - JSON field name for error reporting
/// * `context_name` - Context name for error messages (e.g., class name)
///
/// # Returns
/// A TypeScript expression that runs the validation statements.
fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    field_name: &str,
    context_name: &str,
) -> TsStream {
    let mut code = String::new();

    for spec in validators {
        let condition = match &spec.validator {
            // String validators
            Validator::Email => format!("!/^[^\\s@]+@[^\\s@]+\\.[^\\s@]+$/.test({value_var})"),
            Validator::Url => format!("!/^https?:\\/\\/.+/.test({value_var})"),
            Validator::Uuid => format!(
                "!/^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{4}}-[0-9a-f]{{12}}$/i.test({value_var})"
            ),
            Validator::MaxLength(n) => format!("{value_var}.length > {n}"),
            Validator::MinLength(n) => format!("{value_var}.length < {n}"),
            Validator::Length(n) => format!("{value_var}.length !== {n}"),
            Validator::LengthRange(min, max) => {
                format!("{value_var}.length < {min} || {value_var}.length > {max}")
            }
            Validator::Pattern(pattern) => {
                let escaped = pattern.replace('\\', "\\\\").replace('"', "\\\"");
                format!("!/{escaped}/.test({value_var})")
            }
            Validator::NonEmpty => format!("{value_var}.trim().length === 0"),
            Validator::Trimmed => format!("{value_var} !== {value_var}.trim()"),
            Validator::Lowercase => format!("{value_var} !== {value_var}.toLowerCase()"),
            Validator::Uppercase => format!("{value_var} !== {value_var}.toUpperCase()"),
            Validator::Capitalized => format!(
                "{value_var}.length === 0 || {value_var}[0] !== {value_var}[0].toUpperCase() || {value_var}.slice(1) !== {value_var}.slice(1).toLowerCase()"
            ),
            Validator::Uncapitalized => {
                format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toLowerCase()")
            }
            Validator::StartsWith(prefix) => format!("!{value_var}.startsWith(\"{prefix}\")"),
            Validator::EndsWith(suffix) => format!("!{value_var}.endsWith(\"{suffix}\")"),
            Validator::Includes(text) => format!("!{value_var}.includes(\"{text}\")"),

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
                format!("{value_var}.getTime() <= new Date(\"{date}\").getTime()")
            }
            Validator::GreaterThanOrEqualToDate(date) => {
                format!("{value_var}.getTime() < new Date(\"{date}\").getTime()")
            }
            Validator::LessThanDate(date) => {
                format!("{value_var}.getTime() >= new Date(\"{date}\").getTime()")
            }
            Validator::LessThanOrEqualToDate(date) => {
                format!("{value_var}.getTime() > new Date(\"{date}\").getTime()")
            }
            Validator::BetweenDate(min, max) => format!(
                "{value_var}.getTime() < new Date(\"{min}\").getTime() || {value_var}.getTime() > new Date(\"{max}\").getTime()"
            ),

            // BigInt validators
            Validator::GreaterThanBigInt(n) => format!("{value_var} <= {n}n"),
            Validator::GreaterThanOrEqualToBigInt(n) => format!("{value_var} < {n}n"),
            Validator::LessThanBigInt(n) => format!("{value_var} >= {n}n"),
            Validator::LessThanOrEqualToBigInt(n) => format!("{value_var} > {n}n"),
            Validator::BetweenBigInt(min, max) => {
                format!("{value_var} < {min}n || {value_var} > {max}n")
            }
            Validator::PositiveBigInt => format!("{value_var} <= 0n"),
            Validator::NonNegativeBigInt => format!("{value_var} < 0n"),
            Validator::NegativeBigInt => format!("{value_var} >= 0n"),
            Validator::NonPositiveBigInt => format!("{value_var} > 0n"),

            // Custom validator
            Validator::Custom(fn_name) => format!("!{fn_name}({value_var})"),
        };

        let default_message =
            get_default_validator_message(&spec.validator, field_name, context_name);
        let message = spec.custom_message.as_ref().unwrap_or(&default_message);
        let escaped_message = message.replace('\\', "\\\\").replace('"', "\\\"");

        code.push_str(&format!(
            "                                if ({condition}) {{ errors.push({{ field: \"{field_name}\", message: \"{escaped_message}\" }}); }}\n"
        ));
    }

    TsStream::from_string(code)
}

/// Generate default error message for a validator
fn get_default_validator_message(
    validator: &Validator,
    field_name: &str,
    context_name: &str,
) -> String {
    match validator {
        Validator::Email => format!("{context_name}.{field_name} must be a valid email"),
        Validator::Url => format!("{context_name}.{field_name} must be a valid URL"),
        Validator::Uuid => format!("{context_name}.{field_name} must be a valid UUID"),
        Validator::MaxLength(n) => {
            format!("{context_name}.{field_name} must have at most {n} characters")
        }
        Validator::MinLength(n) => {
            format!("{context_name}.{field_name} must have at least {n} characters")
        }
        Validator::Length(n) => {
            format!("{context_name}.{field_name} must have exactly {n} characters")
        }
        Validator::LengthRange(min, max) => {
            format!("{context_name}.{field_name} must have between {min} and {max} characters")
        }
        Validator::Pattern(pattern) => {
            format!("{context_name}.{field_name} must match pattern {pattern}")
        }
        Validator::NonEmpty => format!("{context_name}.{field_name} must not be empty"),
        Validator::Trimmed => format!("{context_name}.{field_name} must be trimmed"),
        Validator::Lowercase => format!("{context_name}.{field_name} must be lowercase"),
        Validator::Uppercase => format!("{context_name}.{field_name} must be uppercase"),
        Validator::Capitalized => format!("{context_name}.{field_name} must be capitalized"),
        Validator::Uncapitalized => format!("{context_name}.{field_name} must be uncapitalized"),
        Validator::StartsWith(prefix) => {
            format!("{context_name}.{field_name} must start with '{prefix}'")
        }
        Validator::EndsWith(suffix) => {
            format!("{context_name}.{field_name} must end with '{suffix}'")
        }
        Validator::Includes(text) => format!("{context_name}.{field_name} must include '{text}'"),
        Validator::GreaterThan(n) => {
            format!("{context_name}.{field_name} must be greater than {n}")
        }
        Validator::GreaterThanOrEqualTo(n) => {
            format!("{context_name}.{field_name} must be greater than or equal to {n}")
        }
        Validator::LessThan(n) => format!("{context_name}.{field_name} must be less than {n}"),
        Validator::LessThanOrEqualTo(n) => {
            format!("{context_name}.{field_name} must be less than or equal to {n}")
        }
        Validator::Between(min, max) => {
            format!("{context_name}.{field_name} must be between {min} and {max}")
        }
        Validator::Int => format!("{context_name}.{field_name} must be an integer"),
        Validator::NonNaN => format!("{context_name}.{field_name} must not be NaN"),
        Validator::Finite => format!("{context_name}.{field_name} must be finite"),
        Validator::Positive => format!("{context_name}.{field_name} must be positive"),
        Validator::NonNegative => format!("{context_name}.{field_name} must be non-negative"),
        Validator::Negative => format!("{context_name}.{field_name} must be negative"),
        Validator::NonPositive => format!("{context_name}.{field_name} must be non-positive"),
        Validator::MultipleOf(n) => {
            format!("{context_name}.{field_name} must be a multiple of {n}")
        }
        Validator::Uint8 => format!("{context_name}.{field_name} must be a uint8"),
        Validator::MaxItems(n) => {
            format!("{context_name}.{field_name} must have at most {n} items")
        }
        Validator::MinItems(n) => {
            format!("{context_name}.{field_name} must have at least {n} items")
        }
        Validator::ItemsCount(n) => {
            format!("{context_name}.{field_name} must have exactly {n} items")
        }
        Validator::ValidDate => format!("{context_name}.{field_name} must be a valid date"),
        Validator::GreaterThanDate(date) => {
            format!("{context_name}.{field_name} must be after {date}")
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!("{context_name}.{field_name} must be on or after {date}")
        }
        Validator::LessThanDate(date) => {
            format!("{context_name}.{field_name} must be before {date}")
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!("{context_name}.{field_name} must be on or before {date}")
        }
        Validator::BetweenDate(min, max) => {
            format!("{context_name}.{field_name} must be between {min} and {max}")
        }
        Validator::GreaterThanBigInt(n) => {
            format!("{context_name}.{field_name} must be greater than {n}")
        }
        Validator::GreaterThanOrEqualToBigInt(n) => {
            format!("{context_name}.{field_name} must be greater than or equal to {n}")
        }
        Validator::LessThanBigInt(n) => {
            format!("{context_name}.{field_name} must be less than {n}")
        }
        Validator::LessThanOrEqualToBigInt(n) => {
            format!("{context_name}.{field_name} must be less than or equal to {n}")
        }
        Validator::BetweenBigInt(min, max) => {
            format!("{context_name}.{field_name} must be between {min} and {max}")
        }
        Validator::PositiveBigInt => format!("{context_name}.{field_name} must be positive"),
        Validator::NonNegativeBigInt => format!("{context_name}.{field_name} must be non-negative"),
        Validator::NegativeBigInt => format!("{context_name}.{field_name} must be negative"),
        Validator::NonPositiveBigInt => format!("{context_name}.{field_name} must be non-positive"),
        Validator::Custom(fn_name) => {
            format!("{context_name}.{field_name} failed custom validation ({fn_name})")
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
            _field_name: "email".into(),
            _field_ident: ident!("email"),
            ts_type: "string".into(),
            _type_cat: TypeCategory::Primitive,
            optional: false,
            has_default: false,
            _default_expr: None,
            flatten: false,
            validators: vec![ValidatorSpec {
                validator: Validator::Email,
                custom_message: None,
            }],
            _nullable_inner_kind: None,
            _array_elem_kind: None,
            _nullable_serializable_type: None,
            _deserialize_with: None,
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
}
