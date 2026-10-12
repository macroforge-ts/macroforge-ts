//! The coders a generic type takes for its type parameters.
//!
//! A type parameter says nothing of how its values are coded, so the coders
//! of a generic type take one optional coder for each, after the context. A
//! field written with the type's arguments, `Box<Date>`, hands in a coder
//! made from each argument; a value of a parameter given none is carried as
//! it is.

use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;

use super::coder::{TypeScope, endec_type};
use super::decode::helpers::{extract_base_type, nested_decode_fn_name};
use super::encode::types::nested_encode_fn_name;
use super::union_guard::Direction;
use crate::builtin::return_types::{DECODE_CONTEXT, ENCODE_CONTEXT};

/// The parameter a coder of a generic type takes its coder for the type
/// parameter `param` in: `__decodeT` for `T`.
pub(crate) fn param_coder(direction: Direction, param: &str) -> String {
    match direction {
        Direction::Encode => format!("__encode{param}"),
        Direction::Decode => format!("__decode{param}"),
    }
}

/// The coder parameters of a generic type with `params`, as a parameter list
/// declares them after the ones before: `, __decodeT?: Decoder<T>`. Empty for
/// a type with no parameters.
pub(crate) fn declared_param_coders(direction: Direction, params: &[String]) -> String {
    if params.is_empty() {
        return String::new();
    }
    let coder_type = endec_type(match direction {
        Direction::Encode => "Encoder",
        Direction::Decode => "Decoder",
    });
    params
        .iter()
        .map(|param| {
            let name = param_coder(direction, param);
            rendered(ts_template! { , @{name}?: @{&coder_type}<@{param}> })
        })
        .collect()
}

/// The coder parameters of a generic type with `params`, as a call hands
/// them on after the arguments before: `, __decodeT`.
pub(crate) fn passed_param_coders(direction: Direction, params: &[String]) -> String {
    params
        .iter()
        .map(|param| {
            let name = param_coder(direction, param);
            rendered(ts_template! { , @{name} })
        })
        .collect()
}

/// The decoders of a generic type with `params` where a parameter follows
/// them, which an optional one may not be followed by:
/// `, __decodeT: Decoder<T> | undefined`.
pub(crate) fn required_param_coders(params: &[String]) -> String {
    if params.is_empty() {
        return String::new();
    }
    let coder_type = endec_type("Decoder");
    params
        .iter()
        .map(|param| {
            let name = param_coder(Direction::Decode, param);
            rendered(ts_template! { , @{name}: @{&coder_type}<@{param}> | undefined })
        })
        .collect()
}

/// A callee decoding one value of the type `ts_type`, called as
/// `(value, ctx)`: the type's own decoder, or, for a generic type written
/// with arguments that have decoders, a function handing those on.
pub(crate) fn member_decoder(ts_type: &str, scope: &TypeScope<'_>) -> Result<String, String> {
    match scope.nested(ts_type, Direction::Decode, 0)? {
        Some(nested) if nested.arguments.iter().any(Option::is_some) => {
            let decoded = nested.decode_call("__member");
            Ok(rendered(ts_template! {
                ((__member: unknown, ctx: @{DECODE_CONTEXT}) => @{decoded})
            }))
        }
        _ => Ok(nested_decode_fn_name(&extract_base_type(ts_type))),
    }
}

/// A callee encoding one value of the type `ts_type`, called as
/// `(value, ctx)`, like [`member_decoder`].
pub(crate) fn member_encoder(ts_type: &str, scope: &TypeScope<'_>) -> Result<String, String> {
    match scope.nested(ts_type, Direction::Encode, 0)? {
        Some(nested) if nested.arguments.iter().any(Option::is_some) => {
            let encoded = nested.encode_call("__member");
            Ok(rendered(ts_template! {
                ((__member: @{ts_type}, ctx: @{ENCODE_CONTEXT}) => @{encoded})
            }))
        }
        _ => Ok(nested_encode_fn_name(ts_type)),
    }
}
