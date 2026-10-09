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
//! These helpers are used by the `derive::endec::decode` and `derive::partial_ord`
//! modules to generate vanilla TypeScript code.

use crate::builtin::derive::common::rendered;
use crate::macros::ts_template;

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
    rendered(ts_template! {
        { success: true; value: @{type_name} } | { success: false; errors: Array<{ field: string; message: string }> }
    })
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
    rendered(ts_template! { { success: true, value: @{expr} } })
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
    rendered(ts_template! { { success: false, errors: @{expr} } })
}

/// The error result `decode` returns when the root of `type_name` is a
/// forward reference, which nothing could ever resolve.
pub fn root_forward_reference_error(type_name: &str) -> String {
    let message = crate::builtin::derive::common::js_string(&format!(
        "{type_name}.decode: root cannot be a forward reference"
    ));
    wrap_error(&rendered(
        ts_template! { [{ field: "_root", message: @{message} }] },
    ))
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
    rendered(ts_template! { @{expr}.success })
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
}
