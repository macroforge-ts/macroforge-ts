use crate::ast::{Expr, Ident};

use super::super::TypeCategory;
use super::super::value_kind::EndecValueKind;

use convert_case::{Case, Casing};

/// Generates the EncodeWithContext function name for a nested encodable type.
/// For example: "User" -> "userEncodeWithContext"
pub(crate) fn nested_encode_fn_name(type_name: &str) -> String {
    // Strip generic parameters (e.g., "RecordLink<Employee>" -> "RecordLink")
    // before converting to camelCase, since `<>` are not recognized as word
    // boundaries by convert_case and would leak into the function name.
    let base = if let Some(idx) = type_name.find('<') {
        &type_name[..idx]
    } else {
        type_name
    };
    format!("{}EncodeWithContext", base.to_case(Case::Camel))
}

/// Contains field information needed for JSON encoding code generation.
///
/// Each field that should be included in encoding is represented by this struct,
/// capturing the JSON key name, field access name, type category, and encoding options.
#[derive(Clone)]
pub(crate) struct EncodeField {
    /// The JSON key as an AST identifier for direct property access. It may
    /// differ from the field name if `@endec(rename = "...")` is used.
    /// Used in templates as `result.@{json_key_ident}` instead of computed access.
    pub(crate) json_key_ident: Ident,

    /// The field name as an AST identifier for property access.
    pub(crate) field_ident: Ident,

    /// The category of the field's type, used to select the appropriate
    /// encoding strategy (primitive, Date, Array, Map, Set, etc.).
    pub(crate) type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier).
    /// Optional fields are wrapped in `if (value !== undefined)` checks.
    pub(crate) optional: bool,

    /// Whether the field should be flattened into the parent object.
    /// Flattened fields have their properties merged directly into the parent
    /// rather than being nested under their field name.
    pub(crate) flatten: bool,

    /// For `T | undefined` unions: classification of `T`.
    pub(crate) optional_inner_kind: Option<EndecValueKind>,
    /// For `T | null` unions: classification of `T`.
    pub(crate) nullable_inner_kind: Option<EndecValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    pub(crate) array_elem_kind: Option<EndecValueKind>,
    /// For `Set<T>`: classification of `T`.
    pub(crate) set_elem_kind: Option<EndecValueKind>,
    /// For `Map<K, V>`: classification of `V`.
    pub(crate) map_value_kind: Option<EndecValueKind>,
    /// For `Record<K, V>`: classification of `V`.
    pub(crate) record_value_kind: Option<EndecValueKind>,
    /// For wrapper types like Partial<T>, Required<T>, etc.: classification of `T`.
    pub(crate) wrapper_inner_kind: Option<EndecValueKind>,

    // --- Encodable type tracking for direct function calls ---
    /// For `T | undefined` where T is Encodable: the type name.
    pub(crate) optional_encodable_type: Option<String>,
    /// For `T | null` where T is Encodable: the type name.
    pub(crate) nullable_encodable_type: Option<String>,
    /// For `Array<T>` where T is Encodable: the type name.
    pub(crate) array_elem_encodable_type: Option<String>,
    /// For `Set<T>` where T is Encodable: the type name.
    pub(crate) set_elem_encodable_type: Option<String>,
    /// For `Map<K, V>` where V is Encodable: the type name.
    pub(crate) map_value_encodable_type: Option<String>,
    /// For `Record<K, V>` where V is Encodable: the type name.
    pub(crate) record_value_encodable_type: Option<String>,
    /// For wrapper types like Partial<T> where T is Encodable: the type name.
    pub(crate) wrapper_encodable_type: Option<String>,

    /// Custom encoding function expression (from `@endec({encodeWith: "fn"})`)
    /// When set, this function is called instead of type-based encoding.
    pub(crate) encode_with: Option<Expr>,

    /// Whether this field uses decimal format (encode number as string).
    pub(crate) decimal_format: bool,

    /// Set when the field's resolved type is a two-member primitive-or-encodable
    /// union (the shape of `RecordLink<T> = string | T` after alias resolution).
    /// Holds the primitive keyword (e.g. `"string"`). When set, the
    /// `Encodable(name)` encode branch wraps its call in a
    /// `typeof === primitive` guard so raw id strings pass through unchanged.
    pub(crate) primitive_union_guard: Option<String>,

    /// The array-element analogue of `primitive_union_guard`: set when an
    /// `Array<T>` element resolves to a primitive-or-encodable union (e.g.
    /// `Array<RecordLink<T>>` = `Array<string | T>`). When set alongside
    /// `array_elem_encodable_type`, each element is encoded through a
    /// `typeof === primitive` guard so bare-id elements pass through unchanged
    /// while fetched objects are encoded.
    pub(crate) array_elem_primitive_union_guard: Option<String>,
}
