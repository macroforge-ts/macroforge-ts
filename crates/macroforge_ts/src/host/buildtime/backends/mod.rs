//! Backend implementations of [`BuildtimeSandbox`].
//!
//! Only the Boa backend ships today — pure Rust, compiles for native
//! + wasm32, no C toolchain required.
//!
//! [`BuildtimeSandbox`]: crate::host::buildtime::sandbox::BuildtimeSandbox

#[cfg(feature = "buildtime-boa")]
pub mod boa;
