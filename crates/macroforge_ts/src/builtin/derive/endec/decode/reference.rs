//! The check every object-graph decoder opens with: a `{ __ref: n }` stands
//! for the object registered as `n` earlier in the same graph.

use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::return_types::DECODE_ERROR;

/// Returns the referenced object (or a pending reference to it) when `value`
/// is a reference, narrowing `value` from `unknown` before reading `__ref`.
/// `returns` is the decoder's return type, which a reference stands in for.
pub(super) fn decode_reference(type_name: &str, returns: &str) -> TsStream {
    let decode_error_ident = ts_ident!(DECODE_ERROR);
    let returns_ident = ts_ident!(returns);
    ts_template! {
        if (typeof value === "object" && value !== null && "__ref" in value) {
            if (typeof value.__ref !== "number") {
                throw new @{decode_error_ident}([{ field: "__ref", message: "@{type_name}.decodeWithContext: __ref must be a number" }]);
            }
            return ctx.getOrDefer(value.__ref) as @{returns_ident};
        }
    }
}
