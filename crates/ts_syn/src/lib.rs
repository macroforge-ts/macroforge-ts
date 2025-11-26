pub mod derive;
pub mod errors;
pub mod lower;
pub mod parse;
pub mod stream;

pub use derive::*;
pub use errors::*;
pub use lower::*;
#[cfg(feature = "swc")]
pub use quote::*;
pub use stream::*;

// Re-export swc_core for convenience
#[cfg(feature = "swc")]
pub use swc_core;

// Re-export common swc modules at top level for ergonomics
#[cfg(feature = "swc")]
pub use swc_core::common as swc_common;
#[cfg(feature = "swc")]
pub use swc_core::ecma::ast as swc_ecma_ast;
