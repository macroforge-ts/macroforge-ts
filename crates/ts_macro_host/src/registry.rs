//! Macro registry for managing and looking up macros

use crate::{MacroError, TsMacro, error::Result};
use dashmap::DashMap;
use std::sync::Arc;

/// Key for identifying a macro by module and name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroKey {
    /// The module/package the macro comes from (e.g., "@macro/derive")
    pub module: String,
    /// The name of the macro (e.g., "Debug")
    pub name: String,
}

impl MacroKey {
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
        }
    }
}

/// Registry for all available macros
pub struct MacroRegistry {
    /// Map from (module, name) to macro implementation
    macros: DashMap<MacroKey, Arc<dyn TsMacro>>,
}

impl MacroRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            macros: DashMap::new(),
        }
    }

    /// Register a macro in the registry
    pub fn register(
        &self,
        module: impl Into<String>,
        name: impl Into<String>,
        macro_impl: Arc<dyn TsMacro>,
    ) -> Result<()> {
        let key = MacroKey::new(module, name);

        // Check for duplicates
        if self.macros.contains_key(&key) {
            return Err(MacroError::InvalidConfig(format!(
                "Macro '{}::{}' is already registered",
                key.module, key.name
            )));
        }

        self.macros.insert(key, macro_impl);
        Ok(())
    }

    /// Look up a macro by module and name
    pub fn lookup(&self, module: &str, name: &str) -> Result<Arc<dyn TsMacro>> {
        let key = MacroKey::new(module, name);

        self.macros
            .get(&key)
            .map(|entry| Arc::clone(&entry))
            .ok_or_else(|| MacroError::MacroNotFound {
                module: module.to_string(),
                name: name.to_string(),
            })
    }

    /// Get all registered macros
    pub fn all_macros(&self) -> Vec<(MacroKey, Arc<dyn TsMacro>)> {
        self.macros
            .iter()
            .map(|entry| (entry.key().clone(), Arc::clone(entry.value())))
            .collect()
    }

    /// Check if a macro is registered
    pub fn contains(&self, module: &str, name: &str) -> bool {
        let key = MacroKey::new(module, name);
        self.macros.contains_key(&key)
    }

    /// Clear all registered macros
    pub fn clear(&self) {
        self.macros.clear();
    }
}

impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}
