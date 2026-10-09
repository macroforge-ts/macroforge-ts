use super::super::{TypeCategory, get_foreign_types};
use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;
use crate::ts_syn::config::ForeignHandler;
use crate::ts_syn::ts_ident;

/// Tries to generate a composite encode expression for types where a foreign
/// type is nested inside array, set, or nullable wrappers (e.g., `DateTime.Utc[]`,
/// `DateTime.Utc | null`).
///
/// Similar to `try_composite_foreign_decode` in the decode module.
pub(crate) fn try_composite_foreign_encode(ts_type: &str) -> Option<String> {
    let foreign_types = get_foreign_types();

    // Split by | and classify parts
    let parts: Vec<&str> = ts_type
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let has_null = parts.contains(&"null");
    let has_undefined = parts.contains(&"undefined");
    let non_null: Vec<&str> = parts
        .iter()
        .filter(|s| **s != "null" && **s != "undefined")
        .copied()
        .collect();

    // Only handle single non-null type for now
    if non_null.len() != 1 {
        return None;
    }

    let core = non_null[0];

    // Check for array/set types
    let (container, elem_type) = if let Some(inner) = core.strip_suffix("[]") {
        ("array", inner.trim())
    } else if let Some(rest) = core.strip_prefix("Array<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("array", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Set<") {
        if let Some(inner) = rest.strip_suffix('>') {
            ("set", inner.trim())
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Map<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::super::find_top_level_comma(inner) {
                ("map", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else if let Some(rest) = core.strip_prefix("Record<") {
        if let Some(inner) = rest.strip_suffix('>') {
            if let Some(comma_pos) = super::super::find_top_level_comma(inner) {
                ("record", inner[comma_pos + 1..].trim())
            } else {
                ("none", core)
            }
        } else {
            ("none", core)
        }
    } else if let Some((_, value)) = crate::ts_syn::type_normalize::parse_index_signature(core) {
        // Mirror of the decode-side recognition for `{ [k: K]: V }`
        //: see `decode/helpers.rs`. Required so per-value
        // foreign-type encoders round-trip for fields written in the
        // index-signature surface form.
        ("record", value)
    } else {
        ("none", core)
    };

    // Try matching the element type against foreign types
    let ft_match = TypeCategory::match_foreign_type(elem_type, &foreign_types);
    let rewritten = ft_match
        .config
        .and_then(|ft| ft.handler_callee(ForeignHandler::Encode))?;

    let param = ts_ident!(match container {
        "array" => "arr",
        "set" => "s",
        "map" => "m",
        "record" => "obj",
        _ => "raw",
    });
    let encoded = match container {
        "array" => ts_template! { arr.map(item => @{&rewritten}(item)) },
        "set" => ts_template! { Array.from(s).map(item => @{&rewritten}(item)) },
        "map" => ts_template! {
            Object.fromEntries(Array.from(m.entries()).map(([k, v]) => [k, @{&rewritten}(v)]))
        },
        "record" => ts_template! {
            Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, @{&rewritten}(v)]))
        },
        "none" => ts_template! { @{&rewritten}(raw) },
        // A direct match is caught at the field level.
        _ => return None,
    };
    let arrow = match (container, has_null, has_undefined) {
        // A bare foreign type is encoded directly unless it may be absent.
        ("none", false, false) => return None,
        ("none", true, _) => ts_template! { (raw) => raw === null ? null : @{encoded} },
        ("none", false, true) => {
            ts_template! { (raw) => raw === undefined ? undefined : @{encoded} }
        }
        (_, true, _) | (_, _, true) => {
            ts_template! { (@{&param}) => @{&param} == null ? null : @{encoded} }
        }
        _ => ts_template! { (@{&param}) => @{encoded} },
    };
    Some(rendered(arrow))
}
