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
    registry: &TypeRegistry,
) -> String {
    let access = format!("{var}.{field_name}");

    if let Some(resolved) = resolved {
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

    // No resolved type info — shallow copy.
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

    // RegExp → deep copy
    if elem.base_type_name == "RegExp" {
        return format!("new RegExp({var}.source, {var}.flags)");
    }

    // Typed arrays → copy constructor
    if matches!(
        elem.base_type_name.as_str(),
        "Uint8Array"
            | "Int8Array"
            | "Uint16Array"
            | "Int16Array"
            | "Uint32Array"
            | "Int32Array"
            | "Float32Array"
            | "Float64Array"
            | "BigInt64Array"
            | "BigUint64Array"
            | "Uint8ClampedArray"
    ) {
        return format!("new {}({var})", elem.base_type_name);
    }

    // Primitive or builtin → identity
    if is_primitive_type(&elem.base_type_name) {
        return var.to_string();
    }

    // Unknown type → identity (shallow)
    var.to_string()
}

/// Fallback clone expression when no type registry is available.
/// Handles known built-in types; everything else is shallow copy.
pub(crate) fn generate_clone_expr_fallback(field_name: &str, ts_type: &str, var: &str) -> String {
    let access = format!("{var}.{field_name}");
    let t = ts_type.trim();

    if is_primitive_type(t) {
        return access;
    }

    match t {
        "Date" => format!("new Date({access}.getTime())"),
        "RegExp" => format!("new RegExp({access}.source, {access}.flags)"),
        "URL" => format!("new URL({access}.href)"),
        "URLSearchParams" => format!("new URLSearchParams({access})"),
        "Headers" => format!("new Headers({access})"),
        "ArrayBuffer" => format!("{access}.slice(0)"),
        "SharedArrayBuffer" => access,
        "DataView" => format!("new DataView({access}.buffer.slice(0))"),
        "Blob" => format!("new Blob([{access}], {{ type: {access}.type }})"),
        // Error types: shallow copy (errors are rarely cloned deeply)
        "Error" | "TypeError" | "RangeError" | "SyntaxError" | "ReferenceError" | "URIError"
        | "EvalError" => access,
        // Typed arrays: copy constructor
        "Uint8Array" | "Int8Array" | "Uint16Array" | "Int16Array" | "Uint32Array"
        | "Int32Array" | "Float32Array" | "Float64Array" | "BigInt64Array" | "BigUint64Array"
        | "Uint8ClampedArray" => format!("new {t}({access})"),
        // Non-clonable types: identity
        "AbortController" | "FormData" | "FinalizationRegistry" => access,
        // Collections
        _ if t.ends_with("[]") || t.starts_with("Array<") || t.starts_with("ReadonlyArray<") => {
            format!("[...{access}]")
        }
        _ if t.starts_with("Set<") => format!("new Set({access})"),
        _ if t.starts_with("Map<") => format!("new Map({access})"),
        _ if t.starts_with("WeakMap<")
            || t.starts_with("WeakSet<")
            || t.starts_with("WeakRef<") =>
        {
            access
        }
        _ if t.starts_with("Promise<") => access,
        // Utility types that produce object-like values
        _ if crate::ts_syn::type_normalize::is_ts_object_utility_type(
            crate::ts_syn::type_normalize::base_type_name(t),
        ) =>
        {
            format!("({{ ...{access} }})")
        }
        // Unknown type → shallow copy
        _ => access,
    }
}
