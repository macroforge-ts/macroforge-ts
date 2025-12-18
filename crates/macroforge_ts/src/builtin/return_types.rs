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
//! ## Import Strategy
//!
//! For Effect and Custom modes, we import specific functions with `__mf_` prefixed
//! aliases to avoid any risk of collision with user imports:
//!
//! ```typescript
//! // Effect mode
//! import { succeed as __mf_exitSucceed, fail as __mf_exitFail } from "effect/Exit";
//! import type { Exit } from "effect/Exit";
//!
//! // Custom mode
//! import { ok as __mf_resultOk, err as __mf_resultErr } from "@rydshift/mirror/declarative";
//! ```
//!
//! This approach:
//! - Eliminates namespace collisions with user imports
//! - Enables better tree-shaking (only specific functions imported)
//! - Is explicit about what macroforge uses
//!
//! ## Usage
//!
//! These helpers are used by `derive_deserialize.rs` and `derive_partial_ord.rs`
//! to generate mode-appropriate code.

use super::serde::get_return_types_mode;
use crate::host::config::ReturnTypesMode;
use crate::ts_syn::ImportConfig;

// ============================================================================
// Import Configs for Return Types
// ============================================================================

/// Deserialize imports for Custom mode (@rydshift/mirror Result)
pub const CUSTOM_DESERIALIZE_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("ok", "__mf_resultOk", "macroforge/reexports"),
    ImportConfig::value("err", "__mf_resultErr", "macroforge/reexports"),
    ImportConfig::value("isOk", "__mf_resultIsOk", "macroforge/reexports"),
    ImportConfig::type_only("Result", "__mf_Result", "macroforge/reexports"),
];

/// Deserialize imports for Effect mode (effect Exit)
pub const EFFECT_DESERIALIZE_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("exitSucceed", "__mf_exitSucceed", "macroforge/reexports/effect"),
    ImportConfig::value("exitFail", "__mf_exitFail", "macroforge/reexports/effect"),
    ImportConfig::value("exitIsSuccess", "__mf_exitIsSuccess", "macroforge/reexports/effect"),
    ImportConfig::type_only("Exit", "__mf_Exit", "macroforge/reexports/effect"),
];

/// PartialOrd imports for Custom mode (@rydshift/mirror Option)
pub const CUSTOM_PARTIAL_ORD_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("some", "__mf_optionSome", "macroforge/reexports"),
    ImportConfig::value("none", "__mf_optionNone", "macroforge/reexports"),
    ImportConfig::value("isNone", "__mf_optionIsNone", "macroforge/reexports"),
    ImportConfig::type_only("Option", "__mf_Option", "macroforge/reexports"),
];

/// PartialOrd imports for Effect mode (effect Option)
pub const EFFECT_PARTIAL_ORD_IMPORTS: &[ImportConfig] = &[
    ImportConfig::value("optionSome", "__mf_optionSome", "macroforge/reexports/effect"),
    ImportConfig::value("optionNone", "__mf_optionNone", "macroforge/reexports/effect"),
    ImportConfig::value("optionIsNone", "__mf_optionIsNone", "macroforge/reexports/effect"),
    ImportConfig::type_only("Option", "__mf_Option", "macroforge/reexports/effect"),
];

impl ReturnTypesMode {
    /// Get the deserialize imports for this mode.
    ///
    /// Returns a static slice of [`ImportConfig`] for use with [`TsStream::add_imports`].
    pub const fn deserialize_imports(&self) -> &'static [ImportConfig] {
        match self {
            ReturnTypesMode::Vanilla => &[],
            ReturnTypesMode::Custom => CUSTOM_DESERIALIZE_IMPORTS,
            ReturnTypesMode::Effect => EFFECT_DESERIALIZE_IMPORTS,
        }
    }

    /// Get the partial_ord imports for this mode.
    ///
    /// Returns a static slice of [`ImportConfig`] for use with [`TsStream::add_imports`].
    pub const fn partial_ord_imports(&self) -> &'static [ImportConfig] {
        match self {
            ReturnTypesMode::Vanilla => &[],
            ReturnTypesMode::Custom => CUSTOM_PARTIAL_ORD_IMPORTS,
            ReturnTypesMode::Effect => EFFECT_PARTIAL_ORD_IMPORTS,
        }
    }
}

// ============================================================================
// Aliased Function Names
// ============================================================================

// Effect Exit functions
const EXIT_SUCCEED: &str = "__mf_exitSucceed";
const EXIT_FAIL: &str = "__mf_exitFail";
const EXIT_IS_SUCCESS: &str = "__mf_exitIsSuccess";

// Effect Option functions
const EFFECT_OPTION_SOME: &str = "__mf_optionSome";
const EFFECT_OPTION_NONE: &str = "__mf_optionNone";
const EFFECT_OPTION_IS_NONE: &str = "__mf_optionIsNone";

// Custom Result functions
const RESULT_OK: &str = "__mf_resultOk";
const RESULT_ERR: &str = "__mf_resultErr";
const RESULT_IS_OK: &str = "__mf_resultIsOk";

// Custom Option functions
const CUSTOM_OPTION_SOME: &str = "__mf_optionSome";
const CUSTOM_OPTION_NONE: &str = "__mf_optionNone";
const CUSTOM_OPTION_IS_NONE: &str = "__mf_optionIsNone";

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
/// - Custom: `__mf_Result<T, Array<{ field: string; message: string }>>`
/// - Effect: `__mf_Exit<Array<{ field: string; message: string }>, T>`
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
                "__mf_Result<{}, Array<{{ field: string; message: string }}>>",
                type_name
            )
        }
        ReturnTypesMode::Effect => {
            format!(
                "__mf_Exit<Array<{{ field: string; message: string }}>, {}>",
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
/// - Custom: `__mf_resultOk(<expr>)`
/// - Effect: `__mf_exitSucceed(<expr>)`
pub fn wrap_success(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => {
            format!("{{ success: true, value: {} }}", expr)
        }
        ReturnTypesMode::Custom => {
            format!("{}({})", RESULT_OK, expr)
        }
        ReturnTypesMode::Effect => {
            format!("{}({})", EXIT_SUCCEED, expr)
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
/// - Custom: `__mf_resultErr(<expr>)`
/// - Effect: `__mf_exitFail(<expr>)`
pub fn wrap_error(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => {
            format!("{{ success: false, errors: {} }}", expr)
        }
        ReturnTypesMode::Custom => {
            format!("{}({})", RESULT_ERR, expr)
        }
        ReturnTypesMode::Effect => {
            format!("{}({})", EXIT_FAIL, expr)
        }
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
/// - Custom: `__mf_resultIsOk(<expr>)`
/// - Effect: `__mf_exitIsSuccess(<expr>)`
pub fn is_ok_check(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => format!("{}.success", expr),
        ReturnTypesMode::Custom => format!("{}({})", RESULT_IS_OK, expr),
        ReturnTypesMode::Effect => format!("{}({})", EXIT_IS_SUCCESS, expr),
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
/// - Custom: `__mf_Option<number>`
/// - Effect: `__mf_Option<number>`
pub fn partial_ord_return_type() -> &'static str {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => "number | null",
        ReturnTypesMode::Custom => "__mf_Option<number>",
        ReturnTypesMode::Effect => "__mf_Option<number>",
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
/// - Custom: `__mf_optionSome(<expr>)`
/// - Effect: `__mf_optionSome(<expr>)`
pub fn wrap_some(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => expr.to_string(),
        ReturnTypesMode::Custom => format!("{}({})", CUSTOM_OPTION_SOME, expr),
        ReturnTypesMode::Effect => format!("{}({})", EFFECT_OPTION_SOME, expr),
    }
}

/// Returns the "none" expression for PartialOrd.
///
/// # Returns
///
/// The appropriate "none" value for the current mode:
/// - Vanilla: `null`
/// - Custom: `__mf_optionNone()`
/// - Effect: `__mf_optionNone()`
pub fn wrap_none() -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => "null".to_string(),
        ReturnTypesMode::Custom => format!("{}()", CUSTOM_OPTION_NONE),
        ReturnTypesMode::Effect => format!("{}()", EFFECT_OPTION_NONE),
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
/// - Custom: `__mf_optionIsNone(<expr>)`
/// - Effect: `__mf_optionIsNone(<expr>)`
pub fn is_none_check(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => format!("{} === null", expr),
        ReturnTypesMode::Custom => format!("{}({})", CUSTOM_OPTION_IS_NONE, expr),
        ReturnTypesMode::Effect => format!("{}({})", EFFECT_OPTION_IS_NONE, expr),
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
/// - Custom: `__mf_optionIsNone(<expr>) ? null : <expr>.value`
/// - Effect: `__mf_optionIsNone(<expr>) ? null : <expr>.value`
pub fn unwrap_option_or_null(expr: &str) -> String {
    match get_return_types_mode() {
        ReturnTypesMode::Vanilla => expr.to_string(),
        ReturnTypesMode::Custom => format!(
            "{}({}) ? null : {}.value",
            CUSTOM_OPTION_IS_NONE, expr, expr
        ),
        ReturnTypesMode::Effect => format!(
            "{}({}) ? null : {}.value",
            EFFECT_OPTION_IS_NONE, expr, expr
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin::serde::{clear_return_types_mode, set_return_types_mode};

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
        assert!(result.contains("Exit<"));
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
        assert_eq!(wrap_success("value"), "__mf_resultOk(value)");
        clear_return_types_mode();
    }

    #[test]
    fn test_effect_wrap_success() {
        set_return_types_mode(ReturnTypesMode::Effect);
        assert_eq!(wrap_success("value"), "__mf_exitSucceed(value)");
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
        assert_eq!(partial_ord_return_type(), "__mf_Option<number>");
        assert_eq!(wrap_some("0"), "__mf_optionSome(0)");
        assert_eq!(wrap_none(), "__mf_optionNone()");
        clear_return_types_mode();
    }

    #[test]
    fn test_effect_partial_ord() {
        set_return_types_mode(ReturnTypesMode::Effect);
        assert_eq!(partial_ord_return_type(), "__mf_Option<number>");
        assert_eq!(wrap_some("0"), "__mf_optionSome(0)");
        assert_eq!(wrap_none(), "__mf_optionNone()");
        clear_return_types_mode();
    }

    // ========================================================================
    // Aliased Import Tests
    // ========================================================================

    #[test]
    fn test_deserialize_imports_vanilla() {
        assert!(ReturnTypesMode::Vanilla.deserialize_imports().is_empty());
    }

    #[test]
    fn test_deserialize_imports_custom() {
        let imports = ReturnTypesMode::Custom.deserialize_imports();
        assert!(imports.iter().any(|i| i.alias == "__mf_resultOk"));
        assert!(imports.iter().any(|i| i.alias == "__mf_resultErr"));
        assert!(imports.iter().any(|i| i.alias == "__mf_Result"));
        assert!(imports.iter().any(|i| i.module == "macroforge/reexports"));
    }

    #[test]
    fn test_deserialize_imports_effect() {
        let imports = ReturnTypesMode::Effect.deserialize_imports();
        assert!(imports.iter().any(|i| i.alias == "__mf_exitSucceed"));
        assert!(imports.iter().any(|i| i.alias == "__mf_exitFail"));
        assert!(imports.iter().any(|i| i.alias == "__mf_Exit"));
        assert!(imports.iter().any(|i| i.module == "macroforge/reexports/effect"));
    }

    #[test]
    fn test_partial_ord_imports_effect() {
        let imports = ReturnTypesMode::Effect.partial_ord_imports();
        assert!(imports.iter().any(|i| i.alias == "__mf_optionSome"));
        assert!(imports.iter().any(|i| i.alias == "__mf_optionNone"));
        assert!(imports.iter().any(|i| i.alias == "__mf_Option"));
        assert!(imports.iter().any(|i| i.module == "macroforge/reexports/effect"));
    }

    #[test]
    fn test_is_ok_check_uses_aliased_function() {
        set_return_types_mode(ReturnTypesMode::Effect);
        assert_eq!(is_ok_check("result"), "__mf_exitIsSuccess(result)");
        clear_return_types_mode();

        set_return_types_mode(ReturnTypesMode::Custom);
        assert_eq!(is_ok_check("result"), "__mf_resultIsOk(result)");
        clear_return_types_mode();
    }

    #[test]
    fn test_is_none_check_uses_aliased_function() {
        set_return_types_mode(ReturnTypesMode::Effect);
        assert_eq!(is_none_check("opt"), "__mf_optionIsNone(opt)");
        clear_return_types_mode();

        set_return_types_mode(ReturnTypesMode::Custom);
        assert_eq!(is_none_check("opt"), "__mf_optionIsNone(opt)");
        clear_return_types_mode();
    }
}
