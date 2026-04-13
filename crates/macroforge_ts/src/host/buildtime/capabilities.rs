//! Capability set for a single sandbox evaluation.
//!
//! Each `@buildtime` declaration runs with its own [`CapabilitySet`],
//! populated from the `buildtime.capabilities` block in
//! `macroforge.config.js`. Every `buildtime.fs.*`, `buildtime.env`, and
//! network call must be pre-approved — unauthorized calls surface as a
//! [`SandboxError`] with a span pointing at the declaration.
//!
//! Path matching is glob-based (via the `globset` crate). Patterns are
//! matched against the canonicalized absolute path when the host can
//! resolve one; otherwise the path is used as-is.
//!
//! [`SandboxError`]: crate::host::buildtime::sandbox::SandboxError

use globset::{Glob, GlobMatcher};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// A glob pattern that matches filesystem paths.
///
/// Stored as both the original pattern string (for diagnostic messages
/// and debugging) and the compiled [`GlobMatcher`] (for fast matching).
#[derive(Debug, Clone)]
pub struct PathPattern {
    pattern: String,
    matcher: GlobMatcher,
}

impl PathPattern {
    /// Compile a glob pattern. Uses the default `globset` syntax:
    /// `*` matches any path component character, `**` matches across
    /// component boundaries, `?` matches a single character.
    pub fn new(pattern: impl Into<String>) -> Result<Self, CapabilityError> {
        let pattern = pattern.into();
        let matcher = Glob::new(&pattern)
            .map_err(|e| CapabilityError::InvalidGlob {
                pattern: pattern.clone(),
                reason: e.to_string(),
            })?
            .compile_matcher();
        Ok(Self { pattern, matcher })
    }

    /// Return the original pattern string for diagnostics.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.pattern
    }

    /// True if `path` matches this pattern.
    #[must_use]
    pub fn matches(&self, path: &Path) -> bool {
        self.matcher.is_match(path)
    }
}

impl PartialEq for PathPattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

/// Set of capabilities granted for a single evaluation.
///
/// The default is fully restricted: no reads, no writes, no network,
/// no env access. The host populates this from config and hands it to
/// the sandbox through [`SandboxOptions`].
///
/// [`SandboxOptions`]: crate::host::buildtime::sandbox::SandboxOptions
#[derive(Debug, Clone, Default)]
pub struct CapabilitySet {
    /// Patterns that permit filesystem reads. Empty = no reads allowed.
    pub fs_read: Vec<PathPattern>,
    /// Patterns that permit filesystem writes. Empty = no writes allowed.
    /// Writes inside `@buildtime` are *strongly* discouraged; they break
    /// cache invariants and are gated behind an explicit allowlist.
    pub fs_write: Vec<PathPattern>,
    /// Env variable names the sandbox may read. Empty = no env access.
    pub env_allow: Vec<String>,
    /// Whether the sandbox may make network calls.
    pub network: bool,
}

impl CapabilitySet {
    /// A fully-permissive capability set. **Tests only** — production
    /// code should construct a set from user config.
    #[cfg(any(test, doctest))]
    #[must_use]
    pub fn unrestricted() -> Self {
        Self {
            fs_read: vec![PathPattern::new("**").expect("'**' is a valid glob")],
            fs_write: vec![PathPattern::new("**").expect("'**' is a valid glob")],
            env_allow: vec![],
            network: true,
        }
    }

    /// Check whether reads from `path` are permitted.
    pub fn check_read(&self, path: &Path) -> Result<(), CapabilityError> {
        if self.fs_read.iter().any(|p| p.matches(path)) {
            Ok(())
        } else {
            Err(CapabilityError::ReadDenied {
                path: path.to_path_buf(),
            })
        }
    }

    /// Check whether writes to `path` are permitted.
    pub fn check_write(&self, path: &Path) -> Result<(), CapabilityError> {
        if self.fs_write.iter().any(|p| p.matches(path)) {
            Ok(())
        } else {
            Err(CapabilityError::WriteDenied {
                path: path.to_path_buf(),
            })
        }
    }

    /// Check whether reading env variable `key` is permitted.
    pub fn check_env(&self, key: &str) -> Result<(), CapabilityError> {
        if self.env_allow.iter().any(|k| k == key) {
            Ok(())
        } else {
            Err(CapabilityError::EnvDenied {
                var: key.to_string(),
            })
        }
    }

    /// Check whether a network call to `url` is permitted. Currently
    /// network is a binary switch; per-host allowlists could be added
    /// later without breaking callers.
    pub fn check_network(&self, url: &str) -> Result<(), CapabilityError> {
        if self.network {
            Ok(())
        } else {
            Err(CapabilityError::NetworkDenied {
                url: url.to_string(),
            })
        }
    }
}

/// Errors from capability checks.
///
/// These are lower-level than `SandboxError` — the backend converts them
/// to `SandboxError::Unauthorized*` before returning to the host.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CapabilityError {
    #[error("read not permitted: {}", .path.display())]
    ReadDenied { path: PathBuf },
    #[error("write not permitted: {}", .path.display())]
    WriteDenied { path: PathBuf },
    #[error("env var not permitted: {var}")]
    EnvDenied { var: String },
    #[error("network not permitted: {url}")]
    NetworkDenied { url: String },
    #[error("invalid glob pattern {pattern:?}: {reason}")]
    InvalidGlob { pattern: String, reason: String },
}

/// Describes how capabilities are enforced at timing boundaries.
///
/// Kept as a standalone struct rather than fields on [`CapabilitySet`]
/// because `Duration` doesn't implement some traits the rest of the
/// struct needs, and this also keeps the per-capability shape clean.
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub timeout: Duration,
    pub max_heap: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            max_heap: 256 * 1024 * 1024,
        }
    }
}
