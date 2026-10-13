//! The statements that encode an `@endec(flatten)` field: its encoded object
//! is merged into the enclosing one, without its own tag and `__id`.

use crate::macros::ts_template;
use crate::ts_syn::TsStream;

use crate::builtin::derive::common::rendered;
use crate::builtin::derive::endec::encode::types::{EncodeField, FieldEncoding};

/// Merges `field`'s encoded value into `result`. `tag_field` is the tag the
/// nested encoder writes, which the enclosing object does not take over. A
/// value that is `null` or `undefined` has nothing to merge.
pub(super) fn encode_flattened_field(field: &EncodeField, tag_field: &str) -> TsStream {
    let source = rendered(ts_template! { value@{&field.member} });
    let encoded = match &field.encoding {
        FieldEncoding::ByType(Some(encoded)) => encoded.clone(),
        FieldEncoding::ByType(None) | FieldEncoding::With(_) | FieldEncoding::Decimal => {
            source.clone()
        }
    };
    let merge = if field.is_record {
        ts_template! {
            Object.assign(result, @{encoded});
        }
    } else {
        ts_template! {
            const __flattened = @{encoded};
            const { ["@{tag_field}"]: _, __id: __, ...rest } = __flattened as Record<string, unknown>;
            Object.assign(result, rest);
        }
    };
    let mut present = Vec::new();
    if field.optional {
        present.push(rendered(ts_template! { @{&source} !== undefined }));
    }
    if field.nullable {
        present.push(rendered(ts_template! { @{&source} !== null }));
    }
    if present.is_empty() {
        return ts_template! {
            {
                {$typescript merge}
            }
        };
    }
    let present = present.join(" && ");
    ts_template! {
        if (@{present}) {
            {$typescript merge}
        }
    }
}
