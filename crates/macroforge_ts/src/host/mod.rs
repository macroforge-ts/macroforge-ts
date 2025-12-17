//! # TypeScript Macro Host
//!
//! The macro host module provides the core infrastructure for TypeScript macro
//! expansion. It handles the complete lifecycle of macro processing: registration,
//! dispatch, execution, and patch application.
//!
//! ## Architecture Overview
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                      MacroExpander                               │
//! │  (Main entry point - coordinates the expansion process)         │
//! └─────────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                     MacroDispatcher                              │
//! │  (Routes macro calls to implementations with ABI checking)       │
//! └─────────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                      MacroRegistry                               │
//! │  (Thread-safe storage of registered macros using DashMap)        │
//! └─────────────────────────────────────────────────────────────────┘
//!                              │
//!                              ▼
//! ┌─────────────────────────────────────────────────────────────────┐
//! │                    PatchApplicator                               │
//! │  (Applies generated patches with source mapping)                 │
//! └─────────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Module Organization
//!
//! - [`config`] - Configuration loading and management (`macroforge.json`)
//! - [`derived`] - Inventory-based registration for built-in derive macros
//! - [`dispatch`] - Macro call routing and ABI version checking
//! - [`error`] - Error types (`MacroError`) and `Result` type alias
//! - [`expand`] - The main `MacroExpander` that orchestrates expansion
//! - [`macros`] - Helper macros for macro registration
//! - [`package_registry`] - Global registry for macro package registrars
//! - [`patch_applicator`] - Applies code patches with source mapping
//! - [`registry`] - Thread-safe macro storage (`MacroRegistry`)
//! - [`traits`] - Core traits (`Macroforge`, `MacroPackage`)
//!
//! ## Key Types
//!
//! - [`MacroExpander`] - Main entry point for macro expansion
//! - [`MacroDispatcher`] - Routes macros to implementations
//! - [`MacroRegistry`] - Stores registered macros
//! - [`Macroforge`] - Trait that all macros must implement
//! - [`MacroConfig`] - Configuration options
//! - [`MacroError`] - Error type for macro operations
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use macroforge_ts::host::{MacroExpander, Result};
//!
//! fn example() -> Result<()> {
//!     // Create a new expander (registers all built-in macros)
//!     let expander = MacroExpander::new()?;
//!
//!     // Parse TypeScript source code
//!     let source = r#"
//!         /** @derive(Debug) */
//!         class User { name: string; }
//!     "#;
//!
//!     // Expand macros (handles parsing internally)
//!     let result = expander.expand_source(source, "file.ts")?;
//!     println!("Expanded: {}", result.code);
//!     Ok(())
//! }
//! ```

/// Configuration loading and management.
pub mod config;

/// Inventory-based registration for built-in derive macros.
pub mod derived;

/// Macro call dispatching with ABI version checking.
pub mod dispatch;

/// Error types for macro operations.
pub mod error;

/// The main macro expansion engine.
pub mod expand;

/// Helper macros for macro registration.
pub mod macros;

/// Global registry for macro package registrars.
pub mod package_registry;

/// Patch application with source mapping.
pub mod patch_applicator;

/// Thread-safe macro storage.
pub mod registry;

/// Core traits for macro implementations.
pub mod traits;

// Primary exports for convenience
pub use config::{
    ForeignTypeConfig, ImportInfo, MacroConfig, MacroforgeConfig, CONFIG_CACHE,
};
pub use dispatch::MacroDispatcher;
pub use error::{MacroError, Result};
pub use expand::{MacroExpander, MacroExpansion, collect_import_sources};
pub use package_registry::MacroPackageRegistration;
pub use patch_applicator::{PatchApplicator, PatchCollector};
pub use registry::MacroRegistry;
pub use traits::Macroforge;

// Re-export commonly used types from the ABI module for convenience.
// These types are needed when implementing custom macros.
pub use crate::ts_syn::abi::{Diagnostic, DiagnosticLevel, MacroKind, MacroResult, Patch};
