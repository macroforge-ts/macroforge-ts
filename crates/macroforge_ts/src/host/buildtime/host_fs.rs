//! Host filesystem bridge for `buildtime.fs.*`.
//!
//! On native targets every call goes straight through `std::fs`. The wasm
//! build is a Node ES module, so it binds `node:fs` directly; Node, Deno and
//! Bun all provide it.
//!
//! Both paths return the same `Result<T, HostFsError>`, so callers
//! (the Boa backend's `fs_*_impl` helpers) don't need to know which
//! target they're running on.

use std::path::Path;

#[derive(Debug, Clone, thiserror::Error)]
pub enum HostFsError {
    #[error("io error reading {}: {message}", .path.display())]
    Io {
        path: std::path::PathBuf,
        message: String,
    },
}

/// Read a file as UTF-8 text from the host filesystem.
pub fn read_text(path: &Path) -> Result<String, HostFsError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::fs::read_to_string(path).map_err(|e| HostFsError::Io {
            path: path.to_path_buf(),
            message: e.to_string(),
        })
    }
    #[cfg(target_arch = "wasm32")]
    {
        wasm_impl::read_text(path)
    }
}

/// Probe whether a file exists on the host filesystem.
pub fn exists(path: &Path) -> Result<bool, HostFsError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(path.exists())
    }
    #[cfg(target_arch = "wasm32")]
    {
        wasm_impl::exists(path)
    }
}

/// List entries in a directory, sorted alphabetically.
pub fn list_dir(path: &Path) -> Result<Vec<String>, HostFsError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let iter = std::fs::read_dir(path).map_err(|e| HostFsError::Io {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;
        let mut names = iter
            .map(|entry| {
                entry
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .map_err(|e| HostFsError::Io {
                        path: path.to_path_buf(),
                        message: e.to_string(),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        names.sort();
        Ok(names)
    }
    #[cfg(target_arch = "wasm32")]
    {
        wasm_impl::list_dir(path)
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_impl {
    use super::HostFsError;
    use std::path::Path;
    use wasm_bindgen::prelude::*;

    // A snippet module rather than `module = "node:fs"`: the glue already
    // imports `readFileSync` to load the wasm, and wasm-bindgen emits a
    // second, colliding import for an extern of the same name.
    #[wasm_bindgen(inline_js = r#"
import { existsSync, readFileSync, readdirSync } from 'node:fs';
export function readText(path) { return readFileSync(path, 'utf8'); }
export function exists(path) { return existsSync(path); }
export function listDir(path) { return readdirSync(path); }
"#)]
    extern "C" {
        #[wasm_bindgen(catch, js_name = readText)]
        fn node_read_text(path: &str) -> Result<String, JsValue>;
        #[wasm_bindgen(js_name = exists)]
        fn node_exists(path: &str) -> bool;
        #[wasm_bindgen(catch, js_name = listDir)]
        fn node_list_dir(path: &str) -> Result<js_sys::Array, JsValue>;
    }

    fn io_error(path: &Path, error: JsValue) -> HostFsError {
        HostFsError::Io {
            path: path.to_path_buf(),
            message: format!("{error:?}"),
        }
    }

    pub(super) fn read_text(path: &Path) -> Result<String, HostFsError> {
        node_read_text(&path.to_string_lossy()).map_err(|error| io_error(path, error))
    }

    pub(super) fn exists(path: &Path) -> Result<bool, HostFsError> {
        Ok(node_exists(&path.to_string_lossy()))
    }

    pub(super) fn list_dir(path: &Path) -> Result<Vec<String>, HostFsError> {
        let entries =
            node_list_dir(&path.to_string_lossy()).map_err(|error| io_error(path, error))?;
        let mut names = entries
            .iter()
            .map(|entry| {
                entry.as_string().ok_or_else(|| HostFsError::Io {
                    path: path.to_path_buf(),
                    message: "readdirSync returned a non-string entry".to_string(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        names.sort();
        Ok(names)
    }
}
