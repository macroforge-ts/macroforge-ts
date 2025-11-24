//! TypeScript Macro Host
//!
//! This crate provides the core macro hosting infrastructure for TypeScript macros.
//! It handles macro registration, dispatch, and execution.

pub mod registry;
pub mod traits;
pub mod dispatch;
pub mod config;
pub mod error;
pub mod builtin;

pub use registry::MacroRegistry;
pub use traits::TsMacro;
pub use dispatch::MacroDispatcher;
pub use config::MacroConfig;
pub use error::{MacroError, Result};

// Re-export commonly used types from ts_macro_abi
pub use ts_macro_abi::{MacroResult, Patch, Diagnostic, DiagnosticLevel, MacroKind};