//! # PartialEq Macro Implementation
//!
//! The `PartialEq` macro generates an `equals()` method for field-by-field
//! structural equality comparison. This is analogous to Rust's `PartialEq` trait,
//! enabling value-based equality semantics instead of reference equality.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameEquals(a, b)` + `static equals(a, b)` | Standalone function + static wrapper method |
//! | Enum | `enumNameEquals(a: EnumName, b: EnumName): boolean` | Standalone function using strict equality |
//! | Interface | `interfaceNameEquals(a: InterfaceName, b: InterfaceName): boolean` | Standalone function comparing fields |
//! | Type Alias | `typeNameEquals(a: TypeName, b: TypeName): boolean` | Standalone function with type-appropriate comparison |
//!
//! ## Comparison Strategy
//!
//! The generated equality check:
//!
//! 1. **Identity check**: `a === b` returns true immediately
//! 2. **Field comparison**: Compares each non-skipped field
//!
//! ## Type-Specific Comparisons
//!
//! | Type | Comparison Method |
//! |------|-------------------|
//! | Primitives | Strict equality (`===`) |
//! | Arrays | Length + element-by-element (recursive) |
//! | `Date` | `getTime()` comparison |
//! | `Map` | Size + entry-by-entry comparison |
//! | `Set` | Size + membership check |
//! | Objects | Calls `equals()` if available, else `===` |
//!
//! ## Field-Level Options
//!
//! The `@partialEq` decorator supports:
//!
//! - `skip` - Exclude the field from equality comparison
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(PartialEq) */
//! class User {
//!     id: number;
//!     name: string;
//!
//!     /** @partialEq({ skip: true }) */
//!     cachedScore: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class User {
//!     id: number;
//!     name: string;
//!
//!     cachedScore: number;
//!
//!     static equals(a: User, b: User): boolean {
//!         return userEquals(a, b);
//!     }
//! }
//!
//! export function userEquals(a: User, b: User): boolean {
//!     if (a === b) return true;
//!     return a.id === b.id && a.name === b.name;
//! }
//! ```
//!
//! ## Equality Contract
//!
//! When implementing `PartialEq`, consider also implementing `Hash`:
//!
//! - **Reflexivity**: `a.equals(a)` is always true
//! - **Symmetry**: `a.equals(b)` implies `b.equals(a)`
//! - **Hash consistency**: Equal objects must have equal hash codes
//!
//! To maintain the hash contract, skip the same fields in both `PartialEq` and `Hash`,
//! as shown in the example above using `@partialEq({ skip: true }) @hash({ skip: true })`.

mod core;
mod equality;
#[cfg(test)]
mod tests;
mod types;

pub use self::core::derive_partial_eq_macro;
pub use equality::generate_field_equality_for_interface;
pub use types::EqField;
