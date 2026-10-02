//! File access that works from both builds: the filesystem on a native host,
//! and Node's `fs` from the wasm build, which runs inside Node (the Vite
//! plugin, the TypeScript plugin, the Deno plugin).

use std::io;
use std::path::{Path, PathBuf};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn read_to_string(path: &Path) -> io::Result<String> {
    std::fs::read_to_string(path)
}

/// What a path names, as far as module resolution cares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathKind {
    File,
    Directory,
    /// Nothing, or something neither a file nor a directory.
    Absent,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn kind(path: &Path) -> io::Result<PathKind> {
    match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_file() => Ok(PathKind::File),
        Ok(metadata) if metadata.is_dir() => Ok(PathKind::Directory),
        Ok(_) => Ok(PathKind::Absent),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(PathKind::Absent),
        Err(error) => Err(error),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn canonicalize(path: &Path) -> io::Result<PathBuf> {
    path.canonicalize()
}

/// The file's modification time, in nanoseconds since the Unix epoch, and
/// its length.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn modified_and_len(path: &Path) -> io::Result<(u128, u64)> {
    let metadata = std::fs::metadata(path)?;
    let modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| io::Error::other(format!("{} predates 1970: {error}", path.display())))?;
    Ok((modified.as_nanos(), metadata.len()))
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod node {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(inline_js = r#"
import { readFileSync, realpathSync, statSync } from 'node:fs';
export function readText(path) {
    return readFileSync(path, 'utf8');
}
// 1 for a file, 2 for a directory, 0 for anything else, including a path
// that does not exist.
export function pathKind(path) {
    const stats = statSync(path, { throwIfNoEntry: false });
    if (stats === undefined) return 0;
    return stats.isFile() ? 1 : stats.isDirectory() ? 2 : 0;
}
export function realPath(path) {
    return realpathSync(path);
}
// The modification time in nanoseconds and the length, as decimal strings
// because a JS number cannot hold the nanoseconds exactly.
export function modifiedAndLen(path) {
    const stats = statSync(path, { bigint: true });
    return [stats.mtimeNs.toString(), stats.size.toString()];
}
"#)]
    extern "C" {
        #[wasm_bindgen(catch, js_name = readText)]
        pub(super) fn read_text(path: &str) -> Result<String, JsValue>;
        #[wasm_bindgen(js_name = pathKind)]
        pub(super) fn path_kind(path: &str) -> u32;
        #[wasm_bindgen(catch, js_name = realPath)]
        pub(super) fn real_path(path: &str) -> Result<String, JsValue>;
        #[wasm_bindgen(catch, js_name = modifiedAndLen)]
        pub(super) fn modified_and_len(path: &str) -> Result<Vec<String>, JsValue>;
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn read_to_string(path: &Path) -> io::Result<String> {
    node::read_text(&path.to_string_lossy()).map_err(|error| io::Error::other(format!("{error:?}")))
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn kind(path: &Path) -> io::Result<PathKind> {
    Ok(match node::path_kind(&path.to_string_lossy()) {
        1 => PathKind::File,
        2 => PathKind::Directory,
        _ => PathKind::Absent,
    })
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn canonicalize(path: &Path) -> io::Result<PathBuf> {
    node::real_path(&path.to_string_lossy())
        .map(PathBuf::from)
        .map_err(|error| io::Error::other(format!("{error:?}")))
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn modified_and_len(path: &Path) -> io::Result<(u128, u64)> {
    let parts = node::modified_and_len(&path.to_string_lossy())
        .map_err(|error| io::Error::other(format!("{error:?}")))?;
    let [modified, len] = parts.as_slice() else {
        return Err(io::Error::other(format!(
            "stat of {} returned {} parts, not two",
            path.display(),
            parts.len()
        )));
    };
    let invalid = |error: std::num::ParseIntError| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("stat of {}: {error}", path.display()),
        )
    };
    Ok((
        modified.parse().map_err(invalid)?,
        len.parse().map_err(invalid)?,
    ))
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
fn without_filesystem(path: &Path) -> io::Error {
    io::Error::new(
        io::ErrorKind::Unsupported,
        format!(
            "cannot read {}: this WASM build has no `wasm` feature and so no filesystem",
            path.display()
        ),
    )
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
pub(crate) fn read_to_string(path: &Path) -> io::Result<String> {
    Err(without_filesystem(path))
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
pub(crate) fn kind(path: &Path) -> io::Result<PathKind> {
    Err(without_filesystem(path))
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
pub(crate) fn canonicalize(path: &Path) -> io::Result<PathBuf> {
    Err(without_filesystem(path))
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
pub(crate) fn modified_and_len(path: &Path) -> io::Result<(u128, u64)> {
    Err(without_filesystem(path))
}
