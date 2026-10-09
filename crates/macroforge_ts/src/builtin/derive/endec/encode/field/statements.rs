//! The statements that encode every field of an object-shaped type onto
//! `result`, shared by classes, interfaces and object type aliases.

use crate::ts_syn::TsStream;

use super::flatten::encode_flattened_field;
use super::value::encode_field;
use crate::builtin::derive::endec::encode::types::EncodeField;

/// Encodes each regular field under its JSON key, then merges each
/// flattened field into the object.
pub(in crate::builtin::derive::endec::encode) fn encode_fields(
    fields: &[EncodeField],
    tag_field: &str,
) -> TsStream {
    let regular = fields
        .iter()
        .filter(|field| !field.flatten)
        .map(encode_field);
    let flattened = fields
        .iter()
        .filter(|field| field.flatten)
        .map(|field| encode_flattened_field(field, tag_field));
    TsStream::merge_all(regular.chain(flattened))
}
