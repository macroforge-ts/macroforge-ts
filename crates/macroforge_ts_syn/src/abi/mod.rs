//! Application Binary Interface (ABI) types for stable macro communication.
//!
//! This module provides the foundational types for the Macroforge macro system's
//! ABI. These types are designed to be:
//!
//! - **Serializable**: All types implement `Serialize` and `Deserialize` for
//!   communication between the macro host and macro implementations.
//! - **Stable**: The structure is versioned and backward-compatible to ensure
//!   macros work across different versions.
//! - **Self-contained**: Types include all information needed for macro execution.
//!
//! ## Module Structure
//!
//! - [`ir`] - Intermediate representation types for TypeScript constructs
//! - [`span`] - Source span representation for position tracking
//! - [`patch`] - Code modification types for macro output
//! - [`source_map`] - Source mapping for expanded code positions
//! - [`helpers`] - Utility functions for common operations
//!
//! ## Key Types
//!
//! | Type | Purpose |
//! |------|---------|
//! | [`SpanIR`] | Byte-offset source spans |
//! | [`Patch`] | Code modification operations |
//! | [`MacroResult`] | Return type from macro functions |
//! | [`Diagnostic`] | Error/warning messages |
//! | [`MacroContextIR`] | Complete macro invocation context |
//!
//! ## Example: Macro Flow
//!
//! ```rust,ignore
//! use macroforge_ts_syn::{MacroContextIR, MacroResult, Patch, SpanIR};
//!
//! pub fn my_macro(ctx: MacroContextIR) -> MacroResult {
//!     // 1. Access the target from context
//!     let class = ctx.as_class().expect("Expected a class");
//!
//!     // 2. Generate code
//!     let code = format!("debug() {{ return '{}'; }}", class.name);
//!
//!     // 3. Create a patch to insert the code
//!     let patch = Patch::Insert {
//!         at: SpanIR::new(class.body_span.end - 1, class.body_span.end - 1),
//!         code: code.into(),
//!         source_macro: Some("Debug".to_string()),
//!     };
//!
//!     // 4. Return the result
//!     MacroResult {
//!         runtime_patches: vec![patch],
//!         ..Default::default()
//!     }
//! }
//! ```

pub mod helpers;
pub mod ir;
pub mod patch;
pub mod source_map;
pub mod span;

pub use helpers::*;
pub use ir::*;
pub use patch::*;
pub use source_map::*;
pub use span::*;

/// Re-export of SWC's ECMAScript AST types.
///
/// Available when the `swc` feature is enabled. Provides direct access
/// to the underlying AST types used by the parser.
#[cfg(feature = "swc")]
pub use swc_core::ecma::ast as swc_ast;
