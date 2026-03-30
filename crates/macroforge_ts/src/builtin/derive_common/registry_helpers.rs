use convert_case::{Case, Casing};

use crate::ts_syn::abi::ir::type_registry::{
    ResolvedTypeRef, TypeDefinitionIR, TypeRegistry, TypeRegistryEntry,
};

/// Check if a type in the registry has `@derive(MacroName)` applied.
/// Works for classes, interfaces, enums, and type aliases.
/// Returns false if the type is not in the registry or has no such derive.
///
/// When a name is ambiguous (same type name in multiple files), checks ALL
/// qualified entries — returns true if ANY entry with that name has the derive.
pub fn type_has_derive(registry: &TypeRegistry, type_name: &str, derive_name: &str) -> bool {
    let has_derive = |entry: &TypeRegistryEntry| {
        let decorators = match &entry.definition {
            TypeDefinitionIR::Class(c) => &c.decorators,
            TypeDefinitionIR::Interface(i) => &i.decorators,
            TypeDefinitionIR::Enum(e) => &e.decorators,
            TypeDefinitionIR::TypeAlias(t) => &t.decorators,
        };
        decorators.iter().any(|d| {
            d.name.eq_ignore_ascii_case("derive")
                && d.args_src
                    .split(',')
                    .any(|arg| arg.trim().eq_ignore_ascii_case(derive_name))
        })
    };

    // Check primary entry first (fast path for unambiguous names)
    if let Some(entry) = registry.get(type_name)
        && has_derive(entry)
    {
        return true;
    }

    // If ambiguous, check all qualified entries with this name
    if registry.ambiguous_names.iter().any(|n| n == type_name) {
        return registry
            .qualified_types
            .values()
            .any(|entry| entry.name == type_name && has_derive(entry));
    }

    false
}

/// Check if a resolved field type (or its inner element type for collections)
/// has a specific derive. Handles arrays, generics, and direct types.
#[allow(dead_code)]
pub fn resolved_type_has_derive(
    registry: &TypeRegistry,
    resolved: &ResolvedTypeRef,
    derive_name: &str,
) -> bool {
    type_has_derive(registry, &resolved.base_type_name, derive_name)
}

/// Get the inner element [`ResolvedTypeRef`] for a collection type.
/// For `User[]` or `Array<User>`, returns the `User` ref.
/// For `Map<K, V>`, returns the `V` ref (value type).
/// For `Set<T>`, returns the `T` ref.
pub fn collection_element_type(resolved: &ResolvedTypeRef) -> Option<&ResolvedTypeRef> {
    if !resolved.is_collection || resolved.type_args.is_empty() {
        return None;
    }
    match resolved.base_type_name.as_str() {
        "Map" if resolved.type_args.len() >= 2 => Some(&resolved.type_args[1]),
        _ => Some(&resolved.type_args[0]), // Array, Set, etc.
    }
}

/// Get the Map key type for `Map<K, V>` collections.
#[allow(dead_code)]
pub fn map_key_type(resolved: &ResolvedTypeRef) -> Option<&ResolvedTypeRef> {
    if resolved.base_type_name == "Map" && resolved.type_args.len() >= 2 {
        Some(&resolved.type_args[0])
    } else {
        None
    }
}

/// Generate the standalone function name for a given type and derive macro.
/// E.g., `("User", "Clone")` → `"userClone"`, `("User", "HashCode")` → `"userHashCode"`.
pub fn standalone_fn_name(type_name: &str, suffix: &str) -> String {
    format!("{}{}", type_name.to_case(Case::Camel), suffix)
}
