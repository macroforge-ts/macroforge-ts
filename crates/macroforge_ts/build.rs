//! Build script for macroforge_ts.
fn main() {
    #[cfg(feature = "node")]
    {
        napi_build::setup();
    }
}
