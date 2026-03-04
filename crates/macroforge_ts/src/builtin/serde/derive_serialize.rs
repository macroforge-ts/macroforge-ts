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
//! | Class | `classNameSerialize(value)` + `static serialize(value)` | Standalone function + static wrapper method |
//! | Enum | `enumNameSerialize(value)`, `enumNameSerializeWithContext` | Standalone functions |
//! | Interface | `interfaceNameSerialize(value)`, etc. | Standalone functions |
//! | Type Alias | `typeNameSerialize(value)`, etc. | Standalone functions |
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
//! | Arrays | For primitive-like element types, pass through; for `Date`/`Date | null`, map to ISO strings; otherwise map and call `SerializeWithContext(ctx)` when available |
//! | `Map<K,V>` | For primitive-like values, `Object.fromEntries(map.entries())`; for `Date`/`Date | null`, convert to ISO strings; otherwise call `SerializeWithContext(ctx)` per value when available |
//! | `Set<T>` | Convert to array; element handling matches `Array<T>` |
//! | Nullable | Include `null` explicitly; for primitive-like and `Date` unions the generator avoids runtime `SerializeWithContext` checks |
//! | Objects | Call `SerializeWithContext(ctx)` if available (to support user-defined implementations) |
//!
//! Note: the generator specializes some code paths based on the declared TypeScript type to
//! avoid runtime feature detection on primitives and literal unions.
//!
//! ## Field-Level Options
//!
//! The `@serde` decorator supports:
//!
//! - `skip` / `skipSerializing` - Exclude field from serialization
//! - `rename = "jsonKey"` - Use different JSON property name
//! - `flatten` - Merge nested object's fields into parent
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Serialize) */
//! class User {
//!     id: number;
//!
//!     /** @serde({ rename: "userName" }) */
//!     name: string;
//!
//!     /** @serde({ skipSerializing: true }) */
//!     password: string;
//!
//!     /** @serde({ flatten: true }) */
//!     metadata: UserMetadata;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { SerializeContext } from 'macroforge/serde';
//!
//! class User {
//!     id: number;
//!
//!     name: string;
//!
//!     password: string;
//!
//!     metadata: UserMetadata;
//!     /** Serializes a value to a JSON string.
//! @param value - The value to serialize
//! @returns JSON string representation with cycle detection metadata  */
//!
//!     static serialize(value: User): string {
//!         return userSerialize(value);
//!     }
//!     /** @internal Serializes with an existing context for nested/cyclic object graphs.
//! @param value - The value to serialize
//! @param ctx - The serialization context  */
//!
//!     static serializeWithContext(value: User, ctx: @{SERIALIZE_CONTEXT}): Record<string, unknown> {
//!         return userSerializeWithContext(value, ctx);
//!     }
//! }
//!
//! /** Serializes a value to a JSON string.
//! @param value - The value to serialize
//! @returns JSON string representation with cycle detection metadata */ export function userSerialize(
//!     value: User
//! ): string {
//!     const ctx = @{SERIALIZE_CONTEXT}.create();
//!     return JSON.stringify(userSerializeWithContext(value, ctx));
//! } /** @internal Serializes with an existing context for nested/cyclic object graphs.
//! @param value - The value to serialize
//! @param ctx - The serialization context */
//! export function userSerializeWithContext(
//!     value: User,
//!     ctx: SerializeContext
//! ): Record<string, unknown> {
//!     const existingId = ctx.getId(value);
//!     if (existingId !== undefined) {
//!         return { __ref: existingId };
//!     }
//!     const __id = ctx.register(value);
//!     const result: Record<string, unknown> = { __type: 'User', __id };
//!     result['id'] = value.id;
//!     result['userName'] = value.name;
//!     {
//!         const __flattened = userMetadataSerializeWithContext(value.metadata, ctx);
//!         const { __type: _, __id: __, ...rest } = __flattened as any; // tag field name is configurable via `tag` option
//!         Object.assign(result, rest);
//!     }
//!     return result;
//! }
//! ```
//!
//! ## Required Import
//!
//! The generated code automatically imports `SerializeContext` from `macroforge/serde`.

use crate::macros::{ts_macro_derive, ts_template};
use crate::swc_ecma_ast::{Expr, Ident};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_expr,
    parse_ts_macro_input, ts_ident,
};

use convert_case::{Case, Casing};

use super::{
    SerdeContainerOptions, SerdeFieldOptions, TypeCategory, get_foreign_types,
    rewrite_expression_namespaces,
};
use crate::builtin::return_types::SERIALIZE_CONTEXT;

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

/// Generates the SerializeWithContext function name for a nested serializable type.
/// For example: "User" -> "userSerializeWithContext"
fn nested_serialize_fn_name(type_name: &str) -> String {
    // Strip generic parameters (e.g., "RecordLink<Employee>" -> "RecordLink")
    // before converting to camelCase, since `<>` are not recognized as word
    // boundaries by convert_case and would leak into the function name.
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}SerializeWithContext", base.to_case(Case::Camel))
}

/// Tries to generate a composite serialize expression for types where a foreign
/// type is nested inside array, set, or nullable wrappers (e.g., `DateTime.Utc[]`,
/// `DateTime.Utc | null`).
///
/// Similar to `try_composite_foreign_deserialize` in the deserialize module.
fn try_composite_foreign_serialize(ts_type: &str) -> Option<String> {
    let foreign_types = get_foreign_types();

    // Split by | and classify parts
    let parts: Vec<&str> = ts_type
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let has_null = parts.contains(&"null");
    let has_undefined = parts.contains(&"undefined");
    let non_null: Vec<&str> = parts
        .iter()
        .filter(|s| **s != "null" && **s != "undefined")
        .copied()
        .collect();

    // Only handle single non-null type for now
    if non_null.len() != 1 {
        return None;
    }

    let core = non_null[0];

    // Check for array/set types
    let (container, elem_type) = if let Some(inner) = core.strip_suffix("[]") {
        ("array", inner.trim())
    } else if let Some(rest) = core.strip_prefix("Array<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("array", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Set<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("set", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Map<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::find_top_level_comma(inner) {
                ("map", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Record<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::find_top_level_comma(inner) {
                ("record", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else {
        ("none", core)
    };

    // Try matching the element type against foreign types
    let ft_match = TypeCategory::match_foreign_type(elem_type, &foreign_types);
    let ser_expr = ft_match.config.and_then(|ft| ft.serialize_expr.clone())?;
    let rewritten = rewrite_expression_namespaces(&ser_expr);

    match (has_null, has_undefined, container) {
        (_, _, "array") if has_null || has_undefined => Some(format!(
            "(arr) => arr == null ? null : arr.map(item => ({rewritten})(item))"
        )),
        (_, _, "array") => Some(format!("(arr) => arr.map(item => ({rewritten})(item))")),
        (_, _, "set") if has_null || has_undefined => Some(format!(
            "(s) => s == null ? null : Array.from(s).map(item => ({rewritten})(item))"
        )),
        (_, _, "set") => Some(format!(
            "(s) => Array.from(s).map(item => ({rewritten})(item))"
        )),
        (_, _, "map") if has_null || has_undefined => Some(format!(
            "(m) => m == null ? null : Object.fromEntries(Array.from(m.entries()).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (_, _, "map") => Some(format!(
            "(m) => Object.fromEntries(Array.from(m.entries()).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (_, _, "record") if has_null || has_undefined => Some(format!(
            "(obj) => obj == null ? null : Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (_, _, "record") => Some(format!(
            "(obj) => Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, ({rewritten})(v)]))"
        )),
        (true, _, "none") => Some(format!("(raw) => raw === null ? null : ({rewritten})(raw)")),
        (_, true, "none") => Some(format!(
            "(raw) => raw === undefined ? undefined : ({rewritten})(raw)"
        )),
        _ => None, // Direct match should have been caught at field level
    }
}

/// Contains field information needed for JSON serialization code generation.
///
/// Each field that should be included in serialization is represented by this struct,
/// capturing the JSON key name, field access name, type category, and serialization options.
#[derive(Clone)]
struct SerializeField {
    /// The JSON property name to use in the serialized output.
    /// This may differ from `_field_name` if `@serde(rename = "...")` is used.
    _json_key: String,
    /// The JSON key as an AST identifier for direct property access.
    /// Used in templates as `result.@{_json_key_ident}` instead of computed access.
    _json_key_ident: Ident,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property access expressions like `this.fieldName`.
    _field_name: String,
    /// The field name as an AST identifier for property access.
    _field_ident: Ident,

    /// The category of the field's type, used to select the appropriate
    /// serialization strategy (primitive, Date, Array, Map, Set, etc.).
    _type_cat: TypeCategory,

    /// Whether the field is _optional (has `?` modifier).
    /// _optional fields are wrapped in `if (value !== undefined)` checks.
    _optional: bool,

    /// Whether the field should be flattened into the parent object.
    /// Flattened fields have their properties merged directly into the parent
    /// rather than being nested under their field name.
    flatten: bool,

    /// For `T | undefined` unions: classification of `T`.
    _optional_inner_kind: Option<SerdeValueKind>,
    /// For `T | null` unions: classification of `T`.
    _nullable_inner_kind: Option<SerdeValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    _array_elem_kind: Option<SerdeValueKind>,
    /// For `Set<T>`: classification of `T`.
    _set_elem_kind: Option<SerdeValueKind>,
    /// For `Map<K, V>`: classification of `V`.
    _map_value_kind: Option<SerdeValueKind>,
    /// For `Record<K, V>`: classification of `V`.
    _record_value_kind: Option<SerdeValueKind>,
    /// For wrapper types like Partial<T>, Required<T>, etc.: classification of `T`.
    _wrapper_inner_kind: Option<SerdeValueKind>,

    // --- Serializable type tracking for direct function calls ---
    /// For `T | undefined` where T is Serializable: the type name.
    _optional_serializable_type: Option<String>,
    /// For `T | null` where T is Serializable: the type name.
    _nullable_serializable_type: Option<String>,
    /// For `Array<T>` where T is Serializable: the type name.
    _array_elem_serializable_type: Option<String>,
    /// For `Set<T>` where T is Serializable: the type name.
    _set_elem_serializable_type: Option<String>,
    /// For `Map<K, V>` where V is Serializable: the type name.
    _map_value_serializable_type: Option<String>,
    /// For `Record<K, V>` where V is Serializable: the type name.
    _record_value_serializable_type: Option<String>,
    /// For wrapper types like Partial<T> where T is Serializable: the type name.
    _wrapper_serializable_type: Option<String>,

    /// Custom serialization function expression (from `@serde({serializeWith: "fn"})`)
    /// When set, this function is called instead of type-based serialization.
    _serialize_with: Option<Expr>,

    /// Whether this field uses decimal format (serialize number as string).
    _decimal_format: bool,
}

#[ts_macro_derive(
    Serialize,
    description = "Generates serialization methods with cycle detection (toStringifiedJSON, serializeWithContext)",
    attributes((serde, "Configure serialization for this field. Options: skip, rename, flatten"))
)]
pub fn derive_serialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let class_ident = ts_ident!(class_name);
            let serialize_context_ident = ts_ident!(SERIALIZE_CONTEXT);
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);
            let tag_field = container_opts.tag_field();

            // Generate function names (always prefix style)
            let fn_serialize_ident = ts_ident!("{}Serialize", class_name.to_case(Case::Camel));
            let fn_serialize_expr: Expr = fn_serialize_ident.clone().into();
            let fn_serialize_internal_ident =
                ts_ident!("{}SerializeWithContext", class_name.to_case(Case::Camel));

            // Create Expr version of SERIALIZE_CONTEXT for expression positions
            let serialize_context_expr: Expr = serialize_context_ident.clone().into();

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

                    let _json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let _type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    let _optional_inner_kind = match &_type_cat {
                        TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let _nullable_inner_kind = match &_type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let _array_elem_kind = match &_type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let _set_elem_kind = match &_type_cat {
                        TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let map_value_kind = match &_type_cat {
                        TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                        _ => None,
                    };
                    let record_value_kind = match &_type_cat {
                        TypeCategory::Record(_, value) => Some(classify_serde_value_kind(value)),
                        _ => None,
                    };
                    let wrapper_inner_kind = match &_type_cat {
                        TypeCategory::Wrapper(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let _optional_serializable_type = match &_type_cat {
                        TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let nullable_serializable_type = match &_type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let array_elem_serializable_type = match &_type_cat {
                        TypeCategory::Array(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let set_elem_serializable_type = match &_type_cat {
                        TypeCategory::Set(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let map_value_serializable_type = match &_type_cat {
                        TypeCategory::Map(_, value) => get_serializable_type_name(value),
                        _ => None,
                    };
                    let record_value_serializable_type = match &_type_cat {
                        TypeCategory::Record(_, value) => get_serializable_type_name(value),
                        _ => None,
                    };
                    let wrapper_serializable_type = match &_type_cat {
                        TypeCategory::Wrapper(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };

                    // Check for foreign type serializer if no explicit serialize_with
                    let serialize_with_src = if opts.serialize_with.is_some() {
                        opts.serialize_with.clone()
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
                            .and_then(|ft| ft.serialize_expr.clone())
                            .map(|expr| rewrite_expression_namespaces(&expr))
                            // If no direct match, try composite patterns (e.g., DateTime.Utc[])
                            .or_else(|| try_composite_foreign_serialize(&field.ts_type))
                    };

                    let serialize_with = serialize_with_src.as_ref().and_then(|expr_src| {
                        match parse_ts_expr(expr_src) {
                            Ok(expr) => Some(*expr),
                            Err(err) => {
                                all_diagnostics.error(
                                    field.span,
                                    format!(
                                        "@serde(serializeWith): invalid expression for '{}': {err:?}",
                                        field.name
                                    ),
                                );
                                None
                            }
                        }
                    });

                    Some(SerializeField {
                        _json_key_ident: ts_ident!(&_json_key),
                        _json_key,
                        _field_name: field.name.clone(),
                        _field_ident: ts_ident!(field.name.as_str()),
                        _type_cat,
                        _optional: field.optional,
                        flatten: opts.flatten,
                        _optional_inner_kind,
                        _nullable_inner_kind,
                        _array_elem_kind,
                        _set_elem_kind,
                        _map_value_kind: map_value_kind,
                        _record_value_kind: record_value_kind,
                        _wrapper_inner_kind: wrapper_inner_kind,
                        _optional_serializable_type,
                        _nullable_serializable_type: nullable_serializable_type,
                        _array_elem_serializable_type: array_elem_serializable_type,
                        _set_elem_serializable_type: set_elem_serializable_type,
                        _map_value_serializable_type: map_value_serializable_type,
                        _record_value_serializable_type: record_value_serializable_type,
                        _wrapper_serializable_type: wrapper_serializable_type,
                        _serialize_with: serialize_with,
                        _decimal_format: opts.format.as_deref() == Some("decimal"),
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

            // Generate standalone functions
            // Clone ident for use in standalone template and later for class body
            let fn_serialize_internal_ident_standalone = fn_serialize_internal_ident.clone();
            let fn_serialize_internal_expr_standalone: Expr =
                fn_serialize_internal_ident.clone().into();
            let mut standalone = ts_template! {
                /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                export function @{fn_serialize_ident}(value: @{class_ident}, keepMetadata?: boolean): string {
                    const ctx = @{serialize_context_expr}.create();
                    const __raw = @{fn_serialize_internal_expr_standalone}(value, ctx);
                    if (keepMetadata) return JSON.stringify(__raw);
                    return JSON.stringify(__raw, (key, val) => key === "@{tag_field}" || key === "__id" ? undefined : val);
                }

                /** @internal Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                export function @{fn_serialize_internal_ident_standalone}(value: @{class_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                    // Check if already serialized (cycle detection)
                    const existingId = ctx.getId(value);
                    if (existingId !== undefined) {
                        return { __ref: existingId };
                    }

                    // Register this object
                    const __id = ctx.register(value);

                    const result: Record<string, unknown> = {
                        "@{tag_field}": "@{class_name}",
                        __id,
                    };

                    {#if has_regular}
                        {#for field in regular_fields}
                            {#if let Some(fn_name) = &field._serialize_with}
                                // Custom serialization function (serializeWith) - wrapped as IIFE for arrow functions
                                {#if field._optional}
                                    if (value.@{field._field_ident} !== undefined) {
                                        result.@{field._json_key_ident} = (@{fn_name})(value.@{field._field_ident});
                                    }
                                {:else}
                                    result.@{field._json_key_ident} = (@{fn_name})(value.@{field._field_ident});
                                {/if}
                            {:else}
                            {#match &field._type_cat}
                                {:case TypeCategory::Primitive}
                                    {#if field._decimal_format}
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = String(value.@{field._field_ident});
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = String(value.@{field._field_ident});
                                        {/if}
                                    {:else}
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        {/if}
                                    {/if}

                                {:case TypeCategory::Date}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result.@{field._json_key_ident} = value.@{field._field_ident}.toISOString();
                                        }
                                    {:else}
                                        result.@{field._json_key_ident} = value.@{field._field_ident}.toISOString();
                                    {/if}

                                {:case TypeCategory::Array(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date) => item.toISOString());
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field._array_elem_serializable_type}
                                                        {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident}.map(
                                                            (item) => @{serialize_with_context_elem}(item, ctx)
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date) => item.toISOString());
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date | null) => item === null ? null : item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field._array_elem_serializable_type}
                                                    {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident}.map(
                                                        (item) => @{serialize_with_context_elem}(item, ctx)
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Map(_, _)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Array.from(value.@{field._field_ident}.entries()).map(
                                                            ([k, v]) => [k, (v as Date).toISOString()]
                                                        )
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Array.from(value.@{field._field_ident}.entries()).map(
                                                            ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                        )
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type) = &field._map_value_serializable_type}
                                                        {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                        result.@{field._json_key_ident} = Object.fromEntries(
                                                            Array.from(value.@{field._field_ident}.entries()).map(
                                                                ([k, v]) => [k, @{serialize_with_context_value}(v, ctx)]
                                                            )
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Array.from(value.@{field._field_ident}.entries()).map(
                                                        ([k, v]) => [k, (v as Date).toISOString()]
                                                    )
                                                );
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Array.from(value.@{field._field_ident}.entries()).map(
                                                        ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                    )
                                                );
                                            {:case _}
                                                {#if let Some(value_type) = &field._map_value_serializable_type}
                                                    {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Array.from(value.@{field._field_ident}.entries()).map(
                                                            ([k, v]) => [k, @{serialize_with_context_value}(v, ctx)]
                                                        )
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Set(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date) => item.toISOString());
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field._set_elem_serializable_type}
                                                        {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                        result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map(
                                                            (item) => @{serialize_with_context_elem}(item, ctx)
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date) => item.toISOString());
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date | null) => item === null ? null : item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field._set_elem_serializable_type}
                                                    {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map(
                                                        (item) => @{serialize_with_context_elem}(item, ctx)
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Optional(_)}
                                    if (value.@{field._field_ident} !== undefined) {
                                        {#match field._optional_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = (value.@{field._field_ident} as Date).toISOString();
                                            {:case _}
                                                {#if let Some(inner_type) = &field._optional_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    }

                                {:case TypeCategory::Nullable(_)}
                                    {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                        {:case SerdeValueKind::PrimitiveLike}
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        {:case SerdeValueKind::Date}
                                            result.@{field._json_key_ident} = value.@{field._field_ident} === null
                                                ? null
                                                : (value.@{field._field_ident} as Date).toISOString();
                                        {:case _}
                                            if (value.@{field._field_ident} !== null) {
                                                {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                            } else {
                                                result.@{field._json_key_ident} = null;
                                            }
                                    {/match}

                                {:case TypeCategory::Serializable(type_name)}
                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(type_name)).into()}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                        }
                                    {:else}
                                        result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                    {/if}

                                {:case TypeCategory::Record(_, _)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._record_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type) = &field._record_value_serializable_type}
                                                        {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                        result.@{field._json_key_ident} = Object.fromEntries(
                                                            Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, @{serialize_with_context_value}(v, ctx)])
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._record_value_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                                );
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                                );
                                            {:case _}
                                                {#if let Some(value_type) = &field._record_value_serializable_type}
                                                    {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, @{serialize_with_context_value}(v, ctx)])
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Wrapper(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._wrapper_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = (value.@{field._field_ident} as Date).toISOString();
                                                {:case _}
                                                    {#if let Some(inner_type) = &field._wrapper_serializable_type}
                                                        {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                        result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                    {:else}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._wrapper_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = (value.@{field._field_ident} as Date).toISOString();
                                            {:case _}
                                                {#if let Some(inner_type) = &field._wrapper_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Unknown}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        }
                                    {:else}
                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                    {/if}
                            {/match}
                            {/if}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field._type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(type_name)).into()}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                            // Remove tag field and __id from flattened object
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                            // Remove tag field and __id from flattened object
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                                {:case TypeCategory::Record(_, _)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._record_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    Object.assign(result, value.@{field._field_ident});
                                                {:case SerdeValueKind::Date}
                                                    Object.assign(result, Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                                    ));
                                                {:case SerdeValueKind::NullableDate}
                                                    Object.assign(result, Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                                    ));
                                                {:case _}
                                                    {#if let Some(value_type) = &field._record_value_serializable_type}
                                                        {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                        Object.assign(result, Object.fromEntries(
                                                            Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, @{serialize_with_context_value}(v, ctx)])
                                                        ));
                                                    {:else}
                                                        Object.assign(result, value.@{field._field_ident});
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._record_value_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                Object.assign(result, value.@{field._field_ident});
                                            {:case SerdeValueKind::Date}
                                                Object.assign(result, Object.fromEntries(
                                                    Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                                ));
                                            {:case SerdeValueKind::NullableDate}
                                                Object.assign(result, Object.fromEntries(
                                                    Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                                ));
                                            {:case _}
                                                {#if let Some(value_type) = &field._record_value_serializable_type}
                                                    {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                    Object.assign(result, Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, @{serialize_with_context_value}(v, ctx)])
                                                    ));
                                                {:else}
                                                    Object.assign(result, value.@{field._field_ident});
                                                {/if}
                                        {/match}
                                    {/if}
                                {:case _}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            const __flattened = value.@{field._field_ident};
                                            // Remove tag field and __id from flattened object
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = value.@{field._field_ident};
                                            // Remove tag field and __id from flattened object
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                            {/match}
                        {/for}
                    {/if}

                    return result;
                }
            };
            standalone.add_aliased_import("SerializeContext", "macroforge/serde");

            // Generate static wrapper methods that delegate to standalone functions
            let fn_serialize_internal_expr_class: Expr = fn_serialize_internal_ident.into();
            let class_body = ts_template!(Within {
                /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                static serialize(value: @{class_ident}, keepMetadata?: boolean): string {
                    return @{fn_serialize_expr}(value, keepMetadata);
                }

                /** @internal Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                static serializeWithContext(value: @{class_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                    return @{fn_serialize_internal_expr_class}(value, ctx);
                }
            });

            // Combine standalone functions with class body
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            Ok(standalone.merge(class_body))
        }
        Data::Enum(_) => {
            // Enums: return the underlying value directly
            let enum_name = input.name();
            let enum_ident = ts_ident!(enum_name);

            let fn_name_ident = ts_ident!("{}Serialize", enum_name.to_case(Case::Camel));
            let fn_name_internal_ident =
                ts_ident!("{}SerializeWithContext", enum_name.to_case(Case::Camel));
            Ok(ts_template! {
                /** Serializes this enum value to a JSON string. */
                export function @{fn_name_ident}(value: @{enum_ident}): string {
                    return JSON.stringify(value);
                }

                /** Serializes with an existing context for nested/cyclic object graphs. */
                export function @{fn_name_internal_ident}(value: @{enum_ident}, _ctx: @{ts_ident!(SERIALIZE_CONTEXT)}): string | number {
                    return value;
                }
            })
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let interface_ident = ts_ident!(interface_name);
            let serialize_context_ident = ts_ident!(SERIALIZE_CONTEXT);
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);
            let tag_field = container_opts.tag_field();

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

                    let _json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let _type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    let _optional_inner_kind = match &_type_cat {
                        TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let _nullable_inner_kind = match &_type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let _array_elem_kind = match &_type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let _set_elem_kind = match &_type_cat {
                        TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let map_value_kind = match &_type_cat {
                        TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                        _ => None,
                    };
                    let record_value_kind = match &_type_cat {
                        TypeCategory::Record(_, value) => Some(classify_serde_value_kind(value)),
                        _ => None,
                    };
                    let wrapper_inner_kind = match &_type_cat {
                        TypeCategory::Wrapper(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let _optional_serializable_type = match &_type_cat {
                        TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let nullable_serializable_type = match &_type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let array_elem_serializable_type = match &_type_cat {
                        TypeCategory::Array(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let set_elem_serializable_type = match &_type_cat {
                        TypeCategory::Set(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };
                    let map_value_serializable_type = match &_type_cat {
                        TypeCategory::Map(_, value) => get_serializable_type_name(value),
                        _ => None,
                    };
                    let record_value_serializable_type = match &_type_cat {
                        TypeCategory::Record(_, value) => get_serializable_type_name(value),
                        _ => None,
                    };
                    let wrapper_serializable_type = match &_type_cat {
                        TypeCategory::Wrapper(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };

                    // Check for foreign type serializer if no explicit serialize_with
                    let serialize_with_src = if opts.serialize_with.is_some() {
                        opts.serialize_with.clone()
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
                            .and_then(|ft| ft.serialize_expr.clone())
                            .map(|expr| rewrite_expression_namespaces(&expr))
                            // If no direct match, try composite patterns (e.g., DateTime.Utc[])
                            .or_else(|| try_composite_foreign_serialize(&field.ts_type))
                    };

                    let serialize_with = serialize_with_src.as_ref().and_then(|expr_src| {
                        match parse_ts_expr(expr_src) {
                            Ok(expr) => Some(*expr),
                            Err(err) => {
                                all_diagnostics.error(
                                    field.span,
                                    format!(
                                        "@serde(serializeWith): invalid expression for '{}': {err:?}",
                                        field.name
                                    ),
                                );
                                None
                            }
                        }
                    });

                    Some(SerializeField {
                        _json_key_ident: ts_ident!(&_json_key),
                        _json_key,
                        _field_name: field.name.clone(),
                        _field_ident: ts_ident!(field.name.as_str()),
                        _type_cat,
                        _optional: field.optional,
                        flatten: opts.flatten,
                        _optional_inner_kind,
                        _nullable_inner_kind,
                        _array_elem_kind,
                        _set_elem_kind,
                        _map_value_kind: map_value_kind,
                        _record_value_kind: record_value_kind,
                        _wrapper_inner_kind: wrapper_inner_kind,
                        _optional_serializable_type,
                        _nullable_serializable_type: nullable_serializable_type,
                        _array_elem_serializable_type: array_elem_serializable_type,
                        _set_elem_serializable_type: set_elem_serializable_type,
                        _map_value_serializable_type: map_value_serializable_type,
                        _record_value_serializable_type: record_value_serializable_type,
                        _wrapper_serializable_type: wrapper_serializable_type,
                        _serialize_with: serialize_with,
                        _decimal_format: opts.format.as_deref() == Some("decimal"),
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
            let fn_serialize_ident = ts_ident!("{}Serialize", interface_name.to_case(Case::Camel));
            let fn_serialize_internal_ident = ts_ident!(
                "{}SerializeWithContext",
                interface_name.to_case(Case::Camel)
            );

            // Create Expr version of SERIALIZE_CONTEXT for expression positions
            let serialize_context_expr: Expr = serialize_context_ident.clone().into();
            let fn_serialize_internal_expr: Expr = fn_serialize_internal_ident.clone().into();

            let mut result = ts_template! {
                /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                export function @{fn_serialize_ident}(value: @{interface_ident}, keepMetadata?: boolean): string {
                    const ctx = @{serialize_context_expr}.create();
                    const __raw = @{fn_serialize_internal_expr}(value, ctx);
                    if (keepMetadata) return JSON.stringify(__raw);
                    return JSON.stringify(__raw, (key, val) => key === "@{tag_field}" || key === "__id" ? undefined : val);
                }

                /** Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                export function @{fn_serialize_internal_ident}(value: @{interface_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                    // Check if already serialized (cycle detection)
                    const existingId = ctx.getId(value);
                    if (existingId !== undefined) {
                        return { __ref: existingId };
                    }

                    // Register this object
                    const __id = ctx.register(value);

                    const result: Record<string, unknown> = {
                        "@{tag_field}": "@{interface_name}",
                        __id,
                    };

                    {#if has_regular}
                        {#for field in regular_fields}
                            {#if let Some(fn_name) = &field._serialize_with}
                                // Custom serialization function (serializeWith) - wrapped as IIFE for arrow functions
                                {#if field._optional}
                                    if (value.@{field._field_ident} !== undefined) {
                                        result.@{field._json_key_ident} = (@{fn_name})(value.@{field._field_ident});
                                    }
                                {:else}
                                    result.@{field._json_key_ident} = (@{fn_name})(value.@{field._field_ident});
                                {/if}
                            {:else}
                            {#match &field._type_cat}
                                {:case TypeCategory::Primitive}
                                    {#if field._decimal_format}
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = String(value.@{field._field_ident});
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = String(value.@{field._field_ident});
                                        {/if}
                                    {:else}
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        {/if}
                                    {/if}

                                {:case TypeCategory::Date}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result.@{field._json_key_ident} = value.@{field._field_ident}.toISOString();
                                        }
                                    {:else}
                                        result.@{field._json_key_ident} = value.@{field._field_ident}.toISOString();
                                    {/if}

                                {:case TypeCategory::Array(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date) => item.toISOString());
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date | null) => item === null ? null : item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field._array_elem_serializable_type}
                                                        {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident}.map(
                                                            (item) => @{serialize_with_context_elem}(item, ctx)
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date) => item.toISOString());
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = value.@{field._field_ident}.map((item: Date | null) => item === null ? null : item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field._array_elem_serializable_type}
                                                    {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident}.map(
                                                        (item) => @{serialize_with_context_elem}(item, ctx)
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Map(_, _)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Array.from(value.@{field._field_ident}.entries()).map(
                                                            ([k, v]) => [k, (v as Date).toISOString()]
                                                        )
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Array.from(value.@{field._field_ident}.entries()).map(
                                                            ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                        )
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type) = &field._map_value_serializable_type}
                                                        {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                        result.@{field._json_key_ident} = Object.fromEntries(
                                                            Array.from(value.@{field._field_ident}.entries()).map(
                                                                ([k, v]) => [k, @{serialize_with_context_value}(v, ctx)]
                                                            )
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._map_value_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Array.from(value.@{field._field_ident}.entries()).map(
                                                        ([k, v]) => [k, (v as Date).toISOString()]
                                                    )
                                                );
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Array.from(value.@{field._field_ident}.entries()).map(
                                                        ([k, v]) => [k, v === null ? null : (v as Date).toISOString()]
                                                    )
                                                );
                                            {:case _}
                                                {#if let Some(value_type) = &field._map_value_serializable_type}
                                                    {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Array.from(value.@{field._field_ident}.entries()).map(
                                                            ([k, v]) => [k, @{serialize_with_context_value}(v, ctx)]
                                                        )
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = Object.fromEntries(value.@{field._field_ident}.entries());
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Set(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date) => item.toISOString());
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date | null) => item === null ? null : item.toISOString());
                                                {:case _}
                                                    {#if let Some(elem_type) = &field._set_elem_serializable_type}
                                                        {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                        result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map(
                                                            (item) => @{serialize_with_context_elem}(item, ctx)
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._set_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date) => item.toISOString());
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map((item: Date | null) => item === null ? null : item.toISOString());
                                            {:case _}
                                                {#if let Some(elem_type) = &field._set_elem_serializable_type}
                                                    {$let serialize_with_context_elem: Expr = ts_ident!(nested_serialize_fn_name(elem_type)).into()}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident}).map(
                                                        (item) => @{serialize_with_context_elem}(item, ctx)
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = Array.from(value.@{field._field_ident});
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Optional(_)}
                                    if (value.@{field._field_ident} !== undefined) {
                                        {#match field._optional_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = (value.@{field._field_ident} as Date).toISOString();
                                            {:case _}
                                                {#if let Some(inner_type) = &field._optional_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    }

                                {:case TypeCategory::Nullable(_)}
                                    {#match field._nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                        {:case SerdeValueKind::PrimitiveLike}
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        {:case SerdeValueKind::Date}
                                            result.@{field._json_key_ident} = value.@{field._field_ident} === null
                                                ? null
                                                : (value.@{field._field_ident} as Date).toISOString();
                                        {:case _}
                                            if (value.@{field._field_ident} !== null) {
                                                {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                            } else {
                                                result.@{field._json_key_ident} = null;
                                            }
                                    {/match}

                                {:case TypeCategory::Serializable(type_name)}
                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(type_name)).into()}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                        }
                                    {:else}
                                        result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                    {/if}

                                {:case TypeCategory::Record(_, _)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._record_value_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                                    );
                                                {:case SerdeValueKind::NullableDate}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                                    );
                                                {:case _}
                                                    {#if let Some(value_type) = &field._record_value_serializable_type}
                                                        {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                        result.@{field._json_key_ident} = Object.fromEntries(
                                                            Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, @{serialize_with_context_value}(v, ctx)])
                                                        );
                                                    {:else}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._record_value_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, (v as Date).toISOString()])
                                                );
                                            {:case SerdeValueKind::NullableDate}
                                                result.@{field._json_key_ident} = Object.fromEntries(
                                                    Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, v === null ? null : (v as Date).toISOString()])
                                                );
                                            {:case _}
                                                {#if let Some(value_type) = &field._record_value_serializable_type}
                                                    {$let serialize_with_context_value: Expr = ts_ident!(nested_serialize_fn_name(value_type)).into()}
                                                    result.@{field._json_key_ident} = Object.fromEntries(
                                                        Object.entries(value.@{field._field_ident}).map(([k, v]) => [k, @{serialize_with_context_value}(v, ctx)])
                                                    );
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Wrapper(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#match field._wrapper_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {:case SerdeValueKind::Date}
                                                    result.@{field._json_key_ident} = (value.@{field._field_ident} as Date).toISOString();
                                                {:case _}
                                                    {#if let Some(inner_type) = &field._wrapper_serializable_type}
                                                        {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                        result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                    {:else}
                                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                                    {/if}
                                            {/match}
                                        }
                                    {:else}
                                        {#match field._wrapper_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                            {:case SerdeValueKind::PrimitiveLike}
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            {:case SerdeValueKind::Date}
                                                result.@{field._json_key_ident} = (value.@{field._field_ident} as Date).toISOString();
                                            {:case _}
                                                {#if let Some(inner_type) = &field._wrapper_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    result.@{field._json_key_ident} = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                {:else}
                                                    result.@{field._json_key_ident} = value.@{field._field_ident};
                                                {/if}
                                        {/match}
                                    {/if}

                                {:case TypeCategory::Unknown}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        }
                                    {:else}
                                        result.@{field._json_key_ident} = value.@{field._field_ident};
                                    {/if}
                            {/match}
                            {/if}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field._type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(type_name)).into()}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                                {:case TypeCategory::Optional(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            {#if let Some(inner_type) = &field._optional_serializable_type}
                                                {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                Object.assign(result, rest);
                                            {:else}
                                                const __flattened = value.@{field._field_ident};
                                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                Object.assign(result, rest);
                                            {/if}
                                        }
                                    {:else}
                                        {
                                            {#if let Some(inner_type) = &field._optional_serializable_type}
                                                {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                Object.assign(result, rest);
                                            {:else}
                                                const __flattened = value.@{field._field_ident};
                                                const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                Object.assign(result, rest);
                                            {/if}
                                        }
                                    {/if}
                                {:case TypeCategory::Nullable(_)}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            if (value.@{field._field_ident} !== null) {
                                                {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                    const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                    Object.assign(result, rest);
                                                {:else}
                                                    const __flattened = value.@{field._field_ident};
                                                    const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                    Object.assign(result, rest);
                                                {/if}
                                            }
                                        }
                                    {:else}
                                        {
                                            if (value.@{field._field_ident} !== null) {
                                                {#if let Some(inner_type) = &field._nullable_serializable_type}
                                                    {$let serialize_with_context_fn: Expr = ts_ident!(nested_serialize_fn_name(inner_type)).into()}
                                                    const __flattened = @{serialize_with_context_fn}(value.@{field._field_ident}, ctx);
                                                    const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                    Object.assign(result, rest);
                                                {:else}
                                                    const __flattened = value.@{field._field_ident};
                                                    const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                                    Object.assign(result, rest);
                                                {/if}
                                            }
                                        }
                                    {/if}
                                {:case _}
                                    {#if field._optional}
                                        if (value.@{field._field_ident} !== undefined) {
                                            const __flattened = value.@{field._field_ident};
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {:else}
                                        {
                                            const __flattened = value.@{field._field_ident};
                                            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as any;
                                            Object.assign(result, rest);
                                        }
                                    {/if}
                            {/match}
                        {/for}
                    {/if}

                    return result;
                }
            };
            result.add_aliased_import("SerializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            // Build generic type signature if type has type params
            let type_params = type_alias.type_params();
            let has_generics = !type_params.is_empty();
            let type_params_ident: Option<Ident> = if has_generics {
                Some(ts_ident!("{}", type_params.join(", ")))
            } else {
                None
            };
            let full_type_ident = if has_generics {
                ts_ident!("{}<{}>", type_name, type_params.join(", "))
            } else {
                ts_ident!(type_name)
            };

            // Generate function names based on naming style
            let fn_serialize_ident = ts_ident!("{}Serialize", type_name.to_case(Case::Camel));
            let fn_serialize_internal_ident =
                ts_ident!("{}SerializeWithContext", type_name.to_case(Case::Camel));

            // Create Expr version of SERIALIZE_CONTEXT for expression positions
            let serialize_context_ident = ts_ident!(SERIALIZE_CONTEXT);
            let serialize_context_expr: Expr = serialize_context_ident.clone().into();

            if type_alias.is_object() {
                // Object type: serialize fields
                let container_opts =
                    SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);
                let tag_field = container_opts.tag_field();

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

                        let _json_key = opts
                            .rename
                            .clone()
                            .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                        let _type_cat = TypeCategory::from_ts_type(&field.ts_type);

                        let _optional_inner_kind = match &_type_cat {
                            TypeCategory::Optional(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let _nullable_inner_kind = match &_type_cat {
                            TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let _array_elem_kind = match &_type_cat {
                            TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let _set_elem_kind = match &_type_cat {
                            TypeCategory::Set(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let map_value_kind = match &_type_cat {
                            TypeCategory::Map(_, value) => Some(classify_serde_value_kind(value)),
                            _ => None,
                        };
                        let record_value_kind = match &_type_cat {
                            TypeCategory::Record(_, value) => {
                                Some(classify_serde_value_kind(value))
                            }
                            _ => None,
                        };
                        let wrapper_inner_kind = match &_type_cat {
                            TypeCategory::Wrapper(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };

                        // Extract serializable type names for direct function calls
                        let _optional_serializable_type = match &_type_cat {
                            TypeCategory::Optional(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let nullable_serializable_type = match &_type_cat {
                            TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let array_elem_serializable_type = match &_type_cat {
                            TypeCategory::Array(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let set_elem_serializable_type = match &_type_cat {
                            TypeCategory::Set(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };
                        let map_value_serializable_type = match &_type_cat {
                            TypeCategory::Map(_, value) => get_serializable_type_name(value),
                            _ => None,
                        };
                        let record_value_serializable_type = match &_type_cat {
                            TypeCategory::Record(_, value) => get_serializable_type_name(value),
                            _ => None,
                        };
                        let wrapper_serializable_type = match &_type_cat {
                            TypeCategory::Wrapper(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };

                        let serialize_with = opts.serialize_with.as_ref().and_then(|expr_src| {
                            match parse_ts_expr(expr_src) {
                                Ok(expr) => Some(*expr),
                                Err(err) => {
                                    all_diagnostics.error(
                                        field.span,
                                        format!(
                                            "@serde(serializeWith): invalid expression for '{}': {err:?}",
                                            field.name
                                        ),
                                    );
                                    None
                                }
                            }
                        });

                        Some(SerializeField {
                            _json_key_ident: ts_ident!(&_json_key),
                            _json_key,
                            _field_name: field.name.clone(),
                            _field_ident: ts_ident!(field.name.as_str()),
                            _type_cat,
                            _optional: field.optional,
                            flatten: opts.flatten,
                            _optional_inner_kind,
                            _nullable_inner_kind,
                            _array_elem_kind,
                            _set_elem_kind,
                            _map_value_kind: map_value_kind,
                            _record_value_kind: record_value_kind,
                            _wrapper_inner_kind: wrapper_inner_kind,
                            _optional_serializable_type,
                            _nullable_serializable_type: nullable_serializable_type,
                            _array_elem_serializable_type: array_elem_serializable_type,
                            _set_elem_serializable_type: set_elem_serializable_type,
                            _map_value_serializable_type: map_value_serializable_type,
                            _record_value_serializable_type: record_value_serializable_type,
                            _wrapper_serializable_type: wrapper_serializable_type,
                            _serialize_with: serialize_with,
                            _decimal_format: opts.format.as_deref() == Some("decimal"),
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

                let fn_serialize_internal_expr: Expr = fn_serialize_internal_ident.clone().into();
                let mut result = if let Some(params) = &type_params_ident {
                    ts_template! {
                        /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                        export function @{fn_serialize_ident}<@{params}>(value: @{full_type_ident}, keepMetadata?: boolean): string {
                            const ctx = @{serialize_context_expr}.create();
                            const __raw = @{fn_serialize_internal_expr}(value, ctx);
                            if (keepMetadata) return JSON.stringify(__raw);
                            return JSON.stringify(__raw, (key, val) => key === "@{tag_field}" || key === "__id" ? undefined : val);
                        }

                        /** Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                        export function @{fn_serialize_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                            const existingId = ctx.getId(value);
                            if (existingId !== undefined) {
                                return { __ref: existingId };
                            }

                            const __id = ctx.register(value);
                            const result: Record<string, unknown> = {
                                "@{tag_field}": "@{type_name}",
                                __id,
                            };

                            {#if has_regular}
                                {#for field in regular_fields}
                                    {#if let Some(fn_name) = &field._serialize_with}
                                        // Custom serialization function (serializeWith)
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = @{fn_name}(value.@{field._field_ident});
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = @{fn_name}(value.@{field._field_ident});
                                        {/if}
                                    {:else}
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        {/if}
                                    {/if}
                                {/for}
                            {/if}

                            return result;
                        }
                    }
                } else {
                    ts_template! {
                        /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                        export function @{fn_serialize_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
                            const ctx = @{serialize_context_expr}.create();
                            const __raw = @{fn_serialize_internal_expr}(value, ctx);
                            if (keepMetadata) return JSON.stringify(__raw);
                            return JSON.stringify(__raw, (key, val) => key === "@{tag_field}" || key === "__id" ? undefined : val);
                        }

                        /** Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                        export function @{fn_serialize_internal_ident}(value: @{full_type_ident}, ctx: @{serialize_context_ident}): Record<string, unknown> {
                            const existingId = ctx.getId(value);
                            if (existingId !== undefined) {
                                return { __ref: existingId };
                            }

                            const __id = ctx.register(value);
                            const result: Record<string, unknown> = {
                                "@{tag_field}": "@{type_name}",
                                __id,
                            };

                            {#if has_regular}
                                {#for field in regular_fields}
                                    {#if let Some(fn_name) = &field._serialize_with}
                                        // Custom serialization function (serializeWith)
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = @{fn_name}(value.@{field._field_ident});
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = @{fn_name}(value.@{field._field_ident});
                                        {/if}
                                    {:else}
                                        {#if field._optional}
                                            if (value.@{field._field_ident} !== undefined) {
                                                result.@{field._json_key_ident} = value.@{field._field_ident};
                                            }
                                        {:else}
                                            result.@{field._json_key_ident} = value.@{field._field_ident};
                                        {/if}
                                    {/if}
                                {/for}
                            {/if}

                            return result;
                        }
                    }
                };
                result.add_aliased_import("SerializeContext", "macroforge/serde");
                Ok(result)
            } else {
                // Union, tuple, or simple alias: delegate to inner type's serializeWithContext if available
                let fn_serialize_internal_expr: Expr = fn_serialize_internal_ident.clone().into();
                let mut result = if let Some(params) = &type_params_ident {
                    ts_template! {
                        /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                        export function @{fn_serialize_ident}<@{params}>(value: @{full_type_ident}, keepMetadata?: boolean): string {
                            const ctx = @{serialize_context_expr}.create();
                            const __raw = @{fn_serialize_internal_expr}(value, ctx);
                            if (keepMetadata) return JSON.stringify(__raw);
                            return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
                        }

                        /** Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                        export function @{fn_serialize_internal_ident}<@{params}>(value: @{full_type_ident}, ctx: @{serialize_context_ident}): unknown {
                            if (typeof (value as any)?.serializeWithContext === "function") {
                                return (value as any).serializeWithContext(ctx);
                            }
                            return value;
                        }
                    }
                } else {
                    ts_template! {
                        /** Serializes a value to a JSON string. @param value - The value to serialize @param keepMetadata - If true, preserves __type and __id fields in the output @returns JSON string representation */
                        export function @{fn_serialize_ident}(value: @{full_type_ident}, keepMetadata?: boolean): string {
                            const ctx = @{serialize_context_expr}.create();
                            const __raw = @{fn_serialize_internal_expr}(value, ctx);
                            if (keepMetadata) return JSON.stringify(__raw);
                            return JSON.stringify(__raw, (key, val) => key === "__type" || key === "__id" ? undefined : val);
                        }

                        /** Serializes with an existing context for nested/cyclic object graphs. @param value - The value to serialize @param ctx - The serialization context */
                        export function @{fn_serialize_internal_ident}(value: @{full_type_ident}, ctx: @{serialize_context_ident}): unknown {
                            if (typeof (value as any)?.serializeWithContext === "function") {
                                return (value as any).serializeWithContext(ctx);
                            }
                            return value;
                        }
                    }
                };
                result.add_aliased_import("SerializeContext", "macroforge/serde");
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
            _json_key: "name".into(),
            _json_key_ident: ts_ident!("name"),
            _field_name: "name".into(),
            _field_ident: ts_ident!("name"),
            _type_cat: TypeCategory::Primitive,
            _optional: false,
            flatten: false,
            _optional_inner_kind: None,
            _nullable_inner_kind: None,
            _array_elem_kind: None,
            _set_elem_kind: None,
            _map_value_kind: None,
            _record_value_kind: None,
            _wrapper_inner_kind: None,
            _optional_serializable_type: None,
            _nullable_serializable_type: None,
            _array_elem_serializable_type: None,
            _set_elem_serializable_type: None,
            _map_value_serializable_type: None,
            _record_value_serializable_type: None,
            _wrapper_serializable_type: None,
            _serialize_with: None,
            _decimal_format: false,
        };
        assert_eq!(field._json_key, "name");
        assert!(!field._optional);
    }
}
