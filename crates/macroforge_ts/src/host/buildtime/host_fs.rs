//! Host filesystem bridge for `buildtime.fs.*`.
//!
//! On native targets every call goes straight through `std::fs`. On
//! wasm32-unknown-unknown there's no filesystem in the runtime, so the
//! host JS environment must register synchronous read callbacks via
//! [`setup_buildtime_fs`]. The Vite plugin wires this up at startup.
//!
//! Both paths return the same `Result<T, HostFsError>`, so callers
//! (the Boa backend's `fs_*_impl` helpers) don't need to know which
//! target they're running on.

use std::path::Path;

#[derive(Debug, Clone, thiserror::Error)]
pub enum HostFsError {
    #[error("fs not available: {0}")]
    NotAvailable(String),
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
        let mut names: Vec<String> = iter
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
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
    use std::sync::OnceLock;
    use wasm_bindgen::prelude::*;

    /// Three JS callbacks the host plugin registers at startup.
    /// All three are *synchronous* — they're invoked from inside the
    /// Boa sandbox, which doesn't allow async at native call sites.
    pub(super) struct HostFsCallbacks {
        pub read_text: js_sys::Function,
        pub exists: js_sys::Function,
        pub list_dir: js_sys::Function,
    }

    static CALLBACKS: OnceLock<HostFsCallbacks> = OnceLock::new();

    /// Register JS callbacks that bridge `buildtime.fs.*` to the host
    /// filesystem. Called once at startup by the Vite plugin (and any
    /// other JS host). Subsequent calls are ignored.
    #[wasm_bindgen(js_name = "setupBuildtimeFs")]
    pub fn setup_buildtime_fs(
        read_text: js_sys::Function,
        exists: js_sys::Function,
        list_dir: js_sys::Function,
    ) {
        let _ = CALLBACKS.set(HostFsCallbacks {
            read_text,
            exists,
            list_dir,
        });
    }

    fn callbacks() -> Result<&'static HostFsCallbacks, HostFsError> {
        CALLBACKS.get().ok_or_else(|| {
            HostFsError::NotAvailable(
                "no JS host registered fs callbacks; call setupBuildtimeFs() first".to_string(),
            )
        })
    }

    pub(super) fn read_text(path: &Path) -> Result<String, HostFsError> {
        let cb = callbacks()?;
        let arg = JsValue::from_str(&path.to_string_lossy());
        let result = cb
            .read_text
            .call1(&JsValue::NULL, &arg)
            .map_err(|e| HostFsError::Io {
                path: path.to_path_buf(),
                message: format!("{e:?}"),
            })?;
        if result.is_null() || result.is_undefined() {
            return Err(HostFsError::Io {
                path: path.to_path_buf(),
                message: "host returned null/undefined".to_string(),
            });
        }
        result.as_string().ok_or_else(|| HostFsError::Io {
            path: path.to_path_buf(),
            message: "host returned non-string".to_string(),
        })
    }

    pub(super) fn exists(path: &Path) -> Result<bool, HostFsError> {
        let cb = callbacks()?;
        let arg = JsValue::from_str(&path.to_string_lossy());
        let result = cb
            .exists
            .call1(&JsValue::NULL, &arg)
            .map_err(|e| HostFsError::Io {
                path: path.to_path_buf(),
                message: format!("{e:?}"),
            })?;
        Ok(result.as_bool().unwrap_or(false))
    }

    pub(super) fn list_dir(path: &Path) -> Result<Vec<String>, HostFsError> {
        let cb = callbacks()?;
        let arg = JsValue::from_str(&path.to_string_lossy());
        let result = cb
            .list_dir
            .call1(&JsValue::NULL, &arg)
            .map_err(|e| HostFsError::Io {
                path: path.to_path_buf(),
                message: format!("{e:?}"),
            })?;
        let array = js_sys::Array::from(&result);
        let mut names: Vec<String> = (0..array.length())
            .filter_map(|i| array.get(i).as_string())
            .collect();
        names.sort();
        Ok(names)
    }
}
