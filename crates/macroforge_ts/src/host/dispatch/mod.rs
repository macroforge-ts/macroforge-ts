//! # Macro Dispatch and Execution
//!
//! The dispatcher is responsible for routing macro calls to their implementations
//! and handling the execution lifecycle, including:
//!
//! - Looking up macros in the registry (with fallback resolution)
//! - ABI version compatibility checking
//! - Creating the `TsStream` input for macros
//! - Catching panics and converting them to diagnostics
//!
//! ## Dispatch Flow
//!
//! ```text
//! MacroContextIR
//!       │
//!       ▼
//! ┌─────────────────┐
//! │  Registry Lookup │  (with module fallback)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ ABI Version Check│  (reject if mismatch)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Create TsStream  │  (parse context)
//! └────────┬────────┘
//!          │
//!          ▼
//! ┌─────────────────┐
//! │ Execute Macro    │  (with panic catching)
//! └────────┬────────┘
//!          │
//!          ▼
//!    MacroResult
//! ```
//!
//! ## Error Handling
//!
//! The dispatcher never panics. All errors are converted to diagnostics:
//! - Macro not found (both module-path and name-only fallback failed) → Error diagnostic
//! - ABI mismatch → Error diagnostic with version info
//! - TsStream creation failure → Error diagnostic with details
//! - Macro panic → Error diagnostic with panic message

mod dispatcher;

#[cfg(test)]
mod tests;

pub use dispatcher::MacroDispatcher;
