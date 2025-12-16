//! # Built-in Derive Macros for Macroforge
//!
//! This module provides the standard derive macros that ship with macroforge-ts.
//! These macros generate common boilerplate methods for TypeScript classes,
//! interfaces, enums, and type aliases.
//!
//! ## Available Macros
//!
//! ### Equality & Hashing
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialEq` | `equals(other): boolean` | Field-by-field equality comparison |
//! | `Hash` | `hashCode(): number` | Hash code generation for collections |
//!
//! ### Ordering
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialOrd` | `compareTo(other): number \| null` | Partial ordering (can return null) |
//! | `Ord` | `compareTo(other): number` | Total ordering (never null) |
//!
//! ### Cloning & Debugging
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Clone` | `clone(): T` | Deep copy of the object |
//! | `Debug` | `toString(): string` | Human-readable debug representation |
//!
//! ### Initialization
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Default` | `static default(): T` | Factory method with default values |
//!
//! ### Serialization (Serde)
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Serialize` | `toJSON(): Record<string, unknown>` | JSON serialization |
//! | `Deserialize` | `static fromJSON(json): T` | JSON deserialization with validation |
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
mod derive_clone;

/// Shared utilities for comparison macros.
mod derive_common;

/// Debug macro implementation (toString).
mod derive_debug;

/// Default macro implementation (factory method).
mod derive_default;

/// Hash macro implementation (hashCode).
mod derive_hash;

/// Ord macro implementation (total ordering).
mod derive_ord;

/// PartialEq macro implementation (equals).
mod derive_partial_eq;

/// PartialOrd macro implementation (partial ordering).
mod derive_partial_ord;

/// Serialization macros (Serialize, Deserialize).
mod serde;
