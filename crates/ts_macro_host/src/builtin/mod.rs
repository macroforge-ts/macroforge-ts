//! Built-in macros provided by the framework

pub mod derive_clone;
pub mod derive_debug;
pub mod derive_eq;

use crate::{MacroRegistry, Result};
use std::sync::Arc;

/// Register all built-in macros with the registry
pub fn register_builtin_macros(registry: &MacroRegistry) -> Result<()> {
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
