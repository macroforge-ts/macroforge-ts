//! Expands the config so every foreign-type handler is a named export.
//!
//! Generated code imports each handler by name from `#macroforge/config`,
//! which resolves to `.macroforge/config/handlers.ts`: a copy of the root
//! config with each handler hoisted into `export const __foreign__…`. A
//! handler declared in a base config is hoisted in a copy of that file under
//! `modules/`, which `handlers.ts` re-exports. Copying whole files keeps every
//! handler's imports, helpers and closures in scope, so nothing is inlined
//! into generated code.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use macroforge_ts_syn::config::{ForeignTypeConfig, HandlerSite};
use oxc::allocator::Allocator;
use oxc::parser::Parser;
use oxc::span::SourceType;

use super::super::error::{MacroError, Result};
use super::hoist_module::{hoist_module, quoted, slash_path};

/// Where the expanded config is written, under the project root.
pub const EXPANDED_CONFIG_DIR: &str = ".macroforge/config";

/// The expanded root config, the module `#macroforge/config` names.
pub const HANDLERS_STEM: &str = "handlers";

/// One expanded config module, written as TypeScript and as JavaScript.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandedFile {
    /// Its path under [`EXPANDED_CONFIG_DIR`], without an extension.
    pub stem: PathBuf,
    pub typescript: String,
    pub javascript: String,
}

/// The expanded config for the config at `root_config`, whose resolved
/// foreign types are `foreign_types`. `read` reads a config module's source.
pub fn expand_config(
    root_config: &Path,
    foreign_types: &[ForeignTypeConfig],
    read: impl Fn(&Path) -> std::io::Result<String>,
) -> Result<Vec<ExpandedFile>> {
    check_export_names(foreign_types)?;
    let root = canonical(root_config)?;
    let root_dir = root.parent().unwrap_or(Path::new(".")).to_path_buf();
    let out_dir = root_dir.join(EXPANDED_CONFIG_DIR);

    let mut by_module: BTreeMap<PathBuf, Vec<(&HandlerSite, String)>> = BTreeMap::new();
    by_module.insert(root.clone(), Vec::new());
    for foreign_type in foreign_types {
        for site in &foreign_type.handler_sites {
            by_module
                .entry(canonical(&site.module)?)
                .or_default()
                .push((site, site.handler.export_name(&foreign_type.name)));
        }
    }

    let mut files = Vec::new();
    let mut reexports = String::new();
    for (module, sites) in &by_module {
        if *module == root {
            continue;
        }
        let stem = Path::new("modules").join(module_stem(module, &root_dir));
        let parent = out_dir
            .join(&stem)
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let typescript = hoist_module(
            &read_module(&read, module)?,
            module,
            sites,
            &parent,
            &root_dir,
        )?;
        let names: Vec<&str> = sites.iter().map(|(_, name)| name.as_str()).collect();
        if !names.is_empty() {
            reexports.push_str(&format!(
                "export {{ {} }} from {};\n",
                names.join(", "),
                quoted(&format!("./{}.js", slash_path(&stem)))
            ));
        }
        files.push(expanded_file(stem, typescript, module)?);
    }

    let root_sites = by_module.get(&root).map(Vec::as_slice).unwrap_or_default();
    let mut typescript = hoist_module(
        &read_module(&read, &root)?,
        &root,
        root_sites,
        &out_dir,
        &root_dir,
    )?;
    typescript.push_str(&reexports);
    files.push(expanded_file(
        PathBuf::from(HANDLERS_STEM),
        typescript,
        &root,
    )?);
    Ok(files)
}

/// Writes the expanded config of the project at `root_dir` into
/// [`EXPANDED_CONFIG_DIR`], replacing what was there. A project without a
/// config has no expanded config. Nothing is written when the expansion is
/// unchanged, so a watcher of the expanded files is not woken for nothing.
pub fn sync_expanded_config(root_dir: &Path) -> Result<()> {
    let out_dir = root_dir.join(EXPANDED_CONFIG_DIR);
    let Some(config_path) = super::config_file(root_dir) else {
        return remove_dir(&out_dir);
    };
    let source =
        std::fs::read_to_string(&config_path).map_err(|error| io_error(&config_path, error))?;
    let resolved = super::resolve::resolve_config(&source, config_path.to_string_lossy().as_ref())?;
    let files = expand_config(&config_path, &resolved.config.foreign_types, |path| {
        std::fs::read_to_string(path)
    })?;
    let declares_handlers = resolved
        .config
        .foreign_types
        .iter()
        .any(|foreign_type| !foreign_type.handler_sites.is_empty());
    if declares_handlers && !super::manifest::handlers_import_declared(root_dir)? {
        return Err(super::manifest::missing_entry_error(root_dir));
    }
    if written_as(&out_dir, &files) {
        return Ok(());
    }
    write_expanded(&out_dir, &files)
}

/// Whether `out_dir` holds exactly `files`.
fn written_as(out_dir: &Path, files: &[ExpandedFile]) -> bool {
    let mut expected = HashSet::new();
    for file in files {
        for (extension, contents) in [("ts", &file.typescript), ("js", &file.javascript)] {
            let path = out_dir.join(&file.stem).with_extension(extension);
            if std::fs::read_to_string(&path).ok().as_ref() != Some(contents) {
                return false;
            }
            expected.insert(path);
        }
    }
    let mut written = Vec::new();
    collect_files(out_dir, &mut written);
    written.len() == expected.len() && written.iter().all(|path| expected.contains(path))
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// Writes `files` into a staging directory beside `out_dir`, then swaps it in
/// with a rename, so a reader never sees half an expansion.
fn write_expanded(out_dir: &Path, files: &[ExpandedFile]) -> Result<()> {
    let parent = out_dir.parent().unwrap_or(Path::new("."));
    let staging = parent.join(format!("config.staging-{}", std::process::id()));
    let previous = parent.join(format!("config.previous-{}", std::process::id()));
    remove_dir(&staging)?;
    for file in files {
        for (extension, contents) in [("ts", &file.typescript), ("js", &file.javascript)] {
            let path = staging.join(&file.stem).with_extension(extension);
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir).map_err(|error| io_error(dir, error))?;
            }
            std::fs::write(&path, contents).map_err(|error| io_error(&path, error))?;
        }
    }
    remove_dir(&previous)?;
    if out_dir.exists() {
        std::fs::rename(out_dir, &previous).map_err(|error| io_error(out_dir, error))?;
    }
    std::fs::rename(&staging, out_dir).map_err(|error| io_error(&staging, error))?;
    remove_dir(&previous)
}

fn remove_dir(dir: &Path) -> Result<()> {
    match std::fs::remove_dir_all(dir) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error(dir, error)),
    }
}

fn io_error(path: &Path, error: std::io::Error) -> MacroError {
    MacroError::Io(std::io::Error::new(
        error.kind(),
        format!("{}: {error}", path.display()),
    ))
}

fn expanded_file(stem: PathBuf, typescript: String, module: &Path) -> Result<ExpandedFile> {
    let javascript = strip_types(&typescript, module)?;
    Ok(ExpandedFile {
        stem,
        typescript,
        javascript,
    })
}

fn read_module(read: &impl Fn(&Path) -> std::io::Result<String>, module: &Path) -> Result<String> {
    read(module).map_err(|error| {
        MacroError::InvalidConfig(format!(
            "could not read the config module {}: {error}",
            module.display()
        ))
    })
}

/// Two foreign types whose handlers would share an export name.
fn check_export_names(foreign_types: &[ForeignTypeConfig]) -> Result<()> {
    let mut owners: HashMap<String, &str> = HashMap::new();
    for foreign_type in foreign_types {
        for site in &foreign_type.handler_sites {
            let name = site.handler.export_name(&foreign_type.name);
            if let Some(other) = owners.insert(name.clone(), &foreign_type.name)
                && other != foreign_type.name
            {
                return Err(MacroError::InvalidConfig(format!(
                    "the foreign types `{other}` and `{}` would both export `{name}`; rename one of them",
                    foreign_type.name
                )));
            }
        }
    }
    Ok(())
}

/// The type-free JavaScript of an expanded module.
fn strip_types(typescript: &str, module: &Path) -> Result<String> {
    use oxc::codegen::Codegen;
    use oxc::semantic::SemanticBuilder;
    use oxc::transformer::{TransformOptions, Transformer};

    let fail =
        |message: String| MacroError::InvalidConfig(format!("{}: {message}", module.display()));
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, typescript, SourceType::ts()).parse();
    if let Some(diagnostic) = parsed.diagnostics.first() {
        return Err(fail(format!(
            "the expanded config does not parse: {diagnostic}"
        )));
    }
    let mut program = parsed.program;
    let scoping = SemanticBuilder::new()
        .build(&program)
        .semantic
        .into_scoping();
    let transformed = Transformer::new(&allocator, module, &TransformOptions::default())
        .build_with_scoping(scoping, &mut program);
    if let Some(diagnostic) = transformed.diagnostics.first() {
        return Err(fail(format!(
            "could not strip the expanded config's types: {diagnostic}"
        )));
    }
    Ok(Codegen::new().build(&program).code)
}

/// Where a base config module's copy goes, under `modules/`.
fn module_stem(module: &Path, root_dir: &Path) -> PathBuf {
    let without_extension = |path: &Path| path.with_extension("");
    match module.strip_prefix(root_dir) {
        Ok(relative) => without_extension(relative),
        Err(_) => {
            let mut hasher = DefaultHasher::new();
            module.hash(&mut hasher);
            let name = module
                .file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
                .unwrap_or_default();
            PathBuf::from("external").join(format!("{:016x}-{name}", hasher.finish()))
        }
    }
}

fn canonical(path: &Path) -> Result<PathBuf> {
    path.canonicalize().map_err(|error| {
        MacroError::InvalidConfig(format!(
            "could not resolve the config module {}: {error}",
            path.display()
        ))
    })
}

#[cfg(test)]
#[path = "hoist_tests.rs"]
mod tests;
