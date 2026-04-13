//! # `@buildtime` evaluation
//!
//! Compile-time JavaScript execution for macroforge. Semantically modelled
//! on Zig's `comptime`: a declaration annotated with `/** @buildtime */` is
//! evaluated in a sandboxed JS context during source transformation, its
//! result is serialized back to a TypeScript literal, and the original
//! declaration is replaced with that literal. Subsequent passes (declarative
//! macros, derive macros) see the spliced-in result, not the original
//! expression.
//!
//! This module holds the backend-agnostic pieces: the [`BuildtimeSandbox`]
//! trait, value and error types, capability enforcement, and the serializer
//! that converts sandbox values to TS source. The only backend today is
//! Boa ([`backends::boa`]) — a pure-Rust JS engine that compiles to both
//! native targets and wasm32-unknown-unknown, which is what the Vite
//! plugin ships. Earlier prototypes included QuickJS, V8, and deno_core
//! backends; they've been dropped in favour of Boa for portability.
//!
//! ## High-level flow
//!
//! ```text
//! source.ts
//!     │
//!     ▼
//! discovery::find_declarations      ← walks the OXC AST for /** @buildtime */
//!     │
//!     ▼
//! prepass::evaluate                 ← for each decl: synthetic module → sandbox
//!     │
//!     ▼
//! serialize::value_to_ts_source     ← SandboxValue → TS literal
//!     │
//!     ▼
//! Patch::Replace { span, code }     ← fed to the existing PatchApplicator
//! ```

pub mod backends;
pub mod capabilities;
pub mod discovery;
pub mod host_fs;
pub mod prepass;
#[cfg(feature = "oxc")]
pub mod purity;
pub mod sandbox;
pub mod serialize;

#[cfg(test)]
mod tests;

pub use capabilities::{CapabilityError, CapabilitySet, PathPattern};
pub use discovery::{BuildtimeDecl, BuildtimeKind, Visibility, discover};
pub use prepass::{PrepassOutput, run_prepass};
#[cfg(feature = "oxc")]
pub use purity::{Purity, analyze as analyze_purity};
pub use sandbox::{BuildtimeSandbox, EvalResult, SandboxError, SandboxOptions, SandboxValue};
pub use serialize::{SerializeError, value_to_ts_source};

/// Returns the default sandbox backend for the current build.
///
/// Currently only the Boa backend is available; it compiles for every
/// target macroforge runs on. Returns `None` if the `buildtime-boa`
/// feature is disabled — callers treat that as a no-op pre-pass.
#[must_use]
pub fn default_backend() -> Option<Box<dyn BuildtimeSandbox>> {
    #[cfg(feature = "buildtime-boa")]
    {
        Some(Box::new(backends::boa::BoaSandbox::new()))
    }
    #[cfg(not(feature = "buildtime-boa"))]
    {
        None
    }
}
