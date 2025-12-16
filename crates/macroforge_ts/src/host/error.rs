//! # Error Handling for the Macro Host
//!
//! This module provides error types and a `Result` type alias for macro
//! operations. All errors in the macro host system flow through [`MacroError`].
//!
//! ## Error Categories
//!
//! - **Registry errors**: Macro not found, registration failures
//! - **Configuration errors**: Invalid config files, missing options
//! - **Execution errors**: Macro panics, invalid output
//! - **Compatibility errors**: ABI version mismatches
//! - **I/O errors**: File read/write failures
//! - **Serialization errors**: JSON parsing failures
//!
//! ## Error Handling Patterns
//!
//! ```rust,no_run
//! use macroforge_ts::host::{MacroExpander, MacroError, Result};
//!
//! fn process_file(code: &str) -> Result<String> {
//!     let expander = MacroExpander::new()?;
//!
//!     // expand_source returns Result<MacroExpansion, MacroError>
//!     match expander.expand_source(code, "file.ts") {
//!         Ok(expansion) => Ok(expansion.code),
//!         Err(MacroError::MacroNotFound { module, name }) => {
//!             eprintln!("Unknown macro: {}::{}", module, name);
//!             Err(MacroError::MacroNotFound { module, name })
//!         }
//!         Err(e) => Err(e),
//!     }
//! }
//! ```

use thiserror::Error;

/// A specialized `Result` type for macro operations.
///
/// This type alias is used throughout the macro host for functions
/// that can fail with a [`MacroError`].
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts::host::Result;
///
/// fn my_function() -> Result<String> {
///     // This can return any MacroError variant
///     Ok("success".to_string())
/// }
/// ```
pub type Result<T> = std::result::Result<T, MacroError>;

/// Errors that can occur during macro operations.
///
/// This enum covers all possible failure modes in the macro host system,
/// from registry lookups to macro execution to I/O operations.
///
/// # Error Recovery
///
/// Most errors are recoverable at the file level - a single file's failure
/// should not prevent other files from being processed. The macro expander
/// collects diagnostics for non-fatal errors and continues processing.
///
/// # Display
///
/// All variants implement `Display` through `thiserror`, providing
/// human-readable error messages suitable for end users.
#[derive(Debug, Error)]
pub enum MacroError {
    /// A macro was requested that doesn't exist in the registry.
    ///
    /// This occurs when:
    /// - `@derive(UnknownMacro)` references a non-existent macro
    /// - A macro package fails to register its macros
    /// - There's a typo in the macro name
    ///
    /// # Fields
    ///
    /// * `module` - The module that was searched (e.g., "builtin")
    /// * `name` - The macro name that wasn't found (e.g., "UnknownMacro")
    #[error("Macro '{module}::{name}' not found in registry")]
    MacroNotFound {
        /// The module name (or empty string for name-only lookups).
        module: String,
        /// The macro name that wasn't found.
        name: String,
    },

    /// The macro configuration is invalid or malformed.
    ///
    /// This occurs when:
    /// - `macroforge.json` contains invalid JSON
    /// - Required configuration options are missing
    /// - Configuration values are out of valid ranges
    ///
    /// # Fields
    ///
    /// The string contains a description of what's wrong with the configuration.
    #[error("Invalid macro configuration: {0}")]
    InvalidConfig(String),

    /// A macro failed during execution.
    ///
    /// This occurs when:
    /// - A macro panics (caught and converted to this error)
    /// - A macro returns invalid patches
    /// - A macro exceeds resource limits
    ///
    /// # Fields
    ///
    /// The string contains details about what went wrong.
    #[error("Macro execution failed: {0}")]
    ExecutionFailed(String),

    /// The macro's ABI version doesn't match what the host expects.
    ///
    /// This is a safety mechanism to prevent crashes from incompatible
    /// macro implementations. It occurs when:
    /// - Loading macros compiled against a different macroforge version
    /// - Using outdated macro packages
    ///
    /// # Fields
    ///
    /// * `expected` - The ABI version the host expects
    /// * `actual` - The ABI version the macro reported
    #[error("ABI version mismatch: expected {expected}, got {actual}")]
    AbiVersionMismatch {
        /// The ABI version expected by the host.
        expected: u32,
        /// The ABI version reported by the macro.
        actual: u32,
    },

    /// An I/O operation failed.
    ///
    /// This typically occurs when:
    /// - Reading configuration files
    /// - Writing output files
    /// - Loading external macro packages
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization or deserialization failed.
    ///
    /// This occurs when:
    /// - Parsing `macroforge.json` configuration
    /// - Serializing metadata output
    /// - Processing JSON in macro logic
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// A catch-all for other errors.
    ///
    /// Used for errors that don't fit into other categories.
    /// The wrapped `anyhow::Error` provides context and backtrace support.
    #[error("{0}")]
    Other(#[from] anyhow::Error),
}
