//! # Core Macro Traits
//!
//! This module defines the fundamental traits that all macros must implement
//! to integrate with the Macroforge TypeScript macro system.
//!
//! ## Trait Hierarchy
//!
//! - [`Macroforge`] - The core trait for individual macro implementations
//! - `MacroPackage` - Groups multiple macros into a distributable package
//!
//! ## Implementing a Custom Macro
//!
//! To create a custom macro, implement the [`Macroforge`] trait:
//!
//! ```ignore
//! use macroforge_ts::host::Macroforge;
//! use macroforge_ts::ts_syn::{TsStream, abi::{MacroKind, MacroResult}};
//!
//! struct MyMacro;
//!
//! impl Macroforge for MyMacro {
//!     fn name(&self) -> &str { "MyMacro" }
//!     fn kind(&self) -> MacroKind { MacroKind::Derive }
//!
//!     fn run(&self, input: TsStream) -> MacroResult {
//!         // Process the input and generate patches
//!         MacroResult::default()
//!     }
//!
//!     fn description(&self) -> &str {
//!         "My custom macro that does something useful"
//!     }
//! }
//! ```
//!
//! ## ABI Versioning
//!
//! The ABI version ensures compatibility between macros and the host.
//! If a macro's ABI version doesn't match the host's expected version,
//! execution will be refused to prevent crashes or undefined behavior.

use crate::ts_syn::TsStream;
use crate::ts_syn::abi::{MacroKind, MacroResult};

/// The core trait that all TypeScript macros must implement.
///
/// This trait defines the contract between a macro implementation and the
/// macro host. Macros receive a [`TsStream`] containing the decorated item
/// (class, interface, etc.) and return a [`MacroResult`] with patches and
/// diagnostics.
///
/// # Thread Safety
///
/// Macros must be `Send + Sync` because:
/// - Multiple files may be processed concurrently
/// - The macro registry is shared across threads
/// - Macro instances are stored in a `DashMap`
///
/// # Example
///
/// ```ignore
/// use macroforge_ts::host::Macroforge;
/// use macroforge_ts::ts_syn::{TsStream, abi::{MacroKind, MacroResult, Patch}};
///
/// struct GreetMacro;
///
/// impl Macroforge for GreetMacro {
///     fn name(&self) -> &str { "Greet" }
///     fn kind(&self) -> MacroKind { MacroKind::Derive }
///
///     fn run(&self, input: TsStream) -> MacroResult {
///         let class_name = input.class_name().unwrap_or("Unknown");
///         let method = format!(
///             "greet(): string {{ return \"Hello from {}!\"; }}",
///             class_name
///         );
///
///         MacroResult {
///             patches: vec![Patch::insert_method(method)],
///             diagnostics: vec![],
///         }
///     }
/// }
/// ```
pub trait Macroforge: Send + Sync {
    /// Returns the name of this macro.
    ///
    /// The name is used for:
    /// - Registration in the macro registry (e.g., "Debug", "Clone")
    /// - Matching against `@derive(Name)` decorators
    /// - Error messages and diagnostics
    ///
    /// # Convention
    ///
    /// Use PascalCase for macro names (e.g., "Debug", "PartialEq", "Serialize").
    fn name(&self) -> &str;

    /// Returns the kind of this macro.
    ///
    /// The macro kind determines how the macro is invoked:
    ///
    /// - `Derive` - Applied via `@derive(MacroName)` to classes/interfaces
    /// - `Attribute` - Applied as `@macroName(args)` with custom arguments
    /// - `Function` - Called as a function-like macro
    fn kind(&self) -> MacroKind;

    /// Executes the macro with the given input stream.
    ///
    /// This is the main entry point for macro execution. The macro receives
    /// a [`TsStream`] containing the decorated item and should return a
    /// [`MacroResult`] with any generated patches and diagnostics.
    ///
    /// # Arguments
    ///
    /// * `input` - A [`TsStream`] providing access to the decorated class,
    ///   interface, or enum, along with field information and decorator options.
    ///
    /// # Returns
    ///
    /// A [`MacroResult`] containing:
    /// - `patches` - Code modifications to apply (insert methods, replace code, etc.)
    /// - `diagnostics` - Warnings, errors, or informational messages
    ///
    /// # Panics
    ///
    /// Macro implementations should avoid panicking. Instead, return errors
    /// via `MacroResult::diagnostics` with appropriate error levels. Panics
    /// are caught by the host but result in poor error messages.
    fn run(&self, input: TsStream) -> MacroResult;

    /// Returns a human-readable description of what this macro does.
    ///
    /// Used for:
    /// - Documentation generation
    /// - IDE hover information
    /// - The macro manifest
    ///
    /// # Default
    ///
    /// Returns "A TypeScript macro" if not overridden.
    fn description(&self) -> &str {
        "A TypeScript macro"
    }

    /// Returns the ABI version this macro was compiled against.
    ///
    /// The ABI version is used for compatibility checking. If a macro's
    /// ABI version doesn't match the host's expected version, the macro
    /// will not be executed to prevent crashes or undefined behavior.
    ///
    /// # Versioning Rules
    ///
    /// - Increment when the `TsStream` or `MacroResult` structures change
    /// - Increment when the patch format changes
    /// - Macros with mismatched versions are rejected with `MacroError::AbiVersionMismatch`
    ///
    /// # Default
    ///
    /// Returns `1` (the current stable ABI version).
    fn abi_version(&self) -> u32 {
        1
    }
}

/// Trait for macro packages that provide multiple macros as a unit.
///
/// A macro package groups related macros together for distribution and
/// registration. This is useful for:
///
/// - Distributing a set of related macros (e.g., all serde-related macros)
/// - Versioning a collection of macros together
/// - Loading external macro packages dynamically
///
/// # Example
///
/// ```ignore
/// use macroforge_ts::host::{MacroPackage, Macroforge};
///
/// struct MyPackage;
///
/// impl MacroPackage for MyPackage {
///     fn package_name(&self) -> &str { "my-macros" }
///
///     fn macros(&self) -> Vec<Box<dyn Macroforge>> {
///         vec![
///             Box::new(MacroA),
///             Box::new(MacroB),
///             Box::new(MacroC),
///         ]
///     }
///
///     fn version(&self) -> &str { "1.0.0" }
/// }
/// ```
pub trait MacroPackage: Send + Sync {
    /// Returns the package name.
    ///
    /// Used for:
    /// - Identifying the package in configuration
    /// - Error messages
    /// - The macro manifest
    fn package_name(&self) -> &str;

    /// Returns all macros provided by this package.
    ///
    /// Each macro in the returned vector will be registered in the
    /// macro registry with its name as the key.
    ///
    /// # Note
    ///
    /// This method returns owned boxed macros. Each call creates new
    /// instances, so avoid calling this repeatedly in hot paths.
    fn macros(&self) -> Vec<Box<dyn Macroforge>>;

    /// Returns the package version.
    ///
    /// Used for:
    /// - Compatibility checking
    /// - Debugging and logging
    /// - The macro manifest
    ///
    /// # Default
    ///
    /// Returns "0.1.0" if not overridden.
    fn version(&self) -> &str {
        "0.1.0"
    }
}
