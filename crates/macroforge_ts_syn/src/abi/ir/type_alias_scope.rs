//! Expanding a generic alias declared in another module brings names from
//! that module's scope into the expanding file. `type RecordLink<T> = RecordId
//! | T` expands to `RecordId | Employee`, and `RecordId` means what the alias's
//! module imported, so the expanding file has to import it the same way.

use std::collections::{HashMap, HashSet};
use std::path::{Component, Path, PathBuf};

use super::context::relative_module_specifier;
use super::type_alias::{TypeAliasIR, is_primitive_keyword};
use super::type_alias_resolve::{identifiers, render_body};
use super::type_registry::{FileImportEntry, TypeRegistry, TypeRegistryEntry};
use crate::import_registry::with_registry_mut;

/// Prefix for a borrowed name the expanding file already binds otherwise.
const SCOPE_PREFIX: &str = "__mf_scope_";

/// What each name in `alias`'s body that comes from its own module's imports
/// is called in `caller_file_path`, importing it there as a generated import.
/// `None` when the body cannot be carried over: it names a type declared
/// beside the alias, or a relative import with no caller file to rebase onto.
pub(super) fn adopt_alias_scope(
    alias: &TypeAliasIR,
    entry: &TypeRegistryEntry,
    registry: &TypeRegistry,
    caller_file_path: &str,
) -> Option<HashMap<String, String>> {
    let mut names = HashMap::new();
    if entry.file_path == caller_file_path {
        return Some(names);
    }
    let unsubstituted = render_body(&alias.body, &HashMap::new())?;
    let params: HashSet<&str> = alias
        .type_params
        .iter()
        .map(|param| param.name.as_str())
        .collect();
    for ident in identifiers(&unsubstituted) {
        if params.contains(ident) || names.contains_key(ident) || is_primitive_keyword(ident) {
            continue;
        }
        if let Some(import) = entry
            .file_imports
            .iter()
            .find(|import| import.local_name == ident)
        {
            let module =
                rebase_specifier(&import.module_specifier, &entry.file_path, caller_file_path)?;
            let local = bind_in_caller(import, &module, registry, caller_file_path);
            names.insert(ident.to_string(), local);
        } else if registry
            .get_all(ident)
            .any(|declared| declared.file_path == entry.file_path)
        {
            return None;
        }
    }
    Some(names)
}

/// Why a type named in another module cannot be brought into a file.
#[derive(Debug, PartialEq, Eq)]
pub enum Unadopted {
    /// The type is declared beside the one that names it and not exported.
    Private(String),
    /// The file already binds the name to something else.
    Taken(String),
    /// The type's module cannot be named from the file.
    Unplaced(String),
}

/// Brings each type `ts_type` names, as it is written where `entry` is
/// declared, into `caller_file_path` as a generated type import under its
/// own name, so the text means there what it means here. A name the caller
/// binds to the same type already is left alone, as is a keyword or a global.
pub fn adopt_type_scope(
    ts_type: &str,
    entry: &TypeRegistryEntry,
    registry: &TypeRegistry,
    caller_file_path: &str,
) -> Result<(), Unadopted> {
    if entry.file_path == caller_file_path {
        return Ok(());
    }
    let mut adopted = HashSet::new();
    for ident in identifiers(ts_type) {
        if is_primitive_keyword(ident) || !adopted.insert(ident) {
            continue;
        }
        // Where the name comes from in the module that writes it.
        let (module, exported) = if let Some(import) = entry
            .file_imports
            .iter()
            .find(|import| import.local_name == ident)
        {
            let module =
                rebase_specifier(&import.module_specifier, &entry.file_path, caller_file_path)
                    .ok_or_else(|| Unadopted::Unplaced(ident.to_string()))?;
            (module, import.original_name.as_deref().unwrap_or(ident))
        } else if let Some(declared) = registry
            .get_all(ident)
            .find(|declared| declared.file_path == entry.file_path)
        {
            if !declared.is_exported {
                return Err(Unadopted::Private(ident.to_string()));
            }
            let module = relative_module_specifier(caller_file_path, &entry.file_path)
                .ok_or_else(|| Unadopted::Unplaced(ident.to_string()))?;
            (module, ident)
        } else {
            continue;
        };
        let declared_in_caller = registry
            .get_all(ident)
            .any(|declared| declared.file_path == caller_file_path);
        with_registry_mut(|imports| {
            let same_binding = imports.get_source(ident) == Some(module.as_str())
                && imports.resolve_alias(ident).unwrap_or(ident) == exported;
            if same_binding {
                return Ok(());
            }
            // The generated functions are named after the type, so it is
            // brought in under its own name or not at all.
            if declared_in_caller || imports.is_available(ident) || ident != exported {
                return Err(Unadopted::Taken(ident.to_string()));
            }
            imports.borrow_import(ident, None, &module, true);
            Ok(())
        })?;
    }
    Ok(())
}

/// The name `import` goes by in the caller: its own name when the caller
/// binds nothing else to it, otherwise a prefixed alias.
fn bind_in_caller(
    import: &FileImportEntry,
    module: &str,
    registry: &TypeRegistry,
    caller_file_path: &str,
) -> String {
    let exported = import
        .original_name
        .as_deref()
        .unwrap_or(&import.local_name);
    let declared_in_caller = registry
        .get_all(&import.local_name)
        .any(|declared| declared.file_path == caller_file_path);
    with_registry_mut(|imports| {
        let same_binding = imports.get_source(&import.local_name) == Some(module)
            && imports
                .resolve_alias(&import.local_name)
                .unwrap_or(&import.local_name)
                == exported;
        if same_binding {
            return import.local_name.clone();
        }
        let free = !declared_in_caller && !imports.is_available(&import.local_name);
        let local = if free {
            import.local_name.clone()
        } else {
            format!("{SCOPE_PREFIX}{}", import.local_name)
        };
        let original = (local != exported).then_some(exported);
        imports.borrow_import(&local, original, module, import.is_type_only);
        local
    })
}

/// `specifier` as written in `from_file`, rewritten to name the same module
/// from `to_file`. A package specifier is the same everywhere; a relative one
/// cannot be rebased without a `to_file`.
fn rebase_specifier(specifier: &str, from_file: &str, to_file: &str) -> Option<String> {
    if !specifier.starts_with('.') {
        return Some(specifier.to_string());
    }
    if to_file.is_empty() {
        return None;
    }
    let target = normalize(&Path::new(from_file).parent()?.join(specifier));
    let relative = pathdiff::diff_paths(&target, Path::new(to_file).parent()?)?;
    let relative = relative.to_string_lossy().replace('\\', "/");
    Some(if relative.starts_with('.') {
        relative
    } else {
        format!("./{relative}")
    })
}

/// `path` with its `.` and `..` components folded, without touching the disk.
fn normalize(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other),
        }
    }
    normalized
}

#[cfg(test)]
#[path = "type_alias_scope_tests.rs"]
mod tests;
