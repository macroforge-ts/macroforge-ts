//! `macroforge init`: declares `#macroforge/config` in the project manifest.

use anyhow::Result;
use macroforge_ts::host::config::manifest::{InitOutcome, add_handlers_import};
use std::path::Path;

pub fn run_init(root: &Path) -> Result<()> {
    match add_handlers_import(root)? {
        InitOutcome::Added(manifest) => {
            eprintln!(
                "[macroforge init] Declared #macroforge/config in {}",
                manifest.display()
            );
        }
        InitOutcome::AlreadyDeclared(manifest) => {
            eprintln!(
                "[macroforge init] {} already declares #macroforge/config",
                manifest.display()
            );
        }
    }
    Ok(())
}
