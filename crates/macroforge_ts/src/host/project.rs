//! One file of a project, expanded as the CLI expands it: under the macroforge
//! config nearest to the file, against the project's registries, with the
//! registry lookups the expansion made.

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, anyhow};

use crate::host::declarative::ProjectDeclarativeRegistry;
use crate::host::scanner::ScanOutput;
use crate::host::{
    MacroConfig, MacroExpander, MacroExpansion, MacroforgeConfig, MacroforgeConfigLoader,
    clear_foreign_types, clear_registry, set_foreign_types_from,
};
use crate::ts_syn::abi::ir::type_registry::TypeRegistry;

/// The registries a project scan produces, which every file's expansion reads.
/// Both are shared, so handing them to each file's expander copies neither.
#[derive(Debug, Clone, Default)]
pub struct ProjectRegistries {
    pub type_registry: Option<TypeRegistry>,
    pub declarative_registry: Option<Arc<ProjectDeclarativeRegistry>>,
}

impl From<&ScanOutput> for ProjectRegistries {
    fn from(output: &ScanOutput) -> Self {
        Self {
            type_registry: Some(output.registry.clone()),
            declarative_registry: Some(Arc::new(output.declarative_registry.clone())),
        }
    }
}

/// An expander for `path`, configured by the macroforge config nearest to it.
/// It is cloned from one the process keeps per project root and config.
///
/// `root` is the project the command runs on. The file's own config takes
/// precedence, since external macros resolve from its `node_modules` whatever
/// the working directory is. Installs the config's foreign types on the
/// calling thread.
pub fn expander_for_file(
    root: &Path,
    path: &Path,
    registries: &ProjectRegistries,
) -> Result<MacroExpander> {
    let discovered = MacroforgeConfigLoader::find_with_root_from_path(path).with_context(|| {
        format!(
            "failed to load the macroforge config for {}",
            path.display()
        )
    })?;
    let (config, project_root) = match discovered {
        Some((config, config_root)) => (config, config_root),
        None => (MacroforgeConfig::default(), root.to_path_buf()),
    };
    let config = std::sync::Arc::new(config);
    set_foreign_types_from(&config);

    let mut expander = MacroExpander::pooled(MacroConfig::from((*config).clone()), project_root)
        .context("failed to initialize macro expander")?;
    expander.set_project_config(config);
    if let Some(registry) = &registries.type_registry {
        expander.set_type_registry(registry.clone());
    }
    expander.set_declarative_registry_shared(registries.declarative_registry.clone());
    Ok(expander)
}

/// Clears the calling thread's import registry, foreign types and macro
/// context when dropped, so an expansion, even one that fails, does not hand
/// them to the next expansion on the same thread.
pub(crate) struct ThreadStateReset;

impl Drop for ThreadStateReset {
    fn drop(&mut self) {
        clear_registry();
        clear_foreign_types();
        crate::ts_syn::context_registry::clear_context();
    }
}

/// Expands `source`, the text of `path`, and records the registry lookups it
/// made in [`MacroExpansion::registry_reads`].
///
/// Expands whether or not the file has macros: callers that skip macro-free
/// files check [`crate::has_macro_annotations`] first.
pub fn expand_project_file(
    root: &Path,
    path: &Path,
    source: &str,
    registries: &ProjectRegistries,
) -> Result<MacroExpansion> {
    let thread_state = ThreadStateReset;
    let expansion = expander_for_file(root, path, registries)?
        .expand_source_recorded(source, &path.display().to_string())
        .map_err(|err| anyhow!("{err:?}"))?;
    drop(thread_state);
    Ok(expansion)
}
