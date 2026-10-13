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
mod property;
mod registry_helpers;
mod type_names;
mod type_utils;
mod value_type;

#[cfg(test)]
mod tests;

pub use args::{
    DecoratorOption, OptionValue, bigint_literal, decorator_options, extract_named_string,
    has_flag, js_string, js_string_text, parse_string_literal, regex_literal, whole_string_literal,
};
pub(crate) use args::{Takes, find_named_value, find_option, split_top_level_items, unwrapped};
pub use field_options::{CompareFieldOptions, DefaultFieldOptions};
pub use ordering::{OrdField, Ordering, generate_field_order, generate_value_order};
pub use property::{
    Member, is_identifier_name, local_name_part, mentions, parameter_name, property_key,
};
pub use registry_helpers::{
    ClassUse, Declared, InheritedStatics, class_instance_fields, collection_element_type,
    declared_type_has_derive, derive_alias_fields, derive_class_fields, derive_interface_fields,
    entry_derives, fields_from_definition, flatten_intersection_fields, get_effective_fields,
    inherited_statics, interface_instance_fields, resolved_type_has_derive, standalone_fn_name,
    type_has_derive,
};
pub(crate) use type_names::TypeNames;
pub use type_utils::{
    EnumValueKinds, MAX_TYPE_DEPTH, array_element_type, detect_primitive_encodable_union,
    get_type_default, get_type_default_with_registry, has_known_default, is_generic_type,
    is_nullable_type, is_numeric_type, is_primitive_type, parse_generic_type,
    primitive_compare_statements, primitive_keyword_test, record_key_texts, tuple_body,
    tuple_compare_statements, tuple_element, undefaultable, undefined_as_null, ungrouped,
};
pub use value_type::{
    Builtin, ClassifiedType, ValueType, classify_value_type, derived_function, field_value_type,
    fixed_tuple, is_primitive_union, rendered, structural_call, structural_callee,
    structural_helper,
};
