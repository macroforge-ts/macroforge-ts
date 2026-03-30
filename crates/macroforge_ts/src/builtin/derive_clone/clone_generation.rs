use crate::builtin::derive_common::{
    collection_element_type, is_primitive_type, standalone_fn_name, type_has_derive,
};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

/// Generate a type-aware clone expression for a single field.
///
/// When the type registry is available and confirms the field's type has `@derive(Clone)`,
/// generates a direct function call (e.g., `userClone(value.field)`).
/// Otherwise falls back to shallow copy.
pub(crate) fn generate_clone_expr(
    field_name: &str,
    ts_type: &str,
    var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let access = format!("{var}.{field_name}");

    // If we have resolved type info and a registry, generate optimized clones
    if let (Some(resolved), Some(registry)) = (resolved, registry) {
        // Handle optional types: clone inner if present, else pass through
        if resolved.is_optional {
            let inner_expr =
                generate_clone_for_resolved(field_name, ts_type, var, resolved, registry);
            if inner_expr != access {
                return format!("{access} != null ? {inner_expr} : {access}");
            }
            return access;
        }

        return generate_clone_for_resolved(field_name, ts_type, var, resolved, registry);
    }

    // Fallback: no registry available — shallow copy (backward compatible)
    generate_clone_expr_fallback(field_name, ts_type, var)
}

/// Generate a clone expression using resolved type information.
pub(crate) fn generate_clone_for_resolved(
    field_name: &str,
    ts_type: &str,
    var: &str,
    resolved: &ResolvedTypeRef,
    registry: &TypeRegistry,
) -> String {
    let access = format!("{var}.{field_name}");

    // Direct type with Clone derive → call standalone clone function
    if !resolved.is_collection
        && resolved.registry_key.is_some()
        && type_has_derive(registry, &resolved.base_type_name, "Clone")
    {
        let fn_name = standalone_fn_name(&resolved.base_type_name, "Clone");
        return format!("{fn_name}({access})");
    }

    // Date → deep copy
    if resolved.base_type_name == "Date" && !resolved.is_collection {
        return format!("new Date({access}.getTime())");
    }

    // Collection types
    if resolved.is_collection
        && let Some(elem) = collection_element_type(resolved)
    {
        let base = resolved.base_type_name.as_str();

        match base {
            // Array/User[] — map elements
            _ if base != "Map" && base != "Set" => {
                let elem_clone = element_clone_expr(elem, registry, "v");
                if elem_clone == "v" {
                    // Primitive or unknown element — spread copy
                    return format!("[...{access}]");
                }
                return format!("{access}.map(v => {elem_clone})");
            }
            // Set<T> — always copy, clone elements if they have Clone
            "Set" => {
                let elem_clone = element_clone_expr(elem, registry, "v");
                if elem_clone == "v" {
                    return format!("new Set({access})");
                }
                return format!("new Set(Array.from({access}).map(v => {elem_clone}))");
            }
            // Map<K, V> — clone values
            "Map" => {
                let value_clone = element_clone_expr(elem, registry, "v");
                if value_clone == "v" {
                    return format!("new Map({access})");
                }
                return format!(
                    "new Map(Array.from({access}.entries()).map(([k, v]) => [k, {value_clone}]))"
                );
            }
            _ => {}
        }
    }

    // Fallback for non-registry types
    generate_clone_expr_fallback(field_name, ts_type, var)
}

/// Generate a clone expression for a collection element value.
/// Returns `"v"` if no cloning is needed (primitive/unknown).
pub(crate) fn element_clone_expr(
    elem: &ResolvedTypeRef,
    registry: &TypeRegistry,
    var: &str,
) -> String {
    // Known Clone type → direct call
    if elem.registry_key.is_some() && type_has_derive(registry, &elem.base_type_name, "Clone") {
        return format!(
            "{}({var})",
            standalone_fn_name(&elem.base_type_name, "Clone")
        );
    }

    // Date → deep copy
    if elem.base_type_name == "Date" {
        return format!("new Date({var}.getTime())");
    }

    // Primitive → identity
    if is_primitive_type(&elem.base_type_name) {
        return var.to_string();
    }

    // Unknown type → identity (shallow)
    var.to_string()
}

/// Fallback clone expression when no type registry is available.
/// Handles known built-in types; everything else is shallow copy.
pub(crate) fn generate_clone_expr_fallback(
    field_name: &str,
    ts_type: &str,
    var: &str,
) -> String {
    let access = format!("{var}.{field_name}");
    let t = ts_type.trim();

    if is_primitive_type(t) {
        return access;
    }

    if t == "Date" {
        return format!("new Date({access}.getTime())");
    }

    if t.ends_with("[]") || t.starts_with("Array<") {
        return format!("[...{access}]");
    }

    if t.starts_with("Set<") {
        return format!("new Set({access})");
    }

    if t.starts_with("Map<") {
        return format!("new Map({access})");
    }

    // Unknown type → shallow copy
    access
}
