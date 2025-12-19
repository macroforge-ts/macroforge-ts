//! Build script that auto-triggers `napi build` after cargo compilation completes.
//!
//! Uses the macroforge_ts build utility to automatically generate the .node file
//! when `cargo build` finishes.

fn main() {
    napi_build::setup();

    // Auto-trigger napi build after cargo completes
    macroforge_ts::build::napi_auto_build("playground-macros");
}
