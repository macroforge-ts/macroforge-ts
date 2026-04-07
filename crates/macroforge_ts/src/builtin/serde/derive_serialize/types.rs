use crate::swc_ecma_ast::{Expr, Ident};

use super::super::TypeCategory;

use convert_case::{Case, Casing};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SerdeValueKind {
    PrimitiveLike,
    Date,
    NullableDate,
    Other,
}

pub(crate) fn is_ts_primitive_keyword(s: &str) -> bool {
    matches!(
        s.trim(),
        "string" | "number" | "boolean" | "bigint" | "null" | "undefined"
    )
}

pub(crate) fn is_ts_literal(s: &str) -> bool {
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

pub(crate) fn is_union_of_primitive_like(s: &str) -> bool {
    if !s.contains('|') {
        return false;
    }
    s.split('|').all(|part| {
        let part = part.trim();
        is_ts_primitive_keyword(part) || is_ts_literal(part)
    })
}

pub(crate) fn classify_serde_value_kind(ts_type: &str) -> SerdeValueKind {
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
pub(crate) fn get_serializable_type_name(ts_type: &str) -> Option<String> {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Serializable(name) => Some(name),
        _ => None,
    }
}

/// Generates the SerializeWithContext function name for a nested serializable type.
/// For example: "User" -> "userSerializeWithContext"
pub(crate) fn nested_serialize_fn_name(type_name: &str) -> String {
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

/// Contains field information needed for JSON serialization code generation.
///
/// Each field that should be included in serialization is represented by this struct,
/// capturing the JSON key name, field access name, type category, and serialization options.
#[derive(Clone)]
pub(crate) struct SerializeField {
    /// The JSON property name to use in the serialized output.
    /// This may differ from `field_name` if `@serde(rename = "...")` is used.
    #[allow(dead_code)]
    pub(crate) json_key: String,
    /// The JSON key as an AST identifier for direct property access.
    /// Used in templates as `result.@{json_key_ident}` instead of computed access.
    pub(crate) json_key_ident: Ident,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property access expressions like `this.fieldName`.
    #[allow(dead_code)]
    pub(crate) field_name: String,
    /// The field name as an AST identifier for property access.
    pub(crate) field_ident: Ident,

    /// The category of the field's type, used to select the appropriate
    /// serialization strategy (primitive, Date, Array, Map, Set, etc.).
    pub(crate) type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier).
    /// Optional fields are wrapped in `if (value !== undefined)` checks.
    pub(crate) optional: bool,

    /// Whether the field should be flattened into the parent object.
    /// Flattened fields have their properties merged directly into the parent
    /// rather than being nested under their field name.
    pub(crate) flatten: bool,

    /// For `T | undefined` unions: classification of `T`.
    pub(crate) optional_inner_kind: Option<SerdeValueKind>,
    /// For `T | null` unions: classification of `T`.
    pub(crate) nullable_inner_kind: Option<SerdeValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    pub(crate) array_elem_kind: Option<SerdeValueKind>,
    /// For `Set<T>`: classification of `T`.
    pub(crate) set_elem_kind: Option<SerdeValueKind>,
    /// For `Map<K, V>`: classification of `V`.
    pub(crate) map_value_kind: Option<SerdeValueKind>,
    /// For `Record<K, V>`: classification of `V`.
    pub(crate) record_value_kind: Option<SerdeValueKind>,
    /// For wrapper types like Partial<T>, Required<T>, etc.: classification of `T`.
    pub(crate) wrapper_inner_kind: Option<SerdeValueKind>,

    // --- Serializable type tracking for direct function calls ---
    /// For `T | undefined` where T is Serializable: the type name.
    pub(crate) optional_serializable_type: Option<String>,
    /// For `T | null` where T is Serializable: the type name.
    pub(crate) nullable_serializable_type: Option<String>,
    /// For `Array<T>` where T is Serializable: the type name.
    pub(crate) array_elem_serializable_type: Option<String>,
    /// For `Set<T>` where T is Serializable: the type name.
    pub(crate) set_elem_serializable_type: Option<String>,
    /// For `Map<K, V>` where V is Serializable: the type name.
    pub(crate) map_value_serializable_type: Option<String>,
    /// For `Record<K, V>` where V is Serializable: the type name.
    pub(crate) record_value_serializable_type: Option<String>,
    /// For wrapper types like Partial<T> where T is Serializable: the type name.
    pub(crate) wrapper_serializable_type: Option<String>,

    /// Custom serialization function expression (from `@serde({serializeWith: "fn"})`)
    /// When set, this function is called instead of type-based serialization.
    pub(crate) serialize_with: Option<Expr>,

    /// Whether this field uses decimal format (serialize number as string).
    pub(crate) decimal_format: bool,
}
