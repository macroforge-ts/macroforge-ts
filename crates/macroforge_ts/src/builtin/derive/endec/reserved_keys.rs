//! The JSON keys an object's encoding keeps for itself.

use crate::ts_syn::abi::DiagnosticCollector;

use super::source_field::SourceField;
use super::union_guard::Direction;
use super::{EndecContainerOptions, EndecFieldOptions};

/// An error for each of `fields` coded under a key the encoding of its
/// object keeps for itself: the tag `container` writes, `__id` and `__ref`.
///
/// This is for the fields of a class, an interface and an object alias,
/// whose encoding writes the tag. A tagged union's variant holds its tag as
/// a field of its own.
pub(super) fn check_reserved_keys<'a>(
    direction: Direction,
    container: &EndecContainerOptions,
    fields: impl IntoIterator<Item = SourceField<'a>>,
    diagnostics: &mut DiagnosticCollector,
) {
    for field in fields {
        let options = EndecFieldOptions::from_decorators(field.decorators, field.name).options;
        let coded = match direction {
            Direction::Encode => options.should_encode(),
            Direction::Decode => options.should_decode(),
        };
        if !coded || options.flatten {
            continue;
        }
        let key = options
            .rename
            .unwrap_or_else(|| container.rename_all.apply(field.name));
        let Some(taken) = container.reserved_key(&key) else {
            continue;
        };
        let name = field.name;
        diagnostics.error(
            field.span,
            match direction {
                Direction::Encode => format!(
                    "@derive(Encode): field '{name}' is written under the key `{key}`, {taken}; write it under another with `@endec({{ rename: \"...\" }})`"
                ),
                Direction::Decode => format!(
                    "@derive(Decode): field '{name}' is read from the key `{key}`, {taken}; read it from another with `@endec({{ rename: \"...\" }})`"
                ),
            },
        );
    }
}
