//! The JSON text of an encoded value, through the `@macroforge/core/endec`
//! helpers every top-level `encode` shares.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::return_types::ENCODE_CONTEXT;

/// A `@macroforge/core/endec` helper as a callee.
fn endec_helper(name: &str) -> Expr {
    ts_ident!(crate::builtin::derive::endec::coder::endec_helper(name)).into()
}

/// The statements of a top-level `encode(value, keepMetadata)`: the JSON text
/// of what `encode_with_context` returns for `value`, without `tag_field` and
/// `__id` unless `keepMetadata` is set. `passed_coders` hands on the coders
/// of a generic type's parameters.
pub(super) fn encode_text_body(
    encode_with_context: &Expr,
    tag_field: &str,
    passed_coders: &str,
) -> TsStream {
    let encode_context: Expr = ts_ident!(ENCODE_CONTEXT).into();
    let encoded_text = encoded_text_helper();
    let encoded_text_without_metadata = endec_helper("encodedTextWithoutMetadata");
    ts_template! {
        const ctx = @{encode_context}.create();
        const __raw = @{encode_with_context}(value, ctx@{passed_coders});
        return keepMetadata ? @{encoded_text}(__raw) : @{encoded_text_without_metadata}(__raw, "@{tag_field}");
    }
}

/// The helper giving the JSON text of a value that carries no metadata.
pub(super) fn encoded_text_helper() -> Expr {
    endec_helper("encodedText")
}
