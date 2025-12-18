//! Build script that auto-triggers `napi build` after cargo compilation completes.
//!
//! This solves the problem where `cargo build` alone doesn't produce the .node file
//! that Node.js needs. The script spawns a background watcher that runs `napi build`
//! after cargo finishes.
fn main() {
    napi_build::setup();

    #[cfg(unix)]
    napi_auto_build("macroforge_ts");
}

/// Automatically trigger `napi build` after cargo compilation completes.
#[cfg(unix)]
fn napi_auto_build(package_name: &str) {
    use std::process::Command;

    // Skip if NAPI_BUILD_SKIP_WATCHER is set (prevents recursion)
    if std::env::var("NAPI_BUILD_SKIP_WATCHER").is_ok() {
        return;
    }

    // Get parent PID to watch
    let parent_pid = unsafe { libc::getppid() };

    // Check if we're being called from within napi build (prevents recursion)
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
        return;
    }

    let crate_dir = std::env::current_dir().unwrap_or_default();
    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let release_flag = if profile == "release" {
        "--release"
    } else {
        ""
    };

    let script = format!(
        r#"
        while kill -0 {parent_pid} 2>/dev/null; do
            sleep 0.3
        done
        sleep 0.2
        cd "{crate_dir}"
        export NAPI_BUILD_SKIP_WATCHER=1
        if command -v npx >/dev/null 2>&1; then
            npx -y -p @napi-rs/cli napi build --platform {release_flag} -p {package_name} 2>/dev/null
        elif command -v bunx >/dev/null 2>&1; then
            bunx -y @napi-rs/cli napi build --platform {release_flag} -p {package_name} 2>/dev/null
        fi
        "#,
        parent_pid = parent_pid,
        crate_dir = crate_dir.display(),
        release_flag = release_flag,
        package_name = package_name,
    );

    let _ = Command::new("sh")
        .args(["-c", &format!("({}) &", script.trim())])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}
