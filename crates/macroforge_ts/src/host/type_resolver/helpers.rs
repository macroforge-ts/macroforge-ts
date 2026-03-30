use std::collections::HashMap;

use crate::ts_syn::abi::TargetIR;
use crate::ts_syn::abi::ir::type_registry::ResolvedTypeRef;

use super::resolver::TypeResolver;

/// Resolve all field types for a target declaration.
///
/// Returns a map of field name -> [`ResolvedTypeRef`].
pub fn resolve_target_fields(
    target: &TargetIR,
    resolver: &TypeResolver,
) -> HashMap<String, ResolvedTypeRef> {
    let mut resolved = HashMap::new();

    match target {
        TargetIR::Class(class) => {
            for field in &class.fields {
                resolved.insert(field.name.clone(), resolver.resolve(&field.ts_type));
            }
        }
        TargetIR::Interface(iface) => {
            for field in &iface.fields {
                resolved.insert(field.name.clone(), resolver.resolve(&field.ts_type));
            }
        }
        TargetIR::TypeAlias(alias) => {
            if let Some(fields) = alias.body.as_object() {
                for field in fields {
                    resolved.insert(field.name.clone(), resolver.resolve(&field.ts_type));
                }
            }
        }
        _ => {} // Enums don't have typed fields in the same way
    }

    resolved
}

/// Strip trailing `[]` from a type string, returning the inner type.
pub(crate) fn strip_array_suffix(s: &str) -> Option<&str> {
    if let Some(inner) = s.strip_suffix("[]") {
        Some(inner.trim())
    } else if let Some(inner) = s.strip_prefix("Array<").and_then(|s| s.strip_suffix('>')) {
        Some(inner.trim())
    } else if let Some(inner) = s
        .strip_prefix("ReadonlyArray<")
        .and_then(|s| s.strip_suffix('>'))
    {
        Some(inner.trim())
    } else {
        None
    }
}

/// Strip `| undefined` or `| null` from the end of a type string.
pub(crate) fn strip_optional(s: &str) -> Option<&str> {
    // Check for " | undefined" at the end
    if let Some(pos) = s.rfind(" | undefined")
        && pos + " | undefined".len() == s.len()
    {
        return Some(s[..pos].trim());
    }
    // Check for " | null" at the end
    if let Some(pos) = s.rfind(" | null")
        && pos + " | null".len() == s.len()
    {
        return Some(s[..pos].trim());
    }
    None
}

/// Split a generic type into its base name and type arguments.
///
/// `"Map<string, User>"` -> `("Map", ["string", "User"])`
pub(crate) fn split_generic(s: &str) -> Option<(&str, Vec<&str>)> {
    let open = s.find('<')?;
    if !s.ends_with('>') {
        return None;
    }
    let base = &s[..open];
    let args_str = &s[open + 1..s.len() - 1];
    let args = split_type_args(args_str);
    Some((base, args))
}

/// Split comma-separated type arguments, respecting nested generics.
pub(crate) fn split_type_args(s: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                let arg = s[start..i].trim();
                if !arg.is_empty() {
                    result.push(arg);
                }
                start = i + 1;
            }
            _ => {}
        }
    }

    let last = s[start..].trim();
    if !last.is_empty() {
        result.push(last);
    }

    result
}

/// Check if a type name represents a collection type.
pub(crate) fn is_collection_type(name: &str) -> bool {
    matches!(
        name,
        "Array" | "Set" | "Map" | "WeakMap" | "WeakSet" | "ReadonlyArray"
    )
}
