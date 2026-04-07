use crate::builtin::derive_common::{collection_element_type, standalone_fn_name, type_has_derive};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

/// Generate a debug value expression for a field.
/// When the field type has @derive(Debug), calls the standalone toString function.
pub(super) fn debug_value_expr(
    field_name: &str,
    _ts_type: &str,
    var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let access = format!("{var}.{field_name}");

    if let (Some(resolved), Some(registry)) = (resolved, registry) {
        // Direct known Debug type -> call standalone toString function
        if !resolved.is_collection
            && resolved.registry_key.is_some()
            && type_has_derive(registry, &resolved.base_type_name, "Debug")
        {
            let fn_name = standalone_fn_name(&resolved.base_type_name, "ToString");
            return format!("{fn_name}({access})");
        }

        // Array of known Debug type -> map toString
        if resolved.is_collection
            && let Some(elem) = collection_element_type(resolved)
            && elem.registry_key.is_some()
            && type_has_derive(registry, &elem.base_type_name, "Debug")
        {
            let elem_fn = standalone_fn_name(&elem.base_type_name, "ToString");
            return format!("'[' + {access}.map(v => {elem_fn}(v)).join(', ') + ']'");
        }
    }

    // Fallback: string concatenation (original behavior)
    access
}
