use crate::builtin::derive_common::{
    collection_element_type, is_primitive_type, standalone_fn_name, type_has_derive,
};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

use super::types::HashField;

/// Generates JavaScript code that computes a hash contribution for a single field.
///
/// This function produces an expression that evaluates to an integer hash value.
/// The generated code handles different TypeScript types with appropriate
/// hashing strategies.
///
/// # Arguments
///
/// * `field` - The field to generate hash code for
/// * `var` - The variable name to use for field access (e.g., "self", "value")
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to an integer hash value.
/// Field access uses the provided variable name: `var.fieldName`.
///
/// # Type-Specific Strategies
///
/// - **number**: Integer values used directly; floats hashed as strings
/// - **bigint**: String hash of decimal representation
/// - **string**: Character-by-character polynomial hash
/// - **boolean**: 1231 for true, 1237 for false (Java convention)
/// - **Date**: `getTime()` timestamp
/// - **Arrays**: Element-by-element hash combination
/// - **Map**: Entry-by-entry key+value hash
/// - **Set**: Element-by-element hash
/// - **Objects**: Calls `hashCode()` if available, else JSON string hash
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::derive_hash::{HashField, generate_field_hash_for_interface};
///
/// let field = HashField { name: "name".to_string(), ts_type: "string".to_string() };
/// let code = generate_field_hash_for_interface(&field, "self", None, None);
/// assert!(code.contains("self.name"));
/// assert!(code.contains("reduce"));
/// ```
pub fn generate_field_hash_for_interface(
    field: &HashField,
    var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    // Type-aware path: check if the field type has @derive(Hash)
    if let (Some(resolved), Some(registry)) = (resolved, registry) {
        // Direct known Hash type → call standalone function
        if !resolved.is_collection
            && resolved.registry_key.is_some()
            && type_has_derive(registry, &resolved.base_type_name, "Hash")
        {
            let fn_name = standalone_fn_name(&resolved.base_type_name, "HashCode");
            return format!("{fn_name}({var}.{field_name})");
        }

        // Array of known Hash type → direct element calls
        if resolved.is_collection
            && let Some(elem) = collection_element_type(resolved)
            && elem.registry_key.is_some()
            && type_has_derive(registry, &elem.base_type_name, "Hash")
        {
            let elem_fn = standalone_fn_name(&elem.base_type_name, "HashCode");
            let base = resolved.base_type_name.as_str();
            match base {
                "Map" => {
                    return format!(
                        "({var}.{field_name} instanceof Map \
                                    ? Array.from({var}.{field_name}.entries()).reduce((h, [k, v]) => \
                                        (h * 31 + String(k).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) + \
                                        {elem_fn}(v)) | 0, 0) \
                                    : 0)"
                    );
                }
                "Set" => {
                    return format!(
                        "({var}.{field_name} instanceof Set \
                                    ? Array.from({var}.{field_name}).reduce((h, v) => \
                                        (h * 31 + {elem_fn}(v)) | 0, 0) \
                                    : 0)"
                    );
                }
                _ => {
                    // Array types
                    return format!(
                        "(Array.isArray({var}.{field_name}) \
                                    ? {var}.{field_name}.reduce((h, v) => \
                                        (h * 31 + {elem_fn}(v)) | 0, 0) \
                                    : 0)"
                    );
                }
            }
        }
    }

    // Fallback: original duck-typing behavior
    if is_primitive_type(ts_type) {
        match ts_type.as_str() {
            "number" => {
                format!(
                    "(Number.isInteger({var}.{field_name}) \
                        ? {var}.{field_name} | 0 \
                        : {var}.{field_name}.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))"
                )
            }
            "bigint" => {
                format!(
                    "{var}.{field_name}.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)"
                )
            }
            "string" => {
                format!(
                    "({var}.{field_name} ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)"
                )
            }
            "boolean" => {
                format!("({var}.{field_name} ? 1231 : 1237)")
            }
            _ => {
                format!("({var}.{field_name} != null ? 1 : 0)")
            }
        }
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        format!(
            "(Array.isArray({var}.{field_name}) \
                ? {var}.{field_name}.reduce((h, v) => \
                    (h * 31 + (typeof (v as any)?.hashCode === 'function' \
                        ? (v as any).hashCode() \
                        : (v != null ? String(v).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) : 0))) | 0, 0) \
                : 0)"
        )
    } else if ts_type == "Date" {
        format!("({var}.{field_name} instanceof Date ? {var}.{field_name}.getTime() | 0 : 0)")
    } else if ts_type.starts_with("Map<") {
        format!(
            "({var}.{field_name} instanceof Map \
                ? Array.from({var}.{field_name}.entries()).reduce((h, [k, v]) => \
                    (h * 31 + String(k).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) + \
                    (typeof (v as any)?.hashCode === 'function' ? (v as any).hashCode() : 0)) | 0, 0) \
                : 0)"
        )
    } else if ts_type.starts_with("Set<") {
        format!(
            "({var}.{field_name} instanceof Set \
                ? Array.from({var}.{field_name}).reduce((h, v) => \
                    (h * 31 + (typeof (v as any)?.hashCode === 'function' \
                        ? (v as any).hashCode() \
                        : (v != null ? String(v).split('').reduce((hh, c) => (hh * 31 + c.charCodeAt(0)) | 0, 0) : 0))) | 0, 0) \
                : 0)"
        )
    } else {
        format!(
            "(typeof ({var}.{field_name} as any)?.hashCode === 'function' \
                ? ({var}.{field_name} as any).hashCode() \
                : ({var}.{field_name} != null \
                    ? JSON.stringify({var}.{field_name}).split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0) \
                    : 0))"
        )
    }
}
