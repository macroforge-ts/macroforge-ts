//! Project-wide type registry for compile-time type awareness.
//!
//! This module provides the data structures for a project-wide type registry
//! that maps type names to their full IR definitions. The registry is built
//! during a pre-expansion scan phase and passed to macros as context, giving
//! them Zig-style compile-time type awareness.
//!
//! ## Architecture
//!
//! ```text
//! Pre-expansion scan
//!        │
//!        ▼
//! ┌─────────────────┐
//! │  TypeRegistry    │  (HashMap<name, TypeRegistryEntry>)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ MacroContextIR  │  (Registry attached as optional field)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │  Macro Function  │  (Can introspect any project type)
//! └─────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ClassIR, EnumIR, InterfaceIR, TypeAliasIR};

/// The kind of IR stored in a registry entry.
///
/// Wraps the existing IR types to allow uniform storage in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeDefinitionIR {
    /// A class declaration.
    Class(ClassIR),
    /// An interface declaration.
    Interface(InterfaceIR),
    /// An enum declaration.
    Enum(EnumIR),
    /// A type alias declaration.
    TypeAlias(TypeAliasIR),
}

/// A single type in the project-wide type registry.
///
/// Contains the full IR of the type along with its file location
/// and export information, enabling cross-file type resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeRegistryEntry {
    /// The simple type name (e.g., "User", "Status").
    pub name: String,

    /// The absolute file path where this type is defined.
    pub file_path: String,

    /// Whether this type is exported from its module.
    pub is_exported: bool,

    /// The full IR of the type.
    pub definition: TypeDefinitionIR,

    /// Import sources this file uses (for resolving nested type references).
    pub file_imports: Vec<FileImportEntry>,
}

/// A simplified import entry from a file (for cross-file resolution).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileImportEntry {
    /// The local name used in this file (e.g., "User", "MyUser").
    pub local_name: String,
    /// The module specifier (e.g., "./models/user", "@lib/types").
    pub module_specifier: String,
    /// The original exported name, if different from local (e.g., for `import { User as MyUser }`).
    pub original_name: Option<String>,
    /// Whether this is a type-only import (`import type { ... }`).
    pub is_type_only: bool,
}

/// Project-wide type registry mapping type names to their definitions.
///
/// This is the core data structure for type-awareness. It is built during
/// the pre-expansion scan phase and passed to macros as context.
///
/// The registry supports lookup by simple name. When ambiguity exists
/// (multiple types with the same name in different files), the
/// `qualified_types` map resolves via `"relative/path/file.ts::TypeName"` keys.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TypeRegistry {
    /// Primary lookup: simple type name -> entry.
    /// For unique names, this provides O(1) access.
    /// When multiple types share a name, this holds the first one found;
    /// use `qualified_types` for disambiguation.
    pub types: HashMap<String, TypeRegistryEntry>,

    /// Qualified lookup: `"relative/path/to/file.ts::TypeName"` -> entry.
    /// Always populated for all types, used when simple name is ambiguous.
    pub qualified_types: HashMap<String, TypeRegistryEntry>,

    /// Tracks which simple names are ambiguous (exist in multiple files).
    pub ambiguous_names: Vec<String>,
}

impl TypeRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up a type by simple name. Returns `None` if the name is ambiguous
    /// (exists in multiple files) — callers must use `resolve()` with import
    /// context or `get_qualified()` for file-specific resolution.
    pub fn get(&self, name: &str) -> Option<&TypeRegistryEntry> {
        if self.ambiguous_names.iter().any(|n| n == name) {
            None
        } else {
            self.types.get(name)
        }
    }

    /// Get all qualified entries matching a simple type name.
    /// Returns an iterator over entries from different files that share this name.
    pub fn get_all(&self, name: &str) -> impl Iterator<Item = &TypeRegistryEntry> {
        self.qualified_types
            .values()
            .filter(move |entry| entry.name == name)
    }

    /// Resolve a type by name using import context for disambiguation.
    /// If the name is ambiguous (exists in multiple files), uses the caller's
    /// import entries to find the correct qualified entry.
    /// Returns `None` if ambiguous and no matching import is found.
    pub fn resolve(
        &self,
        name: &str,
        file_imports: &[FileImportEntry],
    ) -> Option<&TypeRegistryEntry> {
        // Fast path: unambiguous name
        if !self.ambiguous_names.iter().any(|n| n == name) {
            return self.types.get(name);
        }
        // Find import source for this name and match against qualified entries
        if let Some(import) = file_imports.iter().find(|i| i.local_name == name) {
            for entry in self.qualified_types.values() {
                if entry.name == name && entry.file_path.contains(&import.module_specifier) {
                    return Some(entry);
                }
            }
        }
        None
    }

    /// Look up a type by qualified path (e.g., `"src/models/user.ts::User"`).
    pub fn get_qualified(&self, qualified_name: &str) -> Option<&TypeRegistryEntry> {
        self.qualified_types.get(qualified_name)
    }

    /// Insert a type into the registry.
    ///
    /// `project_root` is used to compute the relative path for the qualified key.
    pub fn insert(&mut self, entry: TypeRegistryEntry, project_root: &str) {
        let relative_path = entry
            .file_path
            .strip_prefix(project_root)
            .unwrap_or(&entry.file_path)
            .trim_start_matches('/');
        let qualified = format!("{}::{}", relative_path, entry.name);

        if self.types.contains_key(&entry.name) {
            if !self.ambiguous_names.iter().any(|n| n == &entry.name) {
                self.ambiguous_names.push(entry.name.clone());
            }
        } else {
            self.types.insert(entry.name.clone(), entry.clone());
        }

        self.qualified_types.insert(qualified, entry);
    }

    /// Get the number of types registered.
    pub fn len(&self) -> usize {
        self.qualified_types.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.qualified_types.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abi::ir::InterfaceIR;
    use crate::abi::{DecoratorIR, SpanIR};

    fn make_interface_entry(
        name: &str,
        file_path: &str,
        decorators: Vec<DecoratorIR>,
    ) -> TypeRegistryEntry {
        TypeRegistryEntry {
            name: name.to_string(),
            file_path: file_path.to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::Interface(InterfaceIR {
                name: name.to_string(),
                span: SpanIR::new(0, 0),
                body_span: SpanIR::new(0, 0),
                type_params: vec![],
                heritage: vec![],
                decorators,
                fields: vec![],
                methods: vec![],
            }),
            file_imports: vec![],
        }
    }

    fn derive_decorator(args: &str) -> DecoratorIR {
        DecoratorIR {
            name: "Derive".to_string(),
            args_src: args.to_string(),
            span: SpanIR::new(0, 0),
            #[cfg(feature = "swc")]
            node: None,
        }
    }

    #[test]
    fn get_returns_none_for_ambiguous_names() {
        let mut registry = TypeRegistry::new();

        let entry1 = make_interface_entry(
            "PhoneNumber",
            "/project/src/types/phone-number.svelte.ts",
            vec![derive_decorator(
                "Default, Serialize, Deserialize, Gigaform",
            )],
        );
        let entry2 = make_interface_entry(
            "PhoneNumber",
            "/project/src/types/all-types.svelte.ts",
            vec![derive_decorator(
                "Default, Serialize, Deserialize, Gigaform",
            )],
        );

        registry.insert(entry1, "/project");
        registry.insert(entry2, "/project");

        // Name should be ambiguous
        assert!(
            registry
                .ambiguous_names
                .contains(&"PhoneNumber".to_string())
        );

        // get() returns None for ambiguous names — callers must use resolve() or get_all()
        assert!(registry.get("PhoneNumber").is_none());

        // get_all() returns both entries
        let all: Vec<_> = registry.get_all("PhoneNumber").collect();
        assert_eq!(all.len(), 2, "get_all() should return both entries");

        // Both files are represented
        let paths: Vec<&str> = all.iter().map(|e| e.file_path.as_str()).collect();
        assert!(paths.contains(&"/project/src/types/phone-number.svelte.ts"));
        assert!(paths.contains(&"/project/src/types/all-types.svelte.ts"));
    }

    #[test]
    fn resolve_picks_correct_entry_via_imports() {
        let mut registry = TypeRegistry::new();

        let entry1 = make_interface_entry(
            "PhoneNumber",
            "/project/src/types/phone-number.svelte.ts",
            vec![derive_decorator("Default, Gigaform")],
        );
        let entry2 = make_interface_entry(
            "PhoneNumber",
            "/project/src/types/all-types.svelte.ts",
            vec![derive_decorator("Default, Gigaform")],
        );

        registry.insert(entry1, "/project");
        registry.insert(entry2, "/project");

        // Simulate an import from "all-types"
        let imports = vec![FileImportEntry {
            local_name: "PhoneNumber".to_string(),
            module_specifier: "all-types".to_string(),
            original_name: None,
            is_type_only: true,
        }];

        let resolved = registry.resolve("PhoneNumber", &imports);
        assert!(resolved.is_some());
        assert!(
            resolved.unwrap().file_path.contains("all-types"),
            "resolve() should pick the entry matching the import source"
        );
    }

    #[test]
    fn resolve_falls_back_for_unambiguous() {
        let mut registry = TypeRegistry::new();
        let entry = make_interface_entry(
            "User",
            "/project/src/user.ts",
            vec![derive_decorator("Clone")],
        );
        registry.insert(entry, "/project");

        let result = registry.resolve("User", &[]);
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "User");
    }

    #[test]
    fn resolve_returns_none_for_ambiguous_without_import() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("Foo", "/project/src/a.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("Foo", "/project/src/b.ts", vec![]),
            "/project",
        );

        // No imports provided — ambiguous name cannot be resolved
        assert!(registry.resolve("Foo", &[]).is_none());
    }
}

/// Resolved type information for a field's type annotation.
///
/// When the type registry can resolve a field's string type to a known
/// type in the project, this provides the structured reference.
/// This is additive - the original `ts_type: String` on fields remains unchanged.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedTypeRef {
    /// The raw type string as it appears in source (e.g., `"User"`, `"User[]"`, `"Map<string, User>"`).
    pub raw_type: String,

    /// The base type name extracted from the raw type (e.g., `"User"` from `"User[]"`).
    pub base_type_name: String,

    /// The qualified key in the registry for the resolved type, if found.
    /// `None` if the type is a primitive, generic parameter, or not found in the registry.
    pub registry_key: Option<String>,

    /// Whether this is an array/collection of the base type.
    pub is_collection: bool,

    /// Whether this is optional (wrapped in `| undefined` or `| null`).
    pub is_optional: bool,

    /// Generic type arguments, if any (e.g., for `Map<string, User>`, this would contain
    /// resolved refs for `"string"` and `"User"`).
    pub type_args: Vec<ResolvedTypeRef>,
}
