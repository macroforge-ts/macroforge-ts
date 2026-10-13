//! The JSON keys an object's encoding keeps for itself, and the keys its
//! fields may not share.

use std::collections::HashMap;

use crate::ts_syn::abi::DiagnosticCollector;

use super::coder::{TypeScope, json_keys_of};
use super::source_field::SourceField;
use super::union_guard::Direction;
use super::{EndecContainerOptions, EndecFieldOptions, TypeCategory};

/// An error for each of `fields` coded under a key it cannot have: one the
/// encoding of its object keeps for itself, the tag `container` writes,
/// `__id` and `__ref`, and one another field is coded under, whether by its
/// name, a `rename`, the container's `renameAll` or a flattened type's own
/// fields. Two fields under one key overwrite each other.
///
/// This is for the fields of a class, an interface and an object alias,
/// whose encoding writes the tag. A tagged union's variant holds its tag as
/// a field of its own.
pub(super) fn check_field_keys<'a>(
    direction: Direction,
    container: &EndecContainerOptions,
    fields: impl IntoIterator<Item = SourceField<'a>>,
    scope: &TypeScope<'_>,
    diagnostics: &mut DiagnosticCollector,
) {
    let (derive, coded_under) = match direction {
        Direction::Encode => ("Encode", "written under"),
        Direction::Decode => ("Decode", "read from"),
    };
    // Each key met so far, with the field coded under it.
    let mut taken: HashMap<String, &'a str> = HashMap::new();
    // The flattened record met so far, which holds every key no field reads.
    let mut holds_the_rest: Option<&'a str> = None;
    for field in fields {
        let options = EndecFieldOptions::from_decorators(field.decorators, field.name).options;
        let coded = match direction {
            Direction::Encode => options.should_encode(),
            Direction::Decode => options.should_decode(),
        };
        if !coded {
            continue;
        }
        let name = field.name;
        let keys = if options.flatten {
            // A flattened type's keys are its own fields', where known.
            let mut present = field.ts_type.trim().to_string();
            while let TypeCategory::Optional(inner) | TypeCategory::Nullable(inner) =
                TypeCategory::from_ts_type(&present)
            {
                present = inner;
            }
            match TypeCategory::from_ts_type(&present) {
                TypeCategory::Encodable(flattened) => {
                    json_keys_of(&flattened, scope).unwrap_or_default()
                }
                TypeCategory::Record(_, _) => {
                    if let Some(other) = holds_the_rest.replace(name) {
                        diagnostics.error(
                            field.span,
                            format!(
                                "@derive({derive}): '{other}' and '{name}' are both flattened records, and each holds every key no other field is {coded_under}, so nothing tells which a key belongs to; flatten one of them"
                            ),
                        );
                    }
                    Vec::new()
                }
                _ => Vec::new(),
            }
        } else {
            let key = options
                .rename
                .unwrap_or_else(|| container.rename_all.apply(name));
            if let Some(reserved) = container.reserved_key(&key) {
                diagnostics.error(
                    field.span,
                    format!(
                        "@derive({derive}): field '{name}' is {coded_under} the key `{key}`, {reserved}; give it another with `@endec({{ rename: \"...\" }})`"
                    ),
                );
                continue;
            }
            vec![key]
        };
        for key in keys {
            match taken.get(key.as_str()) {
                Some(other) if *other != name => diagnostics.error(
                    field.span,
                    format!(
                        "@derive({derive}): fields '{other}' and '{name}' are both {coded_under} the key `{key}`, so one would overwrite the other; give one another key with `@endec({{ rename: \"...\" }})`"
                    ),
                ),
                Some(_) => {}
                None => {
                    taken.insert(key, name);
                }
            }
        }
    }
}
