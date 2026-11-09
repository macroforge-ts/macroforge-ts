//! Which declarations each module exports again from another, followed
//! through the barrels between them.

use std::collections::HashMap;

use crate::ts_syn::abi::ir::type_registry::{
    ModuleReExport, ReExport, file_path_ends_in_module, file_path_is_module,
    resolve_relative_module, specifier_endings,
};

use super::cache::CacheEntry;

/// What a name a module exports stands for: a declaration, by its file and
/// the name it is declared under.
#[derive(Clone, PartialEq)]
struct Exported {
    file_path: String,
    name: String,
    /// Whether everything its declaring module exports came along with it.
    whole: bool,
}

/// For each declaration, by its file and declared name, the names other
/// scanned modules export it under again.
///
/// Every module starts with what it declares, and each round carries every
/// name one module further along the re-exports, until a round carries
/// nothing. A chain of barrels takes as many rounds as it is long, and each
/// round reads each re-export once, so barrels that export each other, or
/// that reach one module by many ways, cost no more than their number.
pub(super) fn link_re_exports(
    files: &[(String, &CacheEntry)],
) -> HashMap<(String, String), Vec<ReExport>> {
    let mut exports: Vec<HashMap<String, Exported>> = files
        .iter()
        .map(|(file_path, entry)| declared_by(file_path, entry))
        .collect();
    // Each module's re-exports, with the scanned file each comes from.
    let sources: Vec<Vec<(usize, &ModuleReExport)>> = files
        .iter()
        .map(|(file_path, entry)| {
            entry
                .re_exports
                .iter()
                .filter_map(|again| {
                    Some((
                        module_index(files, file_path, &again.module_specifier)?,
                        again,
                    ))
                })
                .collect()
        })
        .collect();

    for _ in 0..=files.len() {
        let mut carried = false;
        for (index, again) in sources.iter().enumerate() {
            for (source, re_export) in again {
                let Some(theirs) = exports.get(*source).cloned() else {
                    continue;
                };
                let Some(own) = exports.get_mut(index) else {
                    continue;
                };
                carried |= carry(own, &theirs, re_export);
            }
        }
        if !carried {
            break;
        }
    }

    let mut linked: HashMap<(String, String), Vec<ReExport>> = HashMap::new();
    for ((file_path, _), exported) in files.iter().zip(exports) {
        for (name, declaration) in exported {
            if declaration.file_path != *file_path {
                linked
                    .entry((declaration.file_path, declaration.name))
                    .or_default()
                    .push(ReExport {
                        file_path: file_path.clone(),
                        name,
                        whole: declaration.whole,
                    });
            }
        }
    }
    // A map's order is no order: the registry is the same for the same files.
    for again in linked.values_mut() {
        again.sort_by(|left, right| {
            (&left.file_path, &left.name).cmp(&(&right.file_path, &right.name))
        });
    }
    linked
}

/// The names a module exports of its own: each declaration under its name,
/// and under the other names the module exports it by.
fn declared_by(file_path: &str, entry: &CacheEntry) -> HashMap<String, Exported> {
    let declared = entry
        .classes
        .iter()
        .map(|class| &class.name)
        .chain(entry.interfaces.iter().map(|interface| &interface.name))
        .chain(entry.enums.iter().map(|declared| &declared.name))
        .chain(entry.type_aliases.iter().map(|alias| &alias.name));
    let mut exports: HashMap<String, Exported> = declared
        .map(|name| {
            let own = Exported {
                file_path: file_path.to_string(),
                name: name.clone(),
                whole: true,
            };
            (name.clone(), own)
        })
        .collect();
    for (declared, exported) in &entry.export_aliases {
        if let Some(own) = exports.get(declared).cloned() {
            exports.insert(exported.clone(), own);
        }
    }
    exports
}

/// Adds to `own` what `re_export` takes from `theirs`, and says whether it
/// added anything. A name that already stands for a declaration keeps it:
/// what a module declares comes before what it exports again, and of two
/// modules that give one name the first is kept.
fn carry(
    own: &mut HashMap<String, Exported>,
    theirs: &HashMap<String, Exported>,
    re_export: &ModuleReExport,
) -> bool {
    let mut add = |name: &str, exported: Exported| match own.get_mut(name) {
        // The same declaration reached again, this time with all of its
        // module: its functions come with it after all.
        Some(known) if !known.whole && exported.whole && same_declaration(known, &exported) => {
            known.whole = true;
            true
        }
        Some(_) => false,
        None => {
            own.insert(name.to_string(), exported);
            true
        }
    };
    if re_export.imported == "*" {
        // `export *` takes every name but the default.
        let mut names: Vec<&String> = theirs.keys().filter(|name| *name != "default").collect();
        names.sort();
        // Every name is added, whether or not one before it was.
        let mut added = false;
        for name in names {
            added |= theirs
                .get(name)
                .is_some_and(|exported| add(name, exported.clone()));
        }
        added
    } else {
        // A named re-export brings the type and nothing beside it.
        theirs.get(&re_export.imported).is_some_and(|exported| {
            add(
                &re_export.exported,
                Exported {
                    whole: false,
                    ..exported.clone()
                },
            )
        })
    }
}

fn same_declaration(left: &Exported, right: &Exported) -> bool {
    left.file_path == right.file_path && left.name == right.name
}

/// The index of the scanned file `specifier` names in `importer`: the
/// module a relative one resolves to, or for any other the one file whose
/// path ends as the longest end of the specifier that a scanned file ends in.
fn module_index(files: &[(String, &CacheEntry)], importer: &str, specifier: &str) -> Option<usize> {
    if let Some(module_path) = resolve_relative_module(importer, specifier) {
        return files
            .iter()
            .position(|(path, _)| file_path_is_module(path, &module_path));
    }
    if specifier.starts_with('.') {
        return None;
    }
    specifier_endings(specifier).iter().find_map(|ending| {
        let mut named = files
            .iter()
            .enumerate()
            .filter(|(_, (path, _))| file_path_ends_in_module(path, ending))
            .map(|(index, _)| index);
        let first = named.next()?;
        // Two files under one ending name no one module.
        Some(named.next().is_none().then_some(first))
    })?
}
