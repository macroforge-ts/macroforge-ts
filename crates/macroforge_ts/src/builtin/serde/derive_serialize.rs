//! # Serialize Macro Implementation
//!
//! The `Serialize` macro generates JSON serialization methods with **cycle detection**
//! and object identity tracking. This enables serialization of complex object graphs
//! including circular references.
//!
//! ## Generated Methods
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `toStringifiedJSON()`, `toObject()`, `__serialize(ctx)` | Instance methods |
//! | Enum | `toStringifiedJSONEnumName(value)`, `__serializeEnumName` | Standalone functions |
//! | Interface | `toStringifiedJSONInterfaceName(value)`, etc. | Standalone functions |
//! | Type Alias | `toStringifiedJSONTypeName(value)`, etc. | Standalone functions |
//!
//! ## Configuration
//!
//! The `functionNamingStyle` option in `macroforge.json` controls naming:
//! - `"suffix"` (default): Suffixes with type name (e.g., `toStringifiedJSONMyType`)
//! - `"prefix"`: Prefixes with type name (e.g., `myTypeToStringifiedJSON`)
//! - `"generic"`: Uses TypeScript generics (e.g., `toStringifiedJSON<T extends MyType>`)
//! - `"namespace"`: Legacy namespace wrapping
//!
//! ## Cycle Detection Protocol
//!
//! The generated code handles circular references using `__id` and `__ref` markers:
//!
//! ```json
//! {
//!     "__type": "User",
//!     "__id": 1,
//!     "name": "Alice",
//!     "friend": { "__ref": 2 }  // Reference to object with __id: 2
//! }
//! ```
//!
//! When an object is serialized:
//! 1. Check if it's already been serialized (has an `__id`)
//! 2. If so, return `{ "__ref": existingId }` instead
//! 3. Otherwise, register the object and serialize its fields
//!
//! ## Type-Specific Serialization
//!
//! | Type | Serialization Strategy |
//! |------|------------------------|
//! | Primitives | Direct value |
//! | `Date` | `toISOString()` |
//! | Arrays | For primitive-like element types, pass through; for `Date`/`Date | null`, map to ISO strings; otherwise map and call `__serialize(ctx)` when available |
//! | `Map<K,V>` | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date | null`, convert to ISO strings; otherwise call `__serialize(ctx)` per value when available |
//! | `Set<T>` | Convert to array; element handling matches `Array<T>` |
//! | Nullable | Include `null` explicitly; for primitive-like and `Date` unions the generator avoids runtime `__serialize` checks |
//! | Objects | Call `__serialize(ctx)` if available (to support user-defined implementations) |
//!
//! Note: the generator specializes some code paths based on the declared TypeScript type to
//! avoid runtime feature detection on primitives and literal unions.
//!
//! ## Field-Level Options
//!
//! The `@serde` decorator supports:
//!
//! - `skip` / `skip_serializing` - Exclude field from serialization
//! - `rename = "jsonKey"` - Use different JSON property name
//! - `flatten` - Merge nested object's fields into parent
//!
//! ## Example
//!
//! ```typescript
//! @derive(Serialize)
//! class User {
//!     id: number;
//!
//!     @serde(rename = "userName")
//!     name: string;
//!
//!     @serde(skip_serializing)
//!     password: string;
//!
//!     @serde(flatten)
//!     metadata: UserMetadata;
//! }
//!
//! // Usage:
//! const user = new User();
//! const json = user.toStringifiedJSON();
//! // => '{"__type":"User","__id":1,"id":1,"userName":"Alice",...}'
//!
//! const obj = user.toObject();
//! // => { __type: "User", __id: 1, id: 1, userName: "Alice", ... }
//! ```
//!
//! ## Required Import
//!
//! The generated code automatically imports `SerializeContext` from `macroforge/serde`.

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::abi::{DiagnosticCollector, FunctionNamingStyle};
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_macro_input,
};

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory};

/// Convert a PascalCase name to camelCase (for prefix naming style)
fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
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

/// Contains field information needed for JSON serialization code generation.
///
/// Each field that should be included in serialization is represented by this struct,
/// capturing the JSON key name, field access name, type category, and serialization options.
#[derive(Clone)]
struct SerializeField {
    /// The JSON property name to use in the serialized output.
    /// This may differ from `field_name` if `@serde(rename = "...")` is used.
    json_key: String,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property access expressions like `this.fieldName`.
    field_name: String,

    /// The category of the field's type, used to select the appropriate
    /// serialization strategy (primitive, Date, Array, Map, Set, etc.).
    type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier).
    /// Optional fields are wrapped in `if (value !== undefined)` checks.
    optional: bool,

    /// Whether the field should be flattened into the parent object.
    /// Flattened fields have their properties merged directly into the parent
    /// rather than being nested under their field name.
    flatten: bool,

    /// For `T | undefined` unions: classification of `T`.
    optional_inner_kind: Option<SerdeValueKind>,
    /// For `T | null` unions: classification of `T`.
    nullable_inner_kind: Option<SerdeValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    array_elem_kind: Option<SerdeValueKind>,
    /// For `Set<T>`: classification of `T`.
    set_elem_kind: Option<SerdeValueKind>,
    /// For `Map<K, V>`: classification of `V`.
    map_value_kind: Option<SerdeValueKind>,

    // --- Serializable type tracking for direct function calls ---
    /// For `T | undefined` where T is Serializable: the type name.
    optional_serializable_type: Option<String>,
    /// For `T | null` where T is Serializable: the type name.
    nullable_serializable_type: Option<String>,
    /// For `Array<T>` where T is Serializable: the type name.
    array_elem_serializable_type: Option<String>,
    /// For `Set<T>` where T is Serializable: the type name.
    set_elem_serializable_type: Option<String>,
    /// For `Map<K, V>` where V is Serializable: the type name.
    map_value_serializable_type: Option<String>,
}

#[ts_macro_derive(
    Serialize,
    description = "Generates serialization methods with cycle detection (toStringifiedJSON, __serialize)",
    attributes((serde, "Configure serialization for this field. Options: skip, rename, flatten"))
)]
pub fn derive_serialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Collect serializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<SerializeField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result =
                        SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

                    if !opts.should_serialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    let optional_inner_kind = match &type_cat {
                        TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let nullable_inner_kind = match &type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let array_elem_kind = match &type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let set_elem_kind = match &type_cat {
                        TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let map_value_kind = match &type_cat {
                        TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let optional_serializable_type = match &type_cat {
                        TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let nullable_serializable_type = match &type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let array_elem_serializable_type = match &type_cat {
                        TypeCategory::Array(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let set_elem_serializable_type = match &type_cat {
                        TypeCategory::Set(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let map_value_serializable_type = match &type_cat {
                        TypeCategory::Map(_, value) => get_serializable_type_name(value),
                        _ => None,
                    };

                    Some(SerializeField {
                        json_key,
                        field_name: field.name.clone(),
                        type_cat,
                        optional: field.optional,
                        flatten: opts.flatten,
                        optional_inner_kind,
                        nullable_inner_kind,
                        array_elem_kind,
                        set_elem_kind,
                        map_value_kind,
                        optional_serializable_type,
                        nullable_serializable_type,
                        array_elem_serializable_type,
                        set_elem_serializable_type,
                        map_value_serializable_type,
                    })
                })
                .collect();

            // Check for errors in field parsing before continuing
            if all_diagnostics.has_errors() {
                return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
            }

            // Separate regular fields from flattened fields
            let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            let has_regular = !regular_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();

            let mut result = body! {
                toStringifiedJSON(): string {
                    const ctx = SerializeContext.create();
                    return JSON.stringify(this.__serialize(ctx));
                }

                toObject(): Record<string, unknown> {
                    const ctx = SerializeContext.create();
                    return this.__serialize(ctx);
                }

                __serialize(ctx: SerializeContext): Record<string, unknown> {
                    // Check if already serialized (cycle detection)
                    const existingId = ctx.getId(this);
                    if (existingId !== undefined) {
                        return { __ref: existingId };
                    }

                    // Register this object
                    const __id = ctx.register(this);

                    const result: Record<string, unknown> = {
                        __type: "@{class_name}",
                        __id,
                    };

                    {#if has_regular}
                        {#for field in regular_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Primitive}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name};
                                    {/if}

                                {:case TypeCategory::Date}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name}.toISOString();
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name}.toISOString();
                                    {/if}

                                {:case TypeCategory::Array(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result["@{field.json_key}"] = this.@{field.field_name};
                                                {:case SerdeValueKind::Date}
                                                    result["@{field.json_key}"] = this.@{field.field_name}.map((item: Date) => item.toISOString());
                                                {:case SerdeValueKind::NullableDate}
                                                    result["@{field.json_key}"] = this.@{field.field_name}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                        result["@{field.json_key}"] = this.@{field.field_name}.map(
                                                            (item) => __serialize@{elem_type}(item, ctx)
                                                        );
                                                    {:else}
                                                        result["@{field.json_key}"] = this.@{field.field_name};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result["@{field.json_key}"] = this.@{field.field_name};
                                            {:case SerdeValueKind::Date}
                                                result["@{field.json_key}"] = this.@{field.field_name}.map((item: Date) => item.toISOString());
                                            {:case SerdeValueKind::NullableDate}
                                                result["@{field.json_key}"] = this.@{field.field_name}.map((item: Date | null) => item === null ? null : item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                    result["@{field.json_key}"] = this.@{field.field_name}.map(
                                                        (item) => __serialize@{elem_type}(item, ctx)
                                                    );
                                                {:else}
                                                    result["@{field.json_key}"] = this.@{field.field_name};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Map(_, _)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result["@{field.json_key}"] = Object.fromEntries(this.@{field.field_name}.entries());
                                                {:case SerdeValueKind::Date}
                                                    result["@{field.json_key}"] = Object.fromEntries(
                                                        Array.from(this.@{field.field_name}.entries()).map(
                                                            ([k, v]) => [k, (v as Date).toISOString()]
                                                        )
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    result["@{field.json_key}"] = Object.fromEntries(
                                                        Array.from(this.@{field.field_name}.entries()).map(
                                                            ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                        )
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type) = &field.map_value_serializable_type}
                                                        result["@{field.json_key}"] = Object.fromEntries(
                                                            Array.from(this.@{field.field_name}.entries()).map(
                                                                ([k, v]) => [k, __serialize@{value_type}(v, ctx)]
                                                            )
                                                        );
                                                    {:else}
                                                        result["@{field.json_key}"] = Object.fromEntries(this.@{field.field_name}.entries());
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result["@{field.json_key}"] = Object.fromEntries(this.@{field.field_name}.entries());
                                            {:case SerdeValueKind::Date}
                                                result["@{field.json_key}"] = Object.fromEntries(
                                                    Array.from(this.@{field.field_name}.entries()).map(
                                                        ([k, v]) => [k, (v as Date).toISOString()]
                                                    )
                                                );
                                            {:case SerdeValueKind::NullableDate}
                                                result["@{field.json_key}"] = Object.fromEntries(
                                                    Array.from(this.@{field.field_name}.entries()).map(
                                                        ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                    )
                                                );
                                            {:case _}
                                                {#if let Some(value_type) = &field.map_value_serializable_type}
                                                    result["@{field.json_key}"] = Object.fromEntries(
                                                        Array.from(this.@{field.field_name}.entries()).map(
                                                            ([k, v]) => [k, __serialize@{value_type}(v, ctx)]
                                                        )
                                                    );
                                                {:else}
                                                    result["@{field.json_key}"] = Object.fromEntries(this.@{field.field_name}.entries());
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Set(_)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result["@{field.json_key}"] = Array.from(this.@{field.field_name});
                                                {:case SerdeValueKind::Date}
                                                    result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map((item: Date) => item.toISOString());
                                                {:case SerdeValueKind::NullableDate}
                                                    result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field.set_elem_serializable_type}
                                                        result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map(
                                                            (item) => __serialize@{elem_type}(item, ctx)
                                                        );
                                                    {:else}
                                                        result["@{field.json_key}"] = Array.from(this.@{field.field_name});
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result["@{field.json_key}"] = Array.from(this.@{field.field_name});
                                            {:case SerdeValueKind::Date}
                                                result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map((item: Date) => item.toISOString());
                                            {:case SerdeValueKind::NullableDate}
                                                result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map((item: Date | null) => item === null ? null : item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field.set_elem_serializable_type}
                                                    result["@{field.json_key}"] = Array.from(this.@{field.field_name}).map(
                                                        (item) => __serialize@{elem_type}(item, ctx)
                                                    );
                                                {:else}
                                                    result["@{field.json_key}"] = Array.from(this.@{field.field_name});
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Optional(_)}
                                    if (this.@{field.field_name} !== undefined) {
                                        {#match field.optional_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result["@{field.json_key}"] = this.@{field.field_name};
                                            {:case SerdeValueKind::Date}
                                                result["@{field.json_key}"] = (this.@{field.field_name} as Date).toISOString();
                                            {:case _}
                                                {#if let Some(inner_type) = &field.optional_serializable_type}
                                                    result["@{field.json_key}"] = __serialize@{inner_type}(this.@{field.field_name}, ctx);
                                                {:else}
                                                    result["@{field.json_key}"] = this.@{field.field_name};
                                                {/if}
                                        {/match}
                                    }

                                {:case TypeCategory::Nullable(_)}
                                    {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                        {:case SerdeValueKind::PrimitiveLike}
                                            result["@{field.json_key}"] = this.@{field.field_name};
                                        {:case SerdeValueKind::Date}
                                            result["@{field.json_key}"] = this.@{field.field_name} === null
                                                ? null
                                                : (this.@{field.field_name} as Date).toISOString();
                                        {:case _}
                                            if (this.@{field.field_name} !== null) {
                                                {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                    result["@{field.json_key}"] = __serialize@{inner_type}(this.@{field.field_name}, ctx);
                                                {:else}
                                                    result["@{field.json_key}"] = this.@{field.field_name};
                                                {/if}
                                            } else {
                                                result["@{field.json_key}"] = null;
                                            }
                                    {/match}

                                {:case TypeCategory::Serializable(type_name)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = __serialize@{type_name}(this.@{field.field_name}, ctx);
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = __serialize@{type_name}(this.@{field.field_name}, ctx);
                                    {/if}

                                {:case TypeCategory::Unknown}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            result["@{field.json_key}"] = this.@{field.field_name};
                                        }
                                    {:else}
                                        result["@{field.json_key}"] = this.@{field.field_name};
                                    {/if}
                            {/match}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            const __flattened = __serialize@{type_name}(this.@{field.field_name}, ctx);
                                            // Remove __type and __id from flattened object
                                            const { __type: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = __serialize@{type_name}(this.@{field.field_name}, ctx);
                                            // Remove __type and __id from flattened object
                                            const { __type: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                                {:case _}
                                    {#if field.optional}
                                        if (this.@{field.field_name} !== undefined) {
                                            const __flattened = this.@{field.field_name};
                                            // Remove __type and __id from flattened object
                                            const { __type: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = this.@{field.field_name};
                                            // Remove __type and __id from flattened object
                                            const { __type: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                            {/match}
                        {/for}
                    {/if}

                    return result;
                }
            };
            result.add_import("SerializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::Enum(_) => {
            // Enums: return the underlying value directly
            let enum_name = input.name();
            let naming_style = input.context.function_naming_style;

            match naming_style {
                FunctionNamingStyle::Namespace => Ok(ts_template! {
                    export namespace @{enum_name} {
                        export function toStringifiedJSON(value: @{enum_name}): string {
                            return JSON.stringify(value);
                        }

                        export function __serialize(_ctx: SerializeContext): string | number {
                            return value;
                        }
                    }
                }),
                FunctionNamingStyle::Generic => Ok(ts_template! {
                    export function toStringifiedJSON<T extends @{enum_name}>(value: T): string {
                        return JSON.stringify(value);
                    }

                    export function __serialize<T extends @{enum_name}>(_ctx: SerializeContext): T {
                        return value;
                    }
                }),
                FunctionNamingStyle::Prefix => {
                    let fn_name_json = format!("{}ToStringifiedJSON", to_camel_case(enum_name));
                    let fn_name_serialize = format!("{}__serialize", to_camel_case(enum_name));
                    Ok(ts_template! {
                        export function @{fn_name_json}(value: @{enum_name}): string {
                            return JSON.stringify(value);
                        }

                        export function @{fn_name_serialize}(_ctx: SerializeContext): string | number {
                            return value;
                        }
                    })
                }
                FunctionNamingStyle::Suffix => {
                    let fn_name_json = format!("toStringifiedJSON{}", enum_name);
                    let fn_name_serialize = format!("__serialize{}", enum_name);
                    Ok(ts_template! {
                        export function @{fn_name_json}(value: @{enum_name}): string {
                            return JSON.stringify(value);
                        }

                        export function @{fn_name_serialize}(_ctx: SerializeContext): string | number {
                            return value;
                        }
                    })
                }
            }
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let naming_style = input.context.function_naming_style;
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect serializable fields from interface with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<SerializeField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result =
                        SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

                    if !opts.should_serialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    let optional_inner_kind = match &type_cat {
                        TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let nullable_inner_kind = match &type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let array_elem_kind = match &type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let set_elem_kind = match &type_cat {
                        TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let map_value_kind = match &type_cat {
                        TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let optional_serializable_type = match &type_cat {
                        TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let nullable_serializable_type = match &type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let array_elem_serializable_type = match &type_cat {
                        TypeCategory::Array(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let set_elem_serializable_type = match &type_cat {
                        TypeCategory::Set(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let map_value_serializable_type = match &type_cat {
                        TypeCategory::Map(_, value) => get_serializable_type_name(value),
                        _ => None,
                    };

                    Some(SerializeField {
                        json_key,
                        field_name: field.name.clone(),
                        type_cat,
                        optional: field.optional,
                        flatten: opts.flatten,
                        optional_inner_kind,
                        nullable_inner_kind,
                        array_elem_kind,
                        set_elem_kind,
                        map_value_kind,
                        optional_serializable_type,
                        nullable_serializable_type,
                        array_elem_serializable_type,
                        set_elem_serializable_type,
                        map_value_serializable_type,
                    })
                })
                .collect();

            // Check for errors in field parsing before continuing
            if all_diagnostics.has_errors() {
                return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
            }

            // Separate regular fields from flattened fields
            let regular_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            let has_regular = !regular_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();

            // Generate function names based on naming style
            let (fn_to_json, fn_to_obj, fn_serialize) = match naming_style {
                FunctionNamingStyle::Namespace => (
                    "toStringifiedJSON".to_string(),
                    "toObject".to_string(),
                    "__serialize".to_string(),
                ),
                FunctionNamingStyle::Generic => (
                    "toStringifiedJSON".to_string(),
                    "toObject".to_string(),
                    "__serialize".to_string(),
                ),
                FunctionNamingStyle::Prefix => (
                    format!("{}ToStringifiedJSON", to_camel_case(interface_name)),
                    format!("{}ToObject", to_camel_case(interface_name)),
                    format!("{}__serialize", to_camel_case(interface_name)),
                ),
                FunctionNamingStyle::Suffix => (
                    format!("toStringifiedJSON{}", interface_name),
                    format!("toObject{}", interface_name),
                    format!("__serialize{}", interface_name),
                ),
            };

            let mut result = match naming_style {
                FunctionNamingStyle::Namespace => {
                    ts_template! {
                        export namespace @{interface_name} {
                            export function toStringifiedJSON(self: @{interface_name}): string {
                                const ctx = SerializeContext.create();
                                return JSON.stringify(__serialize(self, ctx));
                            }

                            export function toObject(self: @{interface_name}): Record<string, unknown> {
                                const ctx = SerializeContext.create();
                                return __serialize(self, ctx);
                            }

                            export function __serialize(self: @{interface_name}, ctx: SerializeContext): Record<string, unknown> {
                                // Check if already serialized (cycle detection)
                                const existingId = ctx.getId(self);
                                if (existingId !== undefined) {
                                    return { __ref: existingId };
                                }

                                // Register this object
                                const __id = ctx.register(self);

                                const result: Record<string, unknown> = {
                                    __type: "@{interface_name}",
                                    __id,
                                };

                                {#if has_regular}
                                    {#for field in regular_fields}
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Primitive}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        result["@{field.json_key}"] = self.@{field.field_name};
                                                    }
                                                {:else}
                                                    result["@{field.json_key}"] = self.@{field.field_name};
                                                {/if}

                                            {:case TypeCategory::Date}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        result["@{field.json_key}"] = self.@{field.field_name}.toISOString();
                                                    }
                                                {:else}
                                                    result["@{field.json_key}"] = self.@{field.field_name}.toISOString();
                                                {/if}

                                            {:case TypeCategory::Array(_)}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                            {:case SerdeValueKind::PrimitiveLike}
                                                                result["@{field.json_key}"] = self.@{field.field_name};
                                                            {:case SerdeValueKind::Date}
                                                                result["@{field.json_key}"] = self.@{field.field_name}.map((item: Date) => item.toISOString());
                                                            {:case SerdeValueKind::NullableDate}
                                                                result["@{field.json_key}"] = self.@{field.field_name}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                            {:case _}
                                                                {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                                    result["@{field.json_key}"] = self.@{field.field_name}.map(
                                                                        (item) => __serialize@{elem_type}(item, ctx)
                                                                    );
                                                                {:else}
                                                                    result["@{field.json_key}"] = self.@{field.field_name};
                                                                {/if}
                                                        {/match}
                                                    }
                                                {:else}
                                                    {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = self.@{field.field_name};
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = self.@{field.field_name}.map((item: Date) => item.toISOString());
                                                        {:case SerdeValueKind::NullableDate}
                                                            result["@{field.json_key}"] = self.@{field.field_name}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                        {:case _}
                                                            {#if let Some(elem_type) = &field.array_elem_serializable_type}
                                                                result["@{field.json_key}"] = self.@{field.field_name}.map(
                                                                    (item) => __serialize@{elem_type}(item, ctx)
                                                                );
                                                            {:else}
                                                                result["@{field.json_key}"] = self.@{field.field_name};
                                                            {/if}
                                                    {/match}
                                                {/if}

                                            {:case TypeCategory::Map(_, _)}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                            {:case SerdeValueKind::PrimitiveLike}
                                                                result["@{field.json_key}"] = Object.fromEntries(self.@{field.field_name}.entries());
                                                            {:case SerdeValueKind::Date}
                                                                result["@{field.json_key}"] = Object.fromEntries(
                                                                    Array.from(self.@{field.field_name}.entries()).map(
                                                                        ([k, v]) => [k, (v as Date).toISOString()]
                                                                    )
                                                                );
                                                            {:case SerdeValueKind::NullableDate}
                                                                result["@{field.json_key}"] = Object.fromEntries(
                                                                    Array.from(self.@{field.field_name}.entries()).map(
                                                                        ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                                    )
                                                                );
                                                            {:case _}
                                                                {#if let Some(value_type) = &field.map_value_serializable_type}
                                                                    result["@{field.json_key}"] = Object.fromEntries(
                                                                        Array.from(self.@{field.field_name}.entries()).map(
                                                                            ([k, v]) => [k, __serialize@{value_type}(v, ctx)]
                                                                        )
                                                                    );
                                                                {:else}
                                                                    result["@{field.json_key}"] = Object.fromEntries(self.@{field.field_name}.entries());
                                                                {/if}
                                                        {/match}
                                                    }
                                                {:else}
                                                    {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = Object.fromEntries(self.@{field.field_name}.entries());
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = Object.fromEntries(
                                                                Array.from(self.@{field.field_name}.entries()).map(
                                                                    ([k, v]) => [k, (v as Date).toISOString()]
                                                                )
                                                            );
                                                        {:case SerdeValueKind::NullableDate}
                                                            result["@{field.json_key}"] = Object.fromEntries(
                                                                Array.from(self.@{field.field_name}.entries()).map(
                                                                    ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                                )
                                                            );
                                                        {:case _}
                                                            {#if let Some(value_type) = &field.map_value_serializable_type}
                                                                result["@{field.json_key}"] = Object.fromEntries(
                                                                    Array.from(self.@{field.field_name}.entries()).map(
                                                                        ([k, v]) => [k, __serialize@{value_type}(v, ctx)]
                                                                    )
                                                                );
                                                            {:else}
                                                                result["@{field.json_key}"] = Object.fromEntries(self.@{field.field_name}.entries());
                                                            {/if}
                                                    {/match}
                                                {/if}

                                            {:case TypeCategory::Set(_)}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                            {:case SerdeValueKind::PrimitiveLike}
                                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name});
                                                            {:case SerdeValueKind::Date}
                                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map((item: Date) => item.toISOString());
                                                            {:case SerdeValueKind::NullableDate}
                                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                            {:case _}
                                                                {#if let Some(elem_type) = &field.set_elem_serializable_type}
                                                                    result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map(
                                                                        (item) => __serialize@{elem_type}(item, ctx)
                                                                    );
                                                                {:else}
                                                                    result["@{field.json_key}"] = Array.from(self.@{field.field_name});
                                                                {/if}
                                                        {/match}
                                                    }
                                                {:else}
                                                    {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = Array.from(self.@{field.field_name});
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map((item: Date) => item.toISOString());
                                                        {:case SerdeValueKind::NullableDate}
                                                            result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                        {:case _}
                                                            {#if let Some(elem_type) = &field.set_elem_serializable_type}
                                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name}).map(
                                                                    (item) => __serialize@{elem_type}(item, ctx)
                                                                );
                                                            {:else}
                                                                result["@{field.json_key}"] = Array.from(self.@{field.field_name});
                                                            {/if}
                                                    {/match}
                                                {/if}

                                            {:case TypeCategory::Optional(_)}
                                                if (self.@{field.field_name} !== undefined) {
                                                    {#match field.optional_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = self.@{field.field_name};
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = (self.@{field.field_name} as Date).toISOString();
                                                        {:case _}
                                                            {#if let Some(inner_type) = &field.optional_serializable_type}
                                                                result["@{field.json_key}"] = __serialize@{inner_type}(self.@{field.field_name}, ctx);
                                                            {:else}
                                                                result["@{field.json_key}"] = self.@{field.field_name};
                                                            {/if}
                                                    {/match}
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        result["@{field.json_key}"] = self.@{field.field_name};
                                                    {:case SerdeValueKind::Date}
                                                        result["@{field.json_key}"] = self.@{field.field_name} === null
                                                            ? null
                                                            : (self.@{field.field_name} as Date).toISOString();
                                                    {:case _}
                                                        if (self.@{field.field_name} !== null) {
                                                            {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                                result["@{field.json_key}"] = __serialize@{inner_type}(self.@{field.field_name}, ctx);
                                                            {:else}
                                                                result["@{field.json_key}"] = self.@{field.field_name};
                                                            {/if}
                                                        } else {
                                                            result["@{field.json_key}"] = null;
                                                        }
                                                {/match}

                                            {:case TypeCategory::Serializable(type_name)}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        result["@{field.json_key}"] = __serialize@{type_name}(self.@{field.field_name}, ctx);
                                                    }
                                                {:else}
                                                    result["@{field.json_key}"] = __serialize@{type_name}(self.@{field.field_name}, ctx);
                                                {/if}

                                            {:case TypeCategory::Unknown}
                                                {#if field.optional}
                                                    if (self.@{field.field_name} !== undefined) {
                                                        result["@{field.json_key}"] = self.@{field.field_name};
                                                    }
                                                {:else}
                                                    result["@{field.json_key}"] = self.@{field.field_name};
                                                {/if}
                                        {/match}
                                    {/for}
                                {/if}

                                {#if has_flatten}
                                    {#for field in flatten_fields}
                                        {#if field.optional}
                                            if (self.@{field.field_name} !== undefined) {
                                                const __flattened = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                                    ? (self.@{field.field_name} as any).__serialize(ctx)
                                                    : self.@{field.field_name};
                                                const { __type: _, __id: __, ...rest } = __flattened as any;
                                                Object.assign(result, rest);
                                            }
                                        {:else}
                                            {
                                                const __flattened = typeof (self.@{field.field_name} as any)?.__serialize === "function"
                                                    ? (self.@{field.field_name} as any).__serialize(ctx)
                                                    : self.@{field.field_name};
                                                const { __type: _, __id: __, ...rest } = __flattened as any;
                                                Object.assign(result, rest);
                                            }
                                        {/if}
                                    {/for}
                                {/if}

                                return result;
                            }
                        }
                    }
                }
                _ => {
                    // For non-namespace styles (Generic, Prefix, Suffix), use standalone functions
                    ts_template! {
                        export function @{fn_to_json}(value: @{interface_name}): string {
                            const ctx = SerializeContext.create();
                            return JSON.stringify(@{fn_serialize}(value, ctx));
                        }

                        export function @{fn_to_obj}(value: @{interface_name}): Record<string, unknown> {
                            const ctx = SerializeContext.create();
                            return @{fn_serialize}(value, ctx);
                        }

                        export function @{fn_serialize}(value: @{interface_name}, ctx: SerializeContext): Record<string, unknown> {
                            // Check if already serialized (cycle detection)
                            const existingId = ctx.getId(value);
                            if (existingId !== undefined) {
                                return { __ref: existingId };
                            }

                            // Register this object
                            const __id = ctx.register(value);

                            const result: Record<string, unknown> = {
                                __type: "@{interface_name}",
                                __id,
                            };

                            {#if has_regular}
                                {#for field in regular_fields}
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    result["@{field.json_key}"] = value.@{field.field_name};
                                                }
                                            {:else}
                                                result["@{field.json_key}"] = value.@{field.field_name};
                                            {/if}

                                        {:case TypeCategory::Date}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    result["@{field.json_key}"] = value.@{field.field_name}.toISOString();
                                                }
                                            {:else}
                                                result["@{field.json_key}"] = value.@{field.field_name}.toISOString();
                                            {/if}

                                        {:case TypeCategory::Array(_)}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = value.@{field.field_name};
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = value.@{field.field_name}.map((item: Date) => item.toISOString());
                                                        {:case SerdeValueKind::NullableDate}
                                                            result["@{field.json_key}"] = value.@{field.field_name}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                        {:case _}
                                                            result["@{field.json_key}"] = value.@{field.field_name}.map(
                                                                (item: any) => typeof item?.__serialize === "function"
                                                                    ? item.__serialize(ctx)
                                                                    : item
                                                            );
                                                    {/match}
                                                }
                                            {:else}
                                                {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        result["@{field.json_key}"] = value.@{field.field_name};
                                                    {:case SerdeValueKind::Date}
                                                        result["@{field.json_key}"] = value.@{field.field_name}.map((item: Date) => item.toISOString());
                                                    {:case SerdeValueKind::NullableDate}
                                                        result["@{field.json_key}"] = value.@{field.field_name}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                    {:case _}
                                                        result["@{field.json_key}"] = value.@{field.field_name}.map(
                                                            (item: any) => typeof item?.__serialize === "function"
                                                                ? item.__serialize(ctx)
                                                                : item
                                                        );
                                                {/match}
                                            {/if}

                                        {:case TypeCategory::Map(_, _)}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = Object.fromEntries(value.@{field.field_name}.entries());
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = Object.fromEntries(
                                                                Array.from(value.@{field.field_name}.entries()).map(
                                                                    ([k, v]) => [k, (v as Date).toISOString()]
                                                                )
                                                            );
                                                        {:case SerdeValueKind::NullableDate}
                                                            result["@{field.json_key}"] = Object.fromEntries(
                                                                Array.from(value.@{field.field_name}.entries()).map(
                                                                    ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                                )
                                                            );
                                                        {:case _}
                                                            result["@{field.json_key}"] = Object.fromEntries(
                                                                Array.from(value.@{field.field_name}.entries()).map(
                                                                    ([k, v]) => [k, typeof (v as any)?.__serialize === "function"
                                                                        ? (v as any).__serialize(ctx)
                                                                        : v]
                                                                )
                                                            );
                                                    {/match}
                                                }
                                            {:else}
                                                {#match field.map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        result["@{field.json_key}"] = Object.fromEntries(value.@{field.field_name}.entries());
                                                    {:case SerdeValueKind::Date}
                                                        result["@{field.json_key}"] = Object.fromEntries(
                                                            Array.from(value.@{field.field_name}.entries()).map(
                                                                ([k, v]) => [k, (v as Date).toISOString()]
                                                            )
                                                        );
                                                    {:case SerdeValueKind::NullableDate}
                                                        result["@{field.json_key}"] = Object.fromEntries(
                                                            Array.from(value.@{field.field_name}.entries()).map(
                                                                ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                            )
                                                        );
                                                    {:case _}
                                                        result["@{field.json_key}"] = Object.fromEntries(
                                                            Array.from(value.@{field.field_name}.entries()).map(
                                                                ([k, v]) => [k, typeof (v as any)?.__serialize === "function"
                                                                    ? (v as any).__serialize(ctx)
                                                                    : v]
                                                            )
                                                        );
                                                {/match}
                                            {/if}

                                        {:case TypeCategory::Set(_)}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            result["@{field.json_key}"] = Array.from(value.@{field.field_name});
                                                        {:case SerdeValueKind::Date}
                                                            result["@{field.json_key}"] = Array.from(value.@{field.field_name}).map((item: Date) => item.toISOString());
                                                        {:case SerdeValueKind::NullableDate}
                                                            result["@{field.json_key}"] = Array.from(value.@{field.field_name}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                        {:case _}
                                                            result["@{field.json_key}"] = Array.from(value.@{field.field_name}).map(
                                                                (item: any) => typeof item?.__serialize === "function"
                                                                    ? item.__serialize(ctx)
                                                                    : item
                                                            );
                                                    {/match}
                                                }
                                            {:else}
                                                {#match field.set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        result["@{field.json_key}"] = Array.from(value.@{field.field_name});
                                                    {:case SerdeValueKind::Date}
                                                        result["@{field.json_key}"] = Array.from(value.@{field.field_name}).map((item: Date) => item.toISOString());
                                                    {:case SerdeValueKind::NullableDate}
                                                        result["@{field.json_key}"] = Array.from(value.@{field.field_name}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                    {:case _}
                                                        result["@{field.json_key}"] = Array.from(value.@{field.field_name}).map(
                                                            (item: any) => typeof item?.__serialize === "function"
                                                                ? item.__serialize(ctx)
                                                                : item
                                                        );
                                                {/match}
                                            {/if}

                                        {:case TypeCategory::Optional(_)}
                                            if (value.@{field.field_name} !== undefined) {
                                                {#match field.optional_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        result["@{field.json_key}"] = value.@{field.field_name};
                                                    {:case SerdeValueKind::Date}
                                                        result["@{field.json_key}"] = (value.@{field.field_name} as Date).toISOString();
                                                    {:case _}
                                                        result["@{field.json_key}"] = typeof (value.@{field.field_name} as any)?.__serialize === "function"
                                                            ? (value.@{field.field_name} as any).__serialize(ctx)
                                                            : value.@{field.field_name};
                                                {/match}
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result["@{field.json_key}"] = value.@{field.field_name};
                                                {:case SerdeValueKind::Date}
                                                    result["@{field.json_key}"] = value.@{field.field_name} === null
                                                        ? null
                                                        : (value.@{field.field_name} as Date).toISOString();
                                                {:case _}
                                                    if (value.@{field.field_name} !== null) {
                                                        result["@{field.json_key}"] = typeof (value.@{field.field_name} as any)?.__serialize === "function"
                                                            ? (value.@{field.field_name} as any).__serialize(ctx)
                                                            : value.@{field.field_name};
                                                    } else {
                                                        result["@{field.json_key}"] = null;
                                                    }
                                            {/match}

                                        {:case TypeCategory::Serializable(_)}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    result["@{field.json_key}"] = typeof (value.@{field.field_name} as any)?.__serialize === "function"
                                                        ? (value.@{field.field_name} as any).__serialize(ctx)
                                                        : value.@{field.field_name};
                                                }
                                            {:else}
                                                result["@{field.json_key}"] = typeof (value.@{field.field_name} as any)?.__serialize === "function"
                                                    ? (value.@{field.field_name} as any).__serialize(ctx)
                                                    : value.@{field.field_name};
                                            {/if}

                                        {:case TypeCategory::Unknown}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    result["@{field.json_key}"] = value.@{field.field_name};
                                                }
                                            {:else}
                                                result["@{field.json_key}"] = value.@{field.field_name};
                                            {/if}
                                    {/match}
                                {/for}
                            {/if}

                            {#if has_flatten}
                                {#for field in flatten_fields}
                                    {#if field.optional}
                                        if (value.@{field.field_name} !== undefined) {
                                            const __flattened = typeof (value.@{field.field_name} as any)?.__serialize === "function"
                                                ? (value.@{field.field_name} as any).__serialize(ctx)
                                                : value.@{field.field_name};
                                            const { __type: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = typeof (value.@{field.field_name} as any)?.__serialize === "function"
                                                ? (value.@{field.field_name} as any).__serialize(ctx)
                                                : value.@{field.field_name};
                                            const { __type: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                                {/for}
                            {/if}

                            return result;
                        }
                    }
                }
            };
            result.add_import("SerializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();
            let naming_style = input.context.function_naming_style;

            // Build generic type signature if type has type params
            let type_params = type_alias.type_params();
            let (generic_decl, generic_args) = if type_params.is_empty() {
                (String::new(), String::new())
            } else {
                let params = type_params.join(", ");
                (format!("<{}>", params), format!("<{}>", params))
            };
            let full_type_name = format!("{}{}", type_name, generic_args);

            // Generate function names based on naming style
            let (fn_to_json, fn_to_obj, fn_serialize) = match naming_style {
                FunctionNamingStyle::Namespace => (
                    format!("toStringifiedJSON{}", generic_decl),
                    format!("toObject{}", generic_decl),
                    format!("__serialize{}", generic_decl),
                ),
                FunctionNamingStyle::Generic => (
                    format!("toStringifiedJSON{}", generic_decl),
                    format!("toObject{}", generic_decl),
                    format!("__serialize{}", generic_decl),
                ),
                FunctionNamingStyle::Prefix => (
                    format!(
                        "{}ToStringifiedJSON{}",
                        to_camel_case(type_name),
                        generic_decl
                    ),
                    format!("{}ToObject{}", to_camel_case(type_name), generic_decl),
                    format!("{}__serialize{}", to_camel_case(type_name), generic_decl),
                ),
                FunctionNamingStyle::Suffix => (
                    format!("toStringifiedJSON{}{}", type_name, generic_decl),
                    format!("toObject{}{}", type_name, generic_decl),
                    format!("__serialize{}{}", type_name, generic_decl),
                ),
            };

            if type_alias.is_object() {
                // Object type: serialize fields
                let container_opts =
                    SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);

                // Collect serializable fields with diagnostic collection
                let mut all_diagnostics = DiagnosticCollector::new();
                let fields: Vec<SerializeField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let parse_result =
                            SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                        all_diagnostics.extend(parse_result.diagnostics);
                        let opts = parse_result.options;

                        if !opts.should_serialize() {
                            return None;
                        }

                        let json_key = opts
                            .rename
                            .clone()
                            .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                        let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                        let optional_inner_kind = match &type_cat {
                            TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let nullable_inner_kind = match &type_cat {
                            TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let array_elem_kind = match &type_cat {
                            TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let set_elem_kind = match &type_cat {
                            TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let map_value_kind = match &type_cat {
                            TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                            _ => None,
                        };

                        // Extract serializable type names for direct function calls
                        let optional_serializable_type = match &type_cat {
                            TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let nullable_serializable_type = match &type_cat {
                            TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let array_elem_serializable_type = match &type_cat {
                            TypeCategory::Array(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let set_elem_serializable_type = match &type_cat {
                            TypeCategory::Set(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let map_value_serializable_type = match &type_cat {
                            TypeCategory::Map(_, value) => get_serializable_type_name(value),
                            _ => None,
                        };

                        Some(SerializeField {
                            json_key,
                            field_name: field.name.clone(),
                            type_cat,
                            optional: field.optional,
                            flatten: opts.flatten,
                            optional_inner_kind,
                            nullable_inner_kind,
                            array_elem_kind,
                            set_elem_kind,
                            map_value_kind,
                            optional_serializable_type,
                            nullable_serializable_type,
                            array_elem_serializable_type,
                            set_elem_serializable_type,
                            map_value_serializable_type,
                        })
                    })
                    .collect();

                // Check for errors in field parsing before continuing
                if all_diagnostics.has_errors() {
                    return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
                }

                let regular_fields: Vec<_> =
                    fields.iter().filter(|f| !f.flatten).cloned().collect();
                let has_regular = !regular_fields.is_empty();

                let mut result = match naming_style {
                    FunctionNamingStyle::Namespace => {
                        ts_template! {
                            export namespace @{type_name} {
                                export function {|toStringifiedJSON@{generic_decl}|}(value: @{full_type_name}): string {
                                    const ctx = SerializeContext.create();
                                    return JSON.stringify(__serialize(value, ctx));
                                }

                                export function {|toObject@{generic_decl}|}(value: @{full_type_name}): Record<string, unknown> {
                                    const ctx = SerializeContext.create();
                                    return __serialize(value, ctx);
                                }

                                export function {|__serialize@{generic_decl}|}(value: @{full_type_name}, ctx: SerializeContext): Record<string, unknown> {
                                    const existingId = ctx.getId(value);
                                    if (existingId !== undefined) {
                                        return { __ref: existingId };
                                    }

                                    const __id = ctx.register(value);
                                    const result: Record<string, unknown> = {
                                        __type: "@{type_name}",
                                        __id,
                                    };

                                    {#if has_regular}
                                        {#for field in regular_fields}
                                            {#if field.optional}
                                                if (value.@{field.field_name} !== undefined) {
                                                    result["@{field.json_key}"] = value.@{field.field_name};
                                                }
                                            {:else}
                                                result["@{field.json_key}"] = value.@{field.field_name};
                                            {/if}
                                        {/for}
                                    {/if}

                                    return result;
                                }
                            }
                        }
                    }
                    _ => {
                        ts_template! {
                            export function {|@{fn_to_json}|}(value: @{full_type_name}): string {
                                const ctx = SerializeContext.create();
                                return JSON.stringify({|@{fn_serialize}|}(value, ctx));
                            }

                            export function {|@{fn_to_obj}|}(value: @{full_type_name}): Record<string, unknown> {
                                const ctx = SerializeContext.create();
                                return {|@{fn_serialize}|}(value, ctx);
                            }

                            export function {|@{fn_serialize}|}(value: @{full_type_name}, ctx: SerializeContext): Record<string, unknown> {
                                const existingId = ctx.getId(value);
                                if (existingId !== undefined) {
                                    return { __ref: existingId };
                                }

                                const __id = ctx.register(value);
                                const result: Record<string, unknown> = {
                                    __type: "@{type_name}",
                                    __id,
                                };

                                {#if has_regular}
                                    {#for field in regular_fields}
                                        {#if field.optional}
                                            if (value.@{field.field_name} !== undefined) {
                                                result["@{field.json_key}"] = value.@{field.field_name};
                                            }
                                        {:else}
                                            result["@{field.json_key}"] = value.@{field.field_name};
                                        {/if}
                                    {/for}
                                {/if}

                                return result;
                            }
                        }
                    }
                };
                result.add_import("SerializeContext", "macroforge/serde");
                Ok(result)
            } else {
                // Union, tuple, or simple alias: delegate to inner type's __serialize if available
                let mut result = match naming_style {
                    FunctionNamingStyle::Namespace => {
                        ts_template! {
                            export namespace @{type_name} {
                                export function {|toStringifiedJSON@{generic_decl}|}(value: @{full_type_name}): string {
                                    const ctx = SerializeContext.create();
                                    return JSON.stringify(__serialize(value, ctx));
                                }

                                export function {|toObject@{generic_decl}|}(value: @{full_type_name}): unknown {
                                    const ctx = SerializeContext.create();
                                    return __serialize(value, ctx);
                                }

                                export function {|__serialize@{generic_decl}|}(value: @{full_type_name}, ctx: SerializeContext): unknown {
                                    if (typeof (value as any)?.__serialize === "function") {
                                        return (value as any).__serialize(ctx);
                                    }
                                    return value;
                                }
                            }
                        }
                    }
                    _ => {
                        ts_template! {
                            export function {|@{fn_to_json}|}(value: @{full_type_name}): string {
                                const ctx = SerializeContext.create();
                                return JSON.stringify({|@{fn_serialize}|}(value, ctx));
                            }

                            export function {|@{fn_to_obj}|}(value: @{full_type_name}): unknown {
                                const ctx = SerializeContext.create();
                                return {|@{fn_serialize}|}(value, ctx);
                            }

                            export function {|@{fn_serialize}|}(value: @{full_type_name}, ctx: SerializeContext): unknown {
                                if (typeof (value as any)?.__serialize === "function") {
                                    return (value as any).__serialize(ctx);
                                }
                                return value;
                            }
                        }
                    }
                };
                result.add_import("SerializeContext", "macroforge/serde");
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_field_struct() {
        let field = SerializeField {
            json_key: "name".into(),
            field_name: "name".into(),
            type_cat: TypeCategory::Primitive,
            optional: false,
            flatten: false,
            optional_inner_kind: None,
            nullable_inner_kind: None,
            array_elem_kind: None,
            set_elem_kind: None,
            map_value_kind: None,
            optional_serializable_type: None,
            nullable_serializable_type: None,
            array_elem_serializable_type: None,
            set_elem_serializable_type: None,
            map_value_serializable_type: None,
        };
        assert_eq!(field.json_key, "name");
        assert!(!field.optional);
    }
}
