//! # Built-in Derive Macros for Macroforge
//!
//! This module provides the standard derive macros that ship with macroforge-ts.
//! These macros generate common boilerplate methods for TypeScript classes,
//! interfaces, enums, and type aliases.
//!
//! ## Available Macros
//!
//! For **classes**, each macro generates a static method on the class, where the
//! code reaches the class's `private` and `protected` fields, plus a standalone
//! function (e.g. `userClone`, `userEncode`) that runs it.
//! **Enums, interfaces, and type aliases** get standalone functions only (e.g.
//! `statusDefaultValue`, `pointEquals`), since methods cannot be attached to them.
//! The tables below show the class-side static methods.
//!
//! A derive works over every field an instance holds. For a class that is its
//! `private`, `protected` and `readonly` fields, its constructor parameter
//! properties and, when the project was scanned, the fields of its
//! superclasses; for an interface, those of the interfaces it extends. A field
//! keyed by a string or a number, such as `'content-type'`, is a field like any
//! other. What no derive can reach is an expansion error: a `#private` field,
//! a `private` field of a superclass, a superclass or extended interface the
//! project does not declare, and a generic one. `Clone`, `Default` and `Decode`
//! build an instance without running its constructor, so they also reject a
//! class with a field set up by a Svelte rune such as `$state`.
//!
//! ### Equality & Hashing
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialEq` | `static equals(a, b): boolean` | Field-by-field equality comparison |
//! | `Eq` | None | Marks `PartialEq` equality as total, as Rust's `Eq`; requires `PartialEq` |
//! | `Hash` | `static hashCode(value): number` | Hash code generation for collections |
//!
//! ### Ordering
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `PartialOrd` | `static partialCompare(a, b): number \| null` | Partial ordering (can return null); requires `PartialEq` |
//! | `Ord` | `static compare(a, b): number` | Total ordering (never null); requires `Eq` and `PartialOrd` |
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
//! ### Encoding (Endec)
//!
//! | Macro | Generated Method | Description |
//! |-------|------------------|-------------|
//! | `Encode` | `static encode(value, keepMetadata?): string` | JSON encoding with cycle detection |
//! | `Decode` | `static decode(input, opts?): { success: true; value: T } \| { success: false; errors }` | JSON decoding with validation |
//!
//! `Decode` additionally generates `static is(value)` / `static hasShape(obj)` type
//! guards and `validateField`/`validateFields` helpers; see [`endec`] for the full surface.
//!
//! ## Type-Position Macros
//!
//! | Macro | Expands To | Description |
//! |-------|------------|-------------|
//! | `$Newtype<T>` | `T & { readonly [brand]: true }` | Nominal brand backed by a per-site `unique symbol` |
//!
//! A `$Newtype` over a primitive gets checked `decode` / `is` from `Decode`,
//! including validators written on the alias itself; see [`newtype`].
//!
//! ## Field-Level Decorators
//!
//! Most macros support field-level decorators to customize behavior:
//!
//! ```typescript
//! /** @derive(Debug, PartialEq, Encode) */
//! class User {
//!     /** @debug({ rename: "identifier" }) */
//!     id: number;
//!
//!     /** @partialEq({ skip: true }) @endec({ skipEncoding: true }) */
//!     password: string;
//!
//!     /** @endec({ rename: "emailAddress" }) */
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

/// The derive macros: equality, ordering, hashing, cloning, debugging,
/// defaults and encoding.
pub mod derive;

/// `$Newtype<T>` type-position macro (nominal symbol brands).
pub mod newtype;

/// Return type code generation helpers for Decode and PartialOrd macros.
pub mod return_types;
