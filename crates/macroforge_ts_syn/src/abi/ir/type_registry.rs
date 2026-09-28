//! Project-wide type registry for build-time type awareness.
//!
//! This module provides the data structures for a project-wide type registry
//! that maps type names to their full IR definitions. The registry is built
//! during a pre-expansion scan phase and passed to macros as context, giving
//! them Zig-style build-time type awareness.
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
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Mutex, PoisonError};

use super::{ClassIR, EnumIR, InterfaceIR, TypeAliasIR, TypeBody, TypeMemberKind};

/// The kind of IR stored in a registry entry.
///
/// Wraps the existing IR types to allow uniform storage in the registry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl TypeDefinitionIR {
    /// Resets every source span, for comparing declarations by content.
    fn clear_spans(&mut self) {
        match self {
            TypeDefinitionIR::Class(class) => class.clear_spans(),
            TypeDefinitionIR::Interface(interface) => interface.clear_spans(),
            TypeDefinitionIR::Enum(enum_ir) => enum_ir.clear_spans(),
            TypeDefinitionIR::TypeAlias(alias) => alias.clear_spans(),
        }
    }
}

/// A single type in the project-wide type registry.
///
/// Contains the full IR of the type along with its file location
/// and export information, enabling cross-file type resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl TypeRegistryEntry {
    /// This entry with every source span reset.
    fn without_spans(&self) -> Self {
        let mut entry = self.clone();
        entry.definition.clear_spans();
        entry
    }
}

/// A simplified import entry from a file (for cross-file resolution).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

/// A registry lookup that an expansion depended on.
///
/// Recorded while a macro runs, so a build that caches expansions can tell
/// which of them a change to the registry affects: an expansion is only as
/// current as every lookup it made.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "key", rename_all = "camelCase")]
pub enum RegistryRead {
    /// Everything registered under a simple name: its primary entry, every
    /// definition sharing the name, and whether the name is ambiguous.
    Name(String),
    /// One qualified entry, `"relative/path/to/file.ts::TypeName"`.
    Qualified(String),
    /// The whole registry, for a lookup that enumerated it.
    All,
}

/// Where a registry, and every clone of it, notes the lookups made through it.
///
/// Off until [`TypeRegistry::start_recording`], so a long-lived registry does
/// not accumulate reads that nobody collects.
#[derive(Debug, Clone, Default)]
struct ReadLog(Arc<Mutex<Option<BTreeSet<RegistryRead>>>>);

impl ReadLog {
    fn with<R>(&self, f: impl FnOnce(&mut Option<BTreeSet<RegistryRead>>) -> R) -> R {
        // The set is only ever inserted into, so a poisoned lock still holds a
        // consistent one.
        let mut guard = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        f(&mut guard)
    }

    fn note(&self, read: RegistryRead) {
        self.with(|reads| {
            if let Some(reads) = reads {
                reads.insert(read);
            }
        });
    }
}

/// Which read a lookup in a [`RegistryMap`] records.
#[derive(Debug, Clone, Copy, Default)]
enum MapKeys {
    #[default]
    Names,
    Qualified,
}

/// One of the registry's maps. Reads the same as a `HashMap`, and records each
/// lookup in the registry's read log.
#[derive(Debug, Clone, Default)]
pub struct RegistryMap {
    entries: HashMap<String, TypeRegistryEntry>,
    keys: MapKeys,
    log: ReadLog,
}

impl RegistryMap {
    fn read_of(&self, key: &str) -> RegistryRead {
        match self.keys {
            MapKeys::Names => RegistryRead::Name(key.to_string()),
            MapKeys::Qualified => RegistryRead::Qualified(key.to_string()),
        }
    }

    /// The entry under `key`.
    pub fn get(&self, key: &str) -> Option<&TypeRegistryEntry> {
        self.log.note(self.read_of(key));
        self.entries.get(key)
    }

    /// Whether anything is registered under `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.log.note(self.read_of(key));
        self.entries.contains_key(key)
    }

    /// Every key and entry. Depends on the whole registry.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &TypeRegistryEntry)> {
        self.log.note(RegistryRead::All);
        self.entries.iter()
    }

    /// Every key. Depends on the whole registry.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.log.note(RegistryRead::All);
        self.entries.keys()
    }

    /// Every entry. Depends on the whole registry.
    pub fn values(&self) -> impl Iterator<Item = &TypeRegistryEntry> {
        self.log.note(RegistryRead::All);
        self.entries.values()
    }

    /// How many entries there are. Depends on the whole registry.
    pub fn len(&self) -> usize {
        self.log.note(RegistryRead::All);
        self.entries.len()
    }

    /// Whether there are none. Depends on the whole registry.
    pub fn is_empty(&self) -> bool {
        self.log.note(RegistryRead::All);
        self.entries.is_empty()
    }

    /// Registers `entry` under `key`, returning the entry it replaced.
    pub fn insert(&mut self, key: String, entry: TypeRegistryEntry) -> Option<TypeRegistryEntry> {
        self.entries.insert(key, entry)
    }
}

/// The simple names defined in more than one file. Reads like a list, and
/// records each lookup in the registry's read log.
#[derive(Debug, Clone, Default)]
pub struct AmbiguousNames {
    names: Vec<String>,
    log: ReadLog,
}

impl AmbiguousNames {
    /// Whether `name` is defined in more than one file.
    pub fn contains(&self, name: &str) -> bool {
        self.log.note(RegistryRead::Name(name.to_string()));
        self.names.iter().any(|ambiguous| ambiguous == name)
    }

    /// Every ambiguous name. Depends on the whole registry.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.log.note(RegistryRead::All);
        self.names.iter()
    }

    fn has(&self, name: &str) -> bool {
        self.names.iter().any(|ambiguous| ambiguous == name)
    }
}

/// Project-wide type registry mapping type names to their definitions.
///
/// This is the core data structure for type-awareness. It is built during
/// the pre-expansion scan phase and passed to macros as context.
///
/// The registry supports lookup by simple name. When ambiguity exists
/// (multiple types with the same name in different files), the
/// `qualified_types` map resolves via `"relative/path/file.ts::TypeName"` keys.
///
/// Every lookup, through a method or through the maps, can be recorded: see
/// [`Self::start_recording`]. Clones share one record, so the copies a macro
/// receives report into the same one.
#[derive(Debug, Clone)]
pub struct TypeRegistry {
    /// Primary lookup: simple type name -> entry.
    /// For unique names, this provides O(1) access.
    /// When multiple types share a name, this holds the first one found;
    /// use `qualified_types` for disambiguation.
    pub types: RegistryMap,

    /// Qualified lookup: `"relative/path/to/file.ts::TypeName"` -> entry.
    /// Always populated for all types, used when simple name is ambiguous.
    pub qualified_types: RegistryMap,

    /// Tracks which simple names are ambiguous (exist in multiple files).
    pub ambiguous_names: AmbiguousNames,

    log: ReadLog,
}

/// What a [`RegistryRead`] resolves to: everything the lookup can answer
/// from, with source spans cleared and maps ordered, so two resolutions
/// serialize identically exactly when the lookup's answer is the same.
///
/// Spans are left out because they locate a declaration in its own file: an
/// edit above it moves them without changing anything another module's
/// expansion can use.
#[derive(Debug, PartialEq, Serialize)]
pub enum Resolution {
    /// The entry the name resolves to, whether it is ambiguous, and every
    /// declaration of it by qualified key.
    Name {
        primary: Option<TypeRegistryEntry>,
        ambiguous: bool,
        definitions: BTreeMap<String, TypeRegistryEntry>,
    },
    Qualified(Option<TypeRegistryEntry>),
    All {
        types: BTreeMap<String, TypeRegistryEntry>,
        qualified_types: BTreeMap<String, TypeRegistryEntry>,
        ambiguous_names: BTreeSet<String>,
    },
}

/// The registry's serialized form.
#[derive(Deserialize)]
struct RegistryData {
    types: HashMap<String, TypeRegistryEntry>,
    qualified_types: HashMap<String, TypeRegistryEntry>,
    ambiguous_names: Vec<String>,
}

/// The registry's serialized form, borrowed.
#[derive(Serialize)]
struct RegistryDataRef<'a> {
    types: &'a HashMap<String, TypeRegistryEntry>,
    qualified_types: &'a HashMap<String, TypeRegistryEntry>,
    ambiguous_names: &'a [String],
}

impl Serialize for TypeRegistry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        RegistryDataRef {
            types: &self.types.entries,
            qualified_types: &self.qualified_types.entries,
            ambiguous_names: &self.ambiguous_names.names,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TypeRegistry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let data = RegistryData::deserialize(deserializer)?;
        Ok(Self::from_parts(
            data.types,
            data.qualified_types,
            data.ambiguous_names,
        ))
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        Self::from_parts(HashMap::new(), HashMap::new(), Vec::new())
    }
}

impl TypeRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    fn from_parts(
        types: HashMap<String, TypeRegistryEntry>,
        qualified_types: HashMap<String, TypeRegistryEntry>,
        ambiguous_names: Vec<String>,
    ) -> Self {
        let log = ReadLog::default();
        Self {
            types: RegistryMap {
                entries: types,
                keys: MapKeys::Names,
                log: log.clone(),
            },
            qualified_types: RegistryMap {
                entries: qualified_types,
                keys: MapKeys::Qualified,
                log: log.clone(),
            },
            ambiguous_names: AmbiguousNames {
                names: ambiguous_names,
                log: log.clone(),
            },
            log,
        }
    }

    /// Starts recording the lookups made through this registry and its clones,
    /// discarding any recorded so far.
    pub fn start_recording(&self) {
        self.log.with(|reads| *reads = Some(BTreeSet::new()));
    }

    /// Stops recording and returns what was recorded, or `None` if recording
    /// was never started.
    pub fn finish_recording(&self) -> Option<BTreeSet<RegistryRead>> {
        self.log.with(Option::take)
    }

    /// Adds the lookups a macro reported making against its own copy of the
    /// registry. A macro that reports none was built before lookups were
    /// recorded, so it may have read anything.
    pub fn record_macro_reads(&self, reported: Option<&BTreeSet<RegistryRead>>) {
        self.log.with(|reads| {
            if let Some(reads) = reads {
                match reported {
                    Some(reported) => reads.extend(reported.iter().cloned()),
                    None => {
                        reads.insert(RegistryRead::All);
                    }
                }
            }
        });
    }

    /// What `read` currently resolves to. Unlike the lookups themselves, this
    /// records nothing.
    pub fn resolution_of(&self, read: &RegistryRead) -> Resolution {
        match read {
            RegistryRead::Name(name) => Resolution::Name {
                primary: self
                    .types
                    .entries
                    .get(name)
                    .map(TypeRegistryEntry::without_spans),
                ambiguous: self.ambiguous_names.has(name),
                definitions: without_spans(
                    self.qualified_types
                        .entries
                        .iter()
                        .filter(|(_, entry)| &entry.name == name),
                ),
            },
            RegistryRead::Qualified(key) => Resolution::Qualified(
                self.qualified_types
                    .entries
                    .get(key)
                    .map(TypeRegistryEntry::without_spans),
            ),
            RegistryRead::All => Resolution::All {
                types: without_spans(self.types.entries.iter()),
                qualified_types: without_spans(self.qualified_types.entries.iter()),
                ambiguous_names: self.ambiguous_names.names.iter().cloned().collect(),
            },
        }
    }

    /// Look up a type by simple name. Returns `None` if the name is ambiguous
    /// (exists in multiple files) — callers must use `resolve()` with import
    /// context or `get_qualified()` for file-specific resolution.
    pub fn get(&self, name: &str) -> Option<&TypeRegistryEntry> {
        self.log.note(RegistryRead::Name(name.to_string()));
        if self.ambiguous_names.has(name) {
            None
        } else {
            self.types.entries.get(name)
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
        self.log.note(RegistryRead::Name(name.to_string()));
        self.qualified_types
            .entries
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
        self.log.note(RegistryRead::Name(name.to_string()));
        // Fast path: unambiguous name.
        if !self.ambiguous_names.has(name) {
            return self.types.entries.get(name);
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
        self.log.note(RegistryRead::Name(name.to_string()));
        let mut entries: Vec<(&String, &TypeRegistryEntry)> = self
            .qualified_types
            .entries
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

        if self.types.entries.contains_key(&entry.name) {
            if !self.ambiguous_names.has(&entry.name) {
                self.ambiguous_names.names.push(entry.name.clone());
            }
        } else {
            self.types.entries.insert(entry.name.clone(), entry.clone());
        }

        self.qualified_types.entries.insert(qualified, entry);
    }

    /// Get the number of types registered. Depends on the whole registry.
    pub fn len(&self) -> usize {
        self.log.note(RegistryRead::All);
        self.qualified_types.entries.len()
    }

    /// Check if the registry is empty. Depends on the whole registry.
    pub fn is_empty(&self) -> bool {
        self.log.note(RegistryRead::All);
        self.qualified_types.entries.is_empty()
    }
}

/// `entries` in key order, each without its source spans.
fn without_spans<'a>(
    entries: impl Iterator<Item = (&'a String, &'a TypeRegistryEntry)>,
) -> BTreeMap<String, TypeRegistryEntry> {
    entries
        .map(|(key, entry)| (key.clone(), entry.without_spans()))
        .collect()
}

/// `path` without one trailing source extension.
fn strip_source_extension(path: &str) -> &str {
    [".ts", ".tsx", ".js", ".mjs", ".cjs"]
        .iter()
        .find_map(|extension| path.strip_suffix(extension))
        .unwrap_or(path)
}

/// Extract the module-name portion of an import path, dropping leading
/// relative prefixes (`./`, `../`, repeated `../../`) and any trailing
/// source extension. `./record-link.svelte` → `record-link.svelte`,
/// `../models/user` → `user`, `@lib/foo/bar.ts` → `bar`.
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
        assert!(registry.ambiguous_names.contains("PhoneNumber"));

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

    fn reads_of(registry: &TypeRegistry) -> Vec<RegistryRead> {
        registry
            .finish_recording()
            .expect("recording was started")
            .into_iter()
            .collect()
    }

    #[test]
    fn lookups_are_not_recorded_until_recording_starts() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        assert!(registry.get("User").is_some());
        assert_eq!(registry.finish_recording(), None);
    }

    #[test]
    fn each_lookup_records_what_it_depended_on() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("Order", "/project/src/order.ts", vec![]),
            "/project",
        );

        registry.start_recording();
        assert!(registry.get("User").is_some());
        assert!(registry.types.get("Order").is_some());
        assert!(!registry.ambiguous_names.contains("Missing"));
        assert!(registry.get_qualified("src/user.ts::User").is_some());
        assert_eq!(
            reads_of(&registry),
            vec![
                RegistryRead::Name("Missing".to_string()),
                RegistryRead::Name("Order".to_string()),
                RegistryRead::Name("User".to_string()),
                RegistryRead::Qualified("src/user.ts::User".to_string()),
            ]
        );

        registry.start_recording();
        assert_eq!(registry.types.iter().count(), 2);
        assert_eq!(reads_of(&registry), vec![RegistryRead::All]);
    }

    #[test]
    fn clones_record_into_the_same_log() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.start_recording();
        let copy = registry.clone();
        assert!(copy.get("User").is_some());
        assert_eq!(
            reads_of(&registry),
            vec![RegistryRead::Name("User".to_string())]
        );
    }

    #[test]
    fn a_macro_that_reports_no_reads_may_have_read_anything() {
        let registry = TypeRegistry::new();
        registry.start_recording();
        registry.record_macro_reads(Some(&BTreeSet::from([RegistryRead::Name("A".to_string())])));
        registry.record_macro_reads(None);
        assert_eq!(
            reads_of(&registry),
            vec![RegistryRead::Name("A".to_string()), RegistryRead::All]
        );
    }

    #[test]
    fn a_resolution_ignores_spans_but_not_annotations() {
        let read = RegistryRead::Name("User".to_string());
        let resolution = |entry: TypeRegistryEntry| {
            let mut registry = TypeRegistry::new();
            registry.insert(entry, "/project");
            registry.resolution_of(&read)
        };
        let original = make_interface_entry(
            "User",
            "/project/src/user.ts",
            vec![derive_decorator("Clone")],
        );

        let mut moved = original.clone();
        if let TypeDefinitionIR::Interface(interface) = &mut moved.definition {
            interface.span = SpanIR::new(40, 90);
            interface.body_span = SpanIR::new(60, 90);
            interface.decorators[0].span = SpanIR::new(20, 38);
        }
        assert_eq!(resolution(original.clone()), resolution(moved));

        let annotated = make_interface_entry(
            "User",
            "/project/src/user.ts",
            vec![derive_decorator("Clone, Debug")],
        );
        assert_ne!(resolution(original), resolution(annotated));
    }

    #[test]
    fn a_type_alias_resolution_ignores_its_members_spans() {
        let read = RegistryRead::Name("Shape".to_string());
        let resolution = |decorator_span: SpanIR| {
            let mut member = type_ref("Circle");
            member.decorators = vec![DecoratorIR {
                span: decorator_span,
                ..derive_decorator("Default")
            }];
            let mut registry = TypeRegistry::new();
            registry.insert(
                make_alias_entry("Shape", TypeBody::Union(vec![member])),
                "/project",
            );
            registry.resolution_of(&read)
        };
        assert_eq!(
            resolution(SpanIR::new(0, 10)),
            resolution(SpanIR::new(100, 110))
        );
    }

    #[test]
    fn the_serialized_form_is_the_plain_maps_and_starts_unrecorded() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let json = serde_json::to_value(&registry).expect("the registry serializes");
        let mut keys: Vec<&String> = json.as_object().expect("an object").keys().collect();
        keys.sort();
        assert_eq!(keys, ["ambiguous_names", "qualified_types", "types"]);

        let restored: TypeRegistry =
            serde_json::from_value(json).expect("the registry deserializes");
        assert!(restored.get("User").is_some());
        assert_eq!(restored.finish_recording(), None);
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
