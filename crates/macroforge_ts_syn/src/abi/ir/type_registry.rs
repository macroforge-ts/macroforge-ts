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

use super::{ClassIR, EnumIR, InterfaceIR, TypeAliasIR, TypeBody, TypeMemberKind};

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

    /// The chain of type aliases from `name` back to one already on it, such
    /// as `["A", "B", "A"]`, if there is one. TypeScript rejects circular
    /// aliases; code that follows alias references needs to know, or it
    /// recurses without end.
    pub fn alias_cycle(&self, name: &str) -> Option<Vec<String>> {
        self.find_alias_cycle(name, &mut Vec::new())
    }

    fn find_alias_cycle(&self, name: &str, path: &mut Vec<String>) -> Option<Vec<String>> {
        if let Some(start) = path.iter().position(|visited| visited == name) {
            let mut cycle = path[start..].to_vec();
            cycle.push(name.to_string());
            return Some(cycle);
        }
        let entry = self.get(name)?;
        let TypeDefinitionIR::TypeAlias(alias) = &entry.definition else {
            return None;
        };
        let references: Vec<&str> = match &alias.body {
            TypeBody::Alias(target) => vec![target.as_str()],
            TypeBody::Union(members) | TypeBody::Intersection(members) => members
                .iter()
                .filter_map(|member| match &member.kind {
                    TypeMemberKind::TypeRef(referenced) => Some(referenced.as_str()),
                    _ => None,
                })
                .collect(),
            TypeBody::Object { .. } | TypeBody::Tuple(_) | TypeBody::Other(_) => Vec::new(),
        };
        path.push(name.to_string());
        let found = references
            .into_iter()
            .find_map(|referenced| self.find_alias_cycle(referenced, path));
        path.pop();
        found
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
        self.resolve_in_file(name, "", file_imports)
    }

    /// Like [`resolve`] but also disambiguates by the caller's own file path.
    /// When `name` is ambiguous and not in `file_imports`, this picks the
    /// qualified entry whose `file_path` equals `caller_file_path` — i.e. the
    /// type is declared in the same file that's referencing it (common in
    /// generated aggregator files that re-declare types alongside their
    /// canonical definitions).
    ///
    /// Pass an empty `caller_file_path` to skip same-file resolution.
    pub fn resolve_in_file(
        &self,
        name: &str,
        caller_file_path: &str,
        file_imports: &[FileImportEntry],
    ) -> Option<&TypeRegistryEntry> {
        // Fast path: unambiguous name.
        if !self.ambiguous_names.iter().any(|n| n == name) {
            return self.types.get(name);
        }
        // Same-file resolution — the caller and the declaration share a file,
        // so the entry whose file_path matches the caller is canonical here
        // even when the simple name is ambiguous globally.
        if !caller_file_path.is_empty()
            && let Some(entry) = self
                .candidates(name)
                .into_iter()
                .find(|entry| entry.file_path == caller_file_path)
        {
            return Some(entry);
        }
        // Import-based resolution. The module specifier is the textual import
        // path (`./record-link.svelte`, `$lib/types/user`) and the entry's
        // `file_path` is the on-disk path (`/…/record-link.svelte.ts`).
        let import = file_imports
            .iter()
            .find(|import| import.local_name == name)?;
        let exported = import.original_name.as_deref().unwrap_or(name);
        // A relative specifier names exactly one module: resolve it against
        // the caller's directory. Two modules can share a basename
        // (`lib/index.ts`, `lib/idp/index.ts`), so the basename alone is not
        // enough to pick between same-named types.
        if let Some(module_path) =
            resolve_relative_module(caller_file_path, &import.module_specifier)
            && let Some(entry) = self
                .candidates(exported)
                .into_iter()
                .find(|entry| file_path_is_module(&entry.file_path, &module_path))
        {
            return Some(entry);
        }
        // Otherwise match the trailing path segment, and only when that picks
        // out a single definition.
        let needle = module_specifier_basename(&import.module_specifier);
        let mut matches = self
            .candidates(exported)
            .into_iter()
            .filter(|entry| file_path_module_matches(&entry.file_path, needle));
        let first = matches.next()?;
        matches.next().is_none().then_some(first)
    }

    /// Every definition named `name`, in qualified-key order, so a lookup
    /// never depends on hash map iteration order.
    fn candidates(&self, name: &str) -> Vec<&TypeRegistryEntry> {
        let mut entries: Vec<(&String, &TypeRegistryEntry)> = self
            .qualified_types
            .iter()
            .filter(|(_, entry)| entry.name == name)
            .collect();
        entries.sort_by(|left, right| left.0.cmp(right.0));
        entries.into_iter().map(|(_, entry)| entry).collect()
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

/// Extract the module-name portion of an import path, dropping leading
/// relative prefixes (`./`, `../`, repeated `../../`) and any trailing
/// source extension. `./record-link.svelte` → `record-link.svelte`,
/// `../models/user` → `user`, `@lib/foo/bar.ts` → `bar`.
/// `path` without one trailing source extension.
fn strip_source_extension(path: &str) -> &str {
    [".ts", ".tsx", ".js", ".mjs", ".cjs"]
        .iter()
        .find_map(|extension| path.strip_suffix(extension))
        .unwrap_or(path)
}

fn module_specifier_basename(module_specifier: &str) -> &str {
    let mut s = module_specifier.trim();
    while let Some(rest) = s.strip_prefix("./").or_else(|| s.strip_prefix("../")) {
        s = rest;
    }
    strip_source_extension(s.rsplit('/').next().unwrap_or(s))
}

/// The extension-less path a relative specifier resolves to from the file at
/// `caller_file_path`, or `None` for a bare or aliased specifier.
fn resolve_relative_module(caller_file_path: &str, module_specifier: &str) -> Option<String> {
    let specifier = module_specifier.trim();
    if caller_file_path.is_empty() || !(specifier.starts_with("./") || specifier.starts_with("../"))
    {
        return None;
    }
    let mut segments: Vec<&str> = caller_file_path.split('/').collect();
    segments.pop();
    for segment in specifier.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            other => segments.push(other),
        }
    }
    Some(strip_source_extension(&segments.join("/")).to_string())
}

/// Whether the file at `file_path` is the module at `module_path`, an
/// extension-less path. A `.svelte` sub-extension still counts as the same
/// module, and a directory import resolves to its `index`.
fn file_path_is_module(file_path: &str, module_path: &str) -> bool {
    let trimmed = strip_source_extension(file_path);
    trimmed == module_path
        || trimmed
            .strip_prefix(module_path)
            .is_some_and(|rest| rest.starts_with('.') || rest == "/index")
}

/// Whether `file_path` (an absolute on-disk path) corresponds to a module
/// whose import basename matches `needle`. Strips one source extension from
/// the file's basename, then accepts an exact match or a prefix match
/// followed by a `.` so a needle like `all-types` (extension-less import)
/// still matches `all-types.svelte.ts` (Svelte component output) — the
/// `.svelte` suffix counts as a sub-extension on the same module.
fn file_path_module_matches(file_path: &str, needle: &str) -> bool {
    let trimmed = strip_source_extension(file_path.rsplit('/').next().unwrap_or(file_path));
    if trimmed == needle {
        return true;
    }
    // `all-types` matches `all-types.svelte` (sub-extension boundary). Guard
    // against partial-name collisions like `user-id` matching `user-ident` by
    // requiring the suffix to start with `.`.
    trimmed
        .strip_prefix(needle)
        .is_some_and(|rest| rest.starts_with('.'))
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

    fn make_alias_entry(name: &str, body: TypeBody) -> TypeRegistryEntry {
        TypeRegistryEntry {
            name: name.to_string(),
            file_path: "/project/src/aliases.ts".to_string(),
            is_exported: true,
            definition: TypeDefinitionIR::TypeAlias(TypeAliasIR {
                name: name.to_string(),
                span: SpanIR::new(0, 0),
                decorators: vec![],
                type_params: vec![],
                body,
            }),
            file_imports: vec![],
        }
    }

    fn type_ref(name: &str) -> super::super::TypeMember {
        super::super::TypeMember::new(TypeMemberKind::TypeRef(name.to_string()))
    }

    #[test]
    fn alias_cycle_names_the_loop_through_intersections_and_aliases() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_alias_entry("A", TypeBody::Intersection(vec![type_ref("B")])),
            "/project",
        );
        registry.insert(
            make_alias_entry("B", TypeBody::Alias("A".to_string())),
            "/project",
        );
        registry.insert(
            make_alias_entry("C", TypeBody::Alias("string".to_string())),
            "/project",
        );

        assert_eq!(
            registry.alias_cycle("A"),
            Some(vec!["A".to_string(), "B".to_string(), "A".to_string()])
        );
        assert_eq!(registry.alias_cycle("C"), None);
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

    fn import_from(local_name: &str, module_specifier: &str) -> FileImportEntry {
        FileImportEntry {
            local_name: local_name.to_string(),
            module_specifier: module_specifier.to_string(),
            original_name: None,
            is_type_only: true,
        }
    }

    fn registry_with_two_record_links() -> TypeRegistry {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("RecordLink", "/project/src/lib/index.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("RecordLink", "/project/src/lib/idp/index.ts", vec![]),
            "/project",
        );
        registry
    }

    #[test]
    fn resolve_in_file_follows_a_relative_import_to_the_exact_module() {
        let registry = registry_with_two_record_links();
        let caller = "/project/src/lib/attestation.svelte.ts";
        let imports = [import_from("RecordLink", "./index.js")];
        let resolved = registry.resolve_in_file("RecordLink", caller, &imports);
        assert_eq!(
            resolved.map(|entry| entry.file_path.as_str()),
            Some("/project/src/lib/index.ts")
        );

        let nested_caller = "/project/src/lib/idp/provider.ts";
        let nested_imports = [import_from("RecordLink", "./index")];
        let nested = registry.resolve_in_file("RecordLink", nested_caller, &nested_imports);
        assert_eq!(
            nested.map(|entry| entry.file_path.as_str()),
            Some("/project/src/lib/idp/index.ts")
        );
    }

    #[test]
    fn resolve_in_file_refuses_an_ambiguous_basename_match() {
        let registry = registry_with_two_record_links();
        let imports = [import_from("RecordLink", "$lib/index")];
        assert!(
            registry
                .resolve_in_file("RecordLink", "/project/src/routes/page.ts", &imports)
                .is_none()
        );
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
