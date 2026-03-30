//! # Derived Macro Registration System
//!
//! This module provides compile-time macro registration using the `inventory` crate.
//! It enables macros to be automatically discovered and registered without explicit
//! runtime registration code.
//!
//! ## How It Works
//!
//! 1. Macro implementations use the `#[derive_macro]` attribute (from macroforge_ts_macros)
//! 2. The attribute generates a `DerivedMacroRegistration` entry with static metadata
//! 3. The `inventory` crate collects all entries at link time
//! 4. At runtime, `register_module()` iterates entries and registers macros
//!
//! ## Dynamic Module Resolution
//!
//! Macros registered with `DYNAMIC_MODULE_MARKER` as their module path will be
//! resolved by name alone, regardless of the import path in user code. This enables
//! flexible import patterns:
//!
//! ```typescript
//! // All of these work with dynamic resolution:
//! import { Debug } from "macroforge-ts";
//! import { Debug } from "@my-org/macros";
//! import { Debug } from "./local-macros";
//! ```
//!
//! ## Manifest Generation
//!
//! The module provides `get_manifest()` for tooling to discover available macros
//! and their decorators at runtime. This is used by:
//! - IDE extensions for autocompletion
//! - Documentation generators
//! - The `__macroforgeGetManifest()` NAPI export

mod descriptors;
mod manifest;
mod registry;

pub use descriptors::*;
pub use manifest::*;
pub use registry::*;
