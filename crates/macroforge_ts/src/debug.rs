//! Debug logging for external macros.
//!
//! Writes timestamped log entries to `.macroforge/debug.log` in the macroforge
//! project the logging code is working on: the nearest ancestor holding a
//! `macroforge.config.*`. Outside such a project nothing is written. The file
//! is created on first write and appended to thereafter.
//!
//! On WASM (`wasm32-unknown-unknown`), falls back to `eprintln!` since there
//! is no filesystem access.
//!
//! # Usage
//!
//! ```rust,ignore
//! use macroforge_ts::debug;
//!
//! debug::log("Gigaform", "Starting expansion for User");
//! debug::log_ctx("Gigaform", &ctx);            // logs the full MacroContextIR
//! debug::log_result("Gigaform", &result);       // logs patch/diagnostic counts
//! ```
//!
//! Or use the [`debug_log!`] macro for formatted messages:
//!
//! ```rust,ignore
//! macroforge_ts::debug_log!("MyMacro", "processing {type_name} with {n} fields");
//! ```

use std::fmt::Write as FmtWrite;

use crate::ts_syn::abi::{MacroContextIR, MacroResult, TargetIR};

#[cfg(not(target_arch = "wasm32"))]
mod fs_log {
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::sync::LazyLock;

    /// Log file of the project containing the working directory (computed once
    /// per process). Used when the caller does not say which file it is on.
    static CWD_LOG_PATH: LazyLock<Option<PathBuf>> = LazyLock::new(|| {
        std::env::current_dir()
            .ok()
            .and_then(|cwd| log_path_for(&cwd))
    });

    /// `.macroforge/debug.log` of the nearest ancestor of `start` that holds a
    /// macroforge config, or `None` when `start` is in no macroforge project.
    pub fn log_path_for(start: &Path) -> Option<PathBuf> {
        start
            .ancestors()
            .find(|dir| {
                crate::host::config::CONFIG_FILES
                    .iter()
                    .any(|name| dir.join(name).is_file())
            })
            .map(|root| root.join(".macroforge").join("debug.log"))
    }

    /// Log file of the project containing `file`; relative paths are taken
    /// from the working directory.
    pub fn log_path_for_file(file: &str) -> Option<PathBuf> {
        let path = Path::new(file);
        if path.is_absolute() {
            return log_path_for(path);
        }
        let cwd = std::env::current_dir().ok()?;
        log_path_for(&cwd.join(path))
    }

    pub fn cwd_log_path() -> Option<&'static Path> {
        CWD_LOG_PATH.as_deref()
    }

    pub fn append(log_path: &Path, text: &str) {
        let written = log_path
            .parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| {
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(log_path)
            })
            .and_then(|mut f| f.write_all(text.as_bytes()));
        if let Err(err) = written {
            eprintln!("[macroforge] cannot write {}: {err}", log_path.display());
        }
    }

    pub fn clear(log_path: &Path) {
        if let Err(err) = std::fs::write(log_path, "")
            && err.kind() != std::io::ErrorKind::NotFound
        {
            eprintln!("[macroforge] cannot clear {}: {err}", log_path.display());
        }
    }
}

fn timestamp() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = std::time::SystemTime::now();
        let dur = now
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = dur.as_secs();
        let millis = dur.subsec_millis();
        format!("{secs}.{millis:03}")
    }
    #[cfg(target_arch = "wasm32")]
    {
        "wasm".to_string()
    }
}

fn format_line(tag: &str, msg: &str) -> String {
    format!("[{}] [{}] {}\n", timestamp(), tag, msg)
}

/// Append a single line to the debug log of the project containing the
/// working directory.
pub fn log(tag: &str, msg: &str) {
    let line = format_line(tag, msg);
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(log_path) = fs_log::cwd_log_path() {
            fs_log::append(log_path, &line);
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        eprintln!("{}", line.trim_end());
    }
}

/// Append lines to the debug log of the project containing `file`, the file
/// the lines are about.
pub(crate) fn log_for_file(file: &str, tag: &str, msgs: &[String]) {
    if msgs.is_empty() {
        return;
    }
    let text: String = msgs.iter().map(|msg| format_line(tag, msg)).collect();
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(log_path) = fs_log::log_path_for_file(file) {
            fs_log::append(&log_path, &text);
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        eprint!("{file}:\n{text}");
    }
}

/// Log the `MacroContextIR` summary (macro name, module, file, target kind, field count).
pub fn log_ctx(tag: &str, ctx: &MacroContextIR) {
    let target_kind = match &ctx.target {
        TargetIR::Class(_) => "class",
        TargetIR::Interface(_) => "interface",
        TargetIR::Enum(_) => "enum",
        TargetIR::TypeAlias(_) => "type_alias",
        _ => "other",
    };

    let field_count = match &ctx.target {
        TargetIR::Class(c) => c.fields.len(),
        TargetIR::Interface(i) => i.fields.len(),
        TargetIR::Enum(e) => e.variants.len(),
        _ => 0,
    };

    let mut buf = String::new();
    let _ = write!(
        buf,
        "ctx {{ macro: {}::{}, file: {}, target: {} ({} fields), span: {}-{} }}",
        ctx.module_path,
        ctx.macro_name,
        ctx.file_name,
        target_kind,
        field_count,
        ctx.decorator_span.start,
        ctx.decorator_span.end,
    );
    log(tag, &buf);
}

/// Log a `MacroResult` summary (patch counts, diagnostic counts, token length).
pub fn log_result(tag: &str, result: &MacroResult) {
    let mut buf = String::new();
    let _ = write!(
        buf,
        "result {{ runtime_patches: {}, type_patches: {}, tokens: {}, diagnostics: {} }}",
        result.runtime_patches.len(),
        result.type_patches.len(),
        result
            .tokens
            .as_ref()
            .map(|t| t.len().to_string())
            .unwrap_or_else(|| "None".to_string()),
        result.diagnostics.len(),
    );

    for diag in &result.diagnostics {
        let _ = write!(buf, "\n  [{:?}] {}", diag.level, diag.message);
        if let Some(help) = &diag.help {
            let _ = write!(buf, " (help: {help})");
        }
    }

    log(tag, &buf);
}

/// Clear the debug log (useful at the start of a build).
pub fn clear() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if let Some(log_path) = fs_log::cwd_log_path() {
            fs_log::clear(log_path);
        }
    }
}

/// Log a formatted message.
///
/// # Examples
///
/// ```rust,ignore
/// macroforge_ts::debug_log!("MyMacro", "processing {type_name} with {n} fields");
/// ```
#[macro_export]
macro_rules! debug_log {
    ($tag:expr, $($arg:tt)*) => {
        $crate::debug::log($tag, &format!($($arg)*))
    };
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    fn write_config(dir: &std::path::Path) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join("macroforge.config.ts"), "export default {};\n").unwrap();
    }

    #[test]
    fn log_path_is_the_nearest_macroforge_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let outer = temp.path().join("outer");
        let inner = outer.join("packages").join("inner");
        write_config(&outer);
        write_config(&inner);
        let nested = inner.join("src").join("deep");
        std::fs::create_dir_all(&nested).unwrap();

        assert_eq!(
            fs_log::log_path_for(&nested),
            Some(inner.join(".macroforge").join("debug.log"))
        );
    }

    #[test]
    fn log_path_is_none_outside_a_macroforge_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let plain = temp.path().join("plain").join("src");
        std::fs::create_dir_all(&plain).unwrap();

        assert_eq!(fs_log::log_path_for(&plain), None);
    }

    #[test]
    fn log_for_file_writes_only_inside_the_file_project() {
        let temp = tempfile::TempDir::new().unwrap();
        let project = temp.path().join("project");
        write_config(&project);
        let plain = temp.path().join("plain");
        std::fs::create_dir_all(&plain).unwrap();

        let lines = vec!["hello".to_string()];
        log_for_file(
            project.join("src").join("a.ts").to_str().unwrap(),
            "test",
            &lines,
        );
        log_for_file(plain.join("b.ts").to_str().unwrap(), "test", &lines);

        let log = std::fs::read_to_string(project.join(".macroforge").join("debug.log")).unwrap();
        assert!(log.contains("[test] hello"), "unexpected log: {log}");
        assert!(!plain.join(".macroforge").exists());
        assert!(!temp.path().join(".macroforge").exists());
    }
}
