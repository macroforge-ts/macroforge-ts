//! A value coded through another type's own coders: a user type, or a
//! `Partial`, `Pick`, `Omit` or `Required` of one, wherever a field holds it.

use crate::builtin::derive::common::{js_string, parse_generic_type, rendered};
use crate::macros::ts_template;

use super::coder::endec_helper;
use super::decode::helpers::{nested_decode_fn_name, nested_decode_partial_fn_name};
use super::encode::types::{nested_encode_fn_name, nested_encode_partial_fn_name};
use super::{TypeCategory, WrapperFields};

/// The user type whose coders code a value, and which of its fields the
/// value has.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NestedType {
    /// The type as written, type arguments included: `Box<Date>`.
    pub(crate) name: String,
    pub(crate) fields: WrapperFields,
    /// Whether a `Required` makes every field the value keeps one it has to
    /// have, the optional ones included.
    pub(crate) required: bool,
    /// The coder of each type argument `name` is written with, in order:
    /// `None` for an argument whose values are their own coding.
    pub(crate) arguments: Vec<Option<String>>,
    /// The fields a decoded value has to have beyond those its type
    /// requires, under `required`: each as its JSON key and its name.
    pub(crate) demanded: Vec<(String, String)>,
}

impl NestedType {
    /// A value holding every field of the type `name`.
    pub(crate) fn whole(name: String) -> Self {
        Self {
            name,
            fields: WrapperFields::Whole,
            required: false,
            arguments: Vec::new(),
            demanded: Vec::new(),
        }
    }

    /// `ts_type` as a nested type: a user type, or a utility type over one.
    /// `None` for any other type, a type parameter (per `is_param`) included,
    /// which has no coders to call. An error for a utility type that does not
    /// say which fields a value has.
    ///
    /// What depends on the names in scope is left empty: the coders of the
    /// type's arguments, and the keys a `Required` demands.
    pub(crate) fn of(
        ts_type: &str,
        is_param: &dyn Fn(&str) -> bool,
    ) -> Result<Option<Self>, String> {
        let ts_type = ts_type.trim();
        match TypeCategory::from_ts_type(ts_type) {
            // The category names the type without its arguments.
            TypeCategory::Encodable(name) => {
                Ok((!is_param(&name)).then(|| Self::whole(ts_type.to_string())))
            }
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
                let makes_required =
                    parse_generic_type(ts_type).is_some_and(|(base, _)| base == "Required");
                match (fields?, inner.fields.clone()) {
                    // A `Required` of a `Partial` has every field again.
                    (WrapperFields::Whole, WrapperFields::Any) if makes_required => {
                        Ok(Some(Self {
                            fields: WrapperFields::Whole,
                            required: true,
                            ..inner
                        }))
                    }
                    (WrapperFields::Whole, fields) => Ok(Some(Self {
                        fields,
                        required: inner.required || makes_required,
                        ..inner
                    })),
                    // A `Partial` has to have none of its fields.
                    (WrapperFields::Any, WrapperFields::Whole) => Ok(Some(Self {
                        fields: WrapperFields::Any,
                        required: false,
                        ..inner
                    })),
                    (fields, WrapperFields::Whole) => Ok(Some(Self { fields, ..inner })),
                    _ => Err(format!(
                        "`{ts_type}` selects fields of a type that already selects some; name the inner type with an alias and derive on that"
                    )),
                }
            }
            _ => Ok(None),
        }
    }

    /// The type arguments `name` is written with, each as written.
    pub(crate) fn type_arguments(&self) -> Vec<&str> {
        parse_generic_type(&self.name)
            .filter(|_| self.name.ends_with('>'))
            .map(|(_, arguments)| crate::ts_syn::abi::ir::split_top_level_commas(arguments))
            .unwrap_or_default()
    }

    /// Whether `field` is one the value keeps.
    pub(crate) fn keeps(&self, field: &str) -> bool {
        match &self.fields {
            WrapperFields::Whole | WrapperFields::Any => true,
            WrapperFields::Only(names) => names.iter().any(|name| name == field),
            WrapperFields::Except(names) => !names.iter().any(|name| name == field),
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

    /// The arguments a call takes after its context: the selection of a
    /// partial call, then the coders of the type's arguments up to the last
    /// one there is, each with its leading comma.
    fn trailing_arguments(&self) -> String {
        let coders = match self.arguments.iter().rposition(Option::is_some) {
            Some(last) => &self.arguments[..=last],
            None => &[],
        };
        let selection = match (self.fields.is_partial(), self.selection()) {
            (false, _) => None,
            (true, Some(selection)) => Some(selection),
            (true, None) if coders.is_empty() => None,
            (true, None) => Some("undefined".to_string()),
        };
        selection
            .into_iter()
            .chain(
                coders
                    .iter()
                    .map(|coder| coder.clone().unwrap_or_else(|| "undefined".to_string())),
            )
            .map(|argument| rendered(ts_template! { , @{argument} }))
            .collect()
    }

    /// The type arguments a call is given, when it hands their coders on:
    /// `<Date>` of `Box<Date>`. The callee's parameters are then the types
    /// the coders were made for.
    fn explicit_arguments(&self) -> &str {
        match self.name.find('<') {
            Some(open) if self.arguments.iter().any(Option::is_some) => &self.name[open..],
            _ => "",
        }
    }

    /// The call encoding `value`, with `ctx` in scope.
    pub(crate) fn encode_call(&self, value: &str) -> String {
        let callee = if self.fields.is_partial() {
            nested_encode_partial_fn_name(&self.name)
        } else {
            nested_encode_fn_name(&self.name)
        };
        let (type_arguments, trailing) = (self.explicit_arguments(), self.trailing_arguments());
        rendered(ts_template! { @{callee}@{type_arguments}(@{value}, ctx@{trailing}) })
    }

    /// The call decoding `raw`, with `ctx` in scope.
    pub(crate) fn decode_call(&self, raw: &str) -> String {
        let callee = if self.fields.is_partial() {
            nested_decode_partial_fn_name(&self.name)
        } else {
            nested_decode_fn_name(&self.name)
        };
        let raw = if self.demanded.is_empty() {
            raw.to_string()
        } else {
            let require_keys = endec_helper("requireKeys");
            let demanded: Vec<String> = self
                .demanded
                .iter()
                .map(|(key, name)| {
                    let (key, name) = (js_string(key), js_string(name));
                    rendered(ts_template! { [@{key}, @{name}] })
                })
                .collect();
            let demanded = demanded.join(", ");
            rendered(ts_template! { @{require_keys}(@{raw}, ctx, [@{demanded}]) })
        };
        let (type_arguments, trailing) = (self.explicit_arguments(), self.trailing_arguments());
        rendered(ts_template! { @{callee}@{type_arguments}(@{raw}, ctx@{trailing}) })
    }
}
