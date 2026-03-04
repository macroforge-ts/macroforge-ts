//! Debug logging for external macros.
//!
//! Writes timestamped log entries to `.macroforge/debug.log` relative to the
//! project root (discovered by walking up from CWD). The file is created on
//! first write and appended to thereafter.
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
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::ts_syn::abi::{MacroContextIR, MacroResult, TargetIR};

/// Resolved path to the log file (computed once per process).
static LOG_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    // Walk up from CWD looking for an existing `.macroforge` directory
    let mut dir = cwd.as_path();
    loop {
        let candidate = dir.join(".macroforge");
        if candidate.is_dir() {
            return candidate.join("debug.log");
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    // Fallback: create `.macroforge` in CWD
    let fallback = cwd.join(".macroforge");
    let _ = std::fs::create_dir_all(&fallback);
    fallback.join("debug.log")
});

fn timestamp() -> String {
    let now = std::time::SystemTime::now();
    let dur = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    let millis = dur.subsec_millis();
    format!("{secs}.{millis:03}")
}

/// Append a single line to the debug log.
pub fn log(tag: &str, msg: &str) {
    let line = format!("[{}] [{}] {}\n", timestamp(), tag, msg);
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_PATH.as_path())
        .and_then(|mut f| f.write_all(line.as_bytes()));
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
    let _ = std::fs::write(LOG_PATH.as_path(), "");
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
