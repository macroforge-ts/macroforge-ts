//! The check every object-graph decoder opens with: a `{ __ref: n }` stands
//! for the object registered as `n` earlier in the same graph.

use crate::macros::ts_template;
use crate::ts_syn::{TsStream, ts_ident};

use crate::builtin::return_types::DECODE_ERROR;

/// The reference check of a decoder that also decodes a selection of its
/// type's fields, with the requirement `__fields` in scope. A value decoded
/// by selection may not be a reference: the object referred to was decoded
/// whole, and would carry the fields the selection leaves out.
pub(super) fn decode_selectable_reference(type_name: &str, returns: &str) -> TsStream {
    let decode_error_ident = ts_ident!(DECODE_ERROR);
    let reference = decode_reference(type_name, returns);
    ts_template! {
        if (__fields !== undefined && typeof value === "object" && value !== null && "__ref" in value) {
            throw new @{decode_error_ident}([{ field: "_root", message: "@{type_name}.decodeWithContext: a value holding a selection of fields cannot be a reference" }]);
        }
        {$typescript reference}
    }
}

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
