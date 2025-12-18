//! Build utilities for macroforge napi-rs projects.
//!
//! This module provides utilities for build.rs scripts in napi-rs projects
//! that use macroforge. The main feature is automatic triggering of `napi build`
//! after `cargo build` completes.
//!
//! # Problem
//!
//! napi-rs projects require two build steps:
//! 1. `cargo build` - Compiles Rust to a dynamic library (.dylib/.so/.dll)
//! 2. `napi build` - Wraps the library into a .node file and generates index.js
//!
//! Running `cargo build` alone doesn't produce a usable Node.js module, which
//! confuses developers and AI agents who expect it to "just work".
//!
//! # Solution
//!
//! This module provides [`napi_auto_build`] which spawns a background watcher
//! that detects when cargo finishes and automatically runs `napi build`.
//!
//! # Usage
//!
//! In your `build.rs`:
//!
//! ```rust,ignore
//! fn main() {
//!     napi_build::setup();
//!
//!     // Auto-trigger napi build after cargo completes
//!     macroforge_ts::build::napi_auto_build("my-package-name");
//! }
//! ```
//!
//! Add `libc` to your build-dependencies in `Cargo.toml`:
//!
//! ```toml
//! [build-dependencies]
//! napi-build = "2"
//! libc = "0.2"
//! macroforge_ts = { version = "...", features = ["build"] }
//! ```
//!
//! Now `cargo build --release` will automatically produce a working .node file!

/// Configuration for the napi auto-build feature.
#[derive(Debug, Clone)]
pub struct NapiAutoBuildConfig {
    /// The cargo package name (used with -p flag)
    pub package_name: String,
    /// Use npx (true) or bunx (false) as fallback
    pub prefer_npx: bool,
    /// Additional arguments to pass to napi build
    pub extra_args: Vec<String>,
}

impl NapiAutoBuildConfig {
    /// Create a new config with the given package name.
    pub fn new(package_name: impl Into<String>) -> Self {
        Self {
            package_name: package_name.into(),
            prefer_npx: true,
            extra_args: vec![],
        }
    }

    /// Add extra arguments to pass to napi build.
    pub fn with_args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.extra_args = args.into_iter().map(|s| s.into()).collect();
        self
    }
}

/// Automatically trigger `napi build` after cargo compilation completes.
///
/// This function spawns a background process that:
/// 1. Waits for the parent cargo process to exit
/// 2. Runs `napi build` to generate the .node file
///
/// It includes safeguards to prevent infinite recursion when napi build
/// calls cargo internally.
///
/// # Arguments
///
/// * `package_name` - The cargo package name (passed to `napi build -p`)
///
/// # Example
///
/// ```rust,ignore
/// // In build.rs
/// fn main() {
///     napi_build::setup();
///     macroforge_ts::build::napi_auto_build("my-package");
/// }
/// ```
///
/// # Platform Support
///
/// Currently only supported on Unix systems (macOS, Linux).
/// On other platforms, this function is a no-op.
#[cfg(unix)]
pub fn napi_auto_build(package_name: impl Into<String>) {
    napi_auto_build_with_config(NapiAutoBuildConfig::new(package_name));
}

#[cfg(not(unix))]
pub fn napi_auto_build(_package_name: impl Into<String>) {
    // No-op on non-Unix platforms
    println!("cargo:warning=napi_auto_build is only supported on Unix systems");
}

/// Automatically trigger `napi build` with custom configuration.
///
/// See [`napi_auto_build`] for details.
#[cfg(unix)]
pub fn napi_auto_build_with_config(config: NapiAutoBuildConfig) {
    use std::process::Command;

    // Skip if NAPI_BUILD_SKIP_WATCHER is set (prevents recursion)
    if std::env::var("NAPI_BUILD_SKIP_WATCHER").is_ok() {
        return;
    }

    // Get parent PID to watch
    let parent_pid = unsafe { libc::getppid() };

    // Check if we're being called from within napi build (prevents recursion)
    // by checking if napi/node is in the process ancestry
    let ancestry_check = Command::new("sh")
        .args([
            "-c",
            &format!(
                "ps -o command= -p $(ps -o ppid= -p {}) 2>/dev/null | grep -qE '(napi|node.*napi)'",
                parent_pid
            ),
        ])
        .status();

    if ancestry_check.map(|s| s.success()).unwrap_or(false) {
        // We're being called from napi build, skip to prevent recursion
        return;
    }

    let crate_dir = std::env::current_dir().unwrap_or_default();
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let release_flag = if profile == "release" {
        "--release"
    } else {
        ""
    };

    let extra_args = config.extra_args.join(" ");
    let package_name = config.package_name;

    // Create a shell script that waits for cargo to finish then runs napi build
    let script = format!(
        r#"
        # Wait for cargo (PID {parent_pid}) to finish
        while kill -0 {parent_pid} 2>/dev/null; do
            sleep 0.3
        done

        # Small delay to ensure all file handles are released
        sleep 0.2

        # Run napi build from the crate directory
        cd "{crate_dir}"

        # Set env var to prevent recursion
        export NAPI_BUILD_SKIP_WATCHER=1

        # Try npx first, then bunx
        if command -v npx >/dev/null 2>&1; then
            npx -y -p @napi-rs/cli napi build --platform {release_flag} -p {package_name} {extra_args} 2>/dev/null
        elif command -v bunx >/dev/null 2>&1; then
            bunx -y @napi-rs/cli napi build --platform {release_flag} -p {package_name} {extra_args} 2>/dev/null
        fi
        "#,
        parent_pid = parent_pid,
        crate_dir = crate_dir.display(),
        release_flag = release_flag,
        package_name = package_name,
        extra_args = extra_args,
    );

    // Spawn the watcher as a fully detached background process
    let _ = Command::new("sh")
        .args(["-c", &format!("({}) &", script.trim())])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

#[cfg(not(unix))]
pub fn napi_auto_build_with_config(_config: NapiAutoBuildConfig) {
    println!("cargo:warning=napi_auto_build is only supported on Unix systems");
}
