//! Intermediate Representation (IR) types for TypeScript declarations.
//!
//! This module provides stable, serializable representations of TypeScript
//! language constructs. These IR types are used throughout the Macroforge
//! macro system to represent parsed TypeScript code in a form that macro
//! authors can easily work with.
//!
//! ## Overview
//!
//! The IR types mirror TypeScript's declaration types:
//!
//! - [`ClassIR`] - Represents a TypeScript class with fields, methods, and decorators
//! - [`InterfaceIR`] - Represents a TypeScript interface with properties and method signatures
//! - [`EnumIR`] - Represents a TypeScript enum with its variants and values
//! - [`TypeAliasIR`] - Represents a type alias (`type X = ...`)
//! - [`DecoratorIR`] - Represents a decorator/attribute applied to a declaration
//! - [`MacroContextIR`] - The complete context passed to macro functions
//!
//! ## Design Philosophy
//!
//! These types are designed with several goals in mind:
//!
//! 1. **ABI Stability**: All types implement `Serialize` and `Deserialize` for stable
//!    communication between the macro system and macro implementations.
//!
//! 2. **Ergonomic Access**: Types provide convenient accessor methods and iterators
//!    for common operations (e.g., `class.fields()`, `enum.variants()`).
//!
//! 3. **Source Preservation**: Types preserve source spans for accurate error
//!    reporting and code generation.
//!
//! 4. **Type String Representation**: Type annotations are stored as strings
//!    (`ts_type: String`) for simplicity and to avoid complex type system modeling.
//!
//! ## Example
//!
//! ```rust,no_run
//! use macroforge_ts_syn::{ClassIR, FieldIR, Visibility};
//!
//! fn process_class(class: &ClassIR) {
//!     println!("Class: {}", class.name);
//!
//!     for field in &class.fields {
//!         if field.visibility == Visibility::Public {
//!             println!("  Public field: {} ({})", field.name, field.ts_type);
//!         }
//!     }
//!
//!     for method in &class.methods {
//!         println!("  Method: {}({})", method.name, method.params_src);
//!     }
//! }
//! ```
//!
//! ## Submodules
//!
//! - [`class`] - Class-related IR types ([`ClassIR`], [`FieldIR`], [`MethodSigIR`])
//! - [`interface`] - Interface-related IR types ([`InterfaceIR`], [`InterfaceFieldIR`])
//! - [`enum_`] - Enum-related IR types ([`EnumIR`], [`EnumVariantIR`], [`EnumValue`])
//! - [`type_alias`] - Type alias IR types ([`TypeAliasIR`], [`TypeBody`], [`TypeMember`])
//! - [`decorators`] - Decorator representation ([`DecoratorIR`])
//! - [`context`] - Macro execution context ([`MacroContextIR`], [`MacroKind`], [`TargetIR`])

pub mod class;
pub mod context;
pub mod decorators;
pub mod enum_;
pub mod interface;
pub mod type_alias;

pub use class::*;
pub use context::*;
pub use decorators::*;
pub use enum_::*;
pub use interface::*;
pub use type_alias::*;
