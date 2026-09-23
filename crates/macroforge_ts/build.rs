//! Build script for macroforge_ts.
fn main() {
    // Without this, cargo reruns the script whenever any file in the package
    // changes and recompiles the crate with it. `pkg/` is written here by
    // wasm-bindgen on every build, so each wasm build invalidated the next one.
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(feature, values(\"node\"))");
    #[cfg(feature = "node")]
    {
        napi_build::setup();
    }
}
