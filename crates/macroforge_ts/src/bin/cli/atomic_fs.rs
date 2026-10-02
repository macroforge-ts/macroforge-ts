//! Crash-safe filesystem primitives shared by every CLI subcommand.
//!
//! Macroforge's outputs are read by processes that never took our lock: Vite,
//! editors, `tsc`, publish tooling. A plain `fs::write` of a 4.8 MB registry or
//! a recursive copy into `dist` gives those readers a window in which they see
//! a truncated file or a half-populated tree. Everything here closes that
//! window by making the last step a single `rename`, which the kernel applies
//! atomically: a reader observes either the whole old state or the whole new
//! one.

use anyhow::{Context, Result};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Writes `contents` to `path` atomically.
///
/// The bytes land in a temporary file alongside the destination, which is then
/// renamed over `path`. A concurrent reader sees either the previous file or
/// the complete new one, never a partial write.
///
/// The file is not synced to disk first. Everything written through here is
/// derived and regenerated: a cache or state file that a power loss empties
/// reads as missing and is rebuilt, and syncing every artifact of a build
/// costs far more than that rare rebuild.
///
/// The temporary file is created in the destination's own directory so the
/// final rename stays within one filesystem (a cross-device rename would fail).
pub(crate) fn write_atomic(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;

    let mut tmp = tempfile::NamedTempFile::new_in(parent).with_context(|| {
        format!(
            "failed to create a temporary file in {} for {}",
            parent.display(),
            path.display()
        )
    })?;

    tmp.write_all(contents)
        .with_context(|| format!("failed to write {}", path.display()))?;
    tmp.flush()
        .with_context(|| format!("failed to flush {}", path.display()))?;

    // NamedTempFile creates 0600. Persisting that would silently tighten the
    // permissions of every artifact relative to the plain `fs::write` this
    // replaces, so restore the usual 0644 before the rename.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        tmp.as_file()
            .set_permissions(fs::Permissions::from_mode(0o644))
            .with_context(|| format!("failed to set permissions on {}", path.display()))?;
    }

    tmp.persist(path)
        .map_err(|e| e.error)
        .with_context(|| format!("failed to move the temporary file into {}", path.display()))?;

    Ok(())
}

/// Moves the directory `staging` into place at `dest`, replacing whatever is
/// already there.
///
/// There is no atomic "replace directory" syscall, so this is the closest
/// portable equivalent: the existing `dest` is renamed aside, `staging` is
/// renamed into its place, and only then is the old tree deleted. A reader
/// racing this sees the old tree or the new one (the exposure is two `rename`
/// calls rather than the duration of a recursive copy), and the swap is undone
/// if the second rename fails.
///
/// `staging` must be a sibling of `dest` so both renames stay on one
/// filesystem.
pub(crate) fn swap_dir(staging: &Path, dest: &Path) -> Result<()> {
    if !staging.is_dir() {
        anyhow::bail!(
            "staging directory {} does not exist; refusing to replace {}",
            staging.display(),
            dest.display()
        );
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }

    // Nothing to displace: one rename is the whole operation.
    if !dest.exists() {
        return fs::rename(staging, dest).with_context(|| {
            format!(
                "failed to move {} into {}",
                staging.display(),
                dest.display()
            )
        });
    }

    let backup = sibling_with_suffix(dest, "macroforge-old");
    remove_dir_if_present(&backup)?;

    fs::rename(dest, &backup).with_context(|| {
        format!(
            "failed to move the existing {} aside to {}",
            dest.display(),
            backup.display()
        )
    })?;

    if let Err(e) = fs::rename(staging, dest) {
        // Put the original back rather than leaving the destination missing.
        if let Err(restore) = fs::rename(&backup, dest) {
            eprintln!(
                "[macroforge] warning: could not restore {} from {}: {restore}",
                dest.display(),
                backup.display()
            );
        }
        return Err(e).with_context(|| {
            format!(
                "failed to move {} into {}",
                staging.display(),
                dest.display()
            )
        });
    }

    // The swap already succeeded; a failure to reclaim the old tree costs disk
    // space but must not fail the command.
    if let Err(e) = fs::remove_dir_all(&backup) {
        eprintln!(
            "[macroforge] warning: could not remove the replaced directory {}: {e}",
            backup.display()
        );
    }

    Ok(())
}

/// Removes the directory `path` and everything in it, if it exists.
pub(crate) fn remove_dir_if_present(path: &Path) -> Result<()> {
    match fs::remove_dir_all(path) {
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
            Err(error).with_context(|| format!("failed to remove {}", path.display()))
        }
        _ => Ok(()),
    }
}

/// Removes a scratch directory while already handling a failure, which the
/// directory's removal must not replace; one that cannot be removed is
/// reported and left behind.
pub(crate) fn discard_dir(path: &Path) {
    if let Err(error) = remove_dir_if_present(path) {
        eprintln!("[macroforge] warning: {error:#}");
    }
}

/// Builds a scratch path next to `path`, tagged with `suffix` and this process
/// id so concurrent macroforge runs never pick the same name.
///
/// The result is always a sibling of `path`: it shares the parent directory, so
/// renaming between the two stays on one filesystem, and it sits at the same
/// depth, which matters for `svelte-package`, whose `.d.ts.map` sources are
/// computed relative to the output directory.
pub(crate) fn sibling_with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    parent.join(format!("{name}.{suffix}-{}", std::process::id()))
}
