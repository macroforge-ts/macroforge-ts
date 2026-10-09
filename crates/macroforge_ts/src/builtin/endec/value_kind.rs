//! How a type's values behave when encoded and decoded, shared by both derives.

use super::TypeCategory;
use crate::ts_syn::abi::ir::is_primitive_keyword;

/// Classifies how a type's value behaves during encoding and decoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EndecValueKind {
    PrimitiveLike,
    Date,
    NullableDate,
    Other,
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
        is_primitive_keyword(part) || is_ts_literal(part)
    })
}

pub(crate) fn classify_endec_value_kind(ts_type: &str) -> EndecValueKind {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive => EndecValueKind::PrimitiveLike,
        TypeCategory::Date => EndecValueKind::Date,
        TypeCategory::Nullable(inner) => match classify_endec_value_kind(&inner) {
            EndecValueKind::Date => EndecValueKind::NullableDate,
            EndecValueKind::PrimitiveLike => EndecValueKind::PrimitiveLike,
            _ => EndecValueKind::Other,
        },
        TypeCategory::Optional(inner) => classify_endec_value_kind(&inner),
        _ => {
            if is_union_of_primitive_like(ts_type) {
                EndecValueKind::PrimitiveLike
            } else {
                EndecValueKind::Other
            }
        }
    }
}

/// If the given type string is a Encodable type, return its name.
/// Returns None for primitives, Date, and other non-encodable types.
pub(crate) fn get_encodable_type_name(ts_type: &str) -> Option<String> {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Encodable(name) => Some(name),
        _ => None,
    }
}
