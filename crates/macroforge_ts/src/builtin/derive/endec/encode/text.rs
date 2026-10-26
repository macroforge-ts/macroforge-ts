//! The JSON text of an encoded value, through the `@macroforge/core/endec`
//! helpers every top-level `encode` shares.

use crate::ast::Expr;
use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::return_types::ENCODE_CONTEXT;

/// The local name of a `@macroforge/core/endec` helper, requesting its
/// import for the file being expanded.
fn endec_helper(name: &str) -> Expr {
    let alias = format!("__mf_{name}");
    crate::host::import_registry::with_registry_mut(|registry| {
        registry.request_import(&alias, Some(name), crate::package::ENDEC, false);
    });
    ts_ident!(alias).into()
}

/// The statements of a top-level `encode(value, keepMetadata)`: the JSON text
/// of what `encode_with_context` returns for `value`, without `tag_field` and
/// `__id` unless `keepMetadata` is set.
pub(super) fn encode_text_body(encode_with_context: &Expr, tag_field: &str) -> TsStream {
    let encode_context: Expr = ts_ident!(ENCODE_CONTEXT).into();
    let encoded_text = encoded_text_helper();
    let encoded_text_without_metadata = endec_helper("encodedTextWithoutMetadata");
    ts_template! {
        const ctx = @{encode_context}.create();
        const __raw = @{encode_with_context}(value, ctx);
        return keepMetadata ? @{encoded_text}(__raw) : @{encoded_text_without_metadata}(__raw, "@{tag_field}");
    }
}

/// The helper giving the JSON text of a value that carries no metadata.
pub(super) fn encoded_text_helper() -> Expr {
    endec_helper("encodedText")
}
