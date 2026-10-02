//! Capability set for a single sandbox evaluation.
//!
//! Each `@buildtime` declaration runs with its own [`CapabilitySet`],
//! populated from the `buildtime.capabilities` block in
//! `macroforge.config.js`. Every `buildtime.fs.*` read must match an
//! allowed glob, and `buildtime.env` holds only allowlisted names. There is
//! no write or network API. An unauthorized read surfaces as a
//! [`SandboxError`] with a span pointing at the declaration.
//!
//! Path matching is glob-based (via the `globset` crate). Patterns are
//! matched against a *lexically* normalized absolute path (`.`/`..`
//! components resolved textually): symlinks are not resolved, so a
//! symlink pointing outside an allowed directory still matches.
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
/// The default is fully restricted: no reads and no env access. Writes and
/// network access are never granted; the sandbox exposes no API for them.
/// The host populates this from config and hands it to the sandbox through
/// [`SandboxOptions`].
///
/// [`SandboxOptions`]: crate::host::buildtime::sandbox::SandboxOptions
#[derive(Debug, Clone, Default)]
pub struct CapabilitySet {
    /// Patterns that permit filesystem reads. Empty = no reads allowed.
    pub fs_read: Vec<PathPattern>,
    /// Env variable names the sandbox may read. Empty = no env access.
    pub env_allow: Vec<String>,
}

impl CapabilitySet {
    /// A capability set permitting every filesystem read. `env_allow`
    /// stays empty, so env reads are still denied. **Tests only**: production code should construct
    /// a set from user config.
    #[cfg(any(test, doctest))]
    #[must_use]
    pub fn unrestricted() -> Self {
        Self {
            fs_read: vec![PathPattern::new("**").expect("'**' is a valid glob")],
            env_allow: vec![],
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
}

/// Errors from capability checks.
///
/// These are lower-level than `SandboxError`: the backend converts them
/// to `SandboxError::UnauthorizedRead` before returning to the host.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CapabilityError {
    #[error("read not permitted: {}", .path.display())]
    ReadDenied { path: PathBuf },
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
