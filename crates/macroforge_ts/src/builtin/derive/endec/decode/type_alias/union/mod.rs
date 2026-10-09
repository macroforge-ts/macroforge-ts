//! `Decode` for a union type alias.

mod decode;
mod encodable;
mod guards;
mod literal;
mod members;
mod mixed;
mod shape;

use crate::ts_syn::abi::ir::type_alias::TypeMember;
use crate::ts_syn::{MacroforgeError, TsStream};

use super::AliasDecode;
use members::Union;

pub(super) fn handle_union_type_alias(
    alias: &AliasDecode,
    members: &[TypeMember],
) -> Result<TsStream, MacroforgeError> {
    let union = Union::new(alias, members)?;
    let mut result = if union.is_literal_only {
        literal::literal_union(&union)
    } else {
        decode::decode_entry(&union)
            .merge(decode::decode_with_context(&union))
            .merge(shape::has_shape(&union))
            .merge(shape::is_guard(&union))
            .merge(guards::variant_guards(&union))
    };
    result.add_aliased_import("DecodeContext", crate::package::ENDEC);
    result.add_aliased_import("DecodeError", crate::package::ENDEC);
    result.add_aliased_type_import("DecodeOptions", crate::package::ENDEC);
    result.add_aliased_import("PendingRef", crate::package::ENDEC);
    Ok(result)
}
