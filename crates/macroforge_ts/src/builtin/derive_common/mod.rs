//! # Shared Utilities for Derive Macros
//!
//! This module provides common functionality used by multiple derive macros,
//! including:
//!
//! - **Field options parsing**: `CompareFieldOptions`, `DefaultFieldOptions`
//! - **Type utilities**: Type checking and default value generation
//! - **Decorator parsing**: Flag extraction and named argument parsing
//!
//! ## Field Options
//!
//! Many macros support field-level customization through decorators:
//!
//! ```typescript
//! /** @derive(PartialEq, Hash, Default) */
//! class User {
//!     /** @partialEq({ skip: true }) @hash({ skip: true }) */
//!     cachedValue: number;
//!
//!     /** @default("guest") */
//!     name: string;
//! }
//! ```
//!
//! ## Type Defaults (Rust-like Philosophy)
//!
//! Like Rust's `Default` trait, this module assumes all types implement
//! default values:
//!
//! | Type | Default Value |
//! |------|---------------|
//! | `string` | `""` |
//! | `number` | `0` |
//! | `boolean` | `false` |
//! | `bigint` | `0n` |
//! | `T[]` | `[]` |
//! | `Map<K,V>` | `new Map()` |
//! | `Set<T>` | `new Set()` |
//! | `Date` | `new Date()` |
//! | `T \| null` | `null` |
//! | `CustomType` | `CustomType.defaultValue()` |

mod field_options;
mod registry_helpers;
mod type_utils;

#[cfg(test)]
mod tests;

pub use field_options::{CompareFieldOptions, DefaultFieldOptions, extract_named_string, has_flag};
pub use registry_helpers::{
    collection_element_type, map_key_type, resolved_type_has_derive, standalone_fn_name,
    type_has_derive,
};
pub use type_utils::{
    get_type_default, has_known_default, is_generic_type, is_nullable_type, is_numeric_type,
    is_primitive_type, parse_generic_type,
};
