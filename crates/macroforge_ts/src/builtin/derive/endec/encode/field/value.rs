//! The statement that encodes one regular field of an object-shaped type
//! onto `result`, shared by classes, interfaces and object type aliases.
//!
//! The generated code runs inside an `encodeWithContext` body that has
//! `value`, `ctx` and `result` in scope.

use crate::macros::ts_template;
use crate::ts_syn::TsStream;

use crate::builtin::derive::common::rendered;
use crate::builtin::derive::endec::encode::types::{EncodeField, FieldEncoding};

/// Encodes `field` from `value` onto `result` under its JSON key. A value
/// that is `undefined` leaves the key out.
pub(super) fn encode_field(field: &EncodeField) -> TsStream {
    let source = rendered(ts_template! { value@{&field.member} });
    let encoded = match &field.encoding {
        FieldEncoding::With(encode_with) => rendered(ts_template! { @{encode_with}(@{&source}) }),
        FieldEncoding::Decimal => rendered(ts_template! { String(@{&source}) }),
        FieldEncoding::ByType(None) => source.clone(),
        FieldEncoding::ByType(Some(encoded)) if field.nullable => {
            rendered(ts_template! { @{&source} === null ? null : @{encoded} })
        }
        FieldEncoding::ByType(Some(encoded)) => encoded.clone(),
    };
    if field.optional {
        ts_template! {
            if (@{&source} !== undefined) {
                result@{&field.json_member} = @{encoded};
            }
        }
    } else {
        ts_template! {
            result@{&field.json_member} = @{encoded};
        }
    }
}
