use crate::ast::Expr;
use crate::builtin::derive::common::Member;

use crate::builtin::derive::common::function_prefix;

/// Generates the EncodePartialWithContext function name for a nested
/// encodable type, which encodes whichever of its fields a value has.
/// For example: "User" -> "userEncodePartialWithContext"
pub(crate) fn nested_encode_partial_fn_name(type_name: &str) -> String {
    let base = type_name
        .find('<')
        .map_or(type_name, |generics| &type_name[..generics]);
    format!("{}EncodePartialWithContext", function_prefix(base))
}

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
    format!("{}EncodeWithContext", function_prefix(base))
}

/// How a field's value is encoded once it is known to be there.
#[derive(Clone)]
pub(crate) enum FieldEncoding {
    /// Through the function `@endec({ encodeWith })` names, whatever the type.
    With(Expr),
    /// As a string, under `@endec({ format: "decimal" })`.
    Decimal,
    /// By its type: the expression encoding the value, or `None` for a value
    /// that is its own encoding.
    ByType(Option<String>),
}

/// A field as `Encode` writes it: where it is read from, the key it is
/// written under, and how its value encodes.
#[derive(Clone)]
pub(crate) struct EncodeField {
    /// The JSON key as reached off `result`. It differs from the field's
    /// name under `@endec(rename = "...")` or the container's `renameAll`.
    pub(crate) json_member: Member,

    /// The field's name as written in the source.
    pub(crate) field_name: String,

    /// The field as reached off `value`.
    pub(crate) member: Member,

    /// Whether the value may be `undefined`, which leaves the key out: an
    /// optional field, or one typed `T | undefined`.
    pub(crate) optional: bool,

    /// Whether the value may be `null`, which is written as it is.
    pub(crate) nullable: bool,

    /// Whether the value's own members are merged into the enclosing object
    /// in place of being written under a key.
    pub(crate) flatten: bool,

    /// Whether the value is a `Record`, whose members are all its own: a
    /// flattened one has no tag or `__id` to leave behind.
    pub(crate) is_record: bool,

    pub(crate) encoding: FieldEncoding,
}
