use crate::builtin::derive_common::{
    is_numeric_type, is_primitive_type, standalone_fn_name, type_has_derive,
};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

use super::types::OrdField;

/// Generates JavaScript code that compares fields for ordering.
///
/// This function produces an expression that evaluates to -1, 0, or 1 indicating
/// the ordering relationship between two values. The generated code handles
/// different TypeScript types with appropriate comparison strategies.
///
/// Unlike `PartialOrd`, this function guarantees a total ordering - it **never
/// returns null**. For incomparable values, it falls back to 0 (equal).
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, or 1.
/// Field access uses the provided variable names: `self_var.field` vs `other_var.field`.
///
/// # Type-Specific Strategies
///
/// - **number/bigint**: Direct `<` and `>` comparison
/// - **string**: Uses `localeCompare()`, result clamped to -1, 0, 1
/// - **boolean**: false is less than true
/// - **null/undefined**: Treated as equal (returns 0)
/// - **Arrays**: Lexicographic comparison with fallback to 0 for incomparable elements
/// - **Date**: Timestamp comparison via `getTime()`
/// - **Objects**: Calls `compareTo()` if available (with `?? 0` fallback), else 0
pub(crate) fn generate_field_compare_for_interface(
    field: &OrdField,
    self_var: &str,
    other_var: &str,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;

    // Type-aware path: direct compare call when type has @derive(Ord)
    if let (Some(resolved), Some(registry)) = (resolved, registry)
        && !resolved.is_collection
        && resolved.registry_key.is_some()
        && type_has_derive(registry, &resolved.base_type_name, "Ord")
    {
        let fn_name = standalone_fn_name(&resolved.base_type_name, "Compare");
        return format!("{fn_name}({self_var}.{field_name}, {other_var}.{field_name})");
    }

    if is_numeric_type(ts_type) {
        format!(
            "({self_var}.{field_name} < {other_var}.{field_name} ? -1 : \
             {self_var}.{field_name} > {other_var}.{field_name} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        format!(
            "((cmp => cmp < 0 ? -1 : cmp > 0 ? 1 : 0)({self_var}.{field_name}.localeCompare({other_var}.{field_name})))"
        )
    } else if ts_type == "boolean" {
        format!(
            "({self_var}.{field_name} === {other_var}.{field_name} ? 0 : \
             {self_var}.{field_name} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        "0".to_string()
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        format!(
            "(() => {{ \
                const a = {self_var}.{field_name} ?? []; \
                const b = {other_var}.{field_name} ?? []; \
                const minLen = Math.min(a.length, b.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    const cmp = typeof (a[i] as any)?.compareTo === 'function' \
                        ? (a[i] as any).compareTo(b[i]) ?? 0 \
                        : (a[i] < b[i] ? -1 : a[i] > b[i] ? 1 : 0); \
                    if (cmp !== 0) return cmp; \
                }} \
                return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        format!(
            "(() => {{ \
                const ta = {self_var}.{field_name}?.getTime() ?? 0; \
                const tb = {other_var}.{field_name}?.getTime() ?? 0; \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else {
        format!(
            "(typeof ({self_var}.{field_name} as any)?.compareTo === 'function' \
                ? ({self_var}.{field_name} as any).compareTo({other_var}.{field_name}) ?? 0 \
                : 0)"
        )
    }
}
