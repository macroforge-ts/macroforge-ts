//! # Return Type Code Generation Helpers
//!
//! This module provides helper functions for generating return type code
//! using plain TypeScript discriminated unions.
//!
//! ## Return Types
//!
//! - **Deserialize**: `{ success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }`
//! - **PartialOrd**: `number | null`
//!
//! ## Usage
//!
//! These helpers are used by `derive_deserialize.rs` and `derive_partial_ord.rs`
//! to generate vanilla TypeScript code.

// ============================================================================
// Serde Type Aliases
// ============================================================================

// Serde types (aliased) - use with TsStream::add_aliased_import()
/// Aliased name for DeserializeContext
pub const DESERIALIZE_CONTEXT: &str = "__mf_DeserializeContext";
/// Aliased name for DeserializeError
pub const DESERIALIZE_ERROR: &str = "__mf_DeserializeError";
/// Aliased name for DeserializeOptions
pub const DESERIALIZE_OPTIONS: &str = "__mf_DeserializeOptions";
/// Aliased name for PendingRef
pub const PENDING_REF: &str = "__mf_PendingRef";
/// Aliased name for SerializeContext
pub const SERIALIZE_CONTEXT: &str = "__mf_SerializeContext";

// ============================================================================
// Deserialize Return Type Helpers
// ============================================================================

/// Returns the return type string for Deserialize.
///
/// # Arguments
///
/// * `type_name` - The name of the type being deserialized (e.g., "User")
///
/// # Returns
///
/// The vanilla return type signature:
/// `{ success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }`
pub fn deserialize_return_type(type_name: &str) -> String {
    format!(
        "{{ success: true; value: {} }} | {{ success: false; errors: Array<{{ field: string; message: string }}> }}",
        type_name
    )
}

/// Returns an expression that wraps a success value.
///
/// # Arguments
///
/// * `expr` - The expression to wrap (e.g., "resultOrRef")
///
/// # Returns
///
/// The vanilla success wrapper: `{ success: true, value: <expr> }`
pub fn wrap_success(expr: &str) -> String {
    format!("{{ success: true, value: {} }}", expr)
}

/// Returns an expression that wraps an error value.
///
/// # Arguments
///
/// * `expr` - The error expression to wrap (e.g., "errors")
///
/// # Returns
///
/// The vanilla error wrapper: `{ success: false, errors: <expr> }`
pub fn wrap_error(expr: &str) -> String {
    format!("{{ success: false, errors: {} }}", expr)
}

/// Returns an expression to check if a deserialize result is successful.
///
/// # Arguments
///
/// * `expr` - The result expression to check
///
/// # Returns
///
/// The vanilla success check: `<expr>.success`
pub fn is_ok_check(expr: &str) -> String {
    format!("{}.success", expr)
}

// ============================================================================
// PartialOrd Return Type Helpers
// ============================================================================

/// Returns the return type for PartialOrd.
///
/// # Returns
///
/// The vanilla return type: `number | null`
pub fn partial_ord_return_type() -> &'static str {
    "number | null"
}

/// Returns an expression that wraps a "some" value.
///
/// # Arguments
///
/// * `expr` - The expression to wrap (e.g., "0" or "cmp")
///
/// # Returns
///
/// The vanilla "some" value: `<expr>` (no wrapper needed)
pub fn wrap_some(expr: &str) -> String {
    expr.to_string()
}

/// Returns the "none" expression for PartialOrd.
///
/// # Returns
///
/// The vanilla "none" value: `null`
pub fn wrap_none() -> String {
    "null".to_string()
}

/// Returns true if Option type checking is required.
///
/// Since null is used directly, this always returns false.
pub fn uses_option_type() -> bool {
    false
}

/// Returns the expression to check if an Option value is None.
///
/// # Arguments
///
/// * `expr` - The Option expression to check
///
/// # Returns
///
/// The vanilla null check: `<expr> === null`
pub fn is_none_check(expr: &str) -> String {
    format!("{} === null", expr)
}

/// Returns the expression to unwrap an Option value.
///
/// # Arguments
///
/// * `expr` - The Option expression to unwrap
///
/// # Returns
///
/// The vanilla value: `<expr>` (no unwrap needed, value is already the number)
pub fn unwrap_option(expr: &str) -> String {
    expr.to_string()
}

/// Returns the expression to extract a value from an Option, or null if None.
///
/// This is used in nested compareTo calls where we need to extract the inner value.
///
/// # Arguments
///
/// * `expr` - The Option expression to unwrap or get null from
///
/// # Returns
///
/// The vanilla value: `<expr>` (null is already the None representation)
pub fn unwrap_option_or_null(expr: &str) -> String {
    expr.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_return_type() {
        let result = deserialize_return_type("User");
        assert!(result.contains("success: true"));
        assert!(result.contains("success: false"));
        assert!(result.contains("User"));
    }

    #[test]
    fn test_wrap_success() {
        assert_eq!(wrap_success("value"), "{ success: true, value: value }");
    }

    #[test]
    fn test_wrap_error() {
        assert_eq!(wrap_error("errors"), "{ success: false, errors: errors }");
    }

    #[test]
    fn test_is_ok_check() {
        assert_eq!(is_ok_check("result"), "result.success");
    }

    #[test]
    fn test_partial_ord_return_type() {
        assert_eq!(partial_ord_return_type(), "number | null");
    }

    #[test]
    fn test_wrap_some() {
        assert_eq!(wrap_some("0"), "0");
    }

    #[test]
    fn test_wrap_none() {
        assert_eq!(wrap_none(), "null");
    }

    #[test]
    fn test_is_none_check() {
        assert_eq!(is_none_check("opt"), "opt === null");
    }

    #[test]
    fn test_unwrap_option() {
        assert_eq!(unwrap_option("opt"), "opt");
    }

    #[test]
    fn test_unwrap_option_or_null() {
        assert_eq!(unwrap_option_or_null("opt"), "opt");
    }
}
