use std::collections::HashSet;

use anyhow::Context;

use super::DERIVE_MODULE_PATH;
use crate::host::derived::{self, DYNAMIC_MODULE_MARKER};
use crate::host::{MacroRegistry, package_registry};

/// Registers every linked macro package and derived macro, each once.
pub(super) fn register_packages(registry: &MacroRegistry) -> anyhow::Result<()> {
    let mut packages = HashSet::new();
    for package in package_registry::registrars() {
        if packages.insert(package.module) {
            (package.registrar)(registry)
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to register macro package {}", package.module))?;
        }
    }

    for module in derived::modules() {
        if module != DYNAMIC_MODULE_MARKER {
            derived::register_module(module, registry)
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to register derived macros of {module}"))?;
        }
    }

    // A dynamic macro resolves by name whatever path it is imported from, so
    // it is registered under the derive path that name-only lookups reach.
    for entry in inventory::iter::<derived::DerivedMacroRegistration> {
        let descriptor = entry.descriptor;
        if descriptor.module == DYNAMIC_MODULE_MARKER {
            registry
                .register(
                    DERIVE_MODULE_PATH,
                    descriptor.name,
                    (descriptor.constructor)(),
                )
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to register derive macro {}", descriptor.name))?;
        }
    }

    Ok(())
}
