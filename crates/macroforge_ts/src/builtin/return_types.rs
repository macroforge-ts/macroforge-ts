//! # Return Type Code Generation Helpers
//!
//! This module provides helper functions for generating return type code
//! using plain TypeScript discriminated unions.
//!
//! ## Return Types
//!
//! - **Decode**: `{ success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }`
//! - **PartialOrd**: `number | null`
//!
//! ## Notation
//!
//! Type signatures use **semicolons** (TypeScript type literal syntax: `{ a: string; b: number }`),
//! while runtime object expressions use **commas** (`{ a: "hello", b: 42 }`). Both are correct
//! for their respective contexts.
//!
//! ## Usage
//!
//! These helpers are used by the `serde::derive_decode` and `derive_partial_ord`
//! modules to generate vanilla TypeScript code.

// ============================================================================
// Endec Type Aliases
// ============================================================================

// Endec types (aliased) - use with TsStream::add_aliased_import()
/// Aliased name for DecodeContext
pub const DECODE_CONTEXT: &str = "__mf_DecodeContext";
/// Aliased name for DecodeError
pub const DECODE_ERROR: &str = "__mf_DecodeError";
/// Aliased name for DecodeOptions
pub const DECODE_OPTIONS: &str = "__mf_DecodeOptions";
/// Aliased name for PendingRef
pub const PENDING_REF: &str = "__mf_PendingRef";
/// Aliased name for EncodeContext
pub const ENCODE_CONTEXT: &str = "__mf_EncodeContext";

// ============================================================================
// Decode Return Type Helpers
// ============================================================================

/// Returns the return type string for Decode.
///
/// # Arguments
///
/// * `type_name` - The name of the type being decoded (e.g., "User")
///
/// # Returns
///
/// The vanilla return type signature:
/// `{ success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }`
pub fn decode_return_type(type_name: &str) -> String {
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

/// Returns an expression to check if a decode result is successful.
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
    fn test_decode_return_type() {
        let result = decode_return_type("User");
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
    fn test_is_none_check() {
        assert_eq!(is_none_check("opt"), "opt === null");
    }

    #[test]
    fn test_unwrap_option_or_null() {
        assert_eq!(unwrap_option_or_null("opt"), "opt");
    }
}
