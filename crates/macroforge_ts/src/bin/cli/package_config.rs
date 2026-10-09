//! Ships the expanded config with a packaged library.
//!
//! Generated code imports foreign-type handlers from `#macroforge/config`,
//! which the project's manifest maps to `.macroforge/config/`. A published
//! package does not ship `.macroforge/`, so packaging copies the expanded
//! config into the output under `__macroforge/config/` and points each
//! emitted import at the copy with a relative path.

use anyhow::{Context, Result, bail};
use macroforge_ts::host::config::hoist::EXPANDED_CONFIG_DIR;
use macroforge_ts::ts_syn::config::FOREIGN_HANDLERS_MODULE;
use std::fs;
use std::path::{Path, PathBuf};

/// Where the expanded config goes in a packaged output.
const SHIPPED_DIR: &str = "__macroforge/config";

/// The emitted files that can hold an import.
const REWRITTEN_EXTENSIONS: &[&str] = &["js", "mjs", "cjs", "ts", "mts", "cts"];

/// Copies the expanded config of the project at `root` into `out_dir` and
/// rewrites every `#macroforge/config` import there to a relative path.
pub(crate) fn ship_expanded_config(root: &Path, out_dir: &Path) -> Result<()> {
    let shipped = out_dir.join(SHIPPED_DIR);
    let mut importers = Vec::new();
    collect_importers(out_dir, &shipped, &mut importers)?;
    if importers.is_empty() {
        return Ok(());
    }

    let expanded = root.join(EXPANDED_CONFIG_DIR);
    if !expanded.join("handlers.js").is_file() {
        bail!(
            "{} imports `{FOREIGN_HANDLERS_MODULE}`, but {} holds no expanded config to ship with it",
            importers[0].display(),
            expanded.display()
        );
    }
    copy_dir(&expanded, &shipped)?;

    let handlers = shipped.join("handlers.js");
    for importer in importers {
        let text = fs::read_to_string(&importer)
            .with_context(|| format!("failed to read {}", importer.display()))?;
        let specifier = relative_specifier(&importer, &handlers);
        let rewritten = rewrite_specifier(&text, &specifier);
        fs::write(&importer, rewritten)
            .with_context(|| format!("failed to write {}", importer.display()))?;
    }
    Ok(())
}

/// Every emitted file under `dir` that names `#macroforge/config`.
fn collect_importers(dir: &Path, shipped: &Path, importers: &mut Vec<PathBuf>) -> Result<()> {
    let entries = fs::read_dir(dir).with_context(|| format!("failed to list {}", dir.display()))?;
    for entry in entries {
        let path = entry
            .with_context(|| format!("failed to list {}", dir.display()))?
            .path();
        if path.starts_with(shipped) {
            continue;
        }
        if path.is_dir() {
            collect_importers(&path, shipped, importers)?;
            continue;
        }
        let rewritable = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| REWRITTEN_EXTENSIONS.contains(&extension));
        if !rewritable {
            continue;
        }
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        if text.contains(&format!("\"{FOREIGN_HANDLERS_MODULE}\""))
            || text.contains(&format!("'{FOREIGN_HANDLERS_MODULE}'"))
        {
            importers.push(path);
        }
    }
    Ok(())
}

/// `text` with each quoted `#macroforge/config` specifier replaced by
/// `specifier`, keeping its quote style.
fn rewrite_specifier(text: &str, specifier: &str) -> String {
    let mut rewritten = text.to_string();
    for quote in ['"', '\''] {
        rewritten = rewritten.replace(
            &format!("{quote}{FOREIGN_HANDLERS_MODULE}{quote}"),
            &format!("{quote}{specifier}{quote}"),
        );
    }
    rewritten
}

/// The specifier `importer` reaches `target` by: relative, `/`-separated,
/// and starting with `./` or `../`.
fn relative_specifier(importer: &Path, target: &Path) -> String {
    let from = importer.parent().unwrap_or(Path::new("."));
    let relative = pathdiff::diff_paths(target, from).unwrap_or_else(|| target.to_path_buf());
    let joined = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/");
    if joined.starts_with("../") {
        joined
    } else {
        format!("./{joined}")
    }
}

fn copy_dir(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to).with_context(|| format!("failed to create {}", to.display()))?;
    let entries =
        fs::read_dir(from).with_context(|| format!("failed to list {}", from.display()))?;
    for entry in entries {
        let path = entry
            .with_context(|| format!("failed to list {}", from.display()))?
            .path();
        let Some(name) = path.file_name() else {
            continue;
        };
        let target = to.join(name);
        if path.is_dir() {
            copy_dir(&path, &target)?;
        } else {
            fs::copy(&path, &target).with_context(|| {
                format!("failed to copy {} to {}", path.display(), target.display())
            })?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "package_config_tests.rs"]
mod tests;
