//! Per-file storage for parsed declarative macros.

use std::collections::HashMap;
use std::sync::Arc;

use crate::ts_syn::declarative::MacroDef;

/// Error returned when registration fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Two macros in the same file share the same `$name`.
    DuplicateName(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::DuplicateName(name) => {
                write!(
                    f,
                    "declarative macro `${}` is defined more than once in this file",
                    name
                )
            }
        }
    }
}

impl std::error::Error for RegistryError {}

/// In-file registry of parsed declarative macros, keyed by `$name`.
///
/// A fresh registry is built per-file by the discovery pass. Cross-file
/// macro imports are deferred; the current MVP scope is definition and
/// invocation inside the same file.
#[derive(Debug, Default, Clone)]
pub struct DeclarativeMacroRegistry {
    by_name: HashMap<String, Arc<MacroDef>>,
}

impl DeclarativeMacroRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a parsed macro. Returns an error if the name is already taken.
    pub fn register(&mut self, def: MacroDef) -> Result<(), RegistryError> {
        if self.by_name.contains_key(&def.name) {
            return Err(RegistryError::DuplicateName(def.name));
        }
        let name = def.name.clone();
        self.by_name.insert(name, Arc::new(def));
        Ok(())
    }

    /// Look up a macro by name (the name excludes the leading `$`).
    pub fn lookup(&self, name: &str) -> Option<&Arc<MacroDef>> {
        self.by_name.get(name)
    }

    /// `true` iff no macros have been registered.
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }

    /// Number of registered macros.
    pub fn len(&self) -> usize {
        self.by_name.len()
    }

    /// Iterate over all registered macros.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Arc<MacroDef>)> {
        self.by_name.iter()
    }
}
