//! # Return Type Code Generation Helpers
//!
//! This module provides helper functions for generating return type code
//! based on the configured [`ReturnTypesMode`].
//!
//! ## Modes
//!
//! - **Vanilla** (default): Plain TypeScript discriminated unions
//! - **Custom**: Uses `@rydshift/mirror` Result/Option types
//! - **Effect**: Uses Effect library Exit/Option types
//!
//! ## Usage
//!
//! These helpers are used by `derive_deserialize.rs` and `derive_partial_ord.rs`
//! to generate mode-appropriate code.

use crate::host::config::ReturnTypesMode;
use super::serde::get_return_types_mode;

// ============================================================================
// Deserialize Return Type Helpers
// ============================================================================

/// Returns the return type string for Deserialize based on current mode.
///
/// # Arguments
///
/// * `type_name` - The name of the type being deserialized (e.g., "User")
///
/// # Returns
///
/// The appropriate return type signature for the current mode:
/// - Vanilla: `{ success: true; value: T } | { success: false; errors: Array<{ field: string; message: string }> }`
/// - Custom: `Result<T, Array<{ field: string; message: string }>>`
/// - Effect: `Exit<Array<{ field: string; message: string }>, T>`
pub fn deserialize_return_type(type_name: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => {
            format!(
                "{{ success: true; value: {} }} | {{ success: false; errors: Array<{{ field: string; message: string }}> }}",
                type_name
            )
        }
        ReturnTypesMode::Custom => {
            format!(
                "Result<{}, Array<{{ field: string; message: string }}>>",
                type_name
            )
        }
        ReturnTypesMode::Effect => {
            format!(
                "Exit.Exit<Array<{{ field: string; message: string }}>, {}>",
                type_name
            )
        }
    }
}

/// Returns an expression that wraps a success value.
///
/// # Arguments
///
/// * `expr` - The expression to wrap (e.g., "resultOrRef")
///
/// # Returns
///
/// The appropriate success wrapper for the current mode:
/// - Vanilla: `{ success: true, value: <expr> }`
/// - Custom: `Result.ok(<expr>)`
/// - Effect: `Exit.succeed(<expr>)`
pub fn wrap_success(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => {
            format!("{{ success: true, value: {} }}", expr)
        }
        ReturnTypesMode::Custom => {
            format!("Result.ok({})", expr)
        }
        ReturnTypesMode::Effect => {
            format!("Exit.succeed({})", expr)
        }
    }
}

/// Returns an expression that wraps an error value.
///
/// # Arguments
///
/// * `expr` - The error expression to wrap (e.g., "errors")
///
/// # Returns
///
/// The appropriate error wrapper for the current mode:
/// - Vanilla: `{ success: false, errors: <expr> }`
/// - Custom: `Result.err(<expr>)`
/// - Effect: `Exit.fail(<expr>)`
pub fn wrap_error(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => {
            format!("{{ success: false, errors: {} }}", expr)
        }
        ReturnTypesMode::Custom => {
            format!("Result.err({})", expr)
        }
        ReturnTypesMode::Effect => {
            format!("Exit.fail({})", expr)
        }
    }
}

/// Returns the import needed for the Deserialize return type.
///
/// # Returns
///
/// - Vanilla: `None` (no import needed)
/// - Custom: `Some(("Result", "macroforge/utils"))`
/// - Effect: `Some(("Exit", "macroforge/utils/effect"))`
pub fn deserialize_import() -> Option<(&'static str, &'static str)> {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => None,
        ReturnTypesMode::Custom => Some(("Result", "macroforge/utils")),
        ReturnTypesMode::Effect => Some(("Exit", "macroforge/utils/effect")),
    }
}

/// Returns an expression to check if a deserialize result is successful.
///
/// # Arguments
///
/// * `expr` - The result expression to check
///
/// # Returns
///
/// The appropriate success check for the current mode:
/// - Vanilla: `<expr>.success`
/// - Custom: `Result.isOk(<expr>)`
/// - Effect: `Exit.isSuccess(<expr>)`
pub fn is_ok_check(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => format!("{}.success", expr),
        ReturnTypesMode::Custom => format!("Result.isOk({})", expr),
        ReturnTypesMode::Effect => format!("Exit.isSuccess({})", expr),
    }
}

// ============================================================================
// PartialOrd Return Type Helpers
// ============================================================================

/// Returns the return type for PartialOrd based on current mode.
///
/// # Returns
///
/// The appropriate return type for the current mode:
/// - Vanilla: `number | null`
/// - Custom: `Option<number>`
/// - Effect: `Option.Option<number>`
pub fn partial_ord_return_type() -> &'static str {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => "number | null",
        ReturnTypesMode::Custom => "Option<number>",
        ReturnTypesMode::Effect => "Option.Option<number>",
    }
}

/// Returns an expression that wraps a "some" value.
///
/// # Arguments
///
/// * `expr` - The expression to wrap (e.g., "0" or "cmp")
///
/// # Returns
///
/// The appropriate "some" wrapper for the current mode:
/// - Vanilla: `<expr>` (no wrapper needed)
/// - Custom: `Option.some(<expr>)`
/// - Effect: `Option.some(<expr>)`
pub fn wrap_some(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => expr.to_string(),
        ReturnTypesMode::Custom => format!("Option.some({})", expr),
        ReturnTypesMode::Effect => format!("Option.some({})", expr),
    }
}

/// Returns the "none" expression for PartialOrd.
///
/// # Returns
///
/// The appropriate "none" value for the current mode:
/// - Vanilla: `null`
/// - Custom: `Option.none()`
/// - Effect: `Option.none()`
pub fn wrap_none() -> &'static str {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => "null",
        ReturnTypesMode::Custom => "Option.none()",
        ReturnTypesMode::Effect => "Option.none()",
    }
}

/// Returns the import needed for the PartialOrd return type.
///
/// # Returns
///
/// - Vanilla: `None` (no import needed)
/// - Custom: `Some(("Option", "macroforge/utils"))`
/// - Effect: `Some(("Option", "macroforge/utils/effect"))`
pub fn partial_ord_import() -> Option<(&'static str, &'static str)> {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => None,
        ReturnTypesMode::Custom => Some(("Option", "macroforge/utils")),
        ReturnTypesMode::Effect => Some(("Option", "macroforge/utils/effect")),
    }
}

/// Returns true if the current mode requires Option type checking.
///
/// In vanilla mode, null is used instead of Option.isNone(), so we need
/// different comparison logic.
pub fn uses_option_type() -> bool {
    matches!(
        get_return_types_mode(),
        ReturnTypesMode::Custom | ReturnTypesMode::Effect
    )
}

/// Returns the expression to check if an Option value is None.
///
/// # Arguments
///
/// * `expr` - The Option expression to check
///
/// # Returns
///
/// The appropriate check for the current mode:
/// - Vanilla: `<expr> === null`
/// - Custom: `Option.isNone(<expr>)`
/// - Effect: `Option.isNone(<expr>)`
pub fn is_none_check(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => format!("{} === null", expr),
        ReturnTypesMode::Custom => format!("Option.isNone({})", expr),
        ReturnTypesMode::Effect => format!("Option.isNone({})", expr),
    }
}

/// Returns the expression to unwrap an Option value.
///
/// # Arguments
///
/// * `expr` - The Option expression to unwrap
///
/// # Returns
///
/// The appropriate unwrap for the current mode:
/// - Vanilla: `<expr>` (no unwrap needed, value is already the number)
/// - Custom: `<expr>.value`
/// - Effect: `<expr>.value`
pub fn unwrap_option(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => expr.to_string(),
        ReturnTypesMode::Custom => format!("{}.value", expr),
        ReturnTypesMode::Effect => format!("{}.value", expr),
    }
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
/// The appropriate conditional expression:
/// - Vanilla: `<expr>` (null is already the None representation)
/// - Custom: `Option.isNone(<expr>) ? null : <expr>.value`
/// - Effect: `Option.isNone(<expr>) ? null : <expr>.value`
pub fn unwrap_option_or_null(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => expr.to_string(),
        ReturnTypesMode::Custom => format!("Option.isNone({}) ? null : {}.value", expr, expr),
        ReturnTypesMode::Effect => format!("Option.isNone({}) ? null : {}.value", expr, expr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::serde::{set_return_types_mode, clear_return_types_mode};

    #[test]
    fn test_vanilla_deserialize_return_type() {
        clear_return_types_mode();
        let result = deserialize_return_type("User");
        assert!(result.contains("success: true"));
        assert!(result.contains("success: false"));
        assert!(result.contains("User"));
    }

    #[test]
    fn test_custom_deserialize_return_type() {
        set_return_types_mode(ReturnTypesMode::Custom);
        let result = deserialize_return_type("User");
        assert!(result.contains("Result<User"));
        clear_return_types_mode();
    }

    #[test]
    fn test_effect_deserialize_return_type() {
        set_return_types_mode(ReturnTypesMode::Effect);
        let result = deserialize_return_type("User");
        assert!(result.contains("Exit.Exit<"));
        assert!(result.contains("User"));
        clear_return_types_mode();
    }

    #[test]
    fn test_vanilla_wrap_success() {
        clear_return_types_mode();
        assert_eq!(wrap_success("value"), "{ success: true, value: value }");
    }

    #[test]
    fn test_custom_wrap_success() {
        set_return_types_mode(ReturnTypesMode::Custom);
        assert_eq!(wrap_success("value"), "Result.ok(value)");
        clear_return_types_mode();
    }

    #[test]
    fn test_vanilla_partial_ord() {
        clear_return_types_mode();
        assert_eq!(partial_ord_return_type(), "number | null");
        assert_eq!(wrap_some("0"), "0");
        assert_eq!(wrap_none(), "null");
    }

    #[test]
    fn test_custom_partial_ord() {
        set_return_types_mode(ReturnTypesMode::Custom);
        assert_eq!(partial_ord_return_type(), "Option<number>");
        assert_eq!(wrap_some("0"), "Option.some(0)");
        assert_eq!(wrap_none(), "Option.none()");
        clear_return_types_mode();
    }
}
