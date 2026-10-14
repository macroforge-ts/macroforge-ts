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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

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

    /// The names its module exports it under besides its own: `default` for
    /// a default export, and the alias of `export { User as Member }`.
    #[serde(default)]
    pub exported_as: Vec<String>,

    /// The names other modules export it under again: a barrel's
    /// `export * from './user'` or `export { User as Member } from './user'`.
    #[serde(default)]
    pub re_exported: Vec<ReExport>,

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

    /// Whether some module exports this declaration under `name` where it is
    /// declared under another: its own module, or one that exports it again.
    fn is_exported_as(&self, name: &str) -> bool {
        self.exported_as.iter().any(|exported| exported == name)
            || self.re_exported.iter().any(|again| again.name == name)
    }
}

/// A name a module exports a type under that another module declares.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReExport {
    /// The file of the module that exports it again.
    pub file_path: String,
    /// The name that module exports it under.
    pub name: String,
    /// Whether that module exports everything the declaring one does, by
    /// `export *` all the way: the type's generated functions come with it
    /// then, where a named re-export brings the type alone.
    pub whole: bool,
}

/// What a module exports again from another, as its source writes it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleReExport {
    /// The module it comes from, as written (`./user`).
    pub module_specifier: String,
    /// The name it has there: `default` for that module's default export,
    /// and `*` for `export * from`, which takes every name but `default`.
    pub imported: String,
    /// The name it is exported under here, and `*` for `export * from`.
    pub exported: String,
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
    /// The declarations exported under a name they are not declared by: one
    /// a module gives a declaration of its own (`export { Late as Shown }`),
    /// or one another module exports it again under.
    ExportedAs(String),
    /// Whether the module at this path, written without its extension,
    /// declares a type or exports one again.
    Module(String),
}

/// Where a registry, and every clone of it, notes the lookups made through it.
///
/// Off until [`TypeRegistry::start_recording`], so a long-lived registry does
/// not accumulate reads that nobody collects.
#[derive(Debug, Clone, Default)]
struct ReadLog(Arc<ReadLogState>);

#[derive(Debug, Default)]
struct ReadLogState {
    /// Checked before anything else, so a lookup made while nothing records
    /// costs neither a lock nor an allocation.
    recording: AtomicBool,
    reads: Mutex<BTreeSet<RegistryRead>>,
}

impl ReadLog {
    fn reads(&self) -> MutexGuard<'_, BTreeSet<RegistryRead>> {
        // The set is only ever inserted into or taken whole, so a poisoned
        // lock still holds a consistent one.
        self.0.reads.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn is_recording(&self) -> bool {
        self.0.recording.load(Ordering::Acquire)
    }

    /// Notes the read `read` builds, building it only while recording.
    fn note(&self, read: impl FnOnce() -> RegistryRead) {
        if self.is_recording() {
            self.reads().insert(read());
        }
    }

    fn start(&self) {
        let mut reads = self.reads();
        reads.clear();
        self.0.recording.store(true, Ordering::Release);
    }

    fn finish(&self) -> Option<BTreeSet<RegistryRead>> {
        let mut reads = self.reads();
        self.0
            .recording
            .swap(false, Ordering::AcqRel)
            .then(|| std::mem::take(&mut *reads))
    }

    fn extend(&self, reported: impl IntoIterator<Item = RegistryRead>) {
        if self.is_recording() {
            self.reads().extend(reported);
        }
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
///
/// The entries are shared between clones and copied on the first insert into
/// a shared map, so handing a registry to every macro costs no copy of it.
#[derive(Debug, Clone, Default)]
pub struct RegistryMap {
    entries: Arc<MapEntries>,
    keys: MapKeys,
    log: ReadLog,
    /// Changes whenever the entries do; see [`RegistryGeneration`].
    version: u64,
}

/// A map's entries, indexed by the simple name each one declares.
#[derive(Debug, Clone, Default)]
struct MapEntries {
    by_key: HashMap<String, TypeRegistryEntry>,
    /// The keys of every entry declaring a name, sorted, so a lookup by name
    /// neither scans the map nor depends on its iteration order.
    keys_by_name: HashMap<String, Vec<String>>,
}

impl MapEntries {
    fn new(by_key: HashMap<String, TypeRegistryEntry>) -> Self {
        let mut keys_by_name: HashMap<String, Vec<String>> = HashMap::new();
        for (key, entry) in &by_key {
            keys_by_name
                .entry(entry.name.clone())
                .or_default()
                .push(key.clone());
        }
        for keys in keys_by_name.values_mut() {
            keys.sort();
        }
        Self {
            by_key,
            keys_by_name,
        }
    }

    fn insert(&mut self, key: String, entry: TypeRegistryEntry) -> Option<TypeRegistryEntry> {
        let name = entry.name.clone();
        let keys = self.keys_by_name.entry(name.clone()).or_default();
        if let Err(at) = keys.binary_search(&key) {
            keys.insert(at, key.clone());
        }
        let replaced = self.by_key.insert(key.clone(), entry)?;
        if replaced.name != name
            && let Some(keys) = self.keys_by_name.get_mut(&replaced.name)
        {
            keys.retain(|named| *named != key);
            if keys.is_empty() {
                self.keys_by_name.remove(&replaced.name);
            }
        }
        Some(replaced)
    }

    /// Every entry declaring `name`, in key order.
    fn named<'a>(
        &'a self,
        name: &str,
    ) -> impl Iterator<Item = (&'a String, &'a TypeRegistryEntry)> + 'a {
        self.keys_by_name
            .get(name)
            .into_iter()
            .flatten()
            .filter_map(|key| self.by_key.get_key_value(key))
    }
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
        self.log.note(|| self.read_of(key));
        self.entries.by_key.get(key)
    }

    /// Whether anything is registered under `key`.
    pub fn contains_key(&self, key: &str) -> bool {
        self.log.note(|| self.read_of(key));
        self.entries.by_key.contains_key(key)
    }

    /// Every key and entry. Depends on the whole registry.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &TypeRegistryEntry)> {
        self.log.note(|| RegistryRead::All);
        self.entries.by_key.iter()
    }

    /// Every key. Depends on the whole registry.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.log.note(|| RegistryRead::All);
        self.entries.by_key.keys()
    }

    /// Every entry. Depends on the whole registry.
    pub fn values(&self) -> impl Iterator<Item = &TypeRegistryEntry> {
        self.log.note(|| RegistryRead::All);
        self.entries.by_key.values()
    }

    /// How many entries there are. Depends on the whole registry.
    pub fn len(&self) -> usize {
        self.log.note(|| RegistryRead::All);
        self.entries.by_key.len()
    }

    /// Whether there are none. Depends on the whole registry.
    pub fn is_empty(&self) -> bool {
        self.log.note(|| RegistryRead::All);
        self.entries.by_key.is_empty()
    }

    /// Registers `entry` under `key`, returning the entry it replaced.
    pub fn insert(&mut self, key: String, entry: TypeRegistryEntry) -> Option<TypeRegistryEntry> {
        self.version = next_version();
        Arc::make_mut(&mut self.entries).insert(key, entry)
    }
}

/// The simple names defined in more than one file. Reads like a list, and
/// records each lookup in the registry's read log.
#[derive(Debug, Clone, Default)]
pub struct AmbiguousNames {
    names: Arc<NameList>,
    log: ReadLog,
    /// Changes whenever the names do; see [`RegistryGeneration`].
    version: u64,
}

/// Names in the order they were found, with a set for membership.
#[derive(Debug, Clone, Default)]
struct NameList {
    in_order: Vec<String>,
    members: HashSet<String>,
}

impl NameList {
    fn new(in_order: Vec<String>) -> Self {
        let members = in_order.iter().cloned().collect();
        Self { in_order, members }
    }

    fn push(&mut self, name: String) {
        if self.members.insert(name.clone()) {
            self.in_order.push(name);
        }
    }
}

impl AmbiguousNames {
    /// Whether `name` is defined in more than one file.
    pub fn contains(&self, name: &str) -> bool {
        self.log.note(|| RegistryRead::Name(name.to_string()));
        self.has(name)
    }

    /// Every ambiguous name. Depends on the whole registry.
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.log.note(|| RegistryRead::All);
        self.names.in_order.iter()
    }

    fn has(&self, name: &str) -> bool {
        self.names.members.contains(name)
    }
}

/// Equal when they hold the same entries. Comparing is not a lookup, so it
/// records nothing.
impl PartialEq for RegistryMap {
    fn eq(&self, other: &Self) -> bool {
        self.entries.by_key == other.entries.by_key
    }
}

/// Equal when they name the same types, in whatever order they were found.
impl PartialEq for AmbiguousNames {
    fn eq(&self, other: &Self) -> bool {
        self.names.members == other.names.members
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

    /// When set, this registry serializes as a reference to the resident
    /// registry of this generation instead of its contents.
    resident_reference: Option<RegistryGeneration>,

    /// The file being expanded and the names it declares, when the host has
    /// said: see [`Self::declaring`]. It is one expansion's knowledge, and no
    /// part of what a registry serializes.
    declaring: Option<Arc<DeclaringFile>>,
}

/// A file, and the names of the types it declares itself.
#[derive(Debug)]
struct DeclaringFile {
    file_path: String,
    /// What the file declares, by name: what answers for a name where the
    /// scan has not read the file.
    own: HashMap<String, TypeRegistryEntry>,
}

/// `path` with the separators Windows writes as the `/` every comparison of
/// paths here is made in.
fn slashed(path: &str) -> std::borrow::Cow<'_, str> {
    if path.contains('\\') {
        std::borrow::Cow::Owned(path.replace('\\', "/"))
    } else {
        std::borrow::Cow::Borrowed(path)
    }
}

/// Whether two paths name one file, where one may be written from a root the
/// other is written below: `src/user.ts` and `/project/src/user.ts`.
fn same_file(left: &str, right: &str) -> bool {
    let (left, right) = (slashed(left), slashed(right));
    let (left, right) = (
        left.trim_start_matches("./"),
        right.trim_start_matches("./"),
    );
    let ends_in = |whole: &str, end: &str| {
        whole
            .strip_suffix(end)
            .is_some_and(|before| before.ends_with('/'))
    };
    left == right || ends_in(left, right) || ends_in(right, left)
}

/// Identifies a registry's contents within one process: any insert, through
/// the registry or through one of its maps, gives it a new generation, and a
/// clone keeps its source's. A host that has installed one generation in a
/// macro guest can then send contexts that refer to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegistryGeneration([u64; 3]);

/// The source of every registry version in the process.
static VERSIONS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn next_version() -> u64 {
    VERSIONS.fetch_add(1, Ordering::Relaxed) + 1
}

/// How many installed registries a guest keeps. A guest serves the projects
/// of the processes that load it, normally one, and the bound keeps a long
/// session from holding every version it ever saw.
const RESIDENT_SLOTS: usize = 4;

/// The registries a host installed in this process, most recent last, for
/// contexts that refer to them rather than carrying them.
static RESIDENT: Mutex<Vec<(RegistryGeneration, TypeRegistry)>> = Mutex::new(Vec::new());

/// What a host sends to install a registry in a macro guest.
#[derive(Serialize, Deserialize)]
pub struct ResidentRegistryPayload {
    pub generation: RegistryGeneration,
    pub registry: TypeRegistry,
}

/// Installs the registry a host sent as `payload_json`, so contexts that
/// refer to its generation resolve to it.
///
/// # Errors
///
/// Fails when the payload is not a [`ResidentRegistryPayload`].
pub fn install_resident_registry(payload_json: &str) -> Result<(), String> {
    let payload: ResidentRegistryPayload = serde_json::from_str(payload_json)
        .map_err(|error| format!("invalid resident registry payload: {error}"))?;
    let mut resident = RESIDENT.lock().unwrap_or_else(PoisonError::into_inner);
    resident.retain(|(generation, _)| *generation != payload.generation);
    if resident.len() == RESIDENT_SLOTS {
        resident.remove(0);
    }
    resident.push((payload.generation, payload.registry));
    Ok(())
}

/// The installed registry of `generation`, with a read log of its own so the
/// lookups one call makes are recorded apart from every other call's.
fn resident_registry(generation: RegistryGeneration) -> Result<TypeRegistry, String> {
    RESIDENT
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .iter()
        .find(|(installed, _)| *installed == generation)
        .map(|(_, registry)| registry.with_fresh_log())
        .ok_or_else(|| {
            format!(
                "the host referred to a registry it has not installed here ({generation:?}); \
                 a host must install a registry before sending contexts that refer to it"
            )
        })
}

/// What a [`RegistryRead`] resolves to: everything the lookup can answer
/// from, with source spans cleared and maps ordered, so two resolutions
/// encode identically exactly when the lookup's answer is the same.
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
    /// The declarations exported under the name, by qualified key.
    ExportedAs(BTreeMap<String, TypeRegistryEntry>),
    /// Whether the module holds types.
    Module(bool),
}

/// The version of the compact wire form [`RegistryWire`] writes.
const WIRE_VERSION: u32 = 3;

/// The registry's serialized form. Each entry is written once, under its
/// qualified key, and each file's imports once, so a project registry is a
/// fraction of the size of its two maps written out whole; reading it back
/// rebuilds the maps exactly. The first form, both maps written in full, is
/// still read.
#[derive(Serialize)]
struct RegistryWire<'a> {
    version: u32,
    /// Every entry by qualified key, without its file's imports.
    entries: BTreeMap<&'a str, WireEntry<'a>>,
    /// Each simple name's entry, by the qualified key of the identical entry.
    primary: BTreeMap<&'a str, &'a str>,
    /// Simple names whose entry no qualified entry matches, written whole.
    /// The scanner never produces one; a registry built by hand can.
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    unmatched_types: BTreeMap<&'a str, WireEntry<'a>>,
    ambiguous_names: &'a [String],
    /// Each file's imports, shared by every entry declared in it.
    file_imports: BTreeMap<&'a str, &'a [FileImportEntry]>,
}

/// An entry on the wire. `file_imports` is written only when it differs
/// from the imports recorded for the entry's file.
#[derive(Serialize)]
struct WireEntry<'a> {
    name: &'a str,
    file_path: &'a str,
    is_exported: bool,
    #[serde(skip_serializing_if = "no_names")]
    exported_as: &'a [String],
    #[serde(skip_serializing_if = "no_re_exports")]
    re_exported: &'a [ReExport],
    definition: &'a TypeDefinitionIR,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_imports: Option<&'a [FileImportEntry]>,
}

/// Whether an entry has no other names to write.
fn no_names(names: &&[String]) -> bool {
    names.is_empty()
}

/// Whether an entry has no re-exports to write.
fn no_re_exports(re_exports: &&[ReExport]) -> bool {
    re_exports.is_empty()
}

/// An entry read from the wire, before its file's imports are filled in.
#[derive(Deserialize)]
struct WireEntryData {
    name: String,
    file_path: String,
    is_exported: bool,
    #[serde(default)]
    exported_as: Vec<String>,
    #[serde(default)]
    re_exported: Vec<ReExport>,
    definition: TypeDefinitionIR,
    #[serde(default)]
    file_imports: Option<Vec<FileImportEntry>>,
}

impl WireEntryData {
    fn into_entry(
        self,
        file_imports: &HashMap<String, Vec<FileImportEntry>>,
    ) -> Result<TypeRegistryEntry, String> {
        let imports = match self.file_imports {
            Some(imports) => imports,
            None => file_imports
                .get(&self.file_path)
                .cloned()
                .ok_or_else(|| format!("the registry records no imports for {}", self.file_path))?,
        };
        Ok(TypeRegistryEntry {
            name: self.name,
            file_path: self.file_path,
            is_exported: self.is_exported,
            exported_as: self.exported_as,
            re_exported: self.re_exported,
            definition: self.definition,
            file_imports: imports,
        })
    }
}

impl<'a> RegistryWire<'a> {
    fn of(registry: &'a TypeRegistry) -> Self {
        let mut file_imports: BTreeMap<&'a str, &'a [FileImportEntry]> = BTreeMap::new();
        let mut entries = BTreeMap::new();
        for (key, entry) in registry.qualified_types.entries.by_key.iter() {
            let recorded = *file_imports
                .entry(entry.file_path.as_str())
                .or_insert(entry.file_imports.as_slice());
            entries.insert(
                key.as_str(),
                WireEntry::of(entry, recorded != entry.file_imports.as_slice()),
            );
        }
        let mut primary = BTreeMap::new();
        let mut unmatched_types = BTreeMap::new();
        for (name, entry) in registry.types.entries.by_key.iter() {
            let twin = registry
                .qualified_types
                .entries
                .named(name)
                .find(|(_, qualified)| *qualified == entry)
                .map(|(key, _)| key);
            match twin {
                Some(key) => {
                    primary.insert(name.as_str(), key.as_str());
                }
                None => {
                    unmatched_types.insert(name.as_str(), WireEntry::of(entry, true));
                }
            }
        }
        RegistryWire {
            version: WIRE_VERSION,
            entries,
            primary,
            unmatched_types,
            ambiguous_names: &registry.ambiguous_names.names.in_order,
            file_imports,
        }
    }
}

impl<'a> WireEntry<'a> {
    fn of(entry: &'a TypeRegistryEntry, inline_imports: bool) -> Self {
        WireEntry {
            name: &entry.name,
            file_path: &entry.file_path,
            is_exported: entry.is_exported,
            exported_as: entry.exported_as.as_slice(),
            re_exported: entry.re_exported.as_slice(),
            definition: &entry.definition,
            file_imports: inline_imports.then_some(entry.file_imports.as_slice()),
        }
    }
}

/// The serialized form of a registry that refers to a resident one.
#[derive(Serialize)]
struct ResidentReference {
    resident: RegistryGeneration,
}

impl Serialize for TypeRegistry {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.resident_reference {
            Some(generation) => ResidentReference {
                resident: generation,
            }
            .serialize(serializer),
            None => RegistryWire::of(self).serialize(serializer),
        }
    }
}

/// Reads a registry's contents, or a reference to a resident registry, in
/// one pass, without buffering the contents to try one shape and then the
/// other.
struct RegistryVisitor;

impl<'de> serde::de::Visitor<'de> for RegistryVisitor {
    type Value = TypeRegistry;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a type registry, or a reference to a resident one")
    }

    fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<TypeRegistry, A::Error> {
        use serde::de::Error;

        let mut types = None;
        let mut qualified_types = None;
        let mut ambiguous_names = None;
        let mut resident = None;
        let mut version = None;
        let mut entries: Option<HashMap<String, WireEntryData>> = None;
        let mut primary: Option<HashMap<String, String>> = None;
        let mut unmatched_types: Option<HashMap<String, WireEntryData>> = None;
        let mut file_imports: Option<HashMap<String, Vec<FileImportEntry>>> = None;
        while let Some(key) = map.next_key::<String>()? {
            // A registry in another wire version is refused below; its other
            // fields need not, and may not, parse as this version's.
            if version.is_some_and(|written| written != WIRE_VERSION) {
                map.next_value::<serde::de::IgnoredAny>()?;
                continue;
            }
            match key.as_str() {
                "types" => types = Some(map.next_value()?),
                "qualified_types" => qualified_types = Some(map.next_value()?),
                "ambiguous_names" => ambiguous_names = Some(map.next_value()?),
                "resident" => resident = Some(map.next_value::<RegistryGeneration>()?),
                "version" => version = Some(map.next_value::<u32>()?),
                "entries" => entries = Some(map.next_value()?),
                "primary" => primary = Some(map.next_value()?),
                "unmatched_types" => unmatched_types = Some(map.next_value()?),
                "file_imports" => file_imports = Some(map.next_value()?),
                _ => {
                    map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }
        if let Some(generation) = resident {
            return resident_registry(generation).map_err(A::Error::custom);
        }
        match version {
            Some(WIRE_VERSION) => {
                let file_imports = file_imports.unwrap_or_default();
                let qualified: HashMap<String, TypeRegistryEntry> = entries
                    .ok_or_else(|| A::Error::missing_field("entries"))?
                    .into_iter()
                    .map(|(key, entry)| Ok((key, entry.into_entry(&file_imports)?)))
                    .collect::<Result<_, String>>()
                    .map_err(A::Error::custom)?;
                let mut types: HashMap<String, TypeRegistryEntry> = primary
                    .ok_or_else(|| A::Error::missing_field("primary"))?
                    .into_iter()
                    .map(|(name, key)| {
                        qualified
                            .get(&key)
                            .cloned()
                            .map(|entry| (name, entry))
                            .ok_or_else(|| format!("`{key}` names no registry entry"))
                    })
                    .collect::<Result<_, String>>()
                    .map_err(A::Error::custom)?;
                for (name, entry) in unmatched_types.unwrap_or_default() {
                    types.insert(
                        name,
                        entry.into_entry(&file_imports).map_err(A::Error::custom)?,
                    );
                }
                return Ok(TypeRegistry::from_parts(
                    types,
                    qualified,
                    ambiguous_names.ok_or_else(|| A::Error::missing_field("ambiguous_names"))?,
                ));
            }
            Some(other) => {
                return Err(A::Error::custom(format!(
                    "the type registry is in wire version {other}, which this release cannot read"
                )));
            }
            None => {}
        }
        Ok(TypeRegistry::from_parts(
            types.ok_or_else(|| A::Error::missing_field("types"))?,
            qualified_types.ok_or_else(|| A::Error::missing_field("qualified_types"))?,
            ambiguous_names.ok_or_else(|| A::Error::missing_field("ambiguous_names"))?,
        ))
    }
}

impl<'de> Deserialize<'de> for TypeRegistry {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(RegistryVisitor)
    }
}

/// Equal when every lookup answers the same. The read log is not part of the
/// registry's contents, so it is not compared.
impl PartialEq for TypeRegistry {
    fn eq(&self, other: &Self) -> bool {
        self.types == other.types
            && self.qualified_types == other.qualified_types
            && self.ambiguous_names == other.ambiguous_names
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
        // Every empty map is the same contents, so they share version 0 and a
        // default registry keeps one generation however often it is built.
        let version_of = |empty: bool| if empty { 0 } else { next_version() };
        Self {
            types: RegistryMap {
                version: version_of(types.is_empty()),
                entries: Arc::new(MapEntries::new(types)),
                keys: MapKeys::Names,
                log: log.clone(),
            },
            qualified_types: RegistryMap {
                version: version_of(qualified_types.is_empty()),
                entries: Arc::new(MapEntries::new(qualified_types)),
                keys: MapKeys::Qualified,
                log: log.clone(),
            },
            ambiguous_names: AmbiguousNames {
                version: version_of(ambiguous_names.is_empty()),
                names: Arc::new(NameList::new(ambiguous_names)),
                log: log.clone(),
            },
            log,
            resident_reference: None,
            declaring: None,
        }
    }

    /// This registry as the file at `file_path` reads it while it is being
    /// expanded: a name of `own`, what the file declares, is the file's own
    /// type and none another file declares under it, its scanned entry where
    /// the scan read the file and the declaration in `own` where it did not,
    /// as a component or a file newer than the scan.
    pub fn declaring(
        &self,
        file_path: &str,
        own: impl IntoIterator<Item = TypeRegistryEntry>,
    ) -> TypeRegistry {
        TypeRegistry {
            declaring: Some(Arc::new(DeclaringFile {
                file_path: file_path.to_string(),
                own: own
                    .into_iter()
                    .map(|entry| (entry.name.clone(), entry))
                    .collect(),
            })),
            ..self.clone()
        }
    }

    /// Whether the registry was told what the file at `file_path` declares:
    /// then a name the file neither declares nor resolves through an import
    /// or the project is none of the project's types.
    pub fn knows_declarations_of(&self, file_path: &str) -> bool {
        self.declaring
            .as_ref()
            .is_some_and(|declaring| same_file(&declaring.file_path, file_path))
    }

    /// This registry's generation: what a host installs it under in a macro
    /// guest, and what a reference to it names.
    pub fn generation(&self) -> RegistryGeneration {
        RegistryGeneration([
            self.types.version,
            self.qualified_types.version,
            self.ambiguous_names.version,
        ])
    }

    /// A copy of this registry that serializes as a reference to its
    /// generation instead of its contents, for a host to send to a guest it
    /// has installed this generation in.
    pub fn resident_reference(&self) -> TypeRegistry {
        TypeRegistry {
            resident_reference: Some(self.generation()),
            ..self.clone()
        }
    }

    /// Starts recording the lookups made through this registry and its clones,
    /// discarding any recorded so far.
    pub fn start_recording(&self) {
        self.log.start();
    }

    /// Stops recording and returns what was recorded, or `None` if recording
    /// was never started.
    pub fn finish_recording(&self) -> Option<BTreeSet<RegistryRead>> {
        self.log.finish()
    }

    /// Adds the lookups a macro reported making against its own copy of the
    /// registry. A macro that reports none was built before lookups were
    /// recorded, so it may have read anything.
    pub fn record_macro_reads(&self, reported: Option<&BTreeSet<RegistryRead>>) {
        match reported {
            Some(reported) => self.log.extend(reported.iter().cloned()),
            None => self.log.extend([RegistryRead::All]),
        }
    }

    /// This registry's contents with a record of its own: lookups through the
    /// copy, and through every clone of it, are recorded apart from this
    /// registry's. Recording starts off.
    pub fn with_fresh_log(&self) -> TypeRegistry {
        let log = ReadLog::default();
        Self {
            types: RegistryMap {
                entries: self.types.entries.clone(),
                keys: self.types.keys,
                log: log.clone(),
                version: self.types.version,
            },
            qualified_types: RegistryMap {
                entries: self.qualified_types.entries.clone(),
                keys: self.qualified_types.keys,
                log: log.clone(),
                version: self.qualified_types.version,
            },
            ambiguous_names: AmbiguousNames {
                names: self.ambiguous_names.names.clone(),
                log: log.clone(),
                version: self.ambiguous_names.version,
            },
            log,
            resident_reference: self.resident_reference,
            declaring: self.declaring.clone(),
        }
    }

    /// What `read` currently resolves to. Unlike the lookups themselves, this
    /// records nothing.
    pub fn resolution_of(&self, read: &RegistryRead) -> Resolution {
        match read {
            RegistryRead::Name(name) => Resolution::Name {
                primary: self
                    .types
                    .entries
                    .by_key
                    .get(name)
                    .map(TypeRegistryEntry::without_spans),
                ambiguous: self.ambiguous_names.has(name),
                definitions: without_spans(self.qualified_types.entries.named(name)),
            },
            RegistryRead::Qualified(key) => Resolution::Qualified(
                self.qualified_types
                    .entries
                    .by_key
                    .get(key)
                    .map(TypeRegistryEntry::without_spans),
            ),
            RegistryRead::All => Resolution::All {
                types: without_spans(self.types.entries.by_key.iter()),
                qualified_types: without_spans(self.qualified_types.entries.by_key.iter()),
                ambiguous_names: self.ambiguous_names.names.members.iter().cloned().collect(),
            },
            RegistryRead::ExportedAs(name) => Resolution::ExportedAs(without_spans(
                self.qualified_types
                    .entries
                    .by_key
                    .iter()
                    .filter(|(_, entry)| entry.is_exported_as(name)),
            )),
            RegistryRead::Module(module_path) => {
                Resolution::Module(self.module_holds_types(module_path))
            }
        }
    }

    /// Look up a type by simple name. Returns `None` if the name is ambiguous
    /// (exists in multiple files). Callers must use `resolve()` with import
    /// context or `get_qualified()` for file-specific resolution.
    pub fn get(&self, name: &str) -> Option<&TypeRegistryEntry> {
        self.log.note(|| RegistryRead::Name(name.to_string()));
        if self.ambiguous_names.has(name) {
            None
        } else {
            self.types.entries.by_key.get(name)
        }
    }

    /// The chain of type aliases from `name`, as the file at `file_path`
    /// knows it, back to one already on it, such as `["A", "B", "A"]`, if
    /// there is one. Each alias's references are read as its own file knows
    /// them. TypeScript rejects circular aliases; code that follows alias
    /// references needs to know, or it recurses without end.
    pub fn alias_cycle(
        &self,
        name: &str,
        file_path: &str,
        file_imports: &[FileImportEntry],
    ) -> Option<Vec<String>> {
        self.find_alias_cycle(name, file_path, file_imports, &mut Vec::new())
    }

    fn find_alias_cycle<'a>(
        &'a self,
        name: &str,
        file_path: &str,
        file_imports: &[FileImportEntry],
        path: &mut Vec<(&'a TypeRegistryEntry, String)>,
    ) -> Option<Vec<String>> {
        let entry = self.resolve_in_file(name, file_path, file_imports)?;
        if let Some(start) = path
            .iter()
            .position(|(visited, _)| std::ptr::eq(*visited, entry))
        {
            let mut cycle: Vec<String> = path
                .iter()
                .skip(start)
                .map(|(_, written)| written.clone())
                .collect();
            cycle.push(name.to_string());
            return Some(cycle);
        }
        let TypeDefinitionIR::TypeAlias(alias) = &entry.definition else {
            return None;
        };
        path.push((entry, name.to_string()));
        let found = body_references(&alias.body)
            .into_iter()
            .find_map(|referenced| {
                self.find_alias_cycle(referenced, &entry.file_path, &entry.file_imports, path)
            });
        path.pop();
        found
    }

    /// Get all qualified entries matching a simple type name, in
    /// qualified-key order: the definitions in different files that share it.
    pub fn get_all(&self, name: &str) -> impl Iterator<Item = &TypeRegistryEntry> {
        self.log.note(|| RegistryRead::Name(name.to_string()));
        self.qualified_types
            .entries
            .named(name)
            .map(|(_, entry)| entry)
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
    /// qualified entry whose `file_path` equals `caller_file_path`: i.e. the
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
        self.log.note(|| RegistryRead::Name(name.to_string()));
        // A name the calling file declares itself is that file's type,
        // whether the scan has read the file or not.
        if let Some(declaring) = &self.declaring
            && let Some(own) = declaring.own.get(name)
            && same_file(&declaring.file_path, caller_file_path)
        {
            return self
                .candidates(name)
                .into_iter()
                .find(|entry| same_file(&entry.file_path, caller_file_path))
                .or(Some(own));
        }
        let import = file_imports.iter().find(|import| import.local_name == name);
        // What the module a name is imported from exports it as: the name
        // itself, another (`import { User as Member }`), or `default`. A
        // namespace import names no export.
        let exported = import
            .map(|import| import.original_name.as_deref().unwrap_or(name))
            .filter(|exported| *exported != "*");
        if let (Some(import), Some(exported)) = (import, exported) {
            let specifier = import.module_specifier.trim();
            if !(specifier.starts_with("./") || specifier.starts_with("../")) {
                // A specifier that is not relative names a module of the
                // project by the end of its path (`$lib/types/user`), or a
                // package. A package's types are none of the project's,
                // whatever the project declares under their names.
                return self.exported_by_specifier(specifier, exported);
            }
            // A relative specifier names one module of the project.
            if let Some(module_path) = resolve_relative_module(caller_file_path, specifier) {
                let is_module = |file_path: &str| file_path_is_module(file_path, &module_path);
                if let Some(entry) = self.exported_by(&is_module, exported).into_iter().next() {
                    return Some(entry);
                }
                // A module the scan read types in exports those it was seen
                // to, and none the project declares elsewhere under the name.
                self.log.note(|| RegistryRead::Module(module_path.clone()));
                if self.module_holds_types(&module_path) {
                    return None;
                }
            }
        }
        // A name imported under another is the type its module exports,
        // never one declared elsewhere under the local name.
        let renamed = exported.filter(|exported| *exported != "default" && *exported != name);
        if let Some(exported) = renamed {
            self.log.note(|| RegistryRead::Name(exported.to_string()));
        }
        // Fast path: unambiguous name.
        if !self.ambiguous_names.has(renamed.unwrap_or(name)) {
            return self.types.entries.by_key.get(renamed.unwrap_or(name));
        }
        // Same-file resolution: the caller and the declaration share a file,
        // so the entry whose file_path matches the caller is canonical here
        // even when the simple name is ambiguous globally.
        if renamed.is_none()
            && !caller_file_path.is_empty()
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

    /// The declaration a module named by a specifier that is not relative
    /// exports as `exported`. The longest end of the specifier that is the
    /// end of a module's path names the module (`$lib/types/user` is
    /// `src/lib/types/user.ts`), and only when one declaration answers to
    /// it: two are no answer, and none means the specifier names a package.
    fn exported_by_specifier(&self, specifier: &str, exported: &str) -> Option<&TypeRegistryEntry> {
        for ending in specifier_endings(specifier) {
            let found = self.exported_by(
                &|file_path| file_path_ends_in_module(file_path, &ending),
                exported,
            );
            match found.as_slice() {
                [] => continue,
                [entry] => return Some(entry),
                [_, _, ..] => return None,
            }
        }
        None
    }

    /// The declarations a module whose file `is_module` takes exports as
    /// `exported`: one declared under that name, one it exports under it
    /// (`export { User as Member }`, `export default class User`), or one
    /// of another module that it exports again under it. Each once, those
    /// declared under the name first.
    fn exported_by(
        &self,
        is_module: &dyn Fn(&str) -> bool,
        exported: &str,
    ) -> Vec<&TypeRegistryEntry> {
        let exports_again = |entry: &TypeRegistryEntry| {
            entry
                .re_exported
                .iter()
                .any(|again| again.name == exported && is_module(&again.file_path))
        };
        // A type keeps its name through most modules, so the declarations
        // of that name are asked first.
        let named = self.candidates(exported);
        let mut found: Vec<&TypeRegistryEntry> = named
            .iter()
            .filter(|entry| is_module(&entry.file_path))
            .chain(named.iter().filter(|entry| exports_again(entry)))
            .copied()
            .collect();
        if found.is_empty() {
            self.log
                .note(|| RegistryRead::ExportedAs(exported.to_string()));
            // Qualified keys are ordered, so two declarations a module
            // exports under one name come in the same order each time.
            let mut renamed: Vec<(&String, &TypeRegistryEntry)> = self
                .qualified_types
                .entries
                .by_key
                .iter()
                .filter(|(_, entry)| {
                    exports_again(entry)
                        || (entry.exported_as.iter().any(|name| name == exported)
                            && is_module(&entry.file_path))
                })
                .collect();
            renamed.sort_by_key(|(key, _)| *key);
            found.extend(renamed.into_iter().map(|(_, entry)| entry));
        }
        let mut distinct: Vec<&TypeRegistryEntry> = Vec::with_capacity(found.len());
        for entry in found {
            if !distinct.iter().any(|kept| std::ptr::eq(*kept, entry)) {
                self.log.note(|| RegistryRead::Name(entry.name.clone()));
                distinct.push(entry);
            }
        }
        distinct
    }

    /// Whether the module at `module_path`, written without its extension,
    /// declares a type or exports one again: a module the scan read, and
    /// found types in.
    fn module_holds_types(&self, module_path: &str) -> bool {
        let is_module = |file_path: &str| file_path_is_module(file_path, module_path);
        self.qualified_types.entries.by_key.values().any(|entry| {
            is_module(&entry.file_path)
                || entry
                    .re_exported
                    .iter()
                    .any(|again| is_module(&again.file_path))
        })
    }

    /// Every definition named `name`, in qualified-key order, so a lookup
    /// never depends on hash map iteration order.
    fn candidates(&self, name: &str) -> Vec<&TypeRegistryEntry> {
        self.log.note(|| RegistryRead::Name(name.to_string()));
        self.qualified_types
            .entries
            .named(name)
            .map(|(_, entry)| entry)
            .collect()
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

        if self.types.entries.by_key.contains_key(&entry.name) {
            if !self.ambiguous_names.has(&entry.name) {
                self.ambiguous_names.version = next_version();
                Arc::make_mut(&mut self.ambiguous_names.names).push(entry.name.clone());
            }
        } else {
            self.types.insert(entry.name.clone(), entry.clone());
        }

        self.qualified_types.insert(qualified, entry);
    }

    /// Get the number of types registered. Depends on the whole registry.
    pub fn len(&self) -> usize {
        self.log.note(|| RegistryRead::All);
        self.qualified_types.entries.by_key.len()
    }

    /// Check if the registry is empty. Depends on the whole registry.
    pub fn is_empty(&self) -> bool {
        self.log.note(|| RegistryRead::All);
        self.qualified_types.entries.by_key.is_empty()
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

/// The ends of a specifier that is not relative, longest first, each as a
/// path a module's own may end in: `$lib/types/user` gives `lib/types/user`,
/// `types/user` and `user`. The marks an alias opens with (`$`, `@`, `~`,
/// `#`) are no part of a path.
pub fn specifier_endings(specifier: &str) -> Vec<String> {
    let segments: Vec<&str> = specifier
        .trim()
        .split('/')
        .map(|segment| segment.trim_start_matches(['$', '@', '~', '#']))
        .filter(|segment| !segment.is_empty())
        .collect();
    (0..segments.len())
        .filter_map(|start| segments.get(start..))
        .map(|ending| strip_source_extension(&ending.join("/")).to_string())
        .collect()
}

/// Whether the module `file_path` holds is one a path ending in `ending`
/// names: the file itself, its directory when it is an `index`, or the file
/// without a further extension of its own (`user.svelte.ts` for `user`).
pub fn file_path_ends_in_module(file_path: &str, ending: &str) -> bool {
    let file_path = slashed(file_path);
    let trimmed = strip_source_extension(&file_path);
    let ends = |stem: &str| {
        stem == ending
            || stem
                .strip_suffix(ending)
                .is_some_and(|before| before.ends_with('/'))
    };
    let base_start = trimmed.rfind('/').map_or(0, |slash| slash + 1);
    let without_extensions = trimmed
        .char_indices()
        .filter(|(index, letter)| *index >= base_start && *letter == '.')
        .filter_map(|(index, _)| trimmed.get(..index));
    ends(trimmed)
        || trimmed.strip_suffix("/index").is_some_and(ends)
        || without_extensions.into_iter().any(ends)
}

/// Whether the module `specifier` names in the file at `caller_file_path`
/// gives the generated functions of the type `entry` declares along with
/// the type: its declaring module, or one that exports all of that module
/// again. A module that exports the type again by name gives the type alone.
pub fn module_gives_functions(
    entry: &TypeRegistryEntry,
    caller_file_path: &str,
    specifier: &str,
) -> bool {
    let is_module: Box<dyn Fn(&str) -> bool> =
        match resolve_relative_module(caller_file_path, specifier) {
            Some(module_path) => {
                Box::new(move |file_path| file_path_is_module(file_path, &module_path))
            }
            None => {
                let endings = specifier_endings(specifier);
                Box::new(move |file_path| {
                    endings
                        .iter()
                        .any(|ending| file_path_ends_in_module(file_path, ending))
                })
            }
        };
    is_module(&entry.file_path)
        || entry
            .re_exported
            .iter()
            .any(|again| again.whole && is_module(&again.file_path))
}

/// The extension-less path a relative specifier resolves to from the file at
/// `caller_file_path`, or `None` for a bare or aliased specifier.
pub fn resolve_relative_module(caller_file_path: &str, module_specifier: &str) -> Option<String> {
    let specifier = module_specifier.trim();
    if caller_file_path.is_empty() || !(specifier.starts_with("./") || specifier.starts_with("../"))
    {
        return None;
    }
    let caller_file_path = slashed(caller_file_path);
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
pub fn file_path_is_module(file_path: &str, module_path: &str) -> bool {
    let file_path = slashed(file_path);
    let trimmed = strip_source_extension(&file_path);
    trimmed == module_path
        || trimmed
            .strip_prefix(module_path)
            .is_some_and(|rest| rest.starts_with('.') || rest == "/index")
}

/// Whether `file_path` (an absolute on-disk path) corresponds to a module
/// whose import basename matches `needle`. Strips one source extension from
/// the file's basename, then accepts an exact match or a prefix match
/// followed by a `.` so a needle like `all-types` (extension-less import)
/// still matches `all-types.svelte.ts` (Svelte component output): the
/// `.svelte` suffix counts as a sub-extension on the same module.
fn file_path_module_matches(file_path: &str, needle: &str) -> bool {
    let file_path = slashed(file_path);
    let trimmed = strip_source_extension(file_path.rsplit('/').next().unwrap_or(&file_path));
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

/// The types an alias body names directly, for following alias chains.
fn body_references(body: &TypeBody) -> Vec<&str> {
    match body {
        TypeBody::Alias(target) => alias_reference(target).into_iter().collect(),
        TypeBody::Union(members) | TypeBody::Intersection(members) => members
            .iter()
            .filter_map(|member| match &member.kind {
                TypeMemberKind::TypeRef(referenced) => Some(referenced.as_str()),
                TypeMemberKind::Literal(_)
                | TypeMemberKind::Object { .. }
                | TypeMemberKind::Intersection(_)
                | TypeMemberKind::Brand(_) => None,
            })
            .collect(),
        TypeBody::Newtype(inner) => body_references(inner),
        TypeBody::Object { .. } | TypeBody::Tuple(_) | TypeBody::Other(_) => Vec::new(),
    }
}

/// The type an alias body names directly: `Wrap` for `Wrap<A>`, `A` for `A`.
/// An array (`A[]`) or any other form names nothing, since TypeScript accepts
/// an alias that recurses through one.
fn alias_reference(target: &str) -> Option<&str> {
    let trimmed = target.trim();
    let name = match trimmed.find('<') {
        Some(open) if trimmed.ends_with('>') => trimmed.get(..open)?.trim_end(),
        Some(_) => return None,
        None => trimmed,
    };
    (!name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$' || c == '.'))
    .then_some(name)
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
            exported_as: Vec::new(),
            re_exported: Vec::new(),
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
            exported_as: Vec::new(),
            re_exported: Vec::new(),
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
            registry.alias_cycle("A", "/project/src/aliases.ts", &[]),
            Some(vec!["A".to_string(), "B".to_string(), "A".to_string()])
        );
        assert_eq!(
            registry.alias_cycle("C", "/project/src/aliases.ts", &[]),
            None
        );
    }

    #[test]
    fn alias_cycle_follows_a_generic_reference_but_not_an_array() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_alias_entry("Wrap", TypeBody::Alias("Loop<string>".to_string())),
            "/project",
        );
        registry.insert(
            make_alias_entry("Loop", TypeBody::Alias("Wrap".to_string())),
            "/project",
        );
        registry.insert(
            make_alias_entry("Tree", TypeBody::Alias("Tree[]".to_string())),
            "/project",
        );

        assert_eq!(
            registry.alias_cycle("Wrap", "/project/src/aliases.ts", &[]),
            Some(vec![
                "Wrap".to_string(),
                "Loop".to_string(),
                "Wrap".to_string()
            ])
        );
        assert_eq!(
            registry.alias_cycle("Tree", "/project/src/aliases.ts", &[]),
            None
        );
    }

    /// A loop is found among aliases whose names other files declare too,
    /// and an alias that names another file's type of its own name is none.
    #[test]
    fn alias_cycle_reads_each_alias_in_its_own_file() {
        let in_file = |name: &str, file_path: &str, target: &str, imports: Vec<FileImportEntry>| {
            TypeRegistryEntry {
                file_path: file_path.to_string(),
                file_imports: imports,
                ..make_alias_entry(name, TypeBody::Alias(target.to_string()))
            }
        };
        let mut registry = TypeRegistry::new();
        for file_path in ["/project/src/one.ts", "/project/src/two.ts"] {
            registry.insert(in_file("Left", file_path, "Right", vec![]), "/project");
            registry.insert(in_file("Right", file_path, "Left", vec![]), "/project");
        }
        // `Id` here is the other module's `Id`, which is a string.
        let renamed = FileImportEntry {
            local_name: "Theirs".to_string(),
            module_specifier: "./ids".to_string(),
            original_name: Some("Id".to_string()),
            is_type_only: true,
        };
        registry.insert(
            in_file("Id", "/project/src/page.ts", "Theirs", vec![renamed]),
            "/project",
        );
        registry.insert(
            in_file("Id", "/project/src/ids.ts", "string", vec![]),
            "/project",
        );
        registry.insert(
            in_file("Theirs", "/project/src/other.ts", "Id", vec![]),
            "/project",
        );

        assert_eq!(
            registry.alias_cycle("Left", "/project/src/two.ts", &[]),
            Some(vec![
                "Left".to_string(),
                "Right".to_string(),
                "Left".to_string()
            ])
        );
        let page = registry
            .get_all("Id")
            .find(|entry| entry.file_path == "/project/src/page.ts")
            .map(|entry| entry.file_imports.clone())
            .unwrap_or_default();
        assert_eq!(
            registry.alias_cycle("Id", "/project/src/page.ts", &page),
            None
        );
    }

    /// A name a file imports under another is the type its module exports,
    /// whatever else the project declares under the local name.
    #[test]
    fn a_renamed_import_resolves_to_the_type_it_was_exported_as() {
        let mut registry = TypeRegistry::default();
        let user = TypeRegistryEntry {
            exported_as: vec!["default".to_string()],
            ..make_interface_entry("User", "/project/src/user.ts", vec![])
        };
        registry.insert(user, "/project");
        registry.insert(
            make_interface_entry("Member", "/project/src/club.ts", vec![]),
            "/project",
        );
        let renamed = [FileImportEntry {
            local_name: "Member".to_string(),
            module_specifier: "./user".to_string(),
            original_name: Some("User".to_string()),
            is_type_only: true,
        }];
        let resolved = registry.resolve_in_file("Member", "/project/src/page.ts", &renamed);
        assert_eq!(resolved.map(|entry| entry.name.as_str()), Some("User"));

        // Without the import, the name is the type declared under it.
        let declared = registry.resolve_in_file("Member", "/project/src/page.ts", &[]);
        assert_eq!(
            declared.map(|entry| entry.file_path.as_str()),
            Some("/project/src/club.ts")
        );

        // A default import says nothing of the name it was declared under.
        let defaulted = [FileImportEntry {
            local_name: "User".to_string(),
            module_specifier: "./user".to_string(),
            original_name: Some("default".to_string()),
            is_type_only: false,
        }];
        let resolved = registry.resolve_in_file("User", "/project/src/page.ts", &defaulted);
        assert_eq!(resolved.map(|entry| entry.name.as_str()), Some("User"));
    }

    /// A name a file declares is that file's type: its scanned entry where
    /// the scan read the file, and the declaration the file holds where it
    /// did not.
    #[test]
    fn a_name_a_file_declares_is_not_another_files_type() {
        let mut registry = TypeRegistry::default();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("Page", "/project/src/page.ts", vec![]),
            "/project",
        );
        let resolve = |registry: &TypeRegistry, name: &str, file: &str| {
            registry
                .resolve_in_file(name, file, &[])
                .map(|entry| entry.file_path.clone())
        };

        // Unsaid, a name is the one type the project declares under it.
        let component = "/project/src/routes/+page.svelte";
        assert_eq!(
            resolve(&registry, "User", component).as_deref(),
            Some("/project/src/user.ts")
        );

        // A component the scan did not read declares a `User` of its own.
        let unscanned =
            registry.declaring(component, [make_interface_entry("User", component, vec![])]);
        assert_eq!(
            resolve(&unscanned, "User", component).as_deref(),
            Some(component)
        );
        assert_eq!(
            resolve(&unscanned, "Page", component).as_deref(),
            Some("/project/src/page.ts")
        );
        // What it says holds for the file it is said of.
        assert_eq!(
            resolve(&unscanned, "User", "/project/src/other.ts").as_deref(),
            Some("/project/src/user.ts")
        );

        // A scanned file's own entry is found by either spelling of its path.
        let scanned = registry.declaring(
            "src/page.ts",
            [make_interface_entry("Page", "src/page.ts", vec![])],
        );
        assert_eq!(
            resolve(&scanned, "Page", "src/page.ts").as_deref(),
            Some("/project/src/page.ts")
        );
        assert!(!same_file(
            "/project/src/page.ts",
            "/project/src/homepage.ts"
        ));
    }

    /// A path written with the separators of Windows names the module a
    /// path written with `/` does.
    #[test]
    fn a_windows_path_names_the_module_a_slashed_one_does() {
        let mut registry = TypeRegistry::default();
        registry.insert(
            make_interface_entry("User", r"C:\project\src\models\user.ts", vec![]),
            r"C:\project",
        );
        registry.insert(
            make_interface_entry("User", r"C:\project\src\other.ts", vec![]),
            r"C:\project",
        );
        let page = r"C:\project\src\routes\page.ts";
        let resolve = |specifier: &str| {
            registry
                .resolve_in_file("User", page, &[import_from("User", specifier)])
                .map(|entry| entry.file_path.as_str())
        };

        assert_eq!(
            resolve("../models/user"),
            Some(r"C:\project\src\models\user.ts")
        );
        assert_eq!(resolve("../other"), Some(r"C:\project\src\other.ts"));
        assert_eq!(
            resolve("$lib/models/user"),
            Some(r"C:\project\src\models\user.ts")
        );
        assert_eq!(
            resolve_relative_module(page, "../models/user").as_deref(),
            Some("C:/project/src/models/user")
        );
        assert!(same_file(
            r"src\routes\page.ts",
            "C:/project/src/routes/page.ts"
        ));
    }

    /// A module the scan found types in exports those and no others: a name
    /// imported from it is not the type another module declares under it.
    #[test]
    fn a_name_a_known_module_does_not_export_is_no_type_declared_elsewhere() {
        let mut registry = TypeRegistry::default();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("Other", "/project/src/other.ts", vec![]),
            "/project",
        );
        let barrel = TypeRegistryEntry {
            re_exported: vec![ReExport {
                file_path: "/project/src/barrel.ts".to_string(),
                name: "Shared".to_string(),
                whole: false,
            }],
            ..make_interface_entry("Shared", "/project/src/shared.ts", vec![])
        };
        registry.insert(barrel, "/project");
        let resolve = |module: &str| {
            let imports = [import_from("User", module)];
            registry
                .resolve_in_file("User", "/project/src/page.ts", &imports)
                .map(|entry| entry.file_path.as_str())
        };

        assert_eq!(resolve("./user"), Some("/project/src/user.ts"));
        // A module that declares types, and one that only exports them again.
        assert_eq!(resolve("./other"), None);
        assert_eq!(resolve("./barrel"), None);
        // A module the scan read no types in is not known to export none.
        assert_eq!(resolve("./unread"), Some("/project/src/user.ts"));
    }

    /// A specifier that is not relative names a module of the project by the
    /// end of its path, and a package otherwise: a package's type is no type
    /// of the project, though the project declares one of its name.
    #[test]
    fn a_name_imported_from_a_package_is_no_type_of_the_project() {
        let mut registry = TypeRegistry::default();
        registry.insert(
            make_interface_entry("User", "/project/src/lib/types/user.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry(
                "Session",
                "/project/src/lib/types/session.svelte.ts",
                vec![],
            ),
            "/project",
        );
        registry.insert(
            make_interface_entry("Config", "/project/src/lib/config/index.ts", vec![]),
            "/project",
        );
        let resolve = |name: &str, module: &str| {
            let imports = [FileImportEntry {
                local_name: name.to_string(),
                module_specifier: module.to_string(),
                original_name: None,
                is_type_only: true,
            }];
            registry
                .resolve_in_file(name, "/project/src/routes/page.ts", &imports)
                .map(|entry| entry.file_path.clone())
        };

        // An alias into the project, by the whole path, a part of it, and a directory.
        assert_eq!(
            resolve("User", "$lib/types/user").as_deref(),
            Some("/project/src/lib/types/user.ts")
        );
        assert_eq!(
            resolve("User", "@/lib/types/user").as_deref(),
            Some("/project/src/lib/types/user.ts")
        );
        assert_eq!(
            resolve("User", "~types/user.ts").as_deref(),
            Some("/project/src/lib/types/user.ts")
        );
        assert_eq!(
            resolve("Session", "$lib/types/session").as_deref(),
            Some("/project/src/lib/types/session.svelte.ts")
        );
        assert_eq!(
            resolve("Config", "$lib/config").as_deref(),
            Some("/project/src/lib/config/index.ts")
        );

        // A package, though the project declares the one `User` there is.
        assert_eq!(resolve("User", "some-auth"), None);
        assert_eq!(resolve("User", "@auth/core"), None);
        assert_eq!(resolve("Config", "effect/Config"), None);
        // A path that ends like the module's name without being it.
        assert_eq!(resolve("User", "other/superuser"), None);
    }

    /// An import is the declaration its module exports under the imported
    /// name: one exported under another name, or as the default.
    #[test]
    fn an_import_resolves_by_the_name_its_module_exports_a_type_as() {
        let exported_as = |name: &str, file_path: &str, exported: &[&str]| TypeRegistryEntry {
            exported_as: exported.iter().map(|name| name.to_string()).collect(),
            ..make_interface_entry(name, file_path, vec![])
        };
        let mut registry = TypeRegistry::default();
        registry.insert(
            exported_as("Late", "/project/src/models.ts", &["Shown"]),
            "/project",
        );
        registry.insert(
            exported_as("Model", "/project/src/models.ts", &["default"]),
            "/project",
        );
        registry.insert(
            exported_as("Shown", "/project/src/other.ts", &[]),
            "/project",
        );
        let import = |local: &str, exported: Option<&str>| FileImportEntry {
            local_name: local.to_string(),
            module_specifier: "./models".to_string(),
            original_name: exported.map(str::to_string),
            is_type_only: true,
        };
        let resolve = |name: &str, imports: &[FileImportEntry]| {
            registry
                .resolve_in_file(name, "/project/src/page.ts", imports)
                .map(|entry| (entry.name.as_str(), entry.file_path.as_str()))
        };

        // `export type { Late as Shown }`, though another module declares a `Shown`.
        assert_eq!(
            resolve("Shown", &[import("Shown", None)]),
            Some(("Late", "/project/src/models.ts"))
        );
        // The same export, renamed again on import.
        assert_eq!(
            resolve("Seen", &[import("Seen", Some("Shown"))]),
            Some(("Late", "/project/src/models.ts"))
        );
        // `export default class Model`, imported under a name of the file's own.
        assert_eq!(
            resolve("Anything", &[import("Anything", Some("default"))]),
            Some(("Model", "/project/src/models.ts"))
        );
    }

    #[test]
    fn get_returns_none_for_ambiguous_names() {
        let mut registry = TypeRegistry::new();

        let entry1 = make_interface_entry(
            "PhoneNumber",
            "/project/src/types/phone-number.svelte.ts",
            vec![derive_decorator("Default, Encode, Decode, Gigaform")],
        );
        let entry2 = make_interface_entry(
            "PhoneNumber",
            "/project/src/types/all-types.svelte.ts",
            vec![derive_decorator("Default, Encode, Decode, Gigaform")],
        );

        registry.insert(entry1, "/project");
        registry.insert(entry2, "/project");

        // Name should be ambiguous
        assert!(registry.ambiguous_names.contains("PhoneNumber"));

        // get() returns None for ambiguous names: callers must use resolve() or get_all()
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

        // No imports provided, so the ambiguous name cannot be resolved
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

    /// A lookup that finds no declaration of a name in a module depends on
    /// what is exported under the name, and on whether the module holds
    /// types: each changes what the lookup would find, and each is recorded.
    #[test]
    fn a_lookup_through_an_export_name_or_a_module_records_what_it_asked() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("Late", "/project/src/models.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let import = |module: &str| [import_from("Shown", module)];
        let page = "/project/src/page.ts";

        registry.start_recording();
        assert!(
            registry
                .resolve_in_file("Shown", page, &import("./models"))
                .is_none()
        );
        let reads = reads_of(&registry);
        let exported = RegistryRead::ExportedAs("Shown".to_string());
        let module = RegistryRead::Module("/project/src/models".to_string());
        assert!(reads.contains(&exported), "{reads:?}");
        assert!(reads.contains(&module), "{reads:?}");

        // A module that gives one of its declarations the name changes the
        // first, and one that gains its first type changes the second.
        let before = registry.resolution_of(&exported);
        let mut renamed = registry.clone();
        renamed.insert(
            TypeRegistryEntry {
                exported_as: vec!["Shown".to_string()],
                ..make_interface_entry("Other", "/project/src/models.ts", vec![])
            },
            "/project",
        );
        assert_ne!(renamed.resolution_of(&exported), before);
        assert_eq!(registry.resolution_of(&exported), before);

        let empty = RegistryRead::Module("/project/src/empty".to_string());
        assert_eq!(registry.resolution_of(&empty), Resolution::Module(false));
        let mut filled = registry.clone();
        filled.insert(
            make_interface_entry("First", "/project/src/empty.ts", vec![]),
            "/project",
        );
        assert_eq!(filled.resolution_of(&empty), Resolution::Module(true));
        assert_eq!(registry.resolution_of(&module), Resolution::Module(true));
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
    fn a_generation_changes_with_every_insert_and_survives_a_clone() {
        let mut registry = TypeRegistry::new();
        let empty = registry.generation();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let first = registry.generation();
        assert_ne!(first, empty);
        assert_eq!(
            TypeRegistry::default().generation(),
            empty,
            "every empty registry is one generation"
        );
        assert_eq!(registry.clone().generation(), first);
        assert_eq!(registry.with_fresh_log().generation(), first);
        // An insert straight through a public map moves the generation too.
        registry.types.insert(
            "Other".to_string(),
            make_interface_entry("Other", "/project/src/other.ts", vec![]),
        );
        assert_ne!(registry.generation(), first);
    }

    #[test]
    fn a_resident_reference_resolves_to_the_installed_registry() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let payload = serde_json::to_string(&ResidentRegistryPayload {
            generation: registry.generation(),
            registry: registry.clone(),
        })
        .expect("the payload serializes");
        install_resident_registry(&payload).expect("the payload installs");

        let reference = serde_json::to_string(&registry.resident_reference())
            .expect("the reference serializes");
        assert!(reference.contains("resident"), "{reference}");
        assert!(
            !reference.contains("User"),
            "a reference carries no contents: {reference}"
        );

        let resolved: TypeRegistry =
            serde_json::from_str(&reference).expect("the reference resolves");
        assert_eq!(resolved, registry);
        resolved.start_recording();
        assert!(resolved.get("User").is_some());
        assert_eq!(
            resolved.finish_recording(),
            Some(BTreeSet::from([RegistryRead::Name("User".to_string())]))
        );
        assert_eq!(
            registry.finish_recording(),
            None,
            "the installed copy records apart"
        );
    }

    #[test]
    fn a_reference_to_a_registry_never_installed_is_an_error() {
        let registry = TypeRegistry::new();
        let mut moved = registry.clone();
        moved.insert(
            make_interface_entry("Never", "/project/src/never.ts", vec![]),
            "/project",
        );
        let reference =
            serde_json::to_string(&moved.resident_reference()).expect("the reference serializes");
        let error = serde_json::from_str::<TypeRegistry>(&reference)
            .expect_err("nothing of that generation is installed");
        assert!(error.to_string().contains("has not installed"), "{error}");
    }

    #[test]
    fn clones_share_storage() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let copy = registry.clone();
        assert!(Arc::ptr_eq(&copy.types.entries, &registry.types.entries));
        assert!(Arc::ptr_eq(
            &copy.qualified_types.entries,
            &registry.qualified_types.entries
        ));
        let scoped = registry.with_fresh_log();
        assert!(Arc::ptr_eq(&scoped.types.entries, &registry.types.entries));
    }

    #[test]
    fn insert_after_clone_does_not_leak_into_the_clone() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let copy = registry.clone();
        registry.insert(
            make_interface_entry("User", "/project/src/other.ts", vec![]),
            "/project",
        );
        assert!(registry.ambiguous_names.contains("User"));
        assert!(!copy.ambiguous_names.contains("User"));
        assert_eq!(copy.qualified_types.len(), 1);
        assert_eq!(registry.qualified_types.len(), 2);
    }

    #[test]
    fn definitions_of_a_name_come_in_qualified_key_order() {
        let mut registry = TypeRegistry::new();
        for file in [
            "/project/src/z.ts",
            "/project/src/a.ts",
            "/project/src/m.ts",
        ] {
            registry.insert(make_interface_entry("User", file, vec![]), "/project");
        }
        let files: Vec<&str> = registry
            .get_all("User")
            .map(|entry| entry.file_path.as_str())
            .collect();
        assert_eq!(
            files,
            [
                "/project/src/a.ts",
                "/project/src/m.ts",
                "/project/src/z.ts"
            ]
        );
        assert_eq!(registry.get_all("Missing").count(), 0);
    }

    #[test]
    fn replacing_a_key_with_another_name_moves_it_in_the_index() {
        let mut registry = TypeRegistry::new();
        registry.qualified_types.insert(
            "src/user.ts::User".to_string(),
            make_interface_entry("User", "/project/src/user.ts", vec![]),
        );
        registry.qualified_types.insert(
            "src/user.ts::User".to_string(),
            make_interface_entry("Account", "/project/src/user.ts", vec![]),
        );
        assert_eq!(registry.get_all("User").count(), 0);
        assert_eq!(registry.get_all("Account").count(), 1);
    }

    #[test]
    fn a_fresh_log_records_apart_from_the_original() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.start_recording();
        let scoped = registry.with_fresh_log();
        assert_eq!(scoped, registry);
        scoped.start_recording();
        assert!(scoped.clone().get("User").is_some());
        assert_eq!(
            scoped.finish_recording(),
            Some(BTreeSet::from([RegistryRead::Name("User".to_string())]))
        );
        assert_eq!(registry.finish_recording(), Some(BTreeSet::new()));
    }

    #[test]
    fn lookups_before_recording_starts_are_not_recorded() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        assert!(registry.get("User").is_some());
        registry.record_macro_reads(None);
        registry.start_recording();
        assert_eq!(registry.finish_recording(), Some(BTreeSet::new()));
        assert_eq!(registry.finish_recording(), None);
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
    fn a_registry_in_an_older_wire_version_is_refused_by_version() {
        // Version 2 wrote type parameters as bare names.
        let json = r#"{"version":2,"entries":{"a.ts::Box":{"type_params":["T"]}},"primary":{}}"#;
        let error = serde_json::from_str::<TypeRegistry>(json)
            .expect_err("an older wire version is refused");
        assert!(
            error.to_string().contains("wire version 2"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn the_serialized_form_is_compact_and_starts_unrecorded() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        let json = serde_json::to_value(&registry).expect("the registry encodes");
        let mut keys: Vec<&String> = json.as_object().expect("an object").keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            [
                "ambiguous_names",
                "entries",
                "file_imports",
                "primary",
                "version"
            ]
        );

        let restored: TypeRegistry =
            serde_json::from_value(json).expect("the registry deserializes");
        assert_eq!(restored, registry);
        assert!(restored.get("User").is_some());
        assert_eq!(restored.finish_recording(), None);
    }

    #[test]
    fn each_entry_and_each_files_imports_are_written_once() {
        let imports = vec![FileImportEntry {
            local_name: "Address".to_string(),
            module_specifier: "./address".to_string(),
            original_name: None,
            is_type_only: true,
        }];
        let mut registry = TypeRegistry::new();
        for name in ["User", "Account"] {
            let mut entry = make_interface_entry(name, "/project/src/models.ts", vec![]);
            entry.file_imports = imports.clone();
            registry.insert(entry, "/project");
        }
        let json = serde_json::to_string(&registry).expect("the registry serializes");
        assert_eq!(json.matches("\"./address\"").count(), 1, "{json}");
        assert_eq!(
            json.matches("\"src/models.ts::User\":{").count(),
            1,
            "{json}"
        );
        let restored: TypeRegistry = serde_json::from_str(&json).expect("the registry reads");
        assert_eq!(restored, registry);
        assert_eq!(
            restored.get("User").map(|entry| entry.file_imports.clone()),
            Some(imports)
        );
    }

    #[test]
    fn the_first_wire_form_still_reads() {
        let entry = make_interface_entry("User", "/project/src/user.ts", vec![]);
        let first_form = serde_json::json!({
            "types": { "User": entry },
            "qualified_types": { "src/user.ts::User": entry },
            "ambiguous_names": [],
        });
        let restored: TypeRegistry =
            serde_json::from_value(first_form).expect("the first form reads");
        let mut expected = TypeRegistry::new();
        expected.insert(entry, "/project");
        assert_eq!(restored, expected);
    }

    #[test]
    fn a_name_without_a_matching_qualified_entry_survives_the_wire() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.types.insert(
            "Orphan".to_string(),
            make_interface_entry("Orphan", "/project/src/orphan.ts", vec![]),
        );
        let json = serde_json::to_string(&registry).expect("the registry serializes");
        let restored: TypeRegistry = serde_json::from_str(&json).expect("the registry reads");
        assert_eq!(restored, registry);
    }

    #[test]
    fn a_registry_survives_a_json_round_trip_unchanged() {
        let mut registry = TypeRegistry::new();
        registry.insert(
            make_interface_entry("User", "/project/src/user.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("Shared", "/project/src/a.ts", vec![]),
            "/project",
        );
        registry.insert(
            make_interface_entry("Shared", "/project/src/b.ts", vec![]),
            "/project",
        );
        let json = serde_json::to_string(&registry).expect("the registry serializes");
        let restored: TypeRegistry =
            serde_json::from_str(&json).expect("the registry deserializes");
        assert_eq!(restored, registry);
        assert!(restored.ambiguous_names.contains("Shared"));
    }

    #[test]
    fn registries_with_different_primaries_are_not_equal() {
        let mut first = TypeRegistry::new();
        first.insert(
            make_interface_entry("Shared", "/project/src/a.ts", vec![]),
            "/project",
        );
        first.insert(
            make_interface_entry("Shared", "/project/src/b.ts", vec![]),
            "/project",
        );
        let mut second = TypeRegistry::new();
        second.insert(
            make_interface_entry("Shared", "/project/src/b.ts", vec![]),
            "/project",
        );
        second.insert(
            make_interface_entry("Shared", "/project/src/a.ts", vec![]),
            "/project",
        );
        assert_ne!(first, second);
    }

    #[test]
    fn resolve_in_file_takes_the_longest_ending_and_refuses_one_two_modules_share() {
        let registry = registry_with_two_record_links();
        let resolve = |module: &str| {
            let imports = [import_from("RecordLink", module)];
            registry
                .resolve_in_file("RecordLink", "/project/src/routes/page.ts", &imports)
                .map(|entry| entry.file_path.clone())
        };
        // `lib/index` is the end of one module's path, and `lib` its directory.
        assert_eq!(
            resolve("$lib/index").as_deref(),
            Some("/project/src/lib/index.ts")
        );
        assert_eq!(
            resolve("$lib").as_deref(),
            Some("/project/src/lib/index.ts")
        );
        assert_eq!(
            resolve("$lib/idp").as_deref(),
            Some("/project/src/lib/idp/index.ts")
        );
        // `index` is the end of both.
        assert_eq!(resolve("~/index"), None);
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
