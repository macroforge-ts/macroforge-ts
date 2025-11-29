//! TypeScript Macro Host
//!
//! This crate provides the core macro hosting infrastructure for TypeScript macros.
//! It handles macro registration, dispatch, and execution.

pub mod config;
pub mod derived;
pub mod dispatch;
pub mod error;
pub mod expand;
pub mod macros;
pub mod package_registry;
pub mod patch_applicator;
pub mod registry;
pub mod traits;

pub use config::MacroConfig;
pub use dispatch::MacroDispatcher;
pub use error::{MacroError, Result};
pub use expand::{MacroExpander, MacroExpansion};
pub use package_registry::MacroPackageRegistration;
pub use patch_applicator::{PatchApplicator, PatchCollector};
pub use registry::MacroRegistry;
pub use traits::TsMacro;

// Re-export commonly used types from ts_macro_abi
pub use ts_macro_abi::{Diagnostic, DiagnosticLevel, MacroKind, MacroResult, Patch};
