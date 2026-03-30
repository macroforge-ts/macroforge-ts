//! # Hash Macro Implementation
//!
//! The `Hash` macro generates a `hashCode()` method for computing numeric hash codes.
//! This is analogous to Rust's `Hash` trait and Java's `hashCode()` method, enabling
//! objects to be used as keys in hash-based collections.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameHashCode(value)` + `static hashCode(value)` | Standalone function + static wrapper method |
//! | Enum | `enumNameHashCode(value: EnumName): number` | Standalone function hashing by enum value |
//! | Interface | `interfaceNameHashCode(value: InterfaceName): number` | Standalone function computing hash |
//! | Type Alias | `typeNameHashCode(value: TypeName): number` | Standalone function computing hash |
//!
//!
//! ## Hash Algorithm
//!
//! Uses the standard polynomial rolling hash algorithm:
//!
//! ```text
//! hash = 17  // Initial seed
//! for each field:
//!     hash = (hash * 31 + fieldHash) | 0
//! ```
//!
//! The `| 0` (bitwise OR with zero) at the end of each step coerces the result
//! to a 32-bit signed integer, preventing floating-point drift from repeated
//! multiplication. This is equivalent to casting to `i32` in Rust and consistent
//! with Java's `Objects.hash()` implementation.
//!
//! ## Type-Specific Hashing
//!
//! | Type | Hash Strategy |
//! |------|---------------|
//! | `number` | Integer: direct value; Float: string hash of decimal |
//! | `bigint` | String hash of decimal representation |
//! | `string` | Character-by-character polynomial hash |
//! | `boolean` | 1231 for true, 1237 for false (Java convention) |
//! | `Date` | `getTime()` timestamp |
//! | Arrays | Element-by-element hash combination |
//! | `Map` | Entry-by-entry key+value hash |
//! | `Set` | Element-by-element hash |
//! | Objects | Calls `hashCode()` if available, else JSON string hash |
//!
//! ## Field-Level Options
//!
//! The `@hash` decorator supports:
//!
//! - `skip` - Exclude the field from hash calculation
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Hash) */
//! class User {
//!     id: number;
//!     name: string;
//!
//!     /** @hash({ skip: true }) */
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
//!     static hashCode(value: User): number {
//!         return userHashCode(value);
//!     }
//! }
//!
//! export function userHashCode(value: User): number {
//!     let hash = 17;
//!     hash =
//!         (hash * 31 +
//!             (Number.isInteger(value.id)
//!                 ? value.id | 0
//!                 : value.id
//!                       .toString()
//!                       .split('')
//!                       .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
//!         0;
//!     hash =
//!         (hash * 31 +
//!             (value.name ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
//!         0;
//!     return hash;
//! }
//! ```
//!
//! ## Hash Contract
//!
//! Objects that are equal (`PartialEq`) should produce the same hash code.
//! When using `@hash(skip)`, ensure the same fields are skipped in both
//! `Hash` and `PartialEq` to maintain this contract.

mod core;
mod hash_generation;
mod types;

#[cfg(test)]
mod tests;

pub use self::core::*;
pub use hash_generation::generate_field_hash_for_interface;
pub use types::HashField;
