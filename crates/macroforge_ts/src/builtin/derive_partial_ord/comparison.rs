use crate::builtin::derive_common::{is_numeric_type, is_primitive_type, standalone_fn_name, type_has_derive};
use crate::builtin::return_types::{is_none_check, unwrap_option_or_null};
use crate::ts_syn::abi::ir::type_registry::{ResolvedTypeRef, TypeRegistry};

use super::types::OrdField;

/// Generates JavaScript code that compares fields for partial ordering.
///
/// This function produces an expression that evaluates to -1, 0, 1, or `null`.
/// The `null` value indicates incomparable values (the caller wraps results in `Option`).
///
/// # Arguments
///
/// * `field` - The field to generate comparison code for
/// * `self_var` - Variable name for the first object (e.g., "self", "a")
/// * `other_var` - Variable name for the second object (e.g., "other", "b")
/// * `allow_null` - Whether to return `null` for incomparable values (true for
///   PartialOrd, false for Ord which uses 0 instead)
///
/// # Returns
///
/// A string containing a JavaScript expression that evaluates to -1, 0, 1, or null.
/// Field access uses the provided variable names: `self_var.field` vs `other_var.field`.
///
/// # Type-Specific Strategies
///
/// - **number/bigint**: Direct comparison, never null
/// - **string**: `localeCompare()`, never null
/// - **boolean**: false < true, never null
/// - **null/undefined**: Returns `null_return` if values differ
/// - **Arrays**: Returns null if element comparison returns null
/// - **Date**: Returns null if either value is not a valid Date
/// - **Objects**: Unwraps `Option` from nested `compareTo()` calls
pub(crate) fn generate_field_compare_for_interface(
    field: &OrdField,
    self_var: &str,
    other_var: &str,
    allow_null: bool,
    resolved: Option<&ResolvedTypeRef>,
    registry: Option<&TypeRegistry>,
) -> String {
    let field_name = &field.name;
    let ts_type = &field.ts_type;
    let null_return = if allow_null { "null" } else { "0" };

    // Type-aware path: direct compare call when type has @derive(PartialOrd)
    if let (Some(resolved), Some(registry)) = (resolved, registry)
        && !resolved.is_collection
        && resolved.registry_key.is_some()
        && type_has_derive(registry, &resolved.base_type_name, "PartialOrd")
    {
        let fn_name = standalone_fn_name(&resolved.base_type_name, "PartialCompare");
        return format!("{fn_name}({self_var}.{field_name}, {other_var}.{field_name})");
    }

    if is_numeric_type(ts_type) {
        format!(
            "({self_var}.{field_name} < {other_var}.{field_name} ? -1 : \
             {self_var}.{field_name} > {other_var}.{field_name} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        format!("{self_var}.{field_name}.localeCompare({other_var}.{field_name})")
    } else if ts_type == "boolean" {
        format!(
            "({self_var}.{field_name} === {other_var}.{field_name} ? 0 : \
             {self_var}.{field_name} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        format!("({self_var}.{field_name} === {other_var}.{field_name} ? 0 : {null_return})")
    } else if ts_type.ends_with("[]") || ts_type.starts_with("Array<") {
        // Handle nested compareTo calls that return Option<number> or number | null
        let unwrap_opt = unwrap_option_or_null("optResult");
        format!(
            "(() => {{ \
                const a = {self_var}.{field_name}; \
                const b = {other_var}.{field_name}; \
                if (!Array.isArray(a) || !Array.isArray(b)) return {null_return}; \
                const minLen = Math.min(a.length, b.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    let cmp: number | null; \
                    if (typeof (a[i] as any)?.compareTo === 'function') {{ \
                        const optResult = (a[i] as any).compareTo(b[i]); \
                        cmp = {unwrap_opt}; \
                    }} else {{ \
                        cmp = a[i] < b[i] ? -1 : a[i] > b[i] ? 1 : 0; \
                    }} \
                    if (cmp === null) return {null_return}; \
                    if (cmp !== 0) return cmp; \
                }} \
                return a.length < b.length ? -1 : a.length > b.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        format!(
            "(() => {{ \
                const a = {self_var}.{field_name}; \
                const b = {other_var}.{field_name}; \
                if (!(a instanceof Date) || !(b instanceof Date)) return {null_return}; \
                const ta = a.getTime(); \
                const tb = b.getTime(); \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else {
        // For objects, check for compareTo method that returns Option<number> or number | null
        let unwrap_opt = unwrap_option_or_null("optResult");
        let is_none = is_none_check("optResult");
        format!(
            "(() => {{ \
                if (typeof ({self_var}.{field_name} as any)?.compareTo === 'function') {{ \
                    const optResult = ({self_var}.{field_name} as any).compareTo({other_var}.{field_name}); \
                    return {is_none} ? {null_return} : {unwrap_opt}; \
                }} \
                return {self_var}.{field_name} === {other_var}.{field_name} ? 0 : {null_return}; \
            }})()"
        )
    }
}
