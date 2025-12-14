//! # Global Registry for Macro Package Registrars
//!
//! This module provides compile-time registration of macro package initializers
//! using the `inventory` crate. It enables automatic discovery and registration
//! of macro packages without explicit initialization code.
//!
//! ## How It Works
//!
//! 1. Packages use the `register_macro_package!` macro to declare a registrar
//! 2. The `inventory` crate collects all registrations at link time
//! 3. At runtime, `registrars()` returns all collected registrations
//! 4. Each registrar function is called to populate the macro registry
//!
//! ## Example
//!
//! ```ignore
//! // In your macro package:
//! use macroforge_ts::register_macro_package;
//!
//! fn register_my_macros(registry: &MacroRegistry) -> Result<()> {
//!     registry.register("my-module", "MyMacro", Arc::new(MyMacro))?;
//!     Ok(())
//! }
//!
//! register_macro_package!("my-module", register_my_macros);
//! ```
//!
//! ## Relationship to Other Registration Systems
//!
//! This module works alongside the `derived` module:
//! - `derived` module: Individual macro registration via `DerivedMacroRegistration`
//! - This module: Bulk registration via package registrar functions
//!
//! Most built-in macros use the `derived` system. External packages may prefer
//! the registrar pattern for more control over initialization.

use super::{MacroRegistry, Result};

/// A registration entry for a macro package registrar.
///
/// This struct holds a module name and a function that registers all
/// macros from that module into a [`MacroRegistry`].
///
/// # Fields
///
/// * `module` - The module identifier for this package
/// * `registrar` - A function that registers macros into the given registry
pub struct MacroPackageRegistration {
    /// The module identifier for this package (e.g., "my-macros").
    pub module: &'static str,
    /// Function that registers all macros from this package.
    /// Called during initialization to populate the registry.
    pub registrar: fn(&MacroRegistry) -> Result<()>,
}

// Tell `inventory` to collect all MacroPackageRegistration instances at link time.
inventory::collect!(MacroPackageRegistration);

/// Returns all registered macro package registrars.
///
/// This function iterates through all [`MacroPackageRegistration`] entries
/// collected by the `inventory` crate at link time.
///
/// # Returns
///
/// A vector of references to all registered package registrations.
///
/// # Usage
///
/// ```ignore
/// for registration in registrars() {
///     (registration.registrar)(registry)?;
/// }
/// ```
pub fn registrars() -> Vec<&'static MacroPackageRegistration> {
    inventory::iter::<MacroPackageRegistration>
        .into_iter()
        .collect()
}
