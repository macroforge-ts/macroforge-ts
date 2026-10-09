//! Telling apart the members of a two-member union whose other member is a
//! user type with its own encoder and decoder: `string | Employee`, or
//! `RecordId | Employee`, the resolved shape of `RecordLink<Employee>`.

use crate::ast::Expr;
use crate::builtin::derive::common::{detect_primitive_encodable_union, js_string, rendered};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::split_top_level_union;
use crate::ts_syn::config::ForeignHandler;

use super::{TypeCategory, get_foreign_types};

/// How a union's other member is recognised at runtime, and what it becomes.
#[derive(Clone, Debug)]
pub(crate) enum UnionGuard {
    /// A primitive, recognised by `typeof` and kept as it is.
    Primitive(String),
    /// A foreign type, recognised by its `hasShape` and converted by its
    /// `decode` or `encode` handler.
    Foreign { has_shape: Expr, convert: Expr },
}

/// Which way a guarded member is converted.
#[derive(Clone, Copy)]
pub(crate) enum Direction {
    Decode,
    Encode,
}

impl UnionGuard {
    /// Whether `value` is the guarded member.
    pub(crate) fn test(&self, value: &Expr) -> Expr {
        match self {
            Self::Primitive(keyword) => {
                let keyword = js_string(keyword);
                rendered(ts_template! { typeof @{value} === @{keyword} }).into()
            }
            Self::Foreign { has_shape, .. } => {
                rendered(ts_template! { @{has_shape}(@{value}) }).into()
            }
        }
    }

    /// The guarded member's value, converted.
    pub(crate) fn convert(&self, value: &Expr) -> Expr {
        match self {
            Self::Primitive(_) => value.clone(),
            Self::Foreign { convert, .. } => rendered(ts_template! { @{convert}(@{value}) }).into(),
        }
    }

    /// The keyword of a primitive guard.
    pub(crate) fn primitive(&self) -> Option<&str> {
        match self {
            Self::Primitive(keyword) => Some(keyword),
            Self::Foreign { .. } => None,
        }
    }
}

/// The guard for `ts_type` when it is a two-member union of a primitive or a
/// foreign type and a user type, with the user type's name. A type parameter,
/// per `is_param`, has no encoder or decoder and so is no user type here.
///
/// Errors when the foreign member lacks the `hasShape`, `decode` or `encode`
/// handler the union needs: without it the members cannot be told apart.
pub(crate) fn union_guard(
    ts_type: &str,
    direction: Direction,
    is_param: impl Fn(&str) -> bool,
) -> Result<Option<(UnionGuard, String)>, String> {
    if let Some((primitive, name)) = detect_primitive_encodable_union(ts_type) {
        return Ok((!is_param(&name)).then_some((UnionGuard::Primitive(primitive), name)));
    }
    let Some(parts) = split_top_level_union(ts_type.trim()) else {
        return Ok(None);
    };
    let [left, right] = parts.as_slice() else {
        return Ok(None);
    };
    let foreign_types = get_foreign_types();
    for (foreign, other) in [(left, right), (right, left)] {
        let Some(config) = TypeCategory::match_foreign_type(foreign, &foreign_types).config else {
            continue;
        };
        let TypeCategory::Encodable(name) = TypeCategory::from_ts_type(other) else {
            continue;
        };
        if is_param(&name)
            || TypeCategory::match_foreign_type(other, &foreign_types)
                .config
                .is_some()
        {
            continue;
        }
        let handler = match direction {
            Direction::Decode => ForeignHandler::Decode,
            Direction::Encode => ForeignHandler::Encode,
        };
        let missing = |what: &str| {
            format!(
                "foreign type '{}' in `{ts_type}` needs a `{what}` handler to be told apart from '{name}'",
                config.name
            )
        };
        let has_shape = config
            .handler_callee(ForeignHandler::HasShape)
            .ok_or_else(|| missing("hasShape"))?;
        let convert = config
            .handler_callee(handler)
            .ok_or_else(|| missing(handler.key()))?;
        let guard = UnionGuard::Foreign {
            has_shape: has_shape.into(),
            convert: convert.into(),
        };
        return Ok(Some((guard, name)));
    }
    Ok(None)
}
