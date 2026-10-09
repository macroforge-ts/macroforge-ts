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
//! | `CustomType` | `customTypeDefaultValue()` |

mod args;
mod field_options;
mod ordering;
mod registry_helpers;
mod type_names;
mod type_utils;
mod value_type;

#[cfg(test)]
mod tests;

pub(crate) use args::find_named_value;
pub use args::{
    bigint_literal, extract_named_string, has_flag, js_string, parse_string_literal, regex_literal,
};
pub use field_options::{CompareFieldOptions, DefaultFieldOptions};
pub use ordering::{OrdField, Ordering, generate_field_order, generate_value_order};
pub use registry_helpers::{
    collection_element_type, fields_from_definition, flatten_intersection_fields,
    get_effective_fields, resolved_type_has_derive, standalone_fn_name, type_has_derive,
};
pub(crate) use type_names::TypeNames;
pub use type_utils::{
    array_element_type, detect_primitive_encodable_union, get_type_default,
    get_type_default_with_registry, has_known_default, is_generic_type, is_nullable_type,
    is_numeric_type, is_primitive_type, parse_generic_type, primitive_compare_statements,
    tuple_compare_statements, tuple_element,
};
pub use value_type::{
    Builtin, ClassifiedType, ValueType, classify_value_type, derived_function, field_value_type,
    fixed_tuple, is_primitive_union, rendered, structural_call, structural_helper,
};
