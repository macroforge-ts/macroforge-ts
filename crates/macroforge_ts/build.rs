//! Build script for macroforge_ts.
fn main() {
    println!("cargo::rustc-check-cfg=cfg(feature, values(\"node\"))");
    #[cfg(feature = "node")]
    {
        napi_build::setup();
    }
}
