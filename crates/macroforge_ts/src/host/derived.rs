//! # Derived Macro Registration System
//!
//! This module provides compile-time macro registration using the `inventory` crate.
//! It enables macros to be automatically discovered and registered without explicit
//! runtime registration code.
//!
//! ## How It Works
//!
//! 1. Macro implementations use the `#[derive_macro]` attribute (from macroforge_ts_macros)
//! 2. The attribute generates a `DerivedMacroRegistration` entry with static metadata
//! 3. The `inventory` crate collects all entries at link time
//! 4. At runtime, `register_module()` iterates entries and registers macros
//!
//! ## Dynamic Module Resolution
//!
//! Macros registered with `DYNAMIC_MODULE_MARKER` as their module path will be
//! resolved by name alone, regardless of the import path in user code. This enables
//! flexible import patterns:
//!
//! ```typescript
//! // All of these work with dynamic resolution:
//! import { Debug } from "macroforge-ts";
//! import { Debug } from "@my-org/macros";
//! import { Debug } from "./local-macros";
//! ```
//!
//! ## Manifest Generation
//!
//! The module provides `get_manifest()` for tooling to discover available macros
//! and their decorators at runtime. This is used by:
//! - IDE extensions for autocompletion
//! - Documentation generators
//! - The `__macroforgeGetManifest()` NAPI export

use super::{MacroRegistry, Macroforge, Result};
use crate::ts_syn::abi::MacroKind;
use serde::Serialize;
use std::{collections::BTreeSet, sync::Arc};

/// Special marker module path that indicates dynamic resolution.
///
/// When a macro is registered with this module path, the host will accept
/// any import path and resolve the macro by name alone, enabling flexible
/// import patterns without requiring exact module path matches.
pub const DYNAMIC_MODULE_MARKER: &str = "__DYNAMIC_MODULE__";

/// Static descriptor for a derive macro, containing all metadata needed for registration.
///
/// This struct holds compile-time information about a macro that was registered
/// using the `#[derive_macro]` attribute. The `inventory` crate collects all
/// descriptors for automatic registration.
///
/// # Fields
///
/// * `package` - The npm package name (e.g., "macroforge-ts")
/// * `module` - The module path for registration (e.g., "@macro/derive")
/// * `runtime` - Runtime dependencies this macro requires
/// * `name` - The macro name used in `@derive(Name)`
/// * `kind` - The macro kind (Derive, Attribute, Function)
/// * `description` - Human-readable description
/// * `constructor` - Factory function to create the macro instance
/// * `decorators` - Field/class decorators this macro provides
pub struct DerivedMacroDescriptor {
    /// The npm package providing this macro.
    pub package: &'static str,
    /// Module path for registry lookup (e.g., "@macro/derive", or `DYNAMIC_MODULE_MARKER`).
    pub module: &'static str,
    /// Runtime dependencies (currently unused, reserved for future use).
    pub runtime: &'static [&'static str],
    /// Macro name as used in `@derive(Name)`.
    pub name: &'static str,
    /// The kind of macro (Derive, Attribute, or Function).
    pub kind: MacroKind,
    /// Human-readable description for documentation and IDE hints.
    pub description: &'static str,
    /// Factory function that creates a new instance of the macro.
    pub constructor: fn() -> Arc<dyn Macroforge>,
    /// Field-level decorators provided by this macro.
    pub decorators: &'static [DecoratorDescriptor],
}

/// Descriptor for a field or class decorator provided by a macro.
///
/// Decorators are additional annotations that modify how a macro processes
/// a field or class. For example, `@serde(skip)` tells the Serialize macro
/// to skip a particular field.
pub struct DecoratorDescriptor {
    /// Module this decorator belongs to (e.g., "serde").
    pub module: &'static str,
    /// The exported decorator name (e.g., "skip", "rename").
    pub export: &'static str,
    /// What the decorator can be applied to (class, property, method, etc.).
    pub kind: DecoratorKind,
    /// Documentation string for the decorator.
    pub docs: &'static str,
}

/// The target a decorator can be applied to.
///
/// Used for TypeScript type generation and validation.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DecoratorKind {
    /// Applied to class declarations.
    Class,
    /// Applied to class properties/fields.
    Property,
    /// Applied to class methods.
    Method,
    /// Applied to getters/setters.
    Accessor,
    /// Applied to method/constructor parameters.
    Parameter,
}

impl DecoratorKind {
    /// Returns the TypeScript decorator type for this kind.
    ///
    /// Used when generating `.d.ts` type declarations.
    pub fn ts_type(&self) -> &'static str {
        match self {
            DecoratorKind::Class => "ClassDecorator",
            DecoratorKind::Property => "PropertyDecorator",
            DecoratorKind::Method => "MethodDecorator",
            DecoratorKind::Accessor => "MethodDecorator",
            DecoratorKind::Parameter => "ParameterDecorator",
        }
    }
}

/// A registration entry for the `inventory` crate.
///
/// This is the type that gets collected at link time. Each macro implementation
/// creates one of these via the `#[derive_macro]` attribute.
pub struct DerivedMacroRegistration {
    /// Reference to the static descriptor for this macro.
    pub descriptor: &'static DerivedMacroDescriptor,
}

// Tell `inventory` to collect all DerivedMacroRegistration instances at link time.
inventory::collect!(DerivedMacroRegistration);

/// Returns all unique module names that have registered macros.
///
/// Used to discover which modules are available for registration.
///
/// # Returns
///
/// A sorted set of module names (e.g., `{"@macro/derive", "serde"}`).
pub fn modules() -> BTreeSet<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| entry.descriptor.module)
        .collect()
}

/// Registers all macros from a specific module into the registry.
///
/// This function iterates through all collected `DerivedMacroRegistration`
/// entries, filters by module, and registers each macro.
///
/// # Arguments
///
/// * `module` - The module to register (e.g., "@macro/derive")
/// * `registry` - The registry to register macros into
///
/// # Returns
///
/// `Ok(true)` if macros were registered, `Ok(false)` if no macros found
/// for that module.
///
/// # Errors
///
/// Returns an error if a macro with the same name is already registered.
pub fn register_module(module: &str, registry: &MacroRegistry) -> Result<bool> {
    let descriptors: Vec<&DerivedMacroDescriptor> = inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .filter(|entry| entry.descriptor.module == module)
        .map(|entry| entry.descriptor)
        .collect();

    if descriptors.is_empty() {
        return Ok(false);
    }

    // Collect runtime entries from all packages (reserved for future use)
    let mut runtime: BTreeSet<String> = BTreeSet::new();
    for descriptor in &descriptors {
        for entry in descriptor.runtime {
            runtime.insert(entry.to_string());
        }
    }

    // Register all macros - the registry will catch duplicate names
    for descriptor in descriptors {
        registry.register(module, descriptor.name, (descriptor.constructor)())?;
    }

    Ok(true)
}

/// Metadata about a decorator for serialization and tooling.
#[derive(Debug, Clone, Serialize)]
pub struct DecoratorMetadata {
    /// Module this decorator belongs to.
    pub module: &'static str,
    /// The exported decorator name.
    pub export: &'static str,
    /// What the decorator can be applied to.
    pub kind: DecoratorKind,
    /// Documentation string.
    pub docs: &'static str,
}

/// Returns metadata for all registered decorators.
///
/// Used by manifest generation and tooling.
pub fn decorator_metadata() -> Vec<DecoratorMetadata> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .flat_map(|entry| entry.descriptor.decorators)
        .map(|decorator| DecoratorMetadata {
            module: decorator.module,
            export: decorator.export,
            kind: decorator.kind,
            docs: decorator.docs,
        })
        .collect()
}

/// Returns all unique decorator module names.
///
/// These are the keywords used in field-level decorators like `@serde({ ... })`.
/// Used by `strip_decorators` to identify Macroforge-specific decorators.
///
/// # Returns
///
/// A set of module names (e.g., `{"serde", "debug", "hash"}`).
pub fn decorator_modules() -> BTreeSet<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .flat_map(|entry| entry.descriptor.decorators)
        .map(|decorator| decorator.module)
        .collect()
}

/// Manifest entry describing a single macro.
///
/// Used in [`MacroManifest`] for tooling and documentation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifestEntry {
    /// The macro name (e.g., "Debug", "Clone").
    pub name: &'static str,
    /// The macro kind (Derive, Attribute, Function).
    pub kind: MacroKind,
    /// Human-readable description.
    pub description: &'static str,
    /// The package providing this macro.
    pub package: &'static str,
}

/// Complete manifest of all available macros and decorators.
///
/// This struct is returned by [`get_manifest()`] and contains everything
/// tooling needs to understand what macros are available.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroManifest {
    /// Manifest format version (currently always 1).
    pub version: u32,
    /// All registered macros.
    pub macros: Vec<MacroManifestEntry>,
    /// All registered decorators.
    pub decorators: Vec<DecoratorMetadata>,
}

/// Returns the complete manifest of all registered macros and decorators.
///
/// This is the primary introspection API for tooling to discover available
/// macros at runtime.
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts::host::derived::get_manifest;
///
/// let manifest = get_manifest();
/// println!("Available macros:");
/// for macro_entry in &manifest.macros {
///     println!("  {} - {}", macro_entry.name, macro_entry.description);
/// }
/// ```
pub fn get_manifest() -> MacroManifest {
    let macros: Vec<MacroManifestEntry> = inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| MacroManifestEntry {
            name: entry.descriptor.name,
            kind: entry.descriptor.kind,
            description: entry.descriptor.description,
            package: entry.descriptor.package,
        })
        .collect();

    let decorators = decorator_metadata();

    MacroManifest {
        version: 1,
        macros,
        decorators,
    }
}

/// Returns all macro names registered in this binary.
///
/// # Returns
///
/// A vector of macro names (e.g., `["Debug", "Clone", "Serialize"]`).
pub fn macro_names() -> Vec<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .map(|entry| entry.descriptor.name)
        .collect()
}

/// Looks up a macro by name only, ignoring the module path.
///
/// This is used for dynamic module resolution when the import path
/// doesn't matter.
///
/// # Arguments
///
/// * `name` - The macro name to find (e.g., "Debug")
///
/// # Returns
///
/// `Some(&DerivedMacroDescriptor)` if found, `None` otherwise.
pub fn lookup_by_name(name: &str) -> Option<&'static DerivedMacroDescriptor> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .find(|entry| entry.descriptor.name == name)
        .map(|entry| entry.descriptor)
}

/// Registers all macros with dynamic module support.
///
/// For macros registered with `DYNAMIC_MODULE_MARKER`, uses the provided
/// `actual_module` path instead. This enables a single set of macros to
/// work with any import path.
///
/// # Arguments
///
/// * `actual_module` - The module path to use for dynamic macros
/// * `registry` - The registry to register into
///
/// # Returns
///
/// The number of macros registered.
///
/// # Errors
///
/// Returns an error if a macro with the same name is already registered.
pub fn register_all_with_module(actual_module: &str, registry: &MacroRegistry) -> Result<usize> {
    let mut count = 0;

    for entry in inventory::iter::<DerivedMacroRegistration> {
        let descriptor = entry.descriptor;

        // Use the actual module path for dynamic macros,
        // otherwise use the descriptor's module
        let module = if descriptor.module == DYNAMIC_MODULE_MARKER {
            actual_module
        } else {
            descriptor.module
        };

        // Register the macro
        registry.register(module, descriptor.name, (descriptor.constructor)())?;
        count += 1;
    }

    Ok(count)
}
