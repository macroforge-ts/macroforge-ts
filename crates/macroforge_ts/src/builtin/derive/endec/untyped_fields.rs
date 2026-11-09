//! The fields of a class whose declaration says no type to code them by.

use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::abi::ir::class::FieldIR;

use super::EndecFieldOptions;
use super::union_guard::Direction;

/// An error for each of `fields` coded by its type whose declaration gives
/// none: no annotation, and an initializer that takes the type checker to
/// read. A field coded by a function of its own needs no type.
pub(super) fn check_untyped_fields(
    direction: Direction,
    fields: &[FieldIR],
    diagnostics: &mut DiagnosticCollector,
) {
    for field in fields.iter().filter(|field| field.untyped) {
        let options = EndecFieldOptions::from_decorators(&field.decorators, &field.name).options;
        let (derive, coded, by_function, with) = match direction {
            Direction::Encode => (
                "Encode",
                options.should_encode(),
                options.encode_with.is_some(),
                "encodeWith",
            ),
            Direction::Decode => (
                "Decode",
                options.should_decode(),
                options.decode_with.is_some(),
                "decodeWith",
            ),
        };
        if coded && !by_function {
            diagnostics.error(
                field.span,
                format!(
                    "@derive({derive}): field '{name}' has no type annotation, and its initializer does not say its type without the type checker; write the type (`{name}: ...`), or code it with `@endec({{ {with}: ... }})`",
                    name = field.name
                ),
            );
        }
    }
}
