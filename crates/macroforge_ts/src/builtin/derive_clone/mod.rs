//! # Clone Macro Implementation
//!
//! The `Clone` macro generates a `clone()` method for deep copying objects.
//! This is analogous to Rust's `Clone` trait, providing a way to create
//! independent copies of values.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `{className}Clone(value)` + `static clone(value)` | Standalone function + static wrapper method |
//! | Enum | `{enumName}Clone(value): EnumName` | Standalone function (enums are primitives, returns value as-is) |
//! | Interface | `{ifaceName}Clone(value): InterfaceName` | Standalone function creating a new object literal |
//! | Type Alias | `{typeName}Clone(value): TypeName` | Standalone function with spread copy for objects |
//!
//! Names use **camelCase** conversion (e.g., `Point` -> `pointClone`).
//!
//!
//! ## Cloning Strategy
//!
//! The generated clone is **type-aware** when a type registry is available:
//!
//! - **Primitives** (`string`, `number`, `boolean`, `bigint`): Copied by value
//! - **`Date`**: Deep cloned via `new Date(x.getTime())`
//! - **Arrays**: Spread copy `[...arr]`, or deep map if element type has `Clone`
//! - **`Map`/`Set`**: New collection, deep copy if value type has `Clone`
//! - **Objects with `@derive(Clone)`**: Deep cloned via their standalone clone function
//! - **Optional fields**: Null-checked -- `null`/`undefined` pass through unchanged
//! - **Other objects**: Shallow copy (reference)
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Clone) */
//! class Point {
//!     x: number;
//!     y: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class Point {
//!     x: number;
//!     y: number;
//!
//!     static clone(value: Point): Point {
//!         return pointClone(value);
//!     }
//! }
//!
//! export function pointClone(value: Point): Point {
//!     const cloned = Object.create(Object.getPrototypeOf(value));
//!     cloned.x = value.x;
//!     cloned.y = value.y;
//!     return cloned;
//! }
//! ```
//!
//! ## Implementation Notes
//!
//! - **Classes**: Uses `Object.create(Object.getPrototypeOf(value))` to preserve
//!   the prototype chain, ensuring `instanceof` checks work correctly
//! - **Enums**: Simply returns the value (enums are primitives in TypeScript)
//! - **Interfaces/Type Aliases**: Creates new object literals with spread operator
//!   for union/tuple types, or field-by-field copy for object types

mod clone_generation;
mod core;

pub use core::derive_clone_macro;
