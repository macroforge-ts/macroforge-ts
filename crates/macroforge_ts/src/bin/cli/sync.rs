//! `macroforge sync`: writes the generated files a type check reads.

use anyhow::{Context, Result};
use macroforge_ts::host::config::{config_file, hoist::sync_expanded_config};
use std::path::Path;

use crate::lock::ProjectLock;
use crate::wrappers::{scan_project, write_registries};

/// Writes the type and declarative registries and the expanded config, without
/// expanding any file. Like the checkers, it takes the project lock only to
/// write, so a project with neither macros nor a config gets no `.macroforge/`.
pub fn run_sync(root: &Path) -> Result<()> {
    let output = scan_project(root)?;
    let has_macros = output.macro_files > 0 || output.declarative_registry.file_count() > 0;
    if !has_macros && config_file(root).is_none() {
        eprintln!(
            "[macroforge sync] {} has no macros or config to sync",
            root.display()
        );
        return Ok(());
    }
    let lock = ProjectLock::acquire(root, "sync", false)?;
    if has_macros {
        // Also writes the expanded config.
        write_registries(root, &output)?;
    } else {
        sync_expanded_config(root)
            .with_context(|| format!("failed to expand the config of {}", root.display()))?;
    }
    drop(lock);
    eprintln!("[macroforge sync] Synced {}", root.display());
    Ok(())
}
