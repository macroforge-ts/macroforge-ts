use crate::ast::{Expr, Ident};

use super::super::union_guard::UnionGuard;
use super::super::value_kind::EndecValueKind;
use super::super::{TypeCategory, ValidatorSpec};
use super::validation::Missing;
use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;

/// Contains field information needed for JSON decoding code generation.
///
/// Each field that should be decoded is represented by this struct,
/// capturing all the information needed to generate parsing, validation,
/// and assignment code.
#[derive(Clone)]
pub(super) struct DecodeField {
    /// The JSON property name to read from the input object.
    /// This may differ from `field_name` if `@endec({rename: "..."})` is used.
    pub json_key: String,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property assignments like `instance.fieldName = value`.
    pub field_name: String,
    /// The field name as an AST identifier for property access.
    pub field_ident: Ident,

    /// The TypeScript type annotation string (e.g., "string", "number[]").
    /// Used for type casting in generated code.
    pub ts_type: String,

    /// The encoded (JSON-compatible) type for casting raw values from `obj[key]`.
    /// For most types this equals `ts_type`, but collection types differ:
    /// - `Map<K, V>` -> `Record<K, V>` (JSON objects, not Maps)
    /// - `Set<T>` -> `Array<T>` (JSON arrays, not Sets)
    pub raw_cast_type: String,

    /// The category of the field's type, used to select the appropriate
    /// decoding strategy (primitive, Date, Array, Map, Set, etc.).
    pub type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier or `@endec(default)`).
    /// Optional fields don't require the JSON property to be present.
    pub optional: bool,

    /// The default value expression to use if the field is missing.
    /// Example: expression for `@endec(default = "guest")`.
    pub default_expr: Option<Expr>,

    /// Whether the field should be read from the parent object level.
    /// Flattened fields look for their properties directly in the parent JSON.
    pub flatten: bool,

    /// List of validators to apply after parsing the field value.
    /// Each validator generates a condition check and error message.
    pub validators: Vec<ValidatorSpec>,

    /// For `T | null` unions: classification of `T`.
    pub nullable_inner_kind: Option<EndecValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    pub array_elem_kind: Option<EndecValueKind>,

    // --- Encodable type tracking for direct function calls ---
    /// For `T | null` where T is Encodable: the type name.
    pub nullable_encodable_type: Option<String>,

    /// Custom decoding function expression (from `@endec({decodeWith: "fn"})`)
    /// When set, this function is called instead of type-based decoding.
    pub decode_with: Option<Expr>,
    /// Whether this field uses decimal format (decode string to number).
    pub decimal_format: bool,

    // --- Collection element type tracking for recursive decoding ---
    /// For `Array<T>` where T is Encodable: the type name for direct function calls.
    pub array_elem_encodable_type: Option<String>,
    /// For `Set<T>`: classification of T.
    pub set_elem_kind: Option<EndecValueKind>,
    /// For `Set<T>` where T is Encodable: the type name.
    pub set_elem_encodable_type: Option<String>,
    /// For `Map<K, V>`: classification of V.
    pub map_value_kind: Option<EndecValueKind>,
    /// For `Map<K, V>` where V is Encodable: the type name.
    pub map_value_encodable_type: Option<String>,
    /// For `Record<K, V>` where V is Encodable: the type name.
    pub record_value_encodable_type: Option<String>,
    /// For wrapper types where T is Encodable: the type name.
    pub wrapper_encodable_type: Option<String>,

    /// Set when the field's resolved type is a two-member union of a
    /// primitive or foreign type with the `Encodable` type: the shape an
    /// `Alias<T> = string | T` or `Alias<T> = RecordId | T` generic resolves
    /// to. The `Encodable(name)` branch decodes through the guard's member
    /// when it matches, and through the type's decoder otherwise.
    pub union_guard: Option<UnionGuard>,

    /// The same guard for the non-null member of a `T | null` field.
    pub nullable_union_guard: Option<UnionGuard>,

    /// The same guard for each element of an `Array<T>` field.
    pub array_elem_union_guard: Option<UnionGuard>,

    /// Validators declared on the *primitive arm* of the resolved
    /// `primitive | Encodable` union: e.g. the `nonEmpty` on the `string`
    /// arm of a record-link alias (`Alias<T> = string | T`). These don't appear
    /// on the field referencing the alias, so they're read from the alias
    /// definition and kept separate from `validators`: they apply only to the
    /// primitive form and are emitted inside the decode branch's
    /// `typeof === primitive` guard. Keeping them out of `validators` avoids
    /// leaking an unguarded check into the per-field `validateField` path,
    /// where the value may be the object form.
    pub union_string_validators: Vec<ValidatorSpec>,
}

impl DecodeField {
    /// Returns true if this field has any validators that need to be applied.
    pub fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }

    /// What a missing value means for this field: allowed for `T | null`,
    /// `T | undefined` and optional fields, required otherwise.
    pub fn missing(&self) -> Missing {
        if self.optional
            || matches!(
                self.type_cat,
                TypeCategory::Nullable(_) | TypeCategory::Optional(_)
            )
        {
            Missing::Allowed
        } else {
            Missing::Required
        }
    }

    /// Returns true if the primitive arm of a `primitive | Encodable` union
    /// carries validators to apply to the primitive form.
    pub fn has_union_string_validators(&self) -> bool {
        !self.union_string_validators.is_empty()
    }
}

/// Returns the encoded (JSON-compatible) type string for a given TS type.
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
            let raw = raw_cast_type(v, &inner_cat);
            rendered(ts_template! { Record<@{k}, @{raw}> })
        }
        TypeCategory::Set(t) => {
            let inner_cat = TypeCategory::from_ts_type(t);
            let raw = raw_cast_type(t, &inner_cat);
            rendered(ts_template! { Array<@{raw}> })
        }
        TypeCategory::Date => "string | Date".to_string(),
        TypeCategory::Nullable(inner) => {
            let inner_cat = TypeCategory::from_ts_type(inner);
            let raw = raw_cast_type(inner, &inner_cat);
            rendered(ts_template! { @{raw} | null })
        }
        TypeCategory::Optional(inner) => {
            let inner_cat = TypeCategory::from_ts_type(inner);
            let raw = raw_cast_type(inner, &inner_cat);
            rendered(ts_template! { @{raw} | undefined })
        }
        TypeCategory::Array(elem) => {
            let inner_cat = TypeCategory::from_ts_type(elem);
            let raw_elem = raw_cast_type(elem, &inner_cat);
            if raw_elem == *elem {
                ts_type.to_string()
            } else {
                rendered(ts_template! { Array<@{raw_elem}> })
            }
        }
        _ => ts_type.to_string(),
    }
}

/// Holds information about a encodable type reference in a union.
///
/// For parameterized types like `RecordLink<Product>`, we need both:
/// - The full type string for `__type` comparison and type casting
/// - The base type name for runtime namespace access
///
/// For foreign types (e.g., `DateTime.Utc`), the inline decode expression
/// is pre-computed so the union template can use it directly instead of generating
/// a function call to a non-existent `{camelCase}DecodeWithContext` function.
#[derive(Clone)]
pub(super) struct EncodableTypeRef {
    /// The full type reference string (e.g., "RecordLink<Product>")
    pub full_type: String,
    /// Whether this type is a foreign type (configured in macroforge.config.ts)
    pub is_foreign: bool,
    /// For foreign types: the decode handler as a callable expression,
    /// e.g. `__foreign__dateTimeUtcDecode`.
    pub foreign_decode_callee: Option<String>,
    /// For foreign types: the hasShape handler as a callable expression,
    /// e.g. `__foreign__dateTimeUtcHasShape`.
    pub foreign_has_shape_callee: Option<String>,
}
