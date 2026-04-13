//! Sandbox trait + value/error/options types.
//!
//! The [`BuildtimeSandbox`] trait is the single interface the pre-pass uses
//! to invoke a JavaScript runtime. The only backend today is Boa
//! ([`crate::host::buildtime::backends::boa`]), pure Rust, works on
//! native + wasm32.
//!
//! All types here are runtime-only — they never appear in user-facing
//! TypeScript. The serializer ([`crate::host::buildtime::serialize`])
//! converts [`SandboxValue`] to TS source.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::host::buildtime::capabilities::CapabilitySet;

/// Values the sandbox can return to the host.
///
/// These map 1:1 to JSON-compatible JavaScript values, with two extensions:
///
/// - [`SandboxValue::BigInt`] for arbitrary integers (serialized as `42n`).
/// - [`SandboxValue::SourceCode`] for Tier 2 `@buildtime function` results
///   that return a string of TypeScript source to splice verbatim.
///
/// Unsupported JS values (functions, classes, symbols, circular structures)
/// cause the backend to return [`SandboxError::UnserializableResult`]
/// rather than coercing to a lossy representation.
#[derive(Debug, Clone, PartialEq)]
pub enum SandboxValue {
    Null,
    Undefined,
    Bool(bool),
    Number(f64),
    BigInt(i64),
    String(String),
    Array(Vec<SandboxValue>),
    Object(BTreeMap<String, SandboxValue>),
    /// Tier 2: the `@buildtime function` returned a string and the caller
    /// asked for source-code semantics. The string is spliced verbatim.
    SourceCode(String),
}

impl SandboxValue {
    /// True if this value is `null` or `undefined`.
    #[must_use]
    pub fn is_nullish(&self) -> bool {
        matches!(self, Self::Null | Self::Undefined)
    }

    /// A short human-readable description of the variant — used in
    /// error messages (e.g. `"function"`, `"symbol"`).
    #[must_use]
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::Undefined => "undefined",
            Self::Bool(_) => "boolean",
            Self::Number(_) => "number",
            Self::BigInt(_) => "bigint",
            Self::String(_) => "string",
            Self::Array(_) => "array",
            Self::Object(_) => "object",
            Self::SourceCode(_) => "source-code",
        }
    }
}

/// Options passed to the sandbox for a single evaluation.
///
/// Capabilities live on this struct rather than on the sandbox itself so
/// each evaluation can have its own capability set — a `@buildtime`
/// declaration in file A can't smuggle filesystem access into file B.
#[derive(Debug, Clone)]
pub struct SandboxOptions {
    /// Which filesystem/network/env capabilities the sandbox has.
    pub capabilities: CapabilitySet,

    /// Maximum wall-clock time the evaluation may run before it's killed.
    /// Backends enforce this via their interrupt/event-loop machinery.
    pub timeout: Duration,

    /// Maximum bytes the JS heap may allocate. Backends enforce this
    /// through their memory-limit API (e.g. `JS_SetMemoryLimit` for
    /// Boa enforces this through its memory tracking; callers can also
    /// abandon contexts that go over budget.
    pub max_heap: usize,

    /// Absolute path of the source file the `@buildtime` declaration
    /// was lifted from. Available inside the sandbox as
    /// `buildtime.location.file`.
    pub source_file: PathBuf,

    /// 1-based line number of the `@buildtime` JSDoc annotation in the
    /// source file. Available as `buildtime.location.line`.
    pub source_line: u32,

    /// 1-based column number of the `@buildtime` JSDoc annotation in the
    /// source file. Available as `buildtime.location.column`.
    pub source_column: u32,
}

impl SandboxOptions {
    /// Construct default options anchored at `source_file`.
    ///
    /// `capabilities` defaults to the empty set, `timeout` to 5 seconds,
    /// and `max_heap` to 256 MB — all values matching the config defaults
    /// documented in `plans/buildtime.md`.
    #[must_use]
    pub fn new(source_file: PathBuf) -> Self {
        Self {
            capabilities: CapabilitySet::default(),
            timeout: Duration::from_secs(5),
            max_heap: 256 * 1024 * 1024,
            source_file,
            source_line: 1,
            source_column: 1,
        }
    }
}

/// The full payload a successful sandbox evaluation returns.
#[derive(Debug, Clone)]
pub struct EvalResult {
    /// Whatever the user's expression evaluated to.
    pub value: SandboxValue,

    /// Absolute paths of every file the sandbox read via the `buildtime.fs`
    /// API during evaluation. The host uses this list for incremental
    /// rebuilds — if any of these files change, the declaration must be
    /// re-evaluated.
    pub dependencies: Vec<PathBuf>,
}

/// Failures the sandbox may report.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SandboxError {
    /// The script didn't finish within [`SandboxOptions::timeout`].
    #[error("script timed out after {duration:?}")]
    Timeout { duration: Duration },

    /// The script allocated more than [`SandboxOptions::max_heap`] bytes.
    #[error("script exceeded heap limit of {limit} bytes")]
    OutOfMemory { limit: usize },

    /// A `buildtime.fs.read*` call tried to touch a path not in the
    /// capability allowlist.
    #[error("script tried to read disallowed path {}", .path.display())]
    UnauthorizedRead { path: PathBuf },

    /// A `buildtime.fs.write*` call tried to touch a path not in the
    /// capability allowlist.
    #[error("script tried to write disallowed path {}", .path.display())]
    UnauthorizedWrite { path: PathBuf },

    /// The script accessed `buildtime.env[X]` for an `X` not in the
    /// env allowlist.
    #[error("script tried to read disallowed env var {var}")]
    UnauthorizedEnv { var: String },

    /// The script attempted network access while `capabilities.network`
    /// was false.
    #[error("script tried to make network request to {url} (network capability is off)")]
    UnauthorizedNetwork { url: String },

    /// The script returned a value that can't be expressed as a TS
    /// literal (e.g. a function, a class instance, a symbol, a value
    /// with circular references).
    #[error("script returned an unsupported value: {kind}")]
    UnserializableResult { kind: String },

    /// The script threw a JS error.
    #[error("script threw: {message}")]
    Threw { message: String, stack: String },

    /// Some other backend-level error. These are lumped together because
    /// they aren't user-actionable in a structured way; the message is
    /// surfaced verbatim.
    #[error("backend error: {0}")]
    Backend(String),
}

/// Trait every sandbox backend implements.
///
/// Implementations must be `Send + Sync` so the pre-pass can hold a
/// trait object alongside `MacroExpander` in thread-local state. They do
/// **not** need to be reusable across evaluations — backends are free to
/// create a fresh context per call, and the higher-level pool (PR 12)
/// adds reuse where it matters.
pub trait BuildtimeSandbox: Send + Sync {
    /// Human-readable backend name, e.g. `"boa"`. Used in diagnostics.
    fn name(&self) -> &'static str;

    /// Evaluate a JavaScript module source in the sandbox.
    ///
    /// `source` is the full text of a synthetic module (the pre-pass
    /// wraps user code in a try/catch that stashes the result on
    /// `globalThis.__macroforgeResult`). `origin` is the file path
    /// displayed in stack traces — usually `<buildtime>` followed by the
    /// user's source file name.
    ///
    /// Implementations read `__macroforgeResult` from the global scope
    /// after execution and convert it to a [`SandboxValue`].
    ///
    /// Contract: this call runs synchronously and does not return until
    /// either the top-level promise resolves, the timeout fires, or an
    /// error occurs. Backends that are naturally async (V8, deno_core)
    /// drive their event loop internally.
    fn evaluate(
        &self,
        source: &str,
        origin: &Path,
        options: &SandboxOptions,
    ) -> Result<EvalResult, SandboxError>;
}
