use crate::swc_ecma_ast::{Expr, Ident};

use super::super::{TypeCategory, ValidatorSpec};

/// Classifies how a type's value behaves during deserialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SerdeValueKind {
    PrimitiveLike,
    Date,
    NullableDate,
    Other,
}

/// Contains field information needed for JSON deserialization code generation.
///
/// Each field that should be deserialized is represented by this struct,
/// capturing all the information needed to generate parsing, validation,
/// and assignment code.
#[derive(Clone)]
pub(super) struct DeserializeField {
    /// The JSON property name to read from the input object.
    /// This may differ from `field_name` if `@serde({rename: "..."})` is used.
    pub json_key: String,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property assignments like `instance.fieldName = value`.
    pub field_name: String,
    /// The field name as an AST identifier for property access.
    pub field_ident: Ident,

    /// The TypeScript type annotation string (e.g., "string", "number[]").
    /// Used for type casting in generated code.
    #[allow(dead_code)]
    pub ts_type: String,

    /// The serialized (JSON-compatible) type for casting raw values from `obj[key]`.
    /// For most types this equals `ts_type`, but collection types differ:
    /// - `Map<K, V>` -> `Record<K, V>` (JSON objects, not Maps)
    /// - `Set<T>` -> `Array<T>` (JSON arrays, not Sets)
    pub raw_cast_type: String,

    /// The category of the field's type, used to select the appropriate
    /// deserialization strategy (primitive, Date, Array, Map, Set, etc.).
    pub type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier or `@serde(default)`).
    /// Optional fields don't require the JSON property to be present.
    pub optional: bool,

    /// Whether the field has a default value specified.
    #[allow(dead_code)]
    pub has_default: bool,

    /// The default value expression to use if the field is missing.
    /// Example: expression for `@serde(default = "guest")`.
    pub default_expr: Option<Expr>,

    /// Whether the field should be read from the parent object level.
    /// Flattened fields look for their properties directly in the parent JSON.
    pub flatten: bool,

    /// List of validators to apply after parsing the field value.
    /// Each validator generates a condition check and error message.
    pub validators: Vec<ValidatorSpec>,

    /// For `T | null` unions: classification of `T`.
    pub nullable_inner_kind: Option<SerdeValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    pub array_elem_kind: Option<SerdeValueKind>,

    // --- Serializable type tracking for direct function calls ---
    /// For `T | null` where T is Serializable: the type name.
    pub nullable_serializable_type: Option<String>,

    /// Custom deserialization function expression (from `@serde({deserializeWith: "fn"})`)
    /// When set, this function is called instead of type-based deserialization.
    pub deserialize_with: Option<Expr>,
    /// Whether this field uses decimal format (deserialize string to number).
    pub decimal_format: bool,

    // --- Collection element type tracking for recursive deserialization ---
    /// For `Array<T>` where T is Serializable: the type name for direct function calls.
    pub array_elem_serializable_type: Option<String>,
    /// For `Set<T>`: classification of T.
    pub set_elem_kind: Option<SerdeValueKind>,
    /// For `Set<T>` where T is Serializable: the type name.
    pub set_elem_serializable_type: Option<String>,
    /// For `Map<K, V>`: classification of V.
    pub map_value_kind: Option<SerdeValueKind>,
    /// For `Map<K, V>` where V is Serializable: the type name.
    pub map_value_serializable_type: Option<String>,
    /// For `Record<K, V>`: classification of V.
    #[allow(dead_code)]
    pub record_value_kind: Option<SerdeValueKind>,
    /// For `Record<K, V>` where V is Serializable: the type name.
    pub record_value_serializable_type: Option<String>,
    /// For wrapper types (`Partial<T>`, `Required<T>`, etc.): classification of T.
    #[allow(dead_code)]
    pub wrapper_inner_kind: Option<SerdeValueKind>,
    /// For wrapper types where T is Serializable: the type name.
    pub wrapper_serializable_type: Option<String>,
    /// For `T | undefined`: classification of T.
    #[allow(dead_code)]
    pub optional_inner_kind: Option<SerdeValueKind>,
    /// For `T | undefined` where T is Serializable: the type name.
    #[allow(dead_code)]
    pub optional_serializable_type: Option<String>,
}

impl DeserializeField {
    /// Returns true if this field has any validators that need to be applied.
    pub fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Returns the serialized (JSON-compatible) type string for a given TS type.
///
/// JSON.parse() produces plain objects, arrays, strings, numbers, booleans, and null.
/// This function maps TypeScript types to what they actually look like in JSON:
/// - `Map<K, V>` -> `Record<K, raw(V)>` (JSON objects, not Maps)
/// - `Set<T>` -> `Array<raw(T)>` (JSON arrays, not Sets)
/// - `Date` -> `string | Date` (ISO strings from JSON, or already-parsed Dates)
/// - `T | null` -> `raw(T) | null` (recursive)
/// - `T | undefined` -> `raw(T) | undefined` (recursive)
/// - `Array<T>` -> `Array<raw(T)>` (recursive for element types)
/// - Everything else -> the original type unchanged
pub(super) fn raw_cast_type(ts_type: &str, type_cat: &TypeCategory) -> String {
    match type_cat {
        TypeCategory::Map(k, v) => {
            let inner_cat = TypeCategory::from_ts_type(v);
            format!("Record<{k}, {}>", raw_cast_type(v, &inner_cat))
        }
        TypeCategory::Set(t) => {
            let inner_cat = TypeCategory::from_ts_type(t);
            format!("Array<{}>", raw_cast_type(t, &inner_cat))
        }
        TypeCategory::Date => "string | Date".to_string(),
        TypeCategory::Nullable(inner) => {
            let inner_cat = TypeCategory::from_ts_type(inner);
            format!("{} | null", raw_cast_type(inner, &inner_cat))
        }
        TypeCategory::Optional(inner) => {
            let inner_cat = TypeCategory::from_ts_type(inner);
            format!("{} | undefined", raw_cast_type(inner, &inner_cat))
        }
        TypeCategory::Array(elem) => {
            let inner_cat = TypeCategory::from_ts_type(elem);
            let raw_elem = raw_cast_type(elem, &inner_cat);
            if raw_elem == *elem {
                ts_type.to_string()
            } else {
                format!("Array<{raw_elem}>")
            }
        }
        _ => ts_type.to_string(),
    }
}

/// Holds information about a serializable type reference in a union.
///
/// For parameterized types like `RecordLink<Product>`, we need both:
/// - The full type string for `__type` comparison and type casting
/// - The base type name for runtime namespace access
///
/// For foreign types (e.g., `DateTime.Utc`), the inline deserialize expression
/// is pre-computed so the union template can use it directly instead of generating
/// a function call to a non-existent `{camelCase}DeserializeWithContext` function.
#[derive(Clone)]
pub(super) struct SerializableTypeRef {
    /// The full type reference string (e.g., "RecordLink<Product>")
    pub full_type: String,
    /// Whether this type is a foreign type (configured in macroforge.config.ts)
    pub is_foreign: bool,
    /// For foreign types: the inline deserialize expression, namespace-rewritten.
    /// e.g., `"(raw: unknown) => __mf_DateTime.unsafeMake(raw as string)"`
    pub foreign_deserialize_inline: Option<String>,
    /// For foreign types: the inline shape-check predicate, namespace-rewritten.
    /// e.g., `"(v: unknown) => typeof v === \"string\""`
    pub foreign_has_shape_inline: Option<String>,
}

/// Holds information about an inline object variant in a union type.
///
/// When a union contains inline object type literals like
/// `{ __type: "admin"; permissions: string[] } | { __type: "viewer"; canComment: boolean }`,
/// each object literal becomes an `ObjectVariant` with its tag value extracted and fields
/// converted to `DeserializeField` for inline field-by-field deserialization.
#[derive(Clone)]
#[allow(dead_code)]
pub(super) struct ObjectVariant {
    /// The tag value from the tag field's literal type (e.g., "admin").
    /// None if the object doesn't contain a tag field with a string literal type.
    pub tag_value: Option<String>,
    /// All deserialization fields excluding the tag field itself.
    pub fields: Vec<DeserializeField>,
    /// Required field JSON keys (non-optional, non-flatten) -- used for shape checking.
    pub required_field_keys: Vec<String>,
    /// All field JSON keys (non-flatten) -- used for shape checking.
    #[allow(dead_code)]
    pub all_field_keys: Vec<String>,
    /// Display string for error messages (e.g., `{__type:"admin"}`).
    pub display_name: String,
}
