//! Built-in macros provided by the framework

pub mod derive_clone;
pub mod derive_debug;
pub mod derive_eq;

use crate::{MacroRegistry, Result, registry::MacroManifest};
use std::{path::PathBuf, sync::Arc};

/// Register all built-in macros with the registry
pub fn register_builtin_macros(registry: &MacroRegistry) -> Result<()> {
    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("macro.toml");
    let manifest = MacroManifest::from_toml_file(&manifest_path)?;
    registry.register_manifest("@macro/derive", manifest)?;

    // Register @Derive(Debug)
    registry.register(
        "@macro/derive",
        "Debug",
        Arc::new(derive_debug::DeriveDebugMacro),
    )?;

    // Register @Derive(Clone)
    registry.register(
        "@macro/derive",
        "Clone",
        Arc::new(derive_clone::DeriveCloneMacro),
    )?;

    // Register @Derive(Eq)
    registry.register("@macro/derive", "Eq", Arc::new(derive_eq::DeriveEqMacro))?;

    Ok(())
}
