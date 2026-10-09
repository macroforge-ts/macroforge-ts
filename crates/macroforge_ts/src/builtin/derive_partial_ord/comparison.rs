use crate::builtin::derive_common::{
    is_numeric_type, is_primitive_type, standalone_fn_name, type_has_derive,
};
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
    registry: &TypeRegistry,
) -> String {
    generate_value_compare(
        &field.ts_type,
        &format!("{self_var}.{}", field.name),
        &format!("{other_var}.{}", field.name),
        allow_null,
        resolved,
        registry,
    )
}

/// Generates the comparison of two values of `ts_type`, given as expressions
/// (`left` and `right`), with the same strategies as
/// [`generate_field_compare_for_interface`].
pub(crate) fn generate_value_compare(
    ts_type: &str,
    left: &str,
    right: &str,
    allow_null: bool,
    resolved: Option<&ResolvedTypeRef>,
    registry: &TypeRegistry,
) -> String {
    let null_return = if allow_null { "null" } else { "0" };

    // Type-aware path: direct compare call when type has @derive(PartialOrd)
    if let Some(resolved) = resolved
        && !resolved.is_collection
        && resolved.registry_key.is_some()
        && type_has_derive(registry, &resolved.base_type_name, "PartialOrd")
    {
        let fn_name = standalone_fn_name(&resolved.base_type_name, "PartialCompare");
        return format!("{fn_name}({left}, {right})");
    }

    if is_numeric_type(ts_type) {
        format!(
            "({left} < {right} ? -1 : \
             {left} > {right} ? 1 : 0)"
        )
    } else if ts_type == "string" {
        format!("{left}.localeCompare({right})")
    } else if ts_type == "boolean" {
        format!(
            "({left} === {right} ? 0 : \
             {left} ? 1 : -1)"
        )
    } else if is_primitive_type(ts_type) {
        format!("({left} === {right} ? 0 : {null_return})")
    } else if ts_type.ends_with("[]")
        || ts_type.starts_with("Array<")
        || ts_type.starts_with("ReadonlyArray<")
    {
        // Handle nested compareTo calls that return Option<number> or number | null
        let unwrap_opt = unwrap_option_or_null("optResult");
        format!(
            "(() => {{ \
                const __left = {left}; \
                const __right = {right}; \
                if (!Array.isArray(__left) || !Array.isArray(__right)) return {null_return}; \
                const minLen = Math.min(__left.length, __right.length); \
                for (let i = 0; i < minLen; i++) {{ \
                    const __l: any = __left[i]; \
                    const __r: any = __right[i]; \
                    let cmp: number | null; \
                    if (typeof __l?.compareTo === 'function') {{ \
                        const optResult = __l.compareTo(__r); \
                        cmp = {unwrap_opt}; \
                    }} else {{ \
                        cmp = __l < __r ? -1 : __l > __r ? 1 : 0; \
                    }} \
                    if (cmp === null) return {null_return}; \
                    if (cmp !== 0) return cmp; \
                }} \
                return __left.length < __right.length ? -1 : __left.length > __right.length ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "Date" {
        format!(
            "(() => {{ \
                const __left = {left}; \
                const __right = {right}; \
                if (!(__left instanceof Date) || !(__right instanceof Date)) return {null_return}; \
                const ta = __left.getTime(); \
                const tb = __right.getTime(); \
                return ta < tb ? -1 : ta > tb ? 1 : 0; \
            }})()"
        )
    } else if ts_type == "URL" || ts_type == "URLSearchParams" {
        let method = if ts_type == "URL" {
            "href"
        } else {
            "toString()"
        };
        let a = format!("{left}.{method}");
        let b = format!("{right}.{method}");
        format!("((cmp => cmp < 0 ? -1 : cmp > 0 ? 1 : 0)({a}.localeCompare({b})))")
    } else {
        // For objects, check for compareTo method that returns Option<number> or number | null
        let unwrap_opt = unwrap_option_or_null("optResult");
        let is_none = is_none_check("optResult");
        format!(
            "(() => {{ \
                if (typeof ({left} as any)?.compareTo === 'function') {{ \
                    const optResult = ({left} as any).compareTo({right}); \
                    return {is_none} ? {null_return} : {unwrap_opt}; \
                }} \
                return {left} === {right} ? 0 : {null_return}; \
            }})()"
        )
    }
}
