//! Telling apart the members of a two-member union whose other member is a
//! user type with its own encoder and decoder: `string | Employee`, or
//! `RecordId | Employee`, the resolved shape of `RecordLink<Employee>`.

use crate::ast::Expr;
use crate::builtin::derive::common::{
    detect_primitive_encodable_union, primitive_keyword_test, rendered,
};
use crate::macros::ts_template;
use crate::ts_syn::abi::ir::split_top_level_union;
use crate::ts_syn::abi::ir::type_alias::{TypeMember, TypeMemberKind};
use crate::ts_syn::config::ForeignHandler;

use super::decode::helpers::decoded_keyword_test;
use super::{TypeCategory, get_foreign_types};

/// The tag and the type of a union member written `{ tag: 'name' } & Type`:
/// the one form of an intersection a union is told and coded by. Any other
/// is an error saying so, since a member that is read as none would be left
/// out of the union's coders without a word.
pub(crate) fn tagged_member<'member>(
    sub_members: &'member [TypeMember],
    tag_field: &str,
) -> Result<(String, &'member str), String> {
    let unread = |why: &str| {
        format!(
            "a member of the union is an intersection that {why}, and a union codes an intersection only as `{{ {tag_field}: 'name' }} & Type`: the tag that tells the member, and the one type that holds its fields; name the member with an alias, `type Member = ...`, derive on that, and use the alias in the union"
        )
    };
    let [first, second] = sub_members else {
        return Err(unread("is not of two parts"));
    };
    let (fields, type_ref) = match (&first.kind, &second.kind) {
        (TypeMemberKind::Object { fields }, TypeMemberKind::TypeRef(type_ref))
        | (TypeMemberKind::TypeRef(type_ref), TypeMemberKind::Object { fields }) => {
            (fields, type_ref)
        }
        _ => return Err(unread("is not an object type and a named type")),
    };
    let [tag] = fields.as_slice() else {
        return Err(unread(&format!(
            "holds other fields beside `{tag_field}`, or none"
        )));
    };
    let written = tag.ts_type.trim();
    let name = written
        .strip_prefix('\'')
        .and_then(|rest| rest.strip_suffix('\''))
        .or_else(|| {
            written
                .strip_prefix('"')
                .and_then(|rest| rest.strip_suffix('"'))
        });
    if tag.name != tag_field {
        // A field that names its member is a tag under another name.
        return Err(unread(&match name {
            Some(_) => format!(
                "is tagged by `{}`, where the union's tag is `{tag_field}` (`@endec({{ tag: \"{}\" }})` on the union makes it that)",
                tag.name, tag.name
            ),
            None => format!(
                "holds the field `{}` where the union's tag, `{tag_field}`, would stand",
                tag.name
            ),
        }));
    }
    match name {
        Some(name) => Ok((name.to_string(), type_ref.as_str())),
        None => Err(unread(&format!(
            "has a `{tag_field}` of type `{written}`, which is no one name"
        ))),
    }
}

/// How a union's other member is recognised at runtime, and what it becomes.
#[derive(Clone, Debug)]
pub(crate) enum UnionGuard {
    /// A primitive, recognised by its keyword, or a literal, recognised by
    /// being it, and kept as it is.
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
    /// Whether `value` is the guarded member, as `direction` reads it: a
    /// number being decoded has to be one an encoding can write back.
    pub(crate) fn test(&self, value: &Expr, direction: Direction) -> Expr {
        match self {
            // A literal is told by being itself, a keyword by its `typeof`.
            Self::Primitive(keyword) => {
                let keyword = keyword.trim();
                let is_literal = keyword.starts_with(['"', '\'', '`'])
                    || crate::ts_syn::abi::ir::number_literal(keyword).is_some()
                    || matches!(keyword, "true" | "false");
                if is_literal {
                    rendered(ts_template! { @{value} === @{keyword} }).into()
                } else {
                    let value = rendered(ts_template! { @{value} });
                    match direction {
                        Direction::Decode => decoded_keyword_test(keyword, &value),
                        Direction::Encode => primitive_keyword_test(keyword, &value),
                    }
                    .into()
                }
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
/// foreign type and a user type, with the user type as written, type
/// arguments included. A type parameter, per `is_param`, has no encoder or
/// decoder and so is no user type here.
///
/// Errors when the foreign member lacks the `hasShape`, `decode` or `encode`
/// handler the union needs: without it the members cannot be told apart.
pub(crate) fn union_guard(
    ts_type: &str,
    direction: Direction,
    is_param: impl Fn(&str) -> bool,
) -> Result<Option<(UnionGuard, String)>, String> {
    if let Some((primitive, name)) = detect_primitive_encodable_union(ts_type) {
        let member = split_top_level_union(ts_type.trim())
            .and_then(|parts| parts.into_iter().find(|part| *part != primitive))
            .map_or_else(|| name.clone(), str::to_string);
        return Ok((!is_param(&name)).then_some((UnionGuard::Primitive(primitive), member)));
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
        return Ok(Some((guard, other.trim().to_string())));
    }
    Ok(None)
}
