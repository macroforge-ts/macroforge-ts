use std::collections::HashMap;
use std::path::Path;

use super::{DERIVE_MODULE_PATH, DYNAMIC_MODULE_MARKER};
use crate::host::{MacroConfig, MacroRegistry, Result, derived};

pub(super) type PackageRegistrar = fn(&MacroRegistry) -> Result<()>;

pub(super) fn available_package_registrars() -> Vec<(&'static str, PackageRegistrar)> {
    vec![]
}

pub(super) fn register_packages(
    registry: &MacroRegistry,
    config: &MacroConfig,
    _config_root: &Path,
) -> anyhow::Result<()> {
    use anyhow::Context;

    let mut embedded_map: HashMap<&'static str, PackageRegistrar> =
        available_package_registrars().into_iter().collect();
    for pkg in super::super::package_registry::registrars() {
        embedded_map.entry(pkg.module).or_insert(pkg.registrar);
    }

    let derived_modules = derived::modules();
    let derived_set: std::collections::HashSet<&'static str> =
        derived_modules.iter().copied().collect();

    let mut requested = if config.macro_packages.is_empty() {
        embedded_map.keys().cloned().collect::<Vec<_>>()
    } else {
        config
            .macro_packages
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    };

    if config.macro_packages.is_empty() {
        requested.extend(derived_modules.iter().copied());
    }

    requested.sort();
    requested.dedup();

    for module in requested {
        let mut found = false;

        if let Some(registrar) = embedded_map.get(module) {
            registrar(registry)
                .map_err(anyhow::Error::from)
                .with_context(|| format!("failed to register macro package {module}"))?;
            found = true;
        }

        if derived_set.contains(module) {
            derived::register_module(module, registry)?;
            found = true;
        }

        if !found {
            // Module not found - this is a warning, not an error
        }
    }

    if derived_set.contains(DYNAMIC_MODULE_MARKER) {
        let _ = derived::register_module(DYNAMIC_MODULE_MARKER, registry);
    }

    for entry in inventory::iter::<derived::DerivedMacroRegistration> {
        let descriptor = entry.descriptor;
        if descriptor.module == DYNAMIC_MODULE_MARKER {
            let _ = registry.register(
                DERIVE_MODULE_PATH,
                descriptor.name,
                (descriptor.constructor)(),
            );
        }
    }

    Ok(())
}
