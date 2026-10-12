//! A value coded through another type's own coders: a user type, or a
//! `Partial`, `Pick` or `Omit` of one, wherever a field holds it.

use crate::builtin::derive::common::{js_string, rendered};
use crate::macros::ts_template;

use super::decode::helpers::{nested_decode_fn_name, nested_decode_partial_fn_name};
use super::encode::types::{nested_encode_fn_name, nested_encode_partial_fn_name};
use super::{TypeCategory, WrapperFields};

/// The user type whose coders code a value, and which of its fields the
/// value has.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NestedType {
    pub(crate) name: String,
    pub(crate) fields: WrapperFields,
}

impl NestedType {
    /// A value holding every field of the type `name`.
    pub(crate) fn whole(name: String) -> Self {
        Self {
            name,
            fields: WrapperFields::Whole,
        }
    }

    /// `ts_type` as a nested type: a user type, or a utility type over one.
    /// `None` for any other type, a type parameter (per `is_param`) included,
    /// which has no coders to call. An error for a utility type that does not
    /// say which fields a value has.
    pub(crate) fn of(
        ts_type: &str,
        is_param: &dyn Fn(&str) -> bool,
    ) -> Result<Option<Self>, String> {
        let ts_type = ts_type.trim();
        match TypeCategory::from_ts_type(ts_type) {
            TypeCategory::Encodable(name) => Ok((!is_param(&name)).then(|| Self::whole(name))),
            TypeCategory::Wrapper(inner) => {
                // `NonNullable<T | null>` is `T`: what is wrapped is the
                // member that is there.
                let mut present = inner;
                while let TypeCategory::Optional(member) | TypeCategory::Nullable(member) =
                    TypeCategory::from_ts_type(&present)
                {
                    present = member;
                }
                let Some(inner) = Self::of(&present, is_param)? else {
                    return Ok(None);
                };
                let Some(fields) = WrapperFields::of(ts_type) else {
                    return Ok(None);
                };
                match (fields?, inner.fields) {
                    (WrapperFields::Whole, fields) | (fields, WrapperFields::Whole) => {
                        Ok(Some(Self {
                            name: inner.name,
                            fields,
                        }))
                    }
                    _ => Err(format!(
                        "`{ts_type}` selects fields of a type that already selects some; name the inner type with an alias and derive on that"
                    )),
                }
            }
            _ => Ok(None),
        }
    }

    /// The `{ only: [...] }` or `{ except: [...] }` a `Pick` or `Omit` is
    /// coded with.
    fn selection(&self) -> Option<String> {
        let (key, names) = match &self.fields {
            WrapperFields::Only(names) => ("only", names),
            WrapperFields::Except(names) => ("except", names),
            WrapperFields::Whole | WrapperFields::Any => return None,
        };
        let names: Vec<String> = names.iter().map(|name| js_string(name)).collect();
        let names = names.join(", ");
        Some(rendered(ts_template! { { @{key}: [@{names}] } }))
    }

    /// The call encoding `value`, with `ctx` in scope.
    pub(crate) fn encode_call(&self, value: &str) -> String {
        if !self.fields.is_partial() {
            let whole = nested_encode_fn_name(&self.name);
            return rendered(ts_template! { @{whole}(@{value}, ctx) });
        }
        let partial = nested_encode_partial_fn_name(&self.name);
        match self.selection() {
            Some(selection) => rendered(ts_template! { @{partial}(@{value}, ctx, @{selection}) }),
            None => rendered(ts_template! { @{partial}(@{value}, ctx) }),
        }
    }

    /// The call decoding `raw`, with `ctx` in scope.
    pub(crate) fn decode_call(&self, raw: &str) -> String {
        if !self.fields.is_partial() {
            let whole = nested_decode_fn_name(&self.name);
            return rendered(ts_template! { @{whole}(@{raw}, ctx) });
        }
        let partial = nested_decode_partial_fn_name(&self.name);
        match self.selection() {
            Some(selection) => rendered(ts_template! { @{partial}(@{raw}, ctx, @{selection}) }),
            None => rendered(ts_template! { @{partial}(@{raw}, ctx) }),
        }
    }
}
