//! # Built-in Derive Macros for Macroforge
//!
//! This module provides the standard derive macros that ship with macroforge-ts.
//! These macros generate common boilerplate methods for TypeScript classes,
//! interfaces, enums, and type aliases.
//!
//! ## Available Macros
//!
//! For **classes**, each macro generates a standalone function (e.g. `userClone`,
//! `userSerialize`) plus a static wrapper method on the class that delegates to it.
//! **Enums, interfaces, and type aliases** get standalone functions only (e.g.
//! `statusDefaultValue`, `pointEquals`), since methods cannot be attached to them.
//! The tables below show the class-side static methods.
//!
//! ### Equality & Hashing
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialEq` | `static equals(a, b): boolean` | Field-by-field equality comparison |
//! | `Hash` | `static hashCode(value): number` | Hash code generation for collections |
//!
//! ### Ordering
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialOrd` | `static compareTo(a, b): number \| null` | Partial ordering (can return null) |
//! | `Ord` | `static compareTo(a, b): number` | Total ordering (never null) |
//!
//! ### Cloning & Debugging
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Clone` | `static clone(value): T` | Deep copy of the object |
//! | `Debug` | `static toString(value): string` | Human-readable debug representation |
//!
//! ### Initialization
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Default` | `static defaultValue(): T` | Factory method with default values |
//!
//! ### Serialization (Serde)
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Serialize` | `static serialize(value, keepMetadata?): string` | JSON serialization with cycle detection |
//! | `Deserialize` | `static deserialize(input, opts?): { success: true; value: T } \| { success: false; errors }` | JSON deserialization with validation |
//!
//! `Deserialize` additionally generates `static is(value)` / `static hasShape(obj)` type
//! guards and `validateField`/`validateFields` helpers; see [`serde`] for the full surface.
//!
//! ## Field-Level Decorators
//!
//! Most macros support field-level decorators to customize behavior:
//!
//! ```typescript
//! /** @derive(Debug, PartialEq, Serialize) */
//! class User {
//!     /** @debug({ rename: "identifier" }) */
//!     id: number;
//!
//!     /** @partialEq({ skip: true }) @serde({ skipSerializing: true }) */
//!     password: string;
//!
//!     /** @serde({ rename: "emailAddress" }) */
//!     email: string;
//! }
//! ```
//!
//! ## Example Usage
//!
//! ```typescript
//! /** @derive(Debug, Clone, PartialEq, Hash) */
//! class Point {
//!     x: number;
//!     y: number;
//!
//!     constructor(x: number, y: number) {
//!         this.x = x;
//!         this.y = y;
//!     }
//! }
//! ```

/// Clone macro implementation (deep copy).
pub mod derive_clone;

/// Shared utilities for comparison macros.
pub mod derive_common;

/// Debug macro implementation (toString).
mod derive_debug;

/// Default macro implementation (factory method).
mod derive_default;

/// Hash macro implementation (hashCode).
pub mod derive_hash;

/// Ord macro implementation (total ordering).
mod derive_ord;

/// PartialEq macro implementation (equals).
pub mod derive_partial_eq;

/// PartialOrd macro implementation (partial ordering).
mod derive_partial_ord;

/// Serialization macros (Serialize, Deserialize).
pub mod serde;

/// Return type code generation helpers for Deserialize and PartialOrd macros.
pub mod return_types;
