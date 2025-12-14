//! # Helper Macros for Macro Registration
//!
//! This module provides declarative macros to simplify the registration of
//! macro packages with the Macroforge system.
//!
//! ## Usage
//!
//! The `register_macro_package!` macro is the primary tool for registering
//! a macro package. It works with the `inventory` crate to collect registrations
//! at link time.
//!
//! ```ignore
//! use macroforge_ts::register_macro_package;
//!
//! fn my_registrar(registry: &MacroRegistry) -> Result<()> {
//!     registry.register("my-module", "MacroA", Arc::new(MacroA))?;
//!     registry.register("my-module", "MacroB", Arc::new(MacroB))?;
//!     Ok(())
//! }
//!
//! register_macro_package!("my-module", my_registrar);
//! ```
//!
//! After compilation, your registrar will be automatically discovered and
//! called during macro host initialization.

/// Registers a macro package with the global package registry.
///
/// This macro creates an `inventory` entry that associates a module name
/// with a registrar function. The registrar will be automatically called
/// during macro host initialization to populate the macro registry.
///
/// # Arguments
///
/// * `$module` - A string literal identifying the module (e.g., `"my-macros"`)
/// * `$registrar` - Path to a function with signature `fn(&MacroRegistry) -> Result<()>`
///
/// # Example
///
/// ```ignore
/// use macroforge_ts::register_macro_package;
/// use macroforge_ts::host::{MacroRegistry, Result};
/// use std::sync::Arc;
///
/// struct MyMacro;
/// // ... implement Macroforge for MyMacro ...
///
/// fn register_my_macros(registry: &MacroRegistry) -> Result<()> {
///     registry.register("my-module", "MyMacro", Arc::new(MyMacro))?;
///     Ok(())
/// }
///
/// register_macro_package!("my-module", register_my_macros);
/// ```
///
/// # How It Works
///
/// The macro expands to an `inventory::submit!` call that creates a
/// [`MacroPackageRegistration`](crate::host::package_registry::MacroPackageRegistration)
/// entry. The `inventory` crate collects these entries at link time,
/// making them discoverable via
/// [`registrars()`](crate::host::package_registry::registrars).
#[macro_export]
macro_rules! register_macro_package {
    ($module:expr, $registrar:path) => {
        inventory::submit! {
            $crate::host::package_registry::MacroPackageRegistration {
                module: $module,
                registrar: $registrar,
            }
        }
    };
}
