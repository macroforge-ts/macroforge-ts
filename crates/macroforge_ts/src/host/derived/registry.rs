use super::descriptors::{
    DYNAMIC_MODULE_MARKER, DecoratorMetadata, DerivedMacroDescriptor, DerivedMacroRegistration,
};
use crate::host::{MacroRegistry, Result};
use std::collections::BTreeSet;

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

/// Returns all unique decorator module names (package names).
///
/// Note: this returns the `module` field of `DecoratorDescriptor` which is the
/// npm package name. For the actual annotation keywords used in JSDoc (e.g.,
/// `"serde"`, `"debug"`, `"hash"`), use [`decorator_annotation_names`].
pub fn decorator_modules() -> BTreeSet<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .flat_map(|entry| entry.descriptor.decorators)
        .map(|decorator| decorator.module)
        .collect()
}

/// Returns all unique decorator annotation names.
///
/// These are the keywords used in field-level decorators like `@serde({ ... })`,
/// `@debug(skip)`, `@default(...)`, etc. Used by annotation filtering to only
/// recognize valid macroforge annotations during lowering.
///
/// # Returns
///
/// A set of annotation names (e.g., `{"serde", "debug", "hash", "default", "ord"}`).
pub fn decorator_annotation_names() -> BTreeSet<&'static str> {
    inventory::iter::<DerivedMacroRegistration>
        .into_iter()
        .flat_map(|entry| entry.descriptor.decorators)
        .map(|decorator| decorator.export)
        .collect()
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
