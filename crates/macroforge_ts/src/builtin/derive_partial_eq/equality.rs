use crate::builtin::derive_common::{
    collection_element_type, is_primitive_type, standalone_fn_name, type_has_derive,
};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

use super::types::EqField;

/// Generates JavaScript code that compares fields for equality.
///
/// This function produces an expression that evaluates to a boolean indicating
/// whether the field values are equal. The generated code handles different
/// TypeScript types with appropriate comparison strategies.
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
///
/// # Returns
///
/// A string containing a JavaScript boolean expression comparing `self_var.field`
/// with `other_var.field`. The expression can be combined with `&&` for
/// multiple fields.
///
/// # Type-Specific Strategies
///
/// - **Primitives**: Uses strict equality (`===`)
/// - **Arrays**: Checks length, then compares elements (calls `equals` if available)
/// - **Date**: Compares via `getTime()` timestamps
/// - **Map**: Checks size, then compares all entries
/// - **Set**: Checks size, then verifies all elements present in both
/// - **Objects**: Calls `equals()` method if available, falls back to `===`
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::derive_partial_eq::{EqField, generate_field_equality_for_interface};
///
/// let field = EqField { name: "name".to_string(), ts_type: "string".to_string() };
/// let code = generate_field_equality_for_interface(&field, "self", "other", None, None);
/// assert_eq!(code, "self.name === other.name");
/// ```
pub fn generate_field_equality_for_interface(
    field: &EqField,
    self_var: &str,
    other_var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    // Type-aware path: direct equality calls when type has @derive(PartialEq)
    if let (Some(resolved), Some(registry)) = (resolved, registry) {
        // Direct known PartialEq type -> call standalone function
        if !resolved.is_collection
            && resolved.registry_key.is_some()
            && type_has_derive(registry, &resolved.base_type_name, "PartialEq")
        {
            let fn_name = standalone_fn_name(&resolved.base_type_name, "Equals");
            return format!("{fn_name}({self_var}.{field_name}, {other_var}.{field_name})");
        }

        // Array of known PartialEq type -> direct element calls
        if resolved.is_collection
            && let Some(elem) = collection_element_type(resolved)
            && elem.registry_key.is_some()
            && type_has_derive(registry, &elem.base_type_name, "PartialEq")
        {
            let elem_fn = standalone_fn_name(&elem.base_type_name, "Equals");
            let base = resolved.base_type_name.as_str();
            match base {
                "Map" => {
                    return format!(
                        "({self_var}.{field_name} instanceof Map && {other_var}.{field_name} instanceof Map && \
                                 {self_var}.{field_name}.size === {other_var}.{field_name}.size && \
                                 Array.from({self_var}.{field_name}.entries()).every(([k, v]) => \
                                    {other_var}.{field_name}.has(k) && \
                                    {elem_fn}(v, {other_var}.{field_name}.get(k))))"
                    );
                }
                "Set" => {
                    // Set equality with known type -- fall through to default Set comparison
                    // (Sets use .has() which is identity-based, same logic)
                }
                _ => {
                    // Array types
                    return format!(
                        "(Array.isArray({self_var}.{field_name}) && Array.isArray({other_var}.{field_name}) && \
                                 {self_var}.{field_name}.length === {other_var}.{field_name}.length && \
                                 {self_var}.{field_name}.every((v, i) => \
                                    {elem_fn}(v, {other_var}.{field_name}[i])))"
                    );
                }
            }
        }
    }

    // Fallback: original duck-typing behavior
    if is_primitive_type(ts_type) {
        format!("{self_var}.{field_name} === {other_var}.{field_name}")
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        format!(
            "(Array.isArray({self_var}.{field_name}) && Array.isArray({other_var}.{field_name}) && \
             {self_var}.{field_name}.length === {other_var}.{field_name}.length && \
             {self_var}.{field_name}.every((v, i) => \
                typeof (v as any)?.equals === 'function' \
                    ? (v as any).equals({other_var}.{field_name}[i]) \
                    : v === {other_var}.{field_name}[i]))"
        )
    } else if ts_type == "Date" {
        format!(
            "({self_var}.{field_name} instanceof Date && {other_var}.{field_name} instanceof Date \
             ? {self_var}.{field_name}.getTime() === {other_var}.{field_name}.getTime() \
             : {self_var}.{field_name} === {other_var}.{field_name})"
        )
    } else if ts_type.starts_with("Map<") {
        format!(
            "({self_var}.{field_name} instanceof Map && {other_var}.{field_name} instanceof Map && \
             {self_var}.{field_name}.size === {other_var}.{field_name}.size && \
             Array.from({self_var}.{field_name}.entries()).every(([k, v]) => \
                {other_var}.{field_name}.has(k) && \
                (typeof (v as any)?.equals === 'function' \
                    ? (v as any).equals({other_var}.{field_name}.get(k)) \
                    : v === {other_var}.{field_name}.get(k))))"
        )
    } else if ts_type.starts_with("Set<") {
        format!(
            "({self_var}.{field_name} instanceof Set && {other_var}.{field_name} instanceof Set && \
             {self_var}.{field_name}.size === {other_var}.{field_name}.size && \
             Array.from({self_var}.{field_name}).every(v => {other_var}.{field_name}.has(v)))"
        )
    } else {
        format!(
            "(typeof ({self_var}.{field_name} as any)?.equals === 'function' \
                ? ({self_var}.{field_name} as any).equals({other_var}.{field_name}) \
                : {self_var}.{field_name} === {other_var}.{field_name})"
        )
    }
}
