//! # Macro Registry
//!
//! The registry provides thread-safe storage and lookup for macro implementations.
//! It uses `DashMap` for concurrent access, allowing multiple files to be processed
//! in parallel without locking.
//!
//! ## Lookup Strategies
//!
//! The registry supports multiple lookup strategies:
//!
//! 1. **Exact match**: `lookup(module, name)` - Finds macro by full qualified name
//! 2. **Name-only**: `lookup_by_name(name)` - Finds first macro with matching name
//! 3. **Fallback**: `lookup_with_fallback(module, name)` - Tries exact, then name-only
//!
//! The fallback strategy is used during dispatch to handle cases where the import
//! path doesn't exactly match the registration path.
//!
//! ## Thread Safety
//!
//! All registry operations are thread-safe through `DashMap`. Registration checks
//! for duplicates atomically to prevent race conditions during parallel loading.

use super::{MacroError, Macroforge, error::Result};
use dashmap::DashMap;
use std::sync::Arc;

/// Composite key for identifying a macro by module and name.
///
/// Macros are uniquely identified by the combination of their module path
/// (where they're registered from) and their name.
///
/// # Example
///
/// ```rust
/// use macroforge_ts::host::registry::MacroKey;
///
/// let key = MacroKey::new("builtin", "Debug");
/// // Identifies the "Debug" macro from the "builtin" module
/// assert_eq!(key.module, "builtin");
/// assert_eq!(key.name, "Debug");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroKey {
    /// The module/package the macro comes from (e.g., "builtin", "@my-org/macros").
    pub module: String,
    /// The name of the macro (e.g., "Debug", "Clone", "Serialize").
    pub name: String,
}

impl MacroKey {
    /// Creates a new macro key.
    ///
    /// # Arguments
    ///
    /// * `module` - The module path (converted to String)
    /// * `name` - The macro name (converted to String)
    pub fn new(module: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            name: name.into(),
        }
    }
}

/// Thread-safe registry for all available macros.
///
/// The registry stores macro implementations indexed by their module and name.
/// It uses `DashMap` for lock-free concurrent access, enabling parallel
/// file processing.
///
/// # Concurrency
///
/// Multiple threads can:
/// - Look up macros simultaneously without blocking
/// - Register macros during expansion (though this is rare)
///
/// # Example
///
/// ```rust,no_run
/// use macroforge_ts::host::{MacroRegistry, Macroforge};
/// use macroforge_ts_syn::{TsStream, MacroKind, MacroResult};
/// use std::sync::Arc;
///
/// // Define a simple macro
/// struct MyMacro;
///
/// impl Macroforge for MyMacro {
///     fn name(&self) -> &str { "MyMacro" }
///     fn kind(&self) -> MacroKind { MacroKind::Derive }
///     fn run(&self, _input: TsStream) -> MacroResult { MacroResult::default() }
/// }
///
/// // Create registry and register the macro
/// let registry = MacroRegistry::new();
/// registry.register("my-module", "MyMacro", Arc::new(MyMacro)).unwrap();
///
/// // Look up by exact module and name
/// let macro_impl = registry.lookup("my-module", "MyMacro").unwrap();
/// assert_eq!(macro_impl.name(), "MyMacro");
///
/// // Look up by name only (fallback for flexible resolution)
/// let macro_impl = registry.lookup_by_name("MyMacro").unwrap();
/// assert_eq!(macro_impl.name(), "MyMacro");
/// ```
pub struct MacroRegistry {
    /// Map from (module, name) to macro implementation.
    /// Uses DashMap for concurrent access without external locking.
    macros: DashMap<MacroKey, Arc<dyn Macroforge>>,
}

impl MacroRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self {
            macros: DashMap::new(),
        }
    }

    /// Registers a macro in the registry.
    ///
    /// # Arguments
    ///
    /// * `module` - The module this macro belongs to
    /// * `name` - The macro name (used in `@derive(Name)`)
    /// * `macro_impl` - The macro implementation (wrapped in Arc for sharing)
    ///
    /// # Returns
    ///
    /// `Ok(())` on success.
    ///
    /// # Errors
    ///
    /// Returns `MacroError::InvalidConfig` if a macro with the same
    /// module and name is already registered.
    ///
    /// # Thread Safety
    ///
    /// This operation is atomic - duplicate checking and insertion happen
    /// without race conditions.
    pub fn register(
        &self,
        module: impl Into<String>,
        name: impl Into<String>,
        macro_impl: Arc<dyn Macroforge>,
    ) -> Result<()> {
        let key = MacroKey::new(module, name);

        // Atomic check-and-insert to prevent duplicates in concurrent registration
        if self.macros.contains_key(&key) {
            return Err(MacroError::InvalidConfig(format!(
                "Macro '{}::{}' is already registered",
                key.module, key.name
            )));
        }

        self.macros.insert(key, macro_impl);
        Ok(())
    }

    /// Looks up a macro by its exact module and name.
    ///
    /// # Arguments
    ///
    /// * `module` - The module to search in
    /// * `name` - The macro name to find
    ///
    /// # Returns
    ///
    /// The macro implementation wrapped in `Arc`.
    ///
    /// # Errors
    ///
    /// Returns `MacroError::MacroNotFound` if no matching macro exists.
    pub fn lookup(&self, module: &str, name: &str) -> Result<Arc<dyn Macroforge>> {
        let key = MacroKey::new(module, name);

        self.macros
            .get(&key)
            .map(|entry| Arc::clone(&entry))
            .ok_or_else(|| MacroError::MacroNotFound {
                module: module.to_string(),
                name: name.to_string(),
            })
    }

    /// Returns all registered macros as a vector.
    ///
    /// Useful for debugging, manifest generation, and introspection.
    ///
    /// # Returns
    ///
    /// A vector of (key, implementation) pairs for all registered macros.
    pub fn all_macros(&self) -> Vec<(MacroKey, Arc<dyn Macroforge>)> {
        self.macros
            .iter()
            .map(|entry| (entry.key().clone(), Arc::clone(entry.value())))
            .collect()
    }

    /// Checks if a macro is registered without retrieving it.
    ///
    /// # Arguments
    ///
    /// * `module` - The module to check
    /// * `name` - The macro name to check
    ///
    /// # Returns
    ///
    /// `true` if the macro is registered, `false` otherwise.
    pub fn contains(&self, module: &str, name: &str) -> bool {
        let key = MacroKey::new(module, name);
        self.macros.contains_key(&key)
    }

    /// Looks up a macro by name only, ignoring the module path.
    ///
    /// This is used as a fallback when the exact module path doesn't match,
    /// which can happen with dynamic imports or different module resolution.
    ///
    /// # Arguments
    ///
    /// * `name` - The macro name to find
    ///
    /// # Returns
    ///
    /// The first macro found with the matching name.
    ///
    /// # Errors
    ///
    /// Returns `MacroError::MacroNotFound` if no macro with that name exists.
    ///
    /// # Note
    ///
    /// If multiple macros have the same name in different modules,
    /// this returns the first one found (iteration order is not guaranteed).
    pub fn lookup_by_name(&self, name: &str) -> Result<Arc<dyn Macroforge>> {
        for entry in self.macros.iter() {
            if entry.key().name == name {
                return Ok(Arc::clone(&entry));
            }
        }

        Err(MacroError::MacroNotFound {
            module: "<any>".to_string(),
            name: name.to_string(),
        })
    }

    /// Looks up a macro with fallback to name-only resolution.
    ///
    /// This is the primary lookup method used by the dispatcher.
    ///
    /// # Algorithm
    ///
    /// 1. Try exact match with module and name
    /// 2. If not found, try name-only lookup
    ///
    /// # Arguments
    ///
    /// * `module` - The module to try first
    /// * `name` - The macro name
    ///
    /// # Returns
    ///
    /// The macro implementation if found by either strategy.
    ///
    /// # Errors
    ///
    /// Returns `MacroError::MacroNotFound` only if both lookups fail.
    pub fn lookup_with_fallback(&self, module: &str, name: &str) -> Result<Arc<dyn Macroforge>> {
        // First try exact match
        if let Ok(m) = self.lookup(module, name) {
            return Ok(m);
        }

        // Fall back to name-only lookup
        self.lookup_by_name(name)
    }

    /// Removes all registered macros from the registry.
    ///
    /// Useful for testing or reinitialization scenarios.
    pub fn clear(&self) {
        self.macros.clear();
    }
}

impl Default for MacroRegistry {
    fn default() -> Self {
        Self::new()
    }
}
