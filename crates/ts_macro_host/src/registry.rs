//! Macro registry for managing and looking up macros

use crate::{MacroError, TsMacro, error::Result};
use dashmap::DashMap;
use std::{fs, path::Path, sync::Arc};

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
    /// Manifest data for each registered package
    manifests: DashMap<String, MacroManifest>,
}

/// Manifest data for a macro package
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MacroManifest {
    /// Optional module name this manifest represents (e.g., "@macro/derive")
    #[serde(default)]
    pub module: Option<String>,
    /// Optional native library configuration for dynamic loading
    #[serde(default)]
    pub native: Option<NativeLibrary>,
    /// ABI version this package was built against
    #[serde(rename = "abiVersion")]
    pub abi_version: u32,
    /// List of macros provided by this package
    pub macros: Vec<MacroManifestEntry>,
    /// Supported runtime environments
    pub runtime: Vec<String>,
}

/// Entry in a macro manifest
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MacroManifestEntry {
    /// The kind of macro (derive, attr, call)
    pub kind: String,
    /// The name of the macro
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NativeLibrary {
    /// Path to the compiled native library (relative to the manifest file)
    pub path: String,
    /// Function symbol name that registers macros (defaults to `ts_macro_register`)
    #[serde(default = "default_registrar_symbol")]
    pub symbol: String,
}

fn default_registrar_symbol() -> String {
    "ts_macro_register".to_string()
}

impl MacroManifest {
    /// Load a macro manifest from a macro.toml file
    pub fn from_toml_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

impl MacroRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            macros: DashMap::new(),
            manifests: DashMap::new(),
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

    /// Register a manifest for a package
    pub fn register_manifest(
        &self,
        package_name: impl Into<String>,
        manifest: MacroManifest,
    ) -> Result<()> {
        let package_name = package_name.into();

        // Validate ABI version
        if manifest.abi_version != 1 {
            return Err(MacroError::AbiVersionMismatch {
                expected: 1,
                actual: manifest.abi_version,
            });
        }

        self.manifests.insert(package_name, manifest);
        Ok(())
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
        self.manifests.clear();
    }
}

impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}
